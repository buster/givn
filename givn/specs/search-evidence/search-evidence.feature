Feature: Optional cross-encoder evidence for spec search

  @nli-combined @e2e
  Scenario: Search reports BGE and NLI evidence through the givn CLI
    Given  an end-to-end project with a permanent spec index for evidence search
    And  BGE and NLI evidence are enabled for the end-to-end search
    And  BGE and NLI evidence recommend the listed candidates
    When  the end-to-end user searches "the command rejects an unknown provider" with evidence
    Then  the command exits 0
    And  stdout contains "rank 1"
    And  stdout contains "BGE_RECOMMENDED"
    And  stdout contains "NLI_RECOMMENDED"

  Scenario: Search without the evidence flag prints no evidence lines
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local BGE evidence is enabled
    And  local NLI evidence is enabled
    When  the user searches for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "rank 1"
    And  stdout does not contain "BGE_RECOMMENDED"
    And  stdout does not contain "NLI_RECOMMENDED"
    And  stdout does not contain "BGE evidence: unavailable"
    And  stdout does not contain "NLI evidence: unavailable"

  @bge
  Scenario: Evidence search reports a BGE verdict for every listed candidate
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local BGE evidence is enabled
    And  BGE evidence recommends all listed candidates
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "BGE_RECOMMENDED"
    And  stdout contains "symmetric raw"
    And  stdout contains "forward"
    And  stdout contains "reverse"

  @nli
  Scenario: Evidence search reports an NLI verdict for every candidate
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local NLI evidence is enabled
    And  NLI evidence recommends all listed candidates
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "NLI_RECOMMENDED"
    And  stdout contains "entailment"
    And  stdout contains "neutral"
    And  stdout contains "contradiction"

  @nli
  Scenario: Absent BGE evidence is reported but never presented as a verdict
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local NLI evidence is enabled
    And  NLI evidence recommends all listed candidates
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "NLI_RECOMMENDED"
    And  stdout contains "BGE evidence: unavailable"
    And  stdout does not contain "BGE_RECOMMENDED"
    And  stdout does not contain "BGE_FILTERED"

  Scenario: Evidence search without any cross-encoder answers with the plain ranking
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "rank 1"
    And  stdout contains "BGE evidence: unavailable"
    And  stdout contains "NLI evidence: unavailable"
    And  stdout does not contain "BGE_RECOMMENDED"
    And  stdout does not contain "BGE_FILTERED"
    And  stdout does not contain "NLI_RECOMMENDED"
    And  stdout does not contain "NLI_FILTERED"

  @bge
  Scenario: Candidates beyond the evidence pair budget stay unscored
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local BGE evidence is enabled
    And  BGE evidence recommends all listed candidates
    And  the second-stage pair budget is 2
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "BGE_RECOMMENDED"
    And  stdout contains "BGE_UNSCORED"
    And  stdout does not contain "BGE_FILTERED"

  @bge
  Scenario: An over-limit search pair receives BGE token-cap evidence
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local BGE evidence is enabled
    And  every BGE pair exceeds the token limit
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "BGE_TOKEN_CAP"
    And  stdout contains "limit"
    And  stdout does not contain "BGE_FILTERED"

  @nli
  Scenario: An over-limit search pair receives NLI token-cap evidence
    Given  a givn project with permanent scenarios for retrieval
    And  the local spec index has been built
    And  local NLI evidence is enabled
    And  every NLI pair exceeds the token limit
    When  the user searches with evidence for "the command rejects an unknown provider"
    Then  the command exits 0
    And  stdout contains "NLI_TOKEN_CAP"
    And  stdout contains "limit"
    And  stdout does not contain "NLI_FILTERED"
