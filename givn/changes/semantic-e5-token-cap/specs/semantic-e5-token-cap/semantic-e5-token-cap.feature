# User Interaction Inventory:
# - build or refresh the permanent E5 scenario index through the givn CLI
# - run the permanent duplicate audit through the givn CLI
# - explore a complete scenario through the givn CLI
# - run advisory semantic change review through the givn CLI
# - run blocking semantic change review through the givn CLI

@givn.delta @semantic-e5-token-cap

Feature: E5 token-cap findings preserve usable retrieval

  @givn.added @e2e
  Scenario: Indexing keeps fitting scenarios when another passage exceeds E5 capacity
    Given an end-to-end permanent corpus with one fitting scenario and one complete E5 passage above 512 tokens
    And deterministic E5 token-budget fixtures identify the over-cap scenario as requiring 550 tokens
    When the end-to-end user runs `givn spec index` through the givn CLI
    Then the command exits 0
    And the fitting scenario has a vector in the permanent index
    And the over-cap scenario has no truncated vector in the permanent index
    And the permanent index has one additive `e5_token_cap_candidates` record for the over-cap endpoint
    And that cap record has its file, title, complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and guidance
    And stdout identifies the over-cap scenario as an E5 token-cap refactoring finding

  @givn.added @e2e
  Scenario: Duplicate audit preserves deterministic findings and reports an E5 token-cap candidate
    Given an end-to-end permanent corpus with an exact duplicate, a structural overlap, and two complete E5 passages above 512 tokens
    And deterministic E5 token-budget fixtures identify the over-cap endpoints as requiring 550 and 551 tokens
    When the end-to-end user runs `givn spec duplicates --format json` through the givn CLI
    Then the command exits 0
    And the JSON retains the exact duplicate and structural overlap candidates
    And the normal `candidates` collection contains no E5 token-cap record
    And `candidate_count` equals the normal `candidates` collection length and excludes both cap records
    And `e5_token_cap_count` equals the `e5_token_cap_candidates` collection length of 2
    And the `e5_token_cap_candidates` collection contains endpoint-level `E5_TOKEN_CAP` records for both over-cap files and titles
    And each cap record reports its complete content hash, token count, limit 512, E5 model and tokenizer provenance, and `refactoring_required` true
    And the cap records are ordered by normalized file and title and give guidance to shorten or split the scenario and rerun the audit
    And no E5 semantic candidate contains either over-cap endpoint as either endpoint

  @givn.added @e2e
  Scenario: Explore keeps an E5 token-cap finding visible while fitting results remain usable
    Given an end-to-end permanent corpus with a fitting scenario and one complete E5 passage above 512 tokens
    And a complete fitting draft scenario is available for exploration
    And deterministic E5 token-budget fixtures identify the over-cap permanent scenario as requiring 550 tokens
    When the end-to-end user explores the draft with JSON output through the givn CLI
    Then the command exits 0
    And stdout is valid exploration JSON
    And the fitting permanent result retains its normal E5 rank and score fields
    And the normal exploration `results` collection contains no over-cap endpoint
    And `e5_token_cap_count` equals the `e5_token_cap_candidates` collection length of 1
    And the JSON contains a visible `E5_TOKEN_CAP` refactoring record for the over-cap permanent scenario in `e5_token_cap_candidates`
    And that cap record has its file, title, complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and guidance to shorten or split the scenario and rerun exploration

  @givn.added @e2e
  Scenario: Advisory review exposes an E5 token-cap finding without blocking
    Given an end-to-end project with one fitting permanent scenario and one active complete E5 passage above 512 tokens
    And deterministic E5 token-budget fixtures identify the active scenario as requiring 550 tokens
    When the end-to-end user runs advisory semantic change review for "running-change" through the givn CLI
    Then the command exits 0
    And stdout remains an advisory retrieval result
    And stdout exposes an additive `e5_token_cap_candidates` collection with count 1 for the active file and title
    And stdout reports the cap record's complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, `refactoring_required` true, and shorten-or-split guidance
    And stdout does not report the capped endpoint as a normal ranked E5 pair candidate

  @givn.added @e2e
  Scenario: Blocking review keeps an over-cap active scenario unresolved
    Given an end-to-end project with one fitting permanent scenario and one active complete E5 passage above 512 tokens
    And deterministic E5 token-budget fixtures identify the active scenario as requiring 550 tokens
    When the end-to-end user runs blocking change review for "running-change" through the givn CLI
    Then the command exits non-zero because the E5 token-cap finding is unresolved
    And the semantic review report has normal `Candidate count: 0` and an `e5_token_cap_count` of 1
    And the report's additive `e5_token_cap_candidates` collection contains an `E5_TOKEN_CAP` record with the active file and title
    And that cap record has its complete content hash, token count 550, limit 512, E5 model and tokenizer provenance, and `refactoring_required: true`
    And the report tells the author to shorten or split the scenario before rerunning review
    And the cap record has `classification: null` and `rationale: null` and no automatic `DUPLIKAT`, `VARIANTE`, or `VALID-BOUNDARY` classification

  @givn.added
  Scenario: Exactly 512 E5 passage tokens retain normal embedding behaviour
    Given a permanent corpus with fitting semantic scenarios
    And deterministic E5 token-budget fixtures assign exactly 512 tokens to every complete passage
    When the user runs `givn spec duplicates --format json` through the givn CLI
    Then the command exits 0
    And the JSON contains no `E5_TOKEN_CAP` finding
    And the `e5_token_cap_candidates` collection is empty and `e5_token_cap_count` is 0
    And the fitting scenarios retain the existing E5 candidate behaviour
