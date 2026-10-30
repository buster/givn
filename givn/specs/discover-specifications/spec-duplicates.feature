Feature: Audit duplicate and related permanent scenarios

  @e2e @spec-duplicates
  Scenario: Exact duplicates in one feature are reported together
    Given  a spec discovery project whose corpus contains one feature with two scenarios having literally equal supported executable content
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "exact_duplicate"
    And  stdout contains "scanned permanent corpus"
    And  stdout contains both audit endpoints
    And  the terminal transcript is captured for this scenario

  @spec-duplicates
  Scenario: Exact duplicates across features are reported together
    Given  a spec discovery project whose corpus contains two features with one scenario each having literally equal supported executable content
    And  the scenarios have different titles and file paths
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "exact_duplicate"
    And  both distinct titles and file paths identify the match endpoints

  @spec-duplicates
  Scenario: Same assertion shape without exact behavior is reported as a structural match
    Given  a spec discovery project whose corpus contains two scenarios with identical assertions but different actions
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  stdout contains "same_assertion_shape"
    And  the pair is not labelled "exact_duplicate"
    And  the report describes the structural match as advisory rather than proof of equivalent behavior

  Scenario: A structural subset is reported as an overlap
    Given a spec discovery project whose corpus contains one scenario whose assertions are a subset of another's
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "assertion_subset"

  Scenario: JSON output lists every match with relation and endpoints
    Given a spec discovery project whose corpus contains one feature with two identical scenarios
    When the user audits the permanent corpus with `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is a JSON array
    And every JSON match carries kind left_file left_title right_file and right_title

  Scenario: A malformed permanent feature reports its path and fails the audit
    Given a spec discovery project whose corpus contains a malformed permanent feature
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits non-zero
    And stderr contains "failed to parse"

  Scenario: A missing permanent specifications directory is an empty corpus
    Given a spec discovery project with no permanent specifications directory
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "0 exact/structural match(es)"

  Scenario: Archived scenarios are excluded from the audit
    Given a spec discovery project whose archive contains a duplicate pair and whose corpus has none
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "0 exact/structural match(es)"

  @spec-duplicates
  Scenario: Repeating an unchanged audit produces the same report
    Given  a spec discovery project with several exact and structural matches across different files and titles
    When  the user audits the unchanged corpus twice in text format and twice in JSON format
    Then  the command exits 0
    And  both text audit reports are identical
    And  both JSON audit reports are identical
    And  match endpoints and match ordering do not depend on filesystem enumeration order

  Scenario: Auditing leaves permanent specifications unchanged
    Given a spec discovery project whose corpus contains one feature with two identical scenarios
    And the permanent specifications are snapshotted
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And the permanent specifications are unchanged

  Scenario: The deterministic audit works without model downloads
    Given a spec discovery project whose corpus contains one feature with two identical scenarios
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "scanned permanent corpus"
    And stdout stays free of "embedding" and "download" wording

  @spec-duplicates
  Scenario Outline: Normalized executable differences do not establish literal equality
    Given  two permanent scenarios whose normalized structural shape matches
    And  their supported executable content differs only in <difference>
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  the pair is reported as a structural match
    And  the pair is not labelled "exact_duplicate"
    Examples:
      | difference |
      | a quoted step parameter |
      | a numeric step parameter |
      | a data-table argument cell |
      | a docstring argument |
      | an Examples value |

  @spec-duplicates
  Scenario: Different inherited setup prevents an exact duplicate label
    Given  two permanent scenarios with literally equal steps and step arguments
    And  their supported Background setup differs
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  the pair is not labelled "exact_duplicate"

  @spec-duplicates
  Scenario: Equal supported executable arguments remain exact duplicates
    Given  two permanent Scenario Outlines with different titles and file paths
    And  their supported Backgrounds, step keywords, step text, tables, docstrings, and Examples are literally equal
    When  the user audits the permanent corpus with `givn spec duplicates`
    Then  the command exits 0
    And  the pair is labelled "exact_duplicate"
