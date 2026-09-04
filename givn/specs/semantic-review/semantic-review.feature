Feature: Generate the semantic review report

  Scenario: Change review generates a complete report for a semantic candidate
    Given  a rulebook review project with a single admitted candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report exists
    And  the semantic review report status is "BLOCKED"
    And  the semantic review report candidate count is 1
    And  the candidate embedding score has exactly four decimals in the range "0.9000" through "1.0000"
    And  the semantic review report contains deterministic facts

  Scenario: Removed delta scenarios produce a clean semantic report
    Given  a project prepared for semantic review with only a removed delta
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  the semantic review report exists
    And  the semantic review report status is "PASS"
    And  the semantic review report candidate count is 0
    And  the semantic review report does not mention "Removed scenario"
    And  the permanent index remains structurally permanent-only

  Scenario: The report distinguishes permanent and current-change candidate scopes
    Given  a rulebook review project with two similar active deltas
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains scope "delta-to-permanent"
    And  the semantic review report contains scope "delta-to-delta"
    And  all candidate scores have exactly four decimals and are at least "0.9000"

  Scenario: Active deltas below the E5 boundary produce no candidates
    Given  a project prepared for semantic review with active deltas below the similarity boundary
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  the semantic review report exists
    And  the semantic review report status is "PASS"
    And  the semantic review report candidate count is 0
    And  the report contains no permanent-permanent candidate

  Scenario: A candidate at the exact E5 boundary is included
    Given  a project prepared for semantic review with a candidate scored exactly at the similarity boundary
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains a candidate
    And  the candidate embedding score is exactly "0.9000"
    And  the candidate id is exactly 16 lowercase hexadecimal characters
    And  the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters

  Scenario: Rule-nested active deltas enter the semantic candidate universe
    Given  a project prepared for semantic review with a Rule-nested active delta
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains the exact Rule-nested endpoint title "Rule nested rejects an unknown provider"
    And  the semantic review report contains scope "delta-to-permanent"
    And  the semantic review report contains non-empty content and evidence hashes

  Scenario: Endpoint evidence preserves complete Gherkin source order
    Given  a project prepared for semantic review with feature and Rule backgrounds, step docstrings, and data tables
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  endpoint evidence lists feature Background steps before Rule Background steps and scenario steps
    And  endpoint evidence preserves every step docstring and data-table row and cell in source order
    And  endpoint evidence serializes strings, arrays, and table cells with JSON escaping
    And  the E5 embedding text uses its existing passage representation separately from endpoint evidence

  Scenario: An uncertain semantic candidate blocks change review without editing specs
    Given  a project prepared for semantic review with a similar active delta
    And  the permanent specifications are snapshotted
    And  the active feature files, review dispositions, and index boundary are snapshotted
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "UNCERTAIN"
    And  the active feature files are unchanged
    And  the permanent specifications are unchanged
    And  the review dispositions are unchanged
    And  the permanent index structural snapshot still contains only permanent entries
    And  the permanent index contains no delta vectors, classifications, or report state

  Scenario: Every resolved semantic classification requires a rationale
    Given  a project prepared for semantic review with a similar active delta
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE" without a rationale
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "rationale"

  Scenario: The semantic report obeys its strict wire grammar
    Given  a project prepared for semantic review with a similar active delta
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report has the exact headers in the defined order
    And  every candidate field appears exactly once in the defined field order
    And  the semantic review report contains only allowed fields and no unknown fields
    And  absent scalar and nested values use exactly "null"
    And  strings, arrays, step records, and table cells use compact JSON serialization
    And  candidate ids, content hashes, and evidence hashes match exactly 16 lowercase hexadecimal characters
    And  the semantic review report ends with exactly one final newline
    And  the semantic review report has no timestamp or environment fields

  Scenario: Equal-score candidates use deterministic tie ordering
    Given  a project prepared for semantic review with equal-score candidates
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  equal-score candidates are ordered by left endpoint tuple, right endpoint tuple, and candidate id
    And  the candidate order is stable across repeated report generation

  Scenario Outline: Invalid prior report entries never carry a classification
    Given  a project prepared for semantic review with a similar active delta
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the prior semantic report is replaced with a <invalid prior entry>
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the regenerated semantic review report is complete
    And  the semantic review report status is "BLOCKED"
    And  the semantic review report candidate count is recomputed
    And  the semantic review report contains classification "UNCERTAIN"
    And  the regenerated semantic review report has the exact headers and candidate field order
    And  the regenerated semantic review report contains only allowed fields and no unknown fields
    And  the regenerated semantic review report ends with exactly one final newline
    Examples:
      | invalid prior entry |
      | missing classification |
      | invalid classification |
      | duplicate candidate |
      | stale candidate |
      | unknown field |
      | structurally malformed block |

  Scenario: A blocked report is atomically complete before failure
    Given  a project prepared for semantic review with a similar active delta
    And  a malformed prior semantic report
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the regenerated semantic review report is complete according to the strict grammar
    And  the report headers and candidate fields are in the exact defined order
    And  the report contains no unknown fields
    And  absent values are represented only by "null"
    And  the report ends with exactly one final newline
    And  the report has no partial temporary output
    And  stderr contains "UNCERTAIN"

  Scenario: Malformed delta input cannot produce a semantic pass
    Given  a project prepared for semantic review with a malformed delta feature
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "delta feature is not valid Gherkin"
    And  the semantic review report does not exist

  Scenario: A malformed permanent index is rebuilt before semantic review continues
    Given  a project prepared for semantic review with a similar active delta
    And  the permanent index contains malformed or incompatible content
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report exists
    And  the semantic review report status is "BLOCKED"
    And  the semantic review report contains the active candidate with classification "UNCERTAIN"
    And  the permanent index was rebuilt from the permanent specifications
    And  the rebuilt permanent index remains structurally permanent-only
    And  the rebuilt permanent index contains no "Delta" state, delta vectors, classifications, or report state

  Scenario: A generated semantic report survives the existing archive move
    Given  a completed semantic review change with a generated report and archive prerequisites
    And  the active semantic review report bytes are snapshotted
    And the project declares a complete run declaration
    When  the user runs `givn archive --change running-change`
    Then  the command exits 0
    And  `givn/archive/running-change/semantic-review.md` exists
    And  the archived semantic review report bytes equal the active report snapshot
    And  the archived report remains historical evidence rather than a permanent specification

  Scenario: An unavailable retrieval model cannot produce a semantic pass
    Given  a project prepared for semantic review with a similar active delta
    And  the semantic review retrieval model is unavailable
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "retrieval model is unavailable"
    And  the semantic review report does not exist

  Scenario: Blocking review reports retrieval unavailability without its feature
    Given  a project prepared for semantic review with a similar active delta
    When  the user runs change review through a build without spec retrieval
    Then  the command exits non-zero
    And  stderr contains "spec retrieval is unavailable"
    And  the semantic review report does not exist

  Scenario: Advisory spec review does not create the semantic report
    Given  a project prepared for semantic review with a similar active delta
    When  the user runs the advisory semantic review command
    Then  the command exits 0
    And  stdout contains "retrieval advisory"
    And  the semantic review report does not exist
  Scenario: A DUPLICATE classification without removal remains blocked
    Given  the remediation case carries the distinguishing marker "duplicate removal blocking contract alpha crimson lantern meadow orbit quartz river summit velvet walnut xenon yellow zephyr atlas beacon cobalt delta ember"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "DUPLICATE remediation requires removing the redundant scenario"
    And  the semantic review report status is "BLOCKED"

  Scenario: Removing a DUPLICATE candidate verifies the projected remediation
    Given  the remediation fixture stages a "DUPLICATE" case
    And  the permanent specs are snapshotted
    When  the review gate runs
    Then  the command exits non-zero
    When  the resolved duplicate "removed" is recorded ["0a1b2c40"]
    And  the classified duplicate is removed
    And  the review gate reruns
    Then  the review passes with the remediation receipt
    And  the permanent specs are unchanged

  Scenario: A VARIANT classification without consolidation remains blocked
    Given  the remediation case carries the distinguishing marker "variant consolidation blocking contract gamma acorn birch cedar dune elm fern grove harbor iris jade kiln lagoon maple north oak pine quarry rose stone thistle"
    Given  a project prepared for semantic remediation with a "VARIANT" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VARIANT"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VARIANT remediation requires consolidation"
    And  the semantic review report status is "BLOCKED"

  Scenario: A VARIANT score below the boundary without consolidation remains blocked
    Given  the remediation case carries the distinguishing marker "variant score only independent contracts epsilon pebble plume pollen pumpkin raven sandal shell silver spruce stream thunder timber turbine valley vessel winter wren"
    Given  a project prepared for semantic remediation with a "VARIANT" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VARIANT"
    And  the active variant is rewritten below the boundary without consolidation
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VARIANT remediation requires consolidation"

  Scenario: A VALID-BOUNDARY score below the boundary without distinction remains blocked
    Given  the remediation case carries the distinguishing marker "boundary score only missing invariant zeta ash basil cloud coral dahlia echo frost granite honey juniper lemon mist nutmeg olive pepper rain snow tulip"
    Given  a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the valid boundary is rewritten below the boundary without an explicit distinction
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"

  Scenario: A VALID-BOUNDARY classification above the boundary remains blocked
    Given  the remediation case carries the distinguishing marker "boundary above threshold blocking eta arrow bronze comet dragon eclipse feather galaxy helmet ivory jigsaw kingdom lilac magnet nickel ozone pebble rocket"
    Given  a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And  the semantic review report status is "BLOCKED"

  Scenario: A VALID-BOUNDARY score exactly at the boundary remains blocked
    Given  the remediation case carries the distinguishing marker "boundary exact threshold blocking iota almond butter carrot daisy engine fig grape hazel igloo jelly kiwi lemon melon nut olive peach quince raisin"
    Given  a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the valid boundary is rewritten with an explicit distinction at the similarity boundary
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"

  Scenario: Changing a classified endpoint before remediation makes the decision stale
    Given  the remediation case carries the distinguishing marker "stale endpoint identity changed kappa beacon breeze candle drizzle ember flame glow hearth incense kettle lantern moonlight nightfall"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the active classified endpoint changes without remediation
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "semantic classification is stale"

  Scenario: Failed remediation verification does not mutate project evidence
    Given  the remediation case carries the distinguishing marker "mutation safety evidence preservation lambda archive backup checksum durable enclave fidelity guardian harbor immutable journal kernel ledger memory"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And  the permanent specifications are snapshotted
    And  the active feature files, review dispositions, and index boundary are snapshotted
    And the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "DUPLICATE remediation requires removing the redundant scenario"
    And  the active feature files are unchanged
    And  the permanent specifications are unchanged
    And  the review dispositions are unchanged
    And  the permanent index contains no delta vectors, classifications, or report state

  @e2e
  Scenario: Real CLI review reports successful duplicate remediation
    Given  an end-to-end project prepared for a duplicate remediation
    When  the remediation E2E user runs change review for "running-change"
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"
  Scenario: A first change without permanent specs is gated on its own duplicated pair
    Given  the remediation case carries the distinguishing marker "the polar research outpost logs the same geomagnetic surge onto the east-facing chart roll and the west-facing chart roll of the calibration change in a repository that owns no permanent specification history"
    Given  a project prepared for semantic remediation with a delta-to-delta "DUPLICATE" candidate on change "rc-unclassified"
    Given  the remediation notes record that "the calibration bench ran unattended for eleven full seasons, and the east-tower duty officer transcribed the geomagnetic surge reading onto both chart rolls only because the west-tower gauge unit had been disconnected for a recalibration that nobody scheduled; the archive clerk later stamped both copies with the same observation time despite the fact that the two instruments measure orthogonal field components, a discrepancy the shift supervisor attributed to the storm watch rather than to any deliberate duplication"
    And the project declares a complete run declaration
    When  the user runs `givn check review --change rc-unclassified`
    Then  the command exits non-zero
    And  the semantic review report contains scope "delta-to-delta"
    And  the semantic review report status is "BLOCKED"

  Scenario: A VARIANT pairing stays blocked until the ending is consolidated away
    Given  the remediation case carries the distinguishing marker "the coastal lighthouse keeper repeats the identical fog-signal watch entry on the dusk page and the midnight page of the guard log, and the active change has no permanent baseline to draw from"
    Given  a project prepared for semantic remediation with a delta-to-delta "VARIANT" candidate on change "rc-var-blocked"
    Given  the remediation notes record that "the lighthouse keeper copied the fog-signal watch entry from the dusk page onto the midnight page and initialled both before climbing down from the lantern room, using the same lamp count, the same sounding interval, and the same duty code, while the guard log otherwise would have shown an empty midnight watch for the month"
    And the project declares a complete run declaration
    When  the user runs `givn check review --change rc-var-blocked`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VARIANT"
    And  the user runs `givn check review --change rc-var-blocked`
    Then  the command exits non-zero
    And  stderr contains "VARIANT remediation requires consolidation"
    And  the semantic review report status is "BLOCKED"

  Scenario: A boundary pairing without rewording still fails review on the first change
    Given  the remediation case carries the distinguishing marker "the tide-driven mill records the same sluice gate reading on the upstream rotation table and the downstream rotation table of the gearbox repair change inside a repository that just started its first change"
    Given  a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate on change "rc-boundary-blocked"
    Given  the remediation notes record that "the millwright recorded the sluice gate reading on the upstream rotation table, then matched the identical reading onto the downstream rotation table after the gearbox repair had been signed off by the foreman, copying the same water height, the same bearing temperature, and the same rotation speed onto both tables without any note explaining why one station measured twice"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change rc-boundary-blocked`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the reviewer logs the fix "kept the boundary pairing" and the commits ["2c3d4e5f"]
    And  the user runs `givn check review --change rc-boundary-blocked`
    Then  the command exits non-zero
    And  stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And  the semantic review report status is "BLOCKED"

  Scenario: An explicit distinction below the boundary lets the first change pass
    Given  the remediation case carries the distinguishing marker "the glacier mapping crew writes the same crevasse-stability fix into the inner traverse notebook and the outer traverse notebook of the survey change"
    Given  a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate on change "rc-boundary-sharpened"
    Given  the remediation notes record that "the glacier mapping crew wrote the crevasse-stability value in the inner traverse notebook, then transcribed the same number and the same ice-thickness estimate into the outer notebook on the morning of departure, keeping the recorded figures indistinguishable"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change rc-boundary-sharpened`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the delta-to-delta valid boundary is sharpened with an explicit distinction below the similarity boundary
    And  the reviewer logs the fix "sharpened the distinction" and the commits ["3d4e5f60"]
    And  the user runs `givn check review --change rc-boundary-sharpened`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"
    And  the semantic review report candidate count is 0

  @e2e
  Scenario: Real CLI review verifies a duplicate remediated within the first change
    Given  the remediation case carries the distinguishing marker "the chamber choir binder books the same hymn arrangement onto the alto practice sheet and the tenor practice sheet ahead of the season's first rehearsal change"
    Given  an e2e project staged for a delta-to-delta duplicate remediation on change "rc-e2e"
    Given  the remediation notes record that "the choir binder transcribed the hymn arrangement onto the alto practice sheet first, then set the same notes onto the tenor practice sheet while the copy room printer jammed, so the two sheets ended up carrying the same bar numbers, the same tempo marking, and the same rehearsal repeats ahead of the season's first performance"
    When  the remediation E2E user runs change review for "rc-e2e"
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"
  Scenario: Superseded permanent endpoints leave the semantic pair universe
    Given  a project prepared for semantic review with a modified scenario and another candidate in the same target file
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the report excludes the modified scenario's own projected target endpoint
    And  the report still contains the other candidate in that target file
    And  the report records the modified delta operation and projected target identity
    And  the report pairs no live endpoint against a superseded permanent body
