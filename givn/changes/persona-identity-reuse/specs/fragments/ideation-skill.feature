@givn.delta @ideation-skill

Feature: ideation-skill

  @givn.added
  Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "authority boundary"
    And  the generated givn-ideation guidance contains "evaluation capability"
    And  the generated givn-ideation guidance contains "own vocabulary"
    And  the generated givn-ideation guidance contains "topic stance"
    And  the generated givn-ideation guidance contains "inventory"
    And  the generated givn-ideation guidance contains "Reuse:"
    And  the generated givn-ideation guidance contains "New:"
    And  the generated givn-ideation guidance contains "append-only"
    And  the generated givn-ideation guidance contains "one persona with three stance blocks"
    And  the generated givn-ideation guidance contains "record entries only"

  @givn.added @wip
  Scenario: The ideation guidance keeps the confirmed persona behaviors
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "stakeholder type"
    And  the generated givn-ideation guidance contains "persona segment"
    And  the generated givn-ideation guidance contains "persona promotion"
    And  the generated givn-ideation guidance contains "every artifact that affects"
    And  the generated givn-ideation guidance contains "at most three"
    And  the generated givn-ideation guidance contains "never auto-promote"
    And  the generated givn-ideation guidance contains "confirms every persona"
    And  the generated givn-ideation guidance contains "arbitrates"
    And  the generated givn-ideation command contains "inventory"
    And  the generated givn-ideation command contains "match"
    And  the generated givn-ideation command contains "separate from Actors"

  @givn.added @wip
  Scenario: The persona template carries the stable core and the running record
    Given the persona template is inspected
    When an author reads the persona schema
    Then the template names the authority boundary and evaluation capability
    And the template names the own vocabulary and success criterion
    And the template carries an append-only running record
    And the template carries a dated topic stances appendix
