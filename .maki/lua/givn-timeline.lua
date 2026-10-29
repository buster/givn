-- Givn development timeline for the Maki coding agent.
--
-- Maki runs exactly one file per project plugin root, `init.lua`, and nothing
-- under `lua/` loads on its own: the managed fence Givn writes into
-- `.maki/init.lua` calls this module's `setup()`, which registers the autocmds
-- below. Maki's events carry no timestamp, so every start is stamped with
-- `os.clock()` (the monotonic clock) and paired by the harness's own id;
-- `ToolDone` is the one event that can carry a host-measured `duration_ms`.
--
-- Two constraints are reproduced, not assumed. First, the module body performs
-- no `maki.*` call at load time: top-level async breaks `pcall(require)`, so
-- this file only defines tables and functions and every host call happens
-- inside a callback or an explicit `run()`. Second, the flush writes the
-- payload to a file and runs `sh -c "cat <payload-file> | givn history ingest"`
-- through `maki.fn.jobstart`, awaited with `maki.fn.jobwait` inside the
-- callback that owns it: there is no `maki.fn.system`, `jobstart` has no stdin
-- option, and a job left unawaited in a task scope is lost with its dispatch.
-- The job asks for `scope = "plugin"` because the flush can run from `run()`
-- at load time, where no task is active (the default task scope raises
-- `no active task`), and the `jobwait` in the same call still collects it.
--
-- Every callback is wrapped so a `maki` error is logged once and never
-- propagates into the dispatch, the buffer is bounded, and a failed flush
-- counts its dropped events so the report shows a gap instead of a silently
-- short timeline. Recording is best effort and may be incomplete: a stalled
-- helper is stopped after a bounded wait.

local M = {}

local HARNESS = "maki"
local SCHEMA = 1
local MAX_BUFFER = 1000
local JOB_TIMEOUT_MS = 2000
local LABEL_LIMIT = 120

local buffer = {}
local dropped = 0
local warned = false
local last_session = nil
local logged = {}

-- Per-session state, keyed by the harness's own session id: the buffer is one
-- for the process, and every entry names the session its event belonged to.
local sessions = {}

local function session_state(session_id)
  local key = session_id or "maki"
  local state = sessions[key]
  if not state then
    state = {
      id = key,
      model = nil,
      tools = {},
      turn_started = nil,
      wait_started = nil,
      wait_label = nil,
      compaction_started = nil,
      tasks = {},
    }
    sessions[key] = state
  end
  return state
end

-- Days since 1970-01-01 -> (year, month, day), after Howard Hinnant's
-- `civil_from_days`: os.time is the whole-second clock available here, and the
-- record's `at` is an RFC 3339 UTC instant.
local function civil_from_days(days)
  local shifted = days + 719468
  local era = math.floor(shifted / 146097)
  local day_of_era = shifted - era * 146097
  local year_of_era = math.floor(
    (day_of_era - math.floor(day_of_era / 1460) + math.floor(day_of_era / 36524) - math.floor(day_of_era / 146096)) / 365
  )
  local year = year_of_era + era * 400
  local day_of_year = day_of_era - (365 * year_of_era + math.floor(year_of_era / 4) - math.floor(year_of_era / 100))
  local month_position = math.floor((5 * day_of_year + 2) / 153)
  local day = day_of_year - math.floor((153 * month_position + 2) / 5) + 1
  local month
  if month_position < 10 then
    month = month_position + 3
  else
    month = month_position - 9
  end
  if month <= 2 then
    year = year + 1
  end
  return year, month, day
end

local function at_now()
  local seconds = os.time()
  local days = math.floor(seconds / 86400)
  local second_of_day = seconds - days * 86400
  local year, month, day = civil_from_days(days)
  return string.format(
    "%04d-%02d-%02dT%02d:%02d:%02d.000Z",
    year,
    month,
    day,
    math.floor(second_of_day / 3600),
    math.floor((second_of_day % 3600) / 60),
    second_of_day % 60
  )
end

-- One measured span, in milliseconds, from the reporter's own monotonic stamp.
local function measured(started)
  if not started then
    return nil
  end
  return math.floor((os.clock() - started) * 1000)
end

-- The bounded subject label of a tool call: one line, at most 120 characters,
-- and never the whole input. A call that ran carries `data.input`, so the
-- harness's own subject is what the record keeps; a cancelled call has no
-- input and is labelled with the tool name.
local function tool_label(data)
  local input = data.input
  local subject
  if type(input) == "table" then
    subject = input.command or input.file_path or input.path or input.pattern or input.query
  end
  if type(subject) ~= "string" or subject == "" then
    subject = data.tool or "tool"
  end
  local line = subject:match("^[^\n]*") or ""
  if #line > LABEL_LIMIT then
    line = string.sub(line, 1, LABEL_LIMIT - 1) .. "\226\128\166"
  end
  return line
end

-- Maki's `TurnEnd` usage object as the intake's `usage`, defensively: the four
-- token fields are read under the names the host uses, and no count is invented
-- (an absent usage stays absent, never zero).
local function usage_of(usage)
  if type(usage) ~= "table" then
    return nil
  end
  local input = usage.input_tokens or usage.input
  local output = usage.output_tokens or usage.output
  local cache_read = usage.cache_read_tokens or usage.cache_read or usage.cache_read_input_tokens
  local cache_write = usage.cache_write_tokens or usage.cache_write or usage.cache_creation_input_tokens
  if type(input) ~= "number" and type(output) ~= "number" and type(cache_read) ~= "number" and type(cache_write) ~= "number" then
    return nil
  end
  input = input or 0
  output = output or 0
  cache_read = cache_read or 0
  cache_write = cache_write or 0
  return {
    input = input,
    output = output,
    cache_read = cache_read,
    cache_write = cache_write,
    total_tokens = input + output + cache_read + cache_write,
  }
end

-- The model `ModelChanged` names, in whatever shape the host hands it over.
local function model_name(model)
  if type(model) == "string" then
    return model
  end
  if type(model) == "table" then
    return model.id or model.name or model.spec or model.model
  end
  return nil
end

local function logged_once(kind, message)
  if logged[kind] then
    return
  end
  logged[kind] = true
  pcall(function()
    maki.log.error("givn-timeline: " .. message)
  end)
end

local function push(event)
  if #buffer >= MAX_BUFFER then
    dropped = dropped + 1
    if not warned then
      warned = true
      logged_once("buffer", "buffer full (" .. MAX_BUFFER .. "); dropped events are counted")
    end
    return
  end
  buffer[#buffer + 1] = event
end

-- The payload file's path: Maki's own state directory, so nothing lands in the
-- project, with the working directory as the fallback.
local function payload_path()
  local dir = nil
  local ok, value = pcall(function()
    return maki.env.state_dir()
  end)
  if ok and type(value) == "string" and value ~= "" then
    dir = value
  else
    local cwd_ok, cwd = pcall(function()
      return maki.uv.cwd()
    end)
    if cwd_ok then
      dir = cwd
    end
  end
  return string.format("%s/givn-timeline-%d-%d.json", dir or ".", os.time(), math.floor(os.clock() * 1000))
end

-- One flush: one payload per boundary, delivered through the real intake with a
-- job awaited inside this callback. `timeout_ms` is the teardown grace when the
-- flush runs from `SessionEnd`.
local function flush(timeout_ms)
  if #buffer == 0 then
    return
  end
  local events = buffer
  buffer = {}
  local ok, err = pcall(function()
    local text, encode_err = maki.json.encode({
      schema = SCHEMA,
      reporter = HARNESS,
      session = last_session,
      events = events,
    })
    if not text then
      error(encode_err)
    end
    local path = payload_path()
    local written, write_err = maki.fs.write(path, text)
    if not written then
      error(write_err)
    end
    local job, job_err = maki.fn.jobstart({ "sh", "-c", "cat '" .. path .. "' | givn history ingest" }, { scope = "plugin" })
    if not job then
      maki.fs.rm(path)
      error(job_err)
    end
    local result, wait_err = maki.fn.jobwait(job, timeout_ms or JOB_TIMEOUT_MS)
    maki.fs.rm(path)
    if not result then
      pcall(function()
        maki.fn.jobstop(job)
      end)
      error(wait_err or "the intake helper did not finish within its bounded wait")
    end
  end)
  if not ok then
    dropped = dropped + #events
    logged_once("flush", "the intake could not be reached: " .. tostring(err))
  end
end

-- The project's capture mode: one subprocess query at the first event
-- callback, cached for the process. A failed or malformed query degrades to
-- "on" — metadata only — and never blocks the harness's own work.
local mode = nil

local function record_mode()
  if mode ~= nil then
    return mode
  end
  local resolved = nil
  local ok = pcall(function()
    local path = payload_path() .. ".mode"
    local job, job_err = maki.fn.jobstart(
      { "sh", "-c", "givn history settings --json > '" .. path .. "' 2>/dev/null" },
      { scope = "plugin" }
    )
    if not job then
      error(job_err)
    end
    local result, wait_err = maki.fn.jobwait(job, JOB_TIMEOUT_MS)
    if not result then
      pcall(function()
        maki.fn.jobstop(job)
      end)
      maki.fs.rm(path)
      error(wait_err or "the settings helper did not finish within its bounded wait")
    end
    local text = maki.fs.read(path)
    maki.fs.rm(path)
    if type(text) ~= "string" then
      error("no settings output")
    end
    local decoded, decode_err = maki.json.decode(text)
    if type(decoded) ~= "table" then
      error(decode_err)
    end
    if decoded.mode == "off" or decoded.mode == "on" or decoded.mode == "full" then
      resolved = decoded.mode
    else
      error("the settings query reported no known mode")
    end
  end)
  if not ok or resolved == nil then
    logged_once("mode", "the record mode could not be read; treating it as on")
    resolved = "on"
  end
  mode = resolved
  return mode
end

-- The live transcript's last blocking `question` tool call, joined into the
-- always-captured text. `maki.session.messages` is the live transcript and
-- needs no permission beyond the reporter's existing grants; a transcript with
-- no matching call (a permission prompt, plan form, or Lua float also produces
-- `needs_input`) yields no text, never an invented one.
local function transcript_question()
  local ok, messages = pcall(function()
    return maki.session.messages({ last = 50 })
  end)
  if not ok or type(messages) ~= "table" then
    return nil
  end
  local questions = nil
  for _, message in ipairs(messages) do
    if type(message) == "table" then
      local parts = {}
      local content = message.content
      if type(content) == "table" then
        if content.type ~= nil then
          parts[#parts + 1] = content
        else
          for _, part in ipairs(content) do
            parts[#parts + 1] = part
          end
        end
      end
      if type(message.tool_calls) == "table" then
        for _, call in ipairs(message.tool_calls) do
          parts[#parts + 1] = call
        end
      end
      for _, part in ipairs(parts) do
        if
          type(part) == "table"
          and part.type == "tool_use"
          and part.name == "question"
          and type(part.input) == "table"
          and type(part.input.questions) == "table"
          and #part.input.questions > 0
        then
          questions = part.input.questions
        end
      end
    end
  end
  if not questions then
    return nil
  end
  local joined = {}
  for _, question in ipairs(questions) do
    if type(question) == "table" then
      if type(question.question) == "string" then
        joined[#joined + 1] = question.question
      end
      if type(question.options) == "table" then
        for _, option in ipairs(question.options) do
          local label = type(option) == "table" and option.label or option
          if type(label) == "string" then
            joined[#joined + 1] = label
          end
        end
      end
    end
  end
  if #joined == 0 then
    return nil
  end
  return table.concat(joined, "\n")
end

-- The last assistant message's text and thinking from the live transcript.
-- The maki event stream does not carry the assistant message itself; the
-- transcript is the only source for turn content.
local function transcript_turn()
  local ok, messages = pcall(function()
    return maki.session.messages({ last = 50 })
  end)
  if not ok or type(messages) ~= "table" then
    return nil, nil
  end
  for i = #messages, 1, -1 do
    local message = messages[i]
    if type(message) == "table" and (message.role == "assistant" or message.role == "model") then
      local parts = {}
      local content = message.content
      if type(content) == "string" then
        parts[#parts + 1] = { type = "text", text = content }
      elseif type(content) == "table" then
        if content.type ~= nil then
          parts[#parts + 1] = content
        else
          for _, part in ipairs(content) do
            parts[#parts + 1] = part
          end
        end
      end
      local assistant, thinking = {}, {}
      for _, part in ipairs(parts) do
        if type(part) == "table" then
          if part.type == "text" and type(part.text) == "string" then
            assistant[#assistant + 1] = part.text
          elseif part.type == "thinking" or part.type == "reasoning" then
            local text = part.thinking or part.reasoning or part.text
            if type(text) == "string" then
              thinking[#thinking + 1] = text
            end
          end
        end
      end
      return
        #assistant > 0 and table.concat(assistant, "\n") or nil,
        #thinking > 0 and table.concat(thinking, "\n") or nil
    end
  end
  return nil, nil
end

-- The last tool message's text from the live transcript. The ToolDone event
-- carries no output on the maki event stream; the transcript does.
local function transcript_tool_output(tool_id)
  local ok, messages = pcall(function()
    return maki.session.messages({ last = 50 })
  end)
  if not ok or type(messages) ~= "table" then
    return nil
  end
  for i = #messages, 1, -1 do
    local message = messages[i]
    if type(message) == "table" and message.role == "tool" then
      if tool_id ~= nil and message.tool_call_id ~= nil and message.tool_call_id ~= tool_id then
        -- another call's result; keep looking
      else
        local content = message.content
        if type(content) == "string" then
          return content
        end
        if type(content) == "table" then
          local parts = content.type ~= nil and { content } or content
          local text = {}
          for _, part in ipairs(parts) do
            if type(part) == "string" then
              text[#text + 1] = part
            elseif type(part) == "table" then
              if type(part.text) == "string" then
                text[#text + 1] = part.text
              elseif type(part.content) == "string" then
                text[#text + 1] = part.content
              end
            end
          end
          if #text > 0 then
            return table.concat(text, "\n")
          end
        end
        return nil
      end
    end
  end
  return nil
end

-- One content field on the wire; the intake re-applies the content caps.
local function content_field(text)
  if type(text) ~= "string" or text == "" then
    return nil
  end
  return { text = text, truncated = false, bytes = #text }
end

-- A value as content text: a string as-is, anything else JSON-encoded.
local function content_text(value)
  if type(value) == "string" then
    return value
  end
  if value == nil then
    return nil
  end
  local ok, text = pcall(function()
    return maki.json.encode(value)
  end)
  if ok and type(text) == "string" then
    return text
  end
  return nil
end

-- Whether this project captures the complete content set.
local function captures_content()
  return record_mode() == "full"
end

-- The callback table, exported so a test driver can drive a handler directly;
-- the host reaches the same table through `setup()`.
local handlers = {}

handlers.ToolStart = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  state.tools[data.tool_id or data.tool or "tool"] = { started = os.clock(), tool = data.tool }
end

handlers.ToolDone = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local key = data.tool_id or data.tool or "tool"
  local start = state.tools[key]
  state.tools[key] = nil
  local event = {
    kind = "tool",
    at = at_now(),
    tool = data.tool or (start and start.tool) or "tool",
    label = tool_label(data),
    outcome = data.is_error and "failed" or "ok",
  }
  -- The host's own duration where the call ran and it measured one, the
  -- reporter's stamps where the call ran without one, and nothing at all for a
  -- call that never ran (no duration and no input): a cancelled call is
  -- recorded without a duration, never as zero.
  local duration = nil
  if data.duration_ms ~= nil then
    duration = data.duration_ms
  elseif data.input ~= nil then
    duration = measured(start and start.started)
  end
  if duration ~= nil then
    event.duration_ms = duration
    event.duration_source = data.duration_ms ~= nil and "host" or "reporter"
  end
  event.tool_call_id = data.tool_id
  event.output_bytes = data.bytes
  if captures_content() then
    local content = {}
    local input = content_field(content_text(data.input))
    if input then
      content.tool_input = input
    end
    local output = content_field(content_text(data.output or data.result or data.content))
    if output == nil then
      output = content_field(transcript_tool_output(data.tool_id))
    end
    if output then
      content.tool_output = output
    end
    if next(content) ~= nil then
      event.content = content
    end
  end
  push(event)
  local tool_name = data.tool or (start and start.tool) or "tool"
  if tool_name == "task" or tool_name == "subagent" then
    local input = data.input
    local title = nil
    if type(input) == "table" then
      if type(input.description) == "string" then
        title = input.description
      elseif type(input.prompt) == "string" then
        title = input.prompt
      end
    end
    push({
      kind = "subagent",
      at = at_now(),
      agent = "task",
      subagent = tostring(data.tool_id or tool_name),
      title = title,
      status = data.is_error and "failed" or "completed",
      duration_ms = duration,
      outcome = data.is_error and "failed" or "ok",
    })
  end
end

handlers.TurnStart = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  -- data.text is the prompt that started the turn: stamped, and stored as
  -- content only when the project captures content.
  state.turn_started = os.clock()
  state.prompt = captures_content() and data.text or nil
end

handlers.TurnEnd = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local reason = data.reason
  local outcome = "unknown"
  if reason == "finished" then
    outcome = "completed"
  elseif reason == "cancelled" or reason == "dropped" then
    outcome = "failed"
  end
  local event = {
    kind = "turn",
    at = at_now(),
    turn = data.num_turns,
    status = outcome,
    stop_reason = data.reason,
    model = state.model,
    usage = usage_of(data.usage),
    cost = data.cost,
    list_cost = data.list_cost,
    context_size = data.context_size,
    context_window = data.context_window,
  }
  -- A turn with no TurnStart (a headless or ACP session) is recorded
  -- unmeasured rather than invented.
  event.duration_ms = measured(state.turn_started)
  if captures_content() then
    local content = {}
    local prompt = content_field(state.prompt)
    if prompt then
      content.user_prompt = prompt
    end
    local assistant, thinking = transcript_turn()
    local assistant_field = content_field(assistant)
    if assistant_field then
      content.assistant_text = assistant_field
    end
    local thinking_field = content_field(thinking)
    if thinking_field then
      content.thinking_text = thinking_field
    end
    if next(content) ~= nil then
      event.content = content
    end
  end
  state.turn_started = nil
  state.prompt = nil
  push(event)
  flush(nil)
end

handlers.TurnError = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local event = {
    kind = "turn",
    at = at_now(),
    status = "failed",
    stop_reason = data.reason,
    error = data.error,
    model = state.model,
  }
  event.duration_ms = measured(state.turn_started)
  state.turn_started = nil
  push(event)
  flush(nil)
end

handlers.AutoCompacting = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  state.compaction_started = os.clock()
  push({ kind = "phase", at = at_now(), label = "compacting" })
end

handlers.CompactionDone = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  -- data.summary replaced the history: the phase is recorded, the content is
  -- never stored.
  local event = { kind = "phase", at = at_now(), label = "compacted", outcome = "ok" }
  event.duration_ms = measured(state.compaction_started)
  state.compaction_started = nil
  if captures_content() then
    local summary = content_field(data.summary)
    if summary then
      event.content = { compaction_summary = summary }
    end
  end
  push(event)
end

handlers.SessionStatusChanged = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  if data.status == "needs_input" then
    state.wait_started = os.clock()
    local label = data.title
    if type(label) ~= "string" or label == "" then
      label = "needs input"
    end
    state.wait_label = label:match("^[^\n]*") or "needs input"
    state.wait_title = label
    return
  end
  if data.status == "working" and state.wait_started then
    -- At the wait's close the live transcript names the blocking question
    -- tool call; a transcript without one keeps the bounded title only.
    local text = transcript_question()
    local event = {
      kind = "question",
      at = at_now(),
      label = state.wait_label or "needs input",
      title = state.wait_title,
      duration_ms = measured(state.wait_started),
      status = "completed",
    }
    if text then
      event.question = { text = text, truncated = false, bytes = #text }
    end
    state.wait_started = nil
    state.wait_label = nil
    state.wait_title = nil
    push(event)
  end
end

handlers.TaskStatusChanged = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local key = data.id or data.name or "task"
  if data.status == "working" then
    state.tasks[key] = { started = os.clock(), name = data.name }
    return
  end
  local start = state.tasks[key]
  state.tasks[key] = nil
  local event = {
    kind = "subagent",
    at = at_now(),
    subagent = data.id or data.name or "task",
    agent = data.name or (start and start.name),
    status = data.status == "done" and "completed" or "failed",
  }
  event.duration_ms = measured(start and start.started)
  push(event)
end

handlers.ModelChanged = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local name = model_name(data.model)
  if name then
    state.model = name
  end
end

handlers.SessionEnd = function(ev)
  local data = ev.data or {}
  local timeout = data.deadline_ms
  if type(timeout) ~= "number" or timeout <= 0 then
    timeout = nil
  end
  flush(timeout)
  sessions[data.session_id or "maki"] = nil
end

handlers.SessionReset = function(ev)
  local data = ev.data or {}
  flush(nil)
  sessions[data.session_id or "maki"] = nil
end

M.handlers = handlers

-- Register this reporter's autocmds. Called from the managed `init.lua` fence,
-- so it must not perform an asynchronous call at load time and must never raise
-- into the loader.
function M.setup()
  local events = {
    "ToolStart",
    "ToolDone",
    "TurnStart",
    "TurnEnd",
    "TurnError",
    "AutoCompacting",
    "CompactionDone",
    "SessionStatusChanged",
    "TaskStatusChanged",
    "ModelChanged",
    "SessionEnd",
    "SessionReset",
  }
  for _, event in ipairs(events) do
    local ok, err = pcall(function()
      maki.api.create_autocmd(event, {
        callback = function(ev)
          if record_mode() == "off" then
            return
          end
          if ev and ev.data and ev.data.session_id then
            last_session = ev.data.session_id
          end
          local handler_ok, handler_err = pcall(handlers[event], ev or { data = {} })
          if not handler_ok then
            logged_once("handler:" .. event, "handler " .. event .. " failed: " .. tostring(handler_err))
          end
        end,
      })
    end)
    if not ok then
      logged_once("register:" .. event, "registering " .. event .. " failed: " .. tostring(err))
    end
  end
end

-- Flush the buffered events now: the explicit entry point a loader or a test
-- driver calls after firing scripted events.
function M.run()
  flush(nil)
end

return M
