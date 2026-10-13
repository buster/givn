@givn.delta @visual-design-contract

Feature: Visual Design Contract

  @givn.modified @e2e @wip
  Scenario: Synchronized guidance defines the contract approval and capture mechanics
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-design-review guidance directs the reviewer to the design-review instruction
    And the served design-review instruction contains "Visual Design Contract"
    And the served design-review instruction contains "propose"
    And the served design-review instruction contains "user approves"
    And the served design-review instruction contains "rendered options"
    And the generated givn-steps guidance directs the step writer to the steps instruction
    And the served steps instruction contains "evidence/visual"
    And the served steps instruction contains "final-state"
    And the served steps instruction contains "animations are frozen"
    And the served steps instruction contains "reports/"

  @givn.modified @wip
  Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then the served design instruction contains "renders HTML, CSS, or JavaScript in a browser"
    And the served design instruction contains "default browser driver"
    And the served design-review instruction contains "opposite classification"
    And the served proposal instruction contains "operator confirmation"
