# User Interaction Inventory:
# - (inventory owned by the semantic-review capability; this delta only rewrites three scenarios in place)

@givn.delta @semantic-review

Feature: semantic-review

  @givn.modified
  Scenario: Change review generates a complete report for a semantic candidate
    Given  a rulebook review project with a single admitted candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report exists
    And  the semantic review report status is "BLOCKED"
    And  the semantic review report candidate count is 1
    And  the candidate embedding score has exactly four decimals in the range "0.9000" through "1.0000"
    And  the semantic review report contains deterministic facts

  @givn.modified
  Scenario: The report distinguishes permanent and current-change candidate scopes
    Given  a rulebook review project with two similar active deltas
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains scope "delta-to-permanent"
    And  the semantic review report contains scope "delta-to-delta"
    And  all candidate scores have exactly four decimals and are at least "0.9000"

  @givn.modified
  Scenario: A candidate at the exact E5 boundary is included
    Given  a project prepared for semantic review with a candidate scored exactly at the similarity boundary
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains a candidate
    And  the candidate embedding score is exactly "0.9000"
    And  the candidate id is exactly 16 lowercase hexadecimal characters
    And  the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters
