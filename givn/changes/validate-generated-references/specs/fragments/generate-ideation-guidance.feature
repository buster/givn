@givn.delta @generate-ideation-guidance

Feature: Generate ideation guidance

  @givn.added @wip
  Scenario: The ideation guidance carries the persona schema
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Stable core"
    And  the generated givn-ideation guidance contains "Authority boundary"
    And  the generated givn-ideation guidance contains "Evaluation capability"
    And  the generated givn-ideation guidance contains "Own vocabulary"
    And  the generated givn-ideation guidance contains "Success criterion"
    And  the generated givn-ideation guidance contains "Running record"
    And  the generated givn-ideation guidance contains "append-only"
    And  the generated givn-ideation guidance contains "Topic stances"
