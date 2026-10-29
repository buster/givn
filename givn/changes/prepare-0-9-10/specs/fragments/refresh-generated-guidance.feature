@givn.delta @refresh-generated-guidance
Feature: Refresh generated guidance

  @givn.modified @e2e @wip
  Scenario: Synchronized guidance explains how to resolve the deterministic gates
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then synchronization reports skills rather than skills and agent commands
    And generated review design and steps skills direct readers to their canonical instructions
    And the served review instruction retains split-or-keep while treating shape matches as advisory
    And the served design and steps instructions retain black-box behavior and distinct requirement coverage
    And generated skills contain no repeated dialog-loading or delegation policy blocks
    And no separately authored agent commands are generated

  @givn.modified @wip
  Scenario: Review instructions require deterministic gate dispositions
    Given a givn project
    When the user runs `givn instructions review --change skill-rewrites`
    Then the guidance requires split-or-keep and a brief reason for keeping a long scenario
    And the guidance requires completing a chosen split before sign-off
    And shape-match findings do not require blocking dispositions

  @givn.modified @wip
  Scenario: Generated guidance carries no copy of a built-in rule
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then every generated skill carries trigger metadata and a direction to its served instruction
    And no generated skill copies built-in instruction rules
    And dialog policy and delegation boilerplate are not repeated in thin skills
    And no separately authored agent command is generated
