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
    Given the design instructions are requested
    When an author prepares the technical design
    Then the guidance requires the E2E runner and its tag isolation
    And the guidance requires the step-definition location and driving mechanism
    And the guidance requires local infrastructure and framework decisions
    And the guidance requires the interaction coverage matrix

  @e2e
  Scenario: An author requests the technical E2E design guidance
    Given a Givn project with an active change
    When the author requests the design instructions through the Givn CLI
    Then the CLI output contains every required E2E design decision

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
    Given retrieval workflow guidance is available
    When an author compares retrieval-capable and feature-free guidance
    Then the guidance explains the advisory and blocking retrieval paths
    And the guidance preserves an explicit unavailable result for feature-free builds
    And the guidance does not prohibit retrieval in a retrieval-capable build
  Scenario: Review guidance names the deterministic disposition gates
    Given the review instructions are requested
    When the reviewer reads the deterministic gate guidance
    Then the guidance requires an overlap disposition for every shape match
    And the guidance requires a split-or-keep decision for every long scenario

  @e2e
  Scenario: A reviewer requests disposition-aware review guidance
    Given a Givn project with an active change
    When the reviewer requests the review instructions through the Givn CLI
    Then the CLI output contains the deterministic gate dispositions guidance
