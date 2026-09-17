@givn.delta @ideation-skill

Feature: ideation-skill

  @givn.added
  Scenario: The ideation guidance carries the seed schema and its status values
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "change seed"
    And  the generated givn-ideation guidance contains "Status: ready"
    And  the generated givn-ideation guidance contains "proposed by"
    And  the generated givn-ideation guidance contains "implemented in"
    And  the generated givn-ideation guidance contains "superseded by"
    And  the generated givn-ideation guidance contains "withdrawn"
    And  the generated givn-ideation guidance contains "Accepted decisions"
    And  the generated givn-ideation guidance contains "Suggested start"
