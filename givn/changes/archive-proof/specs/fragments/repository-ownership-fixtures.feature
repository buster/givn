@givn.delta @repository-ownership-fixtures
Feature: Repository-wide ownership fixtures

  @givn.removed @wip
  Scenario: Pre-merge archive failure preserves the permanent specification tree
    Given the scenario is retired

  @givn.added @wip
  Scenario: A failed archive preserves the merged candidate unproven
    Given an isolated ownership fixture with a failing verification command
    And the project declares a complete run declaration
    When the maintainer runs `givn archive --change fixture-consolidation`
    Then the fixture command fails
    And the fixture archive candidate for "fixture-consolidation" is unproven
    And the fixture permanent specification tree contains "Canonical retained behavior"
