# User Interaction Inventory:
# - (inventory owned by the spec-retrieval capability; this delta retires one scenario and re-pins its successor)

@givn.delta @spec-retrieval

Feature: spec-retrieval

  @givn.removed
  Scenario: Review gate keeps retrieval advisory
    Given  a project root with givn/ initialised

  @givn.added
  Scenario: Review gate runs the unified rulebook and stays green without candidates
    Given  a givn project with permanent scenarios for retrieval
    And  an active change delta contains a scenario named "Running change only"
    When  the user runs `givn check review --change running-change`
    Then  the command exits 0
    And  the pair cache accounting reports "0" reused and "0" computed
