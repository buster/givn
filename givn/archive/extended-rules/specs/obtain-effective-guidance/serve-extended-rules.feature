@givn.delta @serve-extended-rules

Feature: The served instruction carries the built-in rules and the project's extension

  @givn.added @e2e
  Scenario: An agent receives the extended rules through the Givn CLI
    Given a Givn project with an active change
    And the project records an extension for the proposal instruction
    When the agent requests the proposal instructions through the Givn CLI
    Then the command exits 0
    And the served content carries the built-in proposal rules
    And the served content carries the project's rule in the marked extension section

  @givn.added
  Scenario: The built-in rules are served alone when no extension exists
    Given a Givn project with an active change
    When the agent requests the specs instructions through the Givn CLI
    Then the served content carries the built-in specs rules
    And the served content carries no extension section

  @givn.added
  Scenario: The extension adds to the built-in rules without replacing or reordering them
    Given a Givn project with an active change
    And the project records an extension for the proposal instruction
    And the extension carries the rule "Never write a release note"
    When the agent requests the proposal instructions through the Givn CLI
    Then every built-in rule of the proposal instruction remains in the served content
    And the built-in rules appear before the marked extension section
    And the contradicting project rule appears beside the built-in release-note rule

  @givn.added
  Scenario: The machine-readable path carries the same extended content as the text path
    Given a Givn project with an active change
    And the project records an extension for the proposal instruction
    When the agent requests the proposal instructions as JSON through the Givn CLI
    Then the response identifies the artifact and its generated paths and prerequisites
    And the response carries the same instruction content as the text path
    And the response carries the project's rule in the marked extension section

  @givn.added
  Scenario: The extension applies on top of a project-owned replacement
    Given a Givn project with an active change
    And a project-owned replacement for the proposal instruction
    And the project records an extension for the proposal instruction
    When the agent requests the proposal instructions through the Givn CLI
    Then the served content carries the replacement text
    And the served content carries the project's rule in the marked extension section

  @givn.added
  Scenario: A guidance request changes nothing in the project
    Given a Givn project with an active change
    And the project records an extension for the proposal instruction
    When the agent requests the instructions for every served step
    Then no project file changed
