# User Interaction Inventory:
# - run blocking change review with givn check review
# - archive a completed change with givn archive

@givn.delta @semantic-remediation-log
Feature: Remediation log for semantic review

  @givn.added
  Scenario: The first blocked review creates the remediation log with before evidence
    Given the remediation case carries the distinguishing marker "bolt cleat elm fern keel pumice quern rudder"
    Given the remediation fixture files carry the telescope corpus:
      """
Feature: Telescope calibration

  Scenario: The dome alignment is recorded twice
    Given a mounted telescope at dusk
    When the operator logs the alignment
    Then the calibration book holds one entry
      --- delta ---
@givn.delta @running-change
Feature: Telescope calibration change

  @givn.added
  Scenario: The dome alignment is recorded twice
    Given a mounted telescope at dusk
    When the operator logs the alignment
    Then the calibration book holds one entry
      """
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
    Given the remediation case carries the distinguishing marker "elm ibis javelin latch orchard parapet plum summit"
    Given the remediation fixture files carry the bakery corpus:
      """
Feature: Bakery ledger

  Scenario: The sourdough batch is logged twice
    Given a proofing schedule for the night
    When the baker records the sourdough batch
    Then the ledger shows one entry
      --- delta ---
@givn.delta @running-change
Feature: Bakery ledger change

  @givn.added
  Scenario: The sourdough batch is logged twice
    Given a proofing schedule for the night
    When the baker records the sourdough batch
    Then the ledger shows one entry
      """
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user runs `givn check review --change running-change` again without edits
    Then the remediation log still contains exactly one candidate entry and one run record

  Scenario Outline: An invalid remediation evidence state blocks the review
    Given the remediation case carries the distinguishing marker "accordion apricot grapple keel quill saffron scythe vest"
    Given a remediation-log project with one "DUPLICATE" finding <repository-state>
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the doubled scenario is withdrawn from the change specs
    And the remediation evidence is left <evidence-state>
    And the gate is evaluated a second time
    Then the run fails
    And the diagnostic names "<expected-diagnostic>"

    Examples:
      | repository-state                    | evidence-state                             | expected-diagnostic |
      | without Git history                 | without commit evidence                    | remediation log     |
      | inside a committed Git repository   | without the stamp                          | semantic-remediation |
      | inside a committed Git repository   | with a stamp naming another candidate      | semantic-remediation |
      | inside a committed Git repository   | with its commit removed from the history   | reachable           |

  Scenario Outline: A verified remediation whose log record was <log-state> still passes review
    Given the remediation case carries the distinguishing marker "dinghy fern harness heather nozzle whip wren zinnia"
    Given a duplicate-log project contains a "DUPLICATE" case
    And the run declaration of the project is complete
    When the blocking review is executed for running-change
    And the operator labels every review candidate "DUPLICATE"
    And the operator records the fix "removed the twin" with commits ["4d5e6f70"]
    And the remediation log is <log-state>
    And the redundant scenario is deleted from the delta
    And the review command reruns for running-change
    Then the review passes with the remediation receipt

    Examples:
      | log-state                           |
      | deleted entirely                    |
      | stripped of its candidate entries   |

  Scenario Outline: A <repository-state> records null run anchors and reviews without stamp checks
    Given the remediation case carries the distinguishing marker "jackdaw jetty mandrel mist needle urn vane yoke"
    Given a remediation-log project with one "DUPLICATE" finding <repository-state>
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the reviewer logs the fix "took the surplus out" and the commits ["3c4d5e6f"]
    And the doubled scenario is withdrawn from the change specs
    Then the remediation log records the before run anchor as null
    And the gate is evaluated a second time
    Then the review passes with the remediation receipt

    Examples:
      | repository-state                    |
      | without Git history                 |
      | inside an unborn Git repository     |

  @givn.added
  Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
    Given the remediation case carries the distinguishing marker "cleat domino lantern mist oarlock sandal thistle wren"
    Given the remediation fixture files carry the ferry corpus:
      """
Feature: Ferry manifest

  Scenario: The car deck count is noted twice
    Given a ferry at the loading ramp
    When the steward counts the cars
    Then the manifest shows one deck count
      --- delta ---
@givn.delta @running-change
Feature: Ferry manifest change

  @givn.added
  Scenario: The car deck count is noted twice
    Given a ferry at the loading ramp
    When the steward counts the cars
    Then the manifest shows one deck count
      """
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the fix "removed the duplicate" and commits ["1a2b3c4d"]
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the review passes with the remediation receipt
    And the remediation log contains an after run record
    And the removed candidate entry records after state "endpoint-absent" with a null after score
    And the active remediation log bytes are snapshotted
    When the user runs `givn check review --change running-change` again without edits
    Then the remediation log bytes equal the active log snapshot

  @givn.added
  Scenario: A changed endpoint appends a candidate entry and receives the verified after state
    Given the remediation case carries the distinguishing marker "basin ember marlin quartz spar tongs"
    Given the remediation fixture files carry the orchard corpus:
      """
Feature: Orchard harvest log

  Scenario: The cider press yield is recorded twice
    Given a cider press during harvest
    Then the harvest log holds one entry
      --- delta ---
@givn.delta @running-change
Feature: Orchard harvest log change

  @givn.added
  Scenario: The cider press yield is recorded twice
    Given a cider press during harvest
    Then the harvest log holds one entry
      """
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the active remediation log bytes are snapshotted
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the active classified corpus endpoint changes without remediation
    And the gate is evaluated a second time
    Then the run fails
    And the remediation log contains a second candidate entry with different endpoint hashes
    And the prior candidate entry is retained unchanged
    When the reviewer marks each semantic finding as "DUPLICATE"
    And the reviewer logs the fix "removed the superseded twin" and the commits ["708192a3"]
    And the doubled scenario is withdrawn from the change specs
    And the gate is evaluated a third time
    Then the review passes with the remediation receipt
    And the remediation log records the after state on the second candidate entry

  @givn.added
  Scenario: A surviving pair records its recomputed embedding score after remediation
    Given the remediation case carries the distinguishing marker "abacus lumen lyre moss nickel quadrant scythe xylophone"
    Given the remediation fixture files carry the beehive corpus:
      """
Feature: Beehive inspection

  Scenario Outline: The queen sighting is logged twice
    Given an inspected beehive in summer
    When the beekeeper logs the queen sighting
    Then the inspection card holds one entry
      --- delta ---
@givn.delta @running-change
Feature: Beehive inspection change

  @givn.added
  Scenario Outline: The queen sighting is logged twice
    Given an inspected beehive in summer
    When the beekeeper logs the queen sighting
    Then the inspection card holds one entry
      """
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "VARIANT"
    And the user records the fix "consolidated" and commits ["2b3c4d5e"]
    And the active corpus variant is consolidated with an explicit parameter
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the remediation log records after state "outline-consolidated" with the recomputed embedding score below the boundary
    And the candidate entry records the consolidated outline text verbatim from the active delta

  @givn.added @rulebook
  Scenario: A logged pair scored by BGE and NLI records their evidence states
    Given the remediation case carries the distinguishing marker "accordion almanac bone hammock heather rivet spar zephyr"
    Given the remediation fixture files carry the greenhouse corpus:
      """
Feature: Greenhouse watering plan

  Scenario: The orchid watering is scheduled twice
    Given an orchid bench with a moisture probe
    When the gardener plans the watering
    Then the plan shows one schedule entry
      --- delta ---
@givn.delta @running-change
Feature: Greenhouse watering plan change

  @givn.added
  Scenario Outline: The orchid watering is scheduled twice
    Given an orchid bench with a moisture probe
    When the gardener plans the watering
    Then the plan shows one schedule entry
      """
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report

  @givn.added
  Scenario: A commit fixing two candidates carries one stamp line per candidate
    Given the remediation case carries the distinguishing marker "beacon cobalt fennel iris nettle pumice quill vane"
    Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidates are removed from the active deltas
    And the user commits the remediation with one matching stamp per candidate
    And the user records both remediation fixes and the commit in the remediation log
    And the user runs `givn check review --change running-change`
    Then the review passes with the remediation receipt

  @givn.added
  Scenario: The remediation log obeys its strict wire grammar
    Given the remediation case carries the distinguishing marker "cobalt elm locomotive lumen piston ridge summit wharf"
    Given a project prepared for semantic remediation whose delta scenario contains blank lines and an Examples table
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the remediation log has the exact headers and entry field order
    And the remediation log contains only allowed fields and no unknown fields
    And absent values are represented only by "null"
    And the remediation log has no timestamp or environment fields
    And the remediation log ends with exactly one final newline
    And the verbatim scenario text preserves interior blank lines and the Examples table

  @givn.added
  Scenario: Failed review before the semantic stage does not create the remediation log
    Given the remediation case carries the distinguishing marker "eucalyptus fennel inlet moss needle nutmeg xylophone yacht"
    Given a project prepared for semantic review with a malformed delta feature
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log does not exist

  @givn.added
  Scenario: The remediation log survives the archive move
    Given the remediation case carries the distinguishing marker "accordion acorn jackal knoll lattice lumen orchard umber"
    Given a completed semantic remediation change with a generated log and archive prerequisites
    And the active remediation log bytes are snapshotted
    And the project declares a complete run declaration
    When the user runs `givn archive --change running-change`
    Then the command exits 0
    And `givn/archive/running-change/semantic-remediation.md` exists
    And the archived remediation log bytes equal the active log snapshot

  @givn.added
  Scenario: Advisory spec review does not create the remediation log
    Given the remediation case carries the distinguishing marker "clover kiln nettle nutmeg rivet trowel tulip wharf"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs the advisory semantic review command
    Then the command exits 0
    And stdout contains "retrieval advisory"
    And the remediation log does not exist

  @givn.added
  Scenario: Blocking review without its retrieval feature does not create the remediation log
    Given the remediation case carries the distinguishing marker "beacon fern heather oarlock oboe thistle trowel vest"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs change review through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
    And the remediation log does not exist

  @givn.added @e2e
  Scenario: Real CLI review completes the remediation log for a stamped duplicate remediation
    Given the remediation case carries the distinguishing marker "abacus anchor apricot cleat domino dune ember glacier"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-e2e"
    When the remediation E2E user runs change review for "rc-log-e2e" after classifying, removing, and stamp-committing the duplicate
    Then the review passes with the remediation receipt
    And the remediation log records the before and after run records with stage durations and the stamped commit

  @givn.added @e2e
  Scenario: Real CLI archive retains the remediation log
    Given the remediation case carries the distinguishing marker "abacus atlas domino eddy needle piston valve zinnia"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-archive-e2e"
    When the remediation E2E user archives change "rc-log-archive-e2e" after a verified stamped remediation
    Then the command exits 0
    And the archived remediation log exists next to the archived semantic report

  @givn.removed
  Scenario: A complete classification allows change review and survives regeneration
    Given a placeholder step

  @givn.removed
  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given a placeholder step

  @givn.removed
  Scenario: Changing a candidate scenario invalidates its previous classification
    Given a placeholder step

  @givn.removed
  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given a placeholder step

  @givn.removed
  Scenario: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given a placeholder step

  @givn.removed
  Scenario: Removed candidates disappear after classification
    Given a placeholder step

  @givn.removed
  Scenario: A modified delta excludes only its own projected target
    Given a placeholder step

  @givn.added
  Scenario: Superseded permanent endpoints leave the semantic pair universe
    Given a project prepared for semantic review with a modified scenario and another candidate in the same target file
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the report excludes the modified scenario's own projected target endpoint
    And the report still contains the other candidate in that target file
    And the report records the modified delta operation and projected target identity
    And the report pairs no live endpoint against a superseded permanent body

  @givn.modified
  Scenario: Removing a DUPLICATE candidate verifies the projected remediation
    Given the remediation case carries the distinguishing marker "cinder gondola locomotive rivet timber trowel tundra umber"
    Given the remediation fixture files carry the observatory corpus:
      """
Feature: Observatory night log

  Scenario: The seeing conditions are noted twice
    Given a clear night at the observatory
    When the astronomer notes the seeing
    Then the night log holds one entry
      --- delta ---
@givn.delta @running-change
Feature: Observatory night log change

  @givn.added
  Scenario: The seeing conditions are noted twice
    Given a clear night at the observatory
    When the astronomer notes the seeing
    Then the night log holds one entry
      """
    And  the permanent specifications are snapshotted
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the gate operator marks the candidate "DUPLICATE"
    And the gate operator files the fix "removed" with the record ["0a1b2c40"]
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the review passes with the remediation receipt
    And the semantic review report status is "PASS"
    And the permanent specifications are unchanged

  @givn.removed
  Scenario: Consolidating a VARIANT candidate verifies the projected remediation
    Given a placeholder step

  @givn.removed
  Scenario: Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation
    Given a placeholder step

  @givn.modified
  Scenario: The migration preserves independent architecture history across three proposed decisions
    Given the remediation case carries the distinguishing marker "harness jib mandrel nozzle oboe yacht yarn zither"
    Given  the consolidated ADR register and canonical sources are available
    When  a maintainer compares the register with the canonical sources
    Then  the final register has 36 accepted decisions and four proposed decisions
    And  the change's migration manifest gives every assigned identifier exactly one disposition
    And  reserved identifiers `ADR-050` and `ADR-051` have no record
    And  non-ADR records are removed from the active register and ADR archive
    And  absorbed records are absent and their refinements are in the target ADRs
    And  superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
    And  `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
    And  independent architecture decisions remain separate
    And  `ADR-063` remains proposed and owned by its active change
