# User Interaction Inventory:
# - run `givn archive --change <id>` with a delta whose permanent merge adds @e2e scenarios
# - configure givn/config.yaml with an empty e2e_command
# - configure givn/config.yaml with a valid e2e_command

@givn.delta @enforce-post-merge-e2e-gate

Feature: Enforce Post-Merge E2E Gate

  @givn.added
  Scenario: Archive is blocked when merged permanent specs gain @e2e scenarios and e2e_command is empty
    Given a project root with givn/ initialised
    And the givn/config.yaml has an empty e2e_command
    And a completed change "add-e2e-scenario" whose delta contains an @e2e-tagged scenario
    And the permanent specs currently have no @e2e scenarios
    When the user runs `givn archive --change add-e2e-scenario`
    Then the command exits non-zero
    And stderr contains "verify.e2e_command"
    And stderr mentions the e2e runner is not configured
    And the change directory still exists under givn/changes/

  @givn.added @wip
  Scenario: Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is empty
    Given a project root with givn/ initialised
    And the givn/config.yaml has an empty e2e_command
    And a completed change "no-e2e" whose delta contains zero @e2e-tagged scenarios
    And the permanent specs currently have no @e2e scenarios
    When the user runs `givn archive --change no-e2e`
    Then the command exits 0
    And the change directory has been moved to givn/archive/

  @givn.added @wip
  Scenario: Archive runs e2e_command when merged permanent specs contain @e2e and e2e_command is configured
    Given a project root with givn/ initialised
    And the givn/config.yaml has e2e_command set to "true"
    And a completed change "configured-e2e" whose delta contains an @e2e-tagged scenario
    When the user runs `givn archive --change configured-e2e`
    Then the command exits 0
    And stdout contains "Running post-merge verify-e2e"
    And the change directory has been moved to givn/archive/

  @givn.added @e2e @wip
  Scenario: Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
    Given a givn project with an empty verify.e2e_command in givn/config.yaml
    And a change "real-e2e-merge" with all artifacts done, whose delta spec has an @e2e scenario
    And the permanent spec directory givn/specs/ has no @e2e scenario
    When the user invokes `givn archive --change real-e2e-merge`
    Then the command fails
    And stderr says the e2e runner is not configured for permanent specs with @e2e coverage
    And the specs directory is rolled back to its pre-archive state
