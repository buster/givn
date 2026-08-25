# User Interaction Inventory:
# - invoke `givn spec explore --file <draft.feature>` and read advisory text results
# - invoke `givn spec explore --stdin` and read advisory text results
# - invoke `givn spec explore --format json` and consume the machine-readable result

@givn.delta @spec-explore-permanent
Feature: Explore permanent scenarios before creation

  @givn.added @e2e
  Scenario: File exploration finds a semantically similar permanent scenario
    Given an end-to-end project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the end-to-end user explores "draft.feature" with the file input
    Then the command exits 0
    And stdout contains "Provider name is unknown"
    And stdout contains "embedding score"

  @givn.added @e2e
  Scenario: Stdin exploration returns the same candidates as file exploration
    Given an end-to-end project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the end-to-end user explores "draft.feature" with the file input
    And the end-to-end user explores the same draft through standard input
    Then the standard-input results equal the file results

  @givn.added @e2e
  Scenario: JSON exploration reports query index and candidate fields
    Given an end-to-end project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the end-to-end user explores "draft.feature" with JSON output
    Then the command exits 0
    And stdout is a stable exploration JSON document
    And the exploration JSON query title is "Provider selection rejects an unknown provider"
    And the exploration JSON includes the query content hash
    And the exploration JSON includes the embedding model and index metadata
    And every exploration JSON result has rank title file and score

  @givn.added
  Scenario: File exploration excludes only the exact permanent scenario identity
    Given a project prepared for exact scenario identity exploration
    When the user explores "givn/specs/provider.feature" with the file input
    Then the command exits 0
    And no human-readable result is titled "Provider name is unknown"
    And stdout contains "Unknown provider is rejected"

  @givn.added
  Scenario: Unchanged permanent scenarios reuse the existing index
    Given a project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" with the file input
    And the user explores "draft.feature" with the file input
    Then stdout contains "Reused 3 scenario vectors"

  @givn.added
  Scenario: A changed permanent scenario receives a new embedding on the next exploration
    Given a project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" with the file input
    And the permanent scenario "Authentication token is missing" is changed
    And the user explores "draft.feature" with the file input
    Then stdout contains "Reused 2 scenario vectors"
    And stdout contains "Embedded 1 new scenario vectors"

  @givn.added
  Scenario: Invalid Gherkin input is rejected
    Given a project prepared for permanent scenario exploration
    And a file at "invalid.feature" containing invalid Gherkin
    When the user explores "invalid.feature" with the file input
    Then the command exits non-zero
    And stderr contains "not valid Gherkin"

  @givn.added
  Scenario: Multiple scenarios in one input are rejected
    Given a project prepared for permanent scenario exploration
    And a file at "multiple.feature" containing two complete scenarios
    When the user explores "multiple.feature" with the file input
    Then the command exits non-zero
    And stderr contains "exactly one scenario"

  @givn.added
  Scenario: A valid feature without a scenario is rejected
    Given a project prepared for permanent scenario exploration
    And a file at "empty.feature" containing no scenario
    When the user explores "empty.feature" with the file input
    Then the command exits non-zero
    And stderr contains "exactly one scenario"

  @givn.added
  Scenario: A missing draft file is rejected
    Given a project prepared for permanent scenario exploration
    When the user explores "missing.feature" with the file input
    Then the command exits non-zero
    And stderr contains "failed to read explore input file"

  @givn.added
  Scenario: File and standard input cannot be selected together
    Given a project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" with both file and standard input
    Then the command exits non-zero
    And stderr contains "exactly one input"

  @givn.added
  Scenario: Explore remains unavailable without the retrieval feature
    Given a project prepared for permanent scenario exploration
    When the user runs explore through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"

  @givn.added
  Scenario: Exploration leaves permanent feature files unchanged
    Given a project prepared for permanent scenario exploration
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    And the permanent specifications are snapshotted
    When the user explores "draft.feature" with the file input
    Then the permanent specifications are unchanged

  @givn.added
  Scenario: Existing spec search keeps its current behavior
    Given a givn project with permanent scenarios for retrieval
    And the local spec index has been built
    When the user searches for "the command rejects an unknown provider"
    Then the command exits 0
    And stdout contains "Provider name is unknown"
    And stdout contains "rank 1"

  @givn.added
  Scenario: Existing spec review keeps its current behavior
    Given a givn project with permanent scenarios for retrieval
    And an active change delta contains a scenario named "Running change only"
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout contains "retrieval advisory"
