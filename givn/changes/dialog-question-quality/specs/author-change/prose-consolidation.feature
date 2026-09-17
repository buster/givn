@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.added @wip
  Scenario: Generated interactive guidance points to the canonical dialog policy
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-ideation guidance contains "givn instructions dialog"
    And the generated givn-explore guidance contains "givn instructions dialog"
    And the generated givn-design-review guidance contains "givn instructions dialog"
    And the generated givn-design-review command contains "givn instructions dialog"
    And the generated givn-ideation command contains "givn instructions dialog"
    And the generated givn-explore command contains "givn instructions dialog"
    And the generated givn-propose command contains "givn instructions dialog"
    And the generated grill-me command contains "givn instructions dialog"
    And the generated grill-with-docs command contains "givn instructions dialog"
