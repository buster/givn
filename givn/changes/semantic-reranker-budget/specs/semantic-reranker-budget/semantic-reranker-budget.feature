# User Interaction Inventory:
# - run a permanent duplicate audit with a configured second-stage pair budget through the givn CLI

@givn.delta @semantic-reranker-budget
Feature: Configurable second-stage candidate budget

  @givn.added @bge
  Scenario: The default second-stage budget scores every candidate below 2048
    Given a permanent corpus with three ordered E5 semantic candidates
    And local BGE evidence is enabled
    When the user runs `givn spec duplicates --format json`
    Then the JSON reports a second-stage pair budget of 2048
    And the JSON reports three scored BGE pairs and zero BGE unscored candidates

  @givn.added @bge
  Scenario: BGE receives the highest E5 candidates within the configured budget
    Given a permanent corpus with three ordered E5 semantic candidates
    And local BGE evidence is enabled
    And the second-stage pair budget is 2
    When the user runs `givn spec duplicates --format json`
    Then the JSON reports a second-stage pair budget of 2
    And BGE scores the two highest E5 candidates
    And the lowest-E5 candidate is recorded as BGE_UNSCORED
    And the BGE run is enabled rather than unavailable for pair-cap

  @givn.added @nli
  Scenario: NLI receives the highest E5 candidates within the configured budget
    Given a permanent corpus with three ordered E5 semantic candidates
    And NLI evidence is enabled while BGE evidence is disabled
    And the second-stage pair budget is 2
    When the user runs `givn spec duplicates --format json`
    Then the JSON reports a second-stage pair budget of 2
    And NLI scores the two highest E5 candidates
    And the lowest-E5 candidate is recorded as NLI_UNSCORED
    And the NLI run is enabled rather than unavailable for pair-cap

  @givn.added @bge
  Scenario: An invalid second-stage pair budget is rejected
    Given a permanent corpus with three ordered E5 semantic candidates
    And local BGE evidence is enabled
    And the second-stage pair budget is "not-a-number"
    When the user runs `givn spec duplicates --format json`
    Then the command exits non-zero
    And stderr identifies the invalid second-stage pair budget

  @givn.added @bge
  Scenario: A zero second-stage pair budget is rejected
    Given a permanent corpus with three ordered E5 semantic candidates
    And local BGE evidence is enabled
    And the second-stage pair budget is "0"
    When the user runs `givn spec duplicates --format json`
    Then the command exits non-zero
    And stderr identifies the invalid second-stage pair budget

  @givn.added @nli-combined
  Scenario: BGE and NLI select the same highest E5 candidates independently
    Given a permanent corpus with three ordered E5 semantic candidates
    And local BGE and NLI evidence are enabled
    And the second-stage pair budget is 2
    When the user runs `givn spec duplicates --format json`
    Then BGE and NLI each score the two highest E5 candidates
    And BGE and NLI each record the lowest-E5 candidate as unscored
    And the combined recommendation uses the intersection of both model recommendations

  @givn.added @bge @e2e
  Scenario: The CLI reports the configured second-stage pair budget
    Given a permanent corpus with three ordered E5 semantic candidates
    And local BGE evidence is enabled
    When the end-to-end user runs the permanent duplicate audit with JSON output through the givn CLI
    Then the command exits 0
    And stdout reports a second-stage pair budget of 2048
    And stdout reports that all three E5 candidates were scored by BGE
