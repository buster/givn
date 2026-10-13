Feature: Change release note

  @e2e
  Scenario: A new change's proposal carries the release-note field
    Given  a project root with givn/ initialised
    When  the user runs `givn new shared-note`
    Then  the command exits 0
    And  the file "givn/changes/shared-note/proposal.md" contains "release note"

  @e2e
  Scenario: The proposal guidance requires a release note
    Given  a project root with givn/ initialised
    And  a change "shared-note" exists with artifact "proposal"
    When  the user runs `givn instructions proposal --change shared-note`
    Then  the command exits 0
    And  stdout contains "release note"
    And  stdout contains "carries the release note as its message subject"

  @e2e
  Scenario: The generated implementation guidance shares the release note
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the generated givn-steps guidance directs the step writer to the steps instruction
    And  the generated givn-implement guidance directs the implementer to the tasks instruction
    And  the generated givn-tasks guidance directs the task author to the tasks instruction
    And  the served tasks instruction contains "carries the release note as its message subject"
    And  the served tasks instruction contains "the scenario title goes in the message body"
    And  generated guidance does not contain "message references the scenario title"
    And  the file "AGENTS.md" contains "carries the release note as its message subject"
