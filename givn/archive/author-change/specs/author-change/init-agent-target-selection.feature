@givn.delta @init-agent-target-selection
Feature: Init agent target selection

  @givn.added @e2e
  Scenario: Consumer initializes a Fossil checkout and receives backend-aware guidance
    Given  a Fossil checkout without a givn/ directory
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  the generated guidance names the commit path of the active backend

  @givn.added @e2e
  Scenario: Consumer initializes a plain directory and receives a named warning
    Given  a project root without a Git worktree or a Fossil checkout
    When  the consumer initializes Givn in the project
    Then  the command exits 0
    And  stdout warns that archive and managed upgrade are unavailable
    And  a later archive is refused by name
