Feature: Validate Artifact ID Input

  Scenario: givn check with unknown artifact validates against manifest and provides helpful error
    Given  a project root with givn/ initialised
    When  the user runs `givn check nonexistent`
    Then  the command exits non-zero
    And  stderr contains "unknown artifact"
    And  stderr lists known artifact ids

  Scenario: givn check with known artifact proceeds to run hooks
    Given  a project root with givn/ initialised
    And  a change "check-demo" exists with artifact "proposal.md"
    When  the user runs `givn check proposal --change check-demo`
    Then  the command exits 0

  Scenario: givn eject instruction with unknown artifact validates against manifest and provides helpful error
    Given  a project root with givn/ initialised
    When  the user runs `givn eject nonexistent:instruction`
    Then  the command exits non-zero
    And  stderr says the artifact is not in the effective manifest
    And  stderr lists known artifact ids

  @e2e
  Scenario: Consumer invokes givn check with an unknown artifact and gets a helpful error
    Given  a givn project with a standard manifest
    When  the user invokes `givn check bogus-artifact`
    Then  the command fails
    And  stderr contains "unknown artifact"
