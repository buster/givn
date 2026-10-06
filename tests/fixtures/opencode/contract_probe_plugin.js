// Test-only contract probe for OpenCode V2. The E2E step copies it into the
// isolated temporary project (never into the repository's .opencode) beside
// the generated plugin. It records the raw payload shape of the tool and shell
// events and never mutates an event, so it cannot change attribution.
import { appendFileSync } from "node:fs"

const LOG = process.env.GIVN_CONTRACT_PROBE_LOG

const record = (hook, event) => {
  const line = {
    hook,
    keys: Object.keys(event ?? {}).sort(),
    inputKeys: event?.input && typeof event.input === "object" ? Object.keys(event.input).sort() : null,
    tool: event?.tool ?? null,
    sessionID: event?.sessionID ?? null,
    messageID: event?.messageID ?? null,
    id: event?.id ?? null,
    command: typeof event?.command === "string" ? event.command : null,
    inputCommand: typeof event?.input?.command === "string" ? event.input.command : null,
  }
  appendFileSync(LOG, `${JSON.stringify(line)}\n`)
}

export default {
  id: "givn-contract-probe",
  async setup(ctx) {
    await ctx.tool.hook("execute.before", (event) => record("execute.before", event))
    await ctx.tool.hook("execute.after", (event) => record("execute.after", event))
    await ctx.shell.hook("create.before", (event) => record("create.before", event))
  },
}
