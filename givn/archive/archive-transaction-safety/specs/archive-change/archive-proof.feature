@givn.delta @archive-proof

Feature: Archive Proof

  @givn.modified
  Scenario: A change present as both active and archived is refused as corrupt
    Given  a change "proof-demo" that exists both under givn/changes/ and givn/archive/
    And  the pending archive candidate conflicts with the active change
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the pending archive candidate conflicts with the active change
    And  the pending receipt is unchanged
    And  the change directory still exists under givn/changes/

  @givn.added
  Scenario: A change present as both active and archived without a receipt is refused as corrupt
    Given  a change "proof-demo" that exists both under givn/changes/ and givn/archive/
    And  the archive candidate has no verification receipt
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits non-zero
    And  stderr says the project state is corrupt and both directories exist
    And  the change directory still exists under givn/changes/

  @givn.added
  Scenario: An interrupted archive move resumes from an identical pending candidate
    Given  an interrupted archive move for change "proof-demo" with an identical pending candidate
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the change directory no longer exists under givn/changes/
    And  each merged scenario appears exactly once in the permanent specs
    And  the verification receipt names the tested revision

  @givn.added
  Scenario: An interrupted merge resumes from its pending candidate
    Given  a pending archive candidate "proof-demo" whose delta merge did not complete
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the permanent specs contain the merged delta scenarios
    And  each merged scenario appears exactly once in the permanent specs

  @givn.added
  Scenario: An in-flight delta file is retried without duplicating its scenarios
    Given  a pending archive candidate "proof-demo" whose delta file was applied but not recorded
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  each merged scenario appears exactly once in the permanent specs

  @givn.added
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

  @givn.added
  Scenario Outline: Change evidence does not block the archive
    Given  a givn project with a completed change "proof-demo" ready to archive
    And  the change has an uncommitted evidence file "<evidence>"
    And  the full suite passes against the merged permanent specs
    When  the user runs `givn archive --change proof-demo`
    Then  the command exits 0
    And  the archive candidate is proven
    And  the archived change contains the evidence file "<evidence>"

    Examples:
      | evidence               |
      | evidence/final.log     |
      | verification/notes.txt |

  @givn.added
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

  @givn.added
  Scenario: An interrupted archive move is surfaced by project status
    Given  an interrupted archive move for change "proof-demo" with an identical pending candidate
    When  the user runs `givn status`
    Then  stdout lists "proof-demo" as an unproven archive
    And  stdout routes the developer to resume it with `givn archive --change proof-demo`
