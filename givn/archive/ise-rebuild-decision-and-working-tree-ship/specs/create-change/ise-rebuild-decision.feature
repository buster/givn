@givn.delta @ise-rebuild-decision

Feature: ise-rebuild-decision

  @givn.added @ise-rebuild-decision
  Scenario: The design guidance names the working-tree ship practice
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design guidance for an active change
    Then  the command exits 0
    And  the served design guidance states that a Single-Component Rebuild with `Rebuilt code: ship` follows the working-tree sequence
    And  the served design guidance names the `ise` instruction as the owner of that sequence

  @givn.added @ise-rebuild-decision
  Scenario: The design guidance omits the Rebuilt code line for No rebuild
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design guidance for an active change
    Then  the command exits 0
    And  the served design guidance states that a No rebuild change records no Rebuilt code line
