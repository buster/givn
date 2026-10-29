@givn.delta @report-change-status
Feature: Report change status

  @givn.modified @e2e @wip
  Scenario: Detail mode — active change with all required done (text)
    Given a project root with givn/ initialised
    And a change "done" exists with all required artifacts complete
    And the change has no verification receipt
    When the user invokes `givn status --change done`
    Then the command exits 0
    And stdout reports that required artifacts are complete
    And stdout reports verification as not checked
    And stdout does not claim "ready-to-archive" or "Ready to archive"
    And the terminal transcript is captured for this scenario

  @givn.added @wip
  Scenario Outline: Status reports receipt freshness separately from artifact completion
    Given a change "done" exists with all required artifacts complete
    And <receipt condition>
    When the user runs <command>
    Then the command exits 0
    And the status reports required artifacts as complete
    And the status reports verification as <verification state>
    And the status does not claim that artifact completion establishes archive readiness

    Examples:
      | receipt condition                                      | command                                  | verification state |
      | a green receipt matches the current relevant inputs     | `givn status --change done`               | current            |
      | tested executable inputs changed after a green receipt  | `givn status --change done`               | stale              |
      | no verification receipt exists                          | `givn status --change done`               | not checked        |
      | a green receipt matches the current relevant inputs     | `givn status --change done --json`        | current            |
      | tested executable inputs changed after a green receipt  | `givn status --change done --json`        | stale              |
      | no verification receipt exists                          | `givn status --change done --json`        | not checked        |

  @givn.modified @wip
  Scenario: Detail mode — archived change (text)
    Given a project root with givn/ initialised
    And a change "old" has a proven archive with a matching archive receipt
    When the user runs `givn status --change old`
    Then the command exits 0
    And stdout reports a proven archive

  @givn.modified @wip
  Scenario: Detail mode — archived change (JSON)
    Given a project root with givn/ initialised
    And a change "old" has a proven archive with a matching archive receipt
    When the user runs `givn status --change old --json`
    Then the command exits 0
    And stdout mentions "\"id\": \"old\""
    And the JSON status reports a proven archive

  @givn.added @wip
  Scenario Outline: An archive candidate without current proof is not a proven archive
    Given a change "candidate" exists in the archive directory
    And <proof condition>
    When the user runs <command>
    Then the command exits 0
    And the status reports a pending archive candidate
    And the status does not report a proven archive

    Examples:
      | proof condition                              | command                                  |
      | the archive receipt is missing               | `givn status --change candidate`          |
      | the archive receipt is stale                 | `givn status --change candidate`          |
      | the archive receipt is missing               | `givn status --change candidate --json`   |
      | the archive receipt is stale                 | `givn status --change candidate --json`   |

  @givn.added @wip
  Scenario Outline: Inspecting status never executes verification
    Given a project with complete active artifacts for change "done" and a pending archive candidate "candidate"
    And verification commands record every invocation and fail if invoked
    And the change artifacts and receipts are snapshotted
    When the user runs <command>
    Then the command exits 0
    And no verification command was invoked
    And the change artifacts and receipts are unchanged

    Examples:
      | command                                  |
      | `givn status`                            |
      | `givn status --json`                     |
      | `givn status --change done`              |
      | `givn status --change done --json`       |
      | `givn status --change candidate`         |
      | `givn status --change candidate --json`  |
