@givn.delta @llm-attribution

Feature: OpenCode commit attribution

  @givn.modified @e2e
  Scenario: An attributed commit exposes the current LLM and harness
    Given a Git project with complete commit attribution context
    When the consumer creates an attributed Git commit with message "fix parser"
    Then the attributed commit succeeds
    And its committer identifies "openai/gpt-5.6-luna" and "opencode"
    And its message contains only the managed co-author attribution

  @givn.added @e2e
  Scenario: OpenCode prepares attribution using its selected model
    Given a project with the generated OpenCode attribution integration
    When OpenCode supplies the selected provider "openai" and model "gpt-5.6-luna" for session "session-1"
    And OpenCode prepares the environment for a shell in session "session-1"
    Then the shell environment identifies LLM "openai/gpt-5.6-luna"
    And the shell environment identifies reasoning mode "high"
    And the shell environment identifies harness "opencode"
    And the shell environment identifies committer "LLM: openai/gpt-5.6-luna via opencode"

  @givn.added @e2e
  Scenario: Claude attributes a commit using its current model
    Given a project with the generated Claude attribution integration
    When Claude provides current model "claude-sonnet-5" in its session transcript
    And Claude prepares an RTK-wrapped Bash commit command with message "claude attribution"
    Then the attributed Claude commit succeeds
    And its Claude committer identifies "claude-sonnet-5" and "claude-code"
    And its Claude message identifies LLM "claude-sonnet-5"
    And its Claude message identifies the co-author attribution
