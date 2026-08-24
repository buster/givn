# User Interaction Inventory:
# - build the permanent semantic index through the givn CLI
# - audit permanent scenarios with JSON output through the givn CLI
# - run the advisory semantic review of an active change through the givn CLI
# - search the permanent semantic index through the givn CLI

@givn.delta @spec-progress-output
Feature: Progress output during long-running spec retrieval commands

  @givn.added @e2e
  Scenario: A cold-cache index build logs every phase of the E5 model download and scenario processing
    Given an initialized project with a fresh XDG Givn cache and a reachable model source
    And the project contains fitting permanent semantic scenarios
    When the end-to-end user builds the permanent semantic index with `givn spec index`
    Then the command exits 0
    And stdout reports a completed permanent semantic index
    And the index build logs the start of the E5 model download into the cache directory
    And the index build logs byte progress for each E5 model file
    And the index build logs the end of the E5 model download
    And the index build logs embedder preparation before scenario processing
    And the index build logs the count of scenarios awaiting embedding
    And the index build logs the end of scenario processing

  @givn.added
  Scenario: A warm-cache index build stays free of model-download wording
    Given an initialized project with a completed E5 model cache in the XDG Givn cache and permanent semantic scenarios
    When the user runs `givn spec index`
    Then the command exits 0
    And stdout reports a completed permanent semantic index
    And the warm-cache run stays free of model-download wording
    And the warm-cache run stays free of per-file byte progress

  @givn.added
  Scenario: A failed model download still logs the started download phase before failing
    Given an initialized project with unreachable semantic model sources
    When the user runs `givn spec index`
    Then the command exits non-zero
    And the failed run still logs the start of the E5 model download into the cache directory
    And the failure names the E5 model file

  @givn.added @nli-combined @e2e
  Scenario: The JSON duplicate audit keeps the report channel free of progress wording
    Given fresh BGE and NLI caches in the XDG Givn cache and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly enabled for automatic acquisition
    When the end-to-end user audits permanent scenarios with `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON
    And the audit emits the start of the reranker download
    And the audit emits the start of the NLI download
    And the audit emits byte progress for the reranker and NLI model files

  @givn.added
  Scenario: Disabled evidence layers leave the audit channel free of second-stage download wording
    Given an initialized project with fresh second-stage caches in the XDG Givn cache and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly disabled
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON
    And the audit channel stays free of reranker and NLI download wording

  @givn.added @nli-combined @e2e
  Scenario: The review run logs the reranker and NLI downloads as they happen
    Given an initialized project with a completed permanent semantic index and a cold second-stage model cache
    And an active change delta is available for advisory review
    And BGE and NLI evidence are explicitly enabled
    When the end-to-end user runs advisory review for the active change through the givn CLI
    Then the command exits 0
    And stdout reports BGE evidence status
    And the review run logs the start of the reranker download
    And the review run logs the start of the NLI download
    And the review run logs byte progress for the reranker and NLI model files

  @givn.added
  Scenario: A three-scenario corpus logs the pending count and one progress line per scenario slot
    Given an initialized project with a completed E5 model cache in the XDG Givn cache and three permanent semantic scenarios
    When the user runs `givn spec index`
    Then the command exits 0
    And the index build logs three scenarios awaiting embedding
    And the index build logs one progress line per scenario slot

  @givn.added @e2e
  Scenario: Searching a cold E5 cache logs the model download before printing ranked results
    Given an initialized project with a completed permanent semantic index and a cold E5 model cache
    And a reachable model source
    When the end-to-end user searches for a permanent semantic scenario with model acquisition through the givn CLI
    Then the command exits 0
    And the search logs the start of the E5 model download into the cache directory
    And the search prints ranked results on stdout