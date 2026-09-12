@givn.delta @visual-design-contract

Feature: Visual Design Contract

  @givn.added @e2e @wip
  Scenario: Author requests the visual design contract guidance
    Given a givn project
    When the user runs `givn instructions design --change visual-ui-review`
    Then stdout contains "Visual Design Contract"
    And stdout contains "style-level"
    And stdout contains "not pixel"
    And stdout contains "docs/design/"
    And stdout contains "design tokens"
    And stdout contains "component states"
    And stdout contains "rendered reference"

  @givn.added
  Scenario: Design guidance names the default viewports
    Given a givn project
    When the user runs `givn instructions design --change visual-ui-review`
    Then stdout contains "Samsung S24"
    And stdout contains "360x780"
    And stdout contains "Full HD"
    And stdout contains "1920x1080"

  @givn.added @wip
  Scenario: Design guidance keeps improvements inside the style
    Given a givn project
    When the user runs `givn instructions design --change visual-ui-review`
    Then stdout contains "improvement"
    And stdout contains "updates the contract"

  @givn.added @e2e @wip
  Scenario: Synchronized guidance defines the contract approval and capture mechanics
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-design-review guidance contains "Visual Design Contract"
    And the generated givn-design-review guidance contains "propose"
    And the generated givn-design-review guidance contains "user approves"
    And the generated givn-design-review guidance contains "rendered options"
    And the generated givn-steps guidance contains "evidence/visual"
    And the generated givn-steps guidance contains "final-state"
    And the generated givn-steps guidance contains "animations are frozen"
    And the generated givn-steps guidance contains "reports/"

  @givn.added @wip
  Scenario: Design-review guidance requires an approved contract
    Given a givn project
    When the user runs `givn instructions design-review --change visual-ui-review`
    Then stdout contains "Visual Design Contract"
    And stdout contains "propose"
    And stdout contains "user approves"
    And stdout contains "rendered options"

  @givn.added @wip
  Scenario: The design template carries the visual contract section
    Given a project root with givn/ initialised
    When the user runs `givn eject design:template`
    Then the command succeeds
    And the ejected design template contains "Visual Design Contract"
    And the ejected design template contains "Interface:"
