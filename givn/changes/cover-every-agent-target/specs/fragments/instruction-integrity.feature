@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.modified
  Scenario: The characterization guidance requests the reverse review from Givn
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-characterize guidance requests `givn instructions review-reverse`
    And  the generated givn-characterize command requests `givn instructions review-reverse`
    And  every command-owning agent target receives the givn-characterize command

  @givn.modified
  Scenario: The generated guidance references resolve with the default configuration
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  every guidance reference resolves in the consumer project or is owned by a named creation step
    And  every agent target Givn supports is covered by the synchronized guidance

  @givn.added
  Scenario: A configured target without synchronized guidance fails the check
    Given  a givn project with all agent skill targets enabled
    When  the skills of the agent target ".claude" are removed
    Then  the configured-target check fails and names the agent target ".claude"
