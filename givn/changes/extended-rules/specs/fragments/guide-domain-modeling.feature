@givn.delta @guide-domain-modeling

Feature: Guide domain modeling

  @givn.modified @e2e @wip
  Scenario: An agent synchronizes skills and the domain skill is routed
    Given a Givn project with the domain model addon enabled and a change awaiting specs
    When the agent synchronizes skills through the Givn CLI
    Then the generated domain skill names the domain-modeling skill
    And the generated skill loads during every workflow step of a change
    And the generated givn-domain-modeling guidance directs the domain modeler to the domain-modeling instruction
    And the served domain-modeling instruction contains "maintaining the project's arc42 glossary"
    And the generated skill does not describe the givn change lifecycle
