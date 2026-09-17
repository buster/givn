@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The propose guidance carries the no-signal routing rule
    Given a givn project
    And the proposal instructions are requested
    When an author reads the routing rule
    Then the guidance tells the author to omit the recommendation when route reports no signal
    And the guidance keeps the recorded decision mandatory
    When the user runs `givn eject proposal:template`
    Then the file "givn/artifacts/proposal/proposal.md" contains "no signal"
