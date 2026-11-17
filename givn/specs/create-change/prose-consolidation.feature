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

  @e2e @prose-consolidation
  Scenario: Generated guidance points to the canonical E2E policy
    Given  a Givn project with all agent skill targets and optional guidance enabled
    When  the consumer synchronizes the selected agent guidance through the Givn CLI
    Then  the generated givn-spec skill directs the author to the canonical specs instruction
    And  managed bootstrap guidance owns loading the effective dialog policy before the first question once per context
    And  thin skills contain no repeated dialog-loading or delegation boilerplate
    And  detailed dialog examples remain available through the policy's companion index
    And  synchronization reports skills without separately generated commands
    And  the terminal transcript is captured for this scenario

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

  @prose-consolidation
  Scenario: Canonical policy requires a rendered-state assertion
    Given  a givn project
    When  the user runs `givn instructions specs --change prose-consolidation`
    Then  browser E2E guidance accepts a real browser rendering static content as interface evidence
    And  it requires state changes only when the product interaction actually changes state
    And  it does not require artificial clocks counters or asynchronous content

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

  @prose-consolidation
  Scenario: The propose and archive commands carry the seed stamping instructions
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  the served proposal instruction preserves the proposed-by seed stamp
    And  the served archive instruction preserves the implemented-in seed stamp
    And  generated propose and archive skills direct authors to those instructions
    And  no separately generated propose or archive command is installed

  @prose-consolidation
  Scenario: Disabling optional addons does not disable the dialog obligation
    Given  a Givn project with all agent skill targets selected and every optional addon disabled
    When  the author synchronizes the selected guidance through the Givn CLI
    Then  the managed bootstrap still requires loading the effective dialog policy before the context's first question
    And  guidance for disabled addons is not installed
    And  core skills contain no repeated policy-loading boilerplate
    And  the selected targets and addon choices remain unchanged

  @prose-consolidation
  Scenario: Successful project skill and command replacements are preserved without policy injection
    Given  a Givn project with all agent skill targets selected
    And  valid project-owned skill replacements intentionally omit bundled dialog-loading text
    When  the author synchronizes the selected guidance through the Givn CLI
    Then  project-owned skill source bytes remain unchanged
    And  every selected skill consumer receives the authoritative replacement without a bundled prefix
    And  user-owned legacy command files remain unchanged without being generated or synchronized
    And  managed project-start guidance still references the effective dialog policy

  @prose-consolidation
  Scenario: Canonical policy accepts the library public API as its real interface
    Given  a Givn project with ise effective whose product is a library without a service or browser
    When  the author reads the canonical E2E guidance
    Then  consumer calls to the public library API may run in process only from the declared e2e package as real-interface evidence
    And  a call that needs production source becomes a component scenario
    And  the policy does not require artificial services containers or browser infrastructure

  @prose-consolidation
  Scenario: Canonical policy selects E2E scope explicitly rather than by command shape
    Given  regular and E2E verification share one runner command
    When  the author reads the canonical verification guidance
    Then  explicit scope selection such as GIVN_RESULT_SCOPE distinguishes the executions
    And  equal test counts or command strings do not invalidate honestly isolated scopes
    And  the E2E execution runs the declared e2e package which links no production source
    And  required executions and zero failed or skipped tests remain required without an extra isolation run

  @prose-consolidation
  Scenario: Justified E2E reclassification preserves real-interface coverage
    Given  an existing E2E scenario is shown to be a variant rather than a distinct action
    When  the author reads the canonical reclassification guidance
    Then  reclassification requires a reason and aligned interaction inventory tasks and review records
    And  every retained real user action still has exactly one E2E smoke scenario
    And  tags cannot be removed merely to evade verification
    And  regular scenarios protect requirements distinct from the retained smoke

  @prose-consolidation
  Scenario: Canonical policy names the declared e2e package as the E2E target
    Given  a Givn project with ise effective
    When  the author reads the canonical E2E guidance
    Then  the policy states that the declared e2e package holds the E2E runner and every E2E step definition
    And  the policy states that the package links no production source
    And  the policy states that an unbound or skipped step fails the E2E run
    And  the policy states that feature files stay shared

  @prose-consolidation
  Scenario: Served ise guidance keeps component scenarios in the Specification
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance states that the @e2e scenarios stay in the Specification with their exact titles
    And  the guidance states that the declared e2e package holds the runner and every @e2e step definition
    And  the guidance states that component scenarios stay in the Specification unchanged
    And  the guidance states that only their step definitions and fakes are implementation
    And  the guidance keeps the two-layer test boundary

  @prose-consolidation
  Scenario: Canonical policy keeps the in-process allowance while ise is off
    Given  a Givn project with ise off whose product is a library without a service or browser
    When  the author reads the canonical E2E guidance
    Then  consumer calls to the public library API may run in process as real-interface evidence
    And  the policy does not require artificial services containers or browser infrastructure

  @prose-consolidation
  Scenario: Served ise guidance states that the trial always carries the declaration
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance states that the spec copy always carries the declaration itself
    And  the guidance states that it is an implicit Specification entry, not a sixth role
