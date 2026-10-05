@givn.delta @enable-commit-attribution

Feature: Enable commit attribution

  @givn.modified @e2e
  Scenario: Consumer sees the commit attribution installation report
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode,claude,omp`
    Then  the command exits 0
    And  stdout reports the shared Git commit attribution hook
    And  stdout reports the OpenCode attribution integration
    And  stdout reports the Claude Code attribution integration
    And  stdout reports the omp attribution integration
