@givn.delta @llm-attribution-installation
Feature: Commit attribution installation

  @givn.added @e2e
  Scenario: Consumer initializes a Fossil project and receives the givn commit path without a hook
    Given  a Fossil checkout without a givn/ directory
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  stdout names the givn commit attribution path
    And  no .githooks directory exists
