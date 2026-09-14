@givn.delta @archive-proof
Feature: Archive Proof

  Archiving a completed change proves the published state on the backend that
  owns the project, or refuses by name before anything changes.

  @givn.modified
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
      | Git     |
      | Fossil  |

  @givn.modified
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
      | Git     |
      | Fossil  |

  @givn.modified @e2e
  Scenario Outline: Consumer archives a completed change and receives a proven archive
    Given  a committed <backend> project with a completed change "proof-real" ready to archive
    And  the project reports scenario counts for every verification scope
    When  the user invokes `givn archive --change proof-real`
    Then  the command succeeds
    And  the archive at givn/archive/proof-real/ carries a proven verification receipt

    Examples:
      | backend |
      | Git     |
      | Fossil  |

  @givn.added
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
      | Git     |
      | Fossil  |

  @givn.added
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
      | Git     |
      | Fossil  |

  @givn.added
  Scenario: A rejected candidate revision on Git leaves the merge staged
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  a commit hook rejects the archive candidate revision
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  the merged state is staged but not committed
    And  the archive candidate is recorded as unproven

  @givn.added
  Scenario: A rejected candidate revision on Fossil leaves the merged state uncommitted
    Given  a Fossil project with a completed change "proof-demo" ready to archive
    And  a commit hook rejects the archive candidate revision
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  the merged state is present in the checkout but not committed
    And  the archive candidate is recorded as unproven

  @givn.added
  Scenario: A Fossil EXTRA file blocks the archive
    Given  a Fossil project with a completed change "proof-demo" ready to archive
    And  the project has an EXTRA file "notes.txt"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names "notes.txt"
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  @givn.added
  Scenario: A Fossil edited file blocks the archive
    Given  a Fossil project with a completed change "proof-demo" ready to archive
    And  the project has an edited file "src/lib.rs"
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names "src/lib.rs"
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  @givn.added
  Scenario: Archive requires a version-controlled project
    Given  a project root without a Git worktree or a Fossil checkout
    And  a completed change "proof-demo" with all required artifacts
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the project root and the missing version-controlled project
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  @givn.added
  Scenario: Archive requires the project root to be the checkout root
    Given  a givn project nested inside a larger Git repository
    And  a completed change "proof-demo" ready to archive
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr names the project root and the worktree top level
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  @givn.added
  Scenario: A Fossil checkout without the fossil executable is refused by name
    Given  a Fossil checkout whose project root is the checkout root
    And  a completed change "proof-demo" with all required artifacts
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change proof-demo` without the `fossil` executable on PATH
    Then  the command exits non-zero
    And  stderr names the missing `fossil` executable
    And  the change directory still exists under givn/changes/
    And  no file in the project changed

  @givn.removed
  Scenario: A rejected candidate commit leaves the merge staged and unproven
    Given placeholder

  @givn.removed
  Scenario: Archive requires Git version control
    Given placeholder

  @givn.removed
  Scenario: Archive requires the project root to be the repository root
    Given placeholder

  @givn.removed
  Scenario: A proven archive records a verification receipt binding the tested commit
    Given placeholder
