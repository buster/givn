# User Interaction Inventory:
# - run `givn skills sync` to regenerate skills
# - request next-step guidance via `givn instructions`
# - request the specs instructions via `givn instructions specs --change <id>`
# - request the design instructions via `givn instructions design --change <id>`

@givn.delta @domain-skill-trigger

Feature: Domain-modeling skill is triggered and routed

  @givn.modified
  Scenario: The skill trigger names conversation states
    Given  the domain-modeling skill template is inspected
    When  an agent reads the skill trigger text
    Then  the trigger loads while naming or discussing domain concepts
    And  the trigger loads during every workflow step of a change
    And  the trigger loads when spec and code terms disagree

  @givn.added
  Scenario: The ubiquitous language is recorded in the project's arc42 glossary
    Given  the domain-modeling skill template is inspected
    When  an agent reads the skill body
    Then  the body instructs maintaining the project's arc42 glossary
    And  the body does not require a per-change domain artifact

  @givn.added
  Scenario: The skill requires the arc42 glossary for domain modeling
    Given  the domain-modeling skill template is inspected
    When  an agent reads the skill body
    Then  the body instructs enabling the arc42 addon first when the glossary is missing

  @givn.added
  Scenario: The next-step guidance names the domain skill during later change phases
    Given  a Givn project with the domain model addon enabled and a change awaiting review
    When  the agent requests the next-step guidance through the Givn CLI
    Then  the guidance says to load the domain-modeling skill now

  @givn.added
  Scenario: A project without the arc42 glossary is not pointed at the domain skill
    Given  a Givn project with the domain model addon enabled but no arc42 glossary and a change awaiting specs
    When  the agent requests the specs instructions through the Givn CLI
    Then  the guidance does not mention the domain-modeling skill

  @givn.added
  Scenario: The design-review skill checks that the ubiquitous language is followed
    Given  the design-review skill template is inspected
    When  an agent reads the check list
    Then  a check requires confirming the ubiquitous language is used consistently and recorded in the arc42 glossary

  @givn.added
  Scenario: The review skill checks that the ubiquitous language is followed
    Given  the review skill template is inspected
    When  an agent reads the check list
    Then  a check requires confirming the ubiquitous language is used consistently everywhere and recorded in the arc42 glossary

  @givn.modified @e2e
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent synchronizes skills through the Givn CLI
    Then  the generated domain skill names the domain-modeling skill
    And  the generated skill loads during every workflow step of a change
    And  the generated skill instructs maintaining the project's arc42 glossary
    And  the generated skill does not describe the givn change lifecycle

  @givn.removed
  Scenario: The skill documents a per-change ubiquitous-language table
    Given  a placeholder removed scenario
