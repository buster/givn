@givn.delta @specification-declaration

Feature: specification-declaration

  @givn.modified @wip
  Scenario: An unreadable declaration is refused by name
    Given  an ise-enabled project selecting opencode whose declaration path is a directory
    When  the user runs `givn lint`
    Then  the command exits 1
    And  stderr names "givn/specification.yaml"
