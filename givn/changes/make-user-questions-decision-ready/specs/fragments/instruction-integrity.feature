@givn.delta @instruction-integrity
Feature: Decision-ready dialog instructions at every user-question boundary

  # Interaction inventory: specs/fragments/fragment.md owns this reusable fragment.
  # Dialog retrieval is an input variant of the existing instruction-request action, not a second E2E action.
  # Start a session -> Author starts a session with the universal dialog obligation
  # Specs JSON is the retained request-by-id E2E; design, review and dialog are regular variants.
  # Redundant design/review E2E identities are removed explicitly below, not retagged in place.

  @givn.modified @wip
  Scenario: The canonical dialog policy is retrievable through the Givn CLI
    Given a Givn project with an active change for the dialog policy
    When the author requests the dialog policy through the Givn CLI
    Then the returned policy applies to every user-facing question inside and outside a named workflow
    And the returned policy assumes project familiarity without assuming knowledge of every referenced artifact
    And the returned policy requires the decision subject brief sourced context and concrete consequences for each answer
    And the returned policy requires material later-change costs and the benefit purchased by difficult commitments
    And the returned policy directs the author to settle discoverable facts and confirmed constraints before asking
    And the returned policy distinguishes authoring obligations from runtime enforcement

  @givn.modified @wip
  Scenario: The canonical dialog policy demonstrates the full question record
    Given an initialized Givn project
    When the author requests the dialog policy through the Givn CLI
    Then the decision-record guidance keeps the complete context alternatives consequences and resolution
    And the chosen and rejected alternatives retain their reasons after the question closes
    And decision-relevant cross-references carry their meaning rather than only their identifier
    And routine information requests are not forced into invented alternatives or tradeoffs
    And confirmations disclose both acceptance and revision consequences without treating consent as a rule waiver

  @givn.added @wip
  Scenario: Dialog teaching contrasts show complete decisions rather than bare labels
    Given an initialized Givn project
    When the author requests the dialog policy through the Givn CLI
    Then the returned teaching examples identify their records as hypothetical rather than current product behavior
    And the goal-confirmation contrast displays every proposed goal and bounds what acceptance authorizes
    And the clarification contrast asks only the missing human intent and shows how the answer scopes subsequent work
    And the behavior-choice contrast shows the observable outcome of both answers rather than implementation labels
    And the commitment contrast shows the saved-information benefit compatibility restriction and later-change work
    And the diagnostic contrast reports a researched assertion defect rather than asking the person to inspect unseen scenarios
    And every good contrast links to its own supplied decision record
    And the diagnostic contrast excludes an answer that violates the confirmed one-answer constraint

  @givn.added @wip
  Scenario: Migration consent does not imply an unverified recovery guarantee
    Given an initialized Givn project
    When the author reads the material-commitment teaching contrast from the returned dialog instructions
    Then the contrast distinguishes required recovery verification from already implemented recovery support
    And the less-committing answer leaves saved projects unchanged rather than promising a reversible upgrade
    And the policy requires unknown compatibility facts to be established or consent narrowed before rollout approval

  @givn.added @wip
  Scenario: A project dialog override remains the effective policy after synchronization
    Given an initialized Givn project with a project-owned dialog instruction override
    And the override requires human approval before broadening the selected project scope
    When the author synchronizes guidance and requests the dialog instructions through the Givn CLI
    Then the returned policy is the project-owned dialog override rather than the bundled default
    And the project-owned override remains unchanged on disk
    And the synchronized entry guidance directs authors to the effective policy rather than a bundled policy copy

  @givn.added @e2e @wip
  Scenario: Author starts a session with the universal dialog obligation
    Given an initialized Givn project without an active change
    When the author requests the normal Givn startup instructions through the Givn CLI
    Then the startup guidance requires loading the effective dialog policy before the context's first user-facing question
    And the startup guidance applies that obligation outside named skills and to delegated question preparation
    And the startup guidance distinguishes loading once per context from rereading before every question
    And the startup guidance still explains how to begin a change

  @givn.added @wip
  Scenario: Machine-readable startup guidance retains the dialog obligation while reporting an active change
    Given an initialized Givn project with an active change awaiting specifications
    When the author requests the normal Givn startup instructions as JSON
    Then the response preserves the active change identity and next required artifact
    And the response includes the same universal effective-dialog loading obligation in its guidance
    And the response does not embed a second complete dialog policy

  @givn.modified @wip
  Scenario: Design guidance requires the complete E2E setup record
    Given a Givn project with an active change
    When the author requests the design instructions through the Givn CLI
    Then the returned design guidance requires the runner scope strictness proof separate bindings real driver and local infrastructure
    And the returned design guidance requires framework justification and a matching interaction coverage matrix
    And the returned design guidance distinguishes terminal transcripts from Web UI viewport evidence

  @givn.removed @e2e @wip
  Scenario: An author requests the technical E2E design guidance
    Given the design-input contract is preserved by the regular design-guidance variant of request-by-id

  @givn.modified @wip
  Scenario: Review guidance names the deterministic disposition gates
    Given a Givn project with an active change
    When the reviewer requests the review instructions through the Givn CLI
    Then the returned review guidance requires a disposition for every reported overlap pair with a reason
    And the returned review guidance requires a split-or-keep decision for every reported long scenario
    And the returned review guidance distinguishes an advisory finding from completed verification evidence

  @givn.removed @e2e @wip
  Scenario: A reviewer requests disposition-aware review guidance
    Given the review-input contract is preserved by the regular review-guidance variant of request-by-id

  @givn.modified @wip
  Scenario: The dialog policy is retrievable without an active change
    Given a project root with givn/ initialised
    When the author requests the dialog policy through the Givn CLI without a change
    Then the command exits 0
    And the returned general policy governs question preparation without requiring a selected change
    And the returned policy distinguishes routine clarification from decisions that require alternatives and consequences
