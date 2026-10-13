@givn.delta @match-guidance-to-enabled-addons

Feature: Match guidance to enabled addons

  @givn.modified
  Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled
    Given a Givn project with the arc42 addon disabled and the skills synchronized
    Then the served review instruction omits the Arc42 implementation conformance section
    And the generated review skill carries no Arc42 implementation conformance section
    And the generated review command carries no Arc42 implementation conformance section
    And the generated review skill directs the reviewer to the review instruction

  @givn.modified
  Scenario: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled
    Given a Givn project with the arc42 addon disabled and the skills synchronized
    Then the served design-review instruction omits the Architecture documentation (arc42) branch
    And the generated design-review skill carries no Architecture documentation (arc42) branch
    And the generated design-review command carries no Architecture documentation (arc42) check

  @givn.modified @e2e
  Scenario: A consumer synchronizes skills and the generated guidance omits disabled-addon sections
    Given a Givn project with the arc42, domain-modeling, and coverage addons disabled and the skills synchronized
    When a consumer synchronizes skills through the Givn CLI
    Then the served review instruction omits the Arc42 implementation conformance section
    And the served review instruction omits the Ubiquitous language conformance section
    And the generated review skill carries no Arc42 implementation conformance section
    And the generated review skill carries no Ubiquitous language conformance section
    And no generated domain-modeling skill exists
    And no generated skill or command mentions any disabled addon

  @givn.modified @e2e @wip
  Scenario: A consumer enables the addons a generated section needs and the guidance gains it
    Given a Givn project with the arc42 and domain-modeling addons disabled and the skills synchronized
    When a consumer enables the arc42 and domain-modeling addons through the Givn CLI
    Then the served review instruction contains the Arc42 implementation conformance section
    And the served review instruction contains the Ubiquitous language conformance section
    And the served design-review instruction contains the Architecture documentation (arc42) branch
    And a generated domain-modeling skill exists
    And the generated review skill directs the reviewer to the review instruction
    And the generated review command directs the reviewer to the review instruction

  @givn.modified @e2e @wip
  Scenario Outline: A consumer disables an addon the domain-modeling skill needs and the generated domain skill is removed
    Given a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When a consumer disables the <addon> addon through the Givn CLI
    Then no generated domain-modeling skill exists
    And the served review instruction omits the Ubiquitous language conformance section
    Examples:
      | addon |
      | arc42 |
      | domain-modeling |
