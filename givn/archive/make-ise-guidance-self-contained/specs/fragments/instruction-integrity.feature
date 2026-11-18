@givn.delta @instruction-integrity

Feature: instruction-integrity

  @givn.added @instruction-integrity
  Scenario: The ise instruction's file references resolve in a consumer project
    Given  a Givn project with the ise addon enabled and the guidance reference check available
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served ise instruction's file references resolve in the consumer project

  @givn.added @instruction-integrity
  Scenario: Printed guidance omits the opt-in ise rules
    Given  a Givn project with an active change and the guidance reference check available
    When  the author requests every instruction Givn prints
    Then  the printed guidance carries no ise-only phrases
