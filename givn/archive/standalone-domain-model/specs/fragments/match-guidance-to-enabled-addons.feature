@givn.delta @match-guidance-to-enabled-addons

Feature: Match guidance to enabled addons

  @givn.removed
  Scenario Outline: A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed
    Given placeholder

  @givn.added
  Scenario: Disabling arc42 keeps the generated domain-modeling skill
    Given a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When a consumer disables the arc42 addon through the Givn CLI
    Then a generated domain-modeling skill exists
    And the served review instruction contains the Ubiquitous language conformance section

  @givn.added
  Scenario: Disabling the domain-modeling addon removes the generated domain-modeling skill
    Given a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When a consumer disables the domain-modeling addon through the Givn CLI
    Then no generated domain-modeling skill exists
    And the served review instruction omits the Ubiquitous language conformance section

  @givn.added
  Scenario: Enabling domain modeling without arc42 generates the domain-modeling skill
    Given a Givn project with the arc42 and domain-modeling addons disabled and the skills synchronized
    When a consumer enables the domain-modeling addon through the Givn CLI
    Then a generated domain-modeling skill exists
    And the served review instruction contains the Ubiquitous language conformance section
    And the served design-review instruction omits the Architecture documentation (arc42) branch
