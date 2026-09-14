@givn.delta @visual-design-contract

Feature: Visual Design Contract

  @givn.added
  Scenario: Design guidance classifies a browser-rendered screen as a Web UI
    Given a givn project
    When the user runs `givn instructions design --change visual-design-contract`
    Then stdout contains "renders HTML, CSS, or JavaScript in a browser"
    And stdout contains "even when it needs only a page request"
    And stdout contains "no controls"

  @givn.added
  Scenario: Design guidance names the default browser driver
    Given a givn project
    When the user runs `givn instructions design --change visual-design-contract`
    Then stdout contains "default browser driver"
    And stdout contains "Playwright"
    And stdout contains "does not pin"

  @givn.added
  Scenario: Design guidance requires breakpoints and accessibility
    Given a givn project
    When the user runs `givn instructions design --change visual-design-contract`
    Then stdout contains "Breakpoints"
    And stdout contains "contrast"
    And stdout contains "keyboard focus"
    And stdout contains "how each is verified"

  @givn.added
  Scenario: Design guidance requires a driver for every Web UI matrix row
    Given a givn project
    When the user runs `givn instructions design --change visual-design-contract`
    Then stdout contains "is a Web UI row"
    And stdout contains "needs a browser driver"

  @givn.added
  Scenario: Design guidance names the user-visible persona boundary
    Given a givn project
    When the user runs `givn instructions design --change visual-design-contract`
    Then stdout contains "user-visible"
    And stdout contains "CLI text"
    And stdout contains "reason"

  @givn.added
  Scenario: Design-review guidance tests the opposite interface classification
    Given a givn project
    When the user runs `givn instructions design-review --change visual-design-contract`
    Then stdout contains "opposite classification"
    And stdout contains "rendered in a browser"
    And stdout contains "must say why"

  @givn.added
  Scenario: Design-review guidance records the persona disposition
    Given a givn project
    When the user runs `givn instructions design-review --change visual-design-contract`
    Then stdout contains "per persona"
    And stdout contains "relevance and evidence"
    And stdout contains "operator confirmation"

  @givn.added
  Scenario: Design-review guidance requires an operator question before a deferral
    Given a givn project
    When the user runs `givn instructions design-review --change visual-design-contract`
    Then stdout contains "operator question"
    And stdout contains "mandatory check"
    And stdout contains "do not sign off"

  @givn.added @wip
  Scenario: Proposal guidance requires confirmed personas or a reasoned none
    Given a givn project
    When the user runs `givn instructions proposal --change visual-design-contract`
    Then stdout contains "user-visible"
    And stdout contains "operator confirmation"
    And stdout contains "reason"

  @givn.added @wip
  Scenario: The design template carries the state, breakpoint, and accessibility lines
    Given a project root with givn/ initialised
    When the user runs `givn eject design:template`
    Then the command succeeds
    And the ejected design template contains "States:"
    And the ejected design template contains "Breakpoints"
    And the ejected design template contains "Accessibility"

  @givn.added @wip
  Scenario: The proposal template carries the persona confirmation line
    Given a project root with givn/ initialised
    When the user runs `givn eject proposal:template`
    Then the command succeeds
    And the ejected proposal template contains "Confirmed Personas"
    And the ejected proposal template contains "operator confirmation"

  @givn.added @wip
  Scenario: The review template carries the persona disposition and deferral markers
    Given a project root with givn/ initialised
    When the user runs `givn eject review:template`
    Then the command succeeds
    And the ejected review template contains "PERSONAS:"
    And the ejected review template contains "DEFERRAL:"
    And the ejected review template contains "OPERATOR-DECISION:"

  @givn.added @wip
  Scenario: The design-review template carries the persona disposition table
    Given a project root with givn/ initialised
    When the user runs `givn eject design-review:template`
    Then the command succeeds
    And the ejected design-review template contains "persona"
    And the ejected design-review template contains "Relevance"
    And the ejected design-review template contains "Evidence"

  @givn.added @wip
  Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-design guidance contains "renders HTML, CSS, or JavaScript in a browser"
    And the generated givn-design guidance contains "default browser driver"
    And the generated givn-design-review guidance contains "opposite classification"
    And the generated givn-propose guidance contains "operator confirmation"
