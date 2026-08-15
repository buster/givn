@givn.delta @spec-retrieval
Feature: spec-retrieval

  @givn.modified @e2e @wip
  Scenario: Review reports an advisory retrieval candidate
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario similar to "Provider name is unknown"
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "similarity candidate"
    And stdout contains "MERGE"
    And stdout contains "KEEP-AS-BOUNDARY"

  @givn.modified @wip
  Scenario: Review suppresses a disposed retrieval candidate
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario similar to "Provider name is unknown"
    And a review disposition classifies "Unknown provider is rejected" and "Provider name is unknown" as "KEEP-AS-BOUNDARY" with a boundary rationale
    When the user runs `givn spec review --change running-change`
    Then stdout contains "Provider name is unknown"
    And the command fails

  @givn.modified @wip
  Scenario: Review gate keeps retrieval advisory
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario similar to "Provider name is unknown"
    When the user runs `givn check review --change running-change`
    Then stdout contains "similarity candidate"
    And the command fails
