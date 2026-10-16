@givn.delta @development-record

Feature: Development Record

  @givn.added
  Scenario: A recorded step keeps a measured duration and its outcome
    Given a project root with givn/ initialised
    And a change "demo" exists with artifact "proposal.md"
    When the user runs `givn status --change demo`
    Then the development record holds one entry for that command
    And that entry carries a measured duration and its outcome

  @givn.added
  Scenario: A verification run is recorded as its own step
    Given a project root with givn/ initialised
    And a change "demo" with a verification command that takes measurable time
    When the user runs `givn check tasks --change demo`
    Then the development record holds one entry for the verification run
    And that entry carries a duration of at least one second

  @givn.added
  Scenario: A record that cannot be written never fails the command it records
    Given a project root with givn/ initialised
    And the development record cannot be written
    When the user runs `givn status`
    Then the command exits 0
    And stderr warns that the development record could not be written

  @givn.added
  Scenario: The record is append-only across commands
    Given a project root with givn/ initialised
    And the development record holds an entry from an earlier command
    When the user runs `givn status`
    Then the earlier entry is still present unchanged
    And the new entry is appended after it without rewriting the store

  @givn.added
  Scenario: Activity outside a change belongs to the project scope
    Given a project root with givn/ initialised
    And no change and no ideation is active for this project
    When the user runs `givn status`
    Then the recorded entry names the project as its scope

  @givn.added @wip
  Scenario: Two writers never mint the same entry id
    Given a project root with givn/ initialised
    And two givn commands record at the same time
    When both commands finish
    Then every entry in the development record has a distinct id
    And every parent link names an entry that exists in the record

  @givn.added @wip
  Scenario: Activity during an ideation belongs to its topic scope
    Given a project root with givn/ initialised
    And an ideation topic "timeline-ideas" whose session is unfinished
    When the user runs `givn status`
    Then the recorded entry names the ideation topic as its scope

  @givn.added @wip
  Scenario: A recorded label is bounded to 120 characters
    Given a project root with givn/ initialised
    And a change "demo" with a command whose full text is longer than 120 characters
    When the user runs that command
    Then the recorded entry's label is at most 120 characters and marks the truncation
    And the record holds no complete command text

  @givn.added @wip
  Scenario: Reading the development record does not extend it
    Given a project root with givn/ initialised
    And a change "demo" with a recorded history
    When the user runs `givn history --change demo --json`
    And the user runs `givn timeline --change demo --out report.html`
    Then the development record holds no new entry

  @givn.added @wip
  Scenario: The record stays out of the repository's own history
    Given a project root with givn/ initialised under Git
    And a change "demo" exists with artifact "proposal.md"
    When the user runs `givn status --change demo`
    Then the repository reports no pending change for the development record

  @givn.added @wip
  Scenario: A Fossil project ignores the record through its own ignore file
    Given a project root with givn/ initialised in a Fossil checkout
    And a change "demo" exists with artifact "proposal.md"
    When the user runs `givn status --change demo`
    Then the repository's ignore glob names the development record

  @givn.added @e2e @wip
  Scenario: Consumer runs a lifecycle step and reads its recorded actions
    Given a project root with givn/ initialised
    And a change "demo" exists with artifact "proposal.md"
    When the user runs `givn status --change demo`
    And the user runs `givn history --change demo --json`
    Then the command exits 0
    And the machine-readable record names the status command for "demo"
    And the machine-readable record carries a duration for that command

  @givn.added @e2e @wip
  Scenario: Consumer reads the development record as machine-readable output
    Given a project root with givn/ initialised
    And a change "demo" with a recorded history
    When the user runs `givn history --change demo --json`
    Then the command exits 0
    And the output names every recorded entry with its kind, its scope, and its duration
