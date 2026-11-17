@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.modified
  Scenario: Served ise guidance keeps component scenarios in the Specification
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance states that the @e2e scenarios stay in the Specification with their exact titles
    And  the guidance states that the declared e2e package holds the runner and every @e2e step definition
    And  the guidance states that component scenarios stay in the Specification unchanged
    And  the guidance states that only their step definitions and fakes are implementation
    And  the guidance keeps the two-layer test boundary
