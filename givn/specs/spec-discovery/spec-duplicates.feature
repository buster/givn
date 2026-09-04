Feature: Audit duplicate and related permanent scenarios

  @e2e
  Scenario: Exact duplicates in one feature are reported together
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "exact_duplicate"
    And  stdout contains "scanned permanent corpus"
    And  stdout contains the audit endpoint titles "The thing happens" and "The thing happens"

  @e2e
  Scenario: Exact duplicates across features are reported together
    Given  a spec discovery project whose corpus contains two features with one identical scenario each
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "exact_duplicate"

  Scenario: Same assertion shape without exact behavior is reported as a structural match
    Given  a spec discovery project whose corpus contains two scenarios with identical assertions but different actions
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "same_assertion_shape"

  Scenario: A structural subset is reported as an overlap
    Given  a spec discovery project whose corpus contains one scenario whose assertions are a subset of another's
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "assertion_subset"

  Scenario: JSON output lists every match with relation and endpoints
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When  the user audits the permanent corpus with `givn spec duplicates --format json`
    Then  the command exits 0
    And  stdout is a JSON array
    And  every JSON match carries kind left_file left_title right_file and right_title

  Scenario: A malformed permanent feature reports its path and fails the audit
    Given  a spec discovery project whose corpus contains a malformed permanent feature
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits non-zero
    And  stderr contains "failed to parse"

  Scenario: A missing permanent specifications directory is an empty corpus
    Given  a spec discovery project with no permanent specifications directory
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "0 exact/structural match(es)"

  Scenario: Archived scenarios are excluded from the audit
    Given  a spec discovery project whose archive contains a duplicate pair and whose corpus has none
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "0 exact/structural match(es)"

  Scenario: Repeating an unchanged audit produces the same report
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When  the user audits the permanent corpus twice with `givn spec duplicates`
    Then  the command exits 0
    And  both audit reports are identical

  Scenario: Auditing leaves permanent specifications unchanged
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    And  the permanent specifications are snapshotted
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  the permanent specifications are unchanged

  Scenario: The deterministic audit works without model downloads
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "scanned permanent corpus"
    And  stdout stays free of "embedding" and "download" wording
