Feature: prose-consolidation

  @e2e
  Scenario: Canonical E2E policy is emitted by specs instructions
    Given  a givn project
    When  the user runs `givn instructions specs --change visual-ui-review`
    Then  stdout contains "one E2E evidence scenario per distinct real user action"
    And  stdout does not contain "one `@e2e` scenario per User Interaction Inventory entry"
    And  stdout contains "final-state screenshot"
    And  stdout contains "Samsung S24"
    And  stdout contains "Full HD"

  @e2e
  Scenario: Generated guidance points to the canonical E2E policy
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then stdout contains "Syncing skills and agent commands..."
    And the generated givn-spec guidance contains "givn instructions specs"
    And the generated givn-design guidance contains "givn instructions specs"
    And the generated givn-review guidance contains "givn instructions specs"
    And the generated givn-spec command contains "givn instructions specs"
    And the generated givn-steps guidance contains "givn instructions specs"
    And the generated givn-tasks guidance contains "givn instructions specs"
    And the generated givn-implement guidance contains "givn instructions specs"
    And the generated givn-design-review guidance contains "givn instructions specs"
    And generated guidance does not contain "one `@e2e` scenario per User Interaction Inventory entry"
    And generated guidance does not contain "one @e2e scenario per distinct happy-path action"

  Scenario: Project override replaces the canonical specs instruction
    Given a givn project
    And a project override for specs instructions containing "Project-specific E2E policy"
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "Project-specific E2E policy"
    And stdout does not contain "one E2E evidence scenario per distinct real user action"

  Scenario: Canonical policy classifies output-affecting flags as variants
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "output-affecting flags are recorded as variants"
    And stdout does not contain "Every subcommand, every flag combination that produces different output"

  Scenario: Design and review instructions defer to the canonical policy
    Given a givn project
    When the user reads the design and review instructions for prose-consolidation
    Then combined instruction output contains "givn instructions specs"
    And combined instruction output does not contain "one `@e2e` scenario per User Interaction Inventory entry"
  Scenario: Canonical policy requires the visual baseline at every assertion
    Given  a givn project
    When  the user runs `givn instructions specs --change visual-ui-review`
    Then  stdout contains "visible label"
    And  stdout contains "internal identifier"
    And  stdout contains "no visible content is duplicated"
    And  stdout contains "way back"
    And  stdout contains "configured language"
    And  stdout contains "reduced-motion"

  Scenario: Canonical policy uses transcripts for terminal interfaces
    Given  a givn project
    When  the user runs `givn instructions specs --change visual-ui-review`
    Then  stdout contains "transcript"
    And  stdout contains "API-only capabilities are exempt"
  Scenario: Canonical policy classifies a browser-rendered page as a Web UI
    Given  a givn project
    When  the user runs `givn instructions specs --change prose-consolidation`
    Then  stdout contains "renders HTML, CSS, or JavaScript in a browser"
    And  stdout contains "even when it needs only a page request"
    And  stdout contains "no controls"
    And  stdout contains "an HTTP client is not a valid driver"

  Scenario: Canonical policy requires a rendered-state assertion
    Given  a givn project
    When  the user runs `givn instructions specs --change prose-consolidation`
    Then  stdout contains "rendered state"
    And  stdout contains "changes between two observations"
    And  stdout contains "static heading"

  Scenario: Canonical policy requires evidence for every declared UI state
    Given  a givn project
    When  the user runs `givn instructions specs --change prose-consolidation`
    Then  stdout contains "declares UI states"
    And  stdout contains "<viewport>--<state>.png"
    And  stdout contains "empty, loading, or error"

  Scenario: Canonical policy refuses a silent deferral of a mandatory check
    Given  a givn project
    When  the user runs `givn instructions specs --change prose-consolidation`
    Then  stdout contains "never deferred"
    And  stdout contains "operator question"
    And  stdout contains "recorded decision"
  Scenario: Generated guidance names the commit path that works on every backend
    Given  an initialized Fossil project
    Then  the generated guidance names the commit path of the active backend
    And  the generated guidance states that a raw fossil commit carries no attribution
  Scenario: Generated interactive guidance points to the canonical dialog policy
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "givn instructions dialog"
    And  the generated givn-explore guidance contains "givn instructions dialog"
    And  the generated givn-design-review guidance contains "givn instructions dialog"
    And  the generated givn-design-review command contains "givn instructions dialog"
    And  the generated givn-ideation command contains "givn instructions dialog"
    And  the generated givn-explore command contains "givn instructions dialog"
    And  the generated givn-propose command contains "givn instructions dialog"
    And  the generated grill-me command contains "givn instructions dialog"
    And  the generated grill-with-docs command contains "givn instructions dialog"
