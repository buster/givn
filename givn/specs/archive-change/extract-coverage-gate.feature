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

  Scenario: A coverage failure leaves the archive candidate unproven
    Given  a givn project with the coverage addon enabled but the non-e2e coverage command exits non-zero
    And  a completed change "cov-fail" ready to archive
    When  the user runs `givn archive --change cov-fail`
    Then  the command exits non-zero
    And  stderr mentions coverage gate failure
    And  the archive at givn/archive/cov-fail/ is unproven
    And  the merged permanent specs preserve the delta scenarios
    And  no archive backup exists for "cov-fail"

  @extract-coverage-gate
  Scenario: A coverage failure proves on rerun after the repair
    Given  an unproven archive candidate whose verification scopes passed but coverage failed
    And  the coverage failure is repaired without changing the scopes' relevant inputs
    When  the developer runs `givn archive --change cov-repair`
    Then  coverage is measured again and fresh valid reports allow the archive to become proven
    And  the passed verification scopes are reused without another suite execution

  @extract-coverage-gate
  Scenario: Equal coverage from honest scope executions does not block proof
    Given  a completed change is ready to archive with coverage enabled
    And  configured regular and end-to-end coverage executions each write fresh valid reports covering identical source lines
    When  the developer runs `givn archive --change cov-demo`
    Then  coverage equality does not block the proven archive
    And  no percentage threshold or additional provenance requirement replaces the equality gate
    And  each configured coverage measurement executes once without an extra isolation run

  @extract-coverage-gate
  Scenario: A successful coverage command cannot reuse an old report
    Given  an archive candidate has old valid coverage reports
    And  coverage is required and its command exits successfully without writing new reports
    When  the developer runs `givn archive --change cov-demo`
    Then  the old measurement inputs were removed before coverage execution
    And  the candidate remains unproven because fresh coverage reports are missing

  @extract-coverage-gate
  Scenario Outline: Fresh coverage reports must be valid
    Given  an archive candidate is ready for coverage measurement
    And  the coverage command exits successfully but produces <report>
    When  the developer runs `givn archive --change cov-demo`
    Then  the command exits non-zero and names the invalid coverage output
    And  the candidate remains unproven
    Examples:
      | report |
      | malformed coverage output |
      | an empty report |

  @extract-coverage-gate
  Scenario: Evidence-only finalization retry reuses valid coverage
    Given  an archive candidate has fresh valid coverage and passed verification evidence
    And  only finalization publication failed
    And  relevant measurement inputs remain unchanged
    When  the developer retries `givn archive --change cov-demo`
    Then  the archive becomes proven using its recorded coverage
    And  neither coverage nor a verification suite executes again
