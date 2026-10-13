Feature: Visual Design Contract

  @e2e
  Scenario: Author requests the visual design contract guidance
    Given  a givn project
    When  the user runs `givn instructions design --change visual-ui-review`
    Then  stdout contains "Visual Design Contract"
    And  stdout contains "style-level"
    And  stdout contains "not pixel"
    And  stdout contains "docs/design/"
    And  stdout contains "design tokens"
    And  stdout contains "component states"
    And  stdout contains "rendered reference"

  Scenario: Design guidance names the default viewports
    Given  a givn project
    When  the user runs `givn instructions design --change visual-ui-review`
    Then  stdout contains "Samsung S24"
    And  stdout contains "360x780"
    And  stdout contains "Full HD"
    And  stdout contains "1920x1080"

  Scenario: Design guidance keeps improvements inside the style
    Given  a givn project
    When  the user runs `givn instructions design --change visual-ui-review`
    Then  stdout contains "improvement"
    And  stdout contains "updates the contract"

  @e2e
  Scenario: Synchronized guidance defines the contract approval and capture mechanics
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-design-review guidance directs the reviewer to the design-review instruction
    And  the served design-review instruction contains "Visual Design Contract"
    And  the served design-review instruction contains "propose"
    And  the served design-review instruction contains "user approves"
    And  the served design-review instruction contains "rendered options"
    And  the generated givn-steps guidance directs the step writer to the steps instruction
    And  the served steps instruction contains "evidence/visual"
    And  the served steps instruction contains "final-state"
    And  the served steps instruction contains "animations are frozen"
    And  the served steps instruction contains "reports/"

  Scenario: Design-review guidance requires an approved contract
    Given  a givn project
    When  the user runs `givn instructions design-review --change visual-ui-review`
    Then  stdout contains "Visual Design Contract"
    And  stdout contains "propose"
    And  stdout contains "user approves"
    And  stdout contains "rendered options"

  Scenario: The design template carries the visual contract section
    Given  a project root with givn/ initialised
    When  the user runs `givn eject design:template`
    Then  the command succeeds
    And  the ejected design template contains "Visual Design Contract"
    And  the ejected design template contains "Interface:"
  Scenario: Design guidance classifies a browser-rendered screen as a Web UI
    Given  a givn project
    When  the user runs `givn instructions design --change visual-design-contract`
    Then  stdout contains "renders HTML, CSS, or JavaScript in a browser"
    And  stdout contains "even when it needs only a page request"
    And  stdout contains "no controls"

  Scenario: Design guidance names the default browser driver
    Given  a givn project
    When  the user runs `givn instructions design --change visual-design-contract`
    Then  stdout contains "default browser driver"
    And  stdout contains "Playwright"
    And  stdout contains "does not pin"

  Scenario: Design guidance requires breakpoints and accessibility
    Given  a givn project
    When  the user runs `givn instructions design --change visual-design-contract`
    Then  stdout contains "Breakpoints"
    And  stdout contains "contrast"
    And  stdout contains "keyboard focus"
    And  stdout contains "how each is verified"

  Scenario: Design guidance requires a driver for every Web UI matrix row
    Given  a givn project
    When  the user runs `givn instructions design --change visual-design-contract`
    Then  stdout contains "is a Web UI row"
    And  stdout contains "needs a browser driver"

  Scenario: Design guidance names the user-visible persona boundary
    Given  a givn project
    When  the user runs `givn instructions design --change visual-design-contract`
    Then  stdout contains "user-visible"
    And  stdout contains "CLI text"
    And  stdout contains "reason"

  Scenario: Design-review guidance tests the opposite interface classification
    Given  a givn project
    When  the user runs `givn instructions design-review --change visual-design-contract`
    Then  stdout contains "opposite classification"
    And  stdout contains "rendered in a browser"
    And  stdout contains "must say why"

  Scenario: Design-review guidance records the persona disposition
    Given  a givn project
    When  the user runs `givn instructions design-review --change visual-design-contract`
    Then  stdout contains "per persona"
    And  stdout contains "relevance and evidence"
    And  stdout contains "operator confirmation"

  Scenario: Design-review guidance requires an operator question before a deferral
    Given  a givn project
    When  the user runs `givn instructions design-review --change visual-design-contract`
    Then  stdout contains "operator question"
    And  stdout contains "mandatory check"
    And  stdout contains "do not sign off"

  Scenario: Proposal guidance requires confirmed personas or a reasoned none
    Given  a givn project
    When  the user runs `givn instructions proposal --change visual-design-contract`
    Then  stdout contains "user-visible"
    And  stdout contains "operator confirmation"
    And  stdout contains "reason"

  Scenario: The design template carries the state, breakpoint, and accessibility lines
    Given  a project root with givn/ initialised
    When  the user runs `givn eject design:template`
    Then  the command succeeds
    And  the ejected design template contains "States:"
    And  the ejected design template contains "Breakpoints"
    And  the ejected design template contains "Accessibility"

  Scenario: The proposal template carries the persona confirmation line
    Given  a project root with givn/ initialised
    When  the user runs `givn eject proposal:template`
    Then  the command succeeds
    And  the ejected proposal template contains "Confirmed Personas"
    And  the ejected proposal template contains "operator confirmation"

  Scenario: The review template carries the persona disposition and deferral markers
    Given  a project root with givn/ initialised
    When  the user runs `givn eject review:template`
    Then  the command succeeds
    And  the ejected review template contains "PERSONAS:"
    And  the ejected review template contains "DEFERRAL:"
    And  the ejected review template contains "OPERATOR-DECISION:"

  Scenario: The design-review template carries the persona disposition table
    Given  a project root with givn/ initialised
    When  the user runs `givn eject design-review:template`
    Then  the command succeeds
    And  the ejected design-review template contains "persona"
    And  the ejected design-review template contains "Relevance"
    And  the ejected design-review template contains "Evidence"

  Scenario: Synchronized authoring guidance carries the classification, driver, and persona rules
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  the served design instruction contains "renders HTML, CSS, or JavaScript in a browser"
    And  the served design instruction contains "default browser driver"
    And  the served design-review instruction contains "opposite classification"
    And  the served proposal instruction contains "operator confirmation"
