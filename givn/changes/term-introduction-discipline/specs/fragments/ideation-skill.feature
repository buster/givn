@givn.delta @ideation-skill

Feature: ideation-skill

  @givn.added @wip
  Scenario: The ideation guidance carries the term schema and the collision procedure
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "domain-terms.md"
    And  the generated givn-ideation guidance contains "`candidate`, `confirmed`, or `rejected`"
    And  the generated givn-ideation guidance contains "Contrast"
    And  the generated givn-ideation guidance contains "First use"
    And  the generated givn-ideation guidance contains "definition and first use"
    And  the generated givn-ideation guidance contains "recorded collision check"
    And  the generated givn-ideation guidance contains "only `confirmed` terms"
    And  the generated givn-ideation guidance contains "never deleted"
    And  the generated givn-ideation guidance contains "givn-domain-modeling"
    And  the generated givn-ideation guidance contains "Collision check"
    And  the generated givn-ideation guidance contains "Grep the durable glossary"
    And  the generated givn-ideation guidance contains "without recorded commands is not a record"
