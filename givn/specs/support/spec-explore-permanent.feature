Feature: Explore permanent scenarios before creation

  @e2e
  Scenario: Lexical search locates a permanent scenario by a step text token
    Given  a spec discovery project whose corpus contains a scenario mentioning "shovel-unique-token"
    When  the user searches the permanent corpus for "shovel-unique-token" with `givn spec find`
    Then  the command exits 0
    And  stdout contains the matching scenario title
    And  stdout identifies the owning group and capability

  @e2e
  Scenario: The spec tree reports per-group counts and the group narrative
    Given  a spec discovery project with two groups and a group.md narrative
    When  the user inspects the corpus with `givn spec tree`
    Then  the command exits 0
    And  stdout contains both group names
    And  stdout contains the scenario count and the e2e count per capability
    And  stdout contains the group actor line

  Scenario: Lexical search is parsed from the AST so docstring payload creates no phantom hits
    Given  a spec discovery project whose corpus embeds literal Gherkin with a "Scenario:" line inside a docstring
    When  the user searches the permanent corpus for the docstring-embedded title with `givn spec find`
    Then  the command exits 0
    And  stdout reports exactly one matching scenario

  Scenario: Lexical search reports an empty result for unknown terms
    Given  a spec discovery project with a non-empty corpus
    When  the user searches the permanent corpus for "no-such-token-xyz" with `givn spec find`
    Then  the command exits 0
    And  stdout contains "no scenarios match"

  Scenario: Discovery commands never modify permanent files
    Given  a spec discovery project with two groups and a group.md narrative
    And  the permanent specifications are snapshotted
    When  the user inspects the corpus with `givn spec tree`
    And  the user searches the permanent corpus for "anything" with `givn spec find`
    Then  the command exits 0
    And  the permanent specifications are unchanged
