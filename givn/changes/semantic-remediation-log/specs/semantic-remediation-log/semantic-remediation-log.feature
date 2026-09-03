# User Interaction Inventory:
# - run blocking change review with givn check review
# - archive a completed change with givn archive

@givn.delta @semantic-remediation-log
Feature: Remediation log for semantic review

  @givn.added
  Scenario: The first blocked review creates the remediation log with before evidence
    Given the remediation case carries the distinguishing marker "remediation log creation evidence aurora basalt cinder ember flint granite harbor ivory jasper kiln lagoon marble nickel opal quartz ridge summit timber velvet"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log exists
    And the remediation log declares log schema version 1 and the change id
    And the remediation log contains a before run record with embedding, BGE, and NLI stage durations
    And the remediation log contains a candidate entry with the report's candidate id and embedding score
    And the candidate entry records the BGE and NLI evidence states from the report
    And the candidate entry records the endpoint scenario text verbatim from the active delta
    And the candidate entry records the endpoint content and evidence hashes from the report

  @givn.added
  Scenario: Rerunning review with unchanged endpoints does not grow the remediation log
    Given the remediation case carries the distinguishing marker "remediation log append-only stability anchor birch cobalt dune elm fern grove inlet jasper knoll larch meadow nest opal prairie quail reed slate"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user runs `givn check review --change running-change` again without edits
    Then the remediation log still contains exactly one candidate entry and one run record

  @givn.added
  Scenario: A changed endpoint appends a new candidate entry and retains the prior entry
    Given the remediation case carries the distinguishing marker "remediation log endpoint change evidence amber bramble cedar delta elm fjord gale heather iris juniper kelp lumen moss nectar osier pine quince rowan"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the active remediation log bytes are snapshotted
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the active classified endpoint changes without remediation
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log contains a second candidate entry with different endpoint hashes
    And the prior candidate entry is retained unchanged

  @givn.added
  Scenario: The passing rerun completes the after state and the after run record
    Given the remediation case carries the distinguishing marker "remediation log completion state evidence alder beacon clover dawn elder frost glade hollow iris kelp laurel maple nectar opal pear reed summit"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["1a2b3c4d5e6f7081"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the remediation log contains an after run record
    And the removed candidate entry records after state "endpoint-absent" with a null after score

  @givn.added @wip
  Scenario: A surviving pair records its recomputed embedding score after remediation
    Given the remediation case carries the distinguishing marker "remediation log surviving score evidence ash birch cobalt fern glade heather jade knoll larch moss nettle opal plum quince reed thorn"
    Given a project prepared for semantic remediation with a "VARIANT" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "VARIANT"
    And the user records the remediation fix "consolidated the pair into one outline" and commits ["2b3c4d5e6f708192"] in the remediation log
    And the active variant is consolidated with an explicit parameter
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the remediation log records after state "outline-consolidated" with the recomputed embedding score below the boundary
    And the candidate entry records the consolidated outline text verbatim from the active delta

  @givn.added @wip
  Scenario: A resolved candidate without a complete log entry blocks review
    Given the remediation case carries the distinguishing marker "remediation log incomplete entry blocking acorn bramble clover delta elm frost grove iris jasper kiln larch moss nettle pine quill reed"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" with no commits in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "remediation log"

  @givn.added @wip
  Scenario: A remediation commit without the semantic-remediation stamp blocks review
    Given the remediation case carries the distinguishing marker "remediation log missing stamp blocking anchor birch cedar dune elm fern gale hollow iris juniper kelp larch moss oak plum reed"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidate is removed from the active delta
    And the user commits the remediation without the semantic-remediation stamp
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "semantic-remediation"

  @givn.added @wip
  Scenario: A stamp naming a different candidate blocks review
    Given the remediation case carries the distinguishing marker "remediation log mismatched stamp blocking alder bramble cedar delta fern glade heather iris juniper kelp maple nest opal plum reed willow"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidate is removed from the active delta
    And the user commits the remediation with the stamp naming candidate "0000000000000000"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "semantic-remediation"

  @givn.added @wip
  Scenario: A stamped remediation commit verifies the log and passes review
    Given the remediation case carries the distinguishing marker "remediation log stamped commit success acorn beacon cedar dune elm fern glade heather jade kiln larch maple nectar opal reed summit"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidate is removed from the active delta
    And the user commits the remediation with the matching semantic-remediation stamp
    And the user records the committed remediation fix and commit in the remediation log
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the remediation log records the after state with the stamped commit as the after anchor

  @givn.added @wip
  Scenario: A project without Git history records null run anchors and reviews without stamp checks
    Given the remediation case carries the distinguishing marker "remediation log absent history anchor ash birch cedar dune elm fern gale heather iris juniper kelp larch moss nectar opal quince"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the remediation log records the before run anchor and entry runs as null
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["3c4d5e6f708192a3"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.added @wip
  Scenario: A verified remediation from before the log existed still passes review
    Given the remediation case carries the distinguishing marker "remediation log backward compatibility acorn birch clover delta elm frost glade heather iris juniper kiln larch moss nectar opal pine"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["4d5e6f708192a3b4"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    When the user deletes the remediation log
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.added @wip
  Scenario: A repeated passing rerun does not grow the remediation log
    Given the remediation case carries the distinguishing marker "remediation log pass rerun stability beacon cedar dune elm fern glade heather iris juniper kelp larch moss nettle opal plum quince"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["5e6f708192a3b4c5"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    And the active remediation log bytes are snapshotted
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the remediation log bytes equal the active log snapshot

  @givn.added @wip
  Scenario: An unreachable remediation commit blocks review
    Given the remediation case carries the distinguishing marker "remediation log unreachable commit blocking ash birch cedar delta fern glade heather iris juniper kelp maple moss nettle opal reed willow"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidate is removed from the active delta
    And the user commits the remediation with the matching semantic-remediation stamp
    And the user removes the remediation commit from Git history
    And the user records the committed remediation fix and commit in the remediation log
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "reachable"

  @givn.added @wip
  Scenario: A Git repository without commits behaves like no Git history
    Given the remediation case carries the distinguishing marker "remediation log unborn branch anchor basil cedar dune elm fern frost glade hollow iris juniper kelp maple nectar opal quince"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository without commits
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the remediation log records the before run anchor and entry runs as null
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["6f708192a3b4c5d6"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.added @wip
  Scenario: The after state lands on the entry matching the verified evidence hashes
    Given the remediation case carries the distinguishing marker "remediation log after field matching alpha bramble cedar delta elm fern gale heather iris juniper kiln larch moss nest opal plum reed"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the active classified endpoint changes without remediation
    And the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["708192a3b4c5d6e7"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the remediation log records the after state on the second candidate entry
    And the prior candidate entry is retained unchanged

  @givn.added @wip @rulebook
  Scenario: A logged pair scored by BGE and NLI records their evidence states
    Given the remediation case carries the distinguishing marker "remediation log second stage evidence states beacon cedar dune elm fern glade heather iris juniper kelp larch moss nettle opal pine quince reed"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate and BGE and NLI evidence
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report

  @givn.added @wip
  Scenario: A commit fixing two candidates carries one stamp line per candidate
    Given the remediation case carries the distinguishing marker "remediation log multi candidate stamp lines acorn birch cedar dune elm fern gale heather iris juniper kiln larch maple nest opal reed slate"
    Given a project prepared for semantic review with two similar active deltas
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidates are removed from the active deltas
    And the user commits the remediation with one matching stamp per candidate
    And the user records both remediation fixes and the commit in the remediation log
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.added @wip
  Scenario: The remediation log obeys its strict wire grammar
    Given the remediation case carries the distinguishing marker "remediation log wire grammar discipline alpha bramble cedar delta elm fern grove heather iris juniper kiln larch moss nettle opal reed"
    Given a project prepared for semantic remediation whose delta scenario contains blank lines and an Examples table
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the remediation log has the exact headers and entry field order
    And the remediation log contains only allowed fields and no unknown fields
    And absent values are represented only by "null"
    And the remediation log has no timestamp or environment fields
    And the remediation log ends with exactly one final newline
    And the verbatim scenario text preserves interior blank lines and the Examples table

  @givn.added @wip
  Scenario: Failed review before the semantic stage does not create the remediation log
    Given the remediation case carries the distinguishing marker "remediation log early failure safety basil cedar dune elm frost grove hollow iris juniper kelp maple nettle opal pine quince"
    Given a project prepared for semantic review with a malformed delta feature
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log does not exist

  @givn.added @wip
  Scenario: The remediation log survives the archive move
    Given the remediation case carries the distinguishing marker "remediation log archive retention evidence acorn birch clover delta elm fern glade heather iris juniper kiln larch moss opal plum quince reed"
    Given a completed semantic remediation change with a generated log and archive prerequisites
    And the active remediation log bytes are snapshotted
    And the project declares a complete run declaration
    When the user runs `givn archive --change running-change`
    Then the command exits 0
    And `givn/archive/running-change/semantic-remediation.md` exists
    And the archived remediation log bytes equal the active log snapshot
    And the active remediation log is unchanged by the archive run

  @givn.added @wip
  Scenario: Advisory spec review does not create the remediation log
    Given the remediation case carries the distinguishing marker "remediation log advisory boundary ash birch cedar delta elm fern glade heather iris juniper kiln larch moss nettle opal plum quince reed"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs the advisory semantic review command
    Then the command exits 0
    And stdout contains "retrieval advisory"
    And the remediation log does not exist

  @givn.added @wip
  Scenario: Blocking review without its retrieval feature does not create the remediation log
    Given the remediation case carries the distinguishing marker "remediation log feature-free boundary acorn bramble cedar delta elm fern glade heather iris juniper kiln larch moss nettle opal quince reed slate"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs change review through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
    And the remediation log does not exist

  @givn.added @e2e @wip
  Scenario: Real CLI review completes the remediation log for a stamped duplicate remediation
    Given the remediation case carries the distinguishing marker "remediation log real review completion alder beacon cedar dune elm fern glade heather jade kiln larch maple nectar opal reed summit"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-e2e"
    When the remediation E2E user runs change review for "rc-log-e2e" after classifying, removing, and stamp-committing the duplicate
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the remediation log records the before and after run records with stage durations and the stamped commit

  @givn.added @e2e @wip
  Scenario: Real CLI archive retains the remediation log
    Given the remediation case carries the distinguishing marker "remediation log real archive retention basil cedar dune elm frost glade hollow iris juniper kelp maple nettle opal pine quince reed slate"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-archive-e2e"
    When the remediation E2E user archives change "rc-log-archive-e2e" after a verified stamped remediation
    Then the command exits 0
    And the archived remediation log exists next to the archived semantic report
