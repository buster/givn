# User Interaction Inventory:
# - invoke repository-wide review for a project with explicit overlap dispositions
# - archive a consolidated project and publish its permanent specifications
# - invoke archive for a project whose pre-merge hook fails

@givn.delta @repository-ownership-fixtures
Feature: Repository-wide ownership fixtures

  @givn.added @e2e
  Scenario: Repository-wide review accepts explicit overlap dispositions
    Given an isolated ownership fixture with dispositions for every overlap finding
    When the maintainer invokes the fixture review command
    Then the fixture command exits 0
    And fixture stdout contains "overlap dispositions passed"
    And fixture stdout contains "net delta: 1 added, 0 modified, 1 removed"

  @givn.added @e2e
  Scenario: Archive publishes consolidated permanent specifications
    Given an isolated ownership fixture with dispositions for every overlap finding
    When the maintainer invokes the fixture archive command
    Then the fixture command exits 0
    And fixture stdout contains "Archived 'fixture-consolidation'"
    And the fixture permanent specification tree contains no duplicate scenario titles
    And the fixture permanent specification tree contains "Canonical retained behavior"
    And the fixture permanent specification tree does not contain "Obsolete behavior"

  @givn.added @e2e
  Scenario: Pre-merge archive failure preserves the permanent specification tree
    Given an isolated ownership fixture with a failing pre-merge archive hook
    When the maintainer runs `givn archive --change fixture-consolidation`
    Then the fixture command fails
    And the fixture permanent specification tree remains unchanged
