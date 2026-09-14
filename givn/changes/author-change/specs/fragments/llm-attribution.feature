@givn.delta @llm-attribution
Feature: Commit attribution

  @givn.added
  Scenario: The SCM-neutral commit path attributes a Git revision
    Given  a Givn project with the shared commit-attribution hook active
    When  the consumer commits with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then  the command exits 0
    And  the revision message carries the managed attribution trailer

  @givn.added
  Scenario: The SCM-neutral commit path attributes a Fossil check-in
    Given  an initialized Fossil project
    When  the consumer commits with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then  the command exits 0
    And  the revision message carries the managed attribution trailer

  @givn.added
  Scenario: The commit wrapper without attribution context leaves the message unchanged
    Given  a Givn project with the shared commit-attribution hook active
    When  the consumer commits with message "fix parser" without attribution context
    Then  the command exits 0
    And  the revision message carries no managed attribution trailer
