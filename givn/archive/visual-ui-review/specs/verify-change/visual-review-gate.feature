@givn.delta @visual-review-gate

Feature: Visual Review Gate

  @givn.added @e2e
  Scenario: A visual change with complete evidence passes review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added @e2e
  Scenario: Review guidance requires the visual verdict and persona lenses
    Given a givn project
    When the user runs `givn instructions review --change visual-ui-review`
    Then stdout contains "VISUAL-REVIEW: PASS"
    And stdout contains "confirmed Persona"
    And stdout contains "vision"
    And stdout contains "transcript"

  @givn.added
  Scenario: A visual change without a visual verdict fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's review.md contains no visual verdict
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "VISUAL-REVIEW"

  @givn.added
  Scenario: A visual change with incomplete evidence fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "Consumer opens the dashboard"
    And stderr contains "desktop"

  @givn.added
  Scenario: A change without a visual contract is exempt from the visual gate
    Given a givn project with a change that has passed design-review
    And the change's design.md declares no visual design contract
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added
  Scenario: An unverified visual verdict cannot pass review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's review.md contains "VISUAL-REVIEW: UNVERIFIED"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "human sign-off"

  @givn.added
  Scenario: A terminal change passes review with a transcript
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract for the terminal interface
    And the change's delta spec contains an @e2e scenario "Consumer runs the export command"
    And the change has a captured transcript for "Consumer runs the export command"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added
  Scenario: Review guidance treats style improvements as contract updates
    Given a givn project
    When the user runs `givn instructions review --change visual-ui-review`
    Then stdout contains "pixel-level"
    And stdout contains "improvement"
    And stdout contains "updates the contract"

  @givn.added
  Scenario: Review guidance records an unverified verdict without vision
    Given a givn project
    When the user runs `givn instructions review --change visual-ui-review`
    Then stdout contains "VISUAL-REVIEW: UNVERIFIED"
    And stdout contains "human sign-off"

  @givn.added
  Scenario: A change that declares no visual interface is exempt
    Given a givn project with a change that has passed design-review
    And the change's design.md declares the visual interface "none"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added
  Scenario: A web contract without viewports fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a web visual contract without viewports
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "viewports"

  @givn.added
  Scenario: An empty evidence file fails review
    Given a givn project with a change that has passed design-review
    And the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And the change has an empty screenshot file for "Consumer opens the dashboard" at viewport "desktop"
    And the change's review.md contains "VISUAL-REVIEW: PASS"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "Consumer opens the dashboard"

  @givn.added
  Scenario: The review template carries the visual review section
    Given a project root with givn/ initialised
    When the user runs `givn eject review:template`
    Then the command succeeds
    And the ejected review template contains "VISUAL-REVIEW: PASS"
    And the ejected review template contains "persona"

  @givn.added
  Scenario: Synchronized review guidance carries the visual block
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-review guidance contains "VISUAL-REVIEW: PASS"
    And the generated givn-review guidance contains "confirmed Persona"
    And the generated givn-review guidance contains "no pixel-level"
