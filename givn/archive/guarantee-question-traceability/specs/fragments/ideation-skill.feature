@givn.delta @ideation-skill

Feature: ideation-skill

  @givn.added
  Scenario: The ideation guidance carries the open-question qualifier rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "(open: Q<N>)"
    And  the generated givn-ideation guidance contains "must not be presented as unconditional"
    And  the generated givn-ideation guidance contains "removes the qualifier"
