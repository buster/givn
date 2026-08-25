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
  Scenario: CLI reports successful catalogue construction
    Given a shell project with one rich fixture
    When the maintainer runs the catalogue command
    Then the command exits 0
    And stdout reports one indexed permanent scenario
    And the catalogue contains the rich fixture
