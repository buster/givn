@givn.delta @describe-the-release

Feature: Givn release metadata and migration route

  @givn.modified
  Scenario: The migration guide is an executable prompt sequence
    Given the 0.7.0 to 0.8.0 migration guide
    When an LLM reads the guide from a project using Givn
    Then it finds ordered prompts that begin with inventory and end with validation and archive
    And the guide requires the LLM to inspect the project before editing it
    And the guide keeps corpus repairs out of the maintenance change
