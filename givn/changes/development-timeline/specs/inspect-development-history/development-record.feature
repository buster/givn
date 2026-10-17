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

  @givn.added
  Scenario: Two writers never mint the same entry id
    Given a project root with givn/ initialised
    And two givn commands record at the same time
    When both commands finish
    Then every entry in the development record has a distinct id
    And every parent link names an entry that exists in the record

  @givn.added
  Scenario: Activity during an ideation belongs to its topic scope
    Given a project root with givn/ initialised
    And an ideation topic "timeline-ideas" whose session is unfinished
    When the user runs `givn status`
    Then the recorded entry names the ideation topic as its scope

  @givn.added
  Scenario: A recorded label is bounded to 120 characters
    Given a project root with givn/ initialised
    And a change "demo" with a command whose full text is longer than 120 characters
    When the user runs that command
    Then the recorded entry's label is at most 120 characters and marks the truncation
    And the record holds no complete command text

  @givn.added
  Scenario: Reading the development record does not extend it
    Given a project root with givn/ initialised
    And a change "demo" with a recorded history
    When the user runs `givn history --change demo --json`
    And the user runs `givn timeline --change demo --out report.html`
    Then the development record holds no new entry

  @givn.added
  Scenario: The record stays out of the repository's own history
    Given a project root with givn/ initialised without a repository
    And a change "demo" exists with artifact "proposal.md"
    When the project records a full-content entry in the full mode
    Then the project's ".gitignore" names the development record
    When the project becomes a Git repository
    Then the repository reports no pending change for the development record
    When the project records a full-content entry in the full mode
    Then the repository reports no pending change for the development record
    When the user runs `givn timeline --change demo`
    Then the repository reports no pending change under the record directory

  @givn.added
  Scenario: A Fossil project ignores the record through its own ignore file
    Given a project root with givn/ initialised in a Fossil checkout
    And a change "demo" exists with artifact "proposal.md"
    When the project records a full-content entry in the full mode
    Then the repository's ignore glob names the development record
    And the Fossil checkout reports no extra file under the record directory
    When a lifecycle command runs
    Then the repository's ignore glob still names the development record
    When the user runs `givn timeline --change demo`
    Then the repository's ignore glob covers the generated reports
    And the Fossil checkout reports no extra file under the record directory

  @givn.added
  Scenario: A refused command outside a Givn project creates no record
    Given a project root without a givn/ directory
    When the consumer runs `givn init`
    Then the command exits non-zero
    And the directory "givn" does not exist
    And the project root holds no development record
    When the consumer runs `givn init --no-addons --agents opencode`
    Then the command exits 0
    And the directory "givn" exists
    And the project root holds a development record

  @givn.added
  Scenario: Default capture keeps the metadata a review needs
    Given a project root with givn/ initialised
    And a change "demo" exists with artifact "proposal.md"
    And the project records in the default mode
    When the omp reporter reports a completed turn and a failed tool call for "demo"
    Then the turn entry names its model, its cost, its context size, and its stop reason
    And the failed tool entry keeps its error text within the documented bound, its tool call id, and its output size

  @givn.added
  Scenario: Question text is captured in full
    Given a change "demo" whose Maki reporter reads a question from its session transcript
    When the reported question is longer than the entry label bound
    Then the record holds the complete question text up to the 4 000-character cap
    And the entry marks the text truncated with its original size
    And the entry's label stays within the label bound
    When the Maki session transcript holds no matching question
    Then the record keeps the bounded title label and no full question text

  @givn.added
  Scenario: The record can be turned off for a project
    Given a change "demo" whose record already holds entries
    When the project sets the record mode to "off"
    And a lifecycle command runs for "demo"
    Then the record gains no new entry
    And the record directory gains no pending state
    And no record-side ignore entry is written
    And the developer still reads the earlier entries for "demo"
    When the agent reports a tool call with its complete input through the intake
    Then the intake exits 0 and warns once that the record is off
    And the record gains no new entry

  @givn.added
  Scenario: The default mode records metadata without content
    Given a change "demo" whose project has no record mode configured
    When the agent reports a tool call with its complete input and output
    Then the record drops the complete content
    And no entry carries a data.content object
    And the record keeps the tool call's metadata

  @givn.added
  Scenario: The full mode records complete content
    Given a change "demo" whose project sets the record mode to "full"
    When the agent reports a tool call whose complete input exceeds the per-field cap
    Then the record keeps the input up to the 64 KiB per-field cap
    And the content field marks truncation and records its original size
    And the entry's content never exceeds the 256 KiB per-entry cap

  @givn.added
  Scenario: An invalid record mode falls back to the default
    Given a change "demo" whose project sets an unknown record mode
    When a lifecycle command runs for "demo"
    Then the command warns once that the record falls back to the default mode
    And the record holds the command's bounded metadata
    And `givn history settings --json` reports mode "on" with source "default"

  @givn.added @wip
  Scenario: An artifact appearance is recorded as one phase transition
    Given a change "demo" whose proposal.md appears while a recorded command runs
    When the command finishes
    Then the record holds one phase entry labelled "proposal.md" for "demo"
    And a later recorded command that sees the same artifact records no second transition
    And a gate check for the same artifact keeps its artifact id label and is not a transition

  @givn.added @e2e
  Scenario: Consumer runs a lifecycle step and reads its recorded actions
    Given a project root with givn/ initialised
    And a change "demo" exists with artifact "proposal.md"
    When the user runs `givn status --change demo`
    And the user runs `givn history --change demo --json`
    Then the command exits 0
    And the machine-readable record names the status command for "demo"
    And the machine-readable record carries a duration for that command

  @givn.added @e2e
  Scenario: Consumer reads the development record as machine-readable output
    Given a project root with givn/ initialised
    And a change "demo" with a recorded history
    When the user runs `givn history --change demo --json`
    Then the command exits 0
    And the output names every recorded entry with its kind, its scope, and its duration
