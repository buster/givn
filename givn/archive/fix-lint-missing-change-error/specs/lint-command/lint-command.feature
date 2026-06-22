@givn.delta @lint-command
Feature: lint-command

  @givn.modified
  Scenario: A non-existent change id is not surfaced
    Given  a project root with givn/ initialised
    When  the user runs `givn lint --change does-not-exist`
    Then  the command exits 1
    And  stderr contains "does-not-exist"
