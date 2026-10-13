@givn.delta @guide-domain-modeling

Feature: Guide domain modeling

  @givn.modified
  Scenario: The skill body is an active discipline
    Given a Givn project
    Then the served domain-modeling instruction contains "Challenge generic or overloaded terms"
    And the served domain-modeling instruction contains "Name aggregates with domain nouns"
    And the served domain-modeling instruction contains "the glossary inline"

  @givn.modified
  Scenario: Lifecycle guidance describes the domain, not the givn change
    Given a Givn project
    Then the served domain-modeling instruction contains "the *domain's own*"
    And the served domain-modeling instruction contains "trip reset"
    And the served domain-modeling instruction does not contain "created -> planned -> implemented -> archived"

  @givn.modified
  Scenario: The ubiquitous language is recorded in the project's arc42 glossary
    Given a Givn project
    Then the served domain-modeling instruction contains "maintaining the project's arc42 glossary"
    And the served domain-modeling instruction does not contain "domain.md"

  @givn.modified @wip
  Scenario: The skill requires the arc42 glossary for domain modeling
    Given a Givn project
    Then the served domain-modeling instruction contains "Requires the arc42 addon"
    And the served domain-modeling instruction contains "enable it first when the glossary is missing"

  @givn.modified @wip
  Scenario: The domain-modeling guidance carries the term introduction test
    Given a Givn project
    Then the served domain-modeling instruction contains "Genus + differentia"
    And the served domain-modeling instruction contains "is a `<kind>` that"
    And the served domain-modeling instruction contains "Contrast pair"
    And the served domain-modeling instruction contains "not `<near neighbour>`"
    And the served domain-modeling instruction contains "concrete domain sentence"
    And the served domain-modeling instruction contains "Substrate"
    And the served domain-modeling instruction contains "has no referent"

  @givn.modified @wip
  Scenario: The challenge bullet hands the name choice to the user
    Given a Givn project
    Then the served domain-modeling instruction contains "do not choose it alone"
    And the served domain-modeling instruction does not contain "Propose it and justify it"
    And the served domain-modeling instruction contains "the concept in one sentence"
    And the served domain-modeling instruction contains "up to three candidate names"
    And the served domain-modeling instruction contains "any collision it carries"
    And the served domain-modeling instruction contains "accept the user's own name"

  @givn.modified @wip
  Scenario: The challenge bullet shows a naming question with candidates and a collision
    Given a Givn project
    Then the served domain-modeling instruction contains "ShoppingList"
    And the served domain-modeling instruction contains "Candidates:"
    And the served domain-modeling instruction contains "Basket"
    And the served domain-modeling instruction contains "collides"
    And the served domain-modeling instruction contains "should the record use, or what is yours"

  @givn.modified
  Scenario: The design-review skill checks that the ubiquitous language is followed
    Given a project root with givn/ initialised
    And an existing change named "design-review-ubiquitous-language"
    Then the served design-review instruction contains "ubiquitous language is used consistently"
    And the served design-review instruction contains "arc42 glossary"

  @givn.modified
  Scenario: The review skill checks that the ubiquitous language is followed
    Given a project root with givn/ initialised
    And an existing change named "review-ubiquitous-language"
    Then the served review instruction contains "ubiquitous language is used consistently everywhere"
    And the served review instruction contains "arc42 glossary"
