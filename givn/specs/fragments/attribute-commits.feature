Feature: Attribute commits

  @e2e
  Scenario: Consumer initializes a project and receives shared attribution plumbing
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode,claude,omp`
    Then  the command exits 0
    And  the checked-in commit attribution hook exists
    And  the Git repository uses the checked-in commit attribution hook
    And  the OpenCode attribution integration exists
    And  the Claude Code attribution integration exists
    And  the omp attribution integration exists

  @e2e
  Scenario: An attributed commit exposes the current LLM and harness
    Given  a Git project with complete commit attribution context
    When  the consumer creates an attributed Git commit with message "fix parser"
    Then  the attributed commit succeeds
    And  its committer identifies "openai/gpt-5.6-luna" and "opencode"
    And  its message contains only the managed co-author attribution

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
  Scenario: The SCM-neutral commit path attributes a Git revision
    Given  a Givn project with the shared commit-attribution hook active
    When  the consumer commits with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then  the command exits 0
    And  the revision message carries the managed attribution trailer

  Scenario: The SCM-neutral commit path attributes a Fossil check-in
    Given  an initialized Fossil project
    When  the consumer commits with message "feat: parser" attributing LLM "Test-LLM", reasoning "high", and harness "opencode"
    Then  the command exits 0
    And  the revision message carries the managed attribution trailer

  Scenario: The commit wrapper records only the named paths on Fossil
    Given  an initialized Fossil project
    And  the project has an uncommitted change to "first.txt"
    And  the project has an uncommitted change to "second.txt"
    When  the consumer commits only "first.txt" with message "feat: first"
    Then  the command exits 0
    And  the latest revision contains "first.txt"
    And  the latest revision does not contain "second.txt"

  Scenario: The commit wrapper without attribution context leaves the message unchanged
    Given  a Givn project with the shared commit-attribution hook active
    When  the consumer commits with message "fix parser" without attribution context
    Then  the command exits 0
    And  the revision message carries no managed attribution trailer
  Scenario: The stage-all path excludes AGENTS.md and generated rules under local-only
    Given  a committed project root with givn/ initialised
    And  a tracked generated rule at ".opencode/skills/givn-propose/SKILL.md"
    And  a pending AGENTS.md user change
    And  an untracked project file at "src/lib.rs"
    When  the user runs `givn commit -m "feat: fixture content"`
    Then  the command exits 0
    And  the revision tracks "src/lib.rs"
    And  the latest revision does not touch "AGENTS.md"

  Scenario: The versioned stage-all path still excludes AGENTS.md
    Given  a committed project root with givn/ initialised
    And  a givn/config.yaml with rules "versioned"
    And  a pending AGENTS.md user change
    And  an untracked project file at "src/lib.rs"
    When  the user runs `givn commit -m "feat: fixture content"`
    Then  the command exits 0
    And  the revision tracks "src/lib.rs"
    And  the latest revision does not touch "AGENTS.md"
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
