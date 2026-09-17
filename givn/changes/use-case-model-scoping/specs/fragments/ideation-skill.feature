@givn.delta @ideation-skill

Feature: ideation-skill

  @givn.added
  Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "contract between one primary actor and the system"
    And  the generated givn-ideation guidance contains "One goal, one primary actor"
    And  the generated givn-ideation guidance contains "Goal-shaped, never component-shaped"
    And  the generated givn-ideation guidance contains "Level discipline"
    And  the generated givn-ideation guidance contains "promotion rule"
    And  the generated givn-ideation guidance contains "three or more distinct actor goals"
    And  the generated givn-ideation guidance contains "Placement test"
    And  the generated givn-ideation guidance contains "Mutation containment"
    And  the generated givn-ideation guidance contains "Extension discipline"
    And  the generated givn-ideation guidance contains "Precondition and flow consistency"
    And  the generated givn-ideation guidance contains "Implementation independence"
    And  the generated givn-ideation guidance contains "Relationship semantics"

  @givn.added
  Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "the first actor is the primary actor"
    And  the generated givn-ideation guidance contains "Supporting:"
    And  the generated givn-ideation guidance contains "protects <guarantee>"
    And  the generated givn-ideation guidance contains "never an extension"
    And  the generated givn-ideation guidance contains "never to a capability inside another goal"
