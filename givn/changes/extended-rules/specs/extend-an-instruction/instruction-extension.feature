@givn.delta @instruction-extension

Feature: A project extends an instruction's built-in rules

  @givn.added @e2e
  Scenario: A developer records an extension and it governs the next guidance request
    Given a Givn project with an active change
    And the developer records the rule "Keep proposals under 500 words" for the proposal instruction in the project's configuration
    When the agent requests the proposal instructions through the Givn CLI
    Then the served content carries the project's rule in the marked extension section
    And the built-in proposal rules stay unchanged

  @givn.added
  Scenario: Changing an extension changes the next served content and touches nothing else
    Given a Givn project with an active change and an extension for the proposal instruction
    When the developer changes the project's extension for the proposal instruction to the rule "Keep proposals under 300 words"
    And the agent requests the proposal instructions through the Givn CLI
    Then the served content carries the changed rule
    And no installed guidance file carries the changed rule
    And no revision carries the changed rule

  @givn.added
  Scenario: Removing an extension restores the built-in rules alone
    Given a Givn project with an active change and an extension for the proposal instruction
    When the developer removes the project's extension for the proposal instruction
    And the agent requests the proposal instructions through the Givn CLI
    Then the served content is the built-in rules alone
    And the installed guidance is unchanged

  @givn.added
  Scenario: An extension for an unknown instruction is dropped with a warning
    Given a Givn project with an active change
    When the project's configuration names the unknown instruction "propozal"
    And the agent requests the proposal instructions through the Givn CLI
    Then a warning names the unknown instruction "propozal"
    And the served content is the built-in rules alone

  @givn.added
  Scenario: A malformed extension entry is dropped with a warning
    Given a Givn project with an active change
    When the project's configuration carries a malformed extension entry for the proposal instruction
    And the agent requests the proposal instructions through the Givn CLI
    Then a warning names the malformed entry
    And the served content is the built-in rules alone

  @givn.added
  Scenario: An empty extension list is valid and adds nothing
    Given a Givn project with an active change
    And the project's configuration carries an empty extension list for the proposal instruction
    When the agent requests the proposal instructions through the Givn CLI
    Then the command exits 0
    And the served content carries the built-in proposal rules
    And the served content carries no extension section

  @givn.added
  Scenario: A whitespace-only rule entry is skipped with a warning
    Given a Givn project with an active change
    And the project's configuration carries a whitespace-only rule for the proposal instruction
    When the agent requests the proposal instructions through the Givn CLI
    Then a warning names the malformed extension for the proposal instruction
    And the served content is the built-in rules alone

  @givn.added
  Scenario: A repeated instruction key keeps the last entry
    Given a Givn project with an active change
    And the project's configuration carries two extension entries for the proposal instruction
    When the agent requests the proposal instructions through the Givn CLI
    Then the served content carries the rule from the last entry
    And the served content does not carry the rule from the earlier entry

  @givn.added
  Scenario: An extension extends a policy and a step instruction alike
    Given a Givn project with an active change
    When the project's configuration records a rule for the dialog policy and for the ideation step
    And the agent requests the dialog policy through the Givn CLI
    And the agent requests the ideation instructions through the Givn CLI
    Then each served content carries its project's rule in the marked extension section

  @givn.added @e2e @wip
  Scenario: An upgrade preserves the extension and the next request serves it
    Given a committed Givn project with an active change and an extension for the proposal instruction
    When the consumer upgrades the project through the Givn CLI
    And the agent requests the proposal instructions through the Givn CLI
    Then the upgrade revision carries no extension content
    And the served content carries the project's rule beside the current built-in rules
