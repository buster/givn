# User Interaction Inventory:
# - run the permanent duplicate audit with complete semantic-pair token-cap evidence
# - run a blocking change review with a complete semantic-pair token-cap finding

@givn.delta @semantic-token-cap-review
Feature: Token-cap evidence keeps complete semantic pairs reviewable

  @givn.added @bge @e2e
  Scenario: Permanent duplicate audit keeps partial BGE token-cap evidence per candidate
    Given an end-to-end permanent corpus with two E5 semantic candidates
    And one complete serialized pair encodes to exactly 512 BGE tokens
    And another complete serialized pair encodes to 513 BGE tokens
    And local BGE evidence is enabled with deterministic token-budget fixtures
    When the end-to-end user runs `givn spec duplicates --format json` through the givn CLI
    Then the command exits 0
    And the JSON response includes the 513-token pair in the `candidates` collection
    And that candidate's JSON `evidence.bge.status` is "BGE_TOKEN_CAP"
    And that candidate's JSON BGE evidence reports both scenario token counts, pair token count 513, and limit 512
    And the JSON response reports `bge_run.status` as "BGE_ENABLED"
    And the candidate is absent from the BGE filtered collection
    And the JSON response marks refactoring as required and instructs the agent to shorten or split the scenarios and rerun the audit

  @givn.added @nli @nli-combined
  Scenario: Permanent duplicate audit preserves BGE evidence when NLI reaches the cap
    Given a permanent corpus with two E5 semantic candidates
    And one complete serialized pair fits both BGE and NLI
    And another complete serialized pair fits BGE but encodes to 513 NLI tokens
    And local BGE and NLI evidence are enabled with deterministic token-budget fixtures
    When the user runs `givn spec duplicates --format json` through the givn CLI
    Then the command exits 0
    And the JSON response preserves a normal BGE evidence object for the 513-token NLI pair
    And that candidate's JSON `evidence.nli.status` is "NLI_TOKEN_CAP"
    And that candidate's JSON NLI evidence reports its pair token count as 513 and its limit as 512
    And the JSON response reports `nli_run.status` as "NLI_ENABLED"
    And the candidate is absent from the NLI filtered collection and the combined recommended collection
    And BGE and NLI evidence identify the same E5 candidate pool and endpoint pair
    And the JSON response instructs the agent to shorten or split the scenarios and rerun the audit

  @givn.added @nli @nli-combined @e2e
  Scenario: Blocking change review keeps an all-over-cap candidate unresolved
    Given an end-to-end project with one unresolved E5 semantic candidate
    And its complete serialized pair encodes to 513 BGE tokens and 513 NLI tokens
    And local BGE and NLI evidence are enabled with deterministic token-budget fixtures
    When the end-to-end user runs blocking change review for "running-change" through the givn CLI
    Then the command exits non-zero because the semantic candidate remains unresolved
    And the semantic review report records BGE status "BGE_UNAVAILABLE" with reason "token-cap" because no BGE pair was scoreable
    And the semantic review report records NLI status "NLI_UNAVAILABLE" with reason "token-cap" because no NLI pair was scoreable
    And the candidate records BGE admission "BGE_TOKEN_CAP" and NLI admission "NLI_TOKEN_CAP"
    And the semantic review report records both scenario token counts, each pair count of 513, and each limit of 512
    And the semantic review report tells the agent to shorten or split the scenarios before rerunning review
    And the candidate remains `UNCERTAIN` and has no automatic Givn classification

  @givn.added @nli @nli-combined
  Scenario: Exactly 512 tokens keeps normal second-stage behavior
    Given a permanent corpus with one E5 semantic candidate
    And its complete serialized pair encodes to exactly 512 BGE tokens and exactly 512 NLI tokens
    And local BGE and NLI evidence are enabled with deterministic token-budget fixtures
    When the user runs `givn spec duplicates --format json` through the givn CLI
    Then the command exits 0
    And the JSON response reports normal scored BGE and NLI evidence statuses for the candidate
    And the JSON response reports no BGE or NLI token-cap status or reason
    And the candidate receives the normal recommended or filtered evidence status
