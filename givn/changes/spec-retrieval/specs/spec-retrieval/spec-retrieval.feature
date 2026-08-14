# User Interaction Inventory:
# - Run `givn spec index` to build a local search index over permanent scenarios
# - Run `givn spec search` to find semantically related existing scenarios

@givn.delta @spec-retrieval

Feature: spec-retrieval

  @givn.added @e2e @wip
  Scenario: Local index reports the permanent scenario inventory
    Given a givn project with permanent scenarios for retrieval
    When the user runs `givn spec index`
    Then stdout contains "Indexed 3 permanent scenarios"
    And the local spec index exists

  @givn.added @e2e @wip
  Scenario: Search returns a semantically related scenario
    Given a givn project with permanent scenarios for retrieval
    And the local spec index has been built
    When the user searches for "the command rejects an unknown provider"
    Then stdout contains "Provider name is unknown"
    And stdout contains "rank 1"

  @givn.added @wip
  Scenario: Search does not return the running change's own delta
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    And the local spec index has been built
    When the user searches for "running change only"
    Then stdout does not contain "Running change only"

  @givn.added
  Scenario: Reindex reuses unchanged scenario vectors
    Given a givn project with permanent scenarios for retrieval
    And the local spec index has been built
    When the user rebuilds the local spec index without changing specifications
    Then stdout contains "Reused 3 scenario vectors"

  @givn.added
  Scenario: Index reports unavailable model setup
    Given a givn project with permanent scenarios for retrieval
    And the retrieval model is unavailable
    When the user runs `givn spec index`
    Then the command fails
    And stderr contains "retrieval model is unavailable"
