# User Interaction Inventory:
# - run blocking change review with givn check review
# - archive a completed change with givn archive

@givn.delta @semantic-remediation-log
Feature: Remediation log for semantic review

  @givn.added
  Scenario: The first blocked review creates the remediation log with before evidence
    Given the remediation case carries the distinguishing marker "the northern telescope array logs its calibration drift onto a waxed Mylar strip while the night operator transcribes the same numbers onto the watch roster because the margin of the strip is too narrow for the extra column, filed as log case 1 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the harbor dredger records its hourly soundings twice, once in the brass logbook and once on the chalkboard in the wheelhouse, because the chief engineer distrusts ink in salt air and wipes the board before anyone can compare the two, filed as log case 2 of the remediation record"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    And the user runs `givn check review --change running-change` again without edits
    Then the remediation log still contains exactly one candidate entry and one run record

  Scenario: A changed endpoint appends a new candidate entry and retains the prior entry
    Given the remediation case carries the distinguishing marker "the bone china workshop tracks each kiln firing on a paper tally hung by the door and in the workshop ledger, and the tally is discarded at month end while the ledger keeps only the fired-not-broken count, filed as log case 24 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the monastery scribe copies the harvest ledger into two illuminated volumes on the same afternoon, and the cellar master later initials both copies without noticing that the barley figures differ by an entire wagonload, filed as log case 4 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the subway control room prints the track-circuit status onto two synchronized roll printers, and when one ribbon jams the operator reads from the faded printout and assumes the track still shows the older occupation, filed as log case 5 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the printing cooperative's paper stock is counted by the storekeeper on a clipboard and entered by the accountant into the ledger, and the accountant's count always reflects the invoice rather than the shelf, filed as log case 28 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the canal lock keeper records boat passages in the toll book and on the tally board mounted at the lock gate, and boaters quote the board while the toll book is what the waterway office actually invoices from, filed as log case 31 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the observatory's dome rotation counter feeds both the guide display and the archival strip chart, and after a power blink the strip chart continues from zero while the guide display keeps its original count, filed as log case 8 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the dairy cooperative weighs each churn twice, once on the platform scale and once on the dock scale, and the clerk copies the heavier reading into the payment ledger believing that the higher figure is the truer one, filed as log case 9 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the mountain rescue post logs stretcher deployments on a laminated card and in the regional incident system, and the laminated card is the only one updated during a power outage that spans two callouts, filed as log case 27 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the vineyard's frost-warning candles are counted by two cellar hands at dusk and at dawn, and each writes the count on a different barn door because neither trusts the other's arithmetic or handwriting, filed as log case 11 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the railway signal box records the train register number in the tallies book and onto the illuminated panel store, and the signalman clears the panel at shift end without reconciling it against the book, filed as log case 12 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the survey ship's sounding fixes are printed on the bridge and transmitted to the hydrographic office, and the office returns corrected tide values that are never copied back onto the bridge printout, filed as log case 25 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the foundry records each pour's temperature on a reusable slate and in the quench book, and the slate is wiped for the next heat before the book's figures have been checked against the pyrometer tape, filed as log case 14 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the glacier hut keeper writes the snow depth onto the window ledger every morning and onto the national survey card every Friday, and the survey card shows Friday's reading for every day of the week, filed as log case 15 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the bakery's proofing room humidity is noted on a chalk dial by the night baker and dictated to the day baker over the telephone, and the dictation drops the decimal point without anyone noticing for a week, filed as log case 16 of the remediation record"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate and BGE and NLI evidence
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log records the candidate's BGE and NLI evidence states as non-null values from the report

  @givn.added
  Scenario: A commit fixing two candidates carries one stamp line per candidate
    Given the remediation case carries the distinguishing marker "the lighthouse supply tender logs its fuel remaining in the engineer's notebook and on the harbor master's wall chart, and the two figures drift apart because the notebook is updated at sea and the chart only in port, filed as log case 17 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the observatory canteen's key board and the site security log each record who took the dome keys, and the board is wiped nightly while the log keeps entries long after the locks themselves are replaced, filed as log case 26 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the wind farm's turbine fault codes appear on the technician's handheld and in the central SCADA journal, and a cached handheld entry is uploaded twice after a dead zone, doubling one fault and erasing its neighbor, filed as log case 19 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the cathedral organ builder keeps the voicing notes in a leather folio and on loose cards pinned beside the console, and the cards are reordered after a recital so the folio and the pins no longer agree, filed as log case 20 of the remediation record"
    Given a project prepared for semantic review with a malformed delta feature
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the remediation log does not exist

  @givn.added
  Scenario: The remediation log survives the archive move
    Given the remediation case carries the distinguishing marker "the apiary's queen sightings are recorded by the visiting inspector on a government form and by the beekeeper on a kitchen calendar, and the two dates differ whenever an inspection is rescheduled after rain, filed as log case 21 of the remediation record"
    Given a completed semantic remediation change with a generated log and archive prerequisites
    And the active remediation log bytes are snapshotted
    And the project declares a complete run declaration
    When the user runs `givn archive --change running-change`
    Then the command exits 0
    And `givn/archive/running-change/semantic-remediation.md` exists
    And the archived remediation log bytes equal the active log snapshot

  @givn.added
  Scenario: Advisory spec review does not create the remediation log
    Given the remediation case carries the distinguishing marker "the tram depot's wheel wear measurements go into the maintenance database and onto a grease-pencil column on the pit wall, and the pencil column survives a database migration that truncates the older readings, filed as log case 22 of the remediation record"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs the advisory semantic review command
    Then the command exits 0
    And stdout contains "retrieval advisory"
    And the remediation log does not exist

  @givn.added
  Scenario: Blocking review without its retrieval feature does not create the remediation log
    Given the remediation case carries the distinguishing marker "the desalination plant's shift supervisor forwards the membrane pressure values by email and reads them aloud on the radio check, and the radio copy includes a sensor that the email template omits entirely, filed as log case 23 of the remediation record"
    Given a project prepared for semantic remediation with a "DUPLICATE" candidate
    When the user runs change review through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
    And the remediation log does not exist

  @givn.added @e2e
  Scenario: Real CLI review completes the remediation log for a stamped duplicate remediation
    Given the remediation case carries the distinguishing marker "the bone china workshop tracks each kiln firing on a paper tally hung by the door and in the workshop ledger, and the tally is discarded at month end while the ledger keeps only the fired-not-broken count, filed as log case 24 of the remediation record"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-e2e"
    When the remediation E2E user runs change review for "rc-log-e2e" after classifying, removing, and stamp-committing the duplicate
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the remediation log records the before and after run records with stage durations and the stamped commit

  @givn.added @e2e
  Scenario: Real CLI archive retains the remediation log
    Given the remediation case carries the distinguishing marker "the survey ship's sounding fixes are printed on the bridge and transmitted to the hydrographic office, and the office returns corrected tide values that are never copied back onto the bridge printout, filed as log case 25 of the remediation record"
    Given an end-to-end project staged for a stamped remediation log on change "rc-log-archive-e2e"
    When the remediation E2E user archives change "rc-log-archive-e2e" after a verified stamped remediation
    Then the command exits 0
    And the archived remediation log exists next to the archived semantic report

  @givn.modified
  Scenario: A complete classification allows change review and survives regeneration
    Given the remediation case carries the distinguishing marker "the observatory canteen's key board and the site security log each record who took the dome keys, and the board is wiped nightly while the log keeps entries long after the locks themselves are replaced, filed as log case 26 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the mountain rescue post logs stretcher deployments on a laminated card and in the regional incident system, and the laminated card is the only one updated during a power outage that spans two callouts, filed as log case 27 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the printing cooperative's paper stock is counted by the storekeeper on a clipboard and entered by the accountant into the ledger, and the accountant's count always reflects the invoice rather than the shelf, filed as log case 28 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the desalination plant's shift supervisor forwards the membrane pressure values by email and reads them aloud on the radio check, and the radio copy includes a sensor that the email template omits entirely, filed as log case 23 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the vineyard's frost-warning candles are counted by two cellar hands at dusk and at dawn, and each writes the count on a different barn door because neither trusts the other's arithmetic or handwriting, filed as log case 11 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the wind farm's turbine fault codes appear on the technician's handheld and in the central SCADA journal, and a cached handheld entry is uploaded twice after a dead zone, doubling one fault and erasing its neighbor, filed as log case 19 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the planetarium's show counter ticks once on the projector housing and once in the front desk journal, and school groups counted by the desk include teachers whom the projector counter never sees, filed as log case 32 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the seed library's lending folder and the branch library system each record borrowed seed packets, and returns checked into the folder are never scanned back into the system until the annual audit, filed as log case 33 of the remediation record"
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
    Given the remediation case carries the distinguishing marker "the harbor pilot's movement book and the tug company's billing journal record the same ship arrivals, and a canceled booking is crossed out in one but entered as completed in the other, filed as log case 34 of the remediation record"
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
