@givn.delta @grouped-merge-for-new-capabilities

Feature: grouped-merge-for-new-capabilities

  @givn.removed
  Scenario: A new capability without a declared group keeps the flat merge layout
    Given a placeholder step

  @e2e @givn.added
  Scenario: Archive is blocked when a new capability has no declared routing decision
    Given a givn project with verify.e2e_command unconfigured in givn/config.yaml
    And a completed change "flat-new" whose proposal records no routing decision and whose delta lives under specs/flat-new/
    And the project declares a complete run declaration
    When the user invokes `givn archive --change flat-new`
    Then the command exits non-zero
    And stderr contains "records no 'NEW in"
    And the directory "givn/specs/flat-new" does not exist
    And the change directory still exists under givn/changes/
