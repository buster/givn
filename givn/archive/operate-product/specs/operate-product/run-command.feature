@givn.delta @run-command
Feature: Run command

  @givn.modified
  Scenario: The upgrade commit includes the backfilled run declaration
    Given a git-managed project with givn initialised
    And the commands file has no run declaration
    When the user runs `givn upgrade`
    Then the command exits 0
    And the working tree is clean
    And the latest revision contains "givn/commands.yaml"

  @givn.modified
  Scenario: A repeated upgrade creates no additional commit
    Given a git-managed project with givn initialised
    And the commands file has no run declaration
    When the user runs `givn upgrade`
    And the user runs `givn upgrade` again
    Then the command exits 0
    And the latest revision is unchanged by the second upgrade
