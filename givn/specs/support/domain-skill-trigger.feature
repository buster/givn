Feature: Domain-modeling skill is triggered and routed

  Scenario: The skill trigger names conversation states
    Given the domain-modeling skill template is inspected
    When an agent reads the skill trigger text
    Then the trigger loads while naming or discussing domain concepts
    And the trigger loads during every workflow step of a change
    And the trigger loads when spec and code terms disagree

  Scenario: The skill body is an active discipline
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body challenges generic and overloaded terms
    And the body names aggregates with domain nouns
    And the body is updated inline as the vocabulary resolves

  Scenario: Lifecycle guidance describes the domain, not the givn change
    Given the domain-modeling skill template is inspected
    When an agent reads the lifecycle guidance
    Then the lifecycle describes the domain's own observable states
    And the lifecycle does not describe the givn change lifecycle

  @e2e
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent synchronizes skills through the Givn CLI
    Then the generated domain skill names the domain-modeling skill
    And the generated skill loads during every workflow step of a change
    And the generated skill instructs maintaining the project's arc42 glossary
    And the generated skill does not describe the givn change lifecycle

  @e2e
  Scenario: An agent requests next-step guidance and is routed to the domain skill
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the next-step guidance through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  Scenario: An agent requests next-step guidance as JSON and the skill route is machine-readable
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the next-step guidance as JSON through the Givn CLI
    Then the JSON response names the domain-modeling skill as the skill to load

  @e2e
  Scenario: An agent requests the specs guidance and is routed to the domain skill
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  @e2e
  Scenario: An agent requests the design guidance and is routed to the domain skill
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent requests the design instructions through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  Scenario: A project without the addon is not routed to the domain skill
    Given a Givn project with the domain model addon disabled and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance does not mention the domain-modeling skill
  Scenario: The ubiquitous language is recorded in the project's arc42 glossary
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body instructs maintaining the project's arc42 glossary
    And the body does not require a per-change domain artifact

  Scenario: The skill requires the arc42 glossary for domain modeling
    Given the domain-modeling skill template is inspected
    When an agent reads the skill body
    Then the body instructs enabling the arc42 addon first when the glossary is missing

  Scenario: The next-step guidance names the domain skill during later change phases
    Given a Givn project with the domain model addon enabled and a change awaiting review
    When the agent requests the next-step guidance through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  Scenario: A project without the arc42 glossary is not pointed at the domain skill
    Given a Givn project with the domain model addon enabled but no arc42 glossary and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance does not mention the domain-modeling skill

  Scenario: The design-review skill checks that the ubiquitous language is followed
    Given the design-review skill template is inspected
    When an agent reads the check list
    Then a check requires confirming the ubiquitous language is used consistently and recorded in the arc42 glossary

  Scenario: The review skill checks that the ubiquitous language is followed
    Given the review skill template is inspected
    When an agent reads the check list
    Then a check requires confirming the ubiquitous language is used consistently everywhere and recorded in the arc42 glossary
