@givn.delta @visual-review-gate

Feature: Visual Review Gate

  @givn.added
  Scenario: Review guidance tests the opposite interface classification
    Given a givn project
    When the user runs `givn instructions review --change visual-review-gate`
    Then stdout contains "opposite classification"
    And stdout contains "rendered page is a Web UI"
    And stdout contains "restates"

  @givn.added @wip
  Scenario: Review guidance fails a deferral without an operator decision
    Given a givn project
    When the user runs `givn instructions review --change visual-review-gate`
    Then stdout contains "deferral"
    And stdout contains "operator decision"
    And stdout contains "fails the review"

  @givn.added @wip
  Scenario: Review guidance records the persona disposition
    Given a givn project
    When the user runs `givn instructions review --change visual-review-gate`
    Then stdout contains "disposition for every confirmed Persona"
    And stdout contains "at least one confirmed Persona"
    And stdout contains "record the reason"

  @givn.added @wip
  Scenario: Review guidance names the state screenshots the vision review saw
    Given a givn project
    When the user runs `givn instructions review --change visual-review-gate`
    Then stdout contains "state screenshots the review saw"
    And stdout contains "each declared UI state"

  @givn.added @wip
  Scenario: A web contract with missing state evidence fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's design.md declares the UI states "empty, loading, error"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "empty"

  @givn.added @wip
  Scenario: A web contract with complete state evidence passes review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's design.md declares the UI states "empty, loading, error"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has final-state screenshots for "Consumer opens the dashboard" at both viewports
    And the change has state screenshots for "Consumer opens the dashboard" at both viewports for states "empty, loading, error"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added @wip
  Scenario: A visual change without a persona disposition fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And the change's review.md declares no persona disposition
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "PERSONAS"

  @givn.added @wip
  Scenario: A visual change with a confirmed persona disposition passes review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added @wip
  Scenario: A visual interface with an empty persona disposition fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And the change's review.md declares the persona disposition ""
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "PERSONAS"

  @givn.added @wip
  Scenario: An empty state screenshot fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's design.md declares the UI states "empty"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has final-state screenshots for "Consumer opens the dashboard" at both viewports
    And the change has an empty state screenshot for "Consumer opens the dashboard" at viewport "mobile" in state "empty"
    And the change has a state screenshot for "Consumer opens the dashboard" at viewport "desktop" in state "empty"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "empty"

  @givn.added @wip
  Scenario: A deferral without an operator decision fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares no visual design contract
    And the change's review.md contains "DEFERRAL: browser evidence"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "OPERATOR-DECISION"

  @givn.added @wip
  Scenario: A deferral with an operator decision passes review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares no visual design contract
    And the change's review.md contains "DEFERRAL: browser evidence"
    And the change's review.md also contains "OPERATOR-DECISION: accepted for this change"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0
