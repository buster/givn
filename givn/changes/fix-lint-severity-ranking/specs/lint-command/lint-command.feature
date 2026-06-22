@givn.delta @lint-command
Feature: lint-command

  @givn.removed @wip
  Scenario: A @wip finding outranks a parse error
    Given  a placeholder step

  @givn.added
  Scenario: A parse error outranks a @wip finding
    Given  a project root with givn/ initialised
    And  a .feature file at "givn/specs/aaa/aaa.feature" with:
      """
      Feature: Aaa
        @wip
        Scenario: W
          Given x
      """
    And  a .feature file at "givn/specs/zzz/zzz.feature" with:
      """
      This is not gherkin
      """
    When  the user runs `givn lint`
    Then  the command exits 1
    And  stderr contains "[WIP  ]"
    And  stderr contains "[ERROR]"
    And  stderr contains "givn lint: 2 file(s) checked, 2 finding(s)"
