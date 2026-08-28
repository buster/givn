# User Interaction Inventory:
# - run `givn skills sync` to regenerate skills
# - request next-step guidance via `givn instructions`
# - request the specs instructions via `givn instructions specs --change <id>`
# - request the design instructions via `givn instructions design --change <id>`

@givn.delta @domain-skill-trigger

Feature: Domain-modeling skill is triggered and routed

  @givn.added
  Scenario: The skill trigger names conversation states
    Given the domain-modeling skill template is inspected
    When an agent reads the skill trigger text
    Then the trigger loads while naming or discussing domain concepts
    And the trigger loads before writing feature specs or the design
    And the trigger loads when spec and code terms disagree

  @givn.added
  Scenario: The skill body is an active discipline
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body challenges generic and overloaded terms
    And the body names aggregates with domain nouns
    And the body is updated inline as the vocabulary resolves

  @givn.added
  Scenario: Lifecycle guidance describes the domain, not the givn change
    Given the domain-modeling skill template is inspected
    When an agent reads the lifecycle guidance
    Then the lifecycle describes the domain's own observable states
    And the lifecycle does not describe the givn change lifecycle

  @givn.added
  Scenario: The skill documents a per-change ubiquitous-language table
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body requires a per-change ubiquitous-language table
    And the body records rejected synonyms as anti-terms

  @givn.added @e2e
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent synchronizes skills through the Givn CLI
    Then the generated domain skill names the domain-modeling skill
    And the generated skill loads before writing feature specs or the design
    And the generated skill does not describe the givn change lifecycle

  @givn.added @e2e
  Scenario: An agent requests next-step guidance and is routed to the domain skill
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the next-step guidance through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  @givn.added
  Scenario: An agent requests next-step guidance as JSON and the skill route is machine-readable
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the next-step guidance as JSON through the Givn CLI
    Then the JSON response names the domain-modeling skill as the skill to load

  @givn.added @e2e
  Scenario: An agent requests the specs guidance and is routed to the domain skill
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  @givn.added @e2e
  Scenario: An agent requests the design guidance and is routed to the domain skill
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the design instructions through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  @givn.added
  Scenario: A project without the addon is not routed to the domain skill
    Given a Givn project with the domain model addon disabled and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance does not mention the domain-modeling skill
