@givn.delta @serve-extended-rules

Feature: The served instruction carries the built-in rules and the project's extension

  @givn.added
  Scenario Outline: Core step guidance names each stack instruction with its load condition
    Given  a Givn project with an active change
    When  the author requests the steps instructions through the Givn CLI
    Then  the served text names "<id>" when the change's runner is "<runner>"
    And  the served text carries no bare "<id>" direction without that condition

    Examples:
      | id           | runner        |
      | steps-rust   | cucumber-rs   |
      | steps-python | behave        |
      | steps-node   | cucumber-js   |
      | steps-java   | cucumber-jvm  |
      | steps-go     | godog         |
      | steps-ruby   | cucumber-ruby |

  @givn.added
  Scenario: Core step guidance carries no stack body
    Given  a Givn project with an active change
    When  the author requests the steps instructions through the Givn CLI
    Then  the served text carries no per-runner step-definition path
    And  the served text carries no empty-body stub form
    And  the served text carries no per-runner binding instruction
    And  the served text still requires one step definition file per capability
    And  the served text keeps the archive binding-token list for every runner

  @givn.added
  Scenario Outline: Design guidance names each stack instruction with its load condition and carries no runner table
    Given  a Givn project with an active change
    When  the author requests the design instructions through the Givn CLI
    Then  the served text names "<id>" when the change's runner is "<runner>"
    And  the served text carries no per-runner strict-mode mechanism
    And  the served text carries no per-stack single-scenario run command
    And  the served text still requires the runner's strict-mode proof

    Examples:
      | id           | runner        |
      | steps-rust   | cucumber-rs   |
      | steps-python | behave        |
      | steps-node   | cucumber-js   |
      | steps-java   | cucumber-jvm  |
      | steps-go     | godog         |
      | steps-ruby   | cucumber-ruby |

  @givn.added
  Scenario: Specs and review guidance carry no stack body
    Given  a Givn project with an active change
    When  the author requests the specs and review instructions through the Givn CLI
    Then  neither served text names a stack instruction
    And  neither served text carries a per-runner mechanism

  @givn.added
  Scenario: The tasks guidance points at the stack mapping and carries no stack body
    Given  a Givn project with an active change
    When  the author requests the tasks and reverse-engineering tasks instructions through the Givn CLI
    Then  the tasks text names "steps" as the stack mapping
    And  neither served text carries a per-runner step path, stub, or strict mechanism
    And  both served texts still require the strict-mode proof

  @givn.added
  Scenario Outline: Every stack instruction is served only on its own request with its runner body
    Given  a Givn project with an active change
    And  the author has requested the steps instructions
    When  the author requests "<id>"
    Then  the command exits 0
    And  the served text carries "<binding>"
    And  the served text carries "<stub>"
    And  the served text carries "<strict>"
    And  the served text carries "<command>"
    And  the previously served steps text carries none of the stub, strict mechanism, or run command
    And  the previously served steps text carries no per-runner binding instruction

    Examples:
      | id           | binding     | stub                                  | strict                | command                                   |
      | steps-rust   | `#[given(`  | `unimplemented!()`                    | `.fail_on_skipped()`  | `cargo test <name> -- --exact`            |
      | steps-python | `@given(`   | `raise NotImplementedError("TODO")`   | `raise`               | `behave -n "<title>"`                     |
      | steps-node   | `.Given(`   | `return "pending";`                   | `--strict`            | `cucumber-js --name "<title>"`            |
      | steps-java   | `@Given(`   | PendingException("TODO")              | `--strict`            | `mvn test -Dcucumber.filter.name="<title>"` |
      | steps-go     | `ctx.Step(`  | `godog.ErrPending`                    | fails by default      | `go test ./... -run TestFeatures`         |
      | steps-ruby   | `Given(`    | `pending("TODO")`                     | fail on pending       | `cucumber --name "<title>"`               |

  @givn.added
  Scenario: The machine-readable stack path carries the same content as the text path
    Given  a Givn project with an active change
    When  the author requests "steps-rust" as machine-readable output
    Then  the command exits 0
    And  the machine-readable instruction field equals the served text

  @givn.added
  Scenario: An unknown stack instruction id is refused by name with the known list
    Given  a Givn project with an active change
    When  the author requests "steps-rustt"
    Then  the command exits non-zero
    And  stderr names "steps-rustt"
    And  stderr lists the known instruction ids including "steps-rust"
