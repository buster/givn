@givn.delta @givn-workflow-enforcement
Feature: givn Workflow Enforcement

  @givn.modified @wip
  Scenario: Archive re-runs verify against merged permanent specs and passes
    Given a givn project with a completed change configured for archive
    And the project's verify.command exits 0 on the current permanent specs
    And the project declares a complete run declaration
    When the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then the command exits 0
    And the change directory has been moved to givn/archive/
    And the archive at givn/archive/integrity-hook-and-archive-gate/ is proven
    And no archive backup exists for "integrity-hook-and-archive-gate"
    And the delta scenarios have been merged into givn/specs/

  @givn.modified @wip
  Scenario: Archive blocked when merged permanent specs fail verify
    Given a givn project with a completed change configured for archive
    And the delta spec introduces a scenario that conflicts with an existing permanent scenario
    And verify.command exits non-zero on the merged permanent spec
    And the project declares a complete run declaration
    When the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then the command exits non-zero
    And stderr contains "verify"
    And stderr contains "permanent"
    And the change directory has been moved to givn/archive/
    And the archive at givn/archive/integrity-hook-and-archive-gate/ is unproven
    And the merged permanent spec preserves the merged delta
    And no archive backup exists for "integrity-hook-and-archive-gate"

  @givn.removed @wip
  Scenario: New-capability merge failure produces clear recovery instructions
    Given the scenario is retired

  @givn.added @wip
  Scenario: A failed new-capability candidate tells the developer how to repair it
    Given a givn project with a completed change that introduces a brand-new capability
    And verify.command exits non-zero on the merged permanent spec
    And the project declares a complete run declaration
    When the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then the command exits non-zero
    And stderr says the candidate is unproven and can be repaired in place
    And the archive at givn/archive/integrity-hook-and-archive-gate/ is unproven

  @givn.added @wip
  Scenario: Review does not run the full suite
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains valid commit hashes
    And the change's design.md contains a valid Step Definitions table with existing files
    And the project declares a complete run declaration
    And the project records whether its verify commands run
    When the user runs `givn check review --change test-change`
    Then the command exits 0
    And the verify command did not run
