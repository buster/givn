# User Interaction Inventory:
# - Run `givn spec review --change <id>` under the unified rulebook
# - Run `givn check review --change <id>` twice without endpoint changes
# - Run `givn check review --change <id>` on a change with more candidate pairs than the pair cap
# - Run `givn archive --change <id>` on a completed change
# - Run `givn check review --change <id>` on a change with two new scenarios duplicating each other

@givn.delta @unified-semantic-rulebook

Feature: The unified semantic rulebook compares, selects, and judges pairs once per change

  @givn.added @rulebook @e2e @wip
  Scenario: Consumer review judges the selected pairs with both evidence models
    Given  a project prepared for semantic review with two similar active deltas
    When  the user runs the advisory semantic review command
    Then  the command exits 0
    And  the review output records exactly 1 unique pair and 2 directional calls per stage
    And  the review output records both second stages enabled
    And  the review output records no unscored candidates

  @givn.added @rulebook @e2e @wip
  Scenario: Consumer rerun reuses recorded pair judgements
    Given  a project prepared for semantic review with two similar active deltas
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the pair cache accounting reports "0" reused and "1" computed
    When  the user runs `givn check review --change running-change` again without edits
    Then  the command exits non-zero
    And  the pair cache accounting reports "1" reused and "0" computed
    And  the semantic review report still contains the candidate

  @givn.added @rulebook @e2e @wip
  Scenario: Consumer review caps the judged pairs at the pair cap
    Given  a project prepared for semantic review with more candidate pairs than the pair cap
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report candidate count is 20
    And  the review output records exactly 20 unique pairs and 40 directional calls per stage
    And  the semantic review report records no candidate as `BGE_UNSCORED`
    And  the semantic review report records no candidate as `NLI_UNSCORED`

  @givn.added @rulebook @e2e @wip
  Scenario: Consumer archive applies the rulebook to the current permanent tree
    Given  a completed semantic review change with a generated report and archive prerequisites
    When  the user runs `givn archive --change running-change`
    Then  the command exits 0
    And  the change is archived
    And  the archive gate output reports the pair cache accounting
    And  the archived semantic review report is retained

  @givn.added @rulebook @e2e @wip
  Scenario: Consumer review keeps a duplicate pair born inside the delta
    Given  a project prepared for semantic review with two similar active deltas and an unrelated permanent corpus
    When  the user runs `givn check review --change running-change`
    Then  the command exits non-zero
    And  the semantic review report contains scope "current-change"
    And  the candidate content and evidence hashes are exactly 16 lowercase hexadecimal characters

  @givn.added @rulebook @wip
  Scenario: A changed endpoint re-judges its recorded pair
    Given  a project prepared for semantic review with two similar active deltas
    When  the user runs `givn check review --change running-change`
    Then  the pair cache accounting reports "0" reused and "1" computed
    When  the active semantic candidate is changed
    And  the user runs `givn check review --change running-change` again without edits
    Then  the pair cache accounting reports "0" reused and "1" computed
