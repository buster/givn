@givn.delta @match-guidance-to-enabled-addons
Feature: Match guidance to enabled addons

  @givn.modified
  Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled
    Given a Givn project with the arc42 addon disabled and the skills synchronized
    Then the served review instruction omits the Arc42 implementation conformance section
    And the generated review skill carries no Arc42 implementation conformance section
    And the generated review skill directs the reviewer to the review instruction
    And no generated review command exists

  @givn.modified
  Scenario: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled
    Given a Givn project with the arc42 addon disabled and the skills synchronized
    Then the served design-review instruction omits the Architecture documentation (arc42) branch
    And the generated design-review skill carries no Architecture documentation (arc42) branch
    And no generated design-review command exists

  @givn.removed
  Scenario: An unbalanced conditional in a command template fails the synchronization
    Given this scenario is removed

  @givn.modified @e2e
  Scenario: A consumer synchronizes skills and the generated guidance omits disabled-addon sections
    Given a Givn project with the arc42, domain-modeling, and coverage addons disabled and the skills synchronized
    When a consumer synchronizes skills through the Givn CLI
    Then the served review instruction omits the Arc42 implementation conformance section
    And the served review instruction omits the Ubiquitous language conformance section
    And the generated review skill carries no Arc42 implementation conformance section
    And the generated review skill carries no Ubiquitous language conformance section
    And no generated domain-modeling skill exists
    And no generated skill mentions any disabled addon
    And no Givn command files are generated
    And the terminal transcript is captured for this scenario

  @givn.modified @e2e
  Scenario: A consumer enables the addons a generated section needs and the guidance gains it
    Given a Givn project with the arc42 and domain-modeling addons disabled and the skills synchronized
    When a consumer enables the arc42 and domain-modeling addons through the Givn CLI
    Then the served review instruction contains the Arc42 implementation conformance section
    And the served review instruction contains the Ubiquitous language conformance section
    And the served design-review instruction contains the Architecture documentation (arc42) branch
    And a generated domain-modeling skill exists
    And the generated review skill directs the reviewer to the review instruction
    And no generated review command exists
    And the terminal transcript is captured for this scenario

  @givn.added
  Scenario: An unbalanced conditional in a skill template fails the synchronization
    Given a Givn project with a project skill template that contains an unbalanced conditional
    When a consumer synchronizes skills through the Givn CLI
    Then the synchronization fails
    And the failure names the template with the invalid directive

  @givn.added
  Scenario Outline: Project skill overrides retain conditional addon behavior without command generation
    Given a Givn project with a project-owned skill template containing a conditional on the arc42 addon
    And the arc42 addon is "<state>"
    When a consumer synchronizes skills through the Givn CLI
    Then the generated skill "<outcome>" the override's conditional Arc42 content
    And the project-owned skill template remains byte-for-byte unchanged
    And no Givn command files are generated

    Examples:
      | state    | outcome  |
      | enabled  | includes |
      | disabled | omits    |
