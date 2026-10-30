@givn.delta @generate-ideation-guidance

Feature: Generate ideation guidance

  @givn.removed
  Scenario: The explore guidance points to ideation for structured domain work
    Given placeholder

  @givn.added @wip
  Scenario: The start guidance points to ideation for structured domain work
    Given a Givn project
    Then the served start instruction contains "givn-ideation"

  @givn.added @wip
  Scenario: The ideation guidance compares start rather than explore
    Given a Givn project
    Then the served ideation instruction contains "givn-start"
    And the served ideation instruction does not contain "givn-explore"
