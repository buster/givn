@givn.delta @domain-skill-trigger

Feature: Domain-modeling skill is triggered and routed

  @givn.added
  Scenario: The domain-modeling guidance carries the term introduction test
    Given the domain-modeling skill template is inspected
    When an agent reads the term introduction test
    Then the body requires a genus and differentia definition
    And the body requires a contrast pair
    And the body requires one concrete domain sentence
    And the body demonstrates a failure when the genus slot cannot be filled
