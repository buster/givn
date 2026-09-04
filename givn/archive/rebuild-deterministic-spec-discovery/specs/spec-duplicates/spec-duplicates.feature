@givn.delta @spec-duplicates
Feature: Audit duplicate and related permanent scenarios

  @givn.removed
  Scenario: Maintainer inspects the complete permanent scenario audit

  @givn.removed
  Scenario: Exact duplicates in one feature are reported together

  @givn.removed
  Scenario: Exact duplicates across features are reported together

  @givn.removed
  Scenario: Structurally overlapping scenarios retain their distinguishing facts

  @givn.removed
  Scenario: A structural subset is reported as an overlap

  @givn.removed
  Scenario: Semantically related scenarios are reported as candidates

  @givn.removed
  Scenario: The audit reports candidates beyond the interactive top five

  @givn.removed
  Scenario: Repeating an unchanged audit produces the same report

  @givn.removed
  Scenario: JSON output identifies every audit candidate

  @givn.removed
  Scenario: Auditing leaves permanent specifications unchanged

  @givn.removed
  Scenario: Auditing leaves the persistent retrieval index unchanged

  @givn.removed
  Scenario: Active change scenarios are not part of the permanent audit

  @givn.removed
  Scenario: An audit with no candidates reports an empty result

  @givn.removed
  Scenario: The audit reports retrieval unavailable without the retrieval feature

  @givn.removed
  Scenario: The audit reports an unavailable retrieval model

  @givn.removed
  Scenario: A malformed permanent feature reports its path

  @givn.removed
  Scenario: An unreadable permanent feature reports its path

  @givn.removed
  Scenario: A missing permanent specifications directory is an empty corpus

  @givn.removed
  Scenario: Archived scenarios are excluded from the audit

  @givn.removed
  Scenario: Auditing leaves active changes and archive contents unchanged

  @givn.removed
  Scenario: An absent persistent index is ignored and remains absent

  @givn.removed
  Scenario: A malformed persistent index is ignored and unchanged

  @givn.removed
  Scenario: A semantic candidate at the inclusive threshold is reported

  @givn.removed
  Scenario: A semantic pair below the threshold is not reported

  @givn.removed
  Scenario: Candidate classes take precedence and equal semantic scores use endpoint order

  @givn.removed
  Scenario: Rule-nested permanent scenarios are included in the audit

  @givn.removed
  Scenario: A blocked permanent specifications path reports its I/O error

  @givn.removed
  Scenario: Removed permanent scenarios are excluded from the audit

  @givn.removed
  Scenario: Structural relations are ordered by relation class

  @givn.removed
  Scenario: Equal semantic scores use right endpoint order

  @givn.removed
  Scenario: Invalid duplicate score overrides are rejected

  @givn.removed
  Scenario: Reversed score override endpoint pairs normalize canonically

  @givn.removed
  Scenario: Empty assertions and reverse subsets retain their distinct relations

  @givn.removed
  Scenario: Semantic quality evidence is measured from labeled stub scores

  @givn.added @e2e
  Scenario: Exact duplicates in one feature are reported together
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "exact_duplicate"
    And    stdout contains "scanned permanent corpus"
    And    stdout contains the audit endpoint titles "The thing happens" and "The thing happens"

  @givn.added @e2e
  Scenario: Exact duplicates across features are reported together
    Given  a spec discovery project whose corpus contains two features with one identical scenario each
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "exact_duplicate"

  @givn.added
  Scenario: Same assertion shape without exact behavior is reported as a structural match
    Given  a spec discovery project whose corpus contains two scenarios with identical assertions but different actions
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "same_assertion_shape"

  @givn.added
  Scenario: A structural subset is reported as an overlap
    Given  a spec discovery project whose corpus contains one scenario whose assertions are a subset of another's
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "assertion_subset"

  @givn.added
  Scenario: JSON output lists every match with relation and endpoints
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus with `givn spec duplicates --format json`
    Then   the command exits 0
    And    stdout is a JSON array
    And    every JSON match carries kind left_file left_title right_file and right_title

  @givn.added
  Scenario: A malformed permanent feature reports its path and fails the audit
    Given  a spec discovery project whose corpus contains a malformed permanent feature
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits non-zero
    And    stderr contains "failed to parse"

  @givn.added
  Scenario: A missing permanent specifications directory is an empty corpus
    Given  a spec discovery project with no permanent specifications directory
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "0 exact/structural match(es)"

  @givn.added
  Scenario: Archived scenarios are excluded from the audit
    Given  a spec discovery project whose archive contains a duplicate pair and whose corpus has none
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "0 exact/structural match(es)"

  @givn.added
  Scenario: Repeating an unchanged audit produces the same report
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus twice with `givn spec duplicates`
    Then   the command exits 0
    And    both audit reports are identical

  @givn.added
  Scenario: Auditing leaves permanent specifications unchanged
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    And    the permanent specifications are snapshotted
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    the permanent specifications are unchanged

  @givn.added
  Scenario: The deterministic audit works without model downloads
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "scanned permanent corpus"
    And    stdout stays free of "embedding" and "download" wording
