@givn.delta @serve-extended-rules
Feature: Ise ship guidance

  @givn.added
  Scenario: The ise instruction documents the whole-system working-tree ship
    Given a Givn project with ise effective
    Then the served ise instruction contains "Whole-system rebuild with Rebuilt code: ship"
    And the served ise instruction contains "component_executed=<executed>/<declared>"

  @givn.added
  Scenario: The design guidance requires the component runner for a whole-system rebuild
    Given a Givn project with ise effective
    Then the served design instruction contains "one step-definition file per capability"
    And the served design instruction contains "leaves the retained component corpus without a runner is incomplete"

  @givn.added
  Scenario: The design-review guidance grills the two-tier runner
    Given a Givn project with ise effective
    Then the served design-review instruction contains "Two-tier runner coverage"

  @givn.added
  Scenario: The tasks guidance keeps the retained corpus when the delta is skipped
    Given a Givn project with ise effective
    Then the served tasks instruction contains "The skip removes the change's delta scenario batch"

  @givn.added
  Scenario: The review guidance treats a missing declared tier as critical
    Given a Givn project with ise effective
    Then the served review instruction contains "A declared verification tier with no executed result is a critical finding"
