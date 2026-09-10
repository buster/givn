Feature: Extract Coverage Gate

  Scenario: Archive with coverage addon enabled still runs the coverage gate
    Given  a givn project with the coverage addon enabled and valid coverage commands
    And  a completed change "cov-demo" ready to archive
    When  the user runs `givn archive --change cov-demo`
    Then  the command exits 0
    And  stdout contains "Coverage gate"
    And  the archive at givn/archive/cov-demo/ is proven
    And  the verification receipt records the coverage result

  @e2e
  Scenario: Consumer archives a change with coverage enabled and the coverage summary appears in README
    Given a givn project with coverage enabled and valid coverage commands that produce Cobertura XML
    And a completed change "cov-real" with all required artifacts
    And the project declares a complete run declaration
    When the user invokes `givn archive --change cov-real`
    Then the command succeeds
    And the README.md contains a coverage summary fence
    And the change directory has been moved to givn/archive/
  Scenario: A line measured by both reports is counted once in the merged report
    Given  a givn project with legacy coverage commands whose reports measure the same lines
    And  a completed change "cov-union" with all required artifacts
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change cov-union`
    Then  the command exits 0
    And  the merged Cobertura report contains line coverage `4/4`

  Scenario: A line only one report measures counts once and uncovered lines are not dropped
    Given  a givn project with legacy coverage commands whose second report measures lines the first does not
    And  a completed change "cov-union-partial" with all required artifacts
    And  the project declares a complete run declaration
    When  the user runs `givn archive --change cov-union-partial`
    Then  the command exits 0
    And  the merged Cobertura report contains line coverage `2/4`
  Scenario: A coverage failure leaves the archive candidate unproven
    Given  a givn project with the coverage addon enabled but the non-e2e coverage command exits non-zero
    And  a completed change "cov-fail" ready to archive
    When  the user runs `givn archive --change cov-fail`
    Then  the command exits non-zero
    And  stderr mentions coverage gate failure
    And  the archive at givn/archive/cov-fail/ is unproven
    And  the merged permanent specs preserve the delta scenarios
    And  no archive backup exists for "cov-fail"

  Scenario: A coverage failure proves on rerun after the repair
    Given  an unproven archive candidate "cov-repair" whose coverage gate failed
    And  the developer commits a repair
    And  the coverage commands now succeed
    When  the user runs `givn archive --change cov-repair`
    Then  the command exits 0
    And  the archive at givn/archive/cov-repair/ is proven
