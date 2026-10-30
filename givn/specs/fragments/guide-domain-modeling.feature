Feature: Guide domain modeling

  Scenario: The skill trigger names conversation states
    Given the domain-modeling skill template is inspected
    When an agent reads the skill trigger text
    Then the trigger loads while naming or discussing domain concepts
    And the trigger loads during every workflow step of a change
    And the trigger loads when spec and code terms disagree

  @guide-domain-modeling
  Scenario: The skill body is an active discipline
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Challenge generic or overloaded terms"
    And  the served domain-modeling instruction contains "Name aggregates with domain nouns"
    And  the served domain-modeling instruction contains "the glossary inline"
    And  the served domain-modeling instruction contains "docs/glossary.md"

  Scenario: Lifecycle guidance describes the domain, not the givn change
    Given  a Givn project
    Then  the served domain-modeling instruction contains "the *domain's own*"
    And  the served domain-modeling instruction contains "trip reset"
    And  the served domain-modeling instruction does not contain "created -> planned -> implemented -> archived"

  @e2e @guide-domain-modeling
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given  a Givn project with the domain model addon enabled and a change awaiting specs
    When  the agent synchronizes skills through the Givn CLI
    Then  the generated domain skill names the domain-modeling skill
    And  the generated skill loads during every workflow step of a change
    And  the generated givn-domain-modeling guidance directs the domain modeler to the domain-modeling instruction
    And  the generated skill instructs maintaining the project's glossary
    And  the served domain-modeling instruction contains "maintaining the project's glossary"
    And  the generated skill does not describe the givn change lifecycle
    And  the terminal transcript is captured for this scenario

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

  Scenario: The next-step guidance names the domain skill during later change phases
    Given a Givn project with the domain model addon enabled and a change awaiting review
    When the agent requests the next-step guidance through the Givn CLI
    Then the guidance says to load the domain-modeling skill now

  @guide-domain-modeling
  Scenario: The design-review skill checks that the ubiquitous language is followed
    Given  a project root with givn/ initialised
    And  an existing change named "design-review-ubiquitous-language"
    Then  the served design-review instruction contains "ubiquitous language is used consistently"
    And  the served design-review instruction contains "docs/glossary.md"

  @guide-domain-modeling
  Scenario: The review skill checks that the ubiquitous language is followed
    Given  a project root with givn/ initialised
    And  an existing change named "review-ubiquitous-language"
    Then  the served review instruction contains "ubiquitous language is used consistently everywhere"
    And  the served review instruction contains "docs/glossary.md"

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

  @guide-domain-modeling
  Scenario: The domain model and glossary have separate owners
    Given  a Givn project
    Then  the served domain-modeling instruction contains "docs/domain-model.md"
    And  the served domain-modeling instruction contains "docs/glossary.md"
    And  the served domain-modeling instruction contains "Vocabulary belongs in the glossary"
    And  the served domain-modeling instruction contains "Structure belongs in the domain model"
    And  the served domain-modeling instruction contains "Behaviour belongs to use-case contracts and specifications"

  @guide-domain-modeling
  Scenario: Durable documents are reused instead of copied per change
    Given  a Givn project
    Then  the served domain-modeling instruction contains "never a per-change copy"
    And  the served domain-modeling instruction contains "reuse the existing model"
    And  the served domain-modeling instruction contains "create it only when it is absent"

  @guide-domain-modeling
  Scenario: Unknown structure stays qualified
    Given  a Givn project
    Then  the served domain-modeling instruction contains "established structure"
    And  the served domain-modeling instruction contains "proposed model"
    And  the served domain-modeling instruction contains "do not invent"
    And  the served domain-modeling instruction contains "unknown"

  @guide-domain-modeling
  Scenario: A rendered diagram is required evidence
    Given  a Givn project
    Then  the served domain-modeling instruction contains "rendered-diagram review"
    And  the served domain-modeling instruction contains "syntactically plausible Mermaid source is not evidence"
    And  the served domain-modeling instruction contains "individually traceable"

  @guide-domain-modeling
  Scenario: An existing arc42 glossary migrates without losing definitions
    Given  a Givn project
    Then  the served domain-modeling instruction contains "docs/arc42/12-glossary.md"
    And  the served domain-modeling instruction contains "preserve their wording and ordering"
    And  the served domain-modeling instruction contains "leave chapter 12 as a link"

  @guide-domain-modeling
  Scenario: The structural template keeps its notation legend separate
    Given  a Givn project
    Then  the served domain-modeling instruction contains "not part of the domain model"
    And  the served domain-modeling instruction contains "Collapsed consistency boundary"
    And  the served domain-modeling instruction contains "External system"
    And  the served domain-modeling instruction contains "layout: elk"
    And  the served domain-modeling instruction contains "flowchart TB"

  @guide-domain-modeling
  Scenario: The structural template does not require a lifecycle or event catalogue
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Do not add lifecycle, event, policy, or aggregate-detail sections by default"
    And  the served domain-modeling instruction contains "only when it provides a specific benefit"

  @guide-domain-modeling
  Scenario: The structural template provides the boundaries and relationships scaffold
    Given  a Givn project
    Then  the served domain-modeling instruction contains "## Boundaries and responsibilities"
    And  the served domain-modeling instruction contains "## Relationships"
    And  the served domain-modeling instruction contains "Governing contract or specification"
    And  the served domain-modeling instruction contains "## Open modeling questions"

  @guide-domain-modeling
  Scenario: Relationships link to their governing contract instead of copying rules
    Given  a Givn project
    Then  the served domain-modeling instruction contains "governing contract"
    And  the served domain-modeling instruction contains "rather than copy them"
    And  the served domain-modeling instruction contains "not a rulebook"

  @guide-domain-modeling
  Scenario: A project without arc42 is still pointed at the domain skill
    Given  a Givn project with the domain model addon enabled but no arc42 glossary and a change awaiting specs
    When  the agent requests the specs instructions through the Givn CLI
    Then  the guidance says to load the domain-modeling skill now
