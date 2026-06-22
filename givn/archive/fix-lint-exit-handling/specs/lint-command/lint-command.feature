@givn.delta @lint-command
Feature: lint-command

  @givn.added
  Scenario: The lint exit-code contract is preserved after refactor
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/clean/clean.feature" with:
      """
      Feature: Clean
        Scenario: S
          Given x
      """
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"
