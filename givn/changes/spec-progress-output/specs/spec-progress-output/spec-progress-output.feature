# User Interaction Inventory:
# - build the permanent semantic index through the givn CLI
# - audit permanent scenarios with JSON output through the givn CLI
# - run the advisory semantic review of an active change through the givn CLI
# - search the permanent semantic index through the givn CLI

@givn.delta @spec-progress-output
Feature: Progress output during long-running spec retrieval commands

  @givn.added @e2e
  Scenario: Cold-cache indexing announces each phase on stderr
    Given an initialized project with a fresh XDG Givn cache and a reachable model source
    And the project contains fitting permanent semantic scenarios
    When the end-to-end user builds the permanent semantic index with `givn spec index`
    Then the command exits 0
    And stdout reports a completed permanent semantic index
    And stderr announces the start of the retrieval model download with the E5 cache location
    And stderr reports per-file download progress for the E5 model artifacts
    And stderr announces the completion of the retrieval model download
    And stderr announces the preparation of the retrieval embedder
    And stderr announces the embedding of the pending scenarios with their count
    And stderr announces the completion of the embedding phase

  @givn.added
  Scenario: A warm model cache produces no download announcements
    Given an initialized project with a completed E5 model cache in the XDG Givn cache and permanent semantic scenarios
    When the user runs `givn spec index`
    Then the command exits 0
    And stderr does not announce a model download
    And stderr does not report per-file download progress
    And stderr announces the embedding of the pending scenarios with their count

  @givn.added
  Scenario: A failed model download still announces the started download phase
    Given an initialized project with unreachable semantic model sources
    When the user runs `givn spec index`
    Then the command exits non-zero
    And stderr announces the start of the retrieval model download with the E5 cache location
    And stderr identifies the failed E5 model artifact

  @givn.added @nli-combined @e2e @wip
  Scenario: Duplicate audit keeps stdout pure JSON while progress runs on stderr
    Given fresh BGE and NLI caches in the XDG Givn cache and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly enabled for automatic acquisition
    When the end-to-end user audits permanent scenarios with `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON
    And stderr announces the BGE and NLI model downloads
    And stderr reports per-file download progress for the second-stage model artifacts

  @givn.added
  Scenario: Disabled evidence layers announce no downloads
    Given an initialized project with fresh BGE and NLI caches in the XDG Givn cache and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly disabled
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON
    And stderr does not announce a BGE or NLI model download

  @givn.added @nli-combined @e2e @wip
  Scenario: Advisory review announces second-stage model acquisition during the run
    Given an initialized project with a completed permanent semantic index and a cold second-stage model cache
    And an active change delta is available for advisory review
    And BGE and NLI evidence are explicitly enabled
    When the end-to-end user runs advisory review for the active change through the givn CLI
    Then the command exits 0
    And stdout reports advisory retrieval evidence
    And stderr announces the BGE and NLI model downloads
    And stderr reports per-file download progress for the second-stage model artifacts

  @givn.added
  Scenario: Embedding multiple scenarios announces the count and reports per-scenario progress
    Given an initialized project with a completed E5 model cache in the XDG Givn cache and three permanent semantic scenarios
    When the user runs `givn spec index`
    Then the command exits 0
    And stderr announces the embedding of three pending scenarios
    And stderr reports the embedding progress for each pending scenario

  @givn.added @e2e @wip
  Scenario: Search with a cold E5 cache announces model acquisition before results
    Given an initialized project with a completed permanent semantic index and a cold E5 model cache
    And a reachable model source
    When the end-to-end user searches for a permanent semantic scenario with model acquisition through the givn CLI
    Then the command exits 0
    And stderr announces the start of the retrieval model download with the E5 cache location
    And stderr reports per-file download progress for the E5 model artifacts
    And stdout reports ranked search results