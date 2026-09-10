@givn.delta @deduplicate-change-dir-detection
Feature: Deduplicate Change Directory Detection

  @givn.modified
  Scenario: Integrity hook resolves the change directory from a relative features path inside givn/changes
    Given a project root with givn/ initialised
    And a change "detect-demo" exists with artifact "proposal.md"
    And a change "detect-demo" exists with specs under "specs/"
    And the project declares a complete run declaration
    When the user runs `givn check review --change detect-demo`
    Then the command exits 0
    And stdout contains "integrity"
    And stdout does not contain "the change directory could not be detected"
