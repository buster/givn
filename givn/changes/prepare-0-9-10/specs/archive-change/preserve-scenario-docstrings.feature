@givn.delta @preserve-scenario-docstrings
Feature: Preserve supported executable Gherkin on archive

  @givn.modified
  Scenario: A modified scenario keeps its step docstring after archiving
    Given a completed change modifies an existing scenario in a supported Gherkin feature
    And the modified step has a typed multiline docstring containing literal lines "Scenario: payload heading" and "Background: payload heading"
    When the developer archives the change
    Then the permanent scenario retains the complete docstring payload, media type and step attachment
    And the payload headings do not become scenarios or Backgrounds

  @givn.modified
  Scenario: An added scenario keeps its step docstring after archiving
    Given a completed change adds a scenario to an existing supported Gherkin feature
    And the added step has a multiline docstring containing literal tags, triple backticks and a "Scenario Outline:" line
    When the developer archives the change
    Then the added permanent scenario retains the complete docstring and its step attachment
    And payload text is not interpreted as feature structure

  @givn.modified
  Scenario: A modified scenario keeps its step data table after archiving
    Given a completed change modifies an existing supported scenario with a step data table
    And the table contains ordered rows, empty cells and escaped pipe characters
    When the developer archives the change
    Then the permanent scenario retains every table cell and row in order attached to the same step

  @givn.added
  Scenario: Merging a modified scenario preserves its supported Background setup
    Given a permanent supported feature has Background steps with a docstring and data table
    And a completed change modifies one scenario without changing that Background
    When the developer archives the change
    Then the permanent feature retains every Background step, argument and order
    And the modified and unaffected scenarios retain the same Background setup

  @givn.added
  Scenario: Supported execution tags retain their inheritance on archive
    Given a completed change modifies a scenario in a feature with supported inherited execution tags
    And the scenario also has its own execution tags
    When the developer archives the change
    Then the permanent scenario retains the inherited and scenario-specific execution tags
    And delta-operation tags are stripped without dropping execution tags

  @givn.added
  Scenario: A supported Scenario Outline retains every Examples block
    Given a completed change modifies a supported Scenario Outline with placeholder steps and step arguments
    And the outline has multiple tagged Examples blocks with ordered headers and rows
    When the developer archives the change
    Then the permanent outline retains every placeholder step, docstring and table
    And every Examples block retains its execution tags, headers, values and row order

  @givn.added
  Scenario: An unsupported Rule is refused before archive mutation
    Given the supported Gherkin dialect does not support Rule
    And a completed change contains a Rule with nested Background and scenarios
    When the developer archives the change
    Then the command exits non-zero and names the unsupported Rule and feature path
    And no active change, archive receipt or permanent specification is mutated


  @givn.added
  Scenario: A conflicting Background replacement is refused before archive mutation
    Given a permanent English feature has Background setup shared by a modified and an unaffected scenario
    And a completed change modifies the first scenario while supplying different Background setup
    When the developer archives the change
    Then the command exits non-zero and names the conflicting Background and feature path
    And both permanent scenarios retain their original Background setup
    And no active change, archive receipt or permanent specification is mutated

  @givn.added
  Scenario: An explicit non-English Gherkin dialect is refused before archive mutation
    Given a completed change contains a valid French feature declaring "# language: fr"
    And 0.9.10 supports English Gherkin only
    When the developer archives the change
    Then the command exits non-zero and names the unsupported dialect "fr" and feature path
    And no active change, archive receipt or permanent specification is mutated
