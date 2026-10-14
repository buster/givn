@givn.delta @change-release-note

Feature: Change release note

  @givn.modified @e2e
  Scenario: The generated implementation guidance shares the release note
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then the command exits 0
    And the generated givn-steps guidance directs the step writer to the steps instruction
    And the generated givn-implement guidance directs the implementer to the tasks instruction
    And the generated givn-tasks guidance directs the task author to the tasks instruction
    And the served tasks instruction contains "carries the release note as its message subject"
    And the served tasks instruction contains "the scenario title goes in the message body"
    And generated guidance does not contain "message references the scenario title"
    And the file "AGENTS.md" contains "carries the release note as its message subject"
