# User Interaction Inventory:
# - Run `givn spec index` to build a local search index over permanent scenarios
# - Run `givn spec search` to find semantically related existing scenarios
# - Run `givn spec review --change` to surface advisory candidates for a delta

@givn.delta @spec-retrieval

Feature: spec-retrieval

  @givn.added @e2e
  Scenario: Local index reports the permanent scenario inventory
    Given a givn project with permanent scenarios for retrieval
    When the user runs `givn spec index`
    Then stdout contains "Indexed 3 permanent scenarios"
    And the local spec index exists

  @givn.added @e2e
  Scenario: Search returns a semantically related scenario
    Given a givn project with permanent scenarios for retrieval
    And the local spec index has been built
    When the user searches for "the command rejects an unknown provider"
    Then stdout contains "Provider name is unknown"
    And stdout contains "rank 1"

  @givn.added
  Scenario: Search does not return the running change's own delta
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    And the local spec index has been built
    When the user searches for "running change only"
    Then stdout does not contain "Running change only"

  @givn.added @e2e
  Scenario: Review reports an advisory retrieval candidate
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    When the user runs `givn spec review --change running-change`
    Then stdout contains "retrieval advisory"
    And the command exits 0

  @givn.added
  Scenario: Review suppresses a disposed retrieval candidate
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    And a review disposition exists for "Running change only" and "Provider name is unknown"
    When the user runs `givn spec review --change running-change`
    Then stdout does not contain "Provider name is unknown"
    And the command exits 0

  @givn.added
  Scenario: Review gate keeps retrieval advisory
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    When the user runs `givn check review --change running-change`
    Then stdout contains "retrieval advisory"
    And the command exits 0

  @givn.added
  Scenario: Review rejects a missing change delta
    Given a givn project with permanent scenarios for retrieval
    When the user runs `givn spec review --change missing-change`
    Then the command fails
    And stderr contains "no specs directory"

  @givn.added
  Scenario: Review rejects an unsafe change identifier
    Given a givn project with permanent scenarios for retrieval
    When the user runs `givn spec review --change ../outside`
    Then the command fails
    And stderr contains "kebab-case"

  @givn.added
  Scenario: Review ignores a removed delta scenario
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a removed scenario named "Removed scenario"
    When the user runs `givn spec review --change running-change`
    Then stdout does not contain "Removed scenario"
    And the command exits 0

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
