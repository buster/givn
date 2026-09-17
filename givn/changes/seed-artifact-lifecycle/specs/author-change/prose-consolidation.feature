@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.added @wip
  Scenario: The propose and archive commands carry the seed stamping instructions
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-propose command contains "proposed by"
    And the generated givn-archive command contains "implemented in"
