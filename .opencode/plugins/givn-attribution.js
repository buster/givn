const sessions = new Map()
let lastModel = null
const UNAVAILABLE = "unavailable"

const rememberModel = (sessionID, model, variant) => {
  if (!model && !variant) return
  const current = sessionID ? sessions.get(sessionID) ?? {} : {}
  const modelID = model?.id ?? model?.modelID ?? current.modelID
  const providerID = model?.providerID ?? current.providerID
  const resolvedVariant = variant ?? model?.variant ?? current.variant

  const record = {
    providerID,
    modelID,
    variant: resolvedVariant,
  }

  if (sessionID) {
    sessions.set(sessionID, record)
  }
  if (record.modelID) {
    lastModel = record
  }
}

export const GivnAttributionPlugin = async () => {
  return {
    "chat.message": async (input) => {
      rememberModel(input.sessionID, input.model, input.variant)
    },
    "chat.params": async (input) => {
      rememberModel(input.sessionID, input.model)
    },
    "shell.env": async (input, output) => {
      output.env.GIVN_ATTRIBUTION_ACTIVE = "1"
      output.env.GIVN_ATTRIBUTION_LLM = UNAVAILABLE
      output.env.GIVN_ATTRIBUTION_REASONING = UNAVAILABLE
      output.env.GIVN_ATTRIBUTION_HARNESS = "opencode"

      const model = sessions.get(input.sessionID) ?? lastModel
      if (!model?.modelID) return

      const identity = model.providerID
        ? `${model.providerID}/${model.modelID}`
        : model.modelID
      output.env.GIVN_ATTRIBUTION_LLM = identity
      output.env.GIT_COMMITTER_NAME = `LLM: ${identity} via opencode`
      if (model.variant) output.env.GIVN_ATTRIBUTION_REASONING = model.variant
    },
  }
}

export default {
  id: "givn-attribution",
  async setup(ctx) {
    await ctx.session.hook("context", (event) => {
      rememberModel(event.sessionID, event.model, event.model?.variant)
    })

    await ctx.session.hook("compaction", (event) => {
      rememberModel(event.sessionID, event.model, event.model?.variant)
    })

    await ctx.session.hook("generate", (event) => {
      rememberModel(event.sessionID, event.model, event.model?.variant)
    })

    await ctx.shell.hook("create.before", async (event) => {
      event.env.GIVN_ATTRIBUTION_ACTIVE = "1"
      event.env.GIVN_ATTRIBUTION_LLM = UNAVAILABLE
      event.env.GIVN_ATTRIBUTION_REASONING = UNAVAILABLE
      event.env.GIVN_ATTRIBUTION_HARNESS = "opencode"

      const sessionID = event.env.OPENCODE_SESSION_ID
      let model = sessionID ? sessions.get(sessionID) : undefined
      if (!model?.modelID) {
        model = lastModel
      }

      if (!model?.modelID && ctx.model?.default) {
        try {
          const defaultModel = await ctx.model.default()
          if (defaultModel) {
            model = {
              providerID: defaultModel.providerID,
              modelID: defaultModel.id ?? defaultModel.modelID,
              variant: defaultModel.variant,
            }
          }
        } catch {
          // ignore fallback error
        }
      }

      if (!model?.modelID) return

      const identity = model.providerID
        ? `${model.providerID}/${model.modelID}`
        : model.modelID
      event.env.GIVN_ATTRIBUTION_LLM = identity
      event.env.GIT_COMMITTER_NAME = `LLM: ${identity} via opencode`
      if (model.variant) {
        event.env.GIVN_ATTRIBUTION_REASONING = model.variant
      }
    })
  },
}
