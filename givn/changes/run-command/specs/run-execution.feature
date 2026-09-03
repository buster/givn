# User Interaction Inventory:
# - start the product with `givn run`
# - start the product with `givn run --port <n>`
# - print the start plan with `givn run --print`
# - interrupt the started product with SIGINT (SIGTERM is the in-process variant)

@givn.delta @run-execution

Feature: Run execution

  @givn.added @e2e
  Scenario: The started server reports readiness and forwards a clean exit
    Given a project with a server run declaration that becomes ready and exits cleanly
    When the user runs `givn run`
    Then the command exits 0
    And stdout contains a line beginning with "givn run: ready at http://127.0.0.1:"
    And the ready line names the readiness path "/healthz"

  @givn.added @e2e
  Scenario: A caller-provided port is handed to the product
    Given a project with a server run declaration that becomes ready and exits cleanly
    When the user runs givn run with a caller-provided port
    Then the command exits 0
    And the ready line names the caller-provided port

  @givn.added @e2e
  Scenario: Print mode emits the start plan without starting
    Given a project with a server run declaration with a port hand-off
    When the user runs `givn run --print`
    Then the command exits 0
    And stdout is one line that exports the port hand-off and names the start command
    And stdout does not contain "givn run: ready"

  @givn.added @e2e
  Scenario: An interrupt stops the started product
    Given a project with a server run declaration that stays ready
    When the user interrupts givn run after the ready line
    Then the command exits 130
    And the started product received the interrupt

  @givn.added
  Scenario: A library has nothing to start
    Given a project with a library run declaration
    When the user runs `givn run`
    Then the command exits 0
    And stdout mentions "nothing to start"

  @givn.added
  Scenario: An unconfigured declaration refuses to start
    Given a project with an unconfigured run declaration
    When the user runs `givn run`
    Then the command exits 1
    And stderr mentions "run.command is unset"

  @givn.added
  Scenario: A caller port without a port hand-off is refused
    Given a project with a server run declaration with a fixed port
    When the user runs givn run with a caller-provided port
    Then the command exits 1
    And stderr mentions "port_env"

  @givn.added
  Scenario: A readiness timeout abandons the start
    Given a project with a server run declaration whose readiness path never succeeds
    When the user runs `givn run`
    Then the command exits 2
    And stdout does not contain "givn run: ready"

  @givn.added
  Scenario: An early product exit is forwarded before readiness
    Given a project with a server run declaration whose product exits 3 before readiness
    When the user runs `givn run`
    Then the command exits 3
    And stdout does not contain "givn run: ready"

  @givn.added
  Scenario: A CLI product forwards the product's exit code
    Given a project with a CLI run declaration that exits 7
    When the user runs `givn run`
    Then the command exits 7

  @givn.added
  Scenario: A fixed declared port is used without a port hand-off
    Given a project with a server run declaration with a fixed port
    When the user runs `givn run`
    Then the command exits 0
    And the ready line names the fixed declared port

  @givn.added
  Scenario: A port that is not listening yet is retried until ready
    Given a readiness probe against a port with no listener yet
    When the listener starts answering success after a short delay
    Then the probe reports ready after more than one attempt

  @givn.added
  Scenario: A failing readiness response is retried until ready
    Given a readiness probe against a listener that answers failure first
    When the listener switches to answering success
    Then the probe reports ready after more than one attempt

  @givn.added
  Scenario: Any success response satisfies the readiness probe
    Given a readiness probe against a listener that answers 204
    Then the probe reports ready on the first attempt

  @givn.added
  Scenario: The readiness probe gives up after the timeout
    Given a readiness probe against a listener that never answers success
    When the readiness timeout elapses
    Then the probe reports a timeout

  @givn.added
  Scenario: A terminate request is forwarded and the start waits
    Given givn run is executing a product that stays ready
    When givn receives a terminate request while running the product
    Then givn exits 143
    And the started product received the terminate signal

  @givn.added
  Scenario: Print mode omits the port hand-off for a fixed-port declaration
    Given a project with a server run declaration with a fixed port
    When the user runs `givn run --print`
    Then the command exits 0
    And stdout is one line that names the start command without exporting a port

  @givn.added
  Scenario: Print mode for a library resolves the declared start command
    Given a project with a library run declaration with a real start command
    When the user runs `givn run --print`
    Then the command exits 0
    And stdout is one line that names the start command without exporting a port

  @givn.added
  Scenario: Print mode on an unconfigured declaration reports preflight findings
    Given a project with an unconfigured run declaration
    When the user runs `givn run --print`
    Then the command exits 1
    And stderr mentions "run.command is unset"
    And stdout does not contain "givn missing-run-command"
