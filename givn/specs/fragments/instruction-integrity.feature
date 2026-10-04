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

  Scenario: Design guidance requires the complete E2E setup record
    Given  a Givn project with an active change
    When  the author requests the design instructions through the Givn CLI
    Then  the returned design guidance requires the runner scope strictness proof separate bindings real driver and local infrastructure
    And  the returned design guidance requires framework justification and a matching interaction coverage matrix
    And  the returned design guidance distinguishes terminal transcripts from Web UI viewport evidence

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
    Given  the synchronized agent guidance for the specs workflow
    When  an author reads the guidance
    Then  the guidance does not mention engine-backed retrieval commands
    And  the guidance does not offer advisory or blocking retrieval paths
    And  the guidance keeps the deterministic spec toolchain wording

  Scenario: Review guidance names the deterministic disposition gates
    Given  a Givn project with an active change
    When  the reviewer requests the review instructions through the Givn CLI
    Then  the returned review guidance requires a disposition for every reported overlap pair with a reason
    And  the returned review guidance requires a split-or-keep decision for every reported long scenario
    And  the returned review guidance distinguishes an advisory finding from completed verification evidence

  Scenario: The canonical dialog policy is retrievable through the Givn CLI
    Given  a Givn project with an active change for the dialog policy
    When  the author requests the dialog policy through the Givn CLI
    Then  the returned policy applies to every user-facing question inside and outside a named workflow
    And  the returned policy assumes project familiarity without assuming knowledge of every referenced artifact
    And  the returned policy requires the decision subject brief sourced context and concrete consequences for each answer
    And  the returned policy requires material later-change costs and the benefit purchased by difficult commitments
    And  the returned policy directs the author to settle discoverable facts and confirmed constraints before asking
    And  the returned policy distinguishes authoring obligations from runtime enforcement

  Scenario: The canonical dialog policy demonstrates the full question record
    Given  an initialized Givn project
    When  the author requests the dialog policy through the Givn CLI
    Then  the decision-record guidance keeps the complete context alternatives consequences and resolution
    And  the chosen and rejected alternatives retain their reasons after the question closes
    And  decision-relevant cross-references carry their meaning rather than only their identifier
    And  routine information requests are not forced into invented alternatives or tradeoffs
    And  confirmations disclose both acceptance and revision consequences without treating consent as a rule waiver

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
    Given  the synchronized agent guidance for the specs workflow
    When  an author reads the capability placement guidance
    Then  the guidance carries the placement test
    And  the guidance rejects a channel as an answer
    And  the guidance requires goal-shaped capability names
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
  Scenario: Dialog teaching contrasts show complete decisions rather than bare labels
    Given  an initialized Givn project
    When  the author requests the dialog policy through the Givn CLI
    Then  the returned teaching examples identify their records as hypothetical rather than current product behavior
    And  the goal-confirmation contrast displays every proposed goal and bounds what acceptance authorizes
    And  the clarification contrast asks only the missing human intent and shows how the answer scopes subsequent work
    And  the behavior-choice contrast shows the observable outcome of both answers rather than implementation labels
    And  the commitment contrast shows the saved-information benefit compatibility restriction and later-change work
    And  the diagnostic contrast reports a researched assertion defect rather than asking the person to inspect unseen scenarios
    And  every good contrast links to its own supplied decision record
    And  the diagnostic contrast excludes an answer that violates the confirmed one-answer constraint

  Scenario: Migration consent does not imply an unverified recovery guarantee
    Given  an initialized Givn project
    When  the author reads the material-commitment teaching contrast from the returned dialog instructions
    Then  the contrast distinguishes required recovery verification from already implemented recovery support
    And  the less-committing answer leaves saved projects unchanged rather than promising a reversible upgrade
    And  the policy requires unknown compatibility facts to be established or consent narrowed before rollout approval

  Scenario: A project dialog override remains the effective policy after synchronization
    Given  an initialized Givn project with a project-owned dialog instruction override
    And  the override requires human approval before broadening the selected project scope
    When  the author synchronizes guidance and requests the dialog instructions through the Givn CLI
    Then  the returned policy is the project-owned dialog override rather than the bundled default
    And  the project-owned override remains unchanged on disk
    And  the synchronized entry guidance directs authors to the effective policy rather than a bundled policy copy

  @e2e
  Scenario: Author starts a session with the universal dialog obligation
    Given  an initialized Givn project without an active change
    When  the author requests the normal Givn startup instructions through the Givn CLI
    Then  the startup guidance requires loading the effective dialog policy before the context's first user-facing question
    And  the startup guidance applies that obligation outside named skills and to delegated question preparation
    And  the startup guidance distinguishes loading once per context from rereading before every question
    And  the startup guidance still explains how to begin a change

  Scenario: Machine-readable startup guidance retains the dialog obligation while reporting an active change
    Given  an initialized Givn project with an active change awaiting specifications
    When  the author requests the normal Givn startup instructions as JSON
    Then  the response preserves the active change identity and next required artifact
    And  the response includes the same universal effective-dialog loading obligation in its guidance
    And  the response does not embed a second complete dialog policy
