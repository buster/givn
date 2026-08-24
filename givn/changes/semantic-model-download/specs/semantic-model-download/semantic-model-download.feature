# User Interaction Inventory:
# - build or refresh the permanent semantic index through `givn spec index`
# - search permanent semantic scenarios through `givn spec search`
# - explore a permanent or change-aware semantic scenario through `givn spec explore`
# - request advisory semantic review through `givn spec review`
# - request blocking semantic review through `givn check review`
# - run a second-stage semantic audit through `givn spec duplicates --format json`

@givn.delta @semantic-model-download
Feature: Automatic semantic model availability

  @givn.added @e2e
  Scenario: First semantic indexing downloads E5 artifacts before applying the token guard
    Given an initialized project with a fresh XDG Givn cache and a reachable model source
    And the project contains fitting permanent semantic scenarios
    When the end-to-end user builds the permanent semantic index with `givn spec index`
    Then the command exits 0
    And stdout reports a completed permanent semantic index
    And the permanent semantic index file exists
    And the E5 tokenizer and model artifacts exist in the XDG Givn cache

  @givn.added
  Scenario: A successful E5 setup is reused on the next indexing run
    Given an initialized project with a completed E5 model cache in the XDG Givn cache and permanent semantic scenarios
    When the user runs `givn spec index`
    Then the command exits 0
    And stdout reports that existing semantic model artifacts were reused
    And the permanent semantic index file exists

  @givn.added
  Scenario: A failed model download explains how to provide the missing artifacts
    Given an initialized project with unavailable semantic model sources
    When the user runs `givn spec index`
    Then the command exits non-zero
    And stderr identifies the E5 model cache and failed artifact
    And stderr names a local artifact or environment-variable alternative
    And no permanent semantic index file is written

  @givn.added
  Scenario: A failed model download leaves an existing permanent index unchanged
    Given an initialized project with an existing permanent semantic index and unavailable semantic model sources
    When the user runs `givn spec index`
    Then the command exits non-zero
    And stderr identifies the E5 model cache and failed artifact
    And the existing permanent semantic index remains unchanged byte-for-byte

  @givn.added
  Scenario: Malformed or incompatible indexes are not reported as absent
    Given an initialized project with a malformed or incompatible permanent semantic index
    When the user searches for a permanent semantic scenario
    Then the command exits non-zero
    And stderr reports the index validation problem
    And stderr does not report `spec index is missing at <path>; run `givn spec index` first`

  @givn.added
  Scenario: Duplicate audit identifies an absent persistent index without changing its fallback contract
    Given an initialized project with permanent semantic scenarios and no persistent semantic index
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON
    And stderr reports `spec index is missing at <path>; run `givn spec index` first`
    And the persistent semantic index remains absent

  @givn.added @nli @nli-combined
  Scenario: Failed second-stage downloads remain unavailable and actionable
    Given an initialized project with unavailable BGE and NLI model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly enabled for automatic acquisition
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON with unavailable BGE and NLI evidence
    And stderr names the failed model artifacts and their local override variables

  @givn.added @nli @nli-combined
  Scenario: A corrupt second-stage cache is repaired before evidence scoring
    Given an initialized project with corrupt BGE and NLI cache snapshots and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly enabled for automatic acquisition
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON with participating BGE and NLI evidence
    And the corrupt cache snapshots are replaced by complete validated snapshots

  @givn.added
  Scenario: Explicitly disabled evidence layers do not download models
    Given an initialized project with fresh BGE and NLI caches in the XDG Givn cache and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly disabled
    When the user runs `givn spec duplicates --format json`
    Then the command exits 0
    And the JSON contains the E5 candidates without BGE or NLI evidence
    And the BGE and NLI caches remain empty

  @givn.added @e2e
  Scenario: Search identifies the missing-index repair command
    Given an initialized project with permanent semantic scenarios and no persistent semantic index
    When the end-to-end user searches for a permanent semantic scenario without an index through the givn CLI
    Then the search reports `spec index is missing at <path>; run `givn spec index` first`

  @givn.added @e2e
  Scenario: Explore identifies the missing-index repair command
    Given an initialized project with permanent semantic scenarios and no persistent semantic index
    And a complete draft scenario is available for exploration
    When the end-to-end user explores the draft without an index through the givn CLI
    Then the exploration reports `spec index is missing at <path>; run `givn spec index` first`

  @givn.added @e2e @wip
  Scenario: Advisory review identifies the missing-index repair command
    Given an initialized project with permanent semantic scenarios and no persistent semantic index
    And an active change delta is available for advisory review
    When the end-to-end user runs advisory review without an index through the givn CLI
    Then the advisory review reports `spec index is missing at <path>; run `givn spec index` first`

  @givn.added @e2e @wip
  Scenario: Blocking review identifies the missing-index repair command
    Given an initialized project with permanent semantic scenarios and no persistent semantic index
    And an active change delta is available for blocking review
    When the end-to-end user runs blocking review without an index through the givn CLI
    Then the blocking review reports `spec index is missing at <path>; run `givn spec index` first`

  @givn.added
  Scenario: Archive maintenance does not download models implicitly
    Given an initialized project with a completed permanent semantic index and a cold model cache
    And a completed change is ready to archive without semantic model artifacts
    When the user archives the completed change without model artifacts
    Then the command does not download model artifacts
    And the archive completes without a model-setup failure

  @givn.added @nli @nli-combined @e2e @wip
  Scenario: Duplicate audit identifies the second-stage model caches
    Given an initialized project with fresh BGE and NLI caches in the XDG Givn cache and reachable model sources
    And the project contains permanent semantic candidates for both evidence layers
    And BGE and NLI evidence are explicitly enabled
    When the end-to-end user audits permanent scenarios with `givn spec duplicates --format json`
    Then the command exits 0
    And stdout is valid duplicate-audit JSON
    And the JSON reports participating BGE and NLI evidence
    And the BGE model artifacts exist in the XDG Givn reranker cache
    And the NLI model artifacts exist in the XDG Givn NLI cache

  @givn.modified @nli
  Scenario: NLI uses the bounded E5 neighbour union for the permanent duplicate audit
    Given a permanent corpus with deterministic semantic candidates for an independent NLI audit
    When the user runs the permanent NLI duplicate audit
    Then NLI receives the full E5 semantic candidate pool independently of BGE
    And the NLI audit retains the E5 candidate identities and ordering

  @givn.added
  Scenario: A feature-free build keeps retrieval explicitly unavailable
    Given an initialized project with permanent semantic scenarios
    When the user runs semantic search through a build without the retrieval capability
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"
