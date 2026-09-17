@givn.delta @lint-command

Feature: lint-command

  @givn.added
  Scenario: A dangling persona reference is reported as an advisory
    Given a project root with givn/ initialised
    And a change use-case document at "givn/changes/persona-change/specs/persona-change/usecase.md" records the persona "missing-persona"
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[PERS ]"
    And stderr contains "missing-persona"

  @givn.added @wip
  Scenario: A persona reference resolved by a project persona is not reported
    Given a project root with givn/ initialised
    And a change use-case document at "givn/changes/persona-change/specs/persona-change/usecase.md" records the persona "known-persona"
    And a project persona file "known-persona" exists
    When the user runs `givn lint`
    Then the command exits 0
    And stdout contains a line "givn lint: no .feature files found under givn/"

  @givn.added @wip
  Scenario: A persona reference resolved by a topic persona is not reported
    Given a project root with givn/ initialised
    And a change use-case document at "givn/changes/persona-change/specs/persona-change/usecase.md" records the persona "topic-persona"
    And a topic persona file at "givn/ideation/topic-a/personas/topic-persona.md" exists
    When the user runs `givn lint`
    Then the command exits 0
    And stdout contains a line "givn lint: no .feature files found under givn/"

  @givn.added @wip
  Scenario: The change scope reports a dangling persona reference
    Given a project root with givn/ initialised
    And a change use-case document at "givn/changes/persona-change/specs/persona-change/usecase.md" records the persona "missing-persona"
    When the user runs `givn lint --change persona-change`
    Then the command exits 0
    And stderr contains "[PERS ]"
    And stderr contains "missing-persona"
