// Characterization harness for the generated OpenCode attribution plugin.
//
// Consumes PUBLIC hooks only, with controlled payloads. This is not evidence
// of real-loader behavior (the E2E scenarios provide that). Each case runs in
// a fresh Node process (tests/common/opencode_characterize.rs spawns one per
// case), so module state (session map, last model) never leaks between cases.
//
// stdin : {"asset": "<abs path>", "generation": "V1"|"V2", "initialEnv": {...}, "actions": [...]}
// stdout: {"module": {...}, "hooks": {...}, "shells": [{"op": "...", "env": {...}}], "events": [{"group", "name", "keys"}]}
import { pathToFileURL } from "node:url"

const readStdin = async () => {
  const chunks = []
  for await (const chunk of process.stdin) chunks.push(chunk)
  return JSON.parse(Buffer.concat(chunks).toString("utf8"))
}

const modelPayload = (action, withVariant) => {
  const model = { providerID: action.provider, [action.identifier ?? "id"]: action.model }
  if (withVariant && action.variant !== undefined && action.variant !== null) {
    model.variant = action.variant
  }
  return model
}

const defaultResponse = (config) => {
  switch (config.mode) {
    case "model":
      return {
        location: {},
        data: { providerID: config.provider, id: config.model, variant: config.variant },
      }
    case "no model":
      return { location: {}, data: undefined }
    case "model without identity":
      return { location: {}, data: { providerID: config.provider ?? "givn-fixture" } }
    case "hangs":
      // Never settles. The timer is unref'd, so the process can still exit when
      // nothing else is pending (the plugin's own bounded wait keeps it alive).
      return new Promise(() => setInterval(() => {}, 1000).unref())
    case "fails":
      throw new Error("configured default lookup failed")
    default:
      throw new Error(`unknown default mode: ${config.mode}`)
  }
}

const main = async () => {
  const input = await readStdin()
  const { asset, generation, actions } = input
  const initialEnv = input.initialEnv ?? {}
  const mod = await import(pathToFileURL(asset).href)
  const result = {
    module: {
      id: mod.default?.id ?? null,
      server: typeof mod.default?.server,
      setup: typeof mod.default?.setup,
      named: typeof mod.GivnAttributionPlugin,
    },
    hooks: {},
    shells: [],
    events: [],
  }
  const defaultConfig = actions.find((action) => action.op === "default")

  let v1
  const registered = { session: {}, tool: {}, shell: {} }
  let cleanup
  if (generation === "V1") {
    v1 = mod.default?.server ? await mod.default.server({}) : await mod.GivnAttributionPlugin({})
    result.hooks.v1 = Object.keys(v1).sort()
  } else {
    const rejected = new Set(
      actions.filter((action) => action.op === "reject").map((action) => `${action.group}.${action.name}`),
    )
    const register = (group) => ({
      hook: async (name, handler) => {
        if (rejected.has(`${group}.${name}`)) throw new Error(`registration rejected: ${group}.${name}`)
        if (registered[group][name]) throw new Error(`duplicate ${group} hook ${name}`)
        registered[group][name] = handler
      },
    })
    const ctx = {
      session: register("session"),
      tool: register("tool"),
      shell: register("shell"),
    }
    if (defaultConfig && defaultConfig.mode !== "absent") {
      ctx.model = { default: async () => defaultResponse(defaultConfig) }
    }
    cleanup = await mod.default.setup(ctx)
    result.hooks.v2 = {
      session: Object.keys(registered.session).sort(),
      tool: Object.keys(registered.tool).sort(),
      shell: Object.keys(registered.shell).sort(),
    }
  }

  // The runtime fires an event whether or not a plugin registered for it, so a
  // plugin without that hook (the immutable prototype has no tool hooks)
  // simply is not called.
  const fire = async (group, name, event) => {
    result.events.push({ group, name, keys: Object.keys(event ?? {}).sort() })
    const handler = registered[group][name]
    if (handler) await handler(event)
  }

  let counter = 0
  const callIDs = new Map()
  const callKey = (action) => JSON.stringify([action.session, action.message ?? `message-${action.session}`, action.command])
  const request = async (action) => {
    counter += 1
    const callID = action.id ?? `call-${counter}`
    callIDs.set(callKey(action), callID)
    const event = {
      tool: "shell",
      agent: "build",
      messageID: action.message ?? `message-${action.session}`,
      id: callID,
      input: { command: action.command },
    }
    // A request without a session omits the key, as a malformed payload would.
    if (action.session !== null && action.session !== undefined) event.sessionID = action.session
    // A malformed payload may also omit the message or call identifier.
    for (const field of action.omit ?? []) delete event[field]
    await fire("tool", "execute.before", event)
    return callID
  }
  const create = async (action, op) => {
    const event = {
      command: action.command,
      cwd: "/fixture/project",
      timeout: 120000,
      shell: "/bin/sh",
    }
    if (action.environment !== false) event.env = { ...initialEnv }
    if (action.abandon) {
      // The runtime gave up on the hook after its first tick: only what the
      // handler wrote synchronously may be relied on. Later writes are not seen.
      result.events.push({ group: "shell", name: "create.before", keys: Object.keys(event).sort() })
      const handler = registered.shell["create.before"]
      if (handler) {
        handler(event).catch(() => {})
        await new Promise((resolve) => setImmediate(resolve))
      }
      result.shells.push({ op, env: { ...event.env } })
      return
    }
    await fire("shell", "create.before", event)
    result.shells.push({ op, env: event.env })
  }

  for (const action of actions) {
    switch (action.op) {
      case "default":
      case "reject":
        break
      case "remember":
        if (generation === "V1") {
          const payload = { sessionID: action.session, model: modelPayload(action, false) }
          if (action.variant !== undefined && action.variant !== null) payload.variant = action.variant
          await v1["chat.message"](payload, {})
        } else {
          await fire("session", action.request ?? "context", {
            sessionID: action.session,
            model: modelPayload(action, true),
          })
        }
        break
      case "request":
        await request(action)
        break
      case "complete": {
        const omitted = action.omit ?? []
        const id = callIDs.get(callKey(action))
        if (!id && !omitted.includes("id")) throw new Error(`no request to complete: ${callKey(action)}`)
        const event = {
          tool: "shell",
          sessionID: action.session,
          agent: "build",
          messageID: action.message ?? `message-${action.session}`,
          id,
          input: { command: action.command },
          status: "completed",
          result: {},
        }
        for (const field of omitted) delete event[field]
        await fire("tool", "execute.after", event)
        break
      }
      case "absent":
        for (const name of Object.keys(registered.session)) await fire("session", name, undefined)
        for (const name of Object.keys(registered.tool)) await fire("tool", name, undefined)
        for (const name of Object.keys(registered.shell)) await fire("shell", name, undefined)
        break
      case "malformed": {
        // Payloads the runtime contract does not promise: each must be ignored.
        const base = { agent: "build", messageID: "message-malformed" }
        const events = [
          // not a shell tool, but carrying a command another session also uses
          { ...base, tool: "read", sessionID: "session-b", id: "m1", input: { command: "git commit" } },
          // shell without an input
          { ...base, tool: "shell", sessionID: "session-b", id: "m2" },
          // shell with a non-string command
          { ...base, tool: "shell", sessionID: "session-b", id: "m3", input: { command: 42 } },
          // shell without a session
          { ...base, tool: "shell", id: "m4", input: { command: "git log" } },
        ]
        for (const event of events) {
          await fire("tool", "execute.before", event)
          await fire("tool", "execute.after", event)
        }
        break
      }
      case "create":
        await create(action, "create")
        break
      case "run": {
        if (generation === "V1") {
          const output = { env: { ...initialEnv } }
          const hookInput = action.session ? { sessionID: action.session } : {}
          await v1["shell.env"](hookInput, output)
          result.shells.push({ op: "run", env: output.env })
        } else if (action.session) {
          const callID = await request(action)
          await create(action, "run")
          await fire("tool", "execute.after", {
            tool: "shell",
            sessionID: action.session,
            agent: "build",
            messageID: action.message ?? `message-${action.session}`,
            id: callID,
            input: { command: action.command },
            status: "completed",
            result: {},
          })
        } else {
          await create(action, "run")
        }
        break
      }
      default:
        throw new Error(`unknown action: ${action.op}`)
    }
  }
  if (typeof cleanup === "function") await cleanup()
  process.stdout.write(JSON.stringify(result))
}

// A hook that never settles leaves the event loop empty; exit loudly instead of silently.
let completed = false
process.on("exit", () => {
  if (completed) return
  process.stderr.write("characterize failed: the process ended before every hook settled\n")
  process.exitCode = 1
})

main().then(() => { completed = true }).catch((error) => {
  completed = true
  process.stderr.write(`characterize failed: ${error?.stack ?? error}\n`)
  process.exit(1)
})
