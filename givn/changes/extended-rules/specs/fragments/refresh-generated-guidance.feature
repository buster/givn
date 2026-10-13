@givn.delta @refresh-generated-guidance

Feature: Refresh generated guidance

  @givn.modified @e2e @wip
  Scenario: Synchronized guidance explains how to resolve the deterministic gates
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then stdout contains "Syncing skills and agent commands..."
    And the generated givn-review guidance directs the reviewer to the review instruction
    And the generated givn-design guidance directs the designer to the design instruction
    And the generated givn-steps guidance directs the step writer to the steps instruction
    And the served review instruction contains "Overlap dispositions"
    And the served review instruction contains "split-or-keep"
    And the served review instruction contains "removed+added"
    And the served design instruction contains "Black-Box-First"
    And the served steps instruction contains "which case this test covers"
    And generated guidance does not contain "Run `givn spec index`"

  @givn.added @wip
  Scenario: Generated guidance carries no copy of a built-in rule
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then every generated skill and command carries trigger metadata and a direction
    And no generated skill or command copies the built-in rules of its instruction
