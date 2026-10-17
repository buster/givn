// Givn development timeline for the omp coding agent.
//
// omp discovers this file under `.omp/hooks/pre/`, loads it through its
// extension runner, and calls the exported factory with the extension API. The
// reporter buffers the session's activity and flushes it to the real intake
// (`givn history ingest`) at the turn boundary and at session shutdown; it
// never blocks a tool call and never fails one, and dropped events are counted
// so a long session shows a gap instead of a silently short timeline.
//
// omp emits no timestamps and no durations on its tool, turn, and subagent
// events, so every duration here is measured from the reporter's own
// `Date.now()` stamps, paired by the harness's own call id. The flush sends one
// compiled payload per boundary (`schema`, `reporter`, `session`, `events`) —
// the intake's own contract; the reporter never knows the record's line format
// (ADR-073).

import { rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

const HARNESS = "omp"
/** The buffer is bounded so a long session cannot grow without limit. */
const MAX_BUFFER = 1000
/** The question tool of the omp toolset (`BUILTIN_TOOL_NAMES`). */
const QUESTION_TOOL = "ask"

/** One harness timestamp as the intake's RFC 3339 UTC form (millisecond precision). */
const at = (ms) => new Date(ms).toISOString()

/**
 * The bounded subject label of a tool call: the same discipline the record
 * keeps — one line, ≤ 120 characters, never a payload dump. The complete
 * command text is never stored (D2).
 */
const bounded = (subject) => {
  const line = String(subject ?? "").split("\n")[0]
  return line.length <= 120 ? line : `${line.slice(0, 119)}…`
}

/** The tool call's subject: the command for a shell call, the path for a file call. */
const toolSubject = (name, args) => {
  const input = args && typeof args === "object" ? args : {}
  const subject = input.command ?? input.file_path ?? input.path ?? input.pattern
  return bounded(typeof subject === "string" && subject.length > 0 ? subject : name)
}

/** `message.usage` (omp's own usage object) as the intake's `usage`, counts kept as reported. */
const usageOf = (message) => {
  const usage = message && typeof message === "object" ? message.usage : undefined
  if (!usage || typeof usage !== "object") return undefined
  const counts = {}
  for (const [key, value] of [
    ["input", usage.input],
    ["output", usage.output],
    ["cache_read", usage.cacheRead ?? usage.cache_read],
    ["cache_write", usage.cacheWrite ?? usage.cache_write],
    ["total_tokens", usage.totalTokens ?? usage.total_tokens],
  ]) {
    if (typeof value === "number") counts[key] = value
  }
  if (Object.keys(counts).length === 0) return undefined
  const model = message.model ?? usage.model
  if (typeof model === "string" && model.length > 0) counts.model = model
  return counts
}

/** The first question of an `ask` call, as the bounded label the record keeps. */
const questionOf = (args) => {
  const questions = args && typeof args === "object" ? args.questions : undefined
  const first = Array.isArray(questions) && questions.length > 0 ? questions[0] : undefined
  const text = first && typeof first === "object" ? first.question : undefined
  return bounded(typeof text === "string" ? text : QUESTION_TOOL)
}

export default function givnTimeline(pi) {
  // The session's open spans, keyed by the harness's own call id. omp emits no
  // durations, so each start carries the reporter's own stamp.
  const tools = new Map()
  const questions = new Map()
  const subagents = new Map()
  const turns = new Map()
  let session
  let buffer = []
  let dropped = 0
  let warned = false

  const push = (event) => {
    if (buffer.length >= MAX_BUFFER) {
      dropped += 1
      if (!warned) {
        warned = true
        console.warn(`givn-timeline: buffer full (${MAX_BUFFER}); dropped events are counted`)
      }
      return
    }
    buffer.push(event)
  }

  /** One measured span; absent when the harness never stamped its start. */
  const measured = (started) => (typeof started === "number" ? Date.now() - started : undefined)

  /** Start one flush of the buffered events: one payload per boundary. */
  const flush = async () => {
    if (buffer.length === 0) return
    const events = buffer
    buffer = []
    const payload = { schema: 1, reporter: HARNESS, session, events }
    const file = join(tmpdir(), `givn-timeline-${HARNESS}-${process.pid}-${Date.now()}.json`)
    try {
      writeFileSync(file, JSON.stringify(payload))
      await pi.exec("sh", ["-c", `givn history ingest < '${file}'`])
    } catch {
      // Best effort: the record is development data, never a work item, and a
      // failing intake must not fail the agent action it reports.
    } finally {
      try {
        rmSync(file)
      } catch {
        // A leftover temp file costs nothing; the payload carries no secrets.
      }
    }
  }

  /** Every start the session never closed still becomes one unmeasured entry. */
  const materialize = () => {
    for (const start of tools.values()) {
      push({
        kind: "tool",
        at: at(start.started),
        tool: start.tool,
        label: start.label,
        outcome: "unknown",
      })
    }
    tools.clear()
    for (const question of questions.values()) {
      push({ kind: "question", at: at(question.started), label: question.label })
    }
    questions.clear()
  }

  pi.on("session_start", (_event, ctx) => {
    try {
      session = ctx?.sessionManager?.getSessionId?.()
    } catch {
      session = undefined
    }
  })

  pi.on("turn_start", (event) => {
    if (typeof event?.turnIndex === "number") turns.set(event.turnIndex, Date.now())
  })

  pi.on("turn_end", async (event) => {
    const index = typeof event?.turnIndex === "number" ? event.turnIndex : undefined
    const started = index === undefined ? undefined : turns.get(index)
    if (index !== undefined) turns.delete(index)
    const message = event?.message
    push({
      kind: "turn",
      at: at(Date.now()),
      turn: index,
      duration_ms: measured(started),
      status: "completed",
      model: message && typeof message.model === "string" ? message.model : undefined,
      usage: usageOf(message),
    })
    materialize()
    await flush()
  })

  pi.on("tool_execution_start", (event) => {
    const started = Date.now()
    if (event?.toolName === QUESTION_TOOL) {
      questions.set(event.toolCallId, { started, label: questionOf(event.args) })
      return
    }
    tools.set(event.toolCallId, {
      started,
      tool: event?.toolName,
      label: toolSubject(event?.toolName, event?.args),
    })
  })

  pi.on("tool_execution_end", (event) => {
    const question = questions.get(event?.toolCallId)
    if (question) {
      questions.delete(event.toolCallId)
      push({
        kind: "question",
        at: at(Date.now()),
        label: question.label,
        duration_ms: measured(question.started),
        status: "completed",
      })
      return
    }
    const start = tools.get(event?.toolCallId)
    tools.delete(event?.toolCallId)
    push({
      kind: "tool",
      at: at(Date.now()),
      tool: start?.tool ?? event?.toolName,
      label: start?.label ?? toolSubject(event?.toolName, event?.args),
      duration_ms: measured(start?.started),
      outcome: event?.isError ? "failed" : "ok",
    })
  })

  pi.events.on("task:subagent:lifecycle", (event) => {
    const identity = typeof event?.id === "string" ? event.id : event?.agent
    if (typeof identity !== "string") return
    const agent = typeof event?.agent === "string" ? event.agent : undefined
    if (event?.status === "started" || event?.status === "running") {
      subagents.set(identity, Date.now())
      push({ kind: "subagent", at: at(Date.now()), agent, subagent: identity, status: "started" })
      return
    }
    const started = subagents.get(identity)
    subagents.delete(identity)
    push({
      kind: "subagent",
      at: at(Date.now()),
      agent,
      subagent: identity,
      duration_ms: measured(started),
      status: typeof event?.status === "string" ? event.status : "completed",
    })
  })

  pi.on("session_shutdown", async () => {
    materialize()
    // The shutdown budget is 2 s and the flush is one exec and an await: a
    // flush that cannot finish costs the events, never the shutdown.
    await flush()
  })
}
