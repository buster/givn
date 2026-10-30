Feature: Agent Activity Reporting

  Scenario: A reported tool call is recorded for its scope
    Given  a project root with givn/ initialised
    And  a change "demo" exists with artifact "proposal.md"
    When  the agent reports a completed tool call for "demo"
    Then  the development record holds one agent entry for that tool call
    And  that entry names the tool and its measured duration

  Scenario: A subagent's activity is attributed to that subagent
    Given  a change "demo" with reported agent activity
    When  the agent reports a subagent that started and completed
    Then  the record holds one entry for that subagent
    And  that entry names the subagent and spans its start and its end

  Scenario: A question wait is recorded with its duration
    Given  a change "demo" with reported agent activity
    When  the agent reports that it asked a question and resumed
    Then  the record holds one entry for the question wait
    And  that entry carries the wait duration

  Scenario: Reported token usage is recorded with the model that produced it
    Given  a change "demo" with reported agent activity
    When  the agent reports a completed turn with token usage
    Then  the record holds that turn's token usage
    And  the usage names the model that produced it

  Scenario: Absent token usage is recorded as unavailable
    Given  a change "demo" with reported agent activity
    When  the agent reports a completed turn without token usage
    Then  the record marks that turn's token usage as unavailable
    And  the turn is still recorded

  Scenario: A reported event that does not match the intake contract is rejected without corrupting the record
    Given  a change "demo" with a recorded history
    When  the agent reports an event that does not match the intake contract
    Then  the intake names the rejected event
    And  the previously recorded history is unchanged

  @agent-activity-reporting
  Scenario: A rejected report never fails the agent action it reports
    Given  a change with recorded history whose development record cannot be written
    When  the installed reporter observes a completed tool call
    Then  the original tool result is returned unchanged
    And  the reporting failure neither fails nor indefinitely delays the agent action
    And  previously recorded history is unchanged

  Scenario: The intake rejects a payload that is not readable JSON
    Given  a project root with givn/ initialised
    When  the agent pipes a payload that is not JSON into `givn history ingest`
    Then  the intake exits 2
    And  the development record gains no entry

  Scenario: The intake keeps the acceptable events of a payload with a rejected event
    Given  a project root with givn/ initialised
    When  the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`
    Then  the intake exits 0
    And  stderr names the rejected event
    And  the development record holds the acceptable event

  Scenario Outline: Every supported harness receives a development reporter
    Given  a project root without a givn/ directory
    When  the consumer runs `givn init --agents <target>`
    Then  the command exits 0
    And  the project's generated guidance for "<target>" includes the development reporter
    Examples:
      | target |
      | omp |
      | opencode |
      | claude |
      | maki |

  Scenario: The neutral skills target receives no development reporter
    Given  a project root without a givn/ directory
    When  the consumer runs `givn init --agents agents`
    Then  the command exits 0
    And  no development reporter is installed for that project

  Scenario: A wait the harness does not time is still recorded with a duration
    Given  a project with the generated Claude Code development reporter
    When  the harness reports a permission request and then completion of that same tool call
    Then  the record holds one wait entry for the permission request
    And  that entry carries a duration measured by the reporter itself

  Scenario: Token usage read from the harness's own session record is recorded with its model
    Given  a project with the generated Claude Code development reporter
    And  the harness's session record holds assistant token usage for "claude-sonnet-5"
    When  the harness reports the end of a turn with that session record's path
    Then  the record holds that turn's token usage
    And  the usage names "claude-sonnet-5"

  Scenario: Maki's reporter keeps the harness's own tool duration and marks what its harness omits
    Given  a project with the generated Maki development reporter
    When  the Maki harness reports a completed tool call
    Then  the record holds that tool call with the duration the harness reported
    When  the Maki harness reports a tool call that never ran
    Then  the record holds that tool call without a measured duration
    When  the Maki harness reports a subagent that started and completed
    Then  the record holds that subagent's span
    And  the record marks that subagent's token usage as unavailable

  @e2e @agent-activity-reporting
  Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
    Given  an initialized project with the installed "<target>" reporter
    And  a change "demo" exists with artifact "proposal.md"
    When  the installed reporter reports a completed tool call, a subagent, and a question wait
    And  the consumer runs `givn history --change demo --json`
    Then  the command exits 0 and the machine-readable record names the reported activity
    And  the reporter describes the record as best-effort rather than complete
    And  local recording paths are gitignored and absent from tracked files and repository history
    And  the terminal transcript is captured for this scenario
    Examples:
      | target |
      | omp |
      | opencode |
      | claude |
      | maki |

  @agent-activity-reporting
  Scenario Outline: A stalled reporting helper cannot block primary agent work
    Given  an installed "<target>" reporter whose helper "<failure>"
    When  the agent completes a primary tool action
    Then  the primary action returns its own result within the finite reporter wait bound
    And  the outstanding helper is abandoned or stopped without an indefinite wait
    And  missing observations are not represented as a complete recording
    Examples:
      | target | failure |
      | omp | never exits |
      | opencode | never exits |
      | claude | never exits |
      | maki | never exits |
      | omp | does not consume its input |
      | opencode | does not consume its input |
      | claude | does not consume its input |
      | maki | does not consume its input |

  @agent-activity-reporting
  Scenario: Recording guidance does not confuse ignored paths with untracked history
    Given  a project whose local recording path was committed before it was ignored
    When  the developer reads the reporter installation or synchronization report
    Then  the report warns that ignoring a path does not remove tracked files or historical recordings
    And  the report identifies the existing tracked recording rather than claiming history is clean
    And  no automatic history rewrite is performed

  @agent-activity-reporting
  Scenario: Recording warnings disclose sensitivity without promising privacy filtering
    Given  a project with an installed development reporter
    When  the developer reads the effective recording guidance
    Then  the guidance warns that recordings can contain sensitive content
    And  it describes local gitignored recordings rather than repository-history storage by default
    And  it makes no redaction, privacy-boundary, or complete-capture guarantee
