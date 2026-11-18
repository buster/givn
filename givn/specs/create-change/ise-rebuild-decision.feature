@ise-rebuild-decision
Feature: ise-rebuild-decision

  @e2e @ise-rebuild-decision
  Scenario: The design guidance requires one Ise rebuild decision
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design guidance for an active change
    Then  the command exits 0
    And  the served design guidance requires exactly one Ise rebuild decision as a `Decides D` entry
    And  the served design guidance names No rebuild, Single-Component Rebuild, and Whole-system rebuild
    And  the served design guidance records a plain Rebuilt code line with discard or ship for a rebuilding change
    And  the served design guidance requires the affected consumers, the persisted state, and the rebuild boundary
    And  the terminal transcript is captured for this scenario

  @ise-rebuild-decision
  Scenario: The design-review guidance keeps a rebuild without a Rebuilt code line a finding
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design-review guidance
    Then  the command exits 0
    And  the served design-review guidance keeps a rebuilding change with no recorded Rebuilt code line a finding
    And  the served design-review guidance keeps an unresolved Ise rebuild decision a finding

  @ise-rebuild-decision
  Scenario: The design guidance names the working-tree ship practice
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design guidance for an active change
    Then  the command exits 0
    And  the served design guidance states that a Single-Component Rebuild with `Rebuilt code: ship` follows the working-tree sequence
    And  the served design guidance names the `ise` instruction as the owner of that sequence

  @ise-rebuild-decision
  Scenario: The design guidance omits the Rebuilt code line for No rebuild
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design guidance for an active change
    Then  the command exits 0
    And  the served design guidance states that a No rebuild change records no Rebuilt code line
