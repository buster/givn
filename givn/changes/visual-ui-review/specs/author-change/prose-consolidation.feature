@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.modified @e2e @wip
  Scenario: Canonical E2E policy is emitted by specs instructions
    Given a givn project
    When the user runs `givn instructions specs --change visual-ui-review`
    Then stdout contains "one E2E evidence scenario per distinct real user action"
    And stdout does not contain "one `@e2e` scenario per User Interaction Inventory entry"
    And stdout contains "final-state screenshot"
    And stdout contains "Samsung S24"
    And stdout contains "Full HD"

  @givn.added
  Scenario: Canonical policy requires the visual baseline at every assertion
    Given a givn project
    When the user runs `givn instructions specs --change visual-ui-review`
    Then stdout contains "visible label"
    And stdout contains "internal identifier"
    And stdout contains "no visible content is duplicated"
    And stdout contains "way back"
    And stdout contains "configured language"
    And stdout contains "reduced-motion"

  @givn.added @wip
  Scenario: Canonical policy uses transcripts for terminal interfaces
    Given a givn project
    When the user runs `givn instructions specs --change visual-ui-review`
    Then stdout contains "transcript"
    And stdout contains "API-only capabilities are exempt"
