@givn.delta @readme-enforcement

Feature: README enforcement

  @givn.modified @e2e @wip
  Scenario: Scaffolding marks the unfilled sections
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the file "README.md" exists
    And the README has the description open placeholder
    And the README has the license open placeholder
    And the README has the development managed block
