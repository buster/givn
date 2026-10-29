@givn.delta @attribute-commits
Feature: Safe attribution and explicit-path revisions

  @givn.modified @e2e @wip
  Scenario: Consumer initializes a project and receives shared attribution plumbing
    Given a new Git project with `core.hooksPath` set to `.githooks`
    And an existing user commit-msg hook that writes a marker
    When the consumer runs `givn init --agents opencode,claude,omp`
    Then the command exits 0 and reports installed attribution plumbing
    And the checked-in attribution hook and selected harness integrations exist
    And a subsequent human Git commit succeeds and runs the user hook exactly once without recursive dispatch

  @givn.modified @e2e @wip
  Scenario: An attributed commit exposes the current LLM and harness
    Given a Git project with complete commit attribution context
    And changed "batch.txt" beside unrelated staged and unstaged work
    When the consumer runs `givn commit batch.txt -m "fix parser"`
    Then the command succeeds and reports the created revision
    And its committer identifies "openai/gpt-5.6-luna" and "opencode"
    And its message contains only the managed co-author attribution
    And the revision contains only the supplied path
    And unrelated index entries and working changes remain unchanged

  @givn.modified @wip
  Scenario: The SCM-neutral commit path attributes a Git revision
    Given an initialized Git project with a change to "batch.txt"
    When the consumer commits only "batch.txt" with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then the command exits 0
    And the revision message carries the managed attribution trailer

  @givn.modified @wip
  Scenario: The SCM-neutral commit path attributes a Fossil check-in
    Given an initialized Fossil project with a change to "batch.txt"
    When the consumer commits only "batch.txt" with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then the command exits 0
    And the revision message carries the managed attribution trailer

  @givn.modified @wip
  Scenario: The commit wrapper records only the named paths on Fossil
    Given an initialized Fossil project with pending changes to "first.txt" and "second.txt"
    And an unrelated pending addition "third.txt"
    When the consumer commits only "first.txt" with message "feat: first"
    Then the command exits 0
    And the revision contains only "first.txt"
    And the pending changes and addition for the other paths remain unchanged

  @givn.modified @wip
  Scenario: The commit wrapper without attribution context leaves the message unchanged
    Given a Git project without attribution context and with a changed "batch.txt"
    When the consumer runs `givn commit batch.txt -m "fix parser"`
    Then the command exits 0
    And the revision message remains "fix parser" without managed attribution trailers

  @givn.removed
  Scenario: The stage-all path excludes AGENTS.md and generated rules under local-only
    Given this scenario is removed

  @givn.removed
  Scenario: The versioned stage-all path still excludes AGENTS.md
    Given this scenario is removed

  @givn.added @wip
  Scenario Outline: A commit without explicit paths fails before staging
    Given an initialized "<backend>" project with modified tracked files and an untracked file
    And unrelated pending selections have been recorded
    When the consumer runs `givn commit -m "feat: batch"`
    Then the command exits nonzero and explains that explicit paths are required
    And no revision is created and no file is staged or added
    And existing pending selections and all working changes remain unchanged

    Examples:
      | backend |
      | Git     |
      | Fossil  |

  @givn.added @wip
  Scenario: Git commits the selected working content without consuming an older staged version
    Given a Git project where "batch.txt" has staged content and a different working version
    And "other.txt" has an unrelated staged change
    When the consumer runs `givn commit batch.txt -m "feat: batch"`
    Then the revision contains the selected working version of "batch.txt" only
    And the unrelated staged content of "other.txt" is unchanged
    And no staged content of another path is included in the revision

  @givn.added @wip
  Scenario Outline: Unsafe explicit-path isolation is refused without consuming other work
    Given an initialized "<backend>" project whose requested path cannot be isolated safely from unrelated pending work
    When the consumer commits only the requested path
    Then the command refuses before creating a revision
    And pending selections and working changes remain unchanged
    And the diagnostic explains the isolation problem

    Examples:
      | backend |
      | Git     |
      | Fossil  |

  @givn.added @wip
  Scenario: Repeated hook installation never dispatches its own launcher
    Given a Git project using `.githooks` with a user commit-msg hook
    When the consumer initializes Givn twice and then creates a human commit without attribution context
    Then the commit succeeds within the hook completion deadline
    And the user hook runs exactly once and remains preserved
    And no launcher invokes itself directly or through a saved hook
    And no managed attribution is invented

  @givn.added @wip
  Scenario: An unavailable optional attribution helper does not fail a human commit
    Given an initialized Git project with no attribution context and an unavailable attribution helper
    When the consumer creates a human Git commit
    Then the commit succeeds without waiting indefinitely
    And the human commit message is unchanged
