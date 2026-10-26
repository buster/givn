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
  Scenario: Two scenarios sharing a batch commit pass the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks two scenarios with the same batch revision
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits 0
    And  stdout says scenario commits passed

  @givn.added
  Scenario: A shared batch revision without a batch label fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks two scenarios with the same revision and no batch label
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr says a shared revision needs a batch label

  Scenario: A commit message that omits the scenario title fails the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks a scenario whose commit message omits the title
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits non-zero
    And  stderr says the commit does not name the scenario

  @givn.added
  Scenario: One real revision per scenario passes the task check
    Given  a clean committed givn project with a change whose delta specs pass
    And  the change's tasks.md checks one scenario with its own commit
    When  the user runs `givn check tasks --change verify-demo`
    Then  the command exits 0
    And  stdout says scenario commits passed
