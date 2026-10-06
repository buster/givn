// Characterization harness for the generated OpenCode attribution plugin.
//
// Consumes PUBLIC hooks only, with controlled payloads. This is not evidence
// of real-loader behavior (the E2E scenarios provide that). Each invocation
// imports the plugin module afresh (cache-busting query string) so module
// state (session map, last model) never leaks between cases.
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
  const mod = await import(`${pathToFileURL(asset).href}?case=${process.pid}-${Date.now()}`)
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
    const register = (group) => ({
      hook: async (name, handler) => {
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
    result.events.push({ group, name, keys: Object.keys(event).sort() })
    const handler = registered[group][name]
    if (handler) await handler(event)
  }

  let counter = 0
  const request = async (action) => {
    counter += 1
    const callID = action.id ?? `call-${counter}`
    await fire("tool", "execute.before", {
      tool: "shell",
      sessionID: action.session,
      agent: "build",
      messageID: action.message ?? `message-${action.session}`,
      id: callID,
      input: { command: action.command },
    })
    return callID
  }
  const create = async (action, op) => {
    const event = {
      command: action.command,
      cwd: "/fixture/project",
      timeout: 120000,
      shell: "/bin/sh",
      env: { ...initialEnv },
    }
    await fire("shell", "create.before", event)
    result.shells.push({ op, env: event.env })
  }

  for (const action of actions) {
    switch (action.op) {
      case "default":
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

main().catch((error) => {
  process.stderr.write(`characterize failed: ${error?.stack ?? error}\n`)
  process.exit(1)
})
