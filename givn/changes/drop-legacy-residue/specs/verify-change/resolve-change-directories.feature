@givn.delta @resolve-change-directories

Feature: Resolve change directories

  @givn.modified
  Scenario: Verify-e2e hook resolves the change directory from a relative features path inside givn/changes
    Given a project root with givn/ initialised
    And the givn/commands.yaml has e2e_command unconfigured
    And a change "e2e-detect" exists with design.md containing a valid Step Definitions table
    And a change "e2e-detect" exists with tasks.md where every COMMIT hash is a valid 40-character hex string
    And a change "e2e-detect" exists with specs under "specs/" containing zero @e2e scenarios
    And the project declares a complete run declaration
    When the user runs `givn check review --change e2e-detect`
    Then the command exits 0
