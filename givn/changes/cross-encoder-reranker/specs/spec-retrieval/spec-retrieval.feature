@givn.delta @spec-retrieval
Feature: spec-retrieval

  @givn.modified @e2e @wip
  Scenario: Review reports an advisory retrieval candidate
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout contains "reranker"

  @givn.modified @wip
  Scenario: Review gate keeps retrieval advisory
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    When the user runs `givn check review --change running-change`
    Then stdout contains "reranker"
    And the command exits 0
