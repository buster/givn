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
    Given  a Givn project with all agent skill targets and optional guidance enabled
    When  the consumer synchronizes the selected agent guidance through the Givn CLI
    Then  every emitted skill and command requires the effective dialog policy before its context asks a user-facing question
    And  direct skill invocation has the same obligation as session-start invocation
    And  reference skills do not narrow the obligation to one workflow phase
    And  the generated givn-spec guidance directs the author to the specs instruction
    And  the served specs instruction carries the canonical interaction and E2E policy
    And  detailed dialog rules and worked examples remain in the effective dialog instructions rather than competing generated copies
    And  the synchronization result reports the selected guidance successfully

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
  Scenario: The propose and archive commands carry the seed stamping instructions
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  the served proposal instruction contains "proposed by"
    And  the served archive instruction contains "implemented in"
    And  the generated givn-propose command directs the author to the proposal instruction
    And  the generated givn-archive command directs the archivist to the archive instruction

  Scenario: Disabling optional addons does not disable the dialog obligation
    Given  a Givn project with all agent skill targets selected and every optional addon disabled
    When  the author synchronizes the selected guidance through the Givn CLI
    Then  every available core skill and command still requires the effective dialog policy for every user-facing question
    And  guidance for disabled addons is not installed
    And  the core dialog obligation does not depend on the optional tone policy
    And  the selected targets and addon choices remain unchanged

  Scenario: Delegated question preparation carries the policy without assuming inherited context
    Given  a Givn project with delegated authoring and review guidance enabled
    When  the author reads the synchronized delegation instructions
    Then  every question-preparing delegation explicitly requires the effective dialog policy in the fresh context
    And  the delegation instructions do not assume the parent's loaded policy is inherited
    And  the presenting agent remains responsible for the question before it reaches the person
    And  a question-tool recommendation or option label does not replace context and answer consequences

  Scenario: Successful project skill and command replacements are preserved without policy injection
    Given  a Givn project with all agent skill targets selected
    And  valid project-owned skill and command replacements intentionally omit the bundled dialog obligation
    When  the author synchronizes the selected guidance through the Givn CLI
    Then  the project-owned skill and command source bytes remain unchanged
    And  every selected skill consumer receives the authoritative rendered replacement without a bundled prefix
    And  the opencode command consumer receives the authoritative rendered replacement without a bundled prefix
    And  the managed project-start guidance still references the effective dialog policy
