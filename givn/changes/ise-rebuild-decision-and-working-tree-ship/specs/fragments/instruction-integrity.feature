@givn.delta @instruction-integrity

Feature: instruction-integrity

  @givn.added @instruction-integrity
  Scenario: The ise instruction teaches the working-tree ship sequence
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction teaches the working-tree ship sequence in order
    And  the served instruction states that no spec copy, candidate revision, early archive merge, generated spec-copy manifest, repository view that hides history, or sandbox is required

  @givn.added @instruction-integrity
  Scenario: The ise instruction states the working-tree ship limitation
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction states that a passing working-tree verification proves tested behaviour
    And  the served instruction states that it does not prove the Specification alone was sufficient

  @givn.added @instruction-integrity
  Scenario: The ise instruction distinguishes the working-tree ship from copy preparation
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction states that the working-tree ship rebuilds in the current project working tree
    And  the served instruction states that the spec-copy commands prepare an external copy and never edit the working tree

  @givn.added @instruction-integrity
  Scenario: The ise instruction restarts verification when the Specification changes after a rebuild starts
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction states that a changed Specification or E2E suite after the rebuild starts records a new frozen revision
    And  the served instruction states that the relevant verification reruns against it

  @givn.added @instruction-integrity
  Scenario: The ise instruction cites the principle source
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction cites https://iseprinciple.org
