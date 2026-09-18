Feature: keep-audit-output-clean

  @e2e
  Scenario: The JSON duplicate audit is valid JSON without progress wording
    Given a spec discovery project whose corpus contains one feature with two identical scenarios
    When the user audits the permanent corpus with `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is a JSON array
    And stdout stays free of "embedding" and "download" wording

  Scenario: Repeated audits emit stable output without progress lines
    Given a spec discovery project whose corpus contains one feature with two identical scenarios
    When the user audits the permanent corpus twice with `givn spec duplicates`
    Then the command exits 0
    And both audit reports are identical

  Scenario: The text audit keeps model wording off the report channel
    Given a spec discovery project whose corpus contains one feature with two identical scenarios
    When the user audits the permanent corpus with `givn spec duplicates`
    Then the command exits 0
    And stdout contains "scanned permanent corpus"
    And stdout stays free of model wording
