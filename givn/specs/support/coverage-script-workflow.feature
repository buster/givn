Feature: Project-owned coverage workflow

  @e2e
  Scenario: An initialized project documents reusable test and coverage scripts
    Given  a fresh givn project
    When  the user runs `givn init`
    Then  givn/commands.yaml contains `measure_command: "./measure-coverage.sh"`
    And  givn/commands.yaml contains `merge_command: "./merge-coverages.sh"`
    And  the README documents `./run-tests.sh`, `./measure-coverage.sh`, and `./merge-coverages.sh`

  @e2e
  Scenario: Archiving publishes line and branch badges from the merged report
    Given  a givn project with script-based coverage commands
    And  a completed change "merged-coverage" with all required artifacts
    When  the user archives the change
    Then  the archive succeeds
    And  the merged Cobertura report contains line coverage `95/100` and branch coverage `19/20`
    And  the README places line and branch badges directly below its title
    And  the README coverage summary contains line coverage `95% (95/100)` and branch coverage `95% (19/20)`
    And  the README coverage summary links the merged report and names the configured scripts

  Scenario: A failed merge command blocks archive and preserves the project
    Given  a givn project with a failing coverage merge command
    And  a completed change "merge-failure" with all required artifacts
    When  the user archives the change
    Then  the archive fails because coverage merging failed
    And  the change remains under givn/changes/
