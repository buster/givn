@givn.delta @prose-consolidation
Feature: Prose consolidation

  @givn.added
  Scenario: Generated guidance names the commit path that works on every backend
    Given  an initialized Fossil project
    Then  the generated guidance names the commit path of the active backend
    And  the generated guidance states that a raw fossil commit carries no attribution
