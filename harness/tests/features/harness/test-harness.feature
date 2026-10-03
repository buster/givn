# User Interaction Inventory:
# - Run the harness with a task description (CLI: `givn-harness run --task <path> --output <dir>`)
# - View run progress on stdout (live output from harness during agent execution)
# - Read a per-run report (markdown file in output directory)
# - Inspect the cross-run index (JSON file tracking all runs)
# - Inspect a workspace from a failed run (preserved temp directory with agent artifacts)

@givn.delta @test-harness

Feature: test-harness

  @givn.added
  Scenario: Harness creates an isolated workspace outside the repository
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the workspace is created in a temporary directory under the configured workspace root
    And the workspace directory is not within the givn repository

  @givn.added
  Scenario: Workspace is initialized with git init and givn init
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the workspace contains an initialized git repository
    And the workspace contains a givn/ directory scaffolded by givn init
    And the workspace contains givn/config.yaml

  @givn.added
  Scenario: verify.command is scaffolded into givn/commands.yaml after initialization
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the workspace's givn/commands.yaml scaffolds verify.command as "./run-tests.sh"

  @givn.added
  Scenario: The task prompt is delivered to the workspace as a file
    Given a task prompt file at "tasks/shopping-list.md" with a task description
    When the user runs `givn-harness run --task tasks/shopping-list.md --output reports/`
    Then the workspace contains a file "task.md" with the task description content

  @givn.added
  Scenario: The task prompt file path that does not exist is rejected
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task nonexistent.md --output reports/`
    Then the command exits non-zero
    And stderr reports that the task file does not exist

  @givn.added
  Scenario: ralph is invoked with --prompt-file pointing to the workspace task file
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/` with the ralph binary at "/usr/bin/true"
    Then ralph is invoked with the --prompt-file argument pointing to the task.md in the workspace

  @givn.added @needs-sandlock
  Scenario: The agent runs inside a sandlock sandbox
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    Then ralph executes inside a sandlock Sandbox instance
    And the sandbox workdir is the workspace directory

  @givn.added @needs-sandlock
  Scenario: Sandbox filesystem uses learn-then-confine mode
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    And the sandlock profile storage directory exists
    When the user runs `givn-harness run --task task.md --output reports/ --profile shopping-list`
    Then on the first run the sandbox learns the workload profile
    And on the second run with the same profile the sandbox enforces the learned filesystem profile

  @givn.added @needs-sandlock
  Scenario: Sandbox network is default-allow with no outbound restrictions
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the sandbox allows unrestricted outbound network access
    And Docker commands succeed inside the sandbox
    And Playwright browser launches succeed inside the sandbox

  @givn.added
  Scenario: Agent execution respects time limit
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/ --timeout 60`
    Then the harness stops agent execution after 60 seconds
    And the report records the timeout as the termination reason

  @givn.added
  Scenario: Agent execution respects maximum iteration limit
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/ --max-iterations 5`
    Then ralph is invoked with --max-iterations 5

  @givn.added
  Scenario: Report includes all twelve automated checks
    Given a completed test run at "reports/run-001/"
    When the harness evaluates the workspace
    Then the run report lists all twelve automated checks:
      | check                                       |
      | artifact existence                          |
      | lint clean                                  |
      | tasks all done                              |
      | valid commits present                       |
      | archive succeeded                           |
      | permanent specs are clean Gherkin            |
      | step definition files are non-empty          |
      | REVIEW: PASS marker present                 |
      | DESIGN-REVIEW: PASS marker present          |
      | verify.command exits zero after archive      |
      | givn addons remain enabled                 |
      | readme substance                            |
    And each check has a pass or fail result

  @givn.added
  Scenario: LLM-as-judge scores each artifact on a 0-4 rubric
    Given a completed test run at "reports/run-001/"
    When the harness evaluates the workspace
    Then an LLM judge scores the proposal artifact on a 0-4 scale
    And an LLM judge scores the specs artifact on a 0-4 scale
    And an LLM judge scores the design artifact on a 0-4 scale
    And an LLM judge scores the tasks artifact on a 0-4 scale
    And an LLM judge scores the review artifact on a 0-4 scale
    And each score is accompanied by a written explanation

  @givn.added
  Scenario: Report is written as a markdown file
    Given a completed test run at "reports/run-001/"
    When the harness evaluates the workspace
    Then a report file "reports/run-001/report.md" exists
    And the report contains a section for automated checks
    And the report contains a section for LLM judge scores
    And the report contains the workspace path

  @givn.added
  Scenario: Cross-run index accumulates results
    Given an existing cross-run index at "reports/index.json" with one prior entry
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the index file at "reports/index.json" contains two entries
    And the new entry includes the run identifier, task name, timestamp, and overall pass/fail

  @givn.added
  Scenario: Index gracefully handles corrupt or non-list JSON
    Given a corrupt JSON file at "reports/index.json" containing "not json at all"
    When the harness writes a run entry for task "test.md" to "reports/"
    Then the index contains exactly one entry
    And a non-list JSON root at "reports/index.json" is reset to an empty array
    And a new entry is appended to the reset index

  @givn.added
  Scenario: Workspace is preserved when a run fails
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    And the agent fails to archive the change
    Then the workspace directory is not deleted
    And the report includes the path to the preserved workspace

  @givn.added @e2e
  Scenario: User runs the harness end-to-end and observes a complete autoresearch loop
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    And a task file "tasks/shopping-list.md" describing a shopping list website
    When the user runs `givn-harness run --task tasks/shopping-list.md --output reports/` end-to-end
    Then the harness creates an isolated workspace
    And initializes the workspace with git and givn
    And invokes ralph to drive the agent through the full givn workflow
    And produces a run report at "reports/<run-id>/report.md"
    And updates the cross-run index at "reports/index.json"
    And stdout shows the workspace creation message
    And stdout shows "git init" and "givn init" progress
    And stdout shows ralph iteration progress
    And stdout shows evaluation progress for each automated check
    And stdout shows LLM judge scoring progress

  @givn.added @e2e
  Scenario: User reads a per-run report after a completed run
    Given a completed test run at "reports/run-001/"
    When the user reads the file "reports/run-001/report.md"
    Then the report contains an "Automated Checks" section with pass/fail for each of the ten checks
    And the report contains an "LLM Judge Scores" section with 0-4 scores and explanations for each artifact
    And the report contains a "Workspace" section with the path to the workspace directory
    And the report contains an "Agent Log" section with ralph output

  @givn.added @e2e
  Scenario: User inspects the cross-run index after multiple runs
    Given a cross-run index at "reports/index.json" with entries from two prior runs
    When the user reads the file "reports/index.json"
    Then the index is valid JSON
    And the index is an array with at least two entries
    And each entry has a "run_id" field
    And each entry has a "task" field
    And each entry has a "timestamp" field
    And each entry has an "overall_pass" field

  @givn.added @e2e
  Scenario: User inspects a workspace from a failed run
    Given a failed test run whose workspace was preserved at "/tmp/givn-harness-workspaces/run-002"
    When the user lists the contents of the preserved workspace
    Then the workspace contains a task.md file
    And the workspace contains a givn/ directory with the agent's partial progress
    And the workspace contains a .git directory with the agent's commits

  @givn.added
  Scenario: Report handles a run where the agent produced no artifacts
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    And the agent crashes before producing any artifacts
    Then all automated checks report FAIL with detail
    And the LLM judge scores default to 0
    And the termination reason is recorded in the report

  @givn.added @needs-sandlock
  Scenario: Sandlock is not available, harness falls back to --no-sandbox
    Given the sandlock SDK is not importable
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the harness logs a warning that sandlock is unavailable
    And the harness runs the agent without sandbox confinement
    And the report notes that the run was unsandboxed

  @givn.added
  Scenario: Ralph binary is not found at the configured path
    Given the ralph binary is not present at "/usr/local/bin/ralph"
    When the user runs `givn-harness run --task task.md --output reports/`
    Then the harness exits with a non-zero exit code
    And stderr reports that the ralph binary was not found

  @givn.added
  Scenario: LLM judge returns unparseable JSON, score defaults to zero
    Given a completed test run at "reports/run-001/"
    And the LLM judge returns invalid JSON for the proposal artifact
    When the harness evaluates the workspace
    Then the proposal artifact judge score is 0
    And the explanation is "judge output unparseable"

  @givn.added
  Scenario: Interrupted run preserves workspace on SIGINT
    Given a workspace directory at "/tmp/givn-harness-workspaces"
    When the user runs `givn-harness run --task task.md --output reports/`
    And the process receives SIGINT
    Then the workspace directory is not deleted
    And the report records the interruption as the termination reason
