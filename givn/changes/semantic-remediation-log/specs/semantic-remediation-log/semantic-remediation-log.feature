# User Interaction Inventory:
# - run blocking change review with givn check review
# - archive a completed change with givn archive

@givn.delta @semantic-remediation-log
Feature: Remediation log for semantic review

  @givn.added
  Scenario: The first blocked review creates the remediation log with before evidence
    Given the remediation case carries the distinguishing marker "quartz jetty fennel accordion basin locomotive marble wren cinder"
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
    Given the remediation case carries the distinguishing marker "cinder apricot summit velvet anchor nickel prairie quill saffron"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user runs `givn check review --change running-change` again without edits
    Then the remediation log still contains exactly one candidate entry and one run record

  Scenario Outline: An invalid remediation evidence state blocks the review
    Given the remediation case carries the distinguishing marker "timber xylophone umber glacier mist bone orchard latch cobalt dune"
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
    Given the remediation case carries the distinguishing marker "ember filbert grove iris jasper kiln lagoon nettle opal"
    Given a duplicate-log project contains a "DUPLICATE" case
    And the run declaration of the project is complete
    When the blocking review is executed for running-change
    And the operator labels every review candidate "DUPLICATE"
    And the operator records the performed fix "took the doubled scenario away" with commits ["4d5e6f708192a3b4"] in the log
    And the remediation log is <log-state>
    And the redundant scenario is deleted from the delta
    And the review command reruns for running-change
    Then the run succeeds
    And stdout announces the verified remediation

    Examples:
      | log-state                           |
      | deleted entirely                    |
      | stripped of its candidate entries   |

  Scenario Outline: A <repository-state> records null run anchors and reviews without stamp checks
    Given the remediation case carries the distinguishing marker "plum ridge slate tulip vest walnut yarrow zinc atlas bramble"
    Given a remediation-log project with one "DUPLICATE" finding <repository-state>
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the reviewer logs the applied fix "took the surplus scenario out" and the commit list ["3c4d5e6f708192a3"] into the remediation log
    And the doubled scenario is withdrawn from the change specs
    Then the remediation log records the before run anchor as null
    And the gate is evaluated a second time
    Then the run succeeds
    And stdout carries the remediation verification receipt

    Examples:
      | repository-state                    |
      | without Git history                 |
      | inside an unborn Git repository     |

  @givn.added
  Scenario: The passing rerun completes the after state, and a further rerun leaves the log unchanged
    Given the remediation case carries the distinguishing marker "clover elm fern gale heather juniper kelp lumen moss pine"
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
    And the active remediation log bytes are snapshotted
    When the user runs `givn check review --change running-change` again without edits
    Then the remediation log bytes equal the active log snapshot

  @givn.added
  Scenario: A changed endpoint appends a new candidate entry that later receives the verified after state
    Given the remediation case carries the distinguishing marker "quince reed thistle urn vine zinnia acorn beacon canyon domino"
    Given a remediation-log project keeps one "DUPLICATE" finding
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the active remediation log bytes are snapshotted
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the active classified endpoint changes without remediation
    And the gate is evaluated a second time
    Then the run fails
    And the remediation log contains a second candidate entry with different endpoint hashes
    And the prior candidate entry is retained unchanged
    When the reviewer marks each semantic finding as "DUPLICATE"
    And the reviewer logs the applied fix "removed the superseded duplicate" and the commit list ["708192a3b4c5d6e7"] into the remediation log
    And the doubled scenario is withdrawn from the change specs
    And the gate is evaluated a third time
    Then the run succeeds
    And stdout carries the remediation verification receipt
    And the remediation log records the after state on the second candidate entry

  @givn.added
  Scenario: A surviving pair records its recomputed embedding score after remediation
    Given the remediation case carries the distinguishing marker "eucalyptus falcon garnet hammock inkwell jackdaw kettle lantern marlin needle"
    Given a semantic-remediation logging project holding one "VARIANT" finding
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "VARIANT"
    And the user records the remediation fix "consolidated the pair into one outline" and commits ["2b3c4d5e6f708192"] in the remediation log
    And the active variant is consolidated with an explicit parameter
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the remediation log records after state "outline-consolidated" with the recomputed embedding score below the boundary
    And the candidate entry records the consolidated outline text verbatim from the active delta

  @givn.added @rulebook
  Scenario: A logged pair scored by BGE and NLI records their evidence states
    Given the remediation case carries the distinguishing marker "ocelot parapet quarry rivet sandal tundra vignette whip yacht zephyr"
    Given a semantic-remediation logging project holding one "DUPLICATE" finding and BGE and NLI evidence
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report

  @givn.added
  Scenario: A commit fixing two candidates carries one stamp line per candidate
    Given the remediation case carries the distinguishing marker "almanac bellows candelabra dredger effigy gondola hinges islet jackal knoll"
    Given a project prepared for semantic review with two permanent-duplicate deltas in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidates are removed from the active deltas
    And the user commits the remediation with one matching stamp per candidate
    And the user records both remediation fixes and the commit in the remediation log
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.added
  Scenario: The remediation log obeys its strict wire grammar
    Given the remediation case carries the distinguishing marker "lyre mantle nutmeg oboe piston scythe tongs umbrella vane willow"
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
    Given the remediation case carries the distinguishing marker "yarn abacus crate dowel easel fluke harness inlet javelin keel"
    Given a project prepared for semantic review with a malformed delta feature
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log does not exist

  @givn.added
  Scenario: The remediation log survives the archive move
    Given the remediation case carries the distinguishing marker "lattice mandrel nozzle pumice quadrant rudder shim trowel udder valve"
    Given a completed semantic remediation change with a generated log and archive prerequisites
    And the active remediation log bytes are snapshotted
    And the project declares a complete run declaration
    When the user runs `givn archive --change running-change`
    Then the command exits 0
    And `givn/archive/running-change/semantic-remediation.md` exists
    And the archived remediation log bytes equal the active log snapshot

  @givn.added
  Scenario: Advisory spec review does not create the remediation log
    Given the remediation case carries the distinguishing marker "wand yoke zenith anvil bolt cleat dinghy eddy flange grapple"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs the advisory semantic review command
    Then the command exits 0
    And stdout contains "retrieval advisory"
    And the remediation log does not exist

  @givn.added
  Scenario: Blocking review without its retrieval feature does not create the remediation log
    Given the remediation case carries the distinguishing marker "halyard ibis jib keelson limpet oarlock prow quern spar tackline"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs change review through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
    And the remediation log does not exist

  @givn.added @e2e
  Scenario: Real CLI review completes the remediation log for a stamped duplicate remediation
    Given the remediation case carries the distinguishing marker "urchin wharf zither apron buoy clipper derrick estuary fathom gangway"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-e2e"
    When the remediation E2E user runs change review for "rc-log-e2e" after classifying, removing, and stamp-committing the duplicate
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the remediation log records the before and after run records with stage durations and the stamped commit

  @givn.added @e2e
  Scenario: Real CLI archive retains the remediation log
    Given the remediation case carries the distinguishing marker "hull isthmus jetty keelson lighthouse mast navigation quay rigging spar"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-archive-e2e"
    When the remediation E2E user archives change "rc-log-archive-e2e" after a verified stamped remediation
    Then the command exits 0
    And the archived remediation log exists next to the archived semantic report

  @givn.modified
  Scenario: A complete classification allows change review and survives regeneration
    Given the remediation case carries the distinguishing marker "tackline urchin wharf xebec yacht zodiac anchorline brine compass depth"
    Given the remediation-log fixture stages one "DUPLICATE" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the assessor resolves the admitted candidate to "DUPLICATE"
    And the assessor registers the applied correction "collapsed the doubled flow" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.modified
  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given the remediation case carries the distinguishing marker "eddy fathom gale hull inlet kelp navigation oar quay rigging"
    Given the remediation-log fixture reproduces one "DUPLICATE" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the assessor resolves the admitted candidate to "DUPLICATE"
    And the assessor registers the applied correction "settled the repeated flow" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Changing a candidate scenario invalidates its previous classification
    Given the remediation case carries the distinguishing marker "shroud trawl untow wharfyard xebec zither apron buoy clipper derrick"
    Given the remediation-log fixture stages one "DUPLICATE" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the gate operator marks the admitted candidate as "DUPLICATE"
    And the gate operator files the executed correction "withdrew the doubled scenario" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given the remediation case carries the distinguishing marker "estuary fathom gangway hull isthmus jetty keelson lighthouse mast navigation"
    Given the remediation-log fixture stages one "DUPLICATE" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the gate operator marks the admitted candidate as "DUPLICATE"
    And the gate operator files the executed correction "retired the mirrored scenario" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given the remediation case carries the distinguishing marker "oarlock prow quayside shroud trawl untow wharfyard xebec zither apron"
    Given the review-flow fixture prepares a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the assessor resolves the admitted candidate to "DUPLICATE"
    And the assessor registers the applied correction "dropped the reworked scenario" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the active classified endpoint changes without remediation
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And a later candidate entry records different endpoint hashes than its predecessor
    And the prior candidate entry is retained unchanged

  @givn.modified
  Scenario: Removed candidates disappear after classification
    Given the remediation case carries the distinguishing marker "buoy clipper derrick estuary fathom gangway hull isthmus jetty keelson"
    Given the remediation-log fixture erases one "DUPLICATE" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the assessor resolves the admitted candidate to "DUPLICATE"
    And the assessor registers the applied correction "erased the redundant copy" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Removing a DUPLICATE candidate verifies the projected remediation
    Given the remediation case carries the distinguishing marker "lighthouse mast navigation quay rigging spar tackline urchin wharf zither"
    Given the remediation-log fixture stages one "DUPLICATE" case
    And  the permanent specifications are snapshotted
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the gate operator marks the admitted candidate as "DUPLICATE"
    And the gate operator files the executed correction "removed the overlapping case" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
    And the permanent specifications are unchanged

  @givn.modified
  Scenario: Consolidating a VARIANT candidate verifies the projected remediation
    Given the remediation case carries the distinguishing marker "apron buoy clipper derrick estuary fathom gangway hull isthmus jetty"
    Given the remediation-log fixture stages one "VARIANT" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the assessor resolves the admitted candidate to "VARIANT"
    And the assessor registers the applied correction "folded the twin flows into one outline" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the active variant is consolidated with an explicit parameter
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.modified
  Scenario: Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation
    Given the remediation case carries the distinguishing marker "anchorline brine compass depth eddy fathom gale hull inlet kelp"
    Given the review-flow fixture prepares a "VALID-BOUNDARY" case
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the assessor resolves the admitted candidate to "VALID-BOUNDARY"
    And the assessor registers the applied correction "sharpened the boundary distinction" with the commit record ["0123456789abcdef0123456789abcdef01234567"] in the log
    And the valid boundary is rewritten with an explicit distinction below the similarity boundary
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.modified
  Scenario: The migration preserves independent architecture history across three proposed decisions
    Given the remediation case carries the distinguishing marker "navigation quay rigging spar tackline urchin wharf xebec yacht zodiac"
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
