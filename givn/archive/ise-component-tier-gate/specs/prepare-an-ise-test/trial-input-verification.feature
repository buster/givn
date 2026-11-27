@givn.delta @trial-input-verification
Feature: Trial input verification

  @givn.added
  Scenario: Comparison labels frozen-input equality as unverified
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: match"
    And  comparison exits zero
    And  comparison prints a "Trial result: unverified" line
    And  the command prints no "Trial result: passed" line

  @givn.added
  Scenario: Verification refuses a missing component runner
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the agent writes a component verification script that writes no runner result
    And  the verifier runs the trial verification
    Then  the trial verification fails and exits 1, naming "component_executed=0/10"
    And  the trial result records "component_executed=0/10"
    And  the trial result records "clean": false
    And  the command prints no "Trial result: passed" line

  @givn.added
  Scenario: Verification refuses a count-only component result
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the agent writes a count-only component verification script
    And  the verifier runs the trial verification
    Then  the trial verification fails and exits 1, naming "does not name"
    And  the trial result records "clean": false

  @givn.added
  Scenario: Verification refuses an End-to-end result bound to another product
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the agent writes a component verification script reporting 10 executed scenarios
    And  the agent writes an End-to-end verification script that reports another product
    And  the verifier runs the trial verification
    Then  the trial verification fails and exits 1, naming "product_sha256"
    And  the trial result records "clean": false

  @givn.added
  Scenario: Verification records the exact declared counts
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the agent writes a component verification script reporting 10 executed scenarios
    And  the verifier runs the trial verification
    Then  the trial verification succeeds and prints "Component scenarios: 10/10 executed, 0 failed, 0 skipped"
    And  the trial verification succeeds and prints "End-to-end tests: 1/1 executed, 0 failed, 0 skipped"
    And  the trial result records "clean": true
    And  the trial result names ten component scenarios and the rebuilt product digest

  @givn.added
  Scenario: Verification refuses an exact count mismatch
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the agent writes a component verification script reporting 9 executed scenarios
    And  the verifier runs the trial verification
    Then  the trial verification fails and exits 1, naming "component_executed=9/10"
    And  the trial result records "clean": false

  @givn.added
  Scenario: A not-run decision cannot replace declared component units
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the verifier runs the trial verification with the not-run decision "out of scope"
    Then  the trial verification fails and exits 1, naming "cannot replace their execution"
    And  the trial result records "clean": false
