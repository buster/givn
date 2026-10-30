@givn.delta @guide-domain-modeling

Feature: Guide domain modeling

  @givn.modified
  Scenario: The skill body is an active discipline
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Challenge generic or overloaded terms"
    And  the served domain-modeling instruction contains "Name aggregates with domain nouns"
    And  the served domain-modeling instruction contains "the glossary inline"
    And  the served domain-modeling instruction contains "docs/glossary.md"

  @givn.modified @e2e
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

  @givn.modified
  Scenario: The design-review skill checks that the ubiquitous language is followed
    Given  a project root with givn/ initialised
    And  an existing change named "design-review-ubiquitous-language"
    Then  the served design-review instruction contains "ubiquitous language is used consistently"
    And  the served design-review instruction contains "docs/glossary.md"

  @givn.modified
  Scenario: The review skill checks that the ubiquitous language is followed
    Given  a project root with givn/ initialised
    And  an existing change named "review-ubiquitous-language"
    Then  the served review instruction contains "ubiquitous language is used consistently everywhere"
    And  the served review instruction contains "docs/glossary.md"

  @givn.removed
  Scenario: The ubiquitous language is recorded in the project's arc42 glossary
    Given placeholder

  @givn.removed
  Scenario: The skill requires the arc42 glossary for domain modeling
    Given placeholder

  @givn.removed
  Scenario: A project without the arc42 glossary is not pointed at the domain skill
    Given placeholder

  @givn.added
  Scenario: The domain model and glossary have separate owners
    Given  a Givn project
    Then  the served domain-modeling instruction contains "docs/domain-model.md"
    And  the served domain-modeling instruction contains "docs/glossary.md"
    And  the served domain-modeling instruction contains "Vocabulary belongs in the glossary"
    And  the served domain-modeling instruction contains "Structure belongs in the domain model"
    And  the served domain-modeling instruction contains "Behaviour belongs to use-case contracts and specifications"

  @givn.added
  Scenario: Durable documents are reused instead of copied per change
    Given  a Givn project
    Then  the served domain-modeling instruction contains "never a per-change copy"
    And  the served domain-modeling instruction contains "reuse the existing model"
    And  the served domain-modeling instruction contains "create it only when it is absent"

  @givn.added
  Scenario: Unknown structure stays qualified
    Given  a Givn project
    Then  the served domain-modeling instruction contains "established structure"
    And  the served domain-modeling instruction contains "proposed model"
    And  the served domain-modeling instruction contains "do not invent"
    And  the served domain-modeling instruction contains "unknown"

  @givn.added
  Scenario: A rendered diagram is required evidence
    Given  a Givn project
    Then  the served domain-modeling instruction contains "rendered-diagram review"
    And  the served domain-modeling instruction contains "syntactically plausible Mermaid source is not evidence"
    And  the served domain-modeling instruction contains "individually traceable"

  @givn.added
  Scenario: An existing arc42 glossary migrates without losing definitions
    Given  a Givn project
    Then  the served domain-modeling instruction contains "docs/arc42/12-glossary.md"
    And  the served domain-modeling instruction contains "preserve their wording and ordering"
    And  the served domain-modeling instruction contains "leave chapter 12 as a link"

  @givn.added
  Scenario: The structural template keeps its notation legend separate
    Given  a Givn project
    Then  the served domain-modeling instruction contains "not part of the domain model"
    And  the served domain-modeling instruction contains "Collapsed consistency boundary"
    And  the served domain-modeling instruction contains "External system"
    And  the served domain-modeling instruction contains "layout: elk"
    And  the served domain-modeling instruction contains "flowchart TB"

  @givn.added
  Scenario: The structural template does not require a lifecycle or event catalogue
    Given  a Givn project
    Then  the served domain-modeling instruction contains "Do not add lifecycle, event, policy, or aggregate-detail sections by default"
    And  the served domain-modeling instruction contains "only when it provides a specific benefit"

  @givn.added
  Scenario: The structural template provides the boundaries and relationships scaffold
    Given  a Givn project
    Then  the served domain-modeling instruction contains "## Boundaries and responsibilities"
    And  the served domain-modeling instruction contains "## Relationships"
    And  the served domain-modeling instruction contains "Governing contract or specification"
    And  the served domain-modeling instruction contains "## Open modeling questions"

  @givn.added
  Scenario: Relationships link to their governing contract instead of copying rules
    Given  a Givn project
    Then  the served domain-modeling instruction contains "governing contract"
    And  the served domain-modeling instruction contains "rather than copy them"
    And  the served domain-modeling instruction contains "not a rulebook"

  @givn.added
  Scenario: A project without arc42 is still pointed at the domain skill
    Given a Givn project with the domain model addon enabled but no arc42 glossary and a change awaiting specs
    When the agent requests the specs instructions through the Givn CLI
    Then the guidance says to load the domain-modeling skill now
