@givn.delta @generate-ideation-guidance

Feature: Generate ideation guidance

  @givn.added
  Scenario: The ideation guidance requires a canvas with distinguishable element kinds
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "one or more mermaid diagrams"
    And  the generated givn-ideation guidance contains "show every event, command, aggregate, and actor, with the kinds"
    And  the generated givn-ideation guidance contains "(per-node kind markers or a legend)"
    And  the generated givn-ideation guidance contains "the actor that issues it and the aggregate it changes"
    And  the generated givn-ideation guidance contains "A text chain without a canvas is an incomplete event storming"
    And  the generated givn-ideation guidance contains "the canvas must render"
    And  the generated givn-ideation guidance contains "drawn as a rendered mermaid canvas"
    And  the generated givn-ideation guidance contains "aggregate (the consistency boundary the command changes)"

  @givn.added
  Scenario: The ideation guidance keeps aggregates to declared consistency boundaries
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Aggregates are given, not minted"
    And  the generated givn-ideation guidance contains "a domain-noun root that receives commands and emits events under one"
    And  the generated givn-ideation guidance contains "confirmed before the canvas relies on it"
    And  the generated givn-ideation guidance contains "Reuse the aggregates the project already declares"
    And  the generated givn-ideation guidance contains "never an aggregate"
    And  the generated givn-ideation guidance contains "Name the kind"

  @givn.added
  Scenario: The event storming review checks canvas coverage, legality, agreement, and rendering
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "canvas that shows every event, command, aggregate, and actor with its kinds"
    And  the generated givn-ideation guidance contains "every aggregate a declared consistency boundary or a"
    And  the generated givn-ideation guidance contains "drawn as an aggregate is a defect"
    And  the generated givn-ideation guidance contains "the lines and the picture naming the"
    And  the generated givn-ideation guidance contains "a canvas that renders"

  @givn.added
  Scenario: The ideation command guardrails require the canvas and legal aggregates
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation command contains "rendered mermaid canvas naming every event"
    And  the generated givn-ideation command contains "every aggregate is a declared"
    And  the generated givn-ideation command contains "never a record or a specification"
