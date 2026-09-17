@givn.delta @lint-command

Feature: lint-command

  @givn.added
  Scenario: A dangling persona reference is reported as an advisory
    Given a project root with givn/ initialised
    And a change use-case document at "givn/changes/persona-change/specs/persona-change/usecase.md" records the persona "missing-persona"
    And the change use-case document also records the persona "known-persona"
    And the change use-case document also records the persona "topic-persona"
    And a project persona file "known-persona" exists
    And a topic persona file at "givn/ideation/topic-a/personas/topic-persona.md" exists
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[PERS ]"
    And stderr contains "missing-persona"
    And stderr does not contain "known-persona"
    And stderr does not contain "topic-persona"

  @givn.added @wip
  Scenario: The change scope reports a dangling persona reference
    Given a project root with givn/ initialised
    And a change use-case document at "givn/changes/persona-change/specs/persona-change/usecase.md" records the persona "missing-persona"
    When the user runs `givn lint --change persona-change`
    Then the command exits 0
    And stderr contains "[PERS ]"
    And stderr contains "missing-persona"
