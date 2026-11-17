@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.modified
  Scenario: The ise instruction says declared retained paths survive archiving
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the served instruction says a declared Specification path must survive `givn archive`
    And  the served instruction names the archive location a change's artifacts move to
