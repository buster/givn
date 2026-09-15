Feature: SCM backend selection

  @e2e
  Scenario: A Git worktree at the project root selects the Git backend
    Given  a Git project whose project root is the worktree top level
    And  a managed file is missing from the project
    When  the consumer runs `givn upgrade`
    Then  the command exits 0
    And  the managed upgrade publishes a revision

  Scenario: A project without version control is refused by name
    Given  a project root without a Git worktree or a Fossil checkout
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the project root and the missing version-controlled project
    And  no file in the project changed

  Scenario: A project root with both markers is refused as ambiguous
    Given  a project root that is both a Git worktree and a Fossil checkout
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the Git marker and the Fossil marker
    And  no file in the project changed

  Scenario: A Fossil checkout nested inside an ancestor Git worktree selects Fossil
    Given  a Fossil checkout nested inside a Git worktree
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout reports the active backend as fossil

  Scenario: A project root nested inside a larger Git worktree is refused by name
    Given  a Git project whose project root is nested inside the worktree top level
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the project root and the worktree top level
    And  no file in the project changed

  Scenario: A Git worktree without the Git executable is refused by name
    Given  a Git worktree whose `git` executable is unavailable
    When  the user runs `givn upgrade` without the `git` executable on PATH
    Then  the command exits non-zero
    And  stderr names the missing `git` executable
    And  no file in the project changed
