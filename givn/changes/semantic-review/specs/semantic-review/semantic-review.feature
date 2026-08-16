# User Interaction Inventory:
# - invoke `givn check review --change <id>` and inspect the generated semantic review report and exit status

@givn.delta @semantic-review
Feature: Generate the semantic review report

  @givn.added @e2e @wip
  Scenario: Change review generates a complete report for a semantic candidate
    Given an end-to-end project prepared for semantic review with a similar active delta
    When the end-to-end user runs change review for "running-change"
    Then the command exits non-zero
    And the semantic review report exists
    And the semantic review report contains a candidate with both endpoints and an "UNCERTAIN" classification
    And the candidate has left scope "current-change" and title "Unknown provider is rejected"
    And the candidate has right scope "permanent" and title "Provider name is unknown"
    And both endpoint content hashes and evidence hashes are exactly 16 lowercase hexadecimal characters
    And the candidate id is exactly 16 lowercase hexadecimal characters
    And the candidate embedding score has exactly four decimals in the range "0.9500" through "1.0000"
    And the candidate contains the concrete fact "fact_contradiction: false"
    And the semantic review report has no timestamp or environment fields

  @givn.added
  Scenario: Removed delta scenarios produce a clean semantic report
    Given a project prepared for semantic review with only a removed delta
    When the user runs `givn check review --change running-change`
    Then the command exits 0
    And the semantic review report exists
    And the semantic review report status is "PASS"
    And the semantic review report candidate count is 0
    And the semantic review report does not mention "Removed scenario"
    And the permanent index remains structurally permanent-only

  @givn.added @wip
  Scenario: The report distinguishes permanent and current-change candidate scopes
    Given a project prepared for semantic review with two similar active deltas
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the semantic review report contains scope "delta-to-permanent"
    And the semantic review report contains scope "delta-to-delta"
    And the report names the exact left and right endpoint titles for both scopes
    And all candidate scores have exactly four decimals and are at least "0.9500"
    And all endpoint hashes are exactly 16 lowercase hexadecimal characters
    And the semantic review report contains deterministic facts

  @givn.added
  Scenario: Active deltas below the E5 boundary produce no candidates
    Given a project prepared for semantic review with active deltas below the similarity boundary
    When the user runs `givn check review --change running-change`
    Then the command exits 0
    And the semantic review report exists
    And the semantic review report status is "PASS"
    And the semantic review report candidate count is 0
    And the report contains no permanent-permanent candidate

  @givn.added
  Scenario: A candidate at the exact E5 boundary is included
    Given a project prepared for semantic review with a candidate scored exactly at the similarity boundary
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the semantic review report contains a candidate
    And the candidate embedding score is exactly "0.9500"
    And the candidate id is exactly 16 lowercase hexadecimal characters
    And the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters

  @givn.added
  Scenario: Rule-nested active deltas enter the semantic candidate universe
    Given a project prepared for semantic review with a Rule-nested active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the semantic review report contains the exact Rule-nested endpoint title "Rule nested rejects an unknown provider"
    And the semantic review report contains scope "delta-to-permanent"
    And the semantic review report contains non-empty content and evidence hashes

  @givn.added
  Scenario: Endpoint evidence preserves complete Gherkin source order
    Given a project prepared for semantic review with feature and Rule backgrounds, step docstrings, and data tables
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And endpoint evidence lists feature Background steps before Rule Background steps and scenario steps
    And endpoint evidence preserves every step docstring and data-table row and cell in source order
    And endpoint evidence serializes strings, arrays, and table cells with JSON escaping
    And the E5 embedding text uses its existing passage representation separately from endpoint evidence

  @givn.added
  Scenario: A modified delta excludes only its own projected target
    Given a project prepared for semantic review with a modified scenario and another candidate in the same target file
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the report excludes the modified scenario's own projected target endpoint
    And the report still contains the other candidate in that target file
    And the report records the modified delta operation and projected target identity

  @givn.added
  Scenario: An uncertain semantic candidate blocks change review without editing specs
    Given a project prepared for semantic review with a similar active delta
    And the permanent specifications are snapshotted
    And the active feature files, review dispositions, and index boundary are snapshotted
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "UNCERTAIN"
    And the active feature files are unchanged
    And the permanent specifications are unchanged
    And the review dispositions are unchanged
    And the permanent index structural snapshot still contains only permanent entries
    And the permanent index contains no delta vectors, classifications, or report state

  @givn.added
  Scenario: A complete classification allows change review and survives regeneration
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the semantic review report status is "PASS"
    And the semantic review report contains classification "DUPLIKAT"

  @givn.added
  Scenario: Every resolved semantic classification requires a rationale
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT" without a rationale
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "rationale"

  @givn.added
  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the semantic review report bytes and candidate order are snapshotted
    When the user runs `givn check review --change running-change` again without edits
    Then the command exits 0
    And the semantic review report bytes equal the snapshot
    And the report candidate order is scope, descending score, left endpoint tuple, right endpoint tuple, candidate id
    And the semantic review report ends with exactly one final newline
    And the semantic review report has no timestamp or environment fields

  @givn.added
  Scenario: The semantic report obeys its strict wire grammar
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the semantic review report has the exact headers in the defined order
    And every candidate field appears exactly once in the defined field order
    And the semantic review report contains only allowed fields and no unknown fields
    And absent scalar and nested values use exactly "null"
    And strings, arrays, step records, and table cells use compact JSON serialization
    And candidate ids, content hashes, and evidence hashes match exactly 16 lowercase hexadecimal characters
    And the semantic review report ends with exactly one final newline
    And the semantic review report has no timestamp or environment fields

  @givn.added @wip
  Scenario: Equal-score candidates use deterministic tie ordering
    Given a project prepared for semantic review with equal-score candidates
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And equal-score candidates are ordered by left endpoint tuple, right endpoint tuple, and candidate id
    And the candidate order is stable across repeated report generation

  @givn.added @wip
  Scenario: Changing a candidate scenario invalidates its previous classification
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    When the active semantic candidate is changed
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the changed endpoint has a different content hash and evidence hash
    And the semantic review report contains classification "UNCERTAIN"

  @givn.added @wip
  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    When the permanent semantic endpoint is changed
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the changed permanent endpoint has a different content hash and evidence hash
    And the semantic review report contains classification "UNCERTAIN"

  @givn.added @wip
  Scenario Outline: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    When <evidence change>
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the specific candidate's regenerated evidence hash differs from the prior report's evidence hash
    And the specific candidate has classification "UNCERTAIN"

    Examples:
      | evidence change |
      | the active endpoint content changes |
      | the active endpoint table contents change |
      | the active endpoint Background changes |
      | the active endpoint delta operation changes |
      | the active endpoint projected target changes |
      | the active endpoint Scenario Outline Examples table changes |
      | the Feature description changes |
      | the Rule description changes |
      | the endpoint tags change |
      | an older model version is recorded in the prior report metadata |
      | an older index version is recorded in the prior report metadata |
      | an older fingerprint schema version is recorded in the prior report metadata |
      | an older evidence schema version is recorded in the prior report metadata |
      | an older report schema version is recorded in the prior report metadata |

  @givn.added @wip
  Scenario: Removed candidates disappear after classification
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    When the classified active semantic delta is removed
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And the semantic review report status is "PASS"
    And the semantic review report candidate count is 0
    And the report contains no stale candidate id or classification

  @givn.added @wip
  Scenario Outline: Invalid prior report entries never carry a classification
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the prior semantic report is replaced with a <invalid prior entry>
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the regenerated semantic review report is complete
    And the semantic review report status is "BLOCKED"
    And the semantic review report candidate count is recomputed
    And the semantic review report contains classification "UNCERTAIN"
    And the regenerated semantic review report has the exact headers and candidate field order
    And the regenerated semantic review report contains only allowed fields and no unknown fields
    And the regenerated semantic review report ends with exactly one final newline

    Examples:
      | invalid prior entry          |
      | missing classification       |
      | invalid classification       |
      | duplicate candidate         |
      | stale candidate             |
      | unknown field               |
      | structurally malformed block |

  @givn.added @wip
  Scenario: A blocked report is atomically complete before failure
    Given a project prepared for semantic review with a similar active delta
    And a malformed prior semantic report
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the regenerated semantic review report is complete according to the strict grammar
    And the report headers and candidate fields are in the exact defined order
    And the report contains no unknown fields
    And absent values are represented only by "null"
    And the report ends with exactly one final newline
    And the report has no partial temporary output
    And stderr contains "UNCERTAIN"

  @givn.added @wip
  Scenario: Malformed delta input cannot produce a semantic pass
    Given a project prepared for semantic review with a malformed delta feature
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "delta feature is not valid Gherkin"
    And the semantic review report does not exist

  @givn.added @wip
  Scenario: A malformed permanent index is rebuilt before semantic review continues
    Given a project prepared for semantic review with a similar active delta
    And the permanent index contains malformed or incompatible content
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the semantic review report exists
    And the semantic review report status is "BLOCKED"
    And the semantic review report contains the active candidate with classification "UNCERTAIN"
    And the permanent index was rebuilt from the permanent specifications
    And the rebuilt permanent index remains structurally permanent-only
    And the rebuilt permanent index contains no "Delta" state, delta vectors, classifications, or report state

  @givn.added @wip
  Scenario: A generated semantic report survives the existing archive move
    Given a completed semantic review change with a generated report and archive prerequisites
    And the active semantic review report bytes are snapshotted
    When the user runs `givn archive --change running-change`
    Then the command exits 0
    And `givn/archive/running-change/semantic-review.md` exists
    And the archived semantic review report bytes equal the active report snapshot
    And the archived report remains historical evidence rather than a permanent specification

  @givn.added @wip
  Scenario: An unavailable retrieval model cannot produce a semantic pass
    Given a project prepared for semantic review with a similar active delta
    And the retrieval model is unavailable
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "retrieval model is unavailable"
    And the semantic review report does not exist

  @givn.added @wip
  Scenario: Blocking review reports retrieval unavailability without its feature
    Given a project prepared for semantic review with a similar active delta
    When the user runs change review through a build without spec retrieval
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
    And the semantic review report does not exist

  @givn.added @wip
  Scenario: Advisory spec review does not create the semantic report
    Given a project prepared for semantic review with a similar active delta
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout contains "retrieval advisory"
    And the semantic review report does not exist
