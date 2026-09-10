@givn.delta @enforce-post-merge-e2e-gate
Feature: Enforce Post-Merge E2E Gate

  @givn.modified @wip
  Scenario: Archive runs e2e_command when merged permanent specs contain @e2e and e2e_command is configured
    Given a givn project whose e2e_command reports scenario counts
    And a completed change "configured-e2e" whose delta contains an @e2e-tagged scenario and is ready to archive
    When the user runs `givn archive --change configured-e2e`
    Then the command exits 0
    And stdout contains "Running post-merge verify-e2e"
    And the verification receipt records the end-to-end scope scenario counts

  @givn.added @wip
  Scenario: A failing end-to-end scope leaves the candidate unproven
    Given a givn project with the e2e_command set to "false"
    And a completed change "failing-e2e" whose delta contains an @e2e-tagged scenario and is ready to archive
    When the user runs `givn archive --change failing-e2e`
    Then the command exits non-zero
    And stderr mentions the e2e scope
    And the archive at givn/archive/failing-e2e/ is unproven
    And the merged permanent specs preserve the delta scenarios

  @givn.modified @e2e @wip
  Scenario: Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner
    Given a committed givn project with verify.e2e_command unconfigured in givn/config.yaml
    And a change "real-e2e-merge" with all artifacts done, whose delta spec has an @e2e scenario
    And the permanent spec directory givn/specs/ has no @e2e scenario
    When the user invokes `givn archive --change real-e2e-merge`
    Then the command fails
    And stderr says the e2e runner is not configured for permanent specs with @e2e coverage
    And the change directory still exists under givn/changes/
    And no archive candidate exists for "real-e2e-merge"
