# User Interaction Inventory:
# - run permanent scenario exploration through the givn CLI
# - run change-aware scenario exploration through the givn CLI
# - run advisory change review through the givn CLI
# - run blocking change review through the givn CLI
# - run the permanent duplicate audit through the givn CLI
# - request semantic authoring guidance through the givn CLI

@givn.delta @bge-reranking-evidence

Feature: BGE reranking evidence for bounded semantic search

  @givn.added @bge @e2e @wip
  Scenario: Permanent exploration exposes bounded second-stage evidence
    Given an end-to-end project with a permanent semantic candidate for second-stage evidence
    And the local second-stage evidence is enabled
    And a complete draft scenario is available for permanent exploration
    When the end-to-end user explores the draft through the givn CLI
    Then the command exits 0
    And stdout identifies the E5 candidate generator
    And stdout identifies the second-stage model
    And stdout reports a symmetric raw score and elapsed runtime
    And stdout reports that the second-stage candidate pool is bounded

  @givn.added @bge @e2e @wip
  Scenario: Change-aware exploration narrows the union of permanent and active neighbours
    Given an end-to-end project with permanent and active-change semantic candidates
    And the local second-stage evidence is enabled
    And a complete draft scenario is available for change-aware exploration
    When the end-to-end user explores the draft against change "current-change" through the givn CLI
    Then the command exits 0
    And stdout contains both a permanent and a current-change candidate
    And stdout reports one bounded second-stage pool and a reduced recommended list for the query
    And stdout excludes the query's own delta and projected target identities

  @givn.added @bge @e2e @wip
  Scenario: Advisory change review presents second-stage evidence without classifying candidates
    Given an end-to-end project with an active semantic review candidate
    And the local second-stage evidence is enabled
    When the end-to-end user runs advisory change review for "running-change" through the givn CLI
    Then the command exits 0
    And stdout identifies the second-stage model, score, and elapsed runtime
    And stdout reports fewer recommended candidates than E5 candidates
    And stdout says that the evidence is advisory
    And stdout contains no automatic Givn classification

  @givn.added @bge @e2e @wip
  Scenario: Blocking change review records second-stage evidence in the semantic report
    Given an end-to-end project with an unresolved blocking semantic candidate
    And the local second-stage evidence is enabled
    When the end-to-end user runs blocking change review for "running-change" through the givn CLI
    Then the command exits non-zero because the candidate is unresolved
    And the semantic review report contains the second-stage model identity
    And the semantic review report contains the symmetric raw score and elapsed runtime
    And the semantic review report records E5 candidates filtered from the recommended worklist
    And the semantic review report keeps the candidate classification unresolved
    And the active and permanent specifications remain unchanged

  @givn.added @bge @e2e @wip
  Scenario: Permanent duplicate audit uses a bounded endpoint-neighbour union
    Given an end-to-end permanent corpus with deterministic overlaps and many semantic candidates
    And the local second-stage evidence is enabled
    When the end-to-end user runs the permanent duplicate audit through the givn CLI
    Then the command exits 0
    And the audit reports the E5 candidate count and bounded second-stage pair count
    And the audit reports the reduced recommended candidate count and filtered candidate count
    And the audit reports the second-stage model identity, symmetric score, and elapsed runtime
    And deterministic exact and structural overlaps are absent from second-stage scoring
    And the second-stage pair count is no greater than the documented endpoint-neighbour bound

  @givn.added @bge @e2e @wip
  Scenario: Semantic authoring guidance prevents score-avoidance prose
    Given an initialized givn project
    When the end-to-end user requests semantic authoring guidance through the givn CLI
    Then the command exits 0
    And stdout tells the author to sharpen invariants and boundary facts
    And stdout rejects unnecessary length, duplicated setup, and irrelevant qualifiers as score workarounds

  @givn.added @bge-disabled
  Scenario: E5-only exploration remains the default when second-stage evidence is disabled
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage evidence is disabled
    And a complete draft scenario is available for permanent exploration
    When the user explores the draft through the givn CLI
    Then the command exits 0
    And the result set contains the E5 candidates
    And stdout identifies the E5-only configuration
    And stdout contains no participating second-stage model, score, or runtime

  @givn.added @bge-disabled
  Scenario: E5-only JSON omits opt-in BGE evidence fields
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage evidence is disabled
    And a complete draft scenario is available for permanent exploration
    When the user explores the draft with JSON output
    Then the JSON retains the existing E5 query, index, and result contract
    And the JSON contains no BGE evidence object or participating model

  @givn.added @bge
  Scenario: Enabling second-stage evidence preserves the E5 raw candidate identities
    Given a project with a permanent semantic candidate for second-stage evidence
    And a complete draft scenario is available for permanent exploration
    When the user explores the draft once with second-stage evidence disabled
    And the user explores the same draft once with second-stage evidence enabled
    Then both runs report the same E5 raw candidate identities in the same order
    And the enabled run reports a reduced recommended candidate list when the evidence rejects a candidate
    And only the enabled run reports participating second-stage evidence

  @givn.added @bge
  Scenario: Both pair directions form the documented symmetric raw score
    Given a project with one permanent semantic pair and directional local evidence scores
    When the user runs the permanent duplicate audit with second-stage evidence enabled
    Then the audit reports both directional scores
    And the reported raw score is their documented symmetric average
    And reversing endpoint order does not change the reported raw score

  @givn.added @bge
  Scenario: Deterministic exclusions happen before second-stage scoring
    Given a permanent corpus containing exact, structural, and semantic relationships
    And the local second-stage evidence is enabled
    When the user runs the permanent duplicate audit with JSON output
    Then exact and structural relationships retain their deterministic evidence
    And only the eligible semantic neighbour union has second-stage evidence
    And no deterministic exclusion is counted as a second-stage pair

  @givn.added @bge
  Scenario: A low-relevance E5 false positive is filtered without receiving a Givn classification
    Given a project with two E5 candidates and one labeled false positive
    And the local second-stage evidence gives the false positive a low relevance score
    When the user runs advisory change review with second-stage evidence enabled
    Then the command exits 0
    And the false positive is recorded as filtered from the recommended list
    And the false positive is not classified as `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN`
    And the remaining recommended candidate retains its E5 and second-stage evidence

  @givn.added @bge @wip
  Scenario: Filtered evidence does not block the blocking change review
    Given a project with only E5 candidates that receive filtered second-stage evidence
    When the user runs blocking change review with second-stage evidence enabled
    Then the command exits 0
    And the semantic review report records the E5 candidates as filtered
    And the filtered candidates have no Givn classification
    And the report status is "PASS"

  @givn.added @bge
  Scenario: The exact bounded pair limit refuses inference before model loading
    Given a project whose deterministic E5 neighbour union contains exactly 513 eligible pairs
    And the local second-stage evidence is enabled
    When the user runs the permanent duplicate audit with second-stage evidence enabled
    Then the command exits 0
    And stdout contains "BGE_UNAVAILABLE: bounded candidate pool exceeds 512 pairs"
    And the command reports the bounded-pool overflow and the unavailable BGE status
    And the output reports BGE unavailable reason "pair-cap"
    And the output reports zero unique pairs, directional calls, recommended pairs, filtered pairs, and model load attempts
    And the output reports all E5 raw pairs as unscored
    And the output identifies the E5-only unavailable fallback
    And the output reports zero BGE model load attempts
    And no output claims that BGE participated

  @givn.added @bge
  Scenario: The exact bounded pair limit accepts 512 unique pairs
    Given a project whose deterministic E5 neighbour union contains exactly 512 eligible pairs
    And the local second-stage evidence is enabled
    When the user runs the permanent duplicate audit with second-stage evidence enabled
    Then the command exits 0
    And the audit reports 512 unique pairs and 1024 directional calls
    And the audit reports no bounded-pool overflow

  @givn.added @bge
  Scenario: BGE does not select NLI without the independent NLI capability
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage evidence is enabled without NLI evidence
    And the legacy NLI selector is set without enabling the independent capability
    And a complete draft scenario is available for permanent exploration
    When the user explores the draft with second-stage evidence enabled
    Then the command exits 0
    And stdout contains no NLI evidence
    And stdout identifies BGE as the participating second-stage model

  @givn.added @nli-future @wip
  Scenario: The independent NLI capability owns the legacy diagnostic selector
    Given a project with a permanent semantic candidate for second-stage evidence
    And the independent NLI evidence capability is enabled
    And the legacy NLI selector is explicitly selected
    When the user runs advisory change review through the NLI-capable CLI
    Then stdout contains diagnostic NLI evidence
    And stdout contains no BGE admission status
    And the E5 candidate identities remain unchanged

  @givn.added @bge
  Scenario: BGE JSON preserves raw ordering and names every admission count
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage evidence is enabled
    And a complete draft scenario is available for permanent exploration
    When the user explores the draft with JSON output and second-stage evidence enabled
    Then the JSON retains the E5 raw result order and legacy result fields
    And the JSON evidence reports raw, recommended, filtered, unscored, unique-pair, and directional-call counts
    And the JSON evidence reports the BGE model and elapsed runtime

  @givn.added @bge
  Scenario: E5 candidates outside the bounded BGE set remain explicitly unscored and blocking
    Given a project with exactly 513 E5 raw semantic pairs and a successful bounded top-K union of exactly 512 pairs
    And the local second-stage evidence is enabled
    When the user runs blocking change review with second-stage evidence enabled
    Then the semantic review report records BGE status "BGE_ENABLED"
    And the semantic review report records exactly 512 unique pairs and 1024 directional calls
    And exactly one E5 candidate outside the bounded set is recorded as `BGE_UNSCORED`
    And the semantic review report records the outside candidates as `BGE_UNSCORED`
    And those candidates remain blocking E5 candidates
    And each unscored candidate block has null BGE directional scores and the E5 rank/endpoint order
    And no unscored candidate is labeled `BGE_FILTERED`
    And no unscored candidate is present in the filtered evidence collection

  @givn.added @bge
  Scenario: Duplicate JSON exposes partial unscored candidates
    Given a permanent corpus with exactly 513 E5 raw semantic pairs and a successful bounded top-K union of exactly 512 pairs
    And the local second-stage evidence is enabled
    When the user runs the permanent duplicate audit with JSON output and second-stage evidence enabled
    Then the JSON reports BGE status "BGE_ENABLED" with 512 unique pairs and 1024 directional calls
    And the JSON contains an `unscored_candidates` collection
    And each unscored JSON candidate has an endpoint, E5 score, and status `BGE_UNSCORED`
    And unscored JSON candidates use descending E5 score and endpoint tuple order
    And each unscored candidate remains outside the filtered collection
    And the JSON contains no unscored candidate with BGE directional scores

  @givn.added @bge @gate-combination
  Scenario: The independent E5 gate remains separate from BGE evidence
    Given an end-to-end project with an active semantic review candidate
    And the local second-stage evidence is enabled
    When the user runs blocking change review with the E5 similarity gate and second-stage evidence enabled
    Then the command reports the E5 gate result separately from BGE admission
    And a BGE status does not replace the E5 gate result
    And no BGE status is rendered as a Givn classification

  @givn.added @gate-only
  Scenario: The E5 similarity gate remains available without BGE
    Given an end-to-end project with an active semantic review candidate
    When the user runs blocking change review with the E5 similarity gate and BGE disabled
    Then the command reports the E5 gate result
    And the output contains no participating BGE model or score

  @givn.added @bge
  Scenario: A same-file candidate remains eligible after identity exclusions
    Given a modified active scenario shares its projected target file with another permanent scenario
    And the local second-stage evidence is enabled
    When the user runs change-aware exploration against that change
    Then the other same-file scenario remains in the E5 raw candidates
    And only the exact source and projected-target identities are excluded

  @givn.added @bge
  Scenario: The admission operating point includes a symmetric score of `3.0000`
    Given a project with one E5 candidate whose symmetric second-stage score is exactly `3.0000`
    When the user runs advisory change review with second-stage evidence enabled
    Then the candidate is recommended rather than filtered
    And the candidate still requires an explicit Givn classification

  @givn.added @bge @wip
  Scenario: Missing second-stage model artifacts are explicitly unavailable
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage model artifacts are unavailable
    When the user explores the draft with second-stage evidence enabled
    Then the command exits 0
    And stdout contains "BGE_UNAVAILABLE"
    And stdout reports BGE unavailable reason "missing-artifact"
    And stdout reports zero unique pairs, directional calls, recommended pairs, filtered pairs, and model load attempts
    And stdout does not claim that the second-stage model participated

  @givn.added @bge @wip
  Scenario: A corrupt local BGE cache is rejected before model construction
    Given a project with a corrupt local second-stage model cache
    And local second-stage artifact mode is selected
    When the user explores the draft with second-stage evidence enabled
    Then the command exits 0
    And stdout reports BGE status "BGE_UNAVAILABLE"
    And stdout reports reason "missing-artifact"
    And stdout reports zero model load attempts

  @givn.added @bge @wip
  Scenario: Unavailable BGE JSON preserves raw candidates and names unscored pairs
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage model artifacts are unavailable
    When the user explores the draft with JSON output and second-stage evidence enabled
    Then the command exits 0
    And the JSON reports BGE status "BGE_UNAVAILABLE"
    And the JSON reports reason "missing-artifact"
    And the JSON reports zero scored, recommended, and filtered pairs
    And the JSON retains the E5 raw result identities
    And the JSON counts the raw candidates as unscored rather than filtered

  @givn.added @bge @wip
  Scenario: Unavailable BGE duplicate JSON preserves the E5 fallback accounting
    Given a permanent corpus with semantic candidates and unavailable second-stage artifacts
    When the user runs the permanent duplicate audit with JSON output and second-stage evidence enabled
    Then the command exits 0
    And the JSON reports BGE status "BGE_UNAVAILABLE"
    And the JSON reports reason "missing-artifact"
    And the JSON reports zero scored, recommended, and filtered pairs
    And the JSON counts all E5 semantic candidates as unscored
    And the JSON retains the E5 fallback candidates

  @givn.added @bge @wip
  Scenario: Unavailable BGE blocking review keeps E5 candidates blocking
    Given an end-to-end project with an unresolved blocking semantic candidate
    And the local second-stage model artifacts are unavailable
    When the end-to-end user runs blocking change review for "running-change" through the givn CLI
    Then the command exits non-zero because the E5 candidate is unresolved
    And the semantic review report has BGE status "BGE_UNAVAILABLE"
    And the semantic review report has BGE reason "missing-artifact"
    And the semantic report has zero BGE unique pairs, directional calls, filtered pairs, and model load attempts
    And the semantic review report contains a normal E5 candidate block
    And the report contains no `BGE_FILTERED` candidate block

  @givn.added @bge @wip
  Scenario: Pair-cap unavailable blocking review retains E5 candidates
    Given an end-to-end project with an unresolved blocking semantic candidate and a 513-pair BGE pool
    And the local second-stage evidence is enabled
    When the end-to-end user runs blocking change review for "running-change" through the givn CLI
    Then the command exits non-zero because the E5 candidate is unresolved
    And the semantic review report has BGE status "BGE_UNAVAILABLE"
    And the semantic review report has BGE reason "pair-cap"
    And the semantic report retains the E5 candidate block without `BGE_FILTERED`

  @givn.added @bge @wip
  Scenario: Unavailable BGE change exploration retains the E5 raw fallback
    Given a project with permanent and active-change semantic candidates
    And the local second-stage model artifacts are unavailable
    And a complete draft scenario is available for change-aware exploration
    When the user explores the draft against change "current-change" with second-stage evidence enabled
    Then the command exits 0
    And stdout reports BGE status "BGE_UNAVAILABLE"
    And stdout retains the permanent and current-change E5 raw candidates

  @givn.added @bge @wip
  Scenario: Unavailable BGE advisory review remains advisory
    Given a project with an active semantic review candidate
    And the local second-stage model artifacts are unavailable
    When the user runs advisory change review with second-stage evidence enabled
    Then the command exits 0
    And stdout reports BGE status "BGE_UNAVAILABLE"
    And stdout remains advisory and does not assign a Givn classification

  @givn.added @bge-disabled @wip
  Scenario: Failed benchmark approval keeps advisory review on E5-only evidence
    Given a project with an active semantic review candidate
    And the BGE admission benchmark has failed
    When the user runs advisory change review with second-stage evidence enabled
    Then the command exits 0
    And stdout identifies the E5-only configuration
    And stdout contains no participating second-stage model, score, or runtime
    And stdout contains no `BGE_FILTERED` evidence

  @givn.added @bge-disabled @wip
  Scenario: Failed benchmark approval keeps permanent exploration on E5-only evidence
    Given a project with a permanent semantic candidate for second-stage evidence
    And the BGE admission benchmark has failed
    And a complete draft scenario is available for permanent exploration
    When the user explores the draft through the givn CLI
    Then the command exits 0
    And stdout identifies the E5-only configuration
    And stdout contains no participating second-stage model, score, or runtime

  @givn.added @bge-disabled @wip
  Scenario: Failed benchmark approval keeps change-aware exploration on E5-only evidence
    Given a project with permanent and active-change semantic candidates
    And the BGE admission benchmark has failed
    And a complete draft scenario is available for change-aware exploration
    When the user explores the draft against change "current-change" through the givn CLI
    Then the command exits 0
    And stdout identifies the E5-only configuration
    And stdout contains no participating second-stage model, score, or runtime

  @givn.added @bge-disabled @wip
  Scenario: Failed benchmark approval keeps blocking review on E5-only evidence
    Given a project with an unresolved blocking semantic candidate
    And the BGE admission benchmark has failed
    When the user runs blocking change review with second-stage evidence enabled
    Then the semantic review report uses schema version 1
    And the report contains no BGE status, model, score, or admission fields
    And the unresolved E5 candidate remains blocking

  @givn.added @bge-disabled @wip
  Scenario: Failed benchmark approval keeps duplicate JSON on E5-only evidence
    Given a permanent corpus with semantic candidates
    And the BGE admission benchmark has failed
    When the user runs the permanent duplicate audit with JSON output and second-stage evidence enabled
    Then the JSON schema version is exactly 1
    And the JSON contains no BGE run, filtered, or unscored collections

  @givn.added @bge-disabled @wip
  Scenario: E5-only blocking review preserves the semantic report contract
    Given a project with an unresolved blocking semantic candidate
    And second-stage evidence is disabled
    When the user runs blocking change review with second-stage evidence disabled
    Then the semantic review report uses schema version 1
    And the report contains no BGE status, model, score, or admission fields
    And the unresolved E5 candidate remains blocking

  @givn.added @bge-disabled @wip
  Scenario: E5-only duplicate JSON preserves schema version 1
    Given a permanent corpus with semantic candidates
    And second-stage evidence is disabled
    When the user runs the permanent duplicate audit with JSON output
    Then the JSON schema version is exactly 1
    And the JSON contains no BGE run, filtered, or unscored collections

  @givn.added @bge @wip
  Scenario: A malformed directional evidence seam is rejected before scoring
    Given a project with one E5 candidate and malformed directional local evidence scores
    When the user runs advisory change review with second-stage evidence enabled
    Then the command exits non-zero
    And stderr identifies the malformed directional evidence entry
    And stderr reports zero BGE model load attempts

  @givn.added @bge @wip
  Scenario: A complete local BGE artifact loads without a downloader
    Given a project with one E5 candidate and a complete local BGE artifact set
    And local second-stage artifact mode is selected
    When the user runs advisory change review with second-stage evidence enabled
    Then the command exits 0
    And stdout identifies the local BGE model as participating
    And stdout reports two directional calls for one unique pair
    And stdout reports no model download

  @givn.added @bge-approval @wip
  Scenario: Approval writer requires enforced Rust parity and complete artifacts
    Given a benchmark report with the Python reference candidate pipeline
    And a Rust production parity attestation emitted by `givn spec bge-parity`
    And a complete local BGE artifact set with matching hashes
    When the maintainer writes the BGE approval artifact with enforcement enabled
    Then the approval artifact has exactly the documented top-level fields
    And the approval artifact records the Rust pipeline fingerprint and all artifact hashes
    And the approval artifact records every benchmark acceptance boolean

  @givn.added @bge-approval @wip
  Scenario: Approval writer rejects a missing Rust parity attestation
    Given a passing benchmark report without a Rust production parity attestation
    When the maintainer writes the BGE approval artifact with enforcement enabled
    Then the approval command exits non-zero
    And stderr says that Rust parity is required
    And no BGE approval artifact is written

  @givn.added @bge-approval @wip
  Scenario: Approval writer rejects an incomplete local BGE artifact set
    Given a benchmark report with the Python reference candidate pipeline
    And a Rust production parity attestation emitted by `givn spec bge-parity`
    And a local BGE artifact set missing `special_tokens_map.json`
    When the maintainer writes the BGE approval artifact with enforcement enabled
    Then the approval command exits non-zero
    And stderr identifies the missing `special_tokens_map.json` artifact
    And no BGE approval artifact is written

  @givn.added @bge-approval @wip
  Scenario: Approval writer rejects a forged parity document
    Given a benchmark report with the Python reference candidate pipeline
    And a forged Rust production parity attestation
    And a complete local BGE artifact set with matching hashes
    When the maintainer writes the BGE approval artifact with enforcement enabled
    Then the approval command exits non-zero
    And stderr says that the parity file does not match the compiled binary
    And no BGE approval artifact is written

  @givn.added @bge-approval @wip
  Scenario: Approval writer requires enforcement for any approval output
    Given a passing benchmark report with a Rust production parity attestation
    When the maintainer writes the BGE approval artifact without enforcement
    Then the approval command exits non-zero
    And stderr says that `--approval-file` requires `--enforce`
    And no BGE approval artifact is written

  @givn.added @wip
  Scenario: A binary without retrieval reports the explicit unavailable mode
    Given a project with a permanent semantic candidate for second-stage evidence
    When the user runs exploration through a binary without the retrieval capability
    Then the command exits non-zero
    And stderr contains "spec retrieval is unavailable"

  @givn.added @bge @wip
  Scenario: Repeated bounded scoring is deterministic and reports runtime separately
    Given a project with a permanent semantic candidate for second-stage evidence
    And the local second-stage evidence is enabled
    And a complete draft scenario is available for permanent exploration
    When the user explores the same draft twice through the givn CLI
    Then the E5 candidate identities and second-stage scores are byte-for-byte stable
    And each run reports a non-negative second-stage elapsed runtime
    And the runtime does not affect candidate ordering
