@givn.delta @archive-proof
Feature: Archive Proof

  @givn.added @wip
  Scenario: Archive requires a committed project state
    Given a givn project with a completed change "proof-demo" ready to archive
    And the project has an uncommitted change to "src/lib.rs"
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And stderr names "src/lib.rs"
    And the change directory still exists under givn/changes/
    And no archive candidate exists for "proof-demo"

  @givn.added @wip
  Scenario: An untracked file blocks the archive
    Given a givn project with a completed change "proof-demo" ready to archive
    And the project has an untracked file "notes.txt"
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And stderr names "notes.txt"
    And the change directory still exists under givn/changes/

  @givn.added @wip
  Scenario: Archive requires Git version control
    Given a givn project without Git version control
    And a completed change "proof-demo" with all required artifacts
    And the project declares a complete run declaration
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And stderr mentions Git
    And the change directory still exists under givn/changes/

  @givn.added @wip
  Scenario: Archive publishes the merged candidate before running the full suite
    Given a givn project with a completed change "proof-demo" ready to archive
    And the full suite fails against the merged permanent specs
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And the change directory has been moved to givn/archive/
    And the permanent specs contain the merged delta scenarios
    And the archive candidate is recorded as unproven
    And the candidate state is committed

  @givn.added @wip
  Scenario: Each verification scope runs once against the merged permanent specs
    Given a givn project with a completed change "proof-demo" whose delta adds an @e2e scenario
    And the runner records every invocation with its scope and features path
    When the user runs `givn archive --change proof-demo`
    Then the command exits 0
    And the runner was invoked once for the regular scope with features path "givn/specs"
    And the runner was invoked once for the e2e scope with features path "givn/specs"

  @givn.added @wip
  Scenario Outline: A scope result that does not prove the suite blocks proof
    Given a givn project with a completed change "proof-demo" ready to archive
    And the full suite passes but its runner result is <result>
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And stderr mentions the runner result
    And the archive candidate is recorded as unproven
    And the change directory has been moved to givn/archive/

    Examples:
      | result                        |
      | missing                       |
      | declaring failed scenarios    |
      | declaring skipped scenarios   |
      | whose counts do not reconcile |
      | declaring zero scenarios      |

  @givn.added @wip
  Scenario: A proven archive records a verification receipt binding the tested commit
    Given a givn project with a completed change "proof-demo" ready to archive
    And the full suite passes against the merged permanent specs
    When the user runs `givn archive --change proof-demo`
    Then the command exits 0
    And the archive candidate is proven
    And the verification receipt names the tested commit
    And the verification receipt records the regular scope scenario counts
    And the git history has a finalization commit after the tested commit

  @givn.added @wip
  Scenario: The end-to-end scope is not applicable when no end-to-end scenario is merged
    Given a givn project with a completed change "proof-demo" ready to archive
    And the full suite passes against the merged permanent specs
    When the user runs `givn archive --change proof-demo`
    Then the command exits 0
    And the verification receipt records the end-to-end scope as not applicable

  @givn.added @wip
  Scenario: Repairing a failed candidate forward proves it without repeating the merge
    Given an unproven archive candidate "proof-demo" whose full suite failed
    And the developer commits a repair
    And the full suite now passes against the merged permanent specs
    When the user runs `givn archive --change proof-demo`
    Then the command exits 0
    And the archive candidate is proven
    And each merged scenario appears exactly once in the permanent specs
    And the verification receipt names the repaired commit

  @givn.added @wip
  Scenario: An already proven archive refuses another archive run
    Given a proven archive "proof-demo"
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And stderr says the archive is already proven

  @givn.added @wip
  Scenario: An unproven archive candidate is surfaced by project status
    Given an unproven archive candidate "proof-demo" whose full suite failed
    When the user runs `givn status`
    Then stdout lists "proof-demo" as an unproven archive
    And stdout routes the developer to `givn archive --change proof-demo`

  @givn.added @wip
  Scenario: Instructions route an unproven archive candidate to its resume command
    Given an unproven archive candidate "proof-demo" whose full suite failed
    When the user runs `givn instructions`
    Then stdout routes the developer to `givn archive --change proof-demo`

  @givn.added @wip
  Scenario: Finalization refuses a tree change outside the declared evidence
    Given a givn project with a completed change "proof-demo" ready to archive
    And the full suite passes against the merged permanent specs
    And the coverage command writes an undeclared source file
    When the user runs `givn archive --change proof-demo`
    Then the command exits non-zero
    And stderr says the tested tree changed
    And the archive candidate is recorded as unproven

  @givn.added @e2e @wip
  Scenario: Consumer archives a completed change and receives a proven archive
    Given a committed givn project with a completed change "proof-real" ready to archive
    And the project reports scenario counts for every verification scope
    When the user invokes `givn archive --change proof-real`
    Then the command succeeds
    And the archive at givn/archive/proof-real/ carries a proven verification receipt

  @givn.added @e2e @wip
  Scenario: Consumer repairs a blocked archive and the rerun proves it
    Given a committed givn project with an unproven archive candidate "proof-repair"
    And the developer commits a repair
    And the project reports scenario counts for every verification scope
    When the user invokes `givn archive --change proof-repair`
    Then the command succeeds
    And the archive at givn/archive/proof-repair/ carries a proven verification receipt

  @givn.added @e2e @wip
  Scenario: Consumer inspects an unproven archive candidate
    Given a committed givn project with an unproven archive candidate "proof-pending"
    When the user invokes `givn status`
    Then stdout lists "proof-pending" as an unproven archive
    And stdout routes the developer to resume it with `givn archive --change proof-pending`
