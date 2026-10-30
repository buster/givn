Feature: report-change-status

  Scenario: No changes at all — text mode
    Given a project root with givn/ initialised
    When the user runs `givn status`
    Then the command exits 0
    And stdout mentions "No changes found"

  Scenario: No changes at all — JSON mode
    Given a project root with givn/ initialised
    When the user runs `givn status --json`
    Then the command exits 0
    And stdout mentions "active"
    And stdout mentions "archived"

  Scenario: Single active change — auto-defaults to detail view (text)
    Given a project root with givn/ initialised
    And a change "my-feature" exists with artifact "proposal.md"
    When the user runs `givn status`
    Then the command exits 0
    And stdout mentions "my-feature"
    And stdout mentions "Stage:"
    And stdout mentions "Artifacts:"

  Scenario: Multiple active changes listed alphabetically — text mode
    Given  a project root with givn/ initialised
    And  a change "z-feature" exists with artifact "proposal.md"
    And  a change "a-feature" exists with artifact "proposal.md"
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout line 4 matches "a-feature"
    And  stdout line 5 matches "z-feature"

  Scenario: Active and archived changes — single active auto-defaults to detail (text)
    Given a project root with givn/ initialised
    And a change "current" exists with artifact "proposal.md"
    And a change "old" exists in archive
    When the user runs `givn status`
    Then the command exits 0
    And stdout mentions "Change:"
    And stdout mentions "current"
    And stdout does not contain "Archived:"

  Scenario: Archived only — text mode (leading newline)
    Given a project root with givn/ initialised
    And a change "old" exists in archive
    When the user runs `givn status`
    Then the command exits 0
    And stdout does not contain "Active changes:"
    And stdout mentions "Archived: old"

  Scenario: Active and archived changes — single active auto-defaults to detail JSON
    Given a project root with givn/ initialised
    And a change "current" exists with artifact "proposal.md"
    And a change "old" exists in archive
    When the user runs `givn status --json`
    Then the command exits 0
    And stdout mentions "\"id\": \"current\""
    And stdout mentions "\"stage\": "

  Scenario: Detail mode — unknown change errors
    Given a project root with givn/ initialised
    When the user runs `givn status --change nosuch`
    Then the command exits non-zero
    And stderr contains "change 'nosuch' not found"

  @report-change-status
  Scenario: Detail mode — archived change (text)
    Given  a project root with givn/ initialised
    And  a change "old" has a proven archive with a matching archive receipt
    When  the user runs `givn status --change old`
    Then  the command exits 0
    And  stdout reports a proven archive

  @report-change-status
  Scenario: Detail mode — archived change (JSON)
    Given  a project root with givn/ initialised
    And  a change "old" has a proven archive with a matching archive receipt
    When  the user runs `givn status --change old --json`
    Then  the command exits 0
    And  stdout mentions "\"id\": \"old\""
    And  the JSON status reports a proven archive

  Scenario: Detail mode — active change with proposal only (text)
    Given a project root with givn/ initialised
    And a change "my-feature" exists with artifact "proposal.md"
    When the user runs `givn status --change my-feature`
    Then the command exits 0
    And stdout mentions "my-feature"
    And stdout mentions "Stage:"
    And stdout mentions "Artifacts:"
    And stdout mentions "proposal"
    And stdout mentions "Next required:"

  @e2e @report-change-status
  Scenario: Detail mode — active change with all required done (text)
    Given  a project root with givn/ initialised
    And  a change "done" exists with all required artifacts complete
    And  the change has no verification receipt
    When  the user invokes `givn status --change done`
    Then  the command exits 0
    And  stdout reports that required artifacts are complete
    And  stdout reports verification as not checked
    And  stdout does not claim "ready-to-archive" or "Ready to archive"
    And  the terminal transcript is captured for this scenario

  Scenario: Detail mode — task progress shown for tasks artifact
    Given a project root with givn/ initialised
    And a change "wip" exists with artifact "proposal.md"
    And a change "wip" exists with artifact "tasks.md" containing "3 done, 5 total"
    When the user runs `givn status --change wip`
    Then the command exits 0
    And stdout mentions "3/5 tasks"

  Scenario: Detail mode — skipped artifacts shown with skip marker
    Given a project root with givn/ initialised
    And a change "skip-demo" exists with artifact "proposal.md"
    And the change "skip-demo" has ".givn-skip" listing "design, design-review"
    When the user runs `givn status --change skip-demo`
    Then the command exits 0
    And stdout mentions "design"
    And stdout mentions "design-review"

  Scenario: Detail mode — active change (JSON)
    Given a project root with givn/ initialised
    And a change "my-feature" exists with artifact "proposal.md"
    When the user runs `givn status --change my-feature --json`
    Then the command exits 0
    And stdout mentions "\"id\": \"my-feature\""
    And stdout mentions "\"stage\": "
    And stdout mentions "\"artifacts\": "
    And stdout mentions "\"verify_command\": "
    And stdout mentions "\"all_required_done\": false"

  Scenario: Uninitialized project errors
    Given a project root without a givn/ directory
    When the user runs `givn status`
    Then the command exits non-zero
    And stderr contains "givn/ not found"

  Scenario: Detail view includes stage label and next required step
    Given a project root with givn/ initialised
    And a change "my-feature" exists with artifact "proposal.md"
    When the user runs `givn status --change my-feature`
    Then the command exits 0
    And stdout mentions "my-feature"
    And stdout mentions "Next required:"

  Scenario: arc42-docs appears in the artifact checklist when the addon is enabled
    Given a project root with givn/ initialised
    And a change "my-feature" exists with artifact "proposal.md"
    And the effective manifest has an optional artifact "arc42-docs"
    When the user runs `givn status --change my-feature`
    Then the command exits 0
    And stdout mentions "arc42-docs"

  Scenario Outline: Status names the active SCM backend
    Given  a <backend> project
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout reports the active backend as <scm>
    Examples:
      | backend | scm |
      | Git | git |
      | Fossil | fossil |

  Scenario: Status without a backend still succeeds and names the unavailable commands
    Given  a project root without a Git worktree or a Fossil checkout
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout reports the active backend as none
    And  stdout names archive and managed upgrade as unavailable

  @report-change-status
  Scenario Outline: Status reports receipt freshness separately from artifact completion
    Given  a change "done" exists with all required artifacts complete
    And  <receipt condition>
    When  the user runs <command>
    Then  the command exits 0
    And  the status reports required artifacts as complete
    And  the status reports verification as <verification state>
    And  the status does not claim that artifact completion establishes archive readiness
    Examples:
      | receipt condition | command | verification state |
      | a green receipt matches the current relevant inputs | `givn status --change done` | current |
      | tested executable inputs changed after a green receipt | `givn status --change done` | stale |
      | no verification receipt exists | `givn status --change done` | not checked |
      | a green receipt matches the current relevant inputs | `givn status --change done --json` | current |
      | tested executable inputs changed after a green receipt | `givn status --change done --json` | stale |
      | no verification receipt exists | `givn status --change done --json` | not checked |

  @report-change-status
  Scenario Outline: An archive candidate without current proof is not a proven archive
    Given  a change "candidate" exists in the archive directory
    And  <proof condition>
    When  the user runs <command>
    Then  the command exits 0
    And  the status reports a pending archive candidate
    And  the status does not report a proven archive
    Examples:
      | proof condition | command |
      | the archive receipt is missing | `givn status --change candidate` |
      | the archive receipt is stale | `givn status --change candidate` |
      | the archive receipt is missing | `givn status --change candidate --json` |
      | the archive receipt is stale | `givn status --change candidate --json` |

  @report-change-status
  Scenario Outline: Inspecting status never executes verification
    Given  a project with complete active artifacts for change "done" and a pending archive candidate "candidate"
    And  verification commands record every invocation and fail if invoked
    And  the change artifacts and receipts are snapshotted
    When  the user runs <command>
    Then  the command exits 0
    And  no verification command was invoked
    And  the change artifacts and receipts are unchanged
    Examples:
      | command |
      | `givn status` |
      | `givn status --json` |
      | `givn status --change done` |
      | `givn status --change done --json` |
      | `givn status --change candidate` |
      | `givn status --change candidate --json` |
