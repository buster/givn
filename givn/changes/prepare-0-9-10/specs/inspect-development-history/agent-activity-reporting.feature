@givn.delta @agent-activity-reporting
Feature: Best-effort bounded agent reporting

  @givn.modified @e2e
  Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
    Given an initialized project with the installed "<target>" reporter
    And a change "demo" exists with artifact "proposal.md"
    When the installed reporter reports a completed tool call, a subagent, and a question wait
    And the consumer runs `givn history --change demo --json`
    Then the command exits 0 and the machine-readable record names the reported activity
    And the reporter describes the record as best-effort rather than complete
    And local recording paths are gitignored and absent from tracked files and repository history

    Examples:
      | target   |
      | omp      |
      | opencode |
      | claude   |
      | maki     |

  @givn.modified
  Scenario: A rejected report never fails the agent action it reports
    Given a change with recorded history whose development record cannot be written
    When the installed reporter observes a completed tool call
    Then the original tool result is returned unchanged
    And the reporting failure neither fails nor indefinitely delays the agent action
    And previously recorded history is unchanged

  @givn.added
  Scenario Outline: A stalled reporting helper cannot block primary agent work
    Given an installed "<target>" reporter whose helper "<failure>"
    When the agent completes a primary tool action
    Then the primary action returns its own result within the finite reporter wait bound
    And the outstanding helper is abandoned or stopped without an indefinite wait
    And missing observations are not represented as a complete recording

    Examples:
      | target   | failure                              |
      | omp      | never exits                          |
      | opencode | never exits                          |
      | claude   | never exits                          |
      | maki     | never exits                          |
      | omp      | does not consume its input           |
      | opencode | does not consume its input           |
      | claude   | does not consume its input           |
      | maki     | does not consume its input           |

  @givn.added
  Scenario: Recording guidance does not confuse ignored paths with untracked history
    Given a project whose local recording path was committed before it was ignored
    When the developer reads the reporter installation or synchronization report
    Then the report warns that ignoring a path does not remove tracked files or historical recordings
    And the report identifies the existing tracked recording rather than claiming history is clean
    And no automatic history rewrite is performed

  @givn.added
  Scenario: Recording warnings disclose sensitivity without promising privacy filtering
    Given a project with an installed development reporter
    When the developer reads the effective recording guidance
    Then the guidance warns that recordings can contain sensitive content
    And it describes local gitignored recordings rather than repository-history storage by default
    And it makes no redaction, privacy-boundary, or complete-capture guarantee
