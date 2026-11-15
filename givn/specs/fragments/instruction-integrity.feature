Feature: Instruction and skill guidance remains complete and machine-readable

  Scenario: Canonical E2E guidance rejects weaker interface evidence
    Given the canonical specs guidance is requested
    When an author reads the interaction and E2E policy
    Then the guidance requires primary assertions on the named real interface
    And the guidance rejects repository-only evidence for a higher-level interface
    And the guidance requires a real browser for browser UI interactions
    And the guidance treats an unresolved interface obstacle as a design gap

  Scenario: An author requests the canonical interaction and E2E guidance
    Given a Givn project with an active change
    When the author requests the specs instructions through the Givn CLI
    Then the CLI output contains the complete interaction and E2E policy
    And the CLI output no longer advertises retrieval commands

  @instruction-integrity
  Scenario: Design guidance requires the complete E2E setup record
    Given  a Givn project with an active change
    When  the author requests the design instructions through the Givn CLI
    Then  the guidance requires explicit regular and E2E scope selection and the appropriate real interface driver
    And  bootstrap requires a project runner to execute the selected Gherkin features from GIVN_FEATURES
    And  bootstrap requires change to execute every delta scenario and regular/E2E to partition permanent scenarios
    And  bootstrap requires fresh actual counts in GIVN_RESULT_FILE
    And  bootstrap distinguishes setup failure from a behavior assertion RED
    And  the runner contract keeps missing malformed zero failed and skipped results fail-closed
    And  the commands template points to the runner contract instead of shipping a generic runner script
    And  runner strictness is proved at initial setup and after relevant runner version invocation or failure-configuration changes
    And  local infrastructure is required only for dependencies the product actually has
    And  the guidance retains framework justification interaction coverage and applicable visual evidence

  Scenario: Machine-readable instructions describe the effective artifact
    Given the effective specs artifact metadata is available
    When an agent inspects the specs instructions as JSON
    Then the response identifies the artifact
    And the response lists the generated paths and prerequisites
    And the response contains the resolved instruction text

  Scenario: Artifact instructions reject an unsafe change context
    Given the effective specs artifact metadata is available
    When an agent requests the specs instructions for an unsafe change context
    Then the Givn CLI rejects the request with a clear kebab-case error

  Scenario: Project instruction overrides survive skill synchronization
    Given a project specs instruction override is prepared
    When the agent synchronizes skills and requests the specs instructions
    Then the resolved instruction contains the project override
    And the project override file remains unchanged

  @e2e
  Scenario: An agent requests machine-readable artifact guidance
    Given a Givn project with an active change
    When the agent requests the specs instructions as JSON through the Givn CLI
    Then the response is valid JSON with artifact paths prerequisites and instructions

  Scenario: Retrieval guidance distinguishes build capabilities
    Given  a Givn project with an active change
    When  the author requests the specs instructions through the Givn CLI
    Then  the returned guidance does not mention engine-backed retrieval commands
    And  the returned guidance does not offer advisory or blocking retrieval paths
    And  the returned guidance keeps the deterministic spec toolchain wording

  @instruction-integrity
  Scenario: Review guidance names the deterministic disposition gates
    Given  a Givn project with an active change
    When  the reviewer requests the review instructions through the Givn CLI
    Then  the guidance requires split-or-keep for each reported long scenario with a brief keep reason
    And  a chosen split must be completed before sign-off
    And  shape-match pairs are advisory discovery rather than mandatory dispositions
    And  advisory findings are not completed verification evidence

  Scenario: The canonical dialog policy is retrievable through the Givn CLI
    Given  a Givn project with an active change for the dialog policy
    When  the author requests the dialog policy through the Givn CLI
    Then  the returned policy applies to every user-facing question inside and outside a named workflow
    And  the returned policy assumes project familiarity without assuming knowledge of every referenced artifact
    And  the returned policy requires the decision subject brief sourced context and concrete consequences for each answer
    And  the returned policy requires material later-change costs and the benefit purchased by difficult commitments
    And  the returned policy directs the author to settle discoverable facts and confirmed constraints before asking
    And  the returned policy distinguishes authoring obligations from runtime enforcement

  @instruction-integrity
  Scenario: The canonical dialog policy demonstrates the full question record
    Given  an initialized Givn project
    When  the author requests the dialog policy through the Givn CLI
    Then  the concise policy requires decision-relevant context alternatives consequences and resolution
    And  chosen and rejected alternatives retain their reasons
    And  routine clarification does not require invented alternatives
    And  confirmations explain acceptance and revision consequences without making consent a rule waiver
    And  the policy indexes the companion regression examples by question type

  Scenario: The dialog policy is retrievable without an active change
    Given  a project root with givn/ initialised
    When  the author requests the dialog policy through the Givn CLI without a change
    Then  the command exits 0
    And  the returned general policy governs question preparation without requiring a selected change
    And  the returned policy distinguishes routine clarification from decisions that require alternatives and consequences

  Scenario: An unresolvable instruction id keeps the known-artifact error
    Given  a project root with givn/ initialised
    When  the author requests an unknown instruction through the Givn CLI
    Then  the command fails
    And  stderr contains "unknown artifact"

  Scenario: The design-review template carries the per-question record shape
    Given  a givn project
    When  the user runs `givn eject design-review:template`
    Then  the ejected design-review template contains "Branch"
    And  the ejected design-review template contains "Outcome"
    And  the ejected design-review template contains "In practice:"
    And  the ejected design-review template contains "Recommended:"
    And  the ejected design-review template contains "Rejected:"

  Scenario: The review instruction carries the use-case scope rejection criteria
    Given  the review instructions are requested
    When  the reviewer reads the use-case scope criteria
    Then  the guidance rejects a goal conjunction
    And  the guidance rejects a missing primary actor
    And  the guidance rejects a capability that fails the placement test
    And  the guidance rejects a mutating scenario under an observing goal
    And  the guidance rejects an extension without a step reference
    And  the guidance rejects a precondition established by the flow
    And  the guidance rejects mechanism vocabulary in the goal trigger or guarantees
    And  the guidance rejects a component- or channel-shaped name
    And  the guidance rejects an oversized leaf
    And  the guidance rejects relationship misuse

  Scenario: The design-review guidance points at the use-case scope criteria
    Given  the design-review instructions are requested
    When  the reviewer reads the use-case branch
    Then  the guidance points at the use-case scope criteria

  Scenario: The specs guidance carries the capability placement test
    Given  a Givn project with an active change
    When  the author requests the specs instructions through the Givn CLI
    Then  the returned guidance carries the placement test
    And  the returned guidance rejects a channel as an answer
    And  the returned guidance requires goal-shaped capability names

  Scenario: The proposal instruction carries the question disposition rule
    Given  the proposal instructions are requested
    When  an author reads the question disposition rule
    Then  the guidance requires each intersecting seed question to be dispositioned
    And  the guidance names the answered carried and out-of-scope dispositions
    And  the guidance rejects a bare deferral to the design
    And  the guidance assigns stable question IDs
    And  the guidance marks the disposition rule advisory without an ideation source

  Scenario: The design and design-review guidance carry the decision ledger
    Given  the design and design-review instructions are requested
    When  a maintainer reads the ledger contract
    Then  the guidance requires a decisions ledger mapping each question ID
    And  the guidance records decisions as "Decides D" with their realization
    And  the guidance records the seed question reference beside the ledger ID
    And  the design-review guidance treats an unresolved unrecorded question as a finding

  Scenario: The review instruction rejects an unqualified guarantee with an open prerequisite
    Given  the review instructions are requested
    When  the reviewer reads the use-case scope criteria
    Then  the guidance rejects an unqualified guarantee whose prerequisite is open

  Scenario: The proposal design and design-review templates carry the ledger shapes
    Given  a givn project
    When  the user runs `givn eject proposal:template`
    And  the user runs `givn eject design:template`
    And  the user runs `givn eject design-review:template`
    Then  the file "givn/artifacts/proposal/proposal.md" contains "Disposition"
    And  the file "givn/artifacts/design/design.md" contains "## Decisions"
    And  the file "givn/artifacts/design/design.md" contains "Decides D"
    And  the file "givn/artifacts/design-review/design-review.md" contains "ledger"

  Scenario: The propose guidance carries the no-signal routing rule
    Given  a givn project
    And  the proposal instructions are requested
    When  an author reads the routing rule
    Then  the guidance tells the author to omit the recommendation when route reports no signal
    And  the guidance keeps the recorded decision mandatory
    When  the user runs `givn eject proposal:template`
    Then  the file "givn/artifacts/proposal/proposal.md" contains "no signal"

  @instruction-integrity
  Scenario: Dialog teaching contrasts show complete decisions rather than bare labels
    Given  an initialized Givn project
    When  the author follows the dialog policy index to its companion regression examples
    Then  the companion retains the scope-approval target-selection behavior-choice persistent-format-commitment and discoverable-diagnosis examples
    And  the examples distinguish hypothetical teaching records from current product behavior
    And  each good example supplies its decision record and concrete answer consequences
    And  the diagnostic example reports the researched assertion defect without asking the person to inspect unseen scenarios
    And  no answer violates a confirmed one-answer constraint

  @instruction-integrity
  Scenario: Migration consent does not imply an unverified recovery guarantee
    Given  an initialized Givn project
    When  the author reads the persistent-format-commitment example linked from the dialog policy
    Then  the example distinguishes recovery requiring verification from recovery already supported
    And  the less-committing answer leaves saved projects unchanged without promising a reversible upgrade
    And  unknown compatibility facts must be established or consent narrowed before rollout approval

  Scenario: A project dialog override remains the effective policy after synchronization
    Given  an initialized Givn project with a project-owned dialog instruction override
    And  the override requires human approval before broadening the selected project scope
    When  the author synchronizes guidance and requests the dialog instructions through the Givn CLI
    Then  the returned policy is the project-owned dialog override rather than the bundled default
    And  the project-owned override remains unchanged on disk
    And  the synchronized entry guidance directs authors to the effective policy rather than a bundled policy copy

  @e2e @instruction-integrity
  Scenario: Author starts a session with the universal dialog obligation
    Given  an initialized Givn project without an active change
    When  the author requests the normal Givn startup instructions through the Givn CLI
    Then  the startup guidance requires loading the effective dialog policy before the context's first user-facing question
    And  the bootstrap owns that obligation independently of optional addons and named skills
    And  the policy is loaded once per context and reloaded if lost
    And  the startup guidance says to load givn-start for new or unclassified work
    And  the startup guidance names givn new as the direct alternative
    And  the terminal startup output recommends givn-start and the direct change command
    And  the terminal transcript is captured for this scenario

  @instruction-integrity
  Scenario: Machine-readable startup guidance retains the dialog obligation while reporting an active change
    Given  an initialized Givn project with an active change awaiting specifications
    When  the author requests the normal Givn startup instructions as JSON
    Then  the response preserves the active change identity and next required artifact
    And  the response includes the same universal effective-dialog loading obligation in its guidance
    And  the response does not embed a second complete dialog policy
    And  the response says the reported step applies when continuing that change
    And  the response says the reported step does not authorize unrelated work

  @instruction-integrity
  Scenario: The characterization guidance requests the reverse review from Givn
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  every selected target receives a givn-characterize skill directing the author to `givn instructions review-reverse`
    And  no separately generated characterization command is installed

  Scenario: The generated guidance references resolve with the default configuration
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  every guidance reference resolves in the consumer project or is owned by a named creation step
    And  every agent target Givn supports is covered by the synchronized guidance

  Scenario: The generated guidance references resolve without addons
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn skills sync`
    Then  every guidance reference resolves in the consumer project or is owned by a named creation step

  Scenario: The printed instructions reference only resolvable files
    Given  a Givn project with an active change
    When  the author requests every instruction Givn prints
    Then  every printed guidance reference resolves in the consumer project or is owned by a named creation step

  Scenario: An undeclared reference in an installed skill fails the check
    Given  a givn project with all agent skill targets enabled
    And  a givn-ideation skill override that names the missing file "givn/reference-that-resolves-nowhere.md"
    When  the user runs `givn skills sync`
    Then  the installed reference check fails and names "givn/reference-that-resolves-nowhere.md"

  Scenario: A printed reference that resolves nowhere fails the check
    Given  a Givn project with an active change
    And  a specs instruction override that names the missing file "givn/reference-that-resolves-nowhere.md"
    When  the author requests the specs instructions through the Givn CLI
    Then  the printed reference check fails and names "givn/reference-that-resolves-nowhere.md"

  Scenario: A configured target without synchronized guidance fails the check
    Given  a givn project with all agent skill targets enabled
    When  the skills of the agent target ".claude" are removed
    Then  the configured-target check fails and names the agent target ".claude"

  @instruction-integrity
  Scenario: Every instruction id named by generated guidance resolves through the Givn CLI
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  every generated skill names a served instruction
    And  every named instruction id resolves through the Givn CLI

  Scenario: Every generated direction is invokable in its documented context
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  every direction to an artifact instruction names the change context it needs
    And  every direction to a policy or step instruction names no change context
    And  every named instruction id resolves in the context the direction names

  @instruction-integrity
  Scenario: Every generated skill and command directs to exactly one served instruction id
    Given  a givn project with all agent skill targets enabled and an active change
    When  the user runs `givn skills sync`
    Then  the command succeeds
    And  every generated skill directs to exactly one served instruction id
    And  no separately authored agent command is generated

  @instruction-integrity
  Scenario: Relevant dialog examples are required before the first question of their type
    Given  the effective dialog policy has been loaded in the current context
    When  the author prepares the first question of a type indexed by the policy
    Then  the policy requires reading that type's companion examples before presenting the question
    And  repeated questions of that type do not require rereading examples retained in context
    And  questions remain one at a time

  @instruction-integrity
  Scenario Outline: Givn shipped templates render parser-compatible documents and examples
    Given  Givn's shipped "<asset>" template with its documented example values
    When  a Givn maintainer renders the template for its documented use
    Then  frontmatter is valid wherever the document requires it
    And  Markdown fences are balanced and preserve the intended example boundaries
    And  ordinary parsers for the documented formats accept the step tables and configuration samples without Givn-specific repairs
    Examples:
      | asset |
      | artifact documents |
      | agent skills |
      | step tables |
      | verification configuration |

  @instruction-integrity
  Scenario Outline: Current shipped guidance and help exclude harness-project leakage without rewriting history
    Given  Givn's current "<surface>" and preserved historical evidence
    When  a Givn maintainer inspects the release's consumer-facing content
    Then  harness-project paths fixture names and recording content are absent from current consumer guidance
    And  legitimate generic examples remain usable outside Givn's own repository
    And  historical decisions and evidence remain unchanged and are clearly distinguished from current guidance
    Examples:
      | surface |
      | shipped guidance assets |
      | command help |

  @instruction-integrity
  Scenario: Givn release distribution excludes local recordings and harness fixtures
    Given  a Givn release checkout containing local runtime recordings and harness fixtures
    When  a Givn maintainer prepares the release distribution
    Then  the distributed package contains neither runtime recordings nor harness fixtures
    And  the checkout's historical evidence remains unchanged
    And  this Givn distribution requirement installs no release gate in consumer projects

  @instruction-integrity
  Scenario: Machine-readable no-active startup guidance carries the start route
    Given  an initialized Givn project without an active change
    When  the author requests the normal Givn startup instructions as JSON
    Then  the response names givn-start as the next skill
    And  the response keeps givn new as the direct command
    And  the response presents the direct command as optional

  @instruction-integrity
  Scenario: The active-change startup guidance qualifies its next step
    Given  an initialized Givn project with an active change awaiting specifications
    When  the author requests the normal Givn startup instructions through the Givn CLI
    Then  the startup guidance says the reported step applies when continuing that change
    And  the startup guidance says the reported step does not authorize unrelated work

  @instruction-integrity
  Scenario: The ise instruction carries the five rebuild obligations
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction names Specification completeness, the two-layer test boundary, persisted-state continuity, the change strategy, and the spec gap classification
    And  the served instruction uses the established Component, Port, contract, Specification, and Source code vocabulary

  @instruction-integrity
  Scenario: The review guidance asks the neighboring-components question without a gate
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the review guidance
    Then  the command exits 0
    And  the served review guidance asks for the neighboring components changed outside the replaced component and the reason
    And  the served review guidance states that the answer is a review question, never a gate

  @instruction-integrity
  Scenario: Every instruction id a served instruction names resolves
    Given  a Givn project with an active change and the guidance reference check available
    When  the maintainer runs the guidance reference check over every served instruction
    Then  every instruction id a served instruction names resolves in the context that instruction needs
    And  every instruction id an effective addon instruction names resolves
    And  the check names no unresolved instruction id

  @instruction-integrity
  Scenario: A dangling stack instruction id fails the reference check by name
    Given  a Givn project with an active change
    And  a steps instruction override that names "steps-rustt"
    When  the maintainer runs the guidance reference check over the served steps guidance
    Then  the check fails
    And  it names "steps-rustt"

  @instruction-integrity
  Scenario: A policy instruction named with a change context is a finding
    Given  a Givn project with an active change and the guidance reference check available
    And  a served instruction names "proposal" without a change context
    And  a served instruction names "steps" with a change context
    When  the maintainer runs the guidance reference check over the served guidance
    Then  the check fails
    And  it names "steps" and the extra change context
    And  it reports no finding for "proposal"

  @instruction-integrity
  Scenario: The ise instruction names only instruction ids that resolve
    Given  a Givn project with the ise addon enabled and the guidance reference check available
    When  the maintainer requests the ise instruction and runs the reference check over it
    Then  the served text names "steps" as the mapping for the change's stack mechanics
    And  the served text names "steps-rust" as one stack instruction example
    And  the served text names the artifact instruction that owns the change-strategy record and the one that owns the spec-gap record
    And  every named instruction id resolves

  @instruction-integrity
  Scenario: The review guidance keeps every surviving obligation
    Given  a Givn project with an active change
    When  the maintainer requests the review instructions through the Givn CLI
    Then  the served text carries "`@e2e` tag integrity"
    And  the served text carries "explicit E2E scope selection"
    And  the served text carries "receipt reading"
    And  the served text carries "opposite classification"
    And  the served text carries "local run command"
    And  the served text carries "second `@e2e` file"
    And  the served text carries "Design match:"
    And  the served text carries "Interaction coverage maps"
    And  the served text carries "no fourth bucket"

  @instruction-integrity
  Scenario: The ise instruction says declared retained paths survive archiving
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the `ise` instruction
    Then  the served instruction says a declared retained path must survive `givn archive`
    And  the served instruction names the archive location a change's artifacts move to
