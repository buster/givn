Feature: NLI evidence for bounded semantic candidate reduction

  @nli @e2e
  Scenario: NLI reduces permanent exploration candidates without changing E5 raw results
    Given  an end-to-end project with two permanent semantic candidates for NLI evidence
    And  NLI evidence is enabled while BGE evidence is disabled
    And  a complete draft scenario is available for NLI exploration
    When  the end-to-end user explores the draft with NLI through the givn CLI
    Then  the command exits 0
    And  stdout identifies E5 as the raw candidate generator
    And  stdout identifies the NLI model and both directional probability sets
    And  stdout reports fewer NLI recommended candidates than E5 candidates
    And  stdout keeps the E5 raw candidate identities visible

  @nli @e2e
  Scenario: NLI reduces change-aware exploration candidates without changing E5 raw results
    Given  an end-to-end project with permanent and active-change semantic candidates for NLI evidence
    And  NLI evidence is enabled while BGE evidence is disabled
    And  a complete draft scenario is available for NLI change-aware exploration
    When  the end-to-end user explores the draft against change "current-change" with NLI through the givn CLI
    Then  the command exits 0
    And  stdout contains both a permanent and a current-change NLI candidate
    And  stdout reports fewer NLI recommended candidates than E5 candidates
    And  stdout excludes the query's own delta and projected target identities

  @nli
  Scenario: NLI reports a symmetric probability summary from both directions
    Given  a project with one semantic candidate and directional NLI evidence scores
    When  the user runs advisory NLI review
    Then  the output reports entailment, neutral, and contradiction probabilities in both directions
    And  the output reports the symmetric NLI probability summary
    And  the output reports an NLI admission status
    And  the symmetric summary is independent of endpoint order

  @nli @e2e
  Scenario: NLI reduces advisory review candidates without assigning a Givn class
    Given  an end-to-end project with an active semantic review candidate for NLI evidence
    And  NLI evidence filters the unrelated candidate
    When  the end-to-end user runs advisory NLI review through the givn CLI
    Then  the command exits 0
    And  stdout reports the NLI model and elapsed runtime
    And  stdout reports fewer NLI recommendations than E5 candidates
    And  stdout records the filtered candidate as NLI evidence
    And  stdout assigns no automatic Givn classification

  @nli @e2e
  Scenario: NLI filtered evidence does not block the blocking review
    Given  an end-to-end project whose active semantic candidates are all filtered by NLI evidence
    When  the end-to-end user runs blocking NLI review through the givn CLI
    Then  the command exits 0
    And  the semantic review report has NLI status enabled
    And  the semantic review report records the filtered candidates without a Givn classification
    And  the semantic review report status is "PASS"

  @nli
  Scenario: NLI uses the bounded E5 neighbour union for the permanent duplicate audit
    Given  a permanent corpus with deterministic semantic candidates for an independent NLI audit
    When  the user runs the permanent NLI duplicate audit
    Then  NLI receives the full E5 semantic candidate pool independently of BGE
    And  the NLI audit retains the E5 candidate identities and ordering

  @nli @nli-combined
  Scenario: BGE and NLI jointly reduce the recommended candidate set
    Given  a project with semantic candidates and independent BGE and NLI evidence
    When  the user runs combined second-stage advisory review
    Then  the E5 raw candidate identities remain unchanged
    And  a candidate is recommended only when both BGE and NLI recommend it
    And  each filtered candidate remains evidence without a Givn classification

  @nli @nli-combined
  Scenario: An unavailable NLI layer keeps a combined candidate blocking
    Given  a project with a semantic candidate that BGE recommends and NLI cannot score
    When  the user runs combined second-stage blocking review
    Then  the command exits non-zero because the E5 candidate remains unresolved
    And  the report records NLI as unavailable without NLI filtered evidence
    And  the candidate remains a normal blocking candidate with no NLI probabilities

  @nli
  Scenario: NLI keeps candidates outside a partial bounded pool blocking
    Given  a permanent corpus with 513 E5 semantic candidates and a successful bounded NLI pool of 512 pairs
    When  the user runs the permanent NLI duplicate audit with JSON output
    Then  the JSON reports NLI as enabled with 512 unique pairs and 1024 directional calls
    And  the JSON contains NLI unscored candidates with null NLI probabilities
    And  the NLI unscored candidates remain outside the NLI filtered collection

  @nli
  Scenario: NLI JSON preserves the raw E5 contract and uses the NLI schema extension
    Given  a project with two E5 semantic candidates and NLI evidence
    When  the user explores the draft with JSON output and NLI evidence enabled
    Then  the JSON uses schema version 3 for NLI evidence
    And  the JSON preserves raw E5 result order and legacy result fields
    And  the JSON reports NLI model, probabilities, raw, recommended, filtered, unscored, pair, and runtime fields

  @nli @nli-disabled
  Scenario: Disabling NLI preserves the E5-only contract in an NLI-capable build
    Given  a project with a permanent semantic candidate and the NLI capability compiled
    And  NLI evidence is disabled
    When  the user explores the draft through the NLI-capable CLI
    Then  the result contains the E5 candidates
    And  stdout contains no NLI model, probability, runtime, or admission status
    And  the output uses the E5-only contract

  @nli
  Scenario: Missing NLI artifacts preserve the explicit E5 fallback
    Given  a project with a semantic candidate and unavailable NLI model artifacts
    When  the user runs advisory NLI review
    Then  the command exits 0
    And  the output reports NLI as unavailable
    And  the output reports the unavailable reason as missing-artifact
    And  the E5 candidates remain visible and blocking when applicable
    And  no output claims that NLI participated

  @nli
  Scenario: The NLI admission boundary includes a symmetric entailment probability of 0.5000
    Given  a project with one semantic candidate at the NLI admission boundary
    When  the user runs advisory NLI review
    Then  the candidate is NLI recommended rather than NLI filtered
    And  the candidate still requires an explicit Givn classification

  @nli
  Scenario: An NLI-recommended candidate still blocks unresolved semantic review
    Given  a project with an active semantic candidate that NLI recommends
    When  the user runs blocking NLI review
    Then  the command exits non-zero because the candidate is unresolved
    And  the semantic report contains an NLI-recommended candidate block
    And  the semantic review report keeps the candidate classification unresolved
