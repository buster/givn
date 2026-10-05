// Givn commit attribution for the omp coding agent.
//
// omp discovers this file under `.omp/hooks/pre/`, loads it through its
// extension runner, and binds the handlers the factory registers. The
// `tool_call` handler rewrites a revision-creating Bash command with the
// attribution environment the checked-in `.githooks/commit-msg` hook reads —
// the same protocol the OpenCode and Claude Code integrations contribute.
//
// Best effort: a command that creates no revision is left untouched, and an
// unknown model is recorded as `unavailable` rather than blocking the commit.

const UNAVAILABLE = "unavailable"
const HARNESS = "omp"
const REVISION_VERB = /\bcommit\b/

const shellQuote = (value) => `'${String(value).replaceAll("'", "'\"'\"'")}'`

const attributionEnv = (model, reasoning) => {
  const identity = model?.id
    ? model.provider
      ? `${model.provider}/${model.id}`
      : model.id
    : undefined
  const env = {
    GIVN_ATTRIBUTION_ACTIVE: "1",
    GIVN_ATTRIBUTION_LLM: identity ?? UNAVAILABLE,
    GIVN_ATTRIBUTION_REASONING: reasoning ?? UNAVAILABLE,
    GIVN_ATTRIBUTION_HARNESS: HARNESS,
  }
  if (identity) env.GIT_COMMITTER_NAME = `LLM: ${identity} via ${HARNESS}`
  return env
}

export default function givnAttribution(pi) {
  pi.on("tool_call", (event, ctx) => {
    if (event?.toolName !== "bash") return
    const input = event.input ?? {}
    const command = input.command
    if (typeof command !== "string" || !REVISION_VERB.test(command)) return

    const model = ctx?.models?.current?.() ?? ctx?.model
    const prefix = Object.entries(attributionEnv(model, pi.getThinkingLevel?.()))
      .map(([name, value]) => `${name}=${shellQuote(value)}`)
      .join(" ")

    return { input: { ...input, command: `export ${prefix}; ${command}` } }
  })
}
