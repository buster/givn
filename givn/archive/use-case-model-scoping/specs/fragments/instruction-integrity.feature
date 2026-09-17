@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The review instruction carries the use-case scope rejection criteria
    Given the review instructions are requested
    When the reviewer reads the use-case scope criteria
    Then the guidance rejects a goal conjunction
    And the guidance rejects a missing primary actor
    And the guidance rejects a capability that fails the placement test
    And the guidance rejects a mutating scenario under an observing goal
    And the guidance rejects an extension without a step reference
    And the guidance rejects a precondition established by the flow
    And the guidance rejects mechanism vocabulary in the goal trigger or guarantees
    And the guidance rejects a component- or channel-shaped name
    And the guidance rejects an oversized leaf
    And the guidance rejects relationship misuse

  @givn.added
  Scenario: The design-review guidance points at the use-case scope criteria
    Given the design-review instructions are requested
    When the reviewer reads the use-case branch
    Then the guidance points at the use-case scope criteria

  @givn.added
  Scenario: The specs guidance carries the capability placement test
    Given the synchronized agent guidance for the specs workflow
    When an author reads the capability placement guidance
    Then the guidance carries the placement test
    And the guidance rejects a channel as an answer
    And the guidance requires goal-shaped capability names
