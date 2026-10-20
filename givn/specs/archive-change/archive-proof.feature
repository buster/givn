Feature: Archive Proof

  Scenario: Archive requires a committed project state
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the project has an uncommitted change to "src/lib.rs"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names "src/lib.rs"
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "proof-demo"

  Scenario: An untracked file blocks the archive
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the project has an untracked file "notes.txt"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names "notes.txt"
    And  the change directory still exists under givn/changes/

  Scenario Outline: Archive publishes the merged candidate before running the full suite
    Given  a <backend> project with a completed change "proof-demo" ready to archive
    And  the full suite fails against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  the change directory has been moved to givn/archive/
    And  the permanent specs contain the merged delta scenarios
    And  the archive candidate is recorded as unproven
    And  the candidate revision is published
    And  the candidate revision contains exactly the merged corpus paths
    Examples:
      | backend |
      | Git |
      | Fossil |

  Scenario: Each verification scope runs once against the merged permanent specs
    Given  a givn project with a completed change "proof-demo" whose delta adds an @e2e scenario
    And  the runner records every invocation with its scope and features path
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the runner was invoked once for the regular scope with features path "givn/specs"
    And  the runner was invoked once for the e2e scope with features path "givn/specs"

  Scenario Outline: A scope result that does not prove the suite blocks proof
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the full suite passes but its runner result is <result>
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr mentions the runner result
    And  the archive candidate is recorded as unproven
    And  the change directory has been moved to givn/archive/
    Examples:
      | result |
      | missing |
      | malformed |
      | declaring failed scenarios |
      | declaring skipped scenarios |
      | whose counts do not reconcile |
      | declaring zero scenarios |
      | declaring the wrong scope |

  Scenario: The end-to-end scope is not applicable when no end-to-end scenario is merged
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the verification receipt records the end-to-end scope as not applicable

  Scenario Outline: Repairing a failed candidate forward proves it without repeating the merge
    Given  an unproven archive candidate "proof-demo" in a <backend> project whose full suite failed
    And  the developer commits a repair
    And  the full suite now passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  each merged scenario appears exactly once in the permanent specs
    And  the verification receipt names the repaired revision
    Examples:
      | backend |
      | Git |
      | Fossil |

  Scenario: An already proven archive refuses another archive run
    Given  a proven archive "proof-demo"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the archive is already proven

  Scenario: A legacy archive without a receipt refuses another archive run
    Given  a legacy archive "proof-demo" without a verification receipt
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the archive is already archived

  Scenario: A change present as both active and archived is refused as corrupt
    Given  a change "proof-demo" that exists both under givn/changes/ and givn/archive/
    And  the pending archive candidate conflicts with the active change
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the pending archive candidate conflicts with the active change
    And  the pending receipt is unchanged
    And  the change directory still exists under givn/changes/

  Scenario: An unproven archive candidate is surfaced by project status
    Given  an unproven archive candidate "proof-demo" whose full suite failed
    When  the user runs `givn status`
    Then  stdout lists "proof-demo" as an unproven archive
    And  stdout routes the developer to `givn archive --change proof-demo`

  Scenario: Instructions route an unproven archive candidate to its resume command
    Given  an unproven archive candidate "proof-demo" whose full suite failed
    When  the user runs `givn instructions`
    Then  stdout routes the developer to `givn archive --change proof-demo`

  Scenario: Finalization refuses a tree change outside the declared evidence
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the full suite passes against the merged permanent specs
    And  the coverage command writes an undeclared source file
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the tested tree changed
    And  the archive candidate is recorded as unproven

  @e2e
  Scenario Outline: Consumer archives a completed change and receives a proven archive
    Given  a committed <backend> project with a completed change "proof-real" ready to archive
    And  the project reports scenario counts for every verification scope
    When  the user invokes `givn archive --change proof-real`
    Then  the command succeeds
    And  the archive at givn/archive/proof-real/ carries a proven verification receipt
    Examples:
      | backend |
      | Git |
      | Fossil |

  @e2e
  Scenario: Consumer repairs a blocked archive and the rerun proves it
    Given  a committed givn project with an unproven archive candidate "proof-repair"
    And  the developer commits a repair
    And  the project reports scenario counts for every verification scope
    When  the user invokes `givn archive --change proof-repair`
    Then  the command succeeds
    And  the archive at givn/archive/proof-repair/ carries a proven verification receipt

  @e2e
  Scenario: Consumer inspects an unproven archive candidate
    Given  a committed givn project with an unproven archive candidate "proof-pending"
    When  the user invokes `givn status`
    Then  stdout lists "proof-pending" as an unproven archive
    And  stdout routes the developer to resume it with `givn archive --change proof-pending`
  Scenario Outline: A proven archive records a verification receipt binding the tested revision
    Given  a <backend> project with a completed change "proof-demo" ready to archive
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the verification receipt names the tested revision
    And  the verification receipt records the regular scope scenario counts
    And  the history has a finalization revision after the tested revision
    And  the finalization revision contains only the resolved evidence paths
    Examples:
      | backend |
      | Git |
      | Fossil |

  Scenario Outline: A rejected candidate revision leaves the archive unproven and repairable
    Given  a <backend> project with a completed change "proof-demo" ready to archive
    And  a commit hook rejects the archive candidate revision
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the candidate revision rejection
    And  the archive candidate is recorded as unproven
    When  the commit hook accepts the archive candidate revision
    And  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the tested revision contains the merged permanent specs
    Examples:
      | backend |
      | Git |
      | Fossil |

  Scenario: A rejected candidate revision on Git leaves the merge staged
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  a commit hook rejects the archive candidate revision
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  the merged state is staged but not committed
    And  the archive candidate is recorded as unproven

  Scenario: A rejected candidate revision on Fossil leaves the merged state uncommitted
    Given  a Fossil project with a completed change "proof-demo" ready to archive
    And  a commit hook rejects the archive candidate revision
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  the merged state is present in the checkout but not committed
    And  the archive candidate is recorded as unproven

  Scenario: A Fossil EXTRA file blocks the archive
    Given  a Fossil project with a completed change "proof-demo" ready to archive
    And  the project has an EXTRA file "notes.txt"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names "notes.txt"
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  Scenario: A Fossil edited file blocks the archive
    Given  a Fossil project with a completed change "proof-demo" ready to archive
    And  the project has an edited file "src/lib.rs"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names "src/lib.rs"
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  Scenario: Archive requires a version-controlled project
    Given  a project root without a Git worktree or a Fossil checkout
    And  a completed change "proof-demo" with all required artifacts
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the project root and the missing version-controlled project
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  Scenario: Archive requires the project root to be the checkout root
    Given  a givn project nested inside a larger Git repository
    And  a completed change "proof-demo" ready to archive
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the project root and the worktree top level
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  Scenario: A Fossil checkout without the fossil executable is refused by name
    Given  a Fossil checkout whose project root is the checkout root
    And  a completed change "proof-demo" with all required artifacts
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change proof-demo` without the `fossil` executable on PATH
    Then  the command exits non-zero
    And  stderr names the missing `fossil` executable
    And  the change directory still exists under givn/changes/
    And  no file in the project changed
  Scenario: A change present as both active and archived without a receipt is refused as corrupt
    Given  a change "proof-demo" that exists both under givn/changes/ and givn/archive/
    And  the archive candidate has no verification receipt
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the project state is corrupt and both directories exist
    And  the change directory still exists under givn/changes/

  Scenario: An interrupted archive move resumes from an identical pending candidate
    Given  an interrupted archive move for change "proof-demo" with an identical pending candidate
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the change directory no longer exists under givn/changes/
    And  each merged scenario appears exactly once in the permanent specs
    And  the verification receipt names the tested revision

  Scenario: An interrupted merge resumes from its pending candidate
    Given  a pending archive candidate "proof-demo" whose delta merge did not complete
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the permanent specs contain the merged delta scenarios
    And  each merged scenario appears exactly once in the permanent specs

  Scenario: An in-flight delta file is retried without duplicating its scenarios
    Given  a pending archive candidate "proof-demo" whose delta file was applied but not recorded
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  each merged scenario appears exactly once in the permanent specs

  Scenario: A failed archive move keeps the change retryable
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the archive location is blocked
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the archive location
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "proof-demo"
    When  the archive location is unblocked
    And  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven

  Scenario Outline: Change evidence does not block the archive
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the change has an uncommitted evidence file "<evidence>"
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the archived change contains the evidence file "<evidence>"
    Examples:
      | evidence |
      | evidence/final.log |
      | verification/notes.txt |

  Scenario: A rejected finalization publication leaves the archive unproven
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  a commit hook rejects the finalization revision
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the finalization rejection
    And  the tested history records the archive as unproven
    When  the commit hook accepts the finalization revision
    And  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven

  Scenario: An interrupted archive move is surfaced by project status
    Given  an interrupted archive move for change "proof-demo" with an identical pending candidate
    When  the user runs `givn status`
    Then  stdout lists "proof-demo" as an unproven archive
    And  stdout routes the developer to resume it with `givn archive --change proof-demo`
  Scenario: Archive refuses a stale change verification receipt before merging
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change has a current verification receipt
    And  the delta specs changed after the receipt
    When  the user runs `givn archive --change verify-demo` without committing the fixture
    Then  the command exits non-zero
    And  stderr says the verification receipt is stale
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "verify-demo"

  Scenario: Archive reuses the change receipt and runs only the merged suite
    Given  a clean committed givn project with a change whose delta specs pass
    And  the runner records every invocation with its scope and features path
    When  the user runs `givn archive --change verify-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the runner was invoked once for the regular scope with features path "givn/specs"
    And  the runner was never invoked with a change features path
  Scenario: CLI interaction transcripts are declared evidence
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change has an uncommitted evidence file "evidence/cli/session.txt"
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change verify-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the archived change contains the evidence file "evidence/cli/session.txt"
