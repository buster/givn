@givn.delta @outline-report

Feature: Outline report

  @givn.removed
  Scenario: The outline experiment is disabled by default
    Given a placeholder step

  @givn.added
  Scenario: The outline experiment is enabled by default
    Given a Givn project
    When the developer runs "givn addons list"
    Then stdout contains "[enabled ]  outline"

  @givn.added
  Scenario: A project can explicitly disable the outline experiment
    Given a Givn project
    When the user runs `givn addons disable outline`
    Then the command exits 0
    When the developer runs "givn addons list"
    Then the outline addon is listed as disabled
