@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.modified
  Scenario: Retrieval guidance distinguishes build capabilities
    Given  a Givn project with an active change
    When  the author requests the specs instructions through the Givn CLI
    Then  the returned guidance does not mention engine-backed retrieval commands
    And  the returned guidance does not offer advisory or blocking retrieval paths
    And  the returned guidance keeps the deterministic spec toolchain wording

  @givn.modified @wip
  Scenario: The specs guidance carries the capability placement test
    Given  a Givn project with an active change
    When  the author requests the specs instructions through the Givn CLI
    Then  the returned guidance carries the placement test
    And  the returned guidance rejects a channel as an answer
    And  the returned guidance requires goal-shaped capability names

  @givn.added @wip
  Scenario: Every instruction id named by generated guidance resolves through the Givn CLI
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then every generated skill and command names a served instruction
    And every named instruction id resolves through the Givn CLI

  @givn.added @wip
  Scenario: Every generated direction is invokable in its documented context
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then every direction to an artifact instruction names the change context it needs
    And every direction to a policy or step instruction names no change context
    And every named instruction id resolves in the context the direction names
