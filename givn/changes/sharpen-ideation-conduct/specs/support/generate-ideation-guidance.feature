@givn.delta @generate-ideation-guidance

Feature: Generate ideation guidance

  @givn.added
  Scenario: The ideation guidance carries the live-question standard
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "A bare option list is a defect"
    And  the generated givn-ideation guidance contains "the decision it blocks"
    And  the generated givn-ideation guidance contains "context that forces it"

  @givn.added
  Scenario: The ideation guidance stops questions at decided directions
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "never asked again"
    And  the generated givn-ideation guidance contains "applied and reported"
    And  the generated givn-ideation guidance contains "phase report"
    And  the generated givn-ideation guidance contains "only when a new fork"
    And  the generated givn-ideation guidance contains "share one direction are one question"

  @givn.added
  Scenario: The frame is confirmed once and later phase boundaries report
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Confirm the Frame"
    And  the generated givn-ideation guidance contains "report what was recorded and proceed"

  @givn.added
  Scenario: Handoff cuts change seeds by deliverable guarantee
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "deliverable guarantee"
    And  the generated givn-ideation guidance contains "true end to end"
    And  the generated givn-ideation guidance contains "can ship alone"
    And  the generated givn-ideation guidance contains "may cover several use cases"
    And  the generated givn-ideation guidance contains "one use case may yield several seeds"
    And  the generated givn-ideation guidance contains "confirmed rule untrue"
    And  the generated givn-ideation guidance contains "dependencies and a suggested start order"
    And  the generated givn-ideation guidance contains "changes/<change-id>.md"

  @givn.added
  Scenario: The ideation command guardrails carry the same conduct rules
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation command contains "deliverable guarantee"
    And  the generated givn-ideation command contains "never asked again"
