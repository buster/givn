@givn.delta @givn-workflow-enforcement
Feature: givn Workflow Enforcement

  @givn.modified @e2e
  Scenario: Consumer runs check tasks and receives a change verification receipt
    Given a clean committed givn project with a change whose delta specs pass
    And the project declares a complete run declaration
    When the user invokes `givn check tasks --change verify-demo`
    Then the command succeeds
    And stdout reports passed change-scoped verification
    And the change contains a passed receipt bound to the revision, selected features, runner command and relevant inputs captured before execution
    And the terminal transcript is captured for this scenario

  @givn.added
  Scenario: A successful runner without new output cannot reuse a stale result
    Given a change ready for changed-scenario verification with a stale passing runner result
    And the runner exits successfully without writing a result
    When the developer runs `givn check tasks --change verify-demo`
    Then the stale result was absent when execution began
    And the command exits non-zero because fresh runner output is missing
    And no passed verification receipt is issued

  @givn.added
  Scenario Outline: Invalid fresh runner output cannot prove changed scenarios
    Given a change ready for changed-scenario verification
    And the runner exits successfully and writes fresh output that is <invalid output>
    When the developer runs `givn check tasks --change verify-demo`
    Then the command exits non-zero and names the invalid runner result
    And no passed verification receipt is issued
    Examples:
      | invalid output |
      | malformed |
      | reporting failures |
      | reporting skipped scenarios |
      | reporting zero scenarios |
      | reporting counts that do not reconcile |
      | reporting a different scope |

  @givn.added
  Scenario Outline: Inputs changed during execution cannot receive a passed receipt
    Given a change ready for changed-scenario verification
    And the runner produces fresh valid passing output
    And the <input> changes during execution and remains changed afterward
    When the developer runs `givn check tasks --change verify-demo`
    Then the command exits non-zero because the tested inputs changed
    And no passed verification receipt is issued for the changed inputs
    Examples:
      | input |
      | tested revision |
      | selected delta feature |
      | runner command |
      | relevant source file |

  @givn.added
  Scenario: An explicit no-behavior-change declaration needs no delta scenarios
    Given a committed change explicitly declares no behavior change and contains no delta feature files
    And all applicable identity and integrity checks pass
    And the project records every verification command invocation
    When the developer runs `givn check tasks --change maintenance-demo`
    Then the command succeeds
    And changed-scenario verification is reported as not applicable
    And no replacement scenario is required
    And neither a changed-scenario runner nor a permanent full-suite runner executes

  @givn.added
  Scenario: Omitting delta specs without declaring no behavior change is not an exemption
    Given a change has no delta feature files and no explicit no-behavior-change declaration
    When the developer runs `givn check tasks --change maintenance-demo`
    Then the command exits non-zero and explains the missing declaration or delta specs
    And no permanent full-suite runner executes as a substitute

  @givn.added
  Scenario Outline: Integrity remains applicable without delta specs
    Given a committed change explicitly declares no behavior change and has no delta feature files
    And the change has <failure>
    When the developer runs `givn check tasks --change maintenance-demo`
    Then the command exits non-zero and names the applicable integrity failure
    And changed-scenario verification is reported as not applicable
    Examples:
      | failure |
      | a mismatched change identity |
      | a checked task with a placeholder revision |

  @givn.added
  Scenario: Review accepts an explicit no-behavior-change verification result
    Given a change explicitly declares no behavior change and has no delta feature files
    And changed-scenario verification is recorded as not applicable
    And the completed review and all applicable integrity checks pass
    When the developer runs `givn check review --change maintenance-demo`
    Then the command succeeds without requiring a changed-scenario receipt
    And no verification suite executes

  @givn.modified
  Scenario: Review does not run the full suite
    Given a change has current applicable verification evidence
    And the completed review and applicable integrity checks pass
    And the project records every verification command invocation
    When the developer runs `givn check review --change test-change`
    Then the command succeeds
    And neither change-scoped nor full-suite verification executes
    And no future archive receipt is required

  @givn.added
  Scenario: Implementation verification does not require the review it will inform
    Given a change is ready for changed-scenario verification and its review has not been written
    And no archive candidate or archive receipt exists
    And the runner writes fresh valid passing results for stable inputs
    When the developer runs `givn check tasks --change verify-demo`
    Then the command succeeds and records passed verification evidence
    And it does not require a completed review or future archive proof

  @givn.modified
  Scenario: Two scenarios sharing a batch commit pass the task check
    Given a change checks two scenarios in one named behavior batch
    And both checked scenarios record the same reachable revision
    And human review confirms that revision contains their implementation or regression tests
    And the revision message names the behavior batch
    And applicable change verification passes
    When the developer runs `givn check tasks --change verify-demo`
    Then the command succeeds and reports that scenario revisions passed
    And no separate revision or production edit per scenario is required
    And no automated semantic diff classifier is required

  @givn.removed
  Scenario: A shared batch revision without a batch label fails the task check
    Given this scenario is removed

  @givn.removed
  Scenario: A commit message that omits the scenario title fails the task check
    Given this scenario is removed

  @givn.added
  Scenario: Batch traceability rejects an unreachable revision
    Given a checked scenario records a revision that is not reachable from the current revision
    And all other applicable task checks pass
    When the developer runs `givn check tasks --change verify-demo`
    Then the command exits non-zero and names the scenario and its unreachable revision

  @givn.added
  Scenario: Final review does not reconstruct TDD order from commit shape
    Given a completed batch records its actual execution evidence and reachable implementation or regression-test revision
    And all applicable verification and review checks pass
    And the batch has no separate RED, GREEN or REFACTOR revisions
    When the developer runs `givn check review --change verify-demo`
    Then the command succeeds without a retrospective commit-shape or TDD-order gate

  @givn.modified
  Scenario: Archive re-runs the static review gates
    Given a givn project with a completed change configured for archive
    And the change's review.md has no README impact decision
    When the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then the command exits non-zero and names the missing README impact decision
    And the active change and permanent specs are unchanged
    And no verification suite executes

  @givn.added
  Scenario Outline: Preservation detects changes to supported executable content
    Given matching before and after behavior snapshots for a supported Gherkin feature
    And only the <executable content> differs without an explicit retirement
    When the developer runs the migration coverage check
    Then preservation comparison reports missing or changed behavior
    Examples:
      | executable content |
      | Background setup |
      | inherited execution tags |
      | step docstring payload |
      | step data table cell |
      | Scenario Outline Examples value |
      | Examples execution tags |

  @givn.added
  Scenario: Moving unchanged supported executable content preserves behavior
    Given matching before and after behavior snapshots including supported Backgrounds, tags, docstrings, tables and Examples
    And only feature paths differ between the snapshots
    When the developer runs the migration coverage check
    Then preservation comparison passes without an extra verification suite
    And the result claims supported executable-content preservation rather than semantic equivalence

  @givn.added
  Scenario Outline: Review ignores verdict examples rather than treating them as decisions
    Given a change has current applicable verification evidence and otherwise complete review evidence
    And its review contains a real verdict "VERDICT: FAIL"
    And its review contains "VERDICT: PASS" only inside <example context>
    When the developer runs `givn check review --change verify-demo`
    Then the command exits non-zero because the actual verdict is FAIL
    Examples:
      | example context |
      | an HTML comment |
      | a fenced code example |

  @givn.added
  Scenario Outline: Non-executable FAIL examples do not override an actual PASS
    Given a change has current applicable verification evidence and otherwise complete review evidence
    And its review contains a real verdict "VERDICT: PASS"
    And its review contains "VERDICT: FAIL" only inside <example context>
    When the developer runs `givn check review --change verify-demo`
    Then the command succeeds without reporting conflicting verdicts
    Examples:
      | example context |
      | an HTML comment |
      | a fenced code example |

  @givn.added
  Scenario: Conflicting actual verdicts cannot pass review
    Given a change has otherwise valid review evidence
    And its review contains actual verdict declarations "VERDICT: PASS" and "VERDICT: FAIL" outside comments and fences
    When the developer runs `givn check review --change verify-demo`
    Then the command exits non-zero and reports conflicting verdicts

  @givn.added
  Scenario: Negated PASS prose is not a passing verdict
    Given a change has otherwise valid review evidence
    And its review contains only "This review does not PASS" instead of a verdict declaration
    When the developer runs `givn check review --change verify-demo`
    Then the command exits non-zero because a valid actual verdict is missing

  @givn.added
  Scenario: Independent cheap failures are reported while dependent checks are deferred
    Given a change has a malformed use-case title, a checked task with a placeholder revision and a missing README impact decision
    When the developer runs `givn check review --change verify-demo`
    Then the command exits non-zero and reports all three independent failures together
    And capability checks requiring the malformed use-case identity are reported as not performed
    And no dependent success is fabricated and no expensive verification executes

  @givn.added @e2e
  Scenario: Consumer invokes bare check and receives explicit non-executing guidance
    Given a givn project whose configured hooks and verification commands record execution
    When the consumer invokes `givn check`
    Then terminal output explains that no checks were executed
    And terminal output lists explicit check commands and labels which execute runners
    And no configured hook or runner executes
    And the terminal transcript is captured for this scenario
