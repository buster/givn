Feature: Coverage badge at the top of the README

  @e2e
  Scenario: Coverage-enabled project is guided to place the coverage badge directly after the README title
    Given  a givn project with the coverage addon enabled
    When  the user runs `givn skills sync`
    Then  the archive guidance places the coverage badge directly after the README title
    And  the archive guidance keeps the totals and reproduction commands
