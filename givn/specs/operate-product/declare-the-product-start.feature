Feature: Declare the product start

  @e2e
  Scenario: Scaffolding ships an unconfigured run declaration
    Given a project root without a givn/ directory
    When the user runs `givn init`
    Then the file "givn/commands.yaml" exists
    And the commands file has run.command set to the sentinel
    And the README has the run managed block
    And the README has the run open placeholder

  @e2e
  Scenario: The review check reports an unconfigured run declaration
    Given a project root with givn/ initialised
    And verify commands are no-ops
    And a reviewable change "run-demo"
    And its review records the impact "none"
    When the user runs `givn check review --change run-demo`
    Then the command exits non-zero
    And stdout mentions "run.command is unset"
    And stdout mentions "run.product_type is unset"

  @e2e
  Scenario: Archive passes once the run declaration is complete
    Given a project root with givn/ initialised
    And a complete README
    And verify commands are no-ops with a complete server run declaration
    And givn addon "coverage" is "false"
    And a completed change "run-demo"
    And the semantic gate is offline
    When the user runs `givn archive --change run-demo`
    Then the command exits 0
    And the archived change directory "run-demo" exists

  @e2e
  Scenario: Upgrade backfills the run declaration into an existing project
    Given a git-managed project with givn initialised
    And the commands file has no run declaration
    When the user runs `givn upgrade`
    Then the command exits 0
    And the file "givn/commands.yaml" contains "run:"
    And stdout mentions "fill it in"

  @e2e
  Scenario: The check names a damaged run managed block
    Given a project root with givn/ initialised
    And a README with a broken run managed block
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "run managed block"

  @e2e
  Scenario: The sentinel start command prints setup guidance
    Given a project root with givn/ initialised
    When the user runs `givn missing-run-command`
    Then the command exits non-zero
    And stderr mentions "run.command is unset"

  Scenario: Archive is blocked while the run declaration is unconfigured
    Given a project root with givn/ initialised
    And a complete README
    And verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "run-demo"
    And the semantic gate is offline
    When the user runs `givn archive --change run-demo`
    Then the command exits non-zero
    And stdout mentions "run declaration"
    And the directory givn/changes/run-demo/ exists

  Scenario: An unconfigured declaration reports only unset findings
    Given a run declaration:
      """
      run:
        command: "givn missing-run-command"
      """
    When the run declaration is validated
    Then the check reports "run.command is unset"
    And the check reports "run.product_type is unset"
    And the check reports no port or readiness findings

  Scenario: A server without a readiness path is incomplete
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port: 8080
      """
    When the run declaration is validated
    Then the check reports "requires run.ready_path"

  Scenario: A server needs exactly one port source
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        ready_path: "/healthz"
      """
    When the run declaration is validated
    Then the check reports "exactly one of run.port or run.port_env"

  Scenario: A server rejects two port sources
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port: 8080
        port_env: "PORT"
        ready_path: "/healthz"
      """
    When the run declaration is validated
    Then the check reports "mutually exclusive"

  Scenario: Server-only fields are rejected for a CLI product
    Given a run declaration:
      """
      run:
        command: "my-tool --serve"
        product_type: cli
        port: 8080
      """
    When the run declaration is validated
    Then the check reports "applies to product_type server only"

  Scenario: A zero readiness timeout is rejected
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port_env: "PORT"
        ready_path: "/healthz"
        ready_timeout_s: 0
      """
    When the run declaration is validated
    Then the check reports "must be greater than zero"

  Scenario: A relative readiness path is rejected
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port_env: "PORT"
        ready_path: "healthz"
      """
    When the run declaration is validated
    Then the check reports "must be an absolute path"

  Scenario: A no-op start command is rejected
    Given a run declaration:
      """
      run:
        command: "true"
        product_type: cli
      """
    When the run declaration is validated
    Then the check reports "does nothing"

  Scenario: A library may keep the placeholder command
    Given a run declaration:
      """
      run:
        command: "givn missing-run-command"
        product_type: library
      """
    When the run declaration is validated
    Then the check reports no findings

  Scenario: A complete server declaration validates cleanly
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port_env: "PORT"
        ready_path: "/healthz"
        ready_timeout_s: 60
      """
    When the run declaration is validated
    Then the check reports no findings

  Scenario: A complete CLI declaration validates cleanly
    Given a run declaration:
      """
      run:
        command: "my-tool --help"
        product_type: cli
      """
    When the run declaration is validated
    Then the check reports no findings

  Scenario: An unknown product type is rejected when the commands file loads
    Given a commands file with a run declaration:
      """
      run:
        command: "my-tool"
        product_type: web
      """
    When the commands file is loaded
    Then loading fails mentioning "product_type"

  Scenario: An unconfigured declaration renders exactly one open placeholder
    Given a run declaration:
      """
      run:
        command: "givn missing-run-command"
      """
    When the run managed block is rendered
    Then the rendered block has the run open placeholder
    And the rendered block has no start command

  Scenario: The README renders a complete server declaration
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port_env: "PORT"
        ready_path: "/healthz"
        ready_timeout_s: 60
      """
    When the run managed block is rendered
    Then the rendered block mentions the start command "./run.sh"
    And the rendered block mentions the readiness path "/healthz"
    And the rendered block mentions the port hand-off "PORT"
    And the rendered block has no open placeholder

  Scenario: The README renders a CLI product
    Given a run declaration:
      """
      run:
        command: "my-tool --help"
        product_type: cli
      """
    When the run managed block is rendered
    Then the rendered block mentions the start command "my-tool --help"
    And the rendered block has no open placeholder

  Scenario: The README renders a library product as nothing to start
    Given a run declaration:
      """
      run:
        command: "givn missing-run-command"
        product_type: library
      """
    When the run managed block is rendered
    Then the rendered block says there is nothing to start
    And the rendered block has no open placeholder
    And the rendered block has no start command

  Scenario: The README renders a library's declared start command
    Given a run declaration:
      """
      run:
        command: "cargo build --lib"
        product_type: library
      """
    When the run managed block is rendered
    Then the rendered block says there is nothing to start
    And the rendered block mentions the start command "cargo build --lib"
    And the rendered block has no open placeholder

  Scenario: A half-configured server declaration renders the placeholder
    Given a run declaration:
      """
      run:
        command: "./run.sh"
        product_type: server
        port: 8080
      """
    When the run managed block is rendered
    Then the rendered block has the run open placeholder
    And the rendered block has no start command

  Scenario: The backfill appends the run scaffold to an existing commands file
    Given a commands file without a run declaration:
      """
      verify:
        command: "./run-tests.sh"
      """
    When the backfill runs
    Then the backfilled commands file contains "run:"
    And the backfilled commands file keeps the original content

  Scenario: The backfill is idempotent
    Given a commands file that already has a run declaration
    When the backfill runs
    Then the commands file is unchanged

  Scenario: The upgrade commit includes the backfilled run declaration
    Given  a git-managed project with givn initialised
    And  the commands file has no run declaration
    When  the user runs `givn upgrade`
    Then  the command exits 0
    And  the working tree is clean
    And  the latest revision contains "givn/commands.yaml"

  Scenario: A repeated upgrade creates no additional commit
    Given  a git-managed project with givn initialised
    And  the commands file has no run declaration
    When  the user runs `givn upgrade`
    And  the user runs `givn upgrade` again
    Then  the command exits 0
    And  the latest revision is unchanged by the second upgrade
