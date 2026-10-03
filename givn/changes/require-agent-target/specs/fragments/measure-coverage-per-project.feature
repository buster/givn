@givn.delta @measure-coverage-per-project

Feature: Measure coverage per project

  @givn.modified @e2e
  Scenario: An initialized project documents reusable test and coverage scripts
    Given a fresh givn project
    When the user runs `givn init --agents opencode`
    Then givn/commands.yaml contains `measure_command: "./measure-coverage.sh"`
    And givn/commands.yaml contains `merge_command: "./merge-coverages.sh"`
    And the README documents `./run-tests.sh`, `./measure-coverage.sh`, and `./merge-coverages.sh`
