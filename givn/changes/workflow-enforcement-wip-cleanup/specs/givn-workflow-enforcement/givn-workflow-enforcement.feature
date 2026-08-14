# User Interaction Inventory:
# - No new user interaction is introduced; this change activates existing
#   workflow-enforcement interactions without changing their observable results.

@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  @givn.modified
  Scenario: Integrity hook passes when tasks.md commit hashes are present
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains `- [x] COMMIT: a1b2c3d4 -- Implement scenario one`
    And  the change's design.md contains a Step Definitions table with valid step file paths
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  stdout contains "integrity"

  @givn.modified
  Scenario: Integrity hook fails on placeholder commit hash
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains `- [x] COMMIT: ___ -- scenario one`
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "placeholder"
    And  stderr contains "commit hash"

  @givn.modified
  Scenario: Integrity hook fails on missing step-definition file
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md Step Definitions table lists `tests/steps/nonexistent.rs`
    And  the file tests/steps/nonexistent.rs does not exist
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "Step Definitions"
    And  stderr contains "nonexistent.rs"

  @givn.modified
  Scenario: Integrity hook fails on empty step-definition file
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md Step Definitions table lists `tests/steps/empty.rs`
    And  the file tests/steps/empty.rs exists and is zero bytes
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "empty"
    And  stderr contains "Step Definitions"

  @givn.modified
  Scenario: Integrity hook fails on Step Definitions section that is not a table
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md Step Definitions section contains prose with backticks instead of a markdown table
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "must use a markdown table"

  @givn.modified
  Scenario: Integrity hook passes when arc42 is disabled
    Given  a givn project with arc42 addon disabled
    And  a change that has passed design-review
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md contains a valid Step Definitions table with existing files
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  @givn.modified
  Scenario: Integrity hook fails on unedited arc42 chapter
    Given  a givn project with arc42 addon enabled
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md contains a valid Step Definitions table with existing files
    And  the change's arc42.md marks chapter "01 introduction-and-goals" as Affected: Yes
    And  docs/arc42/01-introduction-and-goals.md is byte-identical to the givn template
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "arc42"
    And  stderr contains "01-introduction-and-goals"

  @givn.modified
  Scenario: Integrity hook passes on edited arc42 chapter
    Given  a givn project with arc42 addon enabled
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md contains a valid Step Definitions table with existing files
    And  the change's arc42.md marks chapter "01 introduction-and-goals" as Affected: Yes
    And  docs/arc42/01-introduction-and-goals.md differs from the givn template
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  @givn.modified
  Scenario: Integrity hook skips arc42 check when slug cannot be resolved
    Given  a givn project with arc42 addon enabled
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md contains a valid Step Definitions table with existing files
    And  the change's arc42.md marks an unresolvable chapter name as Affected: Yes
    And  no corresponding docs/arc42/ file exists
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  stdout contains "integrity"

  @givn.modified
  Scenario: Archive re-runs verify against merged permanent specs and passes
    Given  a givn project with a completed change configured for archive
    And  the project's verify.command exits 0 on the current permanent specs
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits 0
    And  the change directory has been moved to givn/archive/
    And  the backup directory givn/.archive-backup-integrity-hook-and-archive-gate does not exist
    And  the delta scenarios have been merged into givn/specs/

  @givn.modified
  Scenario: Archive blocked when merged permanent specs fail verify
    Given  a givn project with a completed change configured for archive
    And  the delta spec introduces a scenario that conflicts with an existing permanent scenario
    And  verify.command exits non-zero on the merged permanent spec
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits non-zero
    And  stderr contains "verify"
    And  stderr contains "permanent"
    And  stderr contains "backup"
    And  the change directory still exists under givn/changes/
    And  the backup directory givn/.archive-backup-integrity-hook-and-archive-gate exists with the pre-merge permanent specs
    And  the merged permanent spec has been rolled back to its pre-archive state

  @givn.modified
  Scenario: New-capability merge failure produces clear recovery instructions
    Given  a givn project with a completed change that introduces a brand-new capability
    And  verify.command exits non-zero on the merged permanent spec
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits non-zero
    And  stderr says the specs have been rolled back
    And  the backup directory exists

  @givn.modified
  Scenario: Design.md template includes Internal Primitives section
    Given  a givn project
    When  the user runs `givn new change-with-primitives`
    Then  the change's design.md template contains a section named "Internal Primitives"
    And  the section includes a table with columns "Primitive", "Type", and "Purpose"

  @givn.modified
  Scenario: Archive blocked when delta spec has non-removed @wip scenario
    Given  a givn project with an archive-ready change whose delta spec contains @wip
    When  the user runs `givn archive --change wip-change`
    Then  the command exits non-zero
    And  stderr contains "@wip"

  @givn.modified
  Scenario: Archive allowed when @wip only on a @givn.removed placeholder
    Given  a givn project with an archive-ready change whose @wip is only on a @givn.removed scenario
    When  the user runs `givn archive --change wip-removed-change`
    Then  the command exits 0
    And  the change directory has been moved to givn/archive/

  @givn.modified @wip
  Scenario: Check tasks blocked when delta spec has @wip
    Given  a givn project
    And  a change whose delta spec contains @wip
    When  the user runs `givn check tasks --change wip-check-change`
    Then  the command exits non-zero
    And  stderr contains "@wip"
