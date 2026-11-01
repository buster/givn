@givn.delta @agent-activity-reporting
Feature: Agent Activity Reporting

  @givn.added
  Scenario: Complete mode records a Maki tool call's output from the session transcript
    Given a project with the generated Maki development reporter
    And the project records in the full mode
    When the Maki harness reports a completed tool call whose transcript holds the matching result
    Then the record holds that tool call's output text

  @givn.added
  Scenario: A tool-spawned Maki subagent is recorded once with the harness's own duration
    Given a project with the generated Maki development reporter
    When the Maki harness reports a subagent start, its completion, and the spawning tool completion
    Then the record holds one subagent entry for that subagent
    And that entry names the agent, its title, and the duration the harness reported

  @givn.added
  Scenario: A paused Maki subagent stays open until it finishes
    Given a project with the generated Maki development reporter
    When the Maki harness reports a subagent that becomes idle and then finishes
    Then the record holds one completed entry for that subagent

  @givn.added
  Scenario: A stopped Maki subagent is recorded as skipped
    Given a project with the generated Maki development reporter
    When the Maki harness reports a subagent that the user stops
    Then the record holds one skipped entry for that subagent

  @givn.added
  Scenario: A Maki subagent moved to the background stays open until it finishes
    Given a project with the generated Maki development reporter
    And the Maki task list still lists the subagent after its tool completion
    When the Maki harness reports the subagent's later completion
    Then the record holds one completed entry for that subagent
    And that entry keeps the title and duration its tool completion reported

  @givn.added
  Scenario: A Maki subagent still running when the session ends is recorded as unknown
    Given a project with the generated Maki development reporter
    When the Maki harness ends its session while a subagent is still running
    Then the record holds one unknown entry for that subagent
    And that entry carries the elapsed time observed for it

  @givn.added
  Scenario: A Maki tool call moved to the background is not recorded as completed
    Given a project with the generated Maki development reporter
    And the Maki session transcript names a moved tool call
    When the Maki harness reports that tool call's completion
    Then the record holds that tool call with an unknown outcome

  @givn.added
  Scenario: A Maki question records its answer, source, and time
    Given a project with the generated Maki development reporter
    And the Maki session transcript holds a question and its answer
    When the Maki harness reports that the question wait resumed
    Then the record holds the answer text, its source, and its time

  @givn.added
  Scenario: A Maki session title is recorded as a session entry
    Given a project with the generated Maki development reporter
    When the Maki harness reports that its session title changed
    Then the record holds a session entry naming that title

  @givn.added
  Scenario: A Maki subagent's tool calls are recorded under its own session
    Given a project with the generated Maki development reporter
    And the Maki task list names the subagent's session
    And that subagent's transcript holds a completed tool call
    When the Maki harness reports the subagent's completion
    Then the record holds that tool call under the subagent's session

  @givn.added
  Scenario: A Maki subagent's cost is recorded where the harness reports it
    Given a project with the generated Maki development reporter
    And the Maki task list reports the subagent's cost
    When the Maki harness reports the subagent's cost
    Then the record holds that cost with the subagent
