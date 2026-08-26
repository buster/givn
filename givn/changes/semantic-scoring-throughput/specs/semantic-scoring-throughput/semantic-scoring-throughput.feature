# User Interaction Inventory:
# - run a semantic review with a configured thread budget and read the resolved budget on each model stage's status line

@givn.delta @semantic-scoring-throughput

Feature: Semantic model scoring honours one configured compute budget

  @givn.added @wip
  Scenario: An unset thread budget resolves to the machine's available parallelism
    Given a project prepared for semantic review whose second-stage evidence recommends nothing and filters everything
    When the user runs the advisory semantic review for "running-change"
    Then the command exits 0
    And each model stage status line names the machine's available parallelism as the thread count

  @givn.added @wip
  Scenario: Every stage status line reports the same configured thread budget
    Given a project prepared for semantic review whose second-stage evidence recommends nothing and filters everything
    And the model thread budget is "3"
    When the user runs the advisory semantic review for "running-change"
    Then the command exits 0
    And each model stage status line names the same thread count of 3

  @givn.added @wip
  Scenario: The embedder status line names the configured thread budget
    Given a project prepared for semantic review whose second-stage evidence recommends nothing and filters everything
    And the model thread budget is "6"
    When the user runs the advisory semantic review for "running-change"
    Then the command exits 0
    And the embedder status line names the same thread count of 6

  @givn.added @wip
  Scenario: A malformed thread budget is refused by name before any model work
    Given a project prepared for semantic review whose second-stage evidence recommends nothing and filters everything
    And the model thread budget is "not-a-number"
    When the user runs the advisory semantic review for "running-change"
    Then the command exits non-zero
    And stderr names the invalid thread budget setting
    And no model stage status line is printed

  @givn.added @wip
  Scenario: A zero thread budget is refused by name before any model work
    Given a project prepared for semantic review whose second-stage evidence recommends nothing and filters everything
    And the model thread budget is "0"
    When the user runs the advisory semantic review for "running-change"
    Then the command exits non-zero
    And stderr names the invalid thread budget setting
    And no model stage status line is printed

  @givn.added @wip
  Scenario: BGE evidence identity is unchanged across thread settings
    Given a project prepared for semantic review whose second-stage evidence recommends one pair and filters many
    And the model thread budget is "1"
    When the user runs the advisory semantic review for "running-change"
    Then the command exits non-zero
    When the user runs the advisory semantic review for "running-change" again with the model thread budget set to "8"
    Then the command exits non-zero
    And the second run reports identical candidate identities, admission labels, accounting counters, and label distributions as the first run

  @givn.added @e2e @wip
  Scenario: The real CLI reports the configured thread budget on its stage status lines
    Given an end-to-end project prepared for semantic review whose model stages are stubbed
    When the end-to-end user runs the advisory semantic review with the thread budget set to "3" through the givn CLI
    Then the command exits 0
    And each model stage status line in stdout names the same thread count of 3
