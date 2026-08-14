# User Interaction Inventory:
# - Run `givn skills sync` to receive workflow guidance for the deterministic gates

@givn.delta @skill-rewrites

Feature: skill-rewrites

  @givn.added @e2e
  Scenario: Synchronized guidance explains how to resolve the deterministic gates
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then stdout contains "Syncing skills and agent commands..."
    And the generated givn-review guidance contains "Overlap dispositions"
    And the generated givn-review guidance contains "split-or-keep"
    And the generated givn-review guidance contains "removed+added"
    And the generated givn-design guidance contains "Black-Box-First"
    And the generated givn-steps guidance contains "which case this test covers"
    And generated guidance does not contain "Run `givn spec index`"

  @givn.added
  Scenario: Review instructions require deterministic gate dispositions
    Given a givn project
    When the user runs `givn instructions review --change skill-rewrites`
    Then stdout contains "Overlap"
    And stdout contains "split-or-keep"
    And stdout contains "removed+added"

  @givn.added
  Scenario: Design and step instructions require the Black-Box-First decision
    Given a givn project
    When the user reads the design and steps instructions for skill-rewrites
    Then combined instruction output contains "Black-Box-First"
    And combined instruction output contains "which case this test covers"

  @givn.added
  Scenario: Spec instructions use deterministic gates before retrieval exists
    Given a givn project
    When the user runs `givn instructions specs --change skill-rewrites`
    Then stdout contains "deterministic gates"
    And stdout does not contain "Run `givn spec index`"
