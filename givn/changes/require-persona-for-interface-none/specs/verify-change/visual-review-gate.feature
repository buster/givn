@givn.delta @visual-review-gate

Feature: Visual Review Gate

  @givn.modified
  Scenario: A change that declares no visual interface is exempt
    A declared Interface: none contract still requires a confirmed Persona;
    only the visual verdict and visual evidence are exempt.
    Given a givn project with a change that has passed design-review
    And the change's design.md declares the visual interface "none"
    And the change's review.md names a confirmed persona without a visual verdict
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added
  Scenario: A change that declares the interface none without a persona disposition fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares the visual interface "none"
    And the change's review.md declares no persona disposition
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "PERSONAS"

  @givn.added
  Scenario: A change that declares the interface none with a reasoned none fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares the visual interface "none"
    And the change's review.md declares the persona disposition "none — operator confirmed"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "none — operator confirmed"

  @givn.added
  Scenario: Review guidance requires a persona for a declared interface none contract
    Given a givn project
    When the user runs `givn instructions review --change visual-review-gate`
    Then stdout contains "Interface: none"
    And stdout contains "declared visual design contract"
