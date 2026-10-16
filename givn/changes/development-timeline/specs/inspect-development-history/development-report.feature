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

  @givn.added @wip
  Scenario: The report totals token usage per turn and per subagent
    Given a change "demo" whose record holds reported token usage
    When the developer generates the development report for "demo"
    Then the report names the total tokens for the change
    And the report names the tokens of every subagent
    And the report names the token total for every model that produced the usage

  @givn.added @wip
  Scenario: The report of an ideation topic names the work done before the change exists
    Given an ideation topic "timeline-ideas" whose record holds the topic's own steps
    When the developer generates the development report for the topic "timeline-ideas"
    Then the report names the topic's steps under the topic scope
    And the report names no change that does not exist yet

  @givn.added @wip
  Scenario: The report orders the timeline by recorded time, not by store position
    Given a change "demo" whose record holds an entry appended after an entry with a later end time
    When the developer generates the development report for "demo"
    Then the report orders the entries by their recorded end time

  @givn.added @wip
  Scenario: The report names the agent's fan-out into small subagents
    Given a change "demo" whose record holds many short subagents
    When the developer generates the development report for "demo"
    Then the report names the number of subagents and their combined duration

  @givn.added @wip
  Scenario: The report needs no network access to render
    Given a change "demo" with a recorded history
    When the developer generates the development report for "demo"
    Then the report references no remote resource

  @givn.added @wip
  Scenario: The report shows the empty state for a scope with no recorded activity
    Given a change "demo" with no recorded activity
    When the developer generates the development report for "demo"
    Then the report shows the empty state for "demo"
    And the report draws no timeline axis for it

  @givn.added @wip
  Scenario: A dense record is reported as aggregates instead of raw ticks
    Given a change "demo" whose record holds hundreds of short agent activities
    When the developer generates the development report for "demo"
    Then the report collapses the short agent activities into aggregate markers
    And each aggregate marker names how many entries it covers and their total duration
    And the report summary still counts every recorded entry

  @givn.added @e2e @wip
  Scenario: Consumer generates the development report for a change
    Given a project root with givn/ initialised
    And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
    When the user runs `givn timeline --change demo --out report.html`
    Then the command exits 0
    And the command prints the path of the written report
    And the report file exists and names the recorded phases
    And the report file places the step recorded after that transition in the "implement" phase

  @givn.added @e2e @wip
  Scenario Outline: Consumer opens the generated development report in a browser
    Given a <scope> "demo" with a recorded history
    And the development report for "demo" has been generated
    When the consumer opens the development report in a browser
    Then the browser shows the report heading and the development timeline
    And the browser shows the longest recorded step with its duration

    Examples:
      | scope  |
      | change |
      | topic  |

  @givn.added @e2e @wip
  Scenario: Consumer selects a timeline element and reads its details
    Given an open development report with a recorded history
    When the consumer selects the longest recorded step
    Then the browser shows a details panel for that step
    And the panel names its start, its duration, and its outcome

  @givn.added @e2e @wip
  Scenario: Consumer zooms the timeline to a shorter time range
    Given an open development report with a recorded history
    When the consumer zooms the timeline in
    Then the browser shows a narrower time range than before

  @givn.added @e2e @wip
  Scenario: Consumer opens the report of a scope with no recorded activity
    Given a change "demo" with no recorded activity
    And the development report for "demo" has been generated
    When the consumer opens the development report in a browser
    Then the browser shows the empty state for "demo"
    And the browser draws no timeline axis for it
