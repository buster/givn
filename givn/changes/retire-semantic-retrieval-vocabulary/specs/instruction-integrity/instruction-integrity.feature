@givn.delta @instruction-integrity
Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.modified
  Scenario: An author requests the canonical interaction and E2E guidance
    Given  a Givn project with an active change
    When  the author requests the specs instructions through the Givn CLI
    Then  the CLI output contains the complete interaction and E2E policy
    And  the CLI output no longer advertises retrieval commands
