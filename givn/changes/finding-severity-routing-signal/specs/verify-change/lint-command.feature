@givn.delta @lint-command

Feature: lint-command

  @givn.added @wip
  Scenario: A blocking finding renders its severity
    Given a project root with givn/ initialised
    And a .feature file at "givn/specs/bad/bad.feature" with:
      """
      This is not gherkin
      """
    When the user runs `givn lint`
    Then the command exits 1
    And stderr contains "[ERROR] blocking"

  @givn.added @wip
  Scenario: An advisory finding renders its severity and keeps the exit code
    Given a project root with givn/ initialised
    And a .feature file at "givn/specs/aaa/aaa.feature" with:
      """
      Feature: Aaa
        Scenario: Alpha
          Given x
          Then y
      """
    And a .feature file at "givn/specs/bbb/bbb.feature" with:
      """
      Feature: Bbb
        Scenario: Beta
          Given x
          Then y
      """
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[SHAPE] advisory"

  @givn.added @wip
  Scenario: The JSON output exposes the finding severity
    Given a project root with givn/ initialised
    And a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        @wip
        Scenario: Not done
          Given x
      """
    When the user runs `givn lint --json`
    Then the command exits 2
    And stdout mentions "\"severity\": \"finding\""
    When the user runs `givn lint`
    Then the command exits 2
    And stderr contains "[WIP  ] finding"
