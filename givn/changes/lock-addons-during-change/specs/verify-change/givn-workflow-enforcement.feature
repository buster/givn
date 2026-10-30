@givn.delta @givn-workflow-enforcement

Feature: Givn workflow enforcement

  @givn.added
  Scenario: Review rejects add-on configuration drift
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    And the project configuration sets addon "coverage" to "false"
    And the user runs `givn check review --change addon-lock`
    Then the command exits non-zero
    And stderr contains "add-on configuration changed"
    And stderr contains "coverage: enabled -> disabled"
