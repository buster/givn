@givn.delta @extract-coverage-gate
Feature: Extract coverage gate

  @givn.modified @wip
  Scenario: A coverage failure proves on rerun after the repair
    Given an unproven archive candidate whose verification scopes passed but coverage failed
    And the coverage failure is repaired without changing the scopes' relevant inputs
    When the developer runs `givn archive --change cov-repair`
    Then coverage is measured again and fresh valid reports allow the archive to become proven
    And the passed verification scopes are reused without another suite execution

  @givn.added @wip
  Scenario: Equal coverage from honest scope executions does not block proof
    Given a completed change is ready to archive with coverage enabled
    And configured regular and end-to-end coverage executions each write fresh valid reports covering identical source lines
    When the developer runs `givn archive --change cov-demo`
    Then coverage equality does not block the proven archive
    And no percentage threshold or additional provenance requirement replaces the equality gate
    And each configured coverage measurement executes once without an extra isolation run

  @givn.added @wip
  Scenario: A successful coverage command cannot reuse an old report
    Given an archive candidate has old valid coverage reports
    And coverage is required and its command exits successfully without writing new reports
    When the developer runs `givn archive --change cov-demo`
    Then the old measurement inputs were removed before coverage execution
    And the candidate remains unproven because fresh coverage reports are missing

  @givn.added @wip
  Scenario Outline: Fresh coverage reports must be valid
    Given an archive candidate is ready for coverage measurement
    And the coverage command exits successfully but produces <report>
    When the developer runs `givn archive --change cov-demo`
    Then the command exits non-zero and names the invalid coverage output
    And the candidate remains unproven
    Examples:
      | report |
      | malformed coverage output |
      | an empty report |

  @givn.added @wip
  Scenario: Evidence-only finalization retry reuses valid coverage
    Given an archive candidate has fresh valid coverage and passed verification evidence
    And only finalization publication failed
    And relevant measurement inputs remain unchanged
    When the developer retries `givn archive --change cov-demo`
    Then the archive becomes proven using its recorded coverage
    And neither coverage nor a verification suite executes again
