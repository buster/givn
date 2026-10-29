@givn.delta @archive-proof
Feature: Archive proof

  @givn.modified @wip
  Scenario: A rejected finalization publication leaves the archive unproven
    Given an archive candidate has passed every applicable verification scope and coverage measurement
    And a commit hook rejects the evidence-only finalization publication
    When the developer runs `givn archive --change proof-demo`
    Then the command exits non-zero and names the finalization rejection
    And the candidate remains unproven and repairable in place
    When the hook accepts publication and the developer retries with unchanged relevant inputs
    Then the archive becomes proven using the recorded scope results and coverage
    And no verification suite or coverage measurement runs again

  @givn.modified @wip
  Scenario Outline: Repairing a failed candidate forward proves it without repeating the merge
    Given an unproven archive candidate "proof-demo" in a <backend> project
    And its regular scope passed but its end-to-end scope failed
    And the developer commits a repair changing only inputs relevant to the end-to-end scope
    When the developer runs `givn archive --change proof-demo`
    Then the archive becomes proven after fresh passing end-to-end verification
    And the unchanged regular scope result is reused without another execution
    And each merged scenario appears exactly once in the permanent specs
    And the receipt names the repaired revision and the reused scope evidence
    Examples:
      | backend |
      | Git |
      | Fossil |

  @givn.added @wip
  Scenario: Evidence-only recovery reuses successful scopes and runs only the failed scope
    Given an unproven candidate has a passed regular scope and a failed end-to-end scope
    And only archive evidence has changed since those executions
    And the end-to-end runner now produces a fresh valid passing result
    When the developer retries `givn archive --change proof-demo`
    Then the archive becomes proven
    And the regular scope is not executed again
    And only the failed end-to-end scope executes

  @givn.added @wip
  Scenario: A changed relevant input invalidates each affected scope before reuse
    Given an unproven candidate has passed regular and end-to-end scope results
    And a committed source repair changes relevant inputs shared by both scopes
    When the developer retries `givn archive --change proof-demo`
    Then neither stale scope result is reused as proof
    And each affected scope executes once against the repaired candidate
    And proof requires fresh valid passing results for the repaired inputs

  @givn.added @wip
  Scenario: Retry uses cheap input checks rather than blanket review revalidation
    Given an unproven candidate has completed merging and passed verification
    And finalization publication failed
    And its relevant inputs and accepted review evidence are unchanged
    When the developer retries `givn archive --change proof-demo`
    Then revision and relevant-content checks confirm reusable evidence
    And the completed merge and review are not repeated
    And no full verification run is required solely because this is a retry

  @givn.added @wip
  Scenario: Archive-directory evidence exemptions remain broad during proof reuse
    Given an unproven candidate has valid passed verification results
    And only files within the exempt archive directory have changed
    When the developer retries `givn archive --change proof-demo`
    Then those changes do not invalidate the recorded scope results
    And proof reuse does not require an evidence-output allowlist
    And the command reports that executable changes inside exempt paths may escape cheap input checks

  @givn.added @wip
  Scenario: A stale passing scope result cannot prove a new archive execution
    Given a merged archive candidate has an old passing regular result file
    And regular verification is required and its runner exits successfully without writing output
    When the developer runs `givn archive --change proof-demo`
    Then the old result was removed before execution
    And the candidate remains unproven because fresh output is missing

  @givn.added @wip
  Scenario: Scope inputs are captured before archive verification and checked afterward
    Given a merged archive candidate is ready for regular verification
    And the runner writes fresh valid passing output but changes a relevant source file during execution
    When the developer runs `givn archive --change proof-demo`
    Then the candidate remains unproven because relevant inputs changed during execution
    And no passing scope evidence is rebound to the modified inputs

  @givn.added @wip
  Scenario: No-behavior-change archive retains merged-candidate verification
    Given a completed change declares "Behavior change: none", explicitly skips specs and has no delta feature files
    And changed-scenario verification is recorded as not applicable
    And applicable identity, integrity and review checks pass
    When the developer runs `givn archive --change maintenance-demo`
    Then archive does not require invented delta scenarios or a changed-scenario receipt
    And applicable merged-candidate verification still runs before the archive is proven

  @givn.modified @e2e @wip
  Scenario: Consumer repairs a blocked archive and the rerun proves it
    Given a committed givn project with an unproven candidate whose scopes passed but finalization publication failed
    And the publication blocker is repaired without changing relevant verification inputs
    When the user invokes `givn archive --change proof-repair`
    Then the command succeeds and terminal output reports a proven archive
    And the archive carries a proven receipt reusing its passed scope evidence
    And no verification suite runs again
    And the terminal transcript is captured for this scenario


  @givn.added @wip
  Scenario Outline: An inconsistent no-behavior-change archive exemption is refused before mutation
    Given a completed change declares "Behavior change: none"
    And the change has <delta state> and <specs disposition>
    And changed-scenario verification is recorded as not applicable
    When the developer runs `givn archive --change maintenance-demo`
    Then the command exits non-zero and names <reason>
    And no active change, archive receipt or permanent specification is mutated
    And no merged-candidate verification runner executes

    Examples:
      | delta state              | specs disposition               | reason                                             |
      | executable delta features | an explicitly skipped specs artifact | executable deltas contradicting no behavior change |
      | no delta feature files    | no explicit specs skip               | the missing explicit specs skip                    |
