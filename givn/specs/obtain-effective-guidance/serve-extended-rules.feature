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
