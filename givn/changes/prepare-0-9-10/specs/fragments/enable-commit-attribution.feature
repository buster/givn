@givn.delta @enable-commit-attribution
Feature: Accurate attribution installation reports

  @givn.modified @e2e
  Scenario: Consumer sees the commit attribution installation report
    Given a new Git project with `core.hooksPath` set to `.githooks` and a user commit-msg hook
    When the consumer runs `givn init --agents opencode,claude,omp`
    Then the command exits 0
    And stdout reports the shared Git hook and each selected harness attribution integration
    And the report states that the existing user hook was preserved
    And the installed hook does not dispatch itself

  @givn.modified @e2e
  Scenario: Consumer upgrades an existing project with attribution plumbing missing
    Given a clean initialized Git project using `.githooks` with attribution plumbing missing and a preserved user hook
    When the consumer runs `givn upgrade`
    Then the command exits 0 and reports refreshed attribution plumbing
    And the managed upgrade revision contains the hook and selected attribution integrations
    And a repeated installation preserves the user hook without duplicating dispatch

  @givn.modified @e2e
  Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook
    Given a Fossil checkout without a givn/ directory
    When the consumer initializes Givn with an explicit agent target
    Then the command exits 0
    And stdout names `givn commit` with explicit file paths as the attribution path
    And no .githooks directory exists
    And generated guidance states that raw Fossil commits are not stamped
