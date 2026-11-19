@givn.delta @serve-extended-rules
Feature: The served instruction carries the built-in rules and the project's extension

  @givn.modified @serve-extended-rules
  Scenario: Served ise guidance distinguishes the trial gate from input equality
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance names "givn ise compare" as the normal executed trial path
    And  the guidance states that "givn ise verify-trial" remains the explicit executor
    And  the guidance states that "--inputs-only" prints "Trial result: unverified"

  @givn.added @serve-extended-rules
  Scenario: Served ise guidance requires the ordinary component workflow
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance requires the builder to write the component runner step definitions and fakes
    And  the guidance requires the component feature files to execute even though they stay in the Specification
    And  the guidance requires per-tier coverage and an uninstrumented End-to-end report
    And  the guidance refuses a coverage number as a substitute for the declared scenario execution
