Feature: Guide domain modeling

  Scenario: The skill trigger names conversation states
    Given the domain-modeling skill template is inspected
    When an agent reads the skill trigger text
    Then the trigger loads while naming or discussing domain concepts
    And the trigger loads during every workflow step of a change
    And the trigger loads when spec and code terms disagree

  Scenario: The skill body is an active discipline
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Challenge generic or overloaded terms"
    And  the served domain-modeling instruction contains "Name aggregates with domain nouns"
    And  the served domain-modeling instruction contains "the glossary inline"

  Scenario: Lifecycle guidance describes the domain, not the givn change
    Given  a Givn project
    Then  the served domain-modeling instruction contains "the *domain's own*"
    And  the served domain-modeling instruction contains "trip reset"
    And  the served domain-modeling instruction does not contain "created -> planned -> implemented -> archived"

  @e2e
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent synchronizes skills through the Givn CLI
    Then  the generated domain skill names the domain-modeling skill
    And  the generated skill loads during every workflow step of a change
    And  the generated givn-domain-modeling guidance directs the domain modeler to the domain-modeling instruction
    And  the served domain-modeling instruction contains "maintaining the project's arc42 glossary"
    And  the generated skill does not describe the givn change lifecycle

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
    Given  a Givn project
    Then  the served domain-modeling instruction contains "maintaining the project's arc42 glossary"
    And  the served domain-modeling instruction does not contain "domain.md"

  Scenario: The skill requires the arc42 glossary for domain modeling
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Requires the arc42 addon"
    And  the served domain-modeling instruction contains "enable it first when the glossary is missing"

  Scenario: The next-step guidance names the domain skill during later change phases
    Given a Givn project with the domain model addon enabled and a change awaiting review
    When the agent requests the next-step guidance through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  Scenario: A project without the arc42 glossary is not pointed at the domain skill
    Given a Givn project with the domain model addon enabled but no arc42 glossary and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance does not mention the domain-modeling skill

  Scenario: The design-review skill checks that the ubiquitous language is followed
    Given  a project root with givn/ initialised
    And  an existing change named "design-review-ubiquitous-language"
    Then  the served design-review instruction contains "ubiquitous language is used consistently"
    And  the served design-review instruction contains "arc42 glossary"

  Scenario: The review skill checks that the ubiquitous language is followed
    Given  a project root with givn/ initialised
    And  an existing change named "review-ubiquitous-language"
    Then  the served review instruction contains "ubiquitous language is used consistently everywhere"
    And  the served review instruction contains "arc42 glossary"

  Scenario: The domain-modeling guidance carries the term introduction test
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Genus + differentia"
    And  the served domain-modeling instruction contains "is a `<kind>` that"
    And  the served domain-modeling instruction contains "Contrast pair"
    And  the served domain-modeling instruction contains "not `<near neighbour>`"
    And  the served domain-modeling instruction contains "concrete domain sentence"
    And  the served domain-modeling instruction contains "Substrate"
    And  the served domain-modeling instruction contains "has no referent"

  Scenario: The challenge bullet hands the name choice to the user
    Given  a Givn project
    Then  the served domain-modeling instruction contains "do not choose it alone"
    And  the served domain-modeling instruction does not contain "Propose it and justify it"
    And  the served domain-modeling instruction contains "the concept in one sentence"
    And  the served domain-modeling instruction contains "up to three candidate names"
    And  the served domain-modeling instruction contains "any collision it carries"
    And  the served domain-modeling instruction contains "accept the user's own name"

  Scenario: The challenge bullet shows a naming question with candidates and a collision
    Given  a Givn project
    Then  the served domain-modeling instruction contains "ShoppingList"
    And  the served domain-modeling instruction contains "Candidates:"
    And  the served domain-modeling instruction contains "Basket"
    And  the served domain-modeling instruction contains "collides"
    And  the served domain-modeling instruction contains "should the record use, or what is yours"
