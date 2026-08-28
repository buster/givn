# User Interaction Inventory:
# - synchronize generated skills and commands through the Givn CLI
# - enable an addon through the Givn CLI
# - disable an addon through the Givn CLI

@givn.delta @template-conditional-rendering

Feature: Generated guidance matches the enabled addon configuration

  Scenario: Review guidance omits the Arc42 section when the Arc42 addon is disabled
    Given  a Givn project with the arc42 addon disabled and the skills synchronized
    Then  the generated review skill contains no Arc42 implementation conformance section
    And  the generated review command contains no Arc42 implementation conformance section

  Scenario: Review guidance includes the Arc42 section when the Arc42 addon is enabled
    Given  a Givn project with the arc42 addon enabled and the skills synchronized
    Then  the generated review skill contains the Arc42 implementation conformance section
    And  the generated review command contains the Arc42 implementation conformance section

  Scenario: Review guidance omits the ubiquitous-language check when the domain-modeling addon is disabled
    Given  a Givn project with the domain-modeling addon disabled and the skills synchronized
    Then  the generated review skill contains no Ubiquitous language conformance section

  Scenario: Review guidance includes the ubiquitous-language check when both addons it needs are enabled
    Given  a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    Then  the generated review skill contains the Ubiquitous language conformance section

  Scenario: Design-review guidance omits Arc42 coverage when the Arc42 addon is disabled
    Given  a Givn project with the arc42 addon disabled and the skills synchronized
    Then  the generated design-review skill contains no Architecture documentation (arc42) branch
    And  the generated design-review command contains no Architecture documentation (arc42) check

  Scenario: Design-review guidance includes Arc42 coverage when the Arc42 addon is enabled
    Given  a Givn project with the arc42 addon enabled and the skills synchronized
    Then  the generated design-review skill contains the Architecture documentation (arc42) branch
    And  the generated design-review command contains the Architecture documentation (arc42) check

  Scenario: The domain-modeling skill is not generated when the Arc42 addon it needs is disabled
    Given  a Givn project with the domain-modeling addon enabled but the arc42 addon disabled and the skills synchronized
    Then  no generated domain-modeling skill exists

  Scenario: The domain-modeling skill is generated when both addons it needs are enabled
    Given  a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    Then  a generated domain-modeling skill exists

  Scenario: No generated skill or command mentions a disabled addon
    Given  a Givn project with the arc42, domain-modeling, and coverage addons disabled and the skills synchronized
    Then  no generated skill or command mentions any disabled addon

  Scenario: An unknown addon name in a conditional fails the synchronization and names the template
    Given  a Givn project with a project skill template that contains a conditional on an unknown addon
    When  a consumer synchronizes skills through the Givn CLI
    Then  the synchronization fails
    And  the failure names the template with the invalid directive

  Scenario: An unbalanced conditional in a command template fails the synchronization
    Given  a Givn project with a project command template that contains an unbalanced conditional
    When  a consumer synchronizes skills through the Givn CLI
    Then  the synchronization fails
    And  the failure names the template with the invalid directive

  Scenario: Disabling the Arc42 addon removes an existing generated domain-modeling skill
    Given  a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When  a consumer disables the arc42 addon through the Givn CLI
    Then  no generated domain-modeling skill exists

  @e2e
  Scenario: A consumer synchronizes skills and the generated guidance matches the addon configuration
    Given  a Givn project with the arc42 addon disabled and the skills synchronized
    When  a consumer synchronizes skills through the Givn CLI
    Then  the generated review skill contains no Arc42 implementation conformance section
    And  no generated domain-modeling skill exists

  @e2e
  Scenario: A consumer enables the Arc42 addon and the generated guidance gains the Arc42 section
    Given  a Givn project with the arc42 addon disabled and the skills synchronized
    When  a consumer enables the arc42 addon through the Givn CLI
    Then  the generated review skill contains the Arc42 implementation conformance section
    And  the generated review command contains the Arc42 implementation conformance section
    And  a generated domain-modeling skill exists

  @e2e
  Scenario: A consumer disables the domain-modeling addon and the generated domain skill is removed
    Given  a Givn project with the domain-modeling and arc42 addons enabled and the skills synchronized
    When  a consumer disables the domain-modeling addon through the Givn CLI
    Then  no generated domain-modeling skill exists
    And  the generated review skill contains no Ubiquitous language conformance section
