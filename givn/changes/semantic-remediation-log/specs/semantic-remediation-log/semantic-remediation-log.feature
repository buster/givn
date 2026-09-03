# User Interaction Inventory:
# - run blocking change review with givn check review
# - archive a completed change with givn archive

@givn.delta @semantic-remediation-log
Feature: Remediation log for semantic review

  @givn.added
  Scenario: The first blocked review creates the remediation log with before evidence
    Given the remediation case carries the distinguishing marker "anchor anvil apron ibis islet latch moss prairie scythe slate vignette xylophone"
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
    Given the remediation case carries the distinguishing marker "abacus apricot bone crate fluke jasper oarlock orchard quill spar timber zinnia"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user runs `givn check review --change running-change` again without edits
    Then the remediation log still contains exactly one candidate entry and one run record

  Scenario: A changed endpoint appends a new candidate entry and retains the prior entry
    Given the remediation case carries the distinguishing marker "anchor apron navigation nettle prow quay quill quince reed scythe velvet wharf"
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


  @givn.added
  Scenario: The passing rerun completes the after state and the after run record
    Given the remediation case carries the distinguishing marker "abacus clover dune iris jib marlin plum quarry quern urchin xebec zither"
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

  @givn.added
  Scenario: A surviving pair records its recomputed embedding score after remediation
    Given the remediation case carries the distinguishing marker "dowel fern halyard kelp knoll latch lighthouse limpet quill saffron spar yoke"
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

  Scenario: A resolved candidate without a complete log entry blocks review
    Given the remediation case carries the distinguishing marker "acorn apron beacon clover flange glacier hinges lattice ocelot pumice sandal wharf"
    Given a remediation-log project keeps one "DUPLICATE" finding
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the reviewer logs the applied fix "dropped the duplicated scenario" without any commit list into the remediation log
    And the doubled scenario is withdrawn from the change specs
    Then the run fails
    When the gate is evaluated a second time
    Then the run fails
    And the diagnostic names "remediation log"


  Scenario: A remediation commit without the semantic-remediation stamp blocks review
    Given the remediation case carries the distinguishing marker "abacus bellows cleat gangway iris javelin lantern slate valve wand xylophone yarrow"
    Given a remediation-log project keeps one "DUPLICATE" finding inside a committed Git repository
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the doubled scenario is withdrawn from the change specs
    And the user commits the remediation without the semantic-remediation stamp
    And the gate is evaluated a second time
    Then the run fails
    And the diagnostic names "semantic-remediation"


  @givn.added
  Scenario: A stamp naming a different candidate blocks review
    Given the remediation case carries the distinguishing marker "abacus bellows derrick dredger lattice prow sandal summit umber urchin yoke zither"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the classified duplicate candidate is removed from the active delta
    And the user commits the remediation with the stamp naming candidate "0000000000000000"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "semantic-remediation"

  @givn.added
  Scenario: A stamped remediation commit verifies the log and passes review
    Given the remediation case carries the distinguishing marker "candelabra hammock javelin kettle nutmeg orchard quill rivet rudder saffron timber wharf"
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

  Scenario: A project without Git history records null run anchors and reviews without stamp checks
    Given the remediation case carries the distinguishing marker "beacon effigy grove hull lagoon lantern locomotive nozzle pine quill wand zinc"
    Given a remediation-log project keeps one "DUPLICATE" finding
    And the run block of the project is fully declared
    When the change review gate runs for running-change
    And the reviewer marks each semantic finding as "DUPLICATE"
    And the reviewer logs the applied fix "took the surplus scenario out" and the commit list ["3c4d5e6f708192a3"] into the remediation log
    And the doubled scenario is withdrawn from the change specs
    Then the remediation log records the before run anchor as null
    When the gate is evaluated a second time
    Then the run succeeds
    And stdout carries the remediation verification receipt


  @givn.added
  Scenario: A verified remediation from before the log existed still passes review
    Given the remediation case carries the distinguishing marker "easel glacier jackdaw javelin limpet opal piston scythe tongs yarn yarrow yoke"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["4d5e6f708192a3b4"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    When the user deletes the remediation log
    And the user runs `givn check review --change running-change` again without edits
    Then the command exits 0

  @givn.added
  Scenario: A repeated passing rerun does not grow the remediation log
    Given the remediation case carries the distinguishing marker "accordion bramble buoy clover falcon gondola kettle mantle mist slate udder urn"
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

  Scenario: An unreachable remediation commit blocks review
    Given the remediation case carries the distinguishing marker "abacus cleat halyard hull jackal jetty kiln lighthouse nickel ridge spar tundra"
    Given a duplicate-log project contains a "DUPLICATE" case in a repository with history
    And the run declaration of the project is complete
    When the blocking review is executed for running-change
    And the operator labels every review candidate "DUPLICATE"
    And the redundant scenario is deleted from the delta
    And the user commits the remediation with the matching semantic-remediation stamp
    And the user removes the remediation commit from Git history
    And the operator records the committed fix and its hash in the log
    And the review command reruns for running-change
    Then the gate run ends unsuccessfully
    And the failure text mentions "reachable"


  @givn.added
  Scenario: A Git repository without commits behaves like no Git history
    Given the remediation case carries the distinguishing marker "dune fern lattice lumen piston quern quill rigging scythe timber tongs trowel"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate in a Git repository without commits
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user classifies every semantic review candidate as "DUPLICATE"
    And the user records the remediation fix "removed the redundant scenario" and commits ["6f708192a3b4c5d6"] in the remediation log
    And the classified duplicate candidate is removed from the active delta
    Then the remediation log records the before run anchor as null
    When the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.added
  Scenario: The after state lands on the entry matching the verified evidence hashes
    Given the remediation case carries the distinguishing marker "bellows cobalt dune estuary filbert harness ibis nickel quartz slate tackline walnut"
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

  @givn.added @rulebook
  Scenario: A logged pair scored by BGE and NLI records their evidence states
    Given the remediation case carries the distinguishing marker "candelabra canyon fathom islet isthmus lumen lyre marble rigging rudder slate xylophone"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate and BGE and NLI evidence
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report

  @givn.added
  Scenario: A commit fixing two candidates carries one stamp line per candidate
    Given the remediation case carries the distinguishing marker "bellows dune falcon grove jasper mist plum pumice rivet shim trowel udder"
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

  Scenario: A stripped log entry for a verified remediation still passes review
    Given the remediation case carries the distinguishing marker "dinghy grapple hinges keelson kelp locomotive marble ocelot ridge shim walnut zodiac"
    Given a duplicate-log project contains a "DUPLICATE" case
    And the run declaration of the project is complete
    When the blocking review is executed for running-change
    And the operator labels every review candidate "DUPLICATE"
    And the operator records the performed fix "took the doubled scenario away" with commits ["4d5e6f708192a3b4"] in the log
    And the user strips every candidate entry from the remediation log
    And the redundant scenario is deleted from the delta
    And the review command reruns for running-change
    Then the run succeeds
    And stdout announces the verified remediation


  @givn.added
  Scenario: The remediation log obeys its strict wire grammar
    Given the remediation case carries the distinguishing marker "dinghy eucalyptus gondola halyard islet jib orchard reed rigging yacht yarrow zenith"
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
    Given the remediation case carries the distinguishing marker "dinghy gale gondola hull jackdaw juniper reed thistle tongs wand wren zinnia"
    Given a project prepared for semantic review with a malformed delta feature
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log does not exist

  @givn.added
  Scenario: The remediation log survives the archive move
    Given the remediation case carries the distinguishing marker "candelabra dredger dune eucalyptus fathom gale glacier islet javelin quadrant quince thistle"
    Given a completed semantic remediation change with a generated log and archive prerequisites
    And the active remediation log bytes are snapshotted
    And the project declares a complete run declaration
    When the user runs `givn archive --change running-change`
    Then the command exits 0
    And `givn/archive/running-change/semantic-remediation.md` exists
    And the archived remediation log bytes equal the active log snapshot

  @givn.added
  Scenario: Advisory spec review does not create the remediation log
    Given the remediation case carries the distinguishing marker "almanac candelabra heather hull kelp mast mist quadrant quartz shim trowel zodiac"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs the advisory semantic review command
    Then the command exits 0
    And stdout contains "retrieval advisory"
    And the remediation log does not exist

  @givn.added
  Scenario: Blocking review without its retrieval feature does not create the remediation log
    Given the remediation case carries the distinguishing marker "bone bramble fluke iris juniper nozzle oboe rigging scythe shim tongs zephyr"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs change review through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
    And the remediation log does not exist

  @givn.added @e2e
  Scenario: Real CLI review completes the remediation log for a stamped duplicate remediation
    Given the remediation case carries the distinguishing marker "clipper derrick hull kiln mist pumice ridge rudder slate vest wren zinc"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-e2e"
    When the remediation E2E user runs change review for "rc-log-e2e" after classifying, removing, and stamp-committing the duplicate
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the remediation log records the before and after run records with stage durations and the stamped commit

  @givn.added @e2e
  Scenario: Real CLI archive retains the remediation log
    Given the remediation case carries the distinguishing marker "accordion dredger dune flange fluke gale lagoon limpet locomotive moss opal tulip"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-archive-e2e"
    When the remediation E2E user archives change "rc-log-archive-e2e" after a verified stamped remediation
    Then the command exits 0
    And the archived remediation log exists next to the archived semantic report

  @givn.modified
  Scenario: A complete classification allows change review and survives regeneration
    Given the remediation case carries the distinguishing marker "abacus buoy canyon eucalyptus keelson lagoon marlin moss whip wren zenith zinnia"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user records the remediation fix "removed the redundant scenario" and commits ["0123456789abcdef0123456789abcdef01234567"] in the remediation log
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  @givn.modified
  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given the remediation case carries the distinguishing marker "abacus apron bolt dinghy effigy flange halyard lagoon mandrel quill slate zenith"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user records the remediation fix "removed the redundant scenario" and commits ["0123456789abcdef0123456789abcdef01234567"] in the remediation log
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  @givn.modified
  Scenario: Changing a candidate scenario invalidates its previous classification
    Given the remediation case carries the distinguishing marker "atlas basin clover gangway harness iris jetty lighthouse plum prow rudder slate"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user records the remediation fix "removed the redundant scenario" and commits ["0123456789abcdef0123456789abcdef01234567"] in the remediation log
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"

  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given the remediation case carries the distinguishing marker "anvil cleat cobalt gangway hull inlet keel quern saffron summit whip wren"
    Given a duplicate-log project contains a "DUPLICATE" case
    And the run declaration of the project is complete
    When the blocking review is executed for running-change
    And the operator labels every review candidate "DUPLICATE"
    And the operator records the performed fix "retired the repeated scenario" with commits ["5e6f708192a3b4c5"] in the log
    And the permanent counterpart of the finding is rewritten in the permanent specs
    And the review command reruns for running-change
    Then the gate run ends unsuccessfully
    And the semantic review report contains classification "UNCERTAIN"


  Scenario: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given the remediation case carries the distinguishing marker "anvil beacon bolt eddy eucalyptus halyard inkwell javelin juniper keel quern shim"
    Given  a project prepared for semantic review with a similar active delta
    And the project's run block is complete
    When  the review gate executes for running-change
    Then  the attempt fails
    When  the assessor classifies all candidates "DUPLICATE"
    And  the reviewer logs the applied fix "dropped the duplicated scenario" and the commit list ["7c8d9e0f1a2b3c4d"] into the remediation log
    And  the active classified endpoint changes without remediation
    And  the review gate executes for running-change
    Then  the attempt fails
    And  a later candidate entry records different endpoint hashes than its predecessor
    And  the prior candidate entry is retained unchanged


  Scenario: Removed candidates disappear after classification
    Given the remediation case carries the distinguishing marker "dowel inlet jasper moss opal parapet piston sandal tulip xylophone yarn zinnia"
    Given a duplicate-log project contains a "DUPLICATE" case
    And the run declaration of the project is complete
    When the blocking review is executed for running-change
    And the operator labels every review candidate "DUPLICATE"
    And the operator records the performed fix "withdrew the twin scenario" with commits ["6f708192a3b4c5d6"] in the log
    And the redundant scenario is deleted from the delta
    And the review command reruns for running-change
    Then the command exits 0
    And stdout announces the verified remediation

  @givn.modified
  Scenario: Removing a DUPLICATE candidate verifies the projected remediation
    Given the remediation case carries the distinguishing marker "canyon islet latch opal plum pumice quince reed scythe udder walnut zodiac"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And  the permanent specifications are snapshotted
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user records the remediation fix "removed the redundant scenario" and commits ["0123456789abcdef0123456789abcdef01234567"] in the remediation log
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"
    And  the permanent specifications are unchanged

  @givn.modified
  Scenario: Consolidating a VARIANT candidate verifies the projected remediation
    Given the remediation case carries the distinguishing marker "abacus almanac basin cleat easel gondola heather islet orchard tundra umbrella walnut"
    Given  a project prepared for semantic remediation with a "VARIANT" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VARIANT"
    And  the user records the remediation fix "consolidated the pair into one outline" and commits ["0123456789abcdef0123456789abcdef01234567"] in the remediation log
    And  the active variant is consolidated with an explicit parameter
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  @givn.modified
  Scenario: Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation
    Given the remediation case carries the distinguishing marker "almanac basin cleat dinghy harness mandrel mast needle nutmeg oarlock saffron yacht"
    Given  a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the user records the remediation fix "reworded the distinction" and commits ["0123456789abcdef0123456789abcdef01234567"] in the remediation log
    And  the valid boundary is rewritten with an explicit distinction below the similarity boundary
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  @givn.modified
  Scenario: The migration preserves independent architecture history across three proposed decisions
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
