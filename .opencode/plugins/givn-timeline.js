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
/** The question tool's harness name: its own call is a wait, not a tool span. */
const QUESTION_TOOL = "question"
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

/** Every question and option label the harness carries, joined: the always-captured text. */
const questionsText = (questions) => {
  if (!Array.isArray(questions)) return undefined
  const parts = []
  for (const question of questions) {
    if (typeof question?.question === "string") parts.push(question.question)
    const options = Array.isArray(question?.options) ? question.options : []
    for (const option of options) {
      const label = typeof option === "string" ? option : option?.label
      if (typeof label === "string") parts.push(label)
    }
  }
  return parts.length > 0 ? parts.join("\n") : undefined
}

/** A form's fields as the question text: every field title and description joined. */
const formText = (form) => {
  const fields = form && typeof form === "object" && Array.isArray(form.fields) ? form.fields : []
  const parts = []
  for (const field of fields) {
    if (typeof field?.title === "string") parts.push(field.title)
    if (typeof field?.description === "string") parts.push(field.description)
  }
  return parts.length > 0 ? parts.join("\n") : undefined
}

/** The question text as the intake's wire shape; the intake re-applies the cap. */
const reportedQuestion = (text) =>
  typeof text === "string"
    ? { text, truncated: false, bytes: Buffer.byteLength(text) }
    : undefined

/** One content field on the wire; the intake re-applies the content caps. */
const contentField = (text) => {
  if (typeof text !== "string" || text.length === 0) return undefined
  return { text, truncated: false, bytes: Buffer.byteLength(text) }
}

/** A value as content text: a string as-is, anything else serialized. */
const contentText = (value) => {
  if (typeof value === "string") return value
  if (value === undefined || value === null) return undefined
  try {
    return JSON.stringify(value)
  } catch {
    return String(value)
  }
}

/** The tool call's content set, built only when the mode captures content. */
const toolContent = (input, output) => {
  const content = {
    tool_input: contentField(contentText(input)),
    tool_output: contentField(contentText(output)),
  }
  for (const key of Object.keys(content)) {
    if (content[key] === undefined) delete content[key]
  }
  return Object.keys(content).length > 0 ? content : undefined
}

/** opencode's `Tokens.Info` as the intake's `usage`, counts kept as reported. */
const usageOf = (tokens) => {
  if (!tokens || typeof tokens !== "object") return undefined
  const counts = {}
  for (const [key, value] of [
    ["input", tokens.input],
    ["output", tokens.output],
    ["reasoning", tokens.reasoning ?? tokens.reasoningTokens],
    ["cache_read", tokens.cacheRead ?? tokens.cache_read],
    ["cache_write", tokens.cacheWrite ?? tokens.cache_write],
    ["total_tokens", tokens.total ?? tokens.totalTokens ?? tokens.total_tokens],
  ]) {
    if (typeof value === "number") counts[key] = value
  }
  return Object.keys(counts).length > 0 ? counts : undefined
}

/**
 * The project's capture mode: one `givn history settings --json` subprocess
 * query at plugin setup, cached for the session. A failed or malformed query
 * degrades to `on` — metadata only — and never blocks the harness's own work.
 */
const queryMode = () =>
  new Promise((resolve) => {
    let output = ""
    let child
    try {
      child = spawn("givn", ["history", "settings", "--json"], {
        stdio: ["ignore", "pipe", "ignore"],
      })
    } catch {
      resolve("on")
      return
    }
    child.stdout?.on("data", (chunk) => {
      output += chunk
    })
    child.on("error", () => resolve("on"))
    child.on("close", () => {
      try {
        const parsed = JSON.parse(output)
        resolve(["off", "on", "full"].includes(parsed.mode) ? parsed.mode : "on")
      } catch {
        resolve("on")
      }
    })
  })

export default {
  id: "givn-timeline",
  async setup(ctx) {
    // Subscribe before anything else: the harness can start its first tool
    // while setup is still resolving settings. Events are queued until the
    // handler is ready, so no start, success, or text is lost.
    const pendingEvents = []
    let onEvent = null
    const subscription = ctx.event?.subscribe?.({ signal: ctx.signal })
    if (subscription) {
      ;(async () => {
        try {
          for await (const event of subscription) {
            if (onEvent) await onEvent(event)
            else pendingEvents.push(event)
          }
        } catch {}
      })()
    }
    const modeState = { off: false, full: false }
    const modePromise = queryMode().then((resolved) => {
      modeState.off = resolved === "off"
      modeState.full = resolved === "full"
      return resolved
    })
    let session
    let buffer = []
    let dropped = 0
    let warned = false
    const tools = new Map()
    const steps = new Map()
    const texts = new Map()
    const thoughts = new Map()
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

    /** The step's assistant and thinking text; absent fields are omitted. */
    const turnContentFor = (key) => {
      const content = {
        assistant_text: contentField(texts.get(key)),
        thinking_text: contentField(thoughts.get(key)),
      }
      texts.delete(key)
      thoughts.delete(key)
      for (const name of Object.keys(content)) {
        if (content[name] === undefined) delete content[name]
      }
      return Object.keys(content).length > 0 ? content : undefined
    }

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
          tool: start.name ?? "unknown",
          label: start.label ?? "unknown",
          outcome: "unknown",
        })
      }
      tools.clear()
      for (const wait of waits.values()) {
        push({
          kind: "question",
          at: at(wait.created),
          label: wait.label,
          question: wait.question,
        })
      }
      waits.clear()
    }

    const handle = (event) => {
      if (modeState.off) return undefined
      const type = event?.type
      const data = event?.data ?? {}
      const created = typeof event?.created === "number" ? event.created : Date.now()
      if (typeof data.sessionID === "string") session = data.sessionID
      switch (type) {
        case "session.text.ended": {
          if (typeof data.assistantMessageID === "string" && typeof data.text === "string") {
            texts.set(data.assistantMessageID, data.text)
          }
          break
        }
        case "session.reasoning.ended": {
          if (typeof data.assistantMessageID === "string" && typeof data.text === "string") {
            thoughts.set(data.assistantMessageID, data.text)
          }
          break
        }
        case "session.tool.input.ended": {
          const previous = tools.get(data.id)
          if (previous && previous.input === undefined && typeof data.text === "string") {
            let input = data.text
            try {
              input = JSON.parse(data.text)
            } catch {}
            tools.set(data.id, { ...previous, input })
          }
          break
        }
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
            stop_reason:
              typeof (data.rawFinish ?? data.finish) === "string"
                ? (data.rawFinish ?? data.finish)
                : undefined,
            cost: data.cost,
            usage: usageOf(data.tokens),
            content: modeState.full ? turnContentFor(key) : undefined,
            status: type === "session.step.ended" ? "completed" : "failed",
          })
          break
        }
        case "session.tool.called":
        case "session.tool.input.started": {
          const previous = tools.get(data.id) ?? {}
          const name =
            typeof data.name === "string"
              ? data.name
              : typeof data.tool === "string"
                ? data.tool
                : previous.name
          if (name === QUESTION_TOOL) {
            const text = questionsText(data.input?.questions)
            waits.set(data.id, {
              created,
              label: bounded(text ?? name),
              question: reportedQuestion(text),
            })
            break
          }
          const input = data.input ?? previous.input
          tools.set(data.id, {
            created: previous.created ?? created,
            name,
            label: toolSubject(name, input),
            input,
          })
          if (
            type === "session.tool.called" &&
            (name === "subagent" || name === "task" || name === "agent")
          ) {
            const title = bounded(input?.description ?? input?.prompt ?? name)
            push({
              kind: "subagent",
              at: at(created),
              agent: name,
              subagent: typeof data.id === "string" ? data.id : undefined,
              title,
              status: "started",
            })
          }
          break
        }
        case "session.tool.success":
        case "session.tool.failed": {
          const start = tools.get(data.id)
          tools.delete(data.id)
          completed.add(data.id)
          const duration = between(start?.created, created)
          const toolOutput = Array.isArray(data.content)
            ? data.content
                .filter((block) => block && block.type === "text" && typeof block.text === "string")
                .map((block) => block.text)
                .join("\n")
            : (data.content ?? data.output)
          push({
            kind: "tool",
            at: at(created),
            tool: start?.name ?? data.name ?? data.tool ?? "unknown",
            label: start?.label ?? toolSubject(data.name ?? data.tool, data.input),
            tool_call_id: typeof data.id === "string" ? data.id : undefined,
            duration_ms: duration,
            duration_source: duration === undefined ? undefined : "host",
            outcome: type === "session.tool.success" ? "ok" : "failed",
            error:
              type === "session.tool.failed" && typeof data.error === "string"
                ? data.error
                : undefined,
            output_bytes:
              typeof data.bytes === "number"
                ? data.bytes
                : typeof toolOutput === "string"
                  ? Buffer.byteLength(toolOutput)
                  : undefined,
            result_meta: data.result_meta,
            content: modeState.full
              ? toolContent(start?.input ?? data.input, toolOutput)
              : undefined,
          })
          if (start?.name === "subagent" || start?.name === "task" || start?.name === "agent") {
            push({
              kind: "subagent",
              at: at(created),
              agent: start.name,
              subagent: typeof data.id === "string" ? data.id : undefined,
              title: start.label,
              status: type === "session.tool.success" ? "completed" : "failed",
              duration_ms: duration,
              outcome: type === "session.tool.success" ? "ok" : "failed",
            })
          }
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
              title: typeof data.title === "string" ? data.title : undefined,
              status: "started",
            })
          }
          break
        case "session.renamed": {
          if (typeof data.sessionID === "string" && typeof data.title === "string" && data.title) {
            push({
              kind: "session",
              at: at(created),
              name: bounded(data.title),
              session: data.sessionID,
              title: bounded(data.title),
            })
          }
          if (typeof data.sessionID === "string" && children.has(data.sessionID)) {
            const title = typeof data.title === "string" ? bounded(data.title) : undefined
            if (title) {
              push({
                kind: "subagent",
                at: at(created),
                subagent: data.sessionID,
                title,
                status: "started",
              })
            }
          }
          break
        }
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
        case "permission.asked": {
          waits.set(data.id, {
            created,
            label: bounded(data.tool ?? data.title ?? type),
          })
          break
        }
        case "form.created": {
          const form = data.form && typeof data.form === "object" ? data.form : {}
          waits.set(form.id ?? data.id, {
            created,
            label: bounded(form.title ?? data.title ?? type),
            question: reportedQuestion(formText(form)),
          })
          break
        }
        case "permission.replied":
        case "form.replied":
        case "form.cancelled": {
          const key = data.requestID ?? data.form?.id ?? data.formID ?? data.id
          const wait = waits.get(key)
          waits.delete(key)
          if (!wait) break
          push({
            kind: "question",
            at: at(created),
            label: wait.label,
            question: wait.question,
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
          if (event?.tool === QUESTION_TOOL) return
          if (!tools.has(event.id)) {
            // The hook is the pairing backstop: a call whose input event was
            // missed still gets its start stamped here.
            tools.set(event.id, {
              created: Date.now(),
              name: event.tool,
              label: toolSubject(event.tool, event.input),
              input: event.input,
            })
          }
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

    // Resolve the capture mode, then replay the events that raced setup.
    await modePromise
    onEvent = handle
    for (const event of pendingEvents.splice(0)) {
      await handle(event)
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
