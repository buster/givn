@givn.delta @serve-extended-rules

Feature: The served instruction carries the built-in rules and the project's extension

  @givn.added @wip
  Scenario: The ise instruction is served while ise is effective
    Given  an ise-enabled project
    When  the user requests the `ise` instruction
    Then  the command exits 0
    And  the served instruction names the five rebuild obligations

  @givn.added @wip
  Scenario: An ise instruction request with the addon off is refused by name
    Given  an initialized project with ise disabled
    When  the user requests the `ise` instruction
    Then  the command exits non-zero
    And  stderr names "ise"
    And  no served instruction content is printed

  @givn.added @wip
  Scenario: The machine-readable ise path carries the same content as the text path
    Given  an ise-enabled project
    When  the user requests the `ise` instruction as machine-readable output
    Then  the command exits 0
    And  the machine-readable instruction field equals the served text
