@givn.delta @ideation-skill

Feature: ideation-skill

  @givn.modified
  Scenario: The ideation guidance carries the phase ladder and the one-question rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Event Storming"
    And  the generated givn-ideation guidance contains "Use Case Definition"
    And  the generated givn-ideation guidance contains "Use Case Elaboration"
    And  the generated givn-ideation guidance contains "Handoff"
    And  the generated givn-ideation guidance contains "one question at a time"
    And  the generated givn-ideation guidance contains "quality review"
    And  the generated givn-ideation guidance contains "givn instructions dialog"
    And  the generated givn-ideation guidance contains "In practice:"
    And  the generated givn-ideation guidance contains "Rejected:"
