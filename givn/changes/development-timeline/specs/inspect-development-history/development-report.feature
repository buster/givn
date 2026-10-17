@givn.delta @development-report

Feature: Development Report

  @givn.added
  Scenario: The report ranks the longest single step first
    Given a change "demo" whose record holds steps of different durations
    When the developer generates the development report for "demo"
    Then the report ranks the longest step first

  @givn.added
  Scenario: The report totals the time spent waiting for questions
    Given a change "demo" whose record holds two question waits
    When the developer generates the development report for "demo"
    Then the report names the total wait duration and the number of waits

  @givn.added
  Scenario: The report totals token usage per turn and per subagent
    Given a change "demo" whose record holds reported token usage
    When the developer generates the development report for "demo"
    Then the report names the total tokens for the change
    And the report names the tokens of every subagent
    And the report names the token total for every model that produced the usage

  @givn.added
  Scenario: The report of an ideation topic names the work done before the change exists
    Given an ideation topic "timeline-ideas" whose record holds the topic's own steps
    When the developer generates the development report for the topic "timeline-ideas"
    Then the report names the topic's steps under the topic scope
    And the report names no change that does not exist yet

  @givn.added
  Scenario: The report orders the timeline by recorded time, not by store position
    Given a change "demo" whose record holds an entry appended after an entry with a later end time
    When the developer generates the development report for "demo"
    Then the report orders the entries by their recorded end time

  @givn.added
  Scenario: The report names the agent's fan-out into small subagents
    Given a change "demo" whose record holds many short subagents
    When the developer generates the development report for "demo"
    Then the report names the number of subagents and their combined duration

  @givn.added
  Scenario: The report needs no network access to render
    Given a change "demo" with a recorded history
    When the developer generates the development report for "demo"
    Then the report references no remote resource

  @givn.added
  Scenario: The report shows the empty state for a scope with no recorded activity
    Given a change "demo" with no recorded activity
    When the developer generates the development report for "demo"
    Then the report shows the empty state for "demo"
    And the report draws no workflow phase for it

  @givn.added
  Scenario: The report derives the current workflow state
    Given a change "demo" whose record holds the workflow phases up to "tasks"
    And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
    When the developer generates the development report for "demo"
    Then the report names the current stage "tasks-done"
    And the report marks the phases before "review" done
    And the report marks "review" active
    And the report marks "archive" queued
    And the report names no phase the workflow does not have

  @givn.added
  Scenario: The report lays parallel sessions side by side
    Given a change "demo" whose record holds one phase with two sessions
    And one session is a subagent's own recorded session named "Architect.Scout"
    And one session holds a subagent span the record places in the spawning session
    When the developer generates the development report for "demo"
    Then the report draws one lane per session beside each other in that phase
    And the lane named "Architect.Scout" carries that subagent's own entries
    And the spawning session's lane keeps its own name and carries the subagent span
    And an entry whose session the record does not name stays in the unattributed lane
    And the report never assigns an entry to a subagent the record does not name
    And the report states when no tool call was attributable to a subagent span

  @givn.added
  Scenario: The report marks a skipped workflow phase
    Given a change "demo" whose record holds the workflow phases up to "tasks"
    And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
    And the change skips the "arc42-docs" artifact
    When the developer generates the development report for "demo"
    Then the report marks "arc42" skipped
    And the report does not mark "arc42" done

  @givn.added
  Scenario: The report of an archived change marks every phase done
    Given an archived change "demo" whose record holds the workflow phases up to "review"
    When the developer generates the development report for "demo"
    Then the report names no live change state
    And the report marks every workflow phase done
    And the report names no next required artifact

  @givn.added
  Scenario: The report states a record with no stage transition
    Given a change "demo" whose record holds no artifact transition
    When the developer generates the development report for "demo"
    Then the report groups the recorded work under no workflow phase
    And the report states that the record carries no stage transition
    And the report names no phase for that recorded work

  @givn.added
  Scenario: The report groups repeated steps
    Given a change "demo" whose record holds the same command six times in one phase
    When the developer generates the development report for "demo"
    Then the report names one repeated-step group with its count and combined duration
    And the report still ranks every measured entry

  @givn.added
  Scenario: A dense record is reported as aggregates instead of raw ticks
    Given a change "demo" whose record holds hundreds of short agent activities
    When the developer generates the development report for "demo"
    Then the report collapses the short agent activities into aggregate ticks on the phase lane
    And each aggregate tick names how many entries it covers and their total duration
    And the report summary still counts every recorded entry

  @givn.added
  Scenario: Measured agent activity is ranked alongside Givn's own steps
    Given a change "demo" whose record holds an agent tool call longer than any step
    When the developer generates the development report for "demo"
    Then the report ranks the agent tool call first
    And the ranking names the kind of every entry it lists

  @givn.added
  Scenario: The lane log carries the captured metadata and content
    Given a change "demo" whose record holds captured metadata and full-mode content
    When the developer generates the development report for "demo"
    Then the report model names the session, its title, the status, the stop reason, the cost, the context size and window, the tool call id, the output size, the error, the parent, the retry, and the permission mode
    And the report embeds the content within the 8 MiB documented budget
    And the report notes "content omitted; see the record" beyond the budget
    And the development record still holds every content byte

  @givn.added @wip
  Scenario: The written report's text view presents the workflow
    Given a change "demo" whose record holds the workflow phases up to "tasks"
    And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
    When the developer generates the development report for "demo"
    Then the written report names the current state and every workflow phase
    And the written report draws no band timeline

  @givn.added @e2e
  Scenario: Consumer generates the development report for a change
    Given a project root with givn/ initialised
    And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
    When the user runs `givn timeline --change demo --out report.html`
    Then the command exits 0
    And the command prints the path of the written report
    And the report file exists and names the workflow phases
    And the report file places the step recorded after that transition in the "implement" phase

  @givn.added @e2e
  Scenario Outline: Consumer opens the generated development report in a browser
    Given a <scope> "demo" with a recorded history
    And the development report for "demo" has been generated
    When the consumer opens the development report in a browser
    Then the browser shows the report heading and the current workflow state
    And the browser shows the workflow phases top to bottom
    And the browser shows the longest recorded step with its duration
    And the browser lays the page out in one column without page-level horizontal scrolling at the mobile viewport

    Examples:
      | scope  |
      | change |
      | topic  |

  @givn.added @e2e
  Scenario: Consumer expands a workflow phase and sees its lanes
    Given an open development report with a recorded history holding parallel sessions
    When the consumer expands the "implement" phase
    Then the browser shows that phase's vertical timeline with time flowing down
    And the browser shows one lane per session beside each other
    And the browser shows the Fit, 1 h, and 15 m zoom controls

  @givn.added @e2e
  Scenario: Consumer selects a timeline element and reads its details
    Given an open development report with a recorded history holding parallel sessions
    When the consumer selects the "Architect.Scout" lane
    Then the browser shows that lane's log
    And the log names the lane's start, its duration, and its outcome
    And the log names the entry's tool call id
    And the log renders the entry's content in its bounded content region

  @givn.added @e2e
  Scenario: Consumer zooms the timeline to a shorter time range
    Given an open development report with a recorded history
    When the consumer zooms the expanded phase's timeline in
    Then the browser shows a narrower time range than before

  @givn.added @e2e
  Scenario: Consumer opens the report of a scope with no recorded activity
    Given a change "demo" with no recorded activity
    And the development report for "demo" has been generated
    When the consumer opens the development report in a browser
    Then the browser shows the empty state for "demo"
    And the browser draws no workflow phase for it
