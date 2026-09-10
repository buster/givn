@givn.delta @coverage-script-workflow
Feature: Project-owned coverage workflow

  @givn.removed @wip
  Scenario: A failed merge command blocks archive and preserves the project
    Given a placeholder step

  @givn.added @wip
  Scenario: A failed coverage merge leaves the archive candidate unproven
    Given a givn project with a failing coverage merge command
    And a completed change "merge-failure" with all required artifacts
    When the user archives the change
    Then the archive fails because coverage merging failed
    And the archive at givn/archive/merge-failure/ is unproven
