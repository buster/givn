@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  @givn.added @e2e
  Scenario: Consumer runs check tasks and receives a change verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    When  the user invokes `givn check tasks --change verify-demo`
    Then  the command succeeds
    And  the change directory contains a passed verification receipt naming the tested revision

  @givn.added
  Scenario: Review refuses a change without a verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    And  the change has no verification receipt
    When  the user runs `givn check review --change verify-demo`
    Then  the command exits non-zero
    And  stderr routes to `givn check tasks --change verify-demo`

  @givn.added
  Scenario: Review refuses a receipt after the delta specs changed
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    And  the change has a current verification receipt
    And  the delta specs changed after the receipt
    When  the user runs `givn check review --change verify-demo`
    Then  the command exits non-zero
    And  stderr says the verification receipt is stale

  @givn.added
  Scenario: Review accepts a current verification receipt
    Given  a clean committed givn project with a change whose delta specs pass
    And  the project declares a complete run declaration
    And  the change has a current verification receipt
    When  the user runs `givn check review --change verify-demo`
    Then  the command exits 0
    And  stdout contains "change verification receipt passed"

  @givn.added
  Scenario: The result helper writes four counts without a scope
    Given  a givn project with a result file path
    When  the user runs the result helper for 3 total and 3 passed
    Then  the command exits 0
    And  the result file contains total 3, passed 3, failed 0 and skipped 0 without a scope
