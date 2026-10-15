@givn.delta @enforce-post-merge-e2e-gate

Feature: Enforce Post-Merge E2E Gate

  @givn.modified
  Scenario: Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is unconfigured
    Given a project root with givn/ initialised
    And the givn/commands.yaml has e2e_command unconfigured
    And a completed change "add-e2e-scenario" whose delta contains an @e2e-tagged scenario
    And the permanent specs currently have no @e2e scenarios
    And the project declares a complete run declaration
    When the user runs `givn archive --change add-e2e-scenario`
    Then the command exits non-zero
    And stderr contains "verify.e2e_command"
    And stderr mentions the e2e runner is not configured
    And the change directory still exists under givn/changes/

  @givn.modified
  Scenario: Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured
    Given a project root with givn/ initialised
    And the givn/commands.yaml has e2e_command unconfigured
    And a completed change "no-e2e" whose delta contains zero @e2e-tagged scenarios
    And the permanent specs currently have no @e2e scenarios
    And the project declares a complete run declaration
    When the user runs `givn archive --change no-e2e`
    Then the command exits 0
    And the change directory has been moved to givn/archive/

  @givn.modified @e2e
  Scenario: Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
    Given  a committed givn project with verify.e2e_command unconfigured in givn/commands.yaml
    And  a change "real-e2e-merge" with all artifacts done, whose delta spec has an @e2e scenario
    And  the permanent spec directory givn/specs/ has no @e2e scenario
    When  the user invokes `givn archive --change real-e2e-merge`
    Then  the command fails
    And  stderr says the e2e runner is not configured for permanent specs with @e2e coverage
    And  stderr names givn/commands.yaml as the e2e runner's configuration file
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "real-e2e-merge"
