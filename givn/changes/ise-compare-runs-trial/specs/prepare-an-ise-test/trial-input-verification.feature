@givn.delta @trial-input-verification
Feature: Trial input verification

  @givn.removed @e2e
  Scenario: Verifier compares the spec copy with the manifest
    Given  a placeholder step

  @givn.added @e2e
  Scenario: Verifier compares the spec copy with the manifest and runs the trial
    Given  a prepared spec copy with a manifest
    When  the verifier compares the spec copy with the manifest and runs the trial
    Then  comparison prints "Specification files: match" and "End-to-end tests: match"
    And  comparison exits zero
    And  comparison prints no line beginning "- "
    And  comparison prints "Trial result: passed"

  @givn.modified
  Scenario: Comparison labels frozen-input equality as unverified
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the verifier compares the spec copy with the manifest
    Then  comparison prints "Specification files: match" and "End-to-end tests: match"
    And  comparison exits zero
    And  comparison prints a "Trial result: unverified" line
    And  the command prints no "Trial result: passed" line
    And  the comparison writes no trial result

  @givn.added
  Scenario: Comparison refuses a missing component runner
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the agent writes a component verification script that writes no runner result
    And  the verifier compares the spec copy with the manifest and runs the trial
    Then  the trial verification fails and exits 1, naming "component_executed=0/10"
    And  the trial result records "component_executed=0/10"
    And  the trial result records "clean": false
    And  the command prints no "Trial result: passed" line

  @givn.added
  Scenario: Comparison refuses a trial result inside the spec copy
    Given  a committed verification fixture with ten component scenarios and one End-to-end scenario
    When  the verifier prepares a trial with ten component scenarios and one End-to-end scenario
    And  the verifier compares the spec copy with the manifest and writes the trial result into the copy
    Then  comparison fails and exits 1, naming "inside the spec copy"
    And  comparison prints no "Trial result: passed" line
