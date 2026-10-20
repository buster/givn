@givn.delta @development-record

Feature: Development Record

  @givn.added
  Scenario: A linked usage entry is marked derived
    Given  a project root with givn/ initialised
    When  the agent reports a turn with inline usage, an answered question, and a failed tool
    Then  the turn carries one inline usage representation
    And  the linked usage entry is marked derived

  @givn.added
  Scenario: An answered question records selection, source, and time
    Given  a project root with givn/ initialised
    When  the agent reports a turn with inline usage, an answered question, and a failed tool
    Then  the question records its selected option, source, and answer time

  @givn.added
  Scenario: A failed command records a bounded error line
    Given  a project root with givn/ initialised
    When  the agent reports a turn with inline usage, an answered question, and a failed tool
    Then  the failed tool records a bounded first error line

  @givn.added
  Scenario: Usage identity names provider, model, and variant
    Given  a project root with givn/ initialised
    When  the agent reports a turn with inline usage, an answered question, and a failed tool
    Then  the usage entry names the provider, model, and variant
