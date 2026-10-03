@givn.delta @declare-the-product-start

Feature: Declare the product start

  @givn.modified @e2e
  Scenario: Scaffolding ships an unconfigured run declaration
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the file "givn/commands.yaml" exists
    And the commands file has run.command set to the sentinel
    And the README has the run managed block
    And the README has the run open placeholder
