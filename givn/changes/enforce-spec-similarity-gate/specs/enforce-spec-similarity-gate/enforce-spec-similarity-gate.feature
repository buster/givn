# User Interaction Inventory:
# - Run `givn spec review --change <id>` to audit the permanent scenario tree.
# - Run `givn spec review --change <id>` to audit the current change against the permanent tree.
# - Run `givn archive --change <id>` after the similarity review has passed.

@givn.delta @enforce-spec-similarity-gate
Feature: Enforce semantic scenario similarity

  @givn.added @e2e @wip
  Scenario: Review blocks an unclassified permanent near-duplicate
    Given a givn project with permanent near-duplicate scenarios in different feature files
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "similarity candidate"
    And stdout contains "MERGE"
    And stdout contains "KEEP-AS-BOUNDARY"
    And stderr contains "similarity gate failed"
    And stdout contains "threshold 0.9500"
    And stdout contains "givn/specs/provider/provider.feature"

  @givn.added @wip
  Scenario: Review blocks a same-feature near-duplicate
    Given a givn project with permanent near-duplicate scenarios in one feature file
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "similarity candidate"
    And stdout contains "MERGE"
    And stdout contains "KEEP-AS-BOUNDARY"
    And stderr contains "similarity gate failed"

  @givn.added @wip
  Scenario: Review reports every permanent candidate beyond the top five
    Given a givn project with six permanent near-duplicate scenarios
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "Candidate six"

  @givn.added @wip
  Scenario: Review does not compare a modified scenario with its own predecessor
    Given a givn project with one permanent scenario for similarity review
    And the active change modifies that permanent scenario without changing its title
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout does not contain "similarity candidate"

  @givn.added @e2e @wip
  Scenario: Review blocks an unclassified current scenario against the permanent specifications
    Given a givn project with one permanent scenario for similarity review
    And the active change contains a scenario similar to the permanent scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "similarity candidate"
    And stdout contains "MERGE"
    And stdout contains "KEEP-AS-BOUNDARY"
    And stderr contains "similarity gate failed"

  @givn.added @wip
  Scenario: Review rejects a boundary scenario that remains too similar
    Given a givn project with one permanent scenario for similarity review
    And the active change contains a boundary-specific scenario similar to the permanent scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "rewrite"
    And stderr contains "similarity gate failed"

  @givn.added @wip
  Scenario: Review passes after a valid boundary scenario is rewritten
    Given a givn project with one permanent scenario for similarity review
    And the active change contains a rewritten boundary-specific scenario below the similarity threshold
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout does not contain "similarity candidate"

  @givn.added @wip
  Scenario: Review passes after a duplicate is removed from the projected permanent specifications
    Given a givn project with permanent near-duplicate scenarios in different feature files
    And the active change removes the duplicate scenario named "Unknown provider is rejected"
    When the user runs `givn spec review --change running-change`
    Then the command exits 0
    And stdout does not contain "similarity candidate"

  @givn.added @wip
  Scenario: Review cannot raise the fixed similarity threshold through project configuration
    Given a givn project with a near-duplicate pair above the fixed similarity threshold
    And the project requests a similarity threshold of 1.00
    And an active change delta contains a distinct scenario
    When the user runs `givn spec review --change running-change`
    Then the command fails
    And stdout contains "similarity candidate"

  @givn.added @e2e @wip
  Scenario: Accepted archive refreshes the permanent similarity index
    Given a givn project with a completed change configured for archive
    And the project's verify.command exits 0 on the current permanent specs
    When the user runs `givn archive --change integrity-hook-and-archive-gate`
    Then the command exits 0
    And the local spec index exists
    And the local spec index contains scenario title "Test scenario"
