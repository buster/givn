# User Interaction Inventory:
# - (inventory owned by the bge-reranking-evidence capability; this delta only retires one scenario)

@givn.delta @bge-reranking-evidence

Feature: bge-reranking-evidence

  @givn.removed
  Scenario: E5 candidates outside the bounded BGE set remain explicitly unscored and blocking
    Given  a project root with givn/ initialised
