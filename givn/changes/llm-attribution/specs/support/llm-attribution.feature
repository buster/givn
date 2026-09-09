@givn.delta @llm-attribution

Feature: Commit attribution

  @givn.added @e2e @wip
  Scenario: Consumer initializes a project and receives shared attribution plumbing
    Given a project root without a givn/ directory and with a Git repository
    When the consumer runs `givn init --agents opencode,claude`
    Then the command exits 0
    And the checked-in commit attribution hook exists
    And the Git repository uses the checked-in commit attribution hook
    And the OpenCode attribution integration exists
    And the Claude Code attribution integration exists

  @givn.added @e2e @wip
  Scenario: An attributed commit exposes the current LLM and harness
    Given a Givn project with the shared commit-attribution hook active
    And the current attribution context has LLM "openai/gpt-5.6-luna", reasoning mode "high", and harness "opencode"
    When the consumer creates a commit with message "fix parser"
    Then the commit succeeds
    And the commit committer identifies "openai/gpt-5.6-luna" and "opencode"
    And the commit message ends with the current Givn version, LLM, reasoning mode, and harness

  @givn.added
  Scenario: A human commit remains usable without attribution context
    Given a Givn project with the shared commit-attribution hook active
    And no current attribution context
    When the consumer creates a commit with message "document manual setup"
    Then the commit succeeds
    And the commit message remains "document manual setup"

  @givn.added
  Scenario: A partial attribution context does not block a commit
    Given a Givn project with the shared commit-attribution hook active
    And the current attribution context has harness "opencode" but no LLM or reasoning mode
    When the consumer creates a commit with message "record partial context"
    Then the commit succeeds
    And the commit message identifies the harness as "opencode"
    And the commit message marks the LLM and reasoning mode as unavailable

  @givn.added
  Scenario: An existing commit hook runs before Givn attribution
    Given a Givn project with an existing commit hook that writes a marker
    And the current attribution context has LLM "openai/gpt-5.6-luna", reasoning mode "high", and harness "opencode"
    When the consumer initializes Givn in the project
    And the consumer creates a commit with message "preserve existing hook"
    Then the commit succeeds
    And the existing hook marker is present
    And the commit message contains the Givn attribution trailers

  @givn.added @wip
  Scenario: Attribution follows a model change between commits
    Given a Givn project with the shared commit-attribution hook active
    And the current attribution context has LLM "model-a", reasoning mode "high", and harness "opencode"
    When the consumer creates a commit with message "first change"
    And the current attribution context changes to LLM "model-b", reasoning mode "medium", and harness "claude-code"
    And the consumer creates a commit with message "second change"
    Then the first commit identifies "model-a", "high", and "opencode"
    And the second commit identifies "model-b", "medium", and "claude-code"
