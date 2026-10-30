@givn.delta @create-a-change

Feature: create-a-change

  @givn.modified
  Scenario: Only proposal.md and the add-on lock are created in the change directory
    Given a project root with givn/ initialised
    When the user runs `givn new only-proposal`
    Then the command exits 0
    And the change directory givn/changes/only-proposal/ contains exactly the files proposal.md and config-lock.yaml
    And no subdirectory named specs/ exists inside givn/changes/only-proposal/
