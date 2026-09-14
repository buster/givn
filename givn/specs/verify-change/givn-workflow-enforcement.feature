Feature: givn Workflow Enforcement

  Scenario: Integrity hook passes when tasks.md commit hashes are present
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains `- [x] COMMIT: a1b2c3d4 -- Implement scenario one`
    And the change's design.md contains a Step Definitions table with valid step file paths
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0
    And stdout contains "integrity"

  Scenario: Integrity hook fails on placeholder commit hash
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains `- [x] COMMIT: ___ -- scenario one`
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "placeholder"
    And stderr contains "revision identifier"

  Scenario: Integrity hook fails on missing step-definition file
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains valid commit hashes
    And the change's design.md Step Definitions table lists `tests/steps/nonexistent.rs`
    And the file tests/steps/nonexistent.rs does not exist
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "Step Definitions"
    And stderr contains "nonexistent.rs"

  Scenario: Integrity hook fails on empty step-definition file
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains valid commit hashes
    And the change's design.md Step Definitions table lists `tests/steps/empty.rs`
    And the file tests/steps/empty.rs exists and is zero bytes
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "empty"
    And stderr contains "Step Definitions"

  Scenario: Integrity hook fails on Step Definitions section that is not a table
    Given a givn project with a change that has passed design-review
    And the change's tasks.md contains valid commit hashes
    And the change's design.md Step Definitions section contains prose with backticks instead of a markdown table
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "must use a markdown table"

  Scenario: Integrity hook passes when arc42 is disabled
    Given a givn project with arc42 addon disabled
    And a change that has passed design-review
    And the change's tasks.md contains valid commit hashes
    And the change's design.md contains a valid Step Definitions table with existing files
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  Scenario: Integrity hook fails on unedited arc42 chapter
    Given a givn project with arc42 addon enabled
    And the change's tasks.md contains valid commit hashes
    And the change's design.md contains a valid Step Definitions table with existing files
    And the change's arc42.md marks chapter "01 introduction-and-goals" as Affected: Yes
    And docs/arc42/01-introduction-and-goals.md is byte-identical to the givn template
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "arc42"
    And stderr contains "01-introduction-and-goals"

  Scenario: Integrity hook passes on edited arc42 chapter
    Given a givn project with arc42 addon enabled
    And the change's tasks.md contains valid commit hashes
    And the change's design.md contains a valid Step Definitions table with existing files
    And the change's arc42.md marks chapter "01 introduction-and-goals" as Affected: Yes
    And docs/arc42/01-introduction-and-goals.md differs from the givn template
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  Scenario: Integrity hook skips arc42 check when slug cannot be resolved
    Given a givn project with arc42 addon enabled
    And the change's tasks.md contains valid commit hashes
    And the change's design.md contains a valid Step Definitions table with existing files
    And the change's arc42.md marks an unresolvable chapter name as Affected: Yes
    And no corresponding docs/arc42/ file exists
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0
    And stdout contains "integrity"

  Scenario: Archive re-runs verify against merged permanent specs and passes
    Given  a givn project with a completed change configured for archive
    And  the project's verify.command exits 0 on the current permanent specs
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits 0
    And  the change directory has been moved to givn/archive/
    And  the archive at givn/archive/integrity-hook-and-archive-gate/ is proven
    And  no archive backup exists for "integrity-hook-and-archive-gate"
    And  the delta scenarios have been merged into givn/specs/

  Scenario: Archive blocked when merged permanent specs fail verify
    Given  a givn project with a completed change configured for archive
    And  the delta spec introduces a scenario that conflicts with an existing permanent scenario
    And  verify.command exits non-zero on the merged permanent spec
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits non-zero
    And  stderr contains "regular verification failed"
    And  stderr contains "unproven"
    And  the change directory has been moved to givn/archive/
    And  the archive at givn/archive/integrity-hook-and-archive-gate/ is unproven
    And  the merged permanent spec preserves the merged delta
    And  no archive backup exists for "integrity-hook-and-archive-gate"

  Scenario: Design.md template includes Internal Primitives section
    Given a givn project
    When the user runs `givn new change-with-primitives`
    Then the change's design.md template contains a section named "Internal Primitives"
    And the section includes a table with columns "Primitive", "Type", and "Purpose"

  Scenario: Archive blocked when delta spec has non-removed @wip scenario
    Given a givn project with an archive-ready change whose delta spec contains @wip
    And the project declares a complete run declaration
    When the user runs `givn archive --change wip-change`
    Then the command exits non-zero
    And stderr contains "@wip"

  Scenario: Archive allowed when @wip only on a @givn.removed placeholder
    Given a givn project with an archive-ready change whose @wip is only on a @givn.removed scenario
    And the project declares a complete run declaration
    When the user runs `givn archive --change wip-removed-change`
    Then the command exits 0
    And the change directory has been moved to givn/archive/

  Scenario: Check tasks blocked when delta spec has @wip
    Given a givn project
    And a change whose delta spec contains @wip
    When the user runs `givn check tasks --change wip-check-change`
    Then the command exits non-zero
    And stderr contains "@wip"
  Scenario: A consumer runs migration coverage and sees a passing receipt
    Given  a migration evidence change with matching behavior snapshots
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits 0
    And  stdout contains "migration coverage passed"
    And  the migration comparison is marked passed

  Scenario: Missing behavior blocks migration coverage
    Given  a migration evidence change whose after snapshot is missing one behavior
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits non-zero
    And  stderr contains "migration coverage regression"
    And  the migration comparison is marked failed

  Scenario: Lost E2E evidence blocks migration coverage
    Given  a migration evidence change that loses E2E evidence
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits non-zero
    And  stderr contains "migration coverage regression"

  Scenario: Lower interaction coverage blocks migration coverage
    Given  a migration evidence change with lower interaction coverage
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits non-zero
    And  stderr contains "migration coverage regression"

  Scenario: Lower line coverage blocks migration coverage
    Given  a migration evidence change with lower line coverage
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits non-zero
    And  stderr contains "migration coverage regression"

  Scenario: Lower branch coverage blocks migration coverage
    Given  a migration evidence change with lower branch coverage
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits non-zero
    And  stderr contains "migration coverage regression"

  Scenario: Explicit retirement documents a changed behavior
    Given  a migration evidence change with an explicitly retired behavior
    When  the user runs `givn check migration --change migration-test`
    Then  the command exits 0
    And  stdout contains "migration coverage passed"

  @e2e
  Scenario: Consumer runs migration coverage check and sees passed evidence
    Given  a migration evidence change with matching behavior snapshots
    When  the consumer runs the migration coverage check
    Then  the command exits 0
    And  stdout contains "migration coverage passed"
  Scenario: A failed new-capability candidate tells the developer how to repair it
    Given  a givn project with a completed change that introduces a brand-new capability
    And  verify.command exits non-zero on the merged permanent spec
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits non-zero
    And  stderr says the candidate is unproven and can be repaired in place
    And  the archive at givn/archive/integrity-hook-and-archive-gate/ is unproven

  Scenario: Review does not run the full suite
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains valid commit hashes
    And  the change's design.md contains a valid Step Definitions table with existing files
    And  the project declares a complete run declaration
    And  the project records whether its verify commands run
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  the verify command did not run
