@givn.delta @spec-progress-output
Feature: Progress output during long-running spec retrieval commands

  @givn.removed
  Scenario: A cold-cache index build logs every phase of the E5 model download and scenario processing

  @givn.removed
  Scenario: A warm-cache index build stays free of model-download wording

  @givn.removed
  Scenario: A failed model download still logs the started download phase before failing

  @givn.removed
  Scenario: The JSON duplicate audit keeps the report channel free of progress wording

  @givn.removed
  Scenario: Disabled evidence layers leave the audit channel free of second-stage download wording

  @givn.removed
  Scenario: The review run logs the reranker and NLI downloads as they happen

  @givn.removed
  Scenario: A three-scenario corpus logs the pending count and one progress line per scenario slot

  @givn.removed
  Scenario: Searching a cold E5 cache logs the model download before printing ranked results

  @givn.added @e2e
  Scenario: The JSON duplicate audit is valid JSON without progress wording
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus with `givn spec duplicates --format json`
    Then   the command exits 0
    And    stdout is a JSON array
    And    stdout stays free of "embedding" and "download" wording

  @givn.added
  Scenario: Repeated audits emit stable output without progress lines
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus twice with `givn spec duplicates`
    Then   the command exits 0
    And    both audit reports are identical

  @givn.added
  Scenario: The text audit keeps model wording off the report channel
    Given  a spec discovery project whose corpus contains one feature with two identical scenarios
    When   the user audits the permanent corpus with `givn spec duplicates`
    Then   the command exits 0
    And    stdout contains "scanned permanent corpus"
    And    stdout stays free of model wording
