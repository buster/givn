@givn.delta @outline-report

Feature: Outline report

  @givn.added
  Scenario: The Tree-sitter evaluation leaves the current outline engine in place
    Given a Givn project with the outline experiment enabled
    And a production source unit
    When the developer runs "givn outline report --json"
    Then the JSON report has schema version 3
    And the JSON report names Rust and Python as the supported source languages
