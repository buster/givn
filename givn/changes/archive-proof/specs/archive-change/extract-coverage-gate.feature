@givn.delta @extract-coverage-gate
Feature: Extract Coverage Gate

  @givn.modified
  Scenario: Archive with coverage addon enabled still runs the coverage gate
    Given a givn project with the coverage addon enabled and valid coverage commands
    And a completed change "cov-demo" ready to archive
    When the user runs `givn archive --change cov-demo`
    Then the command exits 0
    And stdout contains "Coverage gate"
    And the archive at givn/archive/cov-demo/ is proven
    And the verification receipt records the coverage result

  @givn.removed
  Scenario: Archive with coverage addon enabled still rolls back specs and README on coverage failure
    Given a placeholder step

  @givn.added
  Scenario: A coverage failure leaves the archive candidate unproven
    Given a givn project with the coverage addon enabled but the non-e2e coverage command exits non-zero
    And a completed change "cov-fail" ready to archive
    When the user runs `givn archive --change cov-fail`
    Then the command exits non-zero
    And stderr mentions coverage gate failure
    And the archive at givn/archive/cov-fail/ is unproven
    And the merged permanent specs preserve the delta scenarios
    And no archive backup exists for "cov-fail"

  @givn.added
  Scenario: A coverage failure proves on rerun after the repair
    Given an unproven archive candidate "cov-repair" whose coverage gate failed
    And the developer commits a repair
    And the coverage commands now succeed
    When the user runs `givn archive --change cov-repair`
    Then the command exits 0
    And the archive at givn/archive/cov-repair/ is proven
