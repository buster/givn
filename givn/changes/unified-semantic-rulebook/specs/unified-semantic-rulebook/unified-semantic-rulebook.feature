# User Interaction Inventory:
# - Run `givn spec review --change <id>` under the unified rulebook
# - Run `givn check review --change <id>` twice without endpoint changes
# - Run `givn check review --change <id>` on a change with more candidate pairs than the pair cap
# - Run `givn archive --change <id>` on a completed change
# - Run `givn check review --change <id>` on a change with two new scenarios duplicating each other

@givn.delta @unified-semantic-rulebook

Feature: The unified semantic rulebook compares, selects, and judges pairs once per change

  @givn.added @rulebook @e2e
  Scenario: Consumer review judges the selected pairs with both evidence models
    Given  a rulebook review project with a single admitted candidate
    When  the user runs the advisory semantic review command
    Then  the command exits 0
    And  the review output records exactly 1 unique pair and 2 directional calls per stage
    And  the review output records both second stages enabled
    And  the review output records no unscored candidates

  @givn.added @rulebook @e2e
  Scenario: Consumer rerun reuses recorded pair judgements
    Given  a rulebook review project with a single admitted candidate
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the pair cache accounting reports "0" reused and "1" computed
    When  the rulebook user reruns change review without edits
    Then  the command exits non-zero
    And  the pair cache accounting reports "1" reused and "0" computed
    And  the semantic review report contains a candidate

  @givn.added @rulebook @e2e
  Scenario: Consumer review caps the judged pairs at the pair cap
    Given  a rulebook review project with more candidate pairs than the pair cap
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report candidate count is 20
    And  the semantic review report records exactly 20 unique pairs and 40 directional calls per stage
    And  the semantic review report records no candidate as `BGE_UNSCORED`
    And  the semantic review report records no candidate as `NLI_UNSCORED`

  @givn.added @rulebook @e2e
  Scenario: Consumer archive applies the rulebook to the current permanent tree
    Given  a completed semantic review change with a generated report and archive prerequisites
    When  the user runs `givn archive --change running-change`
    Then  the command exits 0
    And  the archived rulebook change directory exists
    And  the archive gate output reports the pair cache accounting
    And  the archived semantic review report is retained

  @givn.added @rulebook @e2e
  Scenario: Consumer review keeps a duplicate pair born inside the delta
    Given  a rulebook review project with two similar active deltas and an unrelated permanent corpus
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains scope "delta-to-delta"
    And  the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters

  @givn.added @rulebook
  Scenario: A changed endpoint re-judges its recorded pair
    Given  a rulebook review project with a single admitted candidate
    When  the user runs `givn check review --change running-change`
    Then  the pair cache accounting reports "0" reused and "1" computed
    When  the rulebook user changes the active candidate
    And  the rulebook user reruns change review without edits
    Then  the pair cache accounting reports "0" reused and "1" computed

  @givn.added @rulebook
  Scenario: A changed permanent endpoint re-judges its recorded pair
    Given  a rulebook review project with a single admitted candidate
    When  the user runs `givn check review --change running-change`
    Then  the pair cache accounting reports "0" reused and "1" computed
    When  the rulebook user changes the permanent endpoint
    And  the rulebook user reruns change review without edits
    Then  the pair cache accounting reports "0" reused and "1" computed

  @givn.added @rulebook
  Scenario: The neighbour count cuts per-scenario pairs before the pair cap
    Given  a rulebook review project with one active delta and more neighbours than the neighbour count
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report candidate count is 5
    And  the semantic review report records no candidate as `BGE_UNSCORED`

  @givn.added @rulebook
  Scenario: Emergency budget truncation keeps outside candidates unscored and blocking
    Given  a rulebook review project with more candidate pairs than the pair cap
    And  an emergency pair budget below the selected pair count
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report records the outside candidates as `BGE_UNSCORED`
    And  the unscored candidates remain blocking with their scored neighbours

  @givn.added @rulebook
  Scenario: The pair cap fills deterministically when scores tie
    Given  a rulebook review project with more candidate pairs than the pair cap
    When  the user runs `givn check review --change running-change`
    And  the semantic review report is snapshotted
    When  the rulebook user reruns change review without edits
    Then  the semantic review report candidate count is 20
    And  both reports contain the same candidate ids in the same order

  @givn.added @rulebook
  Scenario: A change without candidates reports empty cache accounting
    Given  a project prepared for semantic review with active deltas below the similarity boundary
    When  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  the pair cache accounting reports "0" reused and "0" computed
