Feature: The served instruction carries the built-in rules and the project's extension

  @e2e
  Scenario: An agent receives the extended rules through the Givn CLI
    Given  a Givn project with an active change
    And  the project records an extension for the proposal instruction
    When  the agent requests the proposal instructions through the Givn CLI
    Then  the command exits 0
    And  the served content carries the built-in proposal rules
    And  the served content carries the project's rule in the marked extension section

  Scenario: The built-in rules are served alone when no extension exists
    Given  a Givn project with an active change
    When  the agent requests the specs instructions through the Givn CLI
    Then  the served content carries the built-in specs rules
    And  the served content carries no extension section

  Scenario: The extension adds to the built-in rules without replacing or reordering them
    Given  a Givn project with an active change
    And  the project records an extension for the proposal instruction
    And  the extension carries the rule "Never write a release note"
    When  the agent requests the proposal instructions through the Givn CLI
    Then  every built-in rule of the proposal instruction remains in the served content
    And  the built-in rules appear before the marked extension section
    And  the contradicting project rule appears beside the built-in release-note rule

  Scenario: The machine-readable path carries the same extended content as the text path
    Given  a Givn project with an active change
    And  the project records an extension for the proposal instruction
    When  the agent requests the proposal instructions as JSON through the Givn CLI
    Then  the response identifies the artifact and its generated paths and prerequisites
    And  the response carries the same instruction content as the text path
    And  the response carries the project's rule in the marked extension section

  Scenario: The extension applies on top of a project-owned replacement
    Given  a Givn project with an active change
    And  a project-owned replacement for the proposal instruction
    And  the project records an extension for the proposal instruction
    When  the agent requests the proposal instructions through the Givn CLI
    Then  the served content carries the replacement text
    And  the served content carries the project's rule in the marked extension section

  Scenario: A guidance request changes nothing in the project
    Given  a Givn project with an active change
    And  the project records an extension for the proposal instruction
    When  the agent requests the instructions for every served step
    Then  no project file changed

  @serve-extended-rules
  Scenario: Served implementation guidance requires behavioral RED before production edits
    Given  a coherent named behavior batch is planned in tasks with scenarios validation and an intended commit boundary
    When  the author requests the implementation and steps guidance
    Then  tests and bindings must run before production edits for new behavior
    And  the observed RED must be a meaningful behavioral failure rather than missing bindings syntax registration fixtures or environment failure
    And  the actual command and short observed failure are recorded compactly in the batch task as they occur
    And  minimum implementation is followed by actual GREEN and REFACTOR checks recorded briefly

  @serve-extended-rules
  Scenario: Immediately passing tests protect existing behavior without a production edit
    Given  a distinct requirement already holds before its regression test is written
    When  the author reads the implementation guidance for an immediately passing test
    Then  the requirement is explicitly recorded as existing behavior protected
    And  no production edit is made for that requirement
    And  another requirement in the same coherent batch may still require behavioral RED and production edits

  @serve-extended-rules
  Scenario: Skipped RED is admitted rather than reconstructed from history
    Given  production behavior was edited before the requirement's behavioral test was run
    When  the author reads the test-first evidence guidance
    Then  the skipped RED must be admitted
    And  backdated evidence history rewriting and deliberately breaking finished behavior are forbidden
    And  mutation or discrimination tests cannot be claimed as historical RED
    And  separate RED GREEN and REFACTOR commits or retrospective phase-shaped history gates are not required

  @serve-extended-rules
  Scenario: A coherent behavior batch is completed with its actual explicit-path revision
    Given  a behavior batch has completed its actual GREEN and REFACTOR checks
    When  the author reads the batch completion guidance
    Then  the intended explicit-path diff is inspected before a coherent completed batch commit
    And  the actual revision is recorded before scenario tasks are completed
    And  commit selections name the batch files explicitly rather than implicitly staging all work
    And  no production diff is required for each scenario that protects existing behavior

  @serve-extended-rules
  Scenario: Refactor guidance uses the outline once for supported changed components and their neighbors
    Given  a behavior batch changes supported and unsupported components
    When  the author reads the REFACTOR guidance
    Then  supported changed components receive one advisory outline inspection per batch
    And  the assessment discloses which languages and components the outline covers and which it omits
    And  unsupported components receive direct source assessment without forced outline analysis
    And  the assessment asks whether responsibilities are cohesive public interfaces narrow dependencies correctly directed and decomposition understandable
    And  suspected problems are confirmed against source before conclusions or edits
    And  structural call order is not represented as a complete runtime trace
    And  uncertain or omitted edges limit the conclusions
    And  a later structural edit is the reason to regenerate a stale outline rather than every small edit
    And  the report is reused until structural changes make its relevant evidence stale
    And  nonstructural edits alone do not require rerunning the outline

  @serve-extended-rules
  Scenario: Unsupported outline languages receive source assessment without a waiver
    Given  a behavior batch changes unsupported components
    When  the author reads the REFACTOR guidance
    Then  the same responsibility interface dependency and decomposition questions are assessed directly against source
    And  unavailable outline support is not failure or a reason for a waiver
    And  no outline verdict metric waiver or forced reviewer context is required

  @serve-extended-rules
  Scenario: Encountered cleanup is a separate GREEN refactoring batch
    Given  the author encounters a concrete cleanup opportunity while implementing agreed behavior
    When  the author reads the cleanup guidance
    Then  the cleanup is made in a separate behavior-preserving refactoring commit with explicit paths and actual GREEN checks
    And  the cleanup does not authorize speculative features or a broad abstraction rewrite

  @serve-extended-rules
  Scenario: Served development guidance requires modular boundaries from the start
    Given  a project is about to make its first production implementation
    When  the author requests the development principles guidance
    Then  the guidance requires cohesive responsibility modules narrow consumer contracts external adapters separate composition and explicit dependency direction
    And  a narrow module API is acceptable unless isolation or substitution needs an explicit interface
    And  the guidance does not demand hypothetical providers or plugin registries

  @serve-extended-rules
  Scenario: Refactor guidance treats an unreadable changed unit as a finding
    Given  a behavior batch changes a unit whose outline cannot carry its story
    When  the author reads the REFACTOR guidance
    Then  the outline must state the unit's responsibility and ordered call flow without bodies or comments
    And  unit type method parameter return and called-type names must state the domain operation
    And  generic machinery names are readability findings
    And  metrics and classifications never overrule an unreadable outline
    And  source confirms a suspected problem rather than substituting for the outline story
    And  an unreadable changed unit is fixed in REFACTOR or recorded as a substantive review finding

  @serve-extended-rules
  Scenario: Implementation guidance routes to the shared step mechanics
    Given  a Givn project with an active change
    When  the author requests the tasks instruction
    Then  the served guidance names givn instructions steps as the step mechanics

  @serve-extended-rules
  Scenario: The start guidance recommends a proportionate outcome
    Given  a Givn project
    Then  the served start instruction contains "Answer and stop"
    And  the served start instruction contains "Direct work without a Givn change"
    And  the served start instruction contains "givn-ideation"
    And  the served start instruction contains "givn-propose"
    And  the served start instruction contains "proportionality"
    And  the served start instruction contains "creates no workflow artifacts"
    And  the served start instruction contains "do not assume every modification requires a Givn change"

  @serve-extended-rules
  Scenario: The start guidance preserves direct-work obligations
    Given  a Givn project
    Then  the served start instruction contains "Read the relevant code"
    And  the served start instruction contains "focused correction"
    And  the served start instruction contains "regression coverage"
    And  the served start instruction contains "Report the modification and verification evidence"
    And  the served start instruction contains "Do not create a proposal, design, task list, or archive ceremony"
    And  the served start instruction contains "authorization to edit is not authorization to commit"

  @serve-extended-rules
  Scenario: The start guidance keeps the handoff with the user
    Given  a Givn project
    Then  the served start instruction contains "wait unless continuation is already authorized"
    And  the served start instruction contains "An ordinary request to discuss is not authorization"
    And  the served start instruction contains "Direct entry remains available"

  @serve-extended-rules
  Scenario: The start guidance is available without optional addons
    Given  a project root with givn/ initialised without addons
    Then  the served start instruction contains "Answer and stop"
    And  the served start instruction contains "Direct work without a Givn change"

  @serve-extended-rules
  Scenario: The retired explore instruction has no callable alias
    Given  a Givn project
    When  the user runs `givn instructions explore`
    Then  the command fails
    And  stderr contains "explore"

  @serve-extended-rules
  Scenario: A project-owned explore override cannot restore a callable alias
    Given  a Givn project with a project-owned explore instruction override
    When  the user runs `givn instructions explore`
    Then  the command fails
    And  stderr contains "givn instructions start"

  @serve-extended-rules
  Scenario: The ise instruction is served while ise is effective
    Given  an ise-enabled project
    When  the user requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction names the six rebuild obligations

  @serve-extended-rules
  Scenario: An ise instruction request with the addon off is refused by name
    Given  an initialized project with ise disabled
    When  the user requests the `ise` instruction
    Then  the command exits non-zero
    And  stderr names "ise"
    And  no served instruction content is printed

  @serve-extended-rules
  Scenario: The machine-readable ise path carries the same content as the text path
    Given  an ise-enabled project
    When  the user requests the `ise` instruction as machine-readable output
    Then  the command exits 0
    And  the machine-readable instruction field equals the served text

  @serve-extended-rules
  Scenario Outline: Core step guidance names each stack instruction with its load condition
    Given  a Givn project with an active change
    When  the author requests the steps instructions through the Givn CLI
    Then  the served text names "<id>" when the change's runner is "<runner>"
    And  the served text carries no bare "<id>" direction without that condition
    Examples:
      | id | runner |
      | steps-rust | cucumber-rs |
      | steps-python | behave |
      | steps-node | cucumber-js |
      | steps-java | cucumber-jvm |
      | steps-go | godog |
      | steps-ruby | cucumber-ruby |

  @serve-extended-rules
  Scenario: Core step guidance carries no stack body
    Given  a Givn project with an active change
    When  the author requests the steps instructions through the Givn CLI
    Then  the served text carries no per-runner step-definition path
    And  the served text carries no empty-body stub form
    And  the served text carries no per-runner binding instruction
    And  the served text still requires one step definition file per capability
    And  the served text keeps the archive binding-token list for every runner

  @serve-extended-rules
  Scenario Outline: Design guidance names each stack instruction with its load condition and carries no runner table
    Given  a Givn project with an active change
    When  the author requests the design instructions through the Givn CLI
    Then  the served text names "<id>" when the change's runner is "<runner>"
    And  the served text carries no per-runner strict-mode mechanism
    And  the served text carries no per-stack single-scenario run command
    And  the served text still requires the runner's strict-mode proof
    Examples:
      | id | runner |
      | steps-rust | cucumber-rs |
      | steps-python | behave |
      | steps-node | cucumber-js |
      | steps-java | cucumber-jvm |
      | steps-go | godog |
      | steps-ruby | cucumber-ruby |

  @serve-extended-rules
  Scenario: Specs and review guidance carry no stack body
    Given  a Givn project with an active change
    When  the author requests the specs and review instructions through the Givn CLI
    Then  neither served text names a stack instruction
    And  neither served text carries a per-runner mechanism

  @serve-extended-rules
  Scenario: The tasks guidance points at the stack mapping and carries no stack body
    Given  a Givn project with an active change
    When  the author requests the tasks and reverse-engineering tasks instructions through the Givn CLI
    Then  the tasks text names "steps" as the stack mapping
    And  neither served text carries a per-runner step path, stub, or strict mechanism
    And  both served texts still require the strict-mode proof

  @serve-extended-rules
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
      | id | binding | stub | strict | command |
      | steps-rust | `#[given(` | `unimplemented!()` | `.fail_on_skipped()` | `cargo test <name> -- --exact` |
      | steps-python | `@given(` | `raise NotImplementedError("TODO")` | `raise` | `behave -n "<title>"` |
      | steps-node | `.Given(` | `return "pending";` | `--strict` | `cucumber-js --name "<title>"` |
      | steps-java | `@Given(` | PendingException("TODO") | `--strict` | `mvn test -Dcucumber.filter.name="<title>"` |
      | steps-go | `ctx.Step(` | `return godog.ErrPending` | fails by default | `go test ./... -run TestFeatures` |
      | steps-ruby | `Given(` | `pending("TODO")` | fail on pending | `cucumber --name "<title>"` |

  @serve-extended-rules
  Scenario: The machine-readable stack path carries the same content as the text path
    Given  a Givn project with an active change
    When  the author requests "steps-rust" as machine-readable output
    Then  the command exits 0
    And  the machine-readable instruction field equals the served text

  @serve-extended-rules
  Scenario: An unknown stack instruction id is refused by name with the known list
    Given  a Givn project with an active change
    When  the author requests "steps-rustt"
    Then  the command exits non-zero
    And  stderr names "steps-rustt"
    And  stderr lists the known instruction ids including "steps-rust"
