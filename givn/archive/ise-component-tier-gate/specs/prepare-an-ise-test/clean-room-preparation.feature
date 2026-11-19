@givn.delta @clean-room-preparation
Feature: Clean-room preparation

  @givn.added
  Scenario: Preparation writes a runnable verification contract
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    Then  the command succeeds
    And  the spec copy holds "givn/ise-verification.yaml"
    And  the verification contract declares 10 component execution units and 1 End-to-end execution unit
