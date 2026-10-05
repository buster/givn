Feature: Enable commit attribution

  @e2e
  Scenario: Consumer sees the commit attribution installation report
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode,claude,omp`
    Then  the command exits 0
    And  stdout reports the shared Git commit attribution hook
    And  stdout reports the OpenCode attribution integration
    And  stdout reports the Claude Code attribution integration
    And  stdout reports the omp attribution integration

  @e2e
  Scenario: Consumer upgrades an existing project with attribution plumbing missing
    Given  an initialized Git project with attribution plumbing removed after its initial commit
    When  the consumer runs `givn upgrade`
    Then  the command exits 0
    And  stdout reports that commit attribution plumbing was refreshed
    And  the managed upgrade commit contains the checked-in hook and attribution integrations
  @e2e
  Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook
    Given  a Fossil checkout without a givn/ directory
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  stdout names the givn commit attribution path
    And  no .githooks directory exists
