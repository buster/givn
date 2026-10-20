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
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains `- [x] COMMIT: ___ -- scenario one`
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "placeholder"
    And  stderr contains "revision identifier"

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
  Scenario: An opaque revision identifier passes the integrity check
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains `- [x] COMMIT: 6f5a2c9e8b7d4a3f91c2e5d8b7a4f3c2e1d0b9a8 -- Implement scenario one`
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  stdout contains "integrity"
  @e2e
  Scenario: Consumer runs check tasks and receives a change verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    When  the user invokes `givn check tasks --change verify-demo`
    Then  the command succeeds
    And  the change directory contains a passed verification receipt naming the tested revision

  Scenario: Review refuses a change without a verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    And  the change has no verification receipt
    When  the user runs `givn check review --change verify-demo`
    Then  the command exits non-zero
    And  stderr routes to `givn check tasks --change verify-demo`

  Scenario: Review refuses a receipt after the delta specs changed
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    And  the change has a current verification receipt
    And  the delta specs changed after the receipt
    When  the user runs `givn check review --change verify-demo`
    Then  the command exits non-zero
    And  stderr says the verification receipt is stale

  Scenario: Review accepts a current verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    And  the change has a current verification receipt
    When  the user runs `givn check review --change verify-demo`
    Then  the command exits 0
    And  stdout contains "change verification receipt passed"

  Scenario: The result helper writes four counts without a scope
    Given  a givn project with a result file path
    When  the user runs the result helper for 3 total and 3 passed
    Then  the command exits 0
    And  the result file contains total 3, passed 3, failed 0 and skipped 0 without a scope
  Scenario: A checked scenario without a commit fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks a scenario without a commit
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr names the scenario and the missing commit

  Scenario: A checked scenario reusing another revision fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks two scenarios with the same revision
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr says each scenario needs its own commit

  Scenario: A commit message that omits the scenario title fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks a scenario whose commit message omits the title
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr says the commit does not name the scenario

  Scenario: One real revision per scenario passes the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks one scenario with its own commit
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits 0
    And  stdout says scenario commits passed
  @e2e
  Scenario: Consumer runs review and receives every use-case schema problem in one pass
    Given  a clean committed change whose delta capability is routed "NEW in review-diagnostics-demo"
    And  the use-case contract has a case-mismatched title, an unknown "## Guarantees" heading, a capability line that is not a bullet, an interaction that names an undeclared capability, and an example line that is not a bullet
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr reports the title, the unknown heading, the capability line, the undeclared interaction, and the example line together in one review run
    And  stderr reports each distinct document problem once
    And  the terminal transcript is captured for this scenario

  Scenario: A case-mismatched document title is refused with the exact correction
    Given  a clean committed change whose use-case contract starts with "# Use Case: review-diagnostics-demo"
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr says headings are case-sensitive
    And  stderr suggests "# Use case: review-diagnostics-demo"

  Scenario: A near-miss section heading is refused with the exact accepted spellings
    Given  a clean committed change whose use-case contract has an unknown "## Guarantees" heading
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr suggests "## Minimal guarantee"
    And  stderr suggests "## Success guarantee"

  Scenario: Missing required sections and a malformed interaction table are reported together
    Given  a clean committed change whose use-case contract omits "## Minimal guarantee" and "## Success guarantee"
    And  the interaction table separator row has the wrong column count
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr names both missing sections and the malformed interaction table together

  Scenario: One use-case parse failure is reported once across several delta capabilities
    Given  a clean committed change with three delta capabilities routed "NEW in review-diagnostics-demo"
    And  the use-case contract has a case-mismatched title and an unknown "## Guarantees" heading
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr names the case-mismatched title once
    And  stderr names the unknown heading once

  Scenario: Review validates an active delta use-case narrative even for an extension
    Given  a clean committed change that replaces an existing use-case narrative and whose routing decision extends that use case's capability
    And  the replacement narrative has a case-mismatched title and an unknown "## Guarantees" heading
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr names the replacement narrative's title and unknown heading

  Scenario: Review accepts a green proof after documentation-only review edits
    Given  a clean committed change with a green change verification receipt recorded at one revision
    And  the developer commits only proposal, design, tasks, review, arc42, and use-case narrative edits after that revision
    When  the developer runs `givn check review --change review-proof-demo`
    Then  the command exits 0
    And  the change suite did not run again
    And  stdout says the change verification receipt was reused
