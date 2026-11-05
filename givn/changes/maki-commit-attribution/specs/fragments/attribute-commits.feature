@givn.delta @attribute-commits
Feature: Attribute commits

  @givn.modified @wip @e2e
  Scenario: Consumer initializes a project and receives shared attribution plumbing
    Given  a new Git project with `core.hooksPath` set to `.githooks`
    And  an existing user commit-msg hook that writes a marker
    When  the consumer runs `givn init --agents opencode,claude,omp,maki`
    Then  the command exits 0 and reports installed attribution plumbing
    And  the checked-in attribution hook and selected harness integrations exist
    And  a subsequent human Git commit succeeds and runs the user hook exactly once without recursive dispatch
    And  the terminal transcript is captured for this scenario

  @givn.added @wip
  Scenario: Maki prepares attribution for a revision command
    Given  a project with the generated Maki attribution integration
    And  Maki's current model is "deepseek/deepseek-flash" at reasoning mode "high"
    When  Maki prepares the command "git commit -m 'maki attribution'"
    Then  the prepared command identifies LLM "deepseek/deepseek-flash"
    And  the prepared command identifies reasoning mode "high"
    And  the prepared command identifies harness "maki"
    And  the prepared command identifies committer "LLM: deepseek/deepseek-flash via maki"

  @givn.added @wip
  Scenario: Maki leaves a command that creates no revision unchanged
    Given  a project with the generated Maki attribution integration
    When  Maki prepares the command "git status"
    Then  the prepared command is "git status"

  @givn.added @wip
  Scenario: Maki marks a partial attribution context as unavailable
    Given  a project with the generated Maki attribution integration
    And  Maki has no current model
    When  Maki prepares the command "git commit -m 'partial'"
    Then  the prepared command identifies LLM "unavailable"
    And  the prepared command identifies reasoning mode "unavailable"
    And  the prepared command identifies harness "maki"
    And  the prepared command carries no committer identity

  @givn.added @wip
  Scenario: Maki marks a missing reasoning mode as unavailable
    Given  a project with the generated Maki attribution integration
    And  Maki's current model is "deepseek/deepseek-flash" without a reasoning mode
    When  Maki prepares the command "git commit -m 'model only'"
    Then  the prepared command identifies LLM "deepseek/deepseek-flash"
    And  the prepared command identifies reasoning mode "unavailable"
    And  the prepared command identifies harness "maki"
    And  the prepared command identifies committer "LLM: deepseek/deepseek-flash via maki"

  @givn.added @wip @e2e
  Scenario: Maki attributes a commit using its current model
    Given  a project with the generated Maki attribution integration
    And  Maki's current model is "deepseek/deepseek-flash" at reasoning mode "high"
    When  Maki prepares and runs a revision command with message "maki attribution"
    Then  the attributed Maki commit succeeds
    And  its Maki committer identifies "deepseek/deepseek-flash" and "maki"
    And  its Maki message identifies LLM "deepseek/deepseek-flash"
    And  its Maki message identifies reasoning mode "high"
