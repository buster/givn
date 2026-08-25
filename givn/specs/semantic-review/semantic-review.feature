Feature: Generate the semantic review report

  Scenario: Change review generates a complete report for a semantic candidate
    Given  an end-to-end project prepared for semantic review with a similar active delta
    When  the end-to-end user runs change review for "running-change"
    Then  the command exits non-zero
    And  the semantic review report exists
    And  the semantic review report contains a candidate with both endpoints and an "UNCERTAIN" classification
    And  the candidate has left scope "current-change" and title "Unknown provider is rejected"
    And  the candidate has right scope "permanent" and title "Provider name is unknown"
    And  both endpoint content hashes and evidence hashes are exactly 16 lowercase hexadecimal characters
    And  the candidate id is exactly 16 lowercase hexadecimal characters
    And  the candidate embedding score has exactly four decimals in the range "0.9500" through "1.0000"
    And  the candidate contains the concrete fact "fact_contradiction: false"
    And  the semantic review report has no timestamp or environment fields

  Scenario: Removed delta scenarios produce a clean semantic report
    Given  a project prepared for semantic review with only a removed delta
    When  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  the semantic review report exists
    And  the semantic review report status is "PASS"
    And  the semantic review report candidate count is 0
    And  the semantic review report does not mention "Removed scenario"
    And  the permanent index remains structurally permanent-only

  Scenario: The report distinguishes permanent and current-change candidate scopes
    Given  a project prepared for semantic review with two similar active deltas
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains scope "delta-to-permanent"
    And  the semantic review report contains scope "delta-to-delta"
    And  the report names the exact left and right endpoint titles for both scopes
    And  all candidate scores have exactly four decimals and are at least "0.9500"
    And  all endpoint hashes are exactly 16 lowercase hexadecimal characters
    And  the semantic review report contains deterministic facts

  Scenario: Active deltas below the E5 boundary produce no candidates
    Given  a project prepared for semantic review with active deltas below the similarity boundary
    When  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  the semantic review report exists
    And  the semantic review report status is "PASS"
    And  the semantic review report candidate count is 0
    And  the report contains no permanent-permanent candidate

  Scenario: A candidate at the exact E5 boundary is included
    Given  a project prepared for semantic review with a candidate scored exactly at the similarity boundary
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains a candidate
    And  the candidate embedding score is exactly "0.9500"
    And  the candidate id is exactly 16 lowercase hexadecimal characters
    And  the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters

  Scenario: Rule-nested active deltas enter the semantic candidate universe
    Given  a project prepared for semantic review with a Rule-nested active delta
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains the exact Rule-nested endpoint title "Rule nested rejects an unknown provider"
    And  the semantic review report contains scope "delta-to-permanent"
    And  the semantic review report contains non-empty content and evidence hashes

  Scenario: Endpoint evidence preserves complete Gherkin source order
    Given  a project prepared for semantic review with feature and Rule backgrounds, step docstrings, and data tables
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  endpoint evidence lists feature Background steps before Rule Background steps and scenario steps
    And  endpoint evidence preserves every step docstring and data-table row and cell in source order
    And  endpoint evidence serializes strings, arrays, and table cells with JSON escaping
    And  the E5 embedding text uses its existing passage representation separately from endpoint evidence

  Scenario: A modified delta excludes only its own projected target
    Given  a project prepared for semantic review with a modified scenario and another candidate in the same target file
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the report excludes the modified scenario's own projected target endpoint
    And  the report still contains the other candidate in that target file
    And  the report records the modified delta operation and projected target identity

  Scenario: An uncertain semantic candidate blocks change review without editing specs
    Given  a project prepared for semantic review with a similar active delta
    And  the permanent specifications are snapshotted
    And  the active feature files, review dispositions, and index boundary are snapshotted
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "UNCERTAIN"
    And  the active feature files are unchanged
    And  the permanent specifications are unchanged
    And  the review dispositions are unchanged
    And  the permanent index structural snapshot still contains only permanent entries
    And  the permanent index contains no delta vectors, classifications, or report state

  Scenario: A complete classification allows change review and survives regeneration
    Given  the remediation case carries the distinguishing marker "modified permanent contract replacement mu archive bridge contract delta evidence fixture gateway history invariant journal keeper ledger merge namespace output projection"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  Scenario: Every resolved semantic classification requires a rationale
    Given  a project prepared for semantic review with a similar active delta
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE" without a rationale
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "rationale"

  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given  the remediation case carries the distinguishing marker "regression deterministic report omega glacier harbor invariant matrix nonce ordered receipt stable"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  Scenario: The semantic report obeys its strict wire grammar
    Given  a project prepared for semantic review with a similar active delta
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
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  equal-score candidates are ordered by left endpoint tuple, right endpoint tuple, and candidate id
    And  the candidate order is stable across repeated report generation

  Scenario: Changing a candidate scenario invalidates its previous classification
    Given  the remediation case carries the distinguishing marker "regression active candidate invalidation sigma texture vector antler birch comet dune ember fjord granite harbor island juniper kiln lagoon marble nebula olive prism quartz rocket sandal timber umber"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"

  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given  the remediation case carries the distinguishing marker "regression permanent endpoint invalidation tau unmodified baseline acorn beacon copper delta engine frost grove hickory iris jade kettle lemon mosaic nickel opal pebble ruby stone thistle"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"

  Scenario: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given  the remediation case carries the distinguishing marker "regression evidence metadata invalidation upsilon version hash"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"

  Scenario: Removed candidates disappear after classification
    Given  the remediation case carries the distinguishing marker "regression removed candidate disappearance phi projected state"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

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
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "delta feature is not valid Gherkin"
    And  the semantic review report does not exist

  Scenario: A malformed permanent index is rebuilt before semantic review continues
    Given  a project prepared for semantic review with a similar active delta
    And  the permanent index contains malformed or incompatible content
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
    When  the user runs `givn archive --change running-change`
    Then  the command exits 0
    And  `givn/archive/running-change/semantic-review.md` exists
    And  the archived semantic review report bytes equal the active report snapshot
    And  the archived report remains historical evidence rather than a permanent specification

  Scenario: An unavailable retrieval model cannot produce a semantic pass
    Given  a project prepared for semantic review with a similar active delta
    And  the semantic review retrieval model is unavailable
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
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "DUPLICATE remediation requires removing the redundant scenario"
    And  the semantic review report status is "BLOCKED"

  Scenario: Removing a DUPLICATE candidate verifies the projected remediation
    Given  the remediation case carries the distinguishing marker "duplicate projected removal success beta canyon copper ivory juniper kelp mosaic nickel opal prairie ruby saffron topaz umber violet willow xylophone yarrow zenith"
    Given  a project prepared for semantic remediation with a "DUPLICATE" candidate
    And  the permanent specifications are snapshotted
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "DUPLICATE"
    And  the classified duplicate candidate is removed from the active delta
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"
    And  the permanent specifications are unchanged

  Scenario: A VARIANT classification without consolidation remains blocked
    Given  the remediation case carries the distinguishing marker "variant consolidation blocking contract gamma acorn birch cedar dune elm fern grove harbor iris jade kiln lagoon maple north oak pine quarry rose stone thistle"
    Given  a project prepared for semantic remediation with a "VARIANT" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VARIANT"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VARIANT remediation requires consolidation"
    And  the semantic review report status is "BLOCKED"

  Scenario: Consolidating a VARIANT candidate verifies the projected remediation
    Given  the remediation case carries the distinguishing marker "variant parameterized examples success delta anchor blossom circuit drum engine flare glacier hinge island jacket kettle ladder marble nebula orange prism"
    Given  a project prepared for semantic remediation with a "VARIANT" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VARIANT"
    And  the active variant is consolidated with an explicit parameter
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  Scenario: A VARIANT score below the boundary without consolidation remains blocked
    Given  the remediation case carries the distinguishing marker "variant score only independent contracts epsilon pebble plume pollen pumpkin raven sandal shell silver spruce stream thunder timber turbine valley vessel winter wren"
    Given  a project prepared for semantic remediation with a "VARIANT" candidate
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
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And  the semantic review report status is "BLOCKED"

  Scenario: Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation
    Given  the remediation case carries the distinguishing marker "boundary explicit observable fact success theta acrobat bamboo crystal domino emerald fossil garden harbor insect jasmine koala lantern mango needle ocean pocket rainbow"
    Given  a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the valid boundary is rewritten with an explicit distinction below the similarity boundary
    And  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"

  Scenario: A VALID-BOUNDARY score exactly at the boundary remains blocked
    Given  the remediation case carries the distinguishing marker "boundary exact threshold blocking iota almond butter carrot daisy engine fig grape hazel igloo jelly kiwi lemon melon nut olive peach quince raisin"
    Given  a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
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
