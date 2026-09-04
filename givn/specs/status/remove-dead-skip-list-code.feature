Feature: Remove Dead Skip List Code

  Scenario: givn status still reports skipped artifacts with the skip marker after dead code removal
    Given a project root with givn/ initialised
    And a change "skip-demo" exists with artifact "proposal.md"
    And the change "skip-demo" has ".givn-skip" listing "design"
    When the user runs `givn status --change skip-demo`
    Then the command exits 0
    And stdout mentions "design"

  Scenario: givn status JSON still reports skipped artifacts after dead code removal
    Given a project root with givn/ initialised
    And a change "skip-json" exists with artifact "proposal.md"
    And the change "skip-json" has ".givn-skip" listing "design"
    When the user runs `givn status --change skip-json --json`
    Then the command exits 0
    And stdout mentions "\"id\": \"design\""
    And stdout mentions "\"status\": \"skipped\""

  @e2e
  Scenario: Consumer runs givn status on a change with skipped artifacts and sees them correctly
    Given a givn project with a change "skip-real" that has proposal.md and a .givn-skip file listing "design"
    When the user invokes `givn status --change skip-real`
    Then the command succeeds
    And stdout shows design as skipped
