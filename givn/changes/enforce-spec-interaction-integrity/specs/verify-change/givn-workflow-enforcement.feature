@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  @givn.added
  Scenario: Review refuses a projected interaction-inventory mismatch before archive
    Given a givn project with a change that has passed design-review
    And the change's delta adds an E2E scenario "Consumer cancels an order" without an interaction row
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "interaction inventory mismatch"
    And stderr contains "use case 'demo-cap'"
    And stderr contains "capability 'demo-cap'"
    And stderr contains "unlisted @e2e scenario 'Consumer cancels an order'"

  @givn.added
  Scenario: Review accepts a delta whose projected interaction inventory is consistent
    Given a givn project with a change that has passed design-review
    And the change's delta adds an E2E scenario "Consumer cancels an order" with its interaction row
    When the user runs `givn check review --change test-change`
    Then the command exits 0
