@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The ise instruction carries the five rebuild obligations
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction names Specification completeness, the two-layer test boundary, persisted-state continuity, the change strategy, and the spec gap classification
    And  the served instruction uses the established Component, Port, contract, Specification, and Source code vocabulary

  @givn.added
  Scenario: The review guidance asks the neighboring-components question without a gate
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the review guidance
    Then  the command exits 0
    And  the served review guidance asks for the neighboring components changed outside the replaced component and the reason
    And  the served review guidance states that the answer is a review question, never a gate
