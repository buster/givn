@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  @givn.added
  Scenario: A checked scenario without a commit fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks a scenario without a commit
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr names the scenario and the missing commit

  @givn.added
  Scenario: A checked scenario reusing another revision fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks two scenarios with the same revision
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr says each scenario needs its own commit
