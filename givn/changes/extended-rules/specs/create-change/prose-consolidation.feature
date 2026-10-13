@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.modified @e2e @wip
  Scenario: Generated guidance points to the canonical E2E policy
    Given  a Givn project with all agent skill targets and optional guidance enabled
    When  the consumer synchronizes the selected agent guidance through the Givn CLI
    Then  every emitted skill and command requires the effective dialog policy before its context asks a user-facing question
    And  direct skill invocation has the same obligation as session-start invocation
    And  reference skills do not narrow the obligation to one workflow phase
    And  the generated givn-spec guidance directs the author to the specs instruction
    And  the served specs instruction carries the canonical interaction and E2E policy
    And  detailed dialog rules and worked examples remain in the effective dialog instructions rather than competing generated copies
    And  the synchronization result reports the selected guidance successfully

  @givn.modified
  Scenario: The propose and archive commands carry the seed stamping instructions
    Given a givn project with all agent skill targets enabled and an active change
    When the user runs `givn skills sync`
    Then the served proposal instruction contains "proposed by"
    And the served archive instruction contains "implemented in"
    And the generated givn-propose command directs the author to the proposal instruction
    And the generated givn-archive command directs the archivist to the archive instruction
