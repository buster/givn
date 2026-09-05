# User Interaction Inventory:
# - Action: archive a change with coverage configured, via the archive command
#   (real interface: givn CLI). Covered by the existing @e2e scenario
#   "Consumer archives a change with coverage enabled and the coverage summary
#   appears in README" (extract-coverage-gate); the scenarios below exercise
#   the same action's merge contract through the same CLI as regular
#   scenarios, so no new interaction is declared.

@givn.delta @extract-coverage-gate

Feature: Extract Coverage Gate

  @givn.added
  Scenario: A line measured by both reports is counted once in the merged report
    Given a givn project with legacy coverage commands whose reports measure the same lines
    And a completed change "cov-union" with all required artifacts
    And the project declares a complete run declaration
    When the user runs `givn archive --change cov-union`
    Then the command exits 0
    And the merged Cobertura report contains line coverage `4/4`

  @givn.added
  Scenario: A line only one report measures counts once and uncovered lines are not dropped
    Given a givn project with legacy coverage commands whose second report measures lines the first does not
    And a completed change "cov-union-partial" with all required artifacts
    And the project declares a complete run declaration
    When the user runs `givn archive --change cov-union-partial`
    Then the command exits 0
    And the merged Cobertura report contains line coverage `2/4`
