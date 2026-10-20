@givn.delta @development-record

Feature: Development Record

  @givn.added
  Scenario: One request records derived usage, an attributable answer, and bounded failure text
    Given  a project root with givn/ initialised
    When  the agent reports a turn with inline usage, an answered question, and a failed tool
    Then  the turn carries one inline usage representation
    And  the linked usage entry is marked derived
    And  the question records its selected option, source, and answer time
    And  the failed tool records a bounded first error line
    And  the usage entry names the provider, model, and variant
