@givn.delta @skill-rewrites
Feature: skill-rewrites

  @givn.modified
  Scenario: Spec instructions use deterministic gates before retrieval exists
    Given  a givn project
    When  the user runs `givn instructions specs --change skill-rewrites`
    Then  stdout contains "User Interaction Inventory"
    And  stdout does not contain "Run `givn spec index`"
