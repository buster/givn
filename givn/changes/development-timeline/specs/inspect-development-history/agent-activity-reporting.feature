@givn.delta @agent-activity-reporting

Feature: Agent Activity Reporting

  @givn.added
  Scenario: A reported tool call is recorded for its scope
    Given a project root with givn/ initialised
    And a change "demo" exists with artifact "proposal.md"
    When the agent reports a completed tool call for "demo"
    Then the development record holds one agent entry for that tool call
    And that entry names the tool and its measured duration

  @givn.added @wip
  Scenario: A subagent's activity is attributed to that subagent
    Given a change "demo" with reported agent activity
    When the agent reports a subagent that started and completed
    Then the record holds one entry for that subagent
    And that entry names the subagent and spans its start and its end

  @givn.added @wip
  Scenario: A question wait is recorded with its duration
    Given a change "demo" with reported agent activity
    When the agent reports that it asked a question and resumed
    Then the record holds one entry for the question wait
    And that entry carries the wait duration

  @givn.added @wip
  Scenario: Reported token usage is recorded with the model that produced it
    Given a change "demo" with reported agent activity
    When the agent reports a completed turn with token usage
    Then the record holds that turn's token usage
    And the usage names the model that produced it

  @givn.added @wip
  Scenario: Absent token usage is recorded as unavailable
    Given a change "demo" with reported agent activity
    When the agent reports a completed turn without token usage
    Then the record marks that turn's token usage as unavailable
    And the turn is still recorded

  @givn.added @wip
  Scenario: A reported event that does not match the intake contract is rejected without corrupting the record
    Given a change "demo" with a recorded history
    When the agent reports an event that does not match the intake contract
    Then the intake names the rejected event
    And the previously recorded history is unchanged

  @givn.added @wip
  Scenario: A rejected report never fails the agent action it reports
    Given a change "demo" with a recorded history
    And the development record cannot be written
    When the agent reports a completed tool call
    Then the intake reports no failure to the agent

  @givn.added @wip
  Scenario: The intake rejects a payload that is not readable JSON
    Given a project root with givn/ initialised
    When the agent pipes a payload that is not JSON into `givn history ingest`
    Then the intake exits 2
    And the development record gains no entry

  @givn.added @wip
  Scenario: The intake keeps the acceptable events of a payload with a rejected event
    Given a project root with givn/ initialised
    When the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`
    Then the intake exits 0
    And stderr names the rejected event
    And the development record holds the acceptable event

  @givn.added @wip
  Scenario Outline: Every supported harness receives a development reporter
    Given a project root without a givn/ directory
    When the consumer runs `givn init --agents <target>`
    Then the command exits 0
    And the project's generated guidance for "<target>" includes the development reporter

    Examples:
      | target   |
      | omp      |
      | opencode |
      | claude   |

  @givn.added @wip
  Scenario: The neutral skills target receives no development reporter
    Given a project root without a givn/ directory
    When the consumer runs `givn init --agents agents`
    Then the command exits 0
    And no development reporter is installed for that project

  @givn.added @wip
  Scenario: A wait the harness does not time is still recorded with a duration
    Given a project with the generated Claude Code development reporter
    When the harness reports a permission request and then completion of that same tool call
    Then the record holds one wait entry for the permission request
    And that entry carries a duration measured by the reporter itself

  @givn.added @wip
  Scenario: Token usage read from the harness's own session record is recorded with its model
    Given a project with the generated Claude Code development reporter
    And the harness's session record holds assistant token usage for "claude-sonnet-5"
    When the harness reports the end of a turn with that session record's path
    Then the record holds that turn's token usage
    And the usage names "claude-sonnet-5"

  @givn.added @e2e @wip
  Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
    Given a project root with givn/ initialised
    And the change agent target "<target>" installs the development reporter
    And a change "demo" exists with artifact "proposal.md"
    When the installed "<target>" reporter reports a completed tool call, a subagent, and a question wait
    And the user runs `givn history --change demo --json`
    Then the command exits 0
    And the machine-readable record names the tool call, the subagent, and the question wait

    Examples:
      | target   |
      | omp      |
      | opencode |
      | claude   |
