Feature: Refresh generated guidance

  @e2e @refresh-generated-guidance
  Scenario: Synchronized guidance explains how to resolve the deterministic gates
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  synchronization reports skills rather than skills and agent commands
    And  generated review design and steps skills direct readers to their canonical instructions
    And  the served review instruction retains split-or-keep while treating shape matches as advisory
    And  the served design and steps instructions retain black-box behavior and distinct requirement coverage
    And  generated skills contain no repeated dialog-loading or delegation policy blocks
    And  no separately authored agent commands are generated
    And  the terminal transcript is captured for this scenario

  @refresh-generated-guidance
  Scenario: Review instructions require deterministic gate dispositions
    Given  a givn project
    When  the user runs `givn instructions review --change skill-rewrites`
    Then  the guidance requires split-or-keep and a brief reason for keeping a long scenario
    And  the guidance requires completing a chosen split before sign-off
    And  shape-match findings do not require blocking dispositions

  Scenario: Design and step instructions require the Black-Box-First decision
    Given a givn project
    When the user reads the design and steps instructions for skill-rewrites
    Then combined instruction output contains "Black-Box-First"
    And combined instruction output contains "which case this test covers"

  Scenario: Spec instructions use deterministic gates before retrieval exists
    Given a givn project
    When the user runs `givn instructions specs --change skill-rewrites`
    Then stdout contains "User Interaction Inventory"
    And stdout does not contain "Run `givn spec index`"

  @refresh-generated-guidance
  Scenario: Generated guidance carries no copy of a built-in rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  every generated skill carries trigger metadata and a direction to its served instruction
    And  no generated skill copies built-in instruction rules
    And  dialog policy and delegation boilerplate are not repeated in thin skills
    And  no separately authored agent command is generated
