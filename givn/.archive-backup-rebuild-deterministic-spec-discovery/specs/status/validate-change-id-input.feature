Feature: Validate Change ID Input

  Scenario: Valid kebab-case change id is accepted by every command that accepts --change
    Given  a project root with givn/ initialised
    And  a change "my-feature" exists with artifact "proposal.md"
    When  the user runs `givn status --change my-feature`
    Then  the command exits 0

  Scenario: Path traversal id is rejected by status
    Given  a project root with givn/ initialised
    When  the user runs `givn status --change "../etc"`
    Then  the command exits non-zero
    And  stderr contains "kebab-case"

  Scenario: Path traversal id is rejected by archive
    Given  a project root with givn/ initialised
    And the project declares a complete run declaration
    When  the user runs `givn archive --change "../../../tmp"`
    Then  the command exits non-zero
    And  stderr contains "kebab-case"

  Scenario: Path traversal id is rejected by check
    Given  a project root with givn/ initialised
    When  the user runs `givn check tasks --change "./.git"`
    Then  the command exits non-zero
    And  stderr contains "kebab-case"

  Scenario: Path traversal id is rejected by lint
    Given  a project root with givn/ initialised
    When  the user runs `givn lint --change "../sibling"`
    Then  the command exits non-zero
    And  stderr contains "kebab-case"

  Scenario: Path traversal id is rejected by graph
    Given  a project root with givn/ initialised
    When  the user runs `givn graph --change "../../escape"`
    Then  the command exits non-zero
    And  stderr contains "kebab-case"

  Scenario: Path traversal id is rejected by instructions
    Given  a project root with givn/ initialised
    When  the user runs `givn instructions --change "../back"`
    Then  the command exits non-zero
    And  stderr contains "kebab-case"

  @e2e
  Scenario: Consumer invokes givn status with a path traversal change id and gets a clear error
    Given  a givn project with a change "my-change" and proposal.md
    When  the user invokes `givn status --change "../etc"`
    Then  the command fails
    And  stderr says the change id must be kebab-case
