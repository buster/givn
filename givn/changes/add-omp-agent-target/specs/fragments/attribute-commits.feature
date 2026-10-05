@givn.delta @attribute-commits

Feature: Attribute commits

  @givn.modified @e2e @wip
  Scenario: Consumer initializes a project and receives shared attribution plumbing
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode,claude,omp`
    Then  the command exits 0
    And  the checked-in commit attribution hook exists
    And  the Git repository uses the checked-in commit attribution hook
    And  the OpenCode attribution integration exists
    And  the Claude Code attribution integration exists
    And  the omp attribution integration exists

  @givn.added @wip
  Scenario: omp leaves a command that creates no revision unchanged
    Given  a project with the generated omp attribution integration
    When  omp prepares the Bash command "git status"
    Then  the prepared command is "git status"

  @givn.added @wip
  Scenario: omp marks a partial attribution context as unavailable
    Given  a project with the generated omp attribution integration
    And  omp has no current model
    When  omp prepares the Bash command "git commit -m 'partial'"
    Then  the prepared command identifies LLM "unavailable"
    And  the prepared command identifies harness "omp"

  @givn.added @e2e @wip
  Scenario: omp attributes a commit using its current model
    Given  a project with the generated omp attribution integration
    And  omp's current model is "deepseek/deepseek-flash" at reasoning mode "high"
    When  omp prepares and runs a Bash revision command with message "omp attribution"
    Then  the attributed omp commit succeeds
    And  its omp committer identifies "deepseek/deepseek-flash" and "omp"
    And  its omp message identifies LLM "deepseek/deepseek-flash"
    And  its omp message identifies reasoning mode "high"
