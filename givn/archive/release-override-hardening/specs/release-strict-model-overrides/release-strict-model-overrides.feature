# User Interaction Inventory:
# - Run `givn spec index` on an installed build with one fixture request per refused channel class
# - Run the change review gate on an installed build with model evidence unavailable

@givn.delta @release-strict-model-overrides

Feature: release-strict-model-overrides

  @givn.added @e2e @release-proof
  Scenario: An installed givn announces each refused request by name
    Given  a project prepared for semantic indexing
    And  a request for simulated embedder output
    And  a request to resize the second-stage pair budget
    When  a consumer runs `givn spec index` with an installed givn build
    Then  the installed givn announces that the simulated-embedder request is ignored
    And  the installed givn announces that the pair-budget request is ignored
    And  the installed givn refuses to proceed without the real embedder model

  @givn.added @e2e @release-proof
  Scenario: An installed givn keeps the review gate closed when model evidence is unavailable
    Given  a project prepared for semantic indexing
    And  a change whose review the gate must evaluate
    When  a consumer runs the change review gate with an installed givn build
    Then  the installed givn refuses to proceed without the real embedder model
