@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  @givn.added @wip
  Scenario: A change whose declared E2E package is a standalone workspace passes the tasks check and binds the E2E scope
    Given an ise-effective change ready for change-scoped verification whose declared e2e package declares its own workspace, whose E2E runner command is "./run-tests.sh --e2e --package e2e", and whose delta has an E2E scenario
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits 0
    And the change verification receipt records a digest of every resolved e2e package
    And the change verification receipt records the E2E runner command digest
    And the change verification receipt records a passed E2E scope

  @givn.modified @wip
  Scenario Outline: An ise-effective change whose E2E runner command names no declared e2e root fails the tasks check
    Given  an ise-effective change ready for change-scoped verification whose E2E runner command is "<command>"
    When  the developer runs `givn check tasks --change ise-boundary`
    Then  the command exits non-zero
    And  stderr names the declared e2e root and the E2E runner command
    And  no change verification receipt is written
    Examples:
      | command |
      | ./run-tests.sh --bin other-target |
      | (empty) |
      | givn missing-e2e-runner |
      | ./run-tests.sh --e2e |
