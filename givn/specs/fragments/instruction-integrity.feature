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
    Given  the synchronized agent guidance for the specs workflow
    When  an author reads the guidance
    Then  the guidance does not mention engine-backed retrieval commands
    And  the guidance does not offer advisory or blocking retrieval paths
    And  the guidance keeps the deterministic spec toolchain wording

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
  Scenario: The canonical dialog policy is retrievable through the Givn CLI
    Given  a Givn project with an active change for the dialog policy
    When  the author requests the dialog policy through the Givn CLI
    Then  the CLI output contains the named-artifact rule
    And  the CLI output contains the option treatment rule
    And  the CLI output contains the standalone-answerability rule
    And  the CLI output contains the rejected-alternative rule

  Scenario: The canonical dialog policy demonstrates the full question record
    Given  the canonical dialog policy is requested
    When  an author reads the option treatment and the record shape
    Then  the policy requires every option's meaning gain cost and concrete example
    And  the policy requires the rejected alternatives with their reasons
    And  the policy rejects an outcome-only resolution
    And  the policy requires a gloss for every cross-reference

  Scenario: The dialog policy is retrievable without an active change
    Given  a project root with givn/ initialised
    When  the author requests the dialog policy through the Givn CLI without a change
    Then  the command exits 0
    And  the CLI output contains the named-artifact rule

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
