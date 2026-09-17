Feature: Visual Review Gate

  @e2e
  Scenario: A visual change with complete evidence passes review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  @e2e
  Scenario: Review guidance requires the visual verdict and persona lenses
    Given  a givn project
    When  the user runs `givn instructions review --change visual-ui-review`
    Then  stdout contains "VISUAL-REVIEW: PASS"
    And  stdout contains "confirmed Persona"
    And  stdout contains "vision"
    And  stdout contains "transcript"

  Scenario: A visual change without a visual verdict fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's review.md contains no visual verdict
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "VISUAL-REVIEW"

  Scenario: A visual change with incomplete evidence fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "Consumer opens the dashboard"
    And  stderr contains "desktop"

  Scenario: A change without a visual contract is exempt from the visual gate
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares no visual design contract
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: An unverified visual verdict cannot pass review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's review.md contains "VISUAL-REVIEW: UNVERIFIED"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "human sign-off"

  Scenario: A terminal change passes review with a transcript
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract for the terminal interface
    And  the change's delta spec contains an @e2e scenario "Consumer runs the export command"
    And  the change has a captured transcript for "Consumer runs the export command"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: Review guidance treats style improvements as contract updates
    Given  a givn project
    When  the user runs `givn instructions review --change visual-ui-review`
    Then  stdout contains "pixel-level"
    And  stdout contains "improvement"
    And  stdout contains "updates the contract"

  Scenario: Review guidance records an unverified verdict without vision
    Given  a givn project
    When  the user runs `givn instructions review --change visual-ui-review`
    Then  stdout contains "VISUAL-REVIEW: UNVERIFIED"
    And  stdout contains "human sign-off"

  Scenario: A change that declares no visual interface is exempt
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares the visual interface "none"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: A web contract without viewports fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a web visual contract without viewports
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "viewports"

  Scenario: An empty evidence file fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has an empty screenshot file for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "Consumer opens the dashboard"

  Scenario: The review template carries the visual review section
    Given  a project root with givn/ initialised
    When  the user runs `givn eject review:template`
    Then  the command succeeds
    And  the ejected review template contains "VISUAL-REVIEW: PASS"
    And  the ejected review template contains "persona"

  Scenario: Synchronized review guidance carries the visual block
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-review guidance contains "VISUAL-REVIEW: PASS"
    And  the generated givn-review guidance contains "confirmed Persona"
    And  the generated givn-review guidance contains "no pixel-level"
  Scenario: Review guidance tests the opposite interface classification
    Given  a givn project
    When  the user runs `givn instructions review --change visual-review-gate`
    Then  stdout contains "opposite classification"
    And  stdout contains "rendered page is a Web UI"
    And  stdout contains "restates"

  Scenario: Review guidance fails a deferral without an operator decision
    Given  a givn project
    When  the user runs `givn instructions review --change visual-review-gate`
    Then  stdout contains "deferral"
    And  stdout contains "operator decision"
    And  stdout contains "fails the review"

  Scenario: Review guidance records the persona disposition
    Given  a givn project
    When  the user runs `givn instructions review --change visual-review-gate`
    Then  stdout contains "disposition for every confirmed Persona"
    And  stdout contains "at least one confirmed Persona"
    And  stdout contains "record the reason"

  Scenario: Review guidance names the state screenshots the vision review saw
    Given  a givn project
    When  the user runs `givn instructions review --change visual-review-gate`
    Then  stdout contains "state screenshots the review saw"
    And  stdout contains "each declared UI state"

  Scenario: A web contract with missing state evidence fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's design.md declares the UI states "empty, loading, error"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "empty"

  Scenario: A web contract with complete state evidence passes review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's design.md declares the UI states "empty, loading, error"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has final-state screenshots for "Consumer opens the dashboard" at both viewports
    And  the change has state screenshots for "Consumer opens the dashboard" at both viewports for states "empty, loading, error"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: A visual change without a persona disposition fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md declares no persona disposition
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "PERSONAS"

  Scenario: A visual change with a confirmed persona disposition passes review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  Scenario: A visual interface with an empty persona disposition fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "mobile"
    And  the change has a final-state screenshot for "Consumer opens the dashboard" at viewport "desktop"
    And  the change's review.md declares the persona disposition ""
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "PERSONAS"

  Scenario: An empty state screenshot fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares a visual design contract naming viewports "mobile" and "desktop"
    And  the change's design.md declares the UI states "empty"
    And  the change's delta spec contains an @e2e scenario "Consumer opens the dashboard"
    And  the change has final-state screenshots for "Consumer opens the dashboard" at both viewports
    And  the change has an empty state screenshot for "Consumer opens the dashboard" at viewport "mobile" in state "empty"
    And  the change has a state screenshot for "Consumer opens the dashboard" at viewport "desktop" in state "empty"
    And  the change's review.md contains "VISUAL-REVIEW: PASS"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "empty"

  Scenario: A deferral without an operator decision fails review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares no visual design contract
    And  the change's review.md contains "DEFERRAL: browser evidence"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "OPERATOR-DECISION"

  Scenario: A deferral with an operator decision passes review
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares no visual design contract
    And  the change's review.md contains "DEFERRAL: browser evidence"
    And  the change's review.md also contains "OPERATOR-DECISION: accepted for this change"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
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

  Scenario: A change without a visual interface passes review with a reasoned none
    Given  a givn project with a change that has passed design-review
    And  the change's design.md declares no visual design contract
    And  the change's review.md declares the persona disposition "none — operator confirmed"
    And  the project declares a complete run declaration
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
