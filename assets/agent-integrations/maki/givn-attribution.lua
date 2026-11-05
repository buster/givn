-- Givn commit attribution for the Maki coding agent.
--
-- Maki loads this module through the managed `-- givn:begin:attribution`
-- fence in `.maki/init.lua`. Maki fires `tool.bash.input` before a Bash call is
-- parsed or checked; the layer reads `maki.model.get()` for the focused
-- session's model and thinking level, then rewrites only a command whose text
-- names a revision verb. The prefix is the same environment protocol the
-- OpenCode, omp, and Claude integrations use; the checked-in commit-msg hook
-- consumes it.
--
-- Maki 0.7.0 passes a plain table to this layer, not the `LuaCtx` handle that
-- `maki.agent.resolve_model` requires, so a subagent command names the focused
-- session. `maki.model.get` answers through the UI event loop, so a headless
-- driver (`maki acp`, `maki -p`) gets nil and no error pair; the layer then
-- reads the focused session snapshot, whose `model` field is always present.
-- Thinking is absent from that snapshot under headless drivers, so reasoning
-- stays `unavailable` there. The layer stays best effort: a missing model
-- leaves `unavailable` facts and no committer override; a non-revision command
-- is untouched; a host without the slot API loads this module inert. Maki
-- requires `run`, `session_read`, and `session_write` for a `tool.bash.input`
-- layer; the generated manifest grants them even though this module never
-- writes session content.

local M = {}
local registered = false

local HARNESS = "maki"
local UNAVAILABLE = "unavailable"
local REVISION_VERB = "%f[%w]commit%f[%W]"

local function shell_quote(value)
  return "'" .. tostring(value):gsub("'", "'\"'\"'") .. "'"
end

local function identity_of(model)
  if type(model) ~= "table" then
    return nil
  end
  local spec = model.spec
  if type(spec) == "string" and spec ~= "" then
    return spec
  end
  local id = model.id
  if type(id) ~= "string" or id == "" then
    return nil
  end
  local provider = model.provider
  if type(provider) == "string" and provider ~= "" then
    return provider .. "/" .. id
  end
  return id
end

local function reasoning_of(model)
  if type(model) ~= "table" then
    return UNAVAILABLE
  end
  local value = model.thinking_shown or model.thinking
  if type(value) == "string" and value ~= "" then
    return value
  end
  return UNAVAILABLE
end

local function ui_model()
  if type(maki) ~= "table" or type(maki.model) ~= "table" or type(maki.model.get) ~= "function" then
    return nil
  end
  local ok, model = pcall(maki.model.get)
  if ok and type(model) == "table" then
    return model
  end
  return nil
end

-- The snapshot a headless driver answers with. Its `model` is a spec string and
-- `thinking` is absent, so only the identity survives.
local function session_model()
  if type(maki) ~= "table" or type(maki.session) ~= "table" or type(maki.session.read) ~= "function" then
    return nil
  end
  local ok, session = pcall(maki.session.read)
  if not ok or type(session) ~= "table" then
    return nil
  end
  local spec = session.model
  if type(spec) ~= "string" or spec == "" then
    return nil
  end
  return { spec = spec, thinking = session.thinking }
end

local function current_model()
  return ui_model() or session_model()
end

local function environment(identity, reasoning)
  local values = {
    "GIVN_ATTRIBUTION_ACTIVE=1",
    "GIVN_ATTRIBUTION_LLM=" .. shell_quote(identity),
    "GIVN_ATTRIBUTION_REASONING=" .. shell_quote(reasoning),
    "GIVN_ATTRIBUTION_HARNESS=" .. shell_quote(HARNESS),
  }
  if identity ~= UNAVAILABLE then
    values[#values + 1] = "GIT_COMMITTER_NAME=" .. shell_quote("LLM: " .. identity .. " via " .. HARNESS)
  end
  return table.concat(values, " ")
end

function M.prepare(command)
  if type(command) ~= "string" or not command:find(REVISION_VERB) then
    return command
  end
  local model = current_model()
  local identity = identity_of(model) or UNAVAILABLE
  local reasoning = reasoning_of(model)
  return "export " .. environment(identity, reasoning) .. "; " .. command
end

function M.setup()
  if type(maki) ~= "table" or type(maki.api) ~= "table" or type(maki.api.set_slot) ~= "function" then
    return
  end
  if registered then
    return
  end
  local function rewrite(prev, value, ctx)
    if type(prev) ~= "function" then
      return value
    end
    local next_value, extra = prev(value, ctx)
    if next_value == nil and extra ~= nil then
      return nil, extra
    end
    local input = next_value
    if type(input) ~= "table" then
      input = value
    end
    if type(input) ~= "table" then
      return next_value, extra
    end
    local command = input.command
    if type(command) ~= "string" then
      return next_value, extra
    end
    local prepared = M.prepare(command)
    if prepared == command then
      return next_value, extra
    end
    local rewritten = {}
    for key, item in pairs(input) do
      rewritten[key] = item
    end
    rewritten.command = prepared
    return rewritten, extra
  end
  local ok = pcall(function()
    maki.api.set_slot("tool.bash.input", rewrite)
  end)
  if ok then
    registered = true
  end
end

return M
