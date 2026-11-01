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
      title = nil,
      tools = {},
      turn_started = nil,
      wait_started = nil,
      wait_label = nil,
      compaction_started = nil,
      tasks = {},
      -- Task ids whose span was already closed: a task's terminal status, its
      -- stop, and its tool completion can arrive in any order, and one
      -- subagent gets one entry.
      closed = {},
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

-- One transcript read, shared by every helper below: the live transcript is
-- the only source for turn content, tool output, question answers, and the
-- receipts that name a moved call. A session id reads that session (a
-- subagent's own transcript); no id reads the focused one, exactly as before.
local function transcript_messages(last, session_id)
  local ok, messages = pcall(function()
    if session_id ~= nil then
      return maki.session.messages({ session = session_id, last = last or 50 })
    end
    return maki.session.messages({ last = last or 50 })
  end)
  if not ok or type(messages) ~= "table" then
    return nil
  end
  return messages
end

-- One message's blocks. Maki 0.7.0 returns `content = { blocks }`; a lone block
-- and a string content are tolerated so an older or partial shape degrades to
-- the same reading rather than dropping the message.
local function blocks_of(message)
  if type(message) ~= "table" then
    return {}
  end
  local content = message.content
  if type(content) == "string" then
    return { { type = "text", text = content } }
  end
  if type(content) ~= "table" then
    return {}
  end
  if content.type ~= nil then
    return { content }
  end
  return content
end

-- One block's text: a string content as-is, otherwise the concatenated text of
-- its text blocks. A tool result's content is a string in 0.7.0.
local function block_text(content)
  if type(content) == "string" then
    return content
  end
  if type(content) ~= "table" then
    return nil
  end
  local parts = content.type ~= nil and { content } or content
  local text = {}
  for _, part in ipairs(parts) do
    if type(part) == "string" then
      text[#text + 1] = part
    elseif type(part) == "table" and type(part.text) == "string" then
      text[#text + 1] = part.text
    end
  end
  if #text == 0 then
    return nil
  end
  return table.concat(text, "\n")
end

-- The live transcript's last blocking `question` tool call, joined into the
-- always-captured text, and its answer when the transcript already holds the
-- matching result. `maki.session.messages` is the live transcript and now
-- needs the `session_read` grant; a transcript with no matching call (a
-- permission prompt, plan form, or Lua float also produces `needs_input`)
-- yields no text, never an invented one.
local function transcript_question()
  local messages = transcript_messages(50)
  if not messages then
    return nil, nil
  end
  local question_id = nil
  local questions = nil
  local results = {}
  for _, message in ipairs(messages) do
    for _, part in ipairs(blocks_of(message)) do
      if type(part) == "table" then
        if part.type == "tool_use" and part.name == "question" then
          if type(part.input) == "table" and type(part.input.questions) == "table" and #part.input.questions > 0 then
            question_id = type(part.id) == "string" and part.id or nil
            questions = part.input.questions
          end
        elseif part.type == "tool_result" and type(part.tool_use_id) == "string" then
          results[part.tool_use_id] = block_text(part.content)
        end
      end
    end
  end
  if not questions then
    return nil, nil
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
  local answer = question_id and results[question_id] or nil
  if #joined == 0 then
    return nil, answer
  end
  return table.concat(joined, "\n"), answer
end

-- The last assistant message's text and thinking from the live transcript.
-- The maki event stream does not carry the assistant message itself; the
-- transcript is the only source for turn content. `messages` lets the caller
-- read the transcript once for the turn and its moved-call receipts.
local function transcript_turn(messages)
  messages = messages or transcript_messages(50)
  if not messages then
    return nil, nil
  end
  for i = #messages, 1, -1 do
    local message = messages[i]
    if type(message) == "table" and (message.role == "assistant" or message.role == "model") then
      local assistant, thinking = {}, {}
      for _, part in ipairs(blocks_of(message)) do
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

-- The tool result's text from the live transcript, by the harness's own call
-- id. The ToolDone event carries no output on the maki event stream; the
-- transcript does, as a `tool_result` block in a user message. Maki has never
-- emitted a `role = "tool"` message, so the lookup reads blocks, not roles.
local function transcript_tool_output(tool_id, messages)
  messages = messages or transcript_messages(50)
  if not messages then
    return nil
  end
  for i = #messages, 1, -1 do
    for _, part in ipairs(blocks_of(messages[i])) do
      if type(part) == "table" and part.type == "tool_result" then
        if tool_id == nil or part.tool_use_id == nil or part.tool_use_id == tool_id then
          return block_text(part.content)
        end
      end
    end
  end
  return nil
end

-- The call ids the transcript says moved to the background: Maki writes them
-- on the message carrying the call's receipt, and the final result arrives as
-- a later message without a second ToolDone event.
local function transcript_receipts(messages)
  if not messages then
    return nil
  end
  local moved = nil
  for _, message in ipairs(messages) do
    if type(message) == "table" and type(message.receipts) == "table" then
      for _, id in ipairs(message.receipts) do
        if type(id) == "string" then
          moved = moved or {}
          moved[id] = true
        end
      end
    end
  end
  return moved
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

-- The task list is Maki's own view of the session's subagents: a tool-spawned
-- task has no `plugin`, a plugin task has one, and the entry names the task's
-- chat session and (where the host reports it) its cost. Every call is
-- best-effort: a host without an interactive UI, or a task already gone, simply
-- degrades to no session and no cost.
local function task_list()
  local ok, tasks = pcall(function()
    return maki.task.list()
  end)
  if not ok or type(tasks) ~= "table" then
    return nil
  end
  return tasks
end

local function task_info(id)
  local tasks = task_list()
  if not tasks then
    return nil
  end
  for _, task in ipairs(tasks) do
    if type(task) == "table" and task.id == id then
      return {
        session = type(task.session) == "string" and task.session or nil,
        cost = type(task.cost) == "number" and task.cost or nil,
        name = type(task.name) == "string" and task.name or nil,
        status = type(task.status) == "string" and task.status or nil,
      }
    end
  end
  return nil
end

-- The record outcome a task status reports: a stopped task did not fail, so it
-- is skipped; a finished task is completed; everything else is failed.
local function task_outcome(status)
  if status == "done" then
    return "completed"
  end
  if status == "cancelled" or status == "idle" then
    return "skipped"
  end
  return "failed"
end

-- The record status word for a span close, kept as the host's own word where
-- the record knows it and normalised only for a finished task.
local function task_status(status)
  if status == "done" then
    return "completed"
  end
  if status == "cancelled" then
    return "cancelled"
  end
  if status == "error" then
    return "failed"
  end
  return status
end

-- The tool calls a subagent's own transcript holds, emitted once under that
-- subagent's session so the report can give it its own lane. A call without a
-- result is not invented; content follows the project's capture mode, and the
-- count is bounded so a long subagent cannot flood one flush.
local MAX_LANE_TOOLS = 100
local LANE_BYTE_CAP = 262144
local emitted_lanes = {}

-- Whether the buffer already carries this call under this session: a host that
-- also delivers the subagent's own tool events must not get a second entry for
-- the same call from the transcript.
local function buffered_tool_event(session, id)
  if id == nil then
    return false
  end
  for _, event in ipairs(buffer) do
    if event.kind == "tool" and event.session == session and event.tool_call_id == id then
      return true
    end
  end
  return false
end

local function push_subagent_tools(session)
  if type(session) ~= "string" or session == "" or emitted_lanes[session] then
    return
  end
  emitted_lanes[session] = true
  local messages = transcript_messages(200, session)
  if not messages then
    return
  end
  local results = {}
  for _, message in ipairs(messages) do
    for _, part in ipairs(blocks_of(message)) do
      if type(part) == "table" and part.type == "tool_result" and type(part.tool_use_id) == "string" then
        results[part.tool_use_id] = part
      end
    end
  end
  local emitted = 0
  local bytes = 0
  local seen = {}
  for _, message in ipairs(messages) do
    for _, part in ipairs(blocks_of(message)) do
      if emitted >= MAX_LANE_TOOLS or bytes >= LANE_BYTE_CAP then
        return
      end
      if type(part) == "table" and part.type == "tool_use" then
        local id = type(part.id) == "string" and part.id or nil
        local result = id and results[id] or nil
        if result and not (id and seen[id]) and not buffered_tool_event(session, id) then
          if id then
            seen[id] = true
          end
          local text = block_text(result.content)
          if type(text) == "string" then
            bytes = bytes + #text
          end
          local event = {
            kind = "tool",
            at = at_now(),
            tool = type(part.name) == "string" and part.name or "tool",
            label = tool_label({ tool = part.name, input = part.input }),
            tool_call_id = id,
            session = session,
            outcome = result.is_error and "failed" or "ok",
          }
          if type(text) == "string" then
            event.output_bytes = #text
          end
          if captures_content() then
            local content = {}
            local input = content_field(content_text(part.input))
            if input then
              content.tool_input = input
            end
            local output = content_field(text)
            if output then
              content.tool_output = output
            end
            if next(content) ~= nil then
              event.content = content
            end
          end
          push(event)
          emitted = emitted + 1
        end
      end
    end
  end
end

-- Close the tasks still open when their session ends: the outcome is unknown,
-- and the elapsed time observed for them is kept rather than invented as a
-- completion or dropped with the process.
local function close_open_tasks(state)
  local keys = {}
  for key in pairs(state.tasks) do
    keys[#keys + 1] = key
  end
  table.sort(keys)
  for _, key in ipairs(keys) do
    local task = state.tasks[key]
    local info = task_info(key)
    local lane = (info and info.session) or (task and task.session)
    local event = {
      kind = "subagent",
      at = at_now(),
      subagent = key,
      agent = task and task.name,
      status = "unknown",
      outcome = "unknown",
      duration_ms = measured(task and task.started),
    }
    if lane then
      event.session = lane
    end
    local cost = (info and info.cost) or (task and task.cost)
    if cost ~= nil then
      event.usage = { cost = cost }
    end
    push(event)
    if lane then
      push_subagent_tools(lane)
    end
  end
  state.tasks = {}
end

-- Mark the buffered tool events whose call the transcript says moved to the
-- background: one completion at move time is not the call's outcome.
local function mark_backgrounded(messages)
  local moved = transcript_receipts(messages)
  if not moved then
    return
  end
  for _, event in ipairs(buffer) do
    if event.kind == "tool" and type(event.tool_call_id) == "string" and moved[event.tool_call_id] then
      event.outcome = "unknown"
      event.backgrounded = true
    end
  end
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
    local task = state.tasks[key]
    local info = task_info(key)
    if info and (info.status == "working" or info.status == "idle") then
      -- The task is still listed after its tool call returned: the call moved
      -- the subagent to the background, so this completion is not its end. Keep
      -- the span open, take what the completion knows, and close on the task's
      -- terminal status or the session's end.
      task = task or { started = os.clock() }
      task.name = task.name or info.name
      task.session = info.session or task.session
      task.title = title or task.title
      task.tool_duration = duration or task.tool_duration
      task.tool_outcome = data.is_error and "failed" or "ok"
      task.tool_status = data.is_error and "failed" or "completed"
      state.tasks[key] = task
      return
    end
    if state.closed[key] then
      return
    end
    state.closed[key] = true
    state.tasks[key] = nil
    local lane = (info and info.session) or (task and task.session)
    local final_outcome = data.is_error and "failed" or (task and task.outcome) or "ok"
    local final_status = data.is_error and "failed"
      or (task and task.status and task_status(task.status))
      or "completed"
    local subagent = {
      kind = "subagent",
      at = at_now(),
      agent = (task and task.name) or "task",
      subagent = tostring(data.tool_id or key),
      title = (task and task.title) or title,
      status = final_status,
      duration_ms = duration or (task and task.tool_duration),
      outcome = final_outcome,
    }
    if lane then
      -- The span lives in the subagent's own session so the report can name
      -- its lane; the parent's lane keeps the tool call that spawned it.
      subagent.session = lane
    end
    local cost = (info and info.cost) or (task and task.cost)
    if cost ~= nil then
      subagent.usage = { cost = cost }
    end
    push(subagent)
    if lane then
      push_subagent_tools(lane)
    end
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
  -- unmeasured rather than invented. One transcript read at the turn boundary
  -- serves both the turn's content and the moved-call receipts: a call that
  -- moved to the background has no final completion event.
  event.duration_ms = measured(state.turn_started)
  local messages = transcript_messages(50)
  mark_backgrounded(messages)
  if captures_content() then
    local content = {}
    local prompt = content_field(state.prompt)
    if prompt then
      content.user_prompt = prompt
    end
    local assistant, thinking = transcript_turn(messages)
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
    -- tool call and, once answered, its result block; a transcript without
    -- one keeps the bounded title only.
    local text, answer = transcript_question()
    local event = {
      kind = "question",
      at = at_now(),
      label = state.wait_label or "needs input",
      title = state.wait_title,
      duration_ms = measured(state.wait_started),
      status = "completed",
    }
    if text then
      event.question = content_field(text)
    end
    if answer then
      event.answer = content_field(answer)
      event.answered_by = "user"
      event.answered_at = at_now()
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
    local task = state.tasks[key] or { started = os.clock() }
    task.name = data.name or task.name
    task.plugin = data.plugin or task.plugin
    local info = task_info(key)
    if info then
      task.session = info.session or task.session
    end
    state.tasks[key] = task
    return
  end
  if data.status == "idle" then
    -- A plugin-owned task between prompts: it has not ended, so its span
    -- stays open until a terminal status or the session's end.
    return
  end
  local task = state.tasks[key] or { started = os.clock() }
  task.name = data.name or task.name
  task.status = data.status
  task.outcome = task_outcome(data.status)
  if state.tools[key] ~= nil and not state.closed[key] then
    -- The spawning tool call has not completed yet: its completion carries the
    -- title and the host duration, so remember this terminal status and close
    -- there. A backgrounded subagent has no pending tool call here and closes
    -- now.
    state.tasks[key] = task
    return
  end
  state.tasks[key] = nil
  if state.closed[key] then
    return
  end
  state.closed[key] = true
  local info = task_info(key)
  local lane = (info and info.session) or task.session
  local event = {
    kind = "subagent",
    at = at_now(),
    subagent = key,
    agent = task.name,
    title = task.title,
    status = task_status(task.status),
    outcome = task.outcome,
  }
  if lane then
    event.session = lane
  end
  event.duration_ms = task.tool_duration or measured(task.started)
  local cost = (info and info.cost) or task.cost
  if cost ~= nil then
    event.usage = { cost = cost }
  end
  push(event)
  if lane then
    push_subagent_tools(lane)
  end
end

-- A user stop. A plugin-owned task has no tool completion to close it; a
-- tool-spawned task is closed by its own (cancelled) tool completion.
handlers.TaskStopped = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local key = data.id or "task"
  local task = state.tasks[key] or { started = os.clock() }
  state.tasks[key] = nil
  if state.closed[key] then
    return
  end
  state.closed[key] = true
  local info = task_info(key)
  local lane = (info and info.session) or task.session
  local event = {
    kind = "subagent",
    at = at_now(),
    subagent = key,
    agent = task.name,
    title = task.title,
    status = "cancelled",
    outcome = "skipped",
    duration_ms = task.tool_duration or measured(task.started),
  }
  if lane then
    event.session = lane
  end
  local cost = (info and info.cost) or (task and task.cost)
  if cost ~= nil then
    event.usage = { cost = cost }
  end
  push(event)
  if lane then
    push_subagent_tools(lane)
  end
end

-- A session title is a point entry: the report's lane naming reads it, and a
-- session without one keeps its short id.
handlers.SessionTitleChanged = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  local title = data.title
  if type(title) ~= "string" then
    return
  end
  title = title:match("^[^\n]*") or ""
  if title == "" or title == state.title then
    return
  end
  state.title = title
  push({
    kind = "session",
    at = at_now(),
    session = data.session_id,
    title = title,
  })
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
  local state = session_state(data.session_id)
  close_open_tasks(state)
  local timeout = data.deadline_ms
  if type(timeout) ~= "number" or timeout <= 0 then
    timeout = nil
  end
  flush(timeout)
  sessions[data.session_id or "maki"] = nil
end

handlers.SessionReset = function(ev)
  local data = ev.data or {}
  local state = session_state(data.session_id)
  close_open_tasks(state)
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
    "SessionTitleChanged",
    "TaskStatusChanged",
    "TaskStopped",
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
