-- Givn commit attribution for the Maki coding agent.
--
-- Maki loads this module through the managed `-- givn:begin:attribution`
-- fence in `.maki/init.lua`. Maki fires `tool.bash.input` before a Bash call is
-- parsed or checked; the layer resolves the issuing agent's model, falls back
-- to the focused session, and rewrites only a command whose text names a
-- revision verb. The prefix is the same environment protocol the OpenCode,
-- omp, and Claude integrations use; the checked-in commit-msg hook consumes it.
--
-- Best effort: a missing model leaves `unavailable` facts and no committer
-- override; a non-revision command is untouched; a host without the slot API
-- loads this module inert. Maki requires `run`, `session_read`, and
-- `session_write` for a `tool.bash.input` layer; the generated manifest grants
-- them even though this module never writes session content.

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

local function current_model()
  if type(maki) ~= "table" or type(maki.model) ~= "table" or type(maki.model.get) ~= "function" then
    return nil
  end
  local ok, model = pcall(maki.model.get)
  if ok and type(model) == "table" then
    return model
  end
  return nil
end

-- The model of the agent that issued the command. A subagent's own context
-- outranks the focused session; an unavailable lookup leaves the caller to
-- fall back.
local function issuing_model(ctx)
  if ctx == nil then
    return nil
  end
  if type(maki) ~= "table" or type(maki.agent) ~= "table"
      or type(maki.agent.resolve_model) ~= "function" then
    return nil
  end
  local ok, model = pcall(maki.agent.resolve_model, ctx, {})
  if ok and type(model) == "table" then
    return model
  end
  return nil
end

-- Identity comes from the issuing context first. Reasoning falls back too:
-- Maki documents no per-context thinking level.
local function attribution_for(ctx)
  local issuing = issuing_model(ctx)
  local identity = identity_of(issuing)
  local reasoning = reasoning_of(issuing)
  if identity ~= nil and reasoning ~= UNAVAILABLE then
    return identity, reasoning
  end
  local focused = current_model()
  identity = identity or identity_of(focused) or UNAVAILABLE
  if reasoning == UNAVAILABLE then
    reasoning = reasoning_of(focused)
  end
  return identity, reasoning
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

function M.prepare(command, ctx)
  if type(command) ~= "string" or not command:find(REVISION_VERB) then
    return command
  end
  local identity, reasoning = attribution_for(ctx)
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
    local prepared = M.prepare(command, ctx)
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
