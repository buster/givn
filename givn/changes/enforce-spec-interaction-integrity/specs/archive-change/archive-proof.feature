@givn.delta @archive-proof

Feature: Archive Proof

  @givn.added
  Scenario: Archive refuses a projected interaction-inventory mismatch before moving the change
    Given a givn project with a completed change "archive-projection" ready to archive
    And the change's delta adds an E2E scenario "Consumer cancels an order" without an interaction row
    When the user runs `givn archive --change archive-projection`
    Then the command exits non-zero
    And stderr contains "interaction inventory mismatch"
    And stderr contains "use case 'demo-cap'"
    And stderr contains "capability 'demo-cap'"
    And stderr contains "unlisted @e2e scenario 'Consumer cancels an order'"
    And the change directory still exists under givn/changes/
    And no archive candidate exists for "archive-projection"

  @givn.added
  Scenario: Archive publishes a consistent projected interaction inventory
    Given a givn project with a completed change "archive-projection" ready to archive
    And the change's delta adds an E2E scenario "Consumer cancels an order" with its interaction row
    When the user runs `givn archive --change archive-projection`
    Then the command exits 0
    And the change directory no longer exists under givn/changes/

  @givn.added
  Scenario: Archive refuses a merged candidate whose interaction inventory is inconsistent
    Given an unproven archive candidate "archive-projection" with a merged corpus and no verification scopes
    And the permanent interaction inventory does not declare an E2E scenario that the merged corpus contains
    When the user runs `givn archive --change archive-projection`
    Then the command exits non-zero
    And stderr contains "interaction inventory mismatch"
    And stderr contains "use case 'demo-cap'"
    And stderr contains "capability 'demo-cap'"
    And the candidate remains unproven and repairable in place
    And no verification scope ran for the candidate
