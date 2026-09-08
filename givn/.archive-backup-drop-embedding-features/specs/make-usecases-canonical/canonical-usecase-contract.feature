Feature: Canonical use-case contract

  @e2e
  Scenario: Use-case tree reports explicit use-case ownership
    Given  a spec discovery project with two use cases and a usecase.md narrative
    When  the user inspects the corpus with `givn spec tree`
    Then  the command exits 0
    And  stdout contains both use-case names
    And  stdout contains the use-case actor line
