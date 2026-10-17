// Givn development timeline for the opencode coding agent.
//
// opencode discovers this file under `.opencode/plugins/`, imports it, and
// calls the definition's `setup(ctx)`. This is the V2 plugin shape: a default
// export with a stable `id` and an async `setup`; every registration goes
// through the `ctx` domains, and the server's event stream arrives through the
// one `ctx.event.subscribe` iterable.
//
// The reporter buffers the session's activity and flushes it to the real
// intake (`givn history ingest`) at the step boundary and at
// `session.execution.*`, so one payload covers one step. opencode's event
// envelope carries the harness's own `created` timestamps (epoch ms), so
// durations are measured from those where the event stream provides them and
// from the reporter's own stamps otherwise. It never blocks the agent and
// never fails a hook: every failure is contained and reported once.

import { spawn } from "node:child_process"

const HARNESS = "opencode"
/** The buffer is bounded so a long session cannot grow without limit. */
const MAX_BUFFER = 1000

/** One harness timestamp as the intake's RFC 3339 UTC form (millisecond precision). */
const at = (ms) => new Date(ms).toISOString()

/**
 * The bounded subject label of a tool call: one line, ≤ 120 characters, never
 * a payload dump (D2).
 */
const bounded = (subject) => {
  const line = String(subject ?? "").split("\n")[0]
  return line.length <= 120 ? line : `${line.slice(0, 119)}…`
}

/** The tool call's subject from the harness's own input. */
const toolSubject = (name, input) => {
  const value = input && typeof input === "object" ? (input.command ?? input.filePath ?? input.file_path ?? input.path ?? input.pattern ?? input.query) : undefined
  return bounded(typeof value === "string" && value.length > 0 ? value : name)
}

/** opencode's `Tokens.Info` as the intake's `usage`, counts kept as reported. */
const usageOf = (tokens) => {
  if (!tokens || typeof tokens !== "object") return undefined
  const counts = {}
  for (const [key, value] of [
    ["input", tokens.input],
    ["output", tokens.output],
    ["cache_read", tokens.cacheRead ?? tokens.cache_read],
    ["cache_write", tokens.cacheWrite ?? tokens.cache_write],
    ["total_tokens", tokens.total ?? tokens.totalTokens ?? tokens.total_tokens],
  ]) {
    if (typeof value === "number") counts[key] = value
  }
  return Object.keys(counts).length > 0 ? counts : undefined
}

export default {
  id: "givn-timeline",
  async setup(ctx) {
    let session
    let buffer = []
    let dropped = 0
    let warned = false
    const tools = new Map()
    const steps = new Map()
    const waits = new Map()
    const children = new Map()
    const completed = new Set()

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

    /** One measured span from two of the harness's own timestamps. */
    const between = (start, end) =>
      typeof start === "number" && typeof end === "number" && end >= start ? end - start : undefined

    /** One payload per boundary, piped to the real intake; the record is never a work item. */
    const flush = () =>
      new Promise((resolve) => {
        if (buffer.length === 0) {
          resolve()
          return
        }
        const payload = { schema: 1, reporter: HARNESS, session, events: buffer }
        buffer = []
        let child
        try {
          child = spawn("givn", ["history", "ingest"], { stdio: ["pipe", "ignore", "ignore"] })
        } catch {
          dropped += payload.events.length
          resolve()
          return
        }
        child.on("error", () => {
          dropped += payload.events.length
          resolve()
        })
        child.on("close", () => resolve())
        try {
          child.stdin.end(JSON.stringify(payload))
        } catch {
          resolve()
        }
      })

    /** Every start the session never closed still becomes one unmeasured entry. */
    const materialize = () => {
      for (const start of tools.values()) {
        push({
          kind: "tool",
          at: at(start.created),
          tool: start.name,
          label: start.label,
          outcome: "unknown",
        })
      }
      tools.clear()
      for (const wait of waits.values()) {
        push({ kind: "question", at: at(wait.created), label: wait.label })
      }
      waits.clear()
    }

    const handle = (event) => {
      const type = event?.type
      const data = event?.data ?? {}
      const created = typeof event?.created === "number" ? event.created : Date.now()
      if (typeof data.sessionID === "string") session = data.sessionID
      switch (type) {
        case "session.step.started":
          steps.set(data.assistantMessageID ?? data.id, created)
          break
        case "session.step.ended":
        case "session.step.failed": {
          const key = data.assistantMessageID ?? data.id
          const started = steps.get(key)
          steps.delete(key)
          push({
            kind: "turn",
            at: at(created),
            duration_ms: between(started, created),
            model: typeof data.model === "string" ? data.model : undefined,
            usage: usageOf(data.tokens),
            status: type === "session.step.ended" ? "completed" : "failed",
          })
          break
        }
        case "session.tool.called":
        case "session.tool.input.started":
          tools.set(data.id, {
            created,
            name: typeof data.name === "string" ? data.name : data.tool,
            label: toolSubject(data.name, data.input),
          })
          break
        case "session.tool.success":
        case "session.tool.failed": {
          const start = tools.get(data.id)
          tools.delete(data.id)
          completed.add(data.id)
          push({
            kind: "tool",
            at: at(created),
            tool: start?.name ?? data.name,
            label: start?.label ?? toolSubject(data.name, data.input),
            duration_ms: between(start?.created, created),
            outcome: type === "session.tool.success" ? "ok" : "failed",
          })
          break
        }
        case "session.created":
        case "session.forked":
          if (typeof data.parentID === "string") {
            children.set(data.id, created)
            push({
              kind: "subagent",
              at: at(created),
              agent: typeof data.agent === "string" ? data.agent : undefined,
              subagent: data.id,
              status: "started",
            })
          }
          break
        case "session.execution.succeeded":
        case "session.execution.failed":
        case "session.execution.interrupted": {
          const started = children.get(data.sessionID)
          if (started === undefined) break
          children.delete(data.sessionID)
          push({
            kind: "subagent",
            at: at(created),
            subagent: data.sessionID,
            duration_ms: between(started, created),
            status: type === "session.execution.succeeded" ? "completed" : "failed",
          })
          break
        }
        case "permission.asked":
        case "form.created":
          waits.set(data.id, {
            created,
            label: bounded(data.tool ?? data.title ?? type),
          })
          break
        case "permission.replied":
        case "form.replied":
        case "form.cancelled": {
          const wait = waits.get(data.id)
          waits.delete(data.id)
          if (!wait) break
          push({
            kind: "question",
            at: at(created),
            label: wait.label,
            duration_ms: between(wait.created, created),
            status: "completed",
          })
          break
        }
        default:
          break
      }
      if (type?.startsWith("session.execution.")) {
        materialize()
        return flush()
      }
      return undefined
    }

    // Every registration is contained: one failing domain must not stop the
    // others, and no registration path does I/O.
    const attempt = async (kind, action) => {
      try {
        await action()
      } catch (error) {
        console.warn(`givn-timeline: registering ${kind} failed: ${error?.message ?? error}`)
      }
    }

    const guarded = (handler) => async (...args) => {
      try {
        return await handler(...args)
      } catch {
        // A hook that throws must never fail the action it observes.
        return undefined
      }
    }

    await attempt("tool.execute.before", () =>
      ctx.tool?.hook?.(
        "execute.before",
        guarded((event) => {
          if (typeof event?.id !== "string") return
          if (!tools.has(event.id)) {
            // The hook is the pairing backstop: a call whose input event was
            // missed still gets its start stamped here.
            tools.set(event.id, { created: Date.now(), name: event.tool, label: toolSubject(event.tool, event.input) })
          }
        }),
      ),
    )

    await attempt("tool.execute.after", () =>
      ctx.tool?.hook?.(
        "execute.after",
        guarded((event) => {
          if (typeof event?.id !== "string" || completed.has(event.id)) return
          const start = tools.get(event.id)
          tools.delete(event.id)
          push({
            kind: "tool",
            at: at(Date.now()),
            tool: start?.name ?? event.tool,
            label: start?.label ?? toolSubject(event.tool, event.input),
            duration_ms: typeof start?.created === "number" ? Date.now() - start.created : undefined,
            outcome: event.error ? "failed" : "ok",
          })
        }),
      ),
    )

    await attempt("permission.evaluate", () =>
      ctx.permission?.hook?.(
        "evaluate",
        guarded((event) => {
          if (typeof event?.id !== "string") return
          waits.set(event.id, { created: Date.now(), label: bounded(event.tool ?? "permission") })
        }),
      ),
    )

    // The event stream: one subscription for the whole session, consumed until
    // the harness closes it or the signal aborts.
    const subscription = ctx.event?.subscribe?.({ signal: ctx.signal })
    if (subscription) {
      ;(async () => {
        try {
          for await (const event of subscription) {
            await handle(event)
          }
        } catch {
          // The stream ends when the session does; nothing to flush beyond the
          // last boundary.
        }
      })()
    }

    return () => {
      tools.clear()
      steps.clear()
      waits.clear()
      children.clear()
      buffer = []
    }
  },
}
