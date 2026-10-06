@givn.delta @attribute-commits
Feature: Attribute commits

  @givn.modified @e2e
  Scenario Outline: Consumer initializes a project and receives shared attribution plumbing
    Given a project root without a givn/ directory and with a Git repository
    When the consumer runs `givn init --agents opencode,claude`
    Then the command exits 0
    And stdout reports the OpenCode attribution integration
    And the checked-in commit attribution hook exists
    And the Git repository uses the checked-in commit attribution hook
    And the Claude Code attribution integration exists
    And an attributed shell command through OpenCode "<release>" reports model "givn-fixture/model-a" and reasoning mode "high"

    Examples:
      | release   |
      | V1 floor  |
      | V1 latest |
      | V2 latest |

  @givn.modified @e2e
  Scenario Outline: OpenCode prepares attribution using its selected model
    Given a project with the generated OpenCode attribution integration
    When the developer asks OpenCode "<release>" with model "givn-fixture/model-a" and reasoning mode "high" to execute the attribution fixture command
    Then the OpenCode command exits 0
    And the OpenCode command output reports active commit attribution
    And the OpenCode command output identifies LLM "givn-fixture/model-a"
    And the OpenCode command output identifies reasoning mode "high"
    And the OpenCode command output identifies harness "opencode"
    And the OpenCode command output identifies committer "LLM: givn-fixture/model-a via opencode"

    Examples:
      | release   |
      | V1 floor  |
      | V1 latest |
      | V2 latest |

  @givn.added
  Scenario Outline: OpenCode accepts either supplied model identifier spelling
    Given an isolated OpenCode "<generation>" attribution integration
    When OpenCode reports provider "givn-fixture" with model "model-a" using identifier "<identifier>" for session "session-a"
    And OpenCode prepares an attributed shell for session "session-a"
    Then the shell environment identifies LLM "givn-fixture/model-a"

    Examples:
      | generation | identifier |
      | V1         | id         |
      | V1         | modelID    |
      | V2         | id         |
      | V2         | modelID    |

  @givn.added
  Scenario Outline: A remembered session model takes precedence over a later model from another session
    Given an isolated OpenCode "<generation>" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When OpenCode prepares an attributed shell for session "session-a"
    Then the shell environment identifies LLM "givn-fixture/model-a"
    And the shell environment identifies reasoning mode "high"

    Examples:
      | generation |
      | V1         |
      | V2         |

  @givn.added
  Scenario Outline: Omitted reasoning metadata retains the session's remembered variant
    Given an isolated OpenCode "<generation>" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    When OpenCode reports model "givn-fixture/model-b" without reasoning metadata for session "session-a"
    And OpenCode prepares an attributed shell for session "session-a"
    Then the shell environment identifies LLM "givn-fixture/model-b"
    And the shell environment identifies reasoning mode "high"

    Examples:
      | generation |
      | V1         |
      | V2         |

  @givn.added
  Scenario Outline: A subagent's shell command is attributed to the subagent's own model
    Given an isolated OpenCode "<generation>" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "main-session"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for subagent session "child-session" of "main-session"
    When OpenCode runs the shell command "git commit" for session "<issuing session>"
    Then the shell environment identifies LLM "<model>"
    And the shell environment identifies reasoning mode "<reasoning>"
    And the shell environment identifies committer "LLM: <model> via opencode"

    Examples:
      | generation | issuing session | model                | reasoning |
      | V1         | main-session    | givn-fixture/model-a | high      |
      | V1         | child-session   | givn-fixture/model-b | low       |
      | V2         | main-session    | givn-fixture/model-a | high      |
      | V2         | child-session   | givn-fixture/model-b | low       |

  @givn.added
  Scenario: Identical shell commands from two OpenCode V2 sessions are attributed in the order issued
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When OpenCode requests the shell command "git commit" for session "session-a"
    And OpenCode requests the shell command "git commit" for session "session-b"
    And OpenCode creates both shells
    Then the first shell environment identifies LLM "givn-fixture/model-a"
    And the second shell environment identifies LLM "givn-fixture/model-b"

  @givn.added
  Scenario: A blocked shell request does not label a later identical command from another session
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And OpenCode has later remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode requested the shell command "git commit" for session "session-a" in message "message-1" and that request never reached shell creation
    When OpenCode requests the shell command "git status" for session "session-a" in message "message-2"
    And OpenCode requests the shell command "git commit" for session "session-b" in message "message-3"
    And OpenCode creates the shell for "git commit"
    Then the shell environment identifies LLM "givn-fixture/model-b"
    And the shell environment identifies reasoning mode "low"
    And the shell environment identifies committer "LLM: givn-fixture/model-b via opencode"

  @givn.added
  Scenario: A request from an older message of the same session is dropped
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And OpenCode requested the shell command "git commit" for session "session-a" in message "message-1" and that request never reached shell creation
    When OpenCode requests the shell command "git status" for session "session-a" in message "message-2"
    And OpenCode creates the shell for "git commit"
    Then the shell environment identifies LLM "givn-fixture/model-b"
    And the shell environment identifies reasoning mode "low"

  @givn.added
  Scenario: Parallel shell requests in one message are all attributed
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When OpenCode requests the shell command "git status" for session "session-a" in message "message-1"
    And OpenCode requests the shell command "git commit" for session "session-a" in message "message-1"
    And OpenCode creates both shells
    Then the first shell environment identifies LLM "givn-fixture/model-a"
    And the second shell environment identifies LLM "givn-fixture/model-a"

  @givn.added
  Scenario: A completed shell request is cleared by its completion event
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    And OpenCode requested the shell command "git commit" for session "session-a" in message "message-1" and that request never reached shell creation
    When OpenCode reports the shell command "git commit" of session "session-a" in message "message-1" as completed
    And OpenCode requests the shell command "git commit" for session "session-b" in message "message-2"
    And OpenCode creates the shell for "git commit"
    Then the shell environment identifies LLM "givn-fixture/model-b"
    And the shell environment identifies reasoning mode "low"

  @givn.added
  Scenario: Malformed tool events are ignored without blocking the shell
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode has later remembered model "givn-fixture/model-b" with reasoning mode "low" for session "session-b"
    When OpenCode delivers tool events that are not shell requests or lack a command, a session or an input
    And OpenCode requests the shell command "git commit" for session "session-a"
    And OpenCode creates the shell for "git commit"
    Then shell preparation succeeds
    And the shell environment identifies LLM "givn-fixture/model-a"
    And the shell environment identifies reasoning mode "high"
    And the shell environment identifies committer "LLM: givn-fixture/model-a via opencode"

  @givn.added @e2e
  Scenario: OpenCode V2 attributes a subagent's command to the subagent's model
    Given a project with the generated OpenCode attribution integration
    And the project's subagent is configured with model "givn-fixture/model-b" and reasoning mode "low"
    When the developer asks OpenCode "V2 latest" with model "givn-fixture/model-a" and reasoning mode "high" to have its subagent execute the attribution fixture command
    Then the OpenCode command exits 0
    And the subagent's command output identifies LLM "givn-fixture/model-b"
    And the subagent's command output identifies reasoning mode "low"
    And the subagent's command output identifies committer "LLM: givn-fixture/model-b via opencode"

  @givn.added @e2e
  Scenario: OpenCode V2 delivers the shell request to the attribution adapter before shell creation
    Given a project with the generated OpenCode attribution integration
    And an event recorder is installed beside the attribution plugin in the project
    When the developer asks OpenCode "V2 latest" with model "givn-fixture/model-a" and reasoning mode "high" to execute the attribution fixture command
    Then the OpenCode command exits 0
    And the recorded shell events arrive as tool start, shell creation, tool completion
    And the shell creation command equals the tool start command
    And the tool start event carries the tool name, session, message and call identifiers
    And the characterization harness fires events with the same key names

  @givn.added
  Scenario Outline: A shell without a remembered session uses the last observed model
    Given an isolated OpenCode "<generation>" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    When OpenCode prepares an attributed shell with "<session context>"
    Then the shell environment identifies LLM "givn-fixture/model-a"
    And the shell environment identifies reasoning mode "high"
    And the shell environment identifies committer "LLM: givn-fixture/model-a via opencode"

    Examples:
      | generation | session context   |
      | V1         | unknown session   |
      | V2         | unknown session   |
      | V2         | no session        |

  @givn.added
  Scenario: The remembered model takes precedence over OpenCode V2's configured default
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode has remembered model "givn-fixture/model-a" with reasoning mode "high" for session "session-a"
    And OpenCode's configured default is model "givn-fixture/model-b" with reasoning mode "low"
    When OpenCode prepares an attributed shell with "no session"
    Then the shell environment identifies LLM "givn-fixture/model-a"
    And the shell environment identifies reasoning mode "high"

  @givn.added
  Scenario: OpenCode V2 uses its configured default when no model has been remembered
    Given an isolated OpenCode "V2" attribution integration
    And OpenCode's configured default is model "givn-fixture/model-b" with reasoning mode "low"
    When OpenCode prepares an attributed shell with "no session"
    Then the shell environment identifies LLM "givn-fixture/model-b"
    And the shell environment identifies reasoning mode "low"
    And the shell environment identifies committer "LLM: givn-fixture/model-b via opencode"

  @givn.added
  Scenario Outline: No usable model leaves attribution unavailable without changing the committer
    Given an isolated OpenCode "<generation>" attribution integration
    And no model has been remembered
    And OpenCode's configured-default lookup is "<fallback>"
    And the shell already identifies committer "Human Developer"
    When OpenCode prepares an attributed shell with "unknown session"
    Then shell preparation succeeds
    And the shell environment enables commit attribution
    And the shell environment identifies harness "opencode"
    And the shell environment identifies LLM "unavailable"
    And the shell environment identifies reasoning mode "unavailable"
    And the shell environment identifies committer "Human Developer"

    Examples:
      | generation | fallback                |
      | V1         | absent                  |
      | V2         | absent                  |
      | V2         | no model                |
      | V2         | model without identity  |
      | V2         | fails                   |

  @givn.added
  Scenario Outline: OpenCode V2 remembers attribution during non-title model requests
    Given an isolated OpenCode "V2" attribution integration
    When OpenCode reports model "givn-fixture/model-b" with reasoning mode "low" for session "session-a" during "<request>"
    And OpenCode prepares an attributed shell for session "session-a"
    Then the shell environment identifies LLM "givn-fixture/model-b"
    And the shell environment identifies reasoning mode "low"

    Examples:
      | request                 |
      | primary conversation    |
      | compaction              |
      | transient generation    |
