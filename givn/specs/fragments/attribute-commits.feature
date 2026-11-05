Feature: Attribute commits

  @e2e @attribute-commits
  Scenario: Consumer initializes a project and receives shared attribution plumbing
    Given  a new Git project with `core.hooksPath` set to `.githooks`
    And  an existing user commit-msg hook that writes a marker
    When  the consumer runs `givn init --agents opencode,claude,omp,maki`
    Then  the command exits 0 and reports installed attribution plumbing
    And  the checked-in attribution hook and selected harness integrations exist
    And  a subsequent human Git commit succeeds and runs the user hook exactly once without recursive dispatch
    And  the terminal transcript is captured for this scenario

  @e2e @attribute-commits
  Scenario: An attributed commit exposes the current LLM and harness
    Given  a Git project with complete commit attribution context
    And  changed "batch.txt" beside unrelated staged and unstaged work
    When  the consumer runs `givn commit batch.txt -m "fix parser"`
    Then  the command succeeds and reports the created revision
    And  its committer identifies "openai/gpt-5.6-luna" and "opencode"
    And  its message contains only the managed co-author attribution
    And  the revision contains only the supplied path
    And  unrelated index entries and working changes remain unchanged
    And  the terminal transcript is captured for this scenario

  Scenario: A human commit remains usable without attribution context
    Given  a Givn project with the shared commit-attribution hook active
    And  no current attribution context
    When  the consumer creates a commit with message "document manual setup"
    Then  the commit succeeds
    And  the commit message remains "document manual setup"

  Scenario: A partial attribution context does not block a commit
    Given  a Givn project with the shared commit-attribution hook active
    And  the current attribution context has harness "opencode" but no LLM or reasoning mode
    When  the consumer creates a commit with message "record partial context"
    Then  the commit succeeds
    And  the commit message identifies the harness as "opencode"
    And  the commit message marks the LLM and reasoning mode as unavailable

  Scenario: An existing commit hook runs before Givn attribution
    Given  a Givn project with an existing commit hook that writes a marker
    And  the current attribution context has LLM "openai/gpt-5.6-luna", reasoning mode "high", and harness "opencode"
    When  the consumer initializes Givn in the project
    And  the consumer creates a commit with message "preserve existing hook"
    Then  the commit succeeds
    And  the existing hook marker is present
    And  the commit message contains the Givn attribution trailers

  Scenario: Attribution follows a model change between commits
    Given  a Givn project with the shared commit-attribution hook active
    And  the current attribution context has LLM "model-a", reasoning mode "high", and harness "opencode"
    When  the consumer creates a commit with message "first change"
    And  the current attribution context changes to LLM "model-b", reasoning mode "medium", and harness "claude-code"
    And  the consumer creates a commit with message "second change"
    Then  the first commit identifies "model-a", "high", and "opencode"
    And  the second commit identifies "model-b", "medium", and "claude-code"

  @e2e
  Scenario: OpenCode prepares attribution using its selected model
    Given  a project with the generated OpenCode attribution integration
    When  OpenCode supplies the selected provider "openai" and model "gpt-5.6-luna" for session "session-1"
    And  OpenCode prepares the environment for a shell in session "session-1"
    Then  the shell environment identifies LLM "openai/gpt-5.6-luna"
    And  the shell environment identifies reasoning mode "high"
    And  the shell environment identifies harness "opencode"
    And  the shell environment identifies committer "LLM: openai/gpt-5.6-luna via opencode"

  @e2e
  Scenario: Claude attributes a commit using its current model
    Given  a project with the generated Claude attribution integration
    When  Claude provides current model "claude-sonnet-5" in its session transcript
    And  Claude prepares an RTK-wrapped Bash commit command with message "claude attribution"
    Then  the attributed Claude commit succeeds
    And  its Claude committer identifies "claude-sonnet-5" and "claude-code"
    And  its Claude message identifies LLM "claude-sonnet-5"
    And  its Claude message identifies the co-author attribution

  @attribute-commits
  Scenario: The SCM-neutral commit path attributes a Git revision
    Given  an initialized Git project with a change to "batch.txt"
    When  the consumer commits only "batch.txt" with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then  the command exits 0
    And  the revision message carries the managed attribution trailer

  @attribute-commits
  Scenario: The SCM-neutral commit path attributes a Fossil check-in
    Given  an initialized Fossil project with a change to "batch.txt"
    When  the consumer commits only "batch.txt" with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then  the command exits 0
    And  the revision message carries the managed attribution trailer

  @attribute-commits
  Scenario: The commit wrapper records only the named paths on Fossil
    Given  an initialized Fossil project with pending changes to "first.txt" and "second.txt"
    And  an unrelated pending addition "third.txt"
    When  the consumer commits only "first.txt" with message "feat: first"
    Then  the command exits 0
    And  the revision contains only "first.txt"
    And  the pending changes and addition for the other paths remain unchanged

  @attribute-commits
  Scenario: The commit wrapper without attribution context leaves the message unchanged
    Given  a Git project without attribution context and with a changed "batch.txt"
    When  the consumer runs `givn commit batch.txt -m "fix parser"`
    Then  the command exits 0
    And  the revision message remains "fix parser" without managed attribution trailers

  Scenario: omp leaves a command that creates no revision unchanged
    Given  a project with the generated omp attribution integration
    When  omp prepares the Bash command "git status"
    Then  the prepared command is "git status"

  Scenario: omp marks a partial attribution context as unavailable
    Given  a project with the generated omp attribution integration
    And  omp has no current model
    When  omp prepares the Bash command "git commit -m 'partial'"
    Then  the prepared command identifies LLM "unavailable"
    And  the prepared command identifies harness "omp"

  @e2e
  Scenario: omp attributes a commit using its current model
    Given  a project with the generated omp attribution integration
    And  omp's current model is "deepseek/deepseek-flash" at reasoning mode "high"
    When  omp prepares and runs a Bash revision command with message "omp attribution"
    Then  the attributed omp commit succeeds
    And  its omp committer identifies "deepseek/deepseek-flash" and "omp"
    And  its omp message identifies LLM "deepseek/deepseek-flash"
    And  its omp message identifies reasoning mode "high"

  Scenario: OpenCode leaves attribution unavailable when no model is observed
    Given  a project with the generated OpenCode attribution integration
    When  OpenCode prepares the environment for a shell in session "session-1"
    Then  the shell environment identifies LLM "unavailable"
    And  the shell environment identifies reasoning mode "unavailable"
    And  the shell environment identifies harness "opencode"
    And  the shell environment carries no committer identity

  Scenario: OpenCode attributes a command to the session that issued it
    Given  a project with the generated OpenCode attribution integration
    When  OpenCode supplies the selected provider "openai" and model "gpt-5.6-luna" for session "session-1"
    And  OpenCode supplies the selected provider "anthropic" and model "claude-sonnet-5" for session "session-2"
    And  OpenCode prepares the environment for a shell in session "session-1"
    Then  the shell environment identifies LLM "openai/gpt-5.6-luna"

  @attribute-commits
  Scenario Outline: A commit without explicit paths fails before staging
    Given  an initialized "<backend>" project with modified tracked files and an untracked file
    And  unrelated pending selections have been recorded
    When  the consumer runs `givn commit -m "feat: batch"`
    Then  the command exits nonzero and explains that explicit paths are required
    And  no revision is created and no file is staged or added
    And  existing pending selections and all working changes remain unchanged
    Examples:
      | backend |
      | Git |
      | Fossil |

  @attribute-commits
  Scenario: Git commits the selected working content without consuming an older staged version
    Given  a Git project where "batch.txt" has staged content and a different working version
    And  "other.txt" has an unrelated staged change
    When  the consumer runs `givn commit batch.txt -m "feat: batch"`
    Then  the revision contains the selected working version of "batch.txt" only
    And  the unrelated staged content of "other.txt" is unchanged
    And  no staged content of another path is included in the revision

  @attribute-commits
  Scenario Outline: Unsafe explicit-path isolation is refused without consuming other work
    Given  an initialized "<backend>" project whose requested path cannot be isolated safely from unrelated pending work
    When  the consumer commits only the requested path
    Then  the command refuses before creating a revision
    And  pending selections and working changes remain unchanged
    And  the diagnostic explains the isolation problem
    Examples:
      | backend |
      | Git |
      | Fossil |

  @attribute-commits
  Scenario: Repeated hook installation never dispatches its own launcher
    Given  a Git project using `.githooks` with a user commit-msg hook
    When  the consumer initializes Givn twice and then creates a human commit without attribution context
    Then  the commit succeeds within the hook completion deadline
    And  the user hook runs exactly once and remains preserved
    And  no launcher invokes itself directly or through a saved hook
    And  no managed attribution is invented

  @attribute-commits
  Scenario: An unavailable optional attribution helper does not fail a human commit
    Given  an initialized Git project with no attribution context and an unavailable attribution helper
    When  the consumer creates a human Git commit
    Then  the commit succeeds without waiting indefinitely
    And  the human commit message is unchanged

  @attribute-commits
  Scenario Outline: An explicit selected tracked deletion is committed successfully
    Given  an initialized "<backend>" project with a tracked file "obsolete.txt"
    And  the developer has deleted "obsolete.txt" with no unrelated pending work
    When  the consumer runs `givn commit obsolete.txt -m "fix: remove obsolete file"`
    Then  the command exits 0 and the revision records the deletion of "obsolete.txt" only
    And  "obsolete.txt" remains absent from the working tree
    Examples:
      | backend |
      | Git |
      | Fossil |

  @attribute-commits
  Scenario Outline: Unrelated staged additions and deletions survive an explicit Git commit
    Given  an initialized Git project with a working change to "batch.txt"
    And  "other.txt" has an unrelated staged <operation>
    When  the consumer runs `givn commit batch.txt -m "fix: selected batch"`
    Then  the command either commits only the selected working content or refuses an isolation conflict before mutation
    And  the staged <operation> of "other.txt" and its working state remain unchanged
    And  a successful revision contains no addition or deletion of "other.txt"
    And  an isolation refusal creates no revision and leaves all index entries and working changes unchanged
    Examples:
      | operation |
      | addition |
      | deletion |

  @attribute-commits
  Scenario: Maki prepares attribution for a revision command
    Given  a project with the generated Maki attribution integration
    And  Maki's current model is "deepseek/deepseek-flash" at reasoning mode "high"
    When  Maki prepares the command "git commit -m 'maki attribution'"
    Then  the prepared command identifies LLM "deepseek/deepseek-flash"
    And  the prepared command identifies reasoning mode "high"
    And  the prepared command identifies harness "maki"
    And  the prepared command identifies committer "LLM: deepseek/deepseek-flash via maki"

  @attribute-commits
  Scenario: Maki leaves a command that creates no revision unchanged
    Given  a project with the generated Maki attribution integration
    When  Maki prepares the command "git status"
    Then  the prepared command is "git status"

  @attribute-commits
  Scenario: Maki marks a partial attribution context as unavailable
    Given  a project with the generated Maki attribution integration
    And  Maki has no current model
    When  Maki prepares the command "git commit -m 'partial'"
    Then  the prepared command identifies LLM "unavailable"
    And  the prepared command identifies reasoning mode "unavailable"
    And  the prepared command identifies harness "maki"
    And  the prepared command carries no committer identity

  @attribute-commits
  Scenario: Maki marks a missing reasoning mode as unavailable
    Given  a project with the generated Maki attribution integration
    And  Maki's current model is "deepseek/deepseek-flash" without a reasoning mode
    When  Maki prepares the command "git commit -m 'model only'"
    Then  the prepared command identifies LLM "deepseek/deepseek-flash"
    And  the prepared command identifies reasoning mode "unavailable"
    And  the prepared command identifies harness "maki"
    And  the prepared command identifies committer "LLM: deepseek/deepseek-flash via maki"

  @attribute-commits
  Scenario: Maki resolves attribution from the session without an interactive UI
    Given  a project with the generated Maki attribution integration
    And  Maki runs without an interactive UI
    And  Maki's current model is "deepseek/deepseek-flash" without a reasoning mode
    When  Maki prepares the command "git commit -m 'headless attribution'"
    Then  the prepared command identifies LLM "deepseek/deepseek-flash"
    And  the prepared command identifies reasoning mode "unavailable"
    And  the prepared command identifies harness "maki"
    And  the prepared command identifies committer "LLM: deepseek/deepseek-flash via maki"

  @e2e @attribute-commits
  Scenario: Maki attributes a commit using its current model
    Given  a project with the generated Maki attribution integration
    And  Maki's current model is "deepseek/deepseek-flash" at reasoning mode "high"
    When  Maki prepares and runs a revision command with message "maki attribution"
    Then  the attributed Maki commit succeeds
    And  its Maki committer identifies "deepseek/deepseek-flash" and "maki"
    And  its Maki message identifies LLM "deepseek/deepseek-flash"
    And  its Maki message identifies reasoning mode "high"
