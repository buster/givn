Feature: Attribute commits

  @e2e
  Scenario Outline: Consumer initializes a project and receives shared attribution plumbing
    Given  a project root without a givn/ directory and with a Git repository
    When  the consumer runs `givn init --agents opencode,claude,omp`
    Then  the command exits 0
    And  stdout reports the OpenCode attribution integration
    And  the checked-in commit attribution hook exists
    And  the Git repository uses the checked-in commit attribution hook
    And  the Claude Code attribution integration exists
    And  the omp attribution integration exists
    And  an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"
    Examples:
      | release |
      | V1 floor |
      | V1 latest |
      | V2 latest |

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
  Scenario Outline: OpenCode prepares attribution using its selected model
    Given  a project with the generated OpenCode attribution integration
    When  the developer asks OpenCode "<release>" with model "givn-fixture/model-a" and reasoning mode "high" to execute the attribution fixture command
    Then  the OpenCode command exits 0
    And  the OpenCode command output reports active commit attribution
    And  the OpenCode command output identifies LLM "givn-fixture/model-a"
    And  the OpenCode command output identifies reasoning mode "high"
    And  the OpenCode command output identifies harness "opencode"
    And  the OpenCode command output identifies committer "LLM: givn-fixture/model-a via opencode"
    Examples:
      | release |
      | V1 floor |
      | V1 latest |
      | V2 latest |

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

  Scenario Outline: OpenCode accepts either supplied model identifier spelling
    Given  an isolated OpenCode "<generation>" attribution integration
    When  OpenCode reports provider "givn-fixture" with model "model-a" using identifier "<identifier>" for session "session-a"
    And  OpenCode prepares an attributed shell for session "session-a"
    Then  the shell environment identifies LLM "givn-fixture/model-a"
    Examples:
      | generation | identifier |
      | V1 | id |
      | V1 | modelID |
      | V2 | id |
      | V2 | modelID |

  Scenario Outline: A remembered session model takes precedence over a later model from another session
    Given  an isolated OpenCode "<generation>" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When  OpenCode prepares an attributed shell for session "session-a"
    Then  the shell environment identifies LLM "givn-fixture/model-a"
    And  the shell environment identifies reasoning mode "high"
    Examples:
      | generation |
      | V1 |
      | V2 |

  Scenario Outline: Omitted reasoning metadata retains the session's remembered variant
    Given  an isolated OpenCode "<generation>" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    When  OpenCode reports model "givn-fixture/model-b" without reasoning metadata for session "session-a"
    And  OpenCode prepares an attributed shell for session "session-a"
    Then  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies reasoning mode "high"
    Examples:
      | generation |
      | V1 |
      | V2 |

  Scenario Outline: A subagent's shell command is attributed to the subagent's own model
    Given  an isolated OpenCode "<generation>" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "main-session"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for subagent session "child-session" of "main-session"
    When  OpenCode runs the shell command "git commit" for session "<issuing session>"
    Then  the shell environment identifies LLM "<model>"
    And  the shell environment identifies reasoning mode "<reasoning>"
    And  the shell environment identifies committer "LLM: <model> via opencode"
    Examples:
      | generation | issuing session | model | reasoning |
      | V1 | main-session | givn-fixture/model-a | high |
      | V1 | child-session | givn-fixture/model-b | low |
      | V2 | main-session | givn-fixture/model-a | high |
      | V2 | child-session | givn-fixture/model-b | low |

  Scenario: Identical shell commands from two OpenCode V2 sessions are attributed in the order issued
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When  OpenCode requests the shell command "git commit" for session "session-a"
    And  OpenCode requests the shell command "git commit" for session "session-b"
    And  OpenCode creates both shells
    Then  the first shell environment identifies LLM "givn-fixture/model-a"
    And  the second shell environment identifies LLM "givn-fixture/model-b"

  Scenario: A blocked shell request does not label a later identical command from another session
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And  OpenCode has later remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode requested the shell command "git commit" for session "session-a" in message "message-1" and that request never reached shell creation
    When  OpenCode requests the shell command "git status" for session "session-a" in message "message-2"
    And  OpenCode requests the shell command "git commit" for session "session-b" in message "message-3"
    And  OpenCode creates the shell for "git commit"
    Then  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies reasoning mode "low"
    And  the shell environment identifies committer "LLM: givn-fixture/model-b via opencode"

  Scenario: A request from an older message of the same session is dropped
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And  OpenCode requested the shell command "git commit" for session "session-a" in message "message-1" and that request never reached shell creation
    When  OpenCode requests the shell command "git status" for session "session-a" in message "message-2"
    And  OpenCode creates the shell for "git commit"
    Then  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies reasoning mode "low"

  Scenario: Parallel shell requests in one message are all attributed
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When  OpenCode requests the shell command "git status" for session "session-a" in message "message-1"
    And  OpenCode requests the shell command "git commit" for session "session-a" in message "message-1"
    And  OpenCode creates both shells
    Then  the first shell environment identifies LLM "givn-fixture/model-a"
    And  the second shell environment identifies LLM "givn-fixture/model-a"

  Scenario: A completed shell request is cleared by its completion event
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And  OpenCode requested the shell command "git commit" for session "session-a" in message "message-1" and that request never reached shell creation
    When  OpenCode reports the shell command "git commit" of session "session-a" in message "message-1" as completed
    And  OpenCode requests the shell command "git commit" for session "session-b" in message "message-2"
    And  OpenCode creates the shell for "git commit"
    Then  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies reasoning mode "low"

  Scenario: Malformed tool events are ignored without blocking the shell
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When  OpenCode delivers tool events that are not shell requests or lack a command, a session or an input
    And  OpenCode requests the shell command "git commit" for session "session-a"
    And  OpenCode creates the shell for "git commit"
    Then  shell preparation succeeds
    And  the shell environment identifies LLM "givn-fixture/model-a"
    And  the shell environment identifies reasoning mode "high"
    And  the shell environment identifies committer "LLM: givn-fixture/model-a via opencode"

  Scenario: A shell request without a session does not shadow a later request
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When  OpenCode requests the shell command "git commit" without a session
    And  OpenCode requests the shell command "git commit" for session "session-a"
    And  OpenCode creates the shell for "git commit"
    Then  the shell environment identifies LLM "givn-fixture/model-a"
    And  the shell environment identifies reasoning mode "high"

  Scenario: A shell request without a message or call identifier cannot keep or wipe other entries
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When  OpenCode requests the shell command "git commit" for session "session-a" in message "message-1"
    And  OpenCode requests the shell command "git status" for session "session-a" without a call identifier
    And  OpenCode reports the shell command "git status" of session "session-a" as completed without a call identifier
    And  OpenCode requests the shell command "git log" for session "session-a" without a message identifier
    And  OpenCode creates the shell for "git commit"
    And  OpenCode creates the shell for "git status"
    And  OpenCode creates the shell for "git log"
    Then  the first shell environment identifies LLM "givn-fixture/model-a"
    And  the second shell environment identifies LLM "givn-fixture/model-b"
    And  the third shell environment identifies LLM "givn-fixture/model-b"

  Scenario: A rejected hook registration does not stop shell attribution
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And  OpenCode rejects the registration of its "tool" hook "execute.before"
    When  OpenCode creates the shell for "git commit"
    Then  shell preparation succeeds
    And  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies committer "LLM: givn-fixture/model-b via opencode"

  Scenario: A rejected hook registration is reported once
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode rejects the registration of its "tool" hook "execute.before"
    When  OpenCode sets up the integration a second time
    And  OpenCode creates the shell for "git commit"
    Then  shell preparation succeeds
    And  the integration warns exactly once that the registration of "tool.execute.before" failed

  Scenario: A failing diagnostic channel does not stop hook registration
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And  OpenCode rejects the registration of its "tool" hook "execute.before"
    And  OpenCode's diagnostic output throws
    When  OpenCode creates the shell for "git commit"
    Then  shell preparation succeeds
    And  the integration warns exactly once that the registration of "tool.execute.before" failed
    And  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies committer "LLM: givn-fixture/model-b via opencode"

  Scenario: A hung configured-default lookup does not stall shell creation
    Given  an isolated OpenCode "V2" attribution integration
    And  no model has been remembered
    And  OpenCode's configured-default lookup is "hangs"
    And  the shell already identifies committer "Human Developer"
    When  OpenCode prepares an attributed shell with "no session"
    Then  shell preparation succeeds
    And  the shell environment enables commit attribution
    And  the shell environment identifies LLM "unavailable"
    And  the shell environment identifies committer "Human Developer"

  Scenario: A cancelled configured-default lookup still leaves attribution unavailable
    Given  an isolated OpenCode "V2" attribution integration
    And  no model has been remembered
    And  OpenCode's configured-default lookup is "hangs"
    And  the shell already identifies committer "Human Developer"
    When  OpenCode creates the shell for "git commit" and abandons the hook after its first tick
    Then  the shell environment enables commit attribution
    And  the shell environment identifies LLM "unavailable"
    And  the shell environment identifies reasoning mode "unavailable"
    And  the shell environment identifies harness "opencode"
    And  the shell environment identifies committer "Human Developer"

  Scenario: Absent hook payloads and a missing shell environment never fail attribution
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    When  OpenCode delivers no payload to any attribution hook
    And  OpenCode creates the shell for "git commit" without an environment
    And  OpenCode requests the shell command "git commit" for session "session-a"
    And  OpenCode creates the shell for "git commit"
    Then  shell preparation succeeds
    And  the shell environment identifies LLM "givn-fixture/model-a"
    And  the shell environment identifies committer "LLM: givn-fixture/model-a via opencode"

  @e2e
  Scenario: OpenCode V2 attributes a subagent's command to the subagent's model
    Given  a project with the generated OpenCode attribution integration
    And  an event recorder is installed beside the attribution plugin in the project
    And  the project's subagent is configured with model "givn-fixture/model-b" and reasoning mode "low"
    When  the developer asks OpenCode "V2 latest" with model "givn-fixture/model-a" and reasoning mode "high" to have its subagent execute the attribution fixture command and then execute it itself
    Then  the OpenCode command exits 0
    And  the subagent's command output identifies LLM "givn-fixture/model-b"
    And  the subagent's command output identifies reasoning mode "low"
    And  the subagent's command output identifies committer "LLM: givn-fixture/model-b via opencode"
    And  the primary session's own command output identifies LLM "givn-fixture/model-a"
    And  the primary session's own command output identifies reasoning mode "high"
    And  the primary session's own command output identifies committer "LLM: givn-fixture/model-a via opencode"
    And  the primary session's command ran after the subagent's command completed
    And  OpenCode announced the subagent's command for the subagent's own session
    And  shell creation carried no session identifier in its environment

  @e2e
  Scenario: OpenCode V2 delivers the shell request to the attribution adapter before shell creation
    Given  a project with the generated OpenCode attribution integration
    And  an event recorder is installed beside the attribution plugin in the project
    When  the developer asks OpenCode "V2 latest" with model "givn-fixture/model-a" and reasoning mode "high" to execute the attribution fixture command
    Then  the OpenCode command exits 0
    And  the recorded shell events arrive as tool start, shell creation, tool completion
    And  the shell creation command equals the tool start command
    And  the tool start event carries the tool name, session, message and call identifiers
    And  the characterization harness fires events with the same key names

  Scenario Outline: A shell without a remembered session uses the last observed model
    Given  an isolated OpenCode "<generation>" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    When  OpenCode prepares an attributed shell with "<session context>"
    Then  the shell environment identifies LLM "givn-fixture/model-a"
    And  the shell environment identifies reasoning mode "high"
    And  the shell environment identifies committer "LLM: givn-fixture/model-a via opencode"
    Examples:
      | generation | session context |
      | V1 | unknown session |
      | V2 | unknown session |
      | V2 | no session |

  Scenario: The remembered model takes precedence over OpenCode V2's configured default
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And  OpenCode's configured default is model "givn-fixture/model-b" with reasoning mode "low"
    When  OpenCode prepares an attributed shell with "no session"
    Then  the shell environment identifies LLM "givn-fixture/model-a"
    And  the shell environment identifies reasoning mode "high"

  Scenario: OpenCode V2 uses its configured default when no model has been remembered
    Given  an isolated OpenCode "V2" attribution integration
    And  OpenCode's configured default is model "givn-fixture/model-b" with reasoning mode "low"
    When  OpenCode prepares an attributed shell with "no session"
    Then  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies reasoning mode "low"
    And  the shell environment identifies committer "LLM: givn-fixture/model-b via opencode"

  Scenario Outline: No usable model leaves attribution unavailable without changing the committer
    Given  an isolated OpenCode "<generation>" attribution integration
    And  no model has been remembered
    And  OpenCode's configured-default lookup is "<fallback>"
    And  the shell already identifies committer "Human Developer"
    When  OpenCode prepares an attributed shell with "unknown session"
    Then  shell preparation succeeds
    And  the shell environment enables commit attribution
    And  the shell environment identifies harness "opencode"
    And  the shell environment identifies LLM "unavailable"
    And  the shell environment identifies reasoning mode "unavailable"
    And  the shell environment identifies committer "Human Developer"
    Examples:
      | generation | fallback |
      | V1 | absent |
      | V2 | absent |
      | V2 | no model |
      | V2 | model without identity |
      | V2 | fails |
      | V2 | throws synchronously |

  Scenario Outline: OpenCode V2 remembers attribution during non-title model requests
    Given  an isolated OpenCode "V2" attribution integration
    When  OpenCode reports model "givn-fixture/model-b" with reasoning mode "low" for session "session-a" during "<request>"
    And  OpenCode prepares an attributed shell for session "session-a"
    Then  the shell environment identifies LLM "givn-fixture/model-b"
    And  the shell environment identifies reasoning mode "low"
    Examples:
      | request |
      | primary conversation |
      | compaction |
      | transient generation |
