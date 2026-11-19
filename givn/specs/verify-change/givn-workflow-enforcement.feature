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

  Scenario: Review refuses a change whose delta fails the lint gate
    Given a givn project with a change that has passed design-review
    And a .feature file at "givn/changes/test-change/specs/cap-a/a.feature" with:
      """
      Feature: Cap A
        Scenario: Repeated title
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And a .feature file at "givn/changes/test-change/specs/cap-b/b.feature" with:
      """
      Feature: Cap B
        Scenario: Repeated title
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "lint gate failed"

  @givn-workflow-enforcement
  Scenario: Archive re-runs the static review gates
    Given  a givn project with a completed change configured for archive
    And  the change's review.md has no README impact decision
    When  the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then  the command exits non-zero and names the missing README impact decision
    And  the active change and permanent specs are unchanged
    And  no verification suite executes

  Scenario: Archive refuses a change without a signed design-review
    Given a givn project with a completed change configured for archive
    And the change's design-review.md is removed
    And the project's verify.command exits 0 on the current permanent specs
    When the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then the command exits non-zero
    And stderr contains "design-review"
    And stderr contains "does not exist"

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

  @givn-workflow-enforcement
  Scenario: Review does not run the full suite
    Given  a change has current applicable verification evidence
    And  the completed review and applicable integrity checks pass
    And  the project records every verification command invocation
    When  the developer runs `givn check review --change test-change`
    Then  the command succeeds
    And  neither change-scoped nor full-suite verification executes
    And  no future archive receipt is required

  Scenario: An opaque revision identifier passes the integrity check
    Given  a givn project with a change that has passed design-review
    And  the change's tasks.md contains `- [x] COMMIT: 6f5a2c9e8b7d4a3f91c2e5d8b7a4f3c2e1d0b9a8 -- Implement scenario one`
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  stdout contains "integrity"

  @e2e @givn-workflow-enforcement
  Scenario: Consumer runs check tasks and receives a change verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    When  the user invokes `givn check tasks --change verify-demo`
    Then  the command succeeds
    And  stdout reports passed change-scoped verification
    And  the change contains a passed receipt bound to the revision, selected features, runner command and relevant inputs captured before execution
    And  the terminal transcript is captured for this scenario

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

  @e2e
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

  @givn-workflow-enforcement
  Scenario: Two scenarios sharing a batch commit pass the task check
    Given  a change checks two scenarios in one named behavior batch
    And  both checked scenarios record the same reachable revision
    And  human review confirms that revision contains their implementation or regression tests
    And  the revision message names the behavior batch
    And  applicable change verification passes
    When  the developer runs `givn check tasks --change verify-demo`
    Then  the command succeeds and reports that scenario revisions passed
    And  no separate revision or production edit per scenario is required
    And  no automated semantic diff classifier is required

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

  Scenario: Review corrects a non-kebab-case use-case title to its canonical identifier
    Given  a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and has an unknown "## Guarantees" heading
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr says headings are case-sensitive
    And  stderr suggests exactly "# Use case: lending-register"
    And  stderr contains "unknown heading '## Guarantees'"

  Scenario: Review does not accept a non-kebab-case title as the use-case identifier
    Given  a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and whose capability list is a table
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr does not contain "leaf use case 'Lending Register'"
    And  stderr does not contain "did you mean '# Use case: Lending Register'"
    And  stderr does not contain "does not declare capability"
    And  stderr does not contain "has no interaction for capability"

  Scenario: Review corrects an exact-prefix non-kebab-case title with the canonical identifier
    Given  a clean committed change whose use-case contract starts with "# Use case: Lending Register"
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr contains "must be a stable kebab-case identifier"
    And  stderr suggests exactly "# Use case: lending-register"
    And  stderr does not contain "document is missing '# Use case:"

  Scenario: A malformed title does not fabricate missing-section or empty-section diagnostics
    Given  a clean committed change whose use-case contract starts with "# Use Case: Lending Register", omits "## Rules", and leaves "## Examples" empty
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr suggests exactly "# Use case: lending-register"
    And  stderr does not contain "document schema does not recognise '## Rules'"
    And  stderr does not contain "section '## Examples' must not be empty"

  Scenario: A malformed Level does not fabricate missing-section or leaf-capability diagnostics
    Given  a clean committed change whose use-case contract has a malformed Level and omits "## Examples"
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr contains "use-case Level must be '!' or '+'"
    And  stderr does not contain "document schema does not recognise '## Examples'"
    And  stderr does not contain "must declare at least one capability"

  Scenario: Independent local findings are still reported with a malformed title
    Given  a clean committed change whose use-case contract starts with "# Use Case: Lending Register", has a rule line that is not a bullet, and has an interaction that names an undeclared capability
    When  the developer runs `givn check review --change review-diagnostics-demo`
    Then  the command exits non-zero
    And  stderr contains "section '## Rules' must contain one rule per bullet"
    And  stderr does not contain "interaction references undeclared capability"
    And  stderr suggests exactly "# Use case: lending-register"

  Scenario: A valid kebab-case use-case title remains accepted
    Given  a clean committed change with a green change verification receipt recorded at one revision
    When  the developer runs `givn check review --change review-proof-demo`
    Then  the command exits 0
    And  stderr does not contain "expected '# Use case:"
    And  stderr does not contain "document schema does not recognise"

  Scenario: A recorded self-review is accepted
    Given  a givn project with a change that has passed design-review
    And  the change's proposal records review kind "self-review"
    And  the change's design-review records review kind "self-review" by author "agent" in reviewer context "agent"
    And  the change's review records review kind "self-review" by author "agent" in reviewer context "agent"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: An independent review by a different context is accepted
    Given  a givn project with a change that has passed design-review
    And  the change's proposal records review kind "independent"
    And  the change's design-review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And  the change's review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: An independent review whose reviewer is the author is refused
    Given  a givn project with a change that has passed design-review
    And  the change's proposal records review kind "independent"
    And  the change's design-review records review kind "independent" by author "agent" in reviewer context "agent"
    And  the change's review records review kind "independent" by author "agent" in reviewer context "agent"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "Reviewer context"
    And  stderr contains "must differ from the author"

  Scenario: A self-review cannot satisfy a recorded independent request
    Given  a givn project with a change that has passed design-review
    And  the change's proposal records review kind "independent"
    And  the change's design-review records review kind "self-review" by author "agent" in reviewer context "agent"
    And  the change's review records review kind "self-review" by author "agent" in reviewer context "agent"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "independent review is required"
    And  stderr contains "Review kind: self-review"

  Scenario: The recorded independent choice is authoritative from the proposal
    Given  a givn project with a change that has passed design-review
    And  the change's proposal records review kind "independent"
    And  the change's design-review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And  the change's review records review kind "self-review" by author "agent" in reviewer context "agent"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change` twice
    Then  both runs exit non-zero
    And  stderr contains "independent review is required"

  Scenario: Design review and final review share one independent context
    Given  a givn project with a change that has passed design-review
    And  the change's proposal records review kind "independent"
    And  the change's design-review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And  the change's review records review kind "independent" by author "agent" in reviewer context "reviewer-two"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "one review context"
    And  stderr contains "reviewer-one"
    And  stderr contains "reviewer-two"

  Scenario: A legacy review record without a kind keeps its existing gate result
    Given  a givn project with a change that has passed design-review
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: The review instruction carries the one-context policy
    Given  a givn project
    When  the developer reads the review instruction
    Then  the review instruction says the default is `Review kind: self-review`
    And  the review instruction permits exactly one fresh context for a major change
    And  the review instruction says one context covers design review and final review
    And  the review instruction forbids more than one review context
    And  the review instruction says the reviewer must not be the author

  Scenario: Guidance assets stop prescribing subagent orchestration
    When  the guidance assets are scanned for subagent vocabulary
    Then  only assets/instructions/review.md names a review subagent
    And  no other guidance asset prescribes a separate agent, session, architect, or reviewer to execute a step

  @givn-workflow-enforcement
  Scenario: A successful runner without new output cannot reuse a stale result
    Given  a change ready for changed-scenario verification with a stale passing runner result
    And  the runner exits successfully without writing a result
    When  the developer runs `givn check tasks --change verify-demo`
    Then  the stale result was absent when execution began
    And  the command exits non-zero because fresh runner output is missing
    And  no passed verification receipt is issued

  @givn-workflow-enforcement
  Scenario Outline: Invalid fresh runner output cannot prove changed scenarios
    Given  a change ready for changed-scenario verification
    And  the runner exits successfully and writes fresh output that is <invalid output>
    When  the developer runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero and names the invalid runner result
    And  no passed verification receipt is issued
    Examples:
      | invalid output |
      | malformed |
      | reporting failures |
      | reporting skipped scenarios |
      | reporting zero scenarios |
      | reporting counts that do not reconcile |
      | reporting a different scope |

  @givn-workflow-enforcement
  Scenario Outline: Inputs changed during execution cannot receive a passed receipt
    Given  a change ready for changed-scenario verification
    And  the runner produces fresh valid passing output
    And  the <input> changes during execution and remains changed afterward
    When  the developer runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero because the tested inputs changed
    And  no passed verification receipt is issued for the changed inputs
    Examples:
      | input |
      | tested revision |
      | selected delta feature |
      | runner command |
      | relevant source file |

  @givn-workflow-enforcement
  Scenario: An explicit no-behavior-change declaration needs no delta scenarios
    Given  a committed change explicitly declares no behavior change and contains no delta feature files
    And  all applicable identity and integrity checks pass
    And  the project records every verification command invocation
    When  the developer runs `givn check tasks --change maintenance-demo`
    Then  the command succeeds
    And  changed-scenario verification is reported as not applicable
    And  no replacement scenario is required
    And  neither a changed-scenario runner nor a permanent full-suite runner executes

  @givn-workflow-enforcement
  Scenario: Omitting delta specs without declaring no behavior change is not an exemption
    Given  a change has no delta feature files and no explicit no-behavior-change declaration
    When  the developer runs `givn check tasks --change maintenance-demo`
    Then  the command exits non-zero and explains the missing declaration or delta specs
    And  no permanent full-suite runner executes as a substitute

  @givn-workflow-enforcement
  Scenario Outline: Integrity remains applicable without delta specs
    Given  a committed change explicitly declares no behavior change and has no delta feature files
    And  the change has <failure>
    When  the developer runs `givn check tasks --change maintenance-demo`
    Then  the command exits non-zero and names the applicable integrity failure
    And  changed-scenario verification is reported as not applicable
    Examples:
      | failure |
      | a mismatched change identity |
      | a checked task with a placeholder revision |

  @givn-workflow-enforcement
  Scenario: Review accepts an explicit no-behavior-change verification result
    Given  a change explicitly declares no behavior change and has no delta feature files
    And  changed-scenario verification is recorded as not applicable
    And  the completed review and all applicable integrity checks pass
    When  the developer runs `givn check review --change maintenance-demo`
    Then  the command succeeds without requiring a changed-scenario receipt
    And  no verification suite executes

  @givn-workflow-enforcement
  Scenario: Implementation verification does not require the review it will inform
    Given  a change is ready for changed-scenario verification and its review has not been written
    And  no archive candidate or archive receipt exists
    And  the runner writes fresh valid passing results for stable inputs
    When  the developer runs `givn check tasks --change verify-demo`
    Then  the command succeeds and records passed verification evidence
    And  it does not require a completed review or future archive proof

  @givn-workflow-enforcement
  Scenario: Batch traceability rejects an unreachable revision
    Given  a checked scenario records a revision that is not reachable from the current revision
    And  all other applicable task checks pass
    When  the developer runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero and names the scenario and its unreachable revision

  @givn-workflow-enforcement
  Scenario: Final review does not reconstruct TDD order from commit shape
    Given  a completed batch records its actual execution evidence and reachable implementation or regression-test revision
    And  all applicable verification and review checks pass
    And  the batch has no separate RED, GREEN or REFACTOR revisions
    When  the developer runs `givn check review --change verify-demo`
    Then  the command succeeds without a retrospective commit-shape or TDD-order gate

  @givn-workflow-enforcement
  Scenario Outline: Preservation detects changes to supported executable content
    Given  matching before and after behavior snapshots for a supported Gherkin feature
    And  only the <executable content> differs without an explicit retirement
    When  the developer runs the migration coverage check
    Then  preservation comparison reports missing or changed behavior
    Examples:
      | executable content |
      | Background setup |
      | inherited execution tags |
      | step docstring payload |
      | step data table cell |
      | Scenario Outline Examples value |
      | Examples execution tags |

  @givn-workflow-enforcement
  Scenario: Moving unchanged supported executable content preserves behavior
    Given  matching before and after behavior snapshots including supported Backgrounds, tags, docstrings, tables and Examples
    And  only feature paths differ between the snapshots
    When  the developer runs the migration coverage check
    Then  preservation comparison passes without an extra verification suite
    And  the result claims supported executable-content preservation rather than semantic equivalence

  @givn-workflow-enforcement
  Scenario Outline: Review ignores verdict examples rather than treating them as decisions
    Given  a change has current applicable verification evidence and otherwise complete review evidence
    And  its review contains a real verdict "VERDICT: FAIL"
    And  its review contains "VERDICT: PASS" only inside <example context>
    When  the developer runs `givn check review --change verify-demo`
    Then  the command exits non-zero because the actual verdict is FAIL
    Examples:
      | example context |
      | an HTML comment |
      | a fenced code example |

  @givn-workflow-enforcement
  Scenario Outline: Non-executable FAIL examples do not override an actual PASS
    Given  a change has current applicable verification evidence and otherwise complete review evidence
    And  its review contains a real verdict "VERDICT: PASS"
    And  its review contains "VERDICT: FAIL" only inside <example context>
    When  the developer runs `givn check review --change verify-demo`
    Then  the command succeeds without reporting conflicting verdicts
    Examples:
      | example context |
      | an HTML comment |
      | a fenced code example |

  @givn-workflow-enforcement
  Scenario: Conflicting actual verdicts cannot pass review
    Given  a change has otherwise valid review evidence
    And  its review contains actual verdict declarations "VERDICT: PASS" and "VERDICT: FAIL" outside comments and fences
    When  the developer runs `givn check review --change verify-demo`
    Then  the command exits non-zero and reports conflicting verdicts

  @givn-workflow-enforcement
  Scenario: Negated PASS prose is not a passing verdict
    Given  a change has otherwise valid review evidence
    And  its review contains only "This review does not PASS" instead of a verdict declaration
    When  the developer runs `givn check review --change verify-demo`
    Then  the command exits non-zero because a valid actual verdict is missing

  @givn-workflow-enforcement
  Scenario: Independent cheap failures are reported while dependent checks are deferred
    Given  a change has a malformed use-case title, a checked task with a placeholder revision and a missing README impact decision
    When  the developer runs `givn check review --change verify-demo`
    Then  the command exits non-zero and reports all three independent failures together
    And  capability checks requiring the malformed use-case identity are reported as not performed
    And  no dependent success is fabricated and no expensive verification executes

  @e2e @givn-workflow-enforcement
  Scenario: Consumer invokes bare check and receives explicit non-executing guidance
    Given  a givn project whose configured hooks and verification commands record execution
    When  the consumer invokes `givn check`
    Then  terminal output explains that no checks were executed
    And  terminal output lists explicit check commands and labels which execute runners
    And  no configured hook or runner executes
    And  the terminal transcript is captured for this scenario

  @givn-workflow-enforcement
  Scenario: Review rejects add-on configuration drift
    Given  a project root with givn/ initialised
    When  the user runs `givn new addon-lock`
    And  the project configuration sets addon "coverage" to "false"
    And  the user runs `givn check review --change addon-lock`
    Then  the command exits non-zero
    And  stderr contains "add-on configuration changed"
    And  stderr contains "coverage: enabled -> disabled"

  @givn-workflow-enforcement
  Scenario: Review refuses a projected interaction-inventory mismatch before archive
    Given  a givn project with a change that has passed design-review
    And  the change's delta adds an E2E scenario "Consumer cancels an order" without an interaction row
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "interaction inventory mismatch"
    And  stderr contains "use case 'demo-cap'"
    And  stderr contains "capability 'demo-cap'"
    And  stderr contains "unlisted @e2e scenario 'Consumer cancels an order'"

  @givn-workflow-enforcement
  Scenario: Review accepts a delta whose projected interaction inventory is consistent
    Given  a givn project with a change that has passed design-review
    And  the change's delta adds an E2E scenario "Consumer cancels an order" with its interaction row
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  @givn-workflow-enforcement
  Scenario Outline: An ise-effective change whose declared e2e role resolves no readable package fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose e2e role "<state>"
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the declared e2e package and "<condition>"
    And  no change verification receipt is written
    Examples:
      | state | condition |
      | matches no project path | matches no path |
      | resolves a package with a malformed manifest | cannot be read |
      | lists no path | the 'e2e' role lists no path |

  @givn-workflow-enforcement
  Scenario: An ise-effective change with no behavior change still needs a readable e2e declaration
    Given  an ise-effective change with no behavior change and no readable e2e declaration
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the absent e2e declaration
    When  the developer runs `givn check review --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the absent e2e declaration

  @givn-workflow-enforcement
  Scenario Outline: An ise-effective change whose e2e package reaches a source package fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose e2e package reaches a source package through a "<edge>" dependency
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the e2e package and the dependency edge into "source"
    And  no change verification receipt is written
    Examples:
      | edge |
      | direct |
      | transitive |
      | workspace-inherited |
      | target-specific |

  @givn-workflow-enforcement
  Scenario: An ise-effective change whose e2e path sits under a source root fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose e2e path sits under a source root
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the e2e path and the source root
    And  no change verification receipt is written

  @givn-workflow-enforcement
  Scenario: An ise-effective change whose e2e package contains a source root fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose e2e package contains a source root
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the e2e package and the source root
    And  no change verification receipt is written

  @givn-workflow-enforcement
  Scenario: An ise-effective change whose e2e path resolves no package and imports a source package fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose e2e path resolves no package and imports a package under a source root
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the e2e path and the imported source package
    And  no change verification receipt is written

  @givn-workflow-enforcement
  Scenario Outline: An ise-effective change whose E2E runner command names no declared e2e root fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose E2E runner command is "<command>"
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the declared e2e root and the E2E runner command
    And  no change verification receipt is written
    Examples:
      | command |
      | ./run-tests.sh --bin other-target |
      | (empty) |
      | givn missing-e2e-runner |
      | ./run-tests.sh --e2e |

  @givn-workflow-enforcement
  Scenario: An ise-effective change whose E2E run leaves a step unbound fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose delta has one E2E scenario with an unbound step
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the failing E2E run and reports 1 failed test
    And  no change verification receipt is written

  @givn-workflow-enforcement
  Scenario: An ise-effective change whose E2E run reports a skipped step fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose delta has one E2E scenario with a skipped step
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the failing E2E run and reports 1 skipped test
    And  no change verification receipt is written

  @givn-workflow-enforcement
  Scenario: An ise-effective change receipt binds the declared e2e package, the E2E runner command, and the Givn version
    Given  an ise-effective change ready for change-scoped verification with a passing E2E run over two declared e2e packages
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits 0
    And  the change verification receipt records a digest of every resolved e2e package
    And  the change verification receipt records the E2E runner command digest
    And  the change verification receipt records the running Givn version
    And  the change verification receipt records a passed E2E scope

  @givn-workflow-enforcement
  Scenario: An ise-effective change whose delta has no E2E scenario records the E2E scope as not applicable
    Given  an ise-effective change ready for change-scoped verification with no E2E scenario in its delta
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits 0
    And  the change verification receipt records a digest of every resolved e2e package
    And  the change verification receipt records the E2E scope as not applicable

  @givn-workflow-enforcement
  Scenario Outline: An ise-effective change receipt whose E2E identity no longer matches the project is stale for review
    Given  an ise-effective change with a current change verification receipt
    And  the receipt's E2E identity no longer matches the project because <input>
    When  the developer runs `givn check review --change ise-boundary`
    Then  the command exits non-zero
    And  stderr says the change verification receipt is stale
    And  stderr reports the stale reason "<reason>"
    Examples:
      | input | reason |
      | the e2e package changed | e2e package digest changed |
      | the E2E runner command changed | E2E runner command changed |
      | the Givn version changed | Givn version changed |
      | the receipt lacks the identity | receipt lacks the E2E identity |
      | the declaration cannot be read | the E2E identity cannot be read |

  @givn-workflow-enforcement
  Scenario Outline: An ise-effective change receipt whose E2E scope no longer matches the delta is stale for review
    Given  an ise-effective change with a current change verification receipt whose delta "<delta>"
    And  the receipt's E2E scope records "<scope>"
    When  the developer runs `givn check review --change ise-boundary`
    Then  the command exits non-zero
    And  stderr says the change verification receipt is stale
    And  stderr reports the stale reason "<reason>"
    Examples:
      | delta | scope | reason |
      | has an E2E scenario | not_applicable | the E2E scope is missing |
      | has no E2E scenario | passed | the E2E scope is not applicable to this delta |

  @givn-workflow-enforcement
  Scenario: A change without ise keeps today's receipt without an E2E identity
    Given  a change ready for change-scoped verification with ise off
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits 0
    And  the change verification receipt contains no E2E identity and no E2E scope

  @givn-workflow-enforcement
  Scenario: A change whose declared E2E package is a standalone workspace passes the tasks check and binds the E2E scope
    Given  an ise-effective change ready for change-scoped verification whose declared e2e package declares its own workspace, whose E2E runner command is "./run-tests.sh --e2e --package e2e", and whose delta has an E2E scenario
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits 0
    And  the change verification receipt records a digest of every resolved e2e package
    And  the change verification receipt records the E2E runner command digest
    And  the change verification receipt records a passed E2E scope

  @givn-workflow-enforcement
  Scenario: A whole-system Ise ship executes the retained component corpus
    Given  an ise-effective ship change declaring two retained component scenarios and a runner that reports every declared scenario
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits 0
    And  stdout contains "component_executed=2/2"
    And  the change verification receipt records the retained component corpus at "givn/specs" with two scenarios

  @givn-workflow-enforcement
  Scenario: An Ise ship with a delta also executes the retained component corpus
    Given  an ise-effective ship change that keeps one delta scenario and two retained component scenarios
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits 0
    And  the change verification receipt records a delta result of one scenario
    And  the change verification receipt records a component result of two scenarios

  @givn-workflow-enforcement
  Scenario: A whole-system Ise ship without a runner result is refused
    Given  an ise-effective ship change declaring two retained component scenarios and a runner that exits 0 without a result file
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names "component_executed=0/2"

  @givn-workflow-enforcement
  Scenario: A whole-system Ise ship that executes fewer component scenarios is refused
    Given  an ise-effective ship change declaring two retained component scenarios and a runner that reports one executed scenario
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names "component_executed=1/2"

  @givn-workflow-enforcement
  Scenario: Review refuses an Ise ship without a component receipt
    Given  an ise-effective ship change declaring two retained component scenarios and a runner that reports every declared scenario
    When  the developer runs `givn check review --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names "change verification receipt missing"
