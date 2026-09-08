@givn.delta @givn-workflow-enforcement
Feature: Migration coverage enforcement

  @givn.added @wip
  Scenario: A consumer runs migration coverage and sees a passing receipt
    Given a migration evidence change with matching behavior snapshots
    When the user runs `givn check migration --change migration-test`
    Then the command exits 0
    And stdout contains "migration coverage passed"
    And the migration comparison is marked passed

  @givn.added @wip
  Scenario: Missing behavior blocks migration coverage
    Given a migration evidence change whose after snapshot is missing one behavior
    When the user runs `givn check migration --change migration-test`
    Then the command exits non-zero
    And stderr contains "migration coverage regression"
    And the migration comparison is marked failed

  @givn.added @wip
  Scenario: Lost E2E evidence blocks migration coverage
    Given a migration evidence change that loses E2E evidence
    When the user runs `givn check migration --change migration-test`
    Then the command exits non-zero
    And stderr contains "migration coverage regression"

  @givn.added @wip
  Scenario: Lower interaction coverage blocks migration coverage
    Given a migration evidence change with lower interaction coverage
    When the user runs `givn check migration --change migration-test`
    Then the command exits non-zero
    And stderr contains "migration coverage regression"

  @givn.added @wip
  Scenario: Lower line coverage blocks migration coverage
    Given a migration evidence change with lower line coverage
    When the user runs `givn check migration --change migration-test`
    Then the command exits non-zero
    And stderr contains "migration coverage regression"

  @givn.added @wip
  Scenario: Lower branch coverage blocks migration coverage
    Given a migration evidence change with lower branch coverage
    When the user runs `givn check migration --change migration-test`
    Then the command exits non-zero
    And stderr contains "migration coverage regression"

  @givn.added @wip
  Scenario: Explicit retirement documents a changed behavior
    Given a migration evidence change with an explicitly retired behavior
    When the user runs `givn check migration --change migration-test`
    Then the command exits 0
    And stdout contains "migration coverage passed"

  @givn.added @e2e @wip
  Scenario: Consumer runs migration coverage check and sees passed evidence
    Given a migration evidence change with matching behavior snapshots
    When the consumer runs the migration coverage check
    Then the command exits 0
    And stdout contains "migration coverage passed"
