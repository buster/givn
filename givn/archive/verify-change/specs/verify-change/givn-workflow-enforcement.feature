@givn.delta @givn-workflow-enforcement
Feature: givn Workflow Enforcement

  @givn.added
  Scenario: An opaque revision identifier passes the integrity check
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains `- [x] COMMIT: 6f5a2c9e8b7d4a3f91c2e5d8b7a4f3c2e1d0b9a8 -- Implement scenario one`
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0
    And stdout contains "integrity"

  @givn.modified
  Scenario: Integrity hook fails on placeholder commit hash
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains `- [x] COMMIT: ___ -- scenario one`
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "placeholder"
    And stderr contains "revision identifier"
