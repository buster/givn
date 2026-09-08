@givn.delta @instruction-integrity

Feature: Instruction integrity for generated guidance

  @givn.modified @wip
  Scenario: Retrieval guidance distinguishes build capabilities
    Given the synchronized agent guidance for the specs workflow
    When an author reads the guidance
    Then the guidance does not mention engine-backed retrieval commands
    And the guidance does not offer advisory or blocking retrieval paths
    And the guidance keeps the deterministic spec toolchain wording
