Feature: Coverage README guidance

  @e2e
  Scenario: Coverage-enabled project receives a complete README coverage example
    Given  a givn project with the coverage addon enabled
    When  the user runs `givn skills sync`
    Then  the archive guidance shows a README coverage example with a badge, report link, non-E2E and E2E totals, and reproduction commands
