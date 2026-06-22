@givn.delta @lint-command
Feature: lint-command

  @givn.added
  Scenario: A removed placeholder suppresses the @wip warning on itself
    Given  a project root with givn/ initialised
    And  a feature file with a removed-wip placeholder at "givn/specs/foo/foo.feature"
    When  the user runs `givn lint`
    Then  the command exits 0
    And  stdout contains a line "givn lint: 1 file(s) checked — clean"

  @givn.added
  Scenario: A removed scenario does not suppress an unrelated later @wip scenario
    Given  a project root with givn/ initialised
    And  a feature file with a removed scenario and a separate wip scenario at "givn/specs/foo/foo.feature"
    When  the user runs `givn lint`
    Then  the command exits 2
    And  stderr contains "[WIP  ]"
    And  stderr contains "scenario 'NewNotDone' is tagged @wip"
