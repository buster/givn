@givn.delta @givn-status-command
Feature: givn status command

  @givn.modified
  Scenario: Multiple active changes listed alphabetically — text mode
    Given a project root with givn/ initialised
    And a change "z-feature" exists with artifact "proposal.md"
    And a change "a-feature" exists with artifact "proposal.md"
    When the user runs `givn status`
    Then the command exits 0
    And stdout line 4 matches "a-feature"
    And stdout line 5 matches "z-feature"

  @givn.added
  Scenario Outline: Status names the active SCM backend
    Given  a <backend> project
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout reports the active backend as <scm>

    Examples:
      | backend | scm    |
      | Git     | git    |
      | Fossil  | fossil |

  @givn.added
  Scenario: Status without a backend still succeeds and names the unavailable commands
    Given  a project root without a Git worktree or a Fossil checkout
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout reports the active backend as none
    And  stdout names archive and managed upgrade as unavailable
