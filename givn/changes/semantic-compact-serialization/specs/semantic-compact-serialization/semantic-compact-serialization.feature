# User Interaction Inventory:
# - rebuild the permanent scenario retrieval index with `givn spec index`

@givn.delta @semantic-compact-serialization
Feature: Compact complete semantic retrieval passages

  @givn.added
  Scenario: Compact serialization preserves empty and non-empty retrieval boundaries
    Given a permanent corpus with minimal and rich retrieval fixtures
    When the user rebuilds the permanent scenario retrieval index
    Then the index contains both fixtures with their expected compact content hashes
    And the index contains only the two fixture vectors

  @givn.added @e2e
  Scenario: A maintainer can index scenarios with complete compact passages
    Given a permanent corpus with required and non-empty optional scenario content
    When the end-to-end user rebuilds the compact retrieval index through the givn CLI
    Then the command exits 0
    And stdout reports that the permanent scenarios were indexed
    And the persistent retrieval index contains the indexed scenarios
