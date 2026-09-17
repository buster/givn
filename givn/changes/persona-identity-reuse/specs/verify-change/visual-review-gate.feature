@givn.delta @visual-review-gate

Feature: Visual Review Gate

  @givn.added
  Scenario: A visual change with an unknown persona disposition fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md declares the persona disposition "unknown-persona"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "unknown-persona"

  @givn.added
  Scenario: A visual change with a reasoned none fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md declares the persona disposition "none — operator confirmed"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "none — operator confirmed"

  @givn.added
  Scenario: A change without a visual interface passes review with a reasoned none
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares no visual design contract
    And  the change's review.md declares the persona disposition "none — operator confirmed"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
