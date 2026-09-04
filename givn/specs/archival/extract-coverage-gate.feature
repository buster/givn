Feature: Extract Coverage Gate

  Scenario: Archive with coverage addon enabled still runs the coverage gate
    Given  a project root with givn/ initialised
    And  the givn/config.yaml has the coverage addon enabled with valid commands
    And  a completed change "cov-demo" with all required artifacts
    And the project declares a complete run declaration
    When  the user runs `givn archive --change cov-demo`
    Then  the command exits 0
    And  stdout contains "Coverage gate"
    And  the change directory has been moved to givn/archive/

  Scenario: Archive with coverage addon enabled still rolls back specs and README on coverage failure
    Given  a project root with givn/ initialised
    And  the givn/config.yaml has the coverage addon enabled but the non-e2e coverage command exits non-zero
    And  a completed change "cov-fail" with all required artifacts
    And the project declares a complete run declaration
    When  the user runs `givn archive --change cov-fail`
    Then  the command exits non-zero
    And  stdout contains "coverage"
    And  stderr mentions coverage gate failure
    And  the change directory still exists under givn/changes/

  @e2e
  Scenario: Consumer archives a change with coverage enabled and the coverage summary appears in README
    Given  a givn project with coverage enabled and valid coverage commands that produce Cobertura XML
    And  a completed change "cov-real" with all required artifacts
    And the project declares a complete run declaration
    When  the user invokes `givn archive --change cov-real`
    Then  the command succeeds
    And  the README.md contains a coverage summary fence
    And  the change directory has been moved to givn/archive/
