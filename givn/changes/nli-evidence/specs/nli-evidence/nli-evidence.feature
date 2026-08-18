# User Interaction Inventory:
# - explore a draft scenario against permanent candidates through the givn CLI
# - explore a draft scenario against permanent and active-change candidates through the givn CLI
# - run advisory change review with NLI evidence through the givn CLI
# - run blocking change review with NLI evidence through the givn CLI
# - run the permanent duplicate audit with NLI evidence through the givn CLI

@givn.delta @nli-evidence
Feature: NLI evidence for bounded semantic candidate reduction

  @givn.added @nli @e2e @wip
  Scenario: NLI reduces permanent exploration candidates without changing E5 raw results
    Given an end-to-end project with two permanent semantic candidates for NLI evidence
    And NLI evidence is enabled while BGE evidence is disabled
    And a complete draft scenario is available for NLI exploration
    When the end-to-end user explores the draft through the givn CLI
    Then the command exits 0
    And stdout identifies E5 as the raw candidate generator
    And stdout identifies the NLI model and both directional probability sets
    And stdout reports fewer NLI recommended candidates than E5 candidates
    And stdout keeps the E5 raw candidate identities visible

  @givn.added @nli @e2e @wip
  Scenario: NLI reduces change-aware exploration candidates without changing E5 raw results
    Given an end-to-end project with permanent and active-change semantic candidates for NLI evidence
    And NLI evidence is enabled while BGE evidence is disabled
    And a complete draft scenario is available for NLI change-aware exploration
    When the end-to-end user explores the draft against change "current-change" through the givn CLI
    Then the command exits 0
    And stdout contains both a permanent and a current-change NLI candidate
    And stdout reports fewer NLI recommended candidates than E5 candidates
    And stdout excludes the query's own delta and projected target identities

  @givn.added @nli
  Scenario: NLI reports a symmetric probability summary from both directions
    Given a project with one semantic candidate and directional NLI evidence scores
    When the user runs advisory NLI review
    Then the output reports entailment, neutral, and contradiction probabilities in both directions
    And the output reports the symmetric NLI probability summary
    And the output reports an NLI admission status
    And the symmetric summary is independent of endpoint order

  @givn.added @nli @e2e @wip
  Scenario: NLI reduces advisory review candidates without assigning a Givn class
    Given an end-to-end project with an active semantic review candidate for NLI evidence
    And NLI evidence filters the unrelated candidate
    When the end-to-end user runs advisory NLI review through the givn CLI
    Then the command exits 0
    And stdout reports the NLI model and elapsed runtime
    And stdout reports fewer recommended candidates than E5 candidates
    And stdout records the filtered candidate as NLI evidence
    And stdout assigns no automatic Givn classification

  @givn.added @nli @e2e @wip
  Scenario: NLI filtered evidence does not block the blocking review
    Given an end-to-end project whose active semantic candidates are all filtered by NLI evidence
    When the end-to-end user runs blocking NLI review through the givn CLI
    Then the command exits 0
    And the semantic review report has NLI status enabled
    And the semantic review report records the filtered candidates without a Givn classification
    And the semantic review report status is "PASS"

  @givn.added @nli @e2e @wip
  Scenario: NLI uses the bounded E5 neighbour union for the permanent duplicate audit
    Given an end-to-end permanent corpus with deterministic overlaps and NLI semantic candidates
    When the end-to-end user runs the permanent NLI duplicate audit through the givn CLI
    Then the command exits 0
    And the audit reports the E5 candidate count and bounded NLI pair count
    And the audit reports reduced recommended and filtered NLI candidate counts
    And deterministic exact and structural overlaps have no NLI evidence

  @givn.added @nli @nli-combined
  Scenario: BGE and NLI jointly reduce the recommended candidate set
    Given a project with semantic candidates and independent BGE and NLI evidence
    When the user runs combined second-stage advisory review
    Then the E5 raw candidate identities remain unchanged
    And a candidate is recommended only when both BGE and NLI recommend it
    And each filtered candidate remains evidence without a Givn classification

  @givn.added @nli @nli-combined
  Scenario: An unavailable NLI layer keeps a combined candidate blocking
    Given a project with a semantic candidate that BGE recommends and NLI cannot score
    When the user runs combined second-stage blocking review
    Then the command exits non-zero because the E5 candidate remains unresolved
    And the report records NLI as unavailable without NLI filtered evidence
    And the candidate remains a normal blocking candidate with no NLI probabilities

  @givn.added @nli
  Scenario: NLI refuses an overflowing bounded pair pool before model loading
    Given a permanent corpus whose bounded NLI pool contains exactly 513 eligible pairs
    When the user runs the permanent NLI duplicate audit
    Then the command exits 0
    And the audit reports NLI as unavailable for reason "pair-cap"
    And the audit reports zero NLI pairs, directional calls, recommendations, filters, and model loads
    And all E5 semantic candidates are recorded as NLI unscored

  @givn.added @nli
  Scenario: NLI keeps candidates outside a partial bounded pool blocking
    Given a permanent corpus with 513 E5 semantic candidates and a successful bounded NLI pool of 512 pairs
    When the user runs the permanent NLI duplicate audit with JSON output
    Then the JSON reports NLI as enabled with 512 unique pairs and 1024 directional calls
    And the JSON contains NLI unscored candidates with null NLI probabilities
    And the NLI unscored candidates remain outside the NLI filtered collection

  @givn.added @nli
  Scenario: NLI JSON preserves the raw E5 contract and uses the NLI schema extension
    Given a project with two E5 semantic candidates and NLI evidence
    When the user explores the draft with JSON output and NLI evidence enabled
    Then the JSON uses schema version 3 for NLI evidence
    And the JSON preserves raw E5 result order and legacy result fields
    And the JSON reports NLI model, probabilities, raw, recommended, filtered, unscored, pair, and runtime fields

  @givn.added @nli @nli-disabled @wip
  Scenario: Disabling NLI preserves the E5-only contract in an NLI-capable build
    Given a project with a permanent semantic candidate and the NLI capability compiled
    And NLI evidence is disabled
    When the user explores the draft through the NLI-capable CLI
    Then the result contains the E5 candidates
    And stdout contains no NLI model, probability, runtime, or admission status
    And the output uses the E5-only contract

  @givn.added @nli @wip
  Scenario: Missing NLI artifacts preserve the explicit E5 fallback
    Given a project with a semantic candidate and unavailable NLI model artifacts
    When the user runs advisory NLI review
    Then the command exits 0
    And the output reports NLI as unavailable
    And the output reports the unavailable reason as missing-artifact
    And the E5 candidates remain visible and blocking when applicable
    And no output claims that NLI participated

  @givn.added @nli @wip
  Scenario: The NLI admission boundary includes a symmetric entailment probability of 0.5000
    Given a project with one semantic candidate at the NLI admission boundary
    When the user runs advisory NLI review
    Then the candidate is NLI recommended rather than NLI filtered
    And the candidate still requires an explicit Givn classification

  @givn.added @nli @wip
  Scenario: An NLI-recommended candidate still blocks unresolved semantic review
    Given a project with an active semantic candidate that NLI recommends
    When the user runs blocking NLI review
    Then the command exits non-zero because the candidate is unresolved
    And the semantic report contains an NLI-recommended candidate block
    And the candidate still requires an explicit Givn classification

  @givn.modified @nli @wip
  Scenario: The independent NLI capability owns the legacy diagnostic selector
    Given a project with a permanent semantic candidate for second-stage evidence
    And the independent NLI evidence capability is enabled
    And the legacy NLI selector is explicitly selected
    When the user runs advisory change review through the NLI-capable CLI
    Then stdout contains diagnostic NLI evidence
    And stdout contains no BGE admission status
    And stdout reports NLI recommended and filtered candidate counts
    And the E5 candidate identities remain unchanged
