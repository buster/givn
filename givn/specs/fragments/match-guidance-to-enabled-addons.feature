Feature: Match guidance to enabled addons

  @match-guidance-to-enabled-addons
  Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled
    Given  a Givn project with the arc42 addon disabled and the skills synchronized
    Then  the served review instruction omits the Arc42 implementation conformance section
    And  the generated review skill carries no Arc42 implementation conformance section
    And  the generated review skill directs the reviewer to the review instruction
    And  no generated review command exists

  @match-guidance-to-enabled-addons
  Scenario: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled
    Given  a Givn project with the arc42 addon disabled and the skills synchronized
    Then  the served design-review instruction omits the Architecture documentation (arc42) branch
    And  the generated design-review skill carries no Architecture documentation (arc42) branch
    And  no generated design-review command exists

  Scenario: Architecture principles guidance is off by default
    Given  a Givn project with the architecture-principles addon disabled and the skills synchronized
    Then  the served design instruction omits the Contracts and components section
    And  the served design-review instruction omits the Contracts and components branch

  Scenario: The architecture-principles instruction keeps the decision-timing rule
    Given  a Givn project with the architecture-principles addon enabled and the skills synchronized
    Then  the served architecture-principles instruction contains "last responsible moment"
    And  the served architecture-principles instruction contains "Out of scope"

  Scenario: An unknown addon name in a conditional fails the synchronization and names the template
    Given a Givn project with a project skill template that contains a conditional on an unknown addon
    When a consumer synchronizes skills through the Givn CLI
    Then the synchronization fails
    And the failure names the template with the invalid directive

  @e2e @match-guidance-to-enabled-addons
  Scenario: A consumer synchronizes skills and the generated guidance omits disabled-addon sections
    Given  a Givn project with the arc42, domain-modeling, and coverage addons disabled and the skills synchronized
    When  a consumer synchronizes skills through the Givn CLI
    Then  the served review instruction omits the Arc42 implementation conformance section
    And  the served review instruction omits the Ubiquitous language conformance section
    And  the generated review skill carries no Arc42 implementation conformance section
    And  the generated review skill carries no Ubiquitous language conformance section
    And  no generated domain-modeling skill exists
    And  no generated skill mentions any disabled addon
    And  no Givn command files are generated
    And  the terminal transcript is captured for this scenario

  @e2e @match-guidance-to-enabled-addons
  Scenario: A consumer enables the addons a generated section needs and the guidance gains it
    Given  a Givn project with the arc42 and domain-modeling addons disabled and the skills synchronized
    When  a consumer enables the arc42 and domain-modeling addons through the Givn CLI
    Then  the served review instruction contains the Arc42 implementation conformance section
    And  the served review instruction contains the Ubiquitous language conformance section
    And  the served design-review instruction contains the Architecture documentation (arc42) branch
    And  a generated domain-modeling skill exists
    And  the generated review skill directs the reviewer to the review instruction
    And  no generated review command exists
    And  the terminal transcript is captured for this scenario

  @match-guidance-to-enabled-addons
  Scenario: An unbalanced conditional in a skill template fails the synchronization
    Given  a Givn project with a project skill template that contains an unbalanced conditional
    When  a consumer synchronizes skills through the Givn CLI
    Then  the synchronization fails
    And  the failure names the template with the invalid directive

  @match-guidance-to-enabled-addons
  Scenario Outline: Project skill overrides retain conditional addon behavior without command generation
    Given  a Givn project with a project-owned skill template containing a conditional on the arc42 addon
    And  the arc42 addon is "<state>"
    When  a consumer synchronizes skills through the Givn CLI
    Then  the generated skill "<outcome>" the override's conditional Arc42 content
    And  the project-owned skill template remains byte-for-byte unchanged
    And  no Givn command files are generated
    Examples:
      | state | outcome |
      | enabled | includes |
      | disabled | omits |

  @match-guidance-to-enabled-addons
  Scenario: Disabling arc42 keeps the generated domain-modeling skill
    Given  a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When  a consumer disables the arc42 addon through the Givn CLI
    Then  a generated domain-modeling skill exists
    And  the served review instruction contains the Ubiquitous language conformance section

  @match-guidance-to-enabled-addons
  Scenario: Disabling the domain-modeling addon removes the generated domain-modeling skill
    Given  a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When  a consumer disables the domain-modeling addon through the Givn CLI
    Then  no generated domain-modeling skill exists
    And  the served review instruction omits the Ubiquitous language conformance section

  @match-guidance-to-enabled-addons
  Scenario: Enabling domain modeling without arc42 generates the domain-modeling skill
    Given  a Givn project with the arc42 and domain-modeling addons disabled and the skills synchronized
    When  a consumer enables the domain-modeling addon through the Givn CLI
    Then  a generated domain-modeling skill exists
    And  the served review instruction contains the Ubiquitous language conformance section
    And  the served design-review instruction omits the Architecture documentation (arc42) branch

  @match-guidance-to-enabled-addons
  Scenario: Architecture principles guidance follows an enabled addon
    Given  a Givn project with the architecture-principles addon disabled and the skills synchronized
    When  a consumer enables the architecture-principles addon through the Givn CLI
    Then  the served design instruction contains the Contracts and components section
    And  the served design-review instruction contains the Contracts and components branch

  @match-guidance-to-enabled-addons
  Scenario Outline: Outline guidance follows an enabled addon
    Given  a Givn project with the outline addon <state> and the skills synchronized
    Then  the served steps instruction <outcome> the advisory outline section
    And  the served review instruction <outcome> the advisory outline section
    And  no generated skill carries outline guidance
    Examples:
      | state | outcome |
      | enabled | contains |
      | disabled | omits |
