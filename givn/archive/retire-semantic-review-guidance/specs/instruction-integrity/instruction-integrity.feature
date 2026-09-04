@givn.delta @instruction-integrity
Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.removed
  Scenario: Review guidance verifies semantic remediation
    Given  a semantic review contains classified candidates

  @givn.removed
  Scenario: A reviewer requests remediation-aware review guidance
    Given  a Givn project with an active change

  @givn.added
  Scenario: Review guidance names the deterministic disposition gates
    Given  the review instructions are requested
    When  the reviewer reads the deterministic gate guidance
    Then  the guidance requires an overlap disposition for every shape match
    And  the guidance requires a split-or-keep decision for every long scenario

  @givn.added
  @e2e
  Scenario: A reviewer requests disposition-aware review guidance
    Given  a Givn project with an active change
    When  the reviewer requests the review instructions through the Givn CLI
    Then  the CLI output contains the deterministic gate dispositions guidance
