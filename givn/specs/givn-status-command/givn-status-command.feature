Feature: givn-status-command

  Scenario: No changes at all — text mode
    Given  a project root with givn/ initialised
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout mentions "No changes found"

  Scenario: No changes at all — JSON mode
    Given  a project root with givn/ initialised
    When  the user runs `givn status --json`
    Then  the command exits 0
    And  stdout mentions "active"
    And  stdout mentions "archived"

  Scenario: Single active change listing — text mode
    Given  a project root with givn/ initialised
    And  a change "my-feature" exists with artifact "proposal.md"
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout mentions "Active changes:"
    And  stdout mentions "my-feature"
    And  stdout mentions "next:"

  Scenario: Multiple active changes listed alphabetically — text mode
    Given  a project root with givn/ initialised
    And  a change "z-feature" exists with artifact "proposal.md"
    And  a change "a-feature" exists with artifact "proposal.md"
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout line 3 matches "a-feature"
    And  stdout line 4 matches "z-feature"

  Scenario: Active and archived changes — text mode
    Given  a project root with givn/ initialised
    And  a change "current" exists with artifact "proposal.md"
    And  a change "old" exists in archive
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout mentions "Active changes:"
    And  stdout mentions "current"
    And  stdout mentions "Archived: old"

  Scenario: Archived only — text mode (leading newline)
    Given  a project root with givn/ initialised
    And  a change "old" exists in archive
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout does not contain "Active changes:"
    And  stdout mentions "Archived: old"

  Scenario: Active and archived changes — JSON mode
    Given  a project root with givn/ initialised
    And  a change "current" exists with artifact "proposal.md"
    And  a change "old" exists in archive
    When  the user runs `givn status --json`
    Then  the command exits 0
    And  stdout mentions "active"
    And  stdout mentions "archived"

  Scenario: Detail mode — unknown change errors
    Given  a project root with givn/ initialised
    When  the user runs `givn status --change nosuch`
    Then  the command exits non-zero
    And  stderr contains "change 'nosuch' not found"

  Scenario: Detail mode — archived change (text)
    Given  a project root with givn/ initialised
    And  a change "old" exists in archive
    When  the user runs `givn status --change old`
    Then  the command exits 0
    And  stdout mentions "archived"

  Scenario: Detail mode — archived change (JSON)
    Given  a project root with givn/ initialised
    And  a change "old" exists in archive
    When  the user runs `givn status --change old --json`
    Then  the command exits 0
    And  stdout mentions "\"id\": \"old\""

  Scenario: Detail mode — active change with proposal only (text)
    Given  a project root with givn/ initialised
    And  a change "my-feature" exists with artifact "proposal.md"
    When  the user runs `givn status --change my-feature`
    Then  the command exits 0
    And  stdout mentions "my-feature"
    And  stdout mentions "Stage:"
    And  stdout mentions "Artifacts:"
    And  stdout mentions "proposal"
    And  stdout mentions "Next required:"

  Scenario: Detail mode — active change with all required done (text)
    Given  a project root with givn/ initialised
    And  a change "done" exists with all required artifacts complete
    When  the user runs `givn status --change done`
    Then  the command exits 0
    And  stdout mentions "ready-to-archive"
    And  stdout mentions "Ready to archive"
    And  stdout mentions "givn archive --change done"

  Scenario: Detail mode — task progress shown for tasks artifact
    Given  a project root with givn/ initialised
    And  a change "wip" exists with artifact "proposal.md"
    And  a change "wip" exists with artifact "tasks.md" containing "3 done, 5 total"
    When  the user runs `givn status --change wip`
    Then  the command exits 0
    And  stdout mentions "3/5 tasks"

  Scenario: Detail mode — skipped artifacts shown with skip marker
    Given  a project root with givn/ initialised
    And  a change "skip-demo" exists with artifact "proposal.md"
    And  the change "skip-demo" has ".givn-skip" listing "design, design-review"
    When  the user runs `givn status --change skip-demo`
    Then  the command exits 0
    And  stdout mentions "design"
    And  stdout mentions "design-review"

  Scenario: Detail mode — active change (JSON)
    Given  a project root with givn/ initialised
    And  a change "my-feature" exists with artifact "proposal.md"
    When  the user runs `givn status --change my-feature --json`
    Then  the command exits 0
    And  stdout mentions "\"id\": \"my-feature\""
    And  stdout mentions "\"stage\": "
    And  stdout mentions "\"artifacts\": "
    And  stdout mentions "\"verify_command\": "
    And  stdout mentions "\"all_required_done\": false"

  Scenario: Uninitialized project errors
    Given  a project root without a givn/ directory
    When  the user runs `givn status`
    Then  the command exits non-zero
    And  stderr contains "givn/ not found"

  Scenario: Summary line includes stage label and next required step
    Given  a project root with givn/ initialised
    And  a change "my-feature" exists with artifact "proposal.md"
    When  the user runs `givn status`
    Then  the command exits 0
    And  stdout mentions "my-feature"
    And  stdout mentions "next: specs"

  Scenario: arc42-docs appears in the artifact checklist when the addon is enabled
    Given  a project root with givn/ initialised
    And  a change "my-feature" exists with artifact "proposal.md"
    And  the effective manifest has an optional artifact "arc42-docs"
    When  the user runs `givn status --change my-feature`
    Then  the command exits 0
    And  stdout mentions "arc42-docs"
