@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.modified
  Scenario: Served ise guidance states that the trial always carries the declaration
    Given  a Givn project with ise effective
    When  the author reads the served ise guidance
    Then  the guidance states that the spec copy always carries the declaration itself
    And  the guidance states that it is an implicit Specification entry, not a sixth role
