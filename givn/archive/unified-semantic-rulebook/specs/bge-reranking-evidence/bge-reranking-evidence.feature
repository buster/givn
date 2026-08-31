# User Interaction Inventory:
# - (inventory owned by the bge-reranking-evidence capability; this delta only retires one scenario)

@givn.delta @bge-reranking-evidence

Feature: bge-reranking-evidence

  @givn.removed
  Scenario: E5 candidates outside the bounded BGE set remain explicitly unscored and blocking
    Given  a project root with givn/ initialised

  @givn.modified
  Scenario: The independent E5 gate remains separate from BGE evidence
    Given  an end-to-end project with an active semantic review candidate
    And  the local second-stage evidence is enabled
    When  the user runs blocking change review with the E5 similarity gate and second-stage evidence enabled
    Then  the semantic review report records the embedding admission for the selected pairs
    And  a BGE status does not remove the embedding admission
    And  no BGE status is rendered as a Givn classification

  @givn.modified
  Scenario: The E5 similarity gate remains available without BGE
    Given  an end-to-end project with an active semantic review candidate
    When  the user runs blocking change review with the E5 similarity gate and BGE disabled
    Then  the pair cache accounting reports "0" reused and "0" computed
    And  the output contains no participating BGE model or score
