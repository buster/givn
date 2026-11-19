@givn.delta @serve-extended-rules
Feature: The served instruction carries the built-in rules and the project's extension

  @givn.added
  Scenario: Served ise guidance distinguishes the trial gate from input equality
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance names "givn ise verify-trial" as the whole-system result gate
    And  the guidance states that "givn ise compare" proves frozen-input equality only
