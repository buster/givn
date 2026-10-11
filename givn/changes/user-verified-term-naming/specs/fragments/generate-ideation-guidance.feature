@givn.delta @generate-ideation-guidance

Feature: Generate ideation guidance

  @givn.added
  Scenario: The ideation guidance binds every confirmed term to the user's verified name
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "the user's verified name"

  @givn.added
  Scenario: The ideation guidance asks the naming question as soon as a name fails
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "generic, overloaded"
    And  the generated givn-ideation guidance contains "cannot fill its genus slot"
    And  the generated givn-ideation guidance contains "ask the naming question immediately"
    And  the generated givn-ideation guidance contains "is collision-checked like any other"

  @givn.added
  Scenario: The ideation guidance asks the term check before a phase closes
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "The user verifies every new core term before it is used"
    And  the generated givn-ideation guidance contains "Term check — <phase>"
    And  the generated givn-ideation guidance contains "Confirm all, or name the terms to revisit"
    And  the generated givn-ideation guidance contains "one-sentence description"
    And  the generated givn-ideation guidance contains "Confirm all makes them `confirmed`"
    And  the generated givn-ideation guidance contains "naming dialog, one at a time, before confirmation"
    And  the generated givn-ideation guidance contains "the other terms stand"
    And  the generated givn-ideation guidance contains "a replaced name becomes an anti-term"

  @givn.added
  Scenario: The verified-name gate holds without the addon
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "the user's verified name"
    And  the generated givn-ideation guidance contains "Term check — <phase>"

  @givn.added
  Scenario: The ideation command guardrails carry the verified-name rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation command contains "verifies every new core term"
    And  the generated givn-ideation command contains "phase term check"
    And  the generated givn-ideation command contains "naming dialog"
    And  the generated givn-ideation command contains "before the canvas, use cases, or seeds rely on it"
