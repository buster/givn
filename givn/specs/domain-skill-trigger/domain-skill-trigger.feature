Feature: Domain-modeling skill is triggered and routed

  Scenario: The skill trigger names conversation states
    Given  the domain-modeling skill template is inspected
    When  an agent reads the skill trigger text
    Then  the trigger loads while naming or discussing domain concepts
    And  the trigger loads before writing feature specs or the design
    And  the trigger loads when spec and code terms disagree

  Scenario: The skill body is an active discipline
    Given  the domain-modeling skill template is inspected
    When  an agent reads the skill body
    Then  the body challenges generic and overloaded terms
    And  the body names aggregates with domain nouns
    And  the body is updated inline as the vocabulary resolves

  Scenario: Lifecycle guidance describes the domain, not the givn change
    Given  the domain-modeling skill template is inspected
    When  an agent reads the lifecycle guidance
    Then  the lifecycle describes the domain's own observable states
    And  the lifecycle does not describe the givn change lifecycle

  Scenario: The skill documents a per-change ubiquitous-language table
    Given  the domain-modeling skill template is inspected
    When  an agent reads the skill body
    Then  the body requires a per-change ubiquitous-language table
    And  the body records rejected synonyms as anti-terms

  @e2e
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent synchronizes skills through the Givn CLI
    Then  the generated domain skill names the domain-modeling skill
    And  the generated skill loads before writing feature specs or the design
    And  the generated skill does not describe the givn change lifecycle

  @e2e
  Scenario: An agent requests next-step guidance and is routed to the domain skill
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent requests the next-step guidance through the Givn CLI
    Then  the guidance says to load the domain-modeling skill now

  Scenario: An agent requests next-step guidance as JSON and the skill route is machine-readable
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent requests the next-step guidance as JSON through the Givn CLI
    Then  the JSON response names the domain-modeling skill as the skill to load

  @e2e
  Scenario: An agent requests the specs guidance and is routed to the domain skill
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent requests the specs instructions through the Givn CLI
    Then  the guidance says to load the domain-modeling skill now

  @e2e
  Scenario: An agent requests the design guidance and is routed to the domain skill
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent requests the design instructions through the Givn CLI
    Then  the guidance says to load the domain-modeling skill now

  Scenario: A project without the addon is not routed to the domain skill
    Given  a Givn project with the domain model addon disabled and a change awaiting specs
    When  the agent requests the specs instructions through the Givn CLI
    Then  the guidance does not mention the domain-modeling skill
