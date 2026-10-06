const sessions = new Map()
let lastModel = null
const UNAVAILABLE = "unavailable"
// A hung configured-default lookup must not stall shell creation.
const DEFAULT_LOOKUP_TIMEOUT_MS = 2000

// Attribution is best effort and never blocks: a hook that throws, for example
// on a malformed payload, must not fail the command or the other hooks.
const guarded = (handler) => async (...args) => {
  try {
    await handler(...args)
  } catch {
    // Ignored on purpose; the shell is created without this attribution step.
  }
}

// A failing hook registration must not stop the remaining hooks from being registered.
const attempt = async (action) => {
  try {
    return await action()
  } catch {
    return undefined
  }
}

const rememberModel = (sessionID, model, variant) => {
  if (!model && !variant) return
  const current = sessionID ? sessions.get(sessionID) ?? {} : {}
  const record = {
    providerID: model?.providerID ?? current.providerID,
    modelID: model?.id ?? model?.modelID ?? current.modelID,
    variant: variant ?? model?.variant ?? current.variant,
  }

  if (sessionID) sessions.set(sessionID, record)
  if (record.modelID) lastModel = record
}

const stamp = (env, model) => {
  if (!env || typeof env !== "object") return

  env.GIVN_ATTRIBUTION_ACTIVE = "1"
  env.GIVN_ATTRIBUTION_LLM = UNAVAILABLE
  env.GIVN_ATTRIBUTION_REASONING = UNAVAILABLE
  env.GIVN_ATTRIBUTION_HARNESS = "opencode"

  if (!model?.modelID) return

  const identity = model.providerID
    ? `${model.providerID}/${model.modelID}`
    : model.modelID
  env.GIVN_ATTRIBUTION_LLM = identity
  env.GIT_COMMITTER_NAME = `LLM: ${identity} via opencode`
  if (model.variant) env.GIVN_ATTRIBUTION_REASONING = model.variant
}

// OpenCode V1: shell.env receives the issuing session, subagent child sessions included.
export const GivnAttributionPlugin = async () => {
  return {
    "chat.message": guarded(async (input) => {
      rememberModel(input.sessionID, input.model, input.variant)
    }),
    "chat.params": guarded(async (input) => {
      rememberModel(input.sessionID, input.model)
    }),
    "shell.env": guarded(async (input, output) => {
      stamp(output.env, sessions.get(input.sessionID) ?? lastModel)
    }),
  }
}

const withTimeout = (promise) =>
  new Promise((resolve) => {
    const timer = setTimeout(() => resolve(undefined), DEFAULT_LOOKUP_TIMEOUT_MS)
    const settle = (value) => {
      clearTimeout(timer)
      resolve(value)
    }
    promise.then(settle, () => settle(undefined))
  })

const configuredDefault = async (ctx) => {
  if (!ctx.model?.default) return undefined
  try {
    const response = await ctx.model.default()
    const model = response?.data
    if (!model) return undefined
    return {
      providerID: model.providerID,
      modelID: model.id ?? model.modelID,
      variant: model.variant,
    }
  } catch {
    return undefined
  }
}

export default {
  id: "givn-attribution",
  async server() {
    return GivnAttributionPlugin()
  },
  async setup(ctx) {
    for (const kind of ["context", "compaction", "generate"]) {
      await attempt(() =>
        ctx.session.hook(
          kind,
          guarded((event) => {
            rememberModel(event.sessionID, event.model, event.model?.variant)
          }),
        ),
      )
    }

    // OpenCode V2: the shell hook carries no session, so a command is linked to
    // its session through the tool hooks, which run before shell creation.
    let pendingShells = []

    await attempt(() =>
      ctx.tool.hook(
        "execute.before",
        guarded((event) => {
          if (
            event.tool !== "shell" ||
            typeof event.sessionID !== "string" ||
            typeof event.input?.command !== "string"
          ) {
            return
          }
          // A request from an older message of the same session never reached shell
          // creation (vetoed, denied, or aborted); it must not label a later command.
          pendingShells = pendingShells.filter(
            (pending) =>
              pending.sessionID !== event.sessionID ||
              pending.messageID === event.messageID,
          )
          pendingShells.push(event)
        }),
      ),
    )

    await attempt(() =>
      ctx.tool.hook(
        "execute.after",
        guarded((event) => {
          if (event.tool !== "shell") return
          pendingShells = pendingShells.filter(
            (pending) =>
              pending.sessionID !== event.sessionID || pending.id !== event.id,
          )
        }),
      ),
    )

    const claimShell = (command) => {
      const index = pendingShells.findIndex(
        (pending) => pending.input?.command === command,
      )
      return index < 0 ? undefined : pendingShells.splice(index, 1)[0]
    }

    await attempt(() =>
      ctx.shell.hook(
        "create.before",
        guarded(async (event) => {
          // The runtime may abandon this hook while the default lookup is pending, so
          // the unavailable defaults are written before the first await.
          stamp(event.env, undefined)
          const linked = claimShell(event.command)
          let model = linked ? sessions.get(linked.sessionID) : undefined
          if (!model?.modelID) model = lastModel
          if (!model?.modelID) model = await configuredDefault(ctx)
          stamp(event.env, model)
        }),
      ),
    )

    return () => {
      pendingShells = []
    }
  },
}
