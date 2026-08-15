# User Interaction Inventory:
# - Run `givn spec review --change <id>` to evaluate scenario pairs with the local pair reranker.

@givn.delta @cross-encoder-reranker
Feature: Rerank semantic scenario candidates

  @givn.added @e2e @wip
  Scenario: Review evaluates semantic candidates through the pair reranker
    Given a givn project with a true duplicate pair for reranking
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "rerank raw"
    And stderr contains "similarity gate failed"

  @givn.added @wip
  Scenario: Review does not block a valid boundary rejected by the pair reranker
    Given a givn project with a valid boundary pair that has high embedding similarity
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout contains "reranker"
    And stdout does not contain "similarity candidate"

  @givn.added @wip
  Scenario: Review applies the pair reranker to current scenarios and the projected permanent tree
    Given a givn project with a true duplicate pair for reranking
    And the active change removes the duplicate scenario named "Unknown provider is rejected"
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout contains "reranker"

  @givn.added @wip
  Scenario: Review reports pair-reranker setup failure
    Given a givn project with a true duplicate pair for reranking
    And the local pair reranker is unavailable
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stderr contains "reranker model is unavailable"
