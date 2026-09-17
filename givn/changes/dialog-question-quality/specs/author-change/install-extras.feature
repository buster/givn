@givn.delta @install-extras

Feature: install-extras

  @givn.added @wip
  Scenario: The absolute-mode policy block carries the dialog context rule
    Given a project root with givn/ initialised without addons
    And givn addon "absolute_mode" is "false"
    When the user runs `givn addons enable absolute_mode`
    Then the command exits 0
    And the file "AGENTS.md" contains "Context required to answer a question is not filler"
