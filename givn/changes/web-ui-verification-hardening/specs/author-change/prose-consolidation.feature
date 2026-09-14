@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.added
  Scenario: Canonical policy classifies a browser-rendered page as a Web UI
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "renders HTML, CSS, or JavaScript in a browser"
    And stdout contains "even when it needs only a page request"
    And stdout contains "no controls"
    And stdout contains "an HTTP client is not a valid driver"

  @givn.added
  Scenario: Canonical policy requires a rendered-state assertion
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "rendered state"
    And stdout contains "changes between two observations"
    And stdout contains "static heading"

  @givn.added
  Scenario: Canonical policy requires evidence for every declared UI state
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "declares UI states"
    And stdout contains "<viewport>--<state>.png"
    And stdout contains "empty, loading, or error"

  @givn.added @wip
  Scenario: Canonical policy refuses a silent deferral of a mandatory check
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "never deferred"
    And stdout contains "operator question"
    And stdout contains "recorded decision"
