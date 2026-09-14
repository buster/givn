@givn.delta @scm-backend-selection
Feature: SCM backend selection

  Commands that select a backend act on the version-control system that owns
  the project root, or refuse by name before anything changes.

  @givn.added @e2e @wip
  Scenario: A Git worktree at the project root selects the Git backend
    Given  a Git project whose project root is the worktree top level
    And  a managed file is missing from the project
    When  the consumer runs `givn upgrade`
    Then  the command exits 0
    And  the managed upgrade publishes a revision

  @givn.added
  Scenario: A project without version control is refused by name
    Given  a project root without a Git worktree or a Fossil checkout
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the project root and the missing version-controlled project
    And  no file in the project changed

  @givn.added
  Scenario: A Fossil checkout is refused by name until the Fossil backend is available
    Given  a Fossil checkout whose project root is the checkout root
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the Fossil checkout and the unavailable Fossil backend
    And  no file in the project changed

  @givn.added @wip
  Scenario: A project root with both markers is refused as ambiguous
    Given  a project root that is both a Git worktree and a Fossil checkout
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the Git marker and the Fossil marker
    And  no file in the project changed

  @givn.added @wip
  Scenario: A project root nested inside a larger Git worktree is refused by name
    Given  a Git project whose project root is nested inside the worktree top level
    When  the user runs `givn upgrade`
    Then  the command exits non-zero
    And  stderr names the project root and the worktree top level
    And  no file in the project changed

  @givn.added @wip
  Scenario: A Git worktree without the Git executable is refused by name
    Given  a Git worktree whose `git` executable is unavailable
    When  the user runs `givn upgrade` without the `git` executable on PATH
    Then  the command exits non-zero
    And  stderr names the missing `git` executable
    And  no file in the project changed
