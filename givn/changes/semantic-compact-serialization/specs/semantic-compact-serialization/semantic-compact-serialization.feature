# User Interaction Inventory:
# - rebuild the permanent scenario retrieval index with `givn spec index`

@givn.delta @semantic-compact-serialization
Feature: Compact complete semantic retrieval passages

  @givn.added
  Scenario: Empty optional sections do not add retrieval content
    Given a permanent scenario with only required feature, scenario, and step content
    When the user rebuilds the permanent scenario retrieval index
    Then the index contains the minimal scenario with the expected compact content hash
    And the index contains no additional scenario vector

  @givn.added @wip
  Scenario: Non-empty optional sections remain complete retrieval content
    Given a permanent scenario with non-empty descriptions, backgrounds, tables, docstrings, outline, and examples
    When the user rebuilds the permanent scenario retrieval index
    Then the index contains the rich scenario with the expected complete compact content hash

  @givn.added @e2e @wip
  Scenario: A maintainer can index scenarios with complete compact passages
    Given a permanent corpus with required and non-empty optional scenario content
    When the end-to-end user rebuilds the compact retrieval index through the givn CLI
    Then the command exits 0
    And stdout reports that the permanent scenarios were indexed
    And the persistent retrieval index contains the indexed scenarios
