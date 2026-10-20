@givn.delta @visual-review-gate

Feature: Visual Review Gate

  @givn.added
  Scenario: A machine-only CLI records not applicable and passes review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares no visual design contract
    And  the change's review.md records `VISUAL-REVIEW: NOT APPLICABLE`
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
