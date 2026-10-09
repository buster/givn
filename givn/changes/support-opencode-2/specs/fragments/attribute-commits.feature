@givn.delta @attribute-commits

Feature: Attribute commits

  @givn.modified @e2e
  Scenario: OpenCode prepares attribution using its selected model
    Given  a project with the generated OpenCode attribution integration
    When  OpenCode supplies the selected provider "openai" and model "gpt-5.6-luna" for session "session-1"
    And  OpenCode prepares the environment for a shell in session "session-1"
    Then  the shell environment identifies LLM "openai/gpt-5.6-luna"
    And  the shell environment identifies reasoning mode "high"
    And  the shell environment identifies harness "opencode"
    And  the shell environment identifies committer "LLM: openai/gpt-5.6-luna via opencode"

  @givn.added
  Scenario: OpenCode leaves attribution unavailable when no model is observed
    Given  a project with the generated OpenCode attribution integration
    When  OpenCode prepares the environment for a shell in session "session-1"
    Then  the shell environment identifies LLM "unavailable"
    And  the shell environment identifies reasoning mode "unavailable"
    And  the shell environment identifies harness "opencode"
    And  the shell environment carries no committer identity

  @givn.added
  Scenario: OpenCode attributes a command to the session that issued it
    Given  a project with the generated OpenCode attribution integration
    When  OpenCode supplies the selected provider "openai" and model "gpt-5.6-luna" for session "session-1"
    And  OpenCode supplies the selected provider "anthropic" and model "claude-sonnet-5" for session "session-2"
    And  OpenCode prepares the environment for a shell in session "session-1"
    Then  the shell environment identifies LLM "openai/gpt-5.6-luna"
