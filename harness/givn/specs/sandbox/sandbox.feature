Feature: sandbox

  Scenario: The agent runs inside a bubblewrap sandbox
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  ralph executes inside a bubblewrap sandbox
    And  the sandbox workdir is the workspace directory

  Scenario: Bubblewrap is not available, harness falls back to unsandboxed
    Given  the bubblewrap binary is not present in PATH
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the harness logs a warning that bubblewrap is unavailable
    And  the harness runs the agent without sandbox confinement
    And  the report notes that the run was unsandboxed

  Scenario: Bubblewrap sandbox restricts filesystem write access to configured writable paths
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the agent can write files inside the workspace directory
    And  the agent can write files to "/tmp"
    And  the agent cannot write files to "/home" or "/var" or "/opt"

  Scenario: Bubblewrap sandbox restricts filesystem read access to configured readable paths
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the agent can read files from "/usr", "/bin", "/lib", "/lib64", and "/etc"
    And  the agent cannot read files outside the configured readable paths

  Scenario: Bubblewrap sandbox enforces process count limit
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    And  the agent attempts to fork more than 256 processes
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the agent terminates with a resource limit error

  Scenario: Bubblewrap sandbox enforces memory limit
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    And  the agent attempts to allocate more than 16 GB of memory
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the agent terminates with a resource limit error

  Scenario: Agent writes to workspace persist after sandbox exits
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    And  the task instructs the agent to create a file "hello.txt" in the workspace
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the file "hello.txt" exists in the original workspace directory after the sandbox exits
    And  the file contains the agent's output

  Scenario: Bubblewrap sandbox makes givn/config.yaml read-only
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the agent cannot modify givn/config.yaml in the workspace
    And  the agent can still write other files in the workspace

  Scenario: Agent stdout and logs are captured from the sandbox
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    When  the user runs `givn-harness run --task task.md --output reports/`
    Then  the agent's stdout is captured and written to the workspace agent log
    And  the report contains the agent log output

  @e2e
  Scenario: E2E smoke test for bubblewrap sandbox with real agent
    Given  a workspace directory at "/tmp/givn-harness-workspaces"
    And  a task file "tasks/trivial.md" describing a trivial task using the givn workflow
    When  the user runs `givn-harness run --task tasks/trivial.md --output reports/`
    Then  the agent successfully completes the task
    And  the run report shows the sandboxed flag as "bwrap"
    And  the agent's workspace changes are visible to the harness evaluator
