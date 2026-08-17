# User Interaction Inventory:
# - request the canonical interaction and E2E authoring guidance, including retrieval capability state
# - request the technical design guidance for E2E setup
# - complete a semantic review with class-specific remediation evidence
# - request machine-readable artifact guidance

@givn.delta @instruction-integrity
Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: Canonical E2E guidance rejects weaker interface evidence
    Given the canonical specs guidance is requested
    When an author reads the interaction and E2E policy
    Then the guidance requires primary assertions on the named real interface
    And the guidance rejects repository-only evidence for a higher-level interface
    And the guidance requires a real browser for browser UI interactions
    And the guidance treats an unresolved interface obstacle as a design gap

  @givn.added @e2e @wip
  Scenario: An author requests the canonical interaction and E2E guidance
    Given a Givn project with an active change
    When the author requests the specs instructions through the Givn CLI
    Then the CLI output contains the complete interaction and E2E policy
    And the CLI output distinguishes available and unavailable retrieval behavior

  @givn.added
  Scenario: Design guidance requires the complete E2E setup record
    Given the design instructions are requested
    When an author prepares the technical design
    Then the guidance requires the E2E runner and its tag isolation
    And the guidance requires the step-definition location and driving mechanism
    And the guidance requires local infrastructure and framework decisions
    And the guidance requires the interaction coverage matrix

  @givn.added @e2e @wip
  Scenario: An author requests the technical E2E design guidance
    Given a Givn project with an active change
    When the author requests the design instructions through the Givn CLI
    Then the CLI output contains every required E2E design decision

  @givn.added
  Scenario: Review guidance verifies semantic remediation
    Given a semantic review contains classified candidates
    When the reviewer completes the review artifact
    Then the review records the remediation contract for each classification
    And the review records the projected-tree evidence and verification receipt
    And an unresolved or stale remediation remains blocking

  @givn.added @e2e @wip
  Scenario: A reviewer requests remediation-aware review guidance
    Given a Givn project with an active change
    When the reviewer requests the review instructions through the Givn CLI
    Then the CLI output contains semantic classification and remediation checks

  @givn.added
  Scenario: Machine-readable instructions describe the effective artifact
    Given the effective specs artifact metadata is available
    When an agent inspects the specs instructions as JSON
    Then the response identifies the artifact
    And the response lists the generated paths and prerequisites
    And the response contains the resolved instruction text

  @givn.added @wip
  Scenario: Artifact instructions reject an unknown change context
    Given the effective specs artifact metadata is available
    When an agent requests the specs instructions for an unknown change context
    Then the Givn CLI rejects the request with a clear change-not-found error

  @givn.added @e2e @wip
  Scenario: An agent requests machine-readable artifact guidance
    Given a Givn project with an active change
    When the agent requests the specs instructions as JSON through the Givn CLI
    Then the response is valid JSON with artifact paths prerequisites and instructions

  @givn.added @wip
  Scenario: Retrieval guidance distinguishes build capabilities
    Given retrieval workflow guidance is available
    When an author compares retrieval-capable and feature-free guidance
    Then the guidance explains the advisory and blocking retrieval paths
    And the guidance preserves an explicit unavailable result for feature-free builds
    And the guidance does not prohibit retrieval in a retrieval-capable build

  @givn.added @wip
  Scenario: An author requests retrieval-aware workflow guidance
    Given a Givn project with an active change
    When the author requests the specs instructions through the Givn CLI
    Then the CLI output distinguishes available and unavailable retrieval behavior
