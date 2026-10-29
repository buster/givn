@givn.delta @readme-enforcement
Feature: README enforcement

  @givn.modified
  Scenario: The check names a damaged managed block
    Given a project root with givn/ initialised
    And a README with a broken development managed block
    And the README is snapshotted
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "development managed block"
    And the README is unchanged

  @givn.modified @e2e
  Scenario: Archiving refreshes the development block
    Given a project root with givn/ initialised
    And a stale development managed block with exactly one ordered opening and closing marker
    And user-authored prose before and after the managed block
    And a givn project providing its own workflow scripts
    And a completed change "readme-demo"
    And the project declares a complete run declaration
    When the user invokes `givn archive --change readme-demo`
    Then the command succeeds
    And the development block lists the configured commands
    And the user-authored prose before and after the managed block is byte-for-byte unchanged
    And the archived change directory exists
    And the terminal transcript is captured for this scenario

  @givn.added
  Scenario Outline: Ambiguous managed markers are rejected before README modification
    Given a project root with givn/ initialised
    And an otherwise complete README whose <block> managed block markers have <damage>
    And user-authored prose surrounds and separates the marker occurrences
    And the README is snapshotted
    And an otherwise completed change "readme-demo" is ready for archive preflight
    When the user runs <command>
    Then the command exits non-zero
    And the diagnostic names the invalid <block> managed block markers
    And the README is byte-for-byte unchanged
    And no automatic marker repair is attempted

    Examples:
      | block       | damage                              | command                            |
      | development | a closing marker before its opening | `givn readme check`                 |
      | development | a duplicated opening marker         | `givn readme check`                 |
      | development | a duplicated closing marker         | `givn readme check`                 |
      | development | two complete marker pairs           | `givn readme check`                 |
      | run         | a closing marker before its opening | `givn readme check`                 |
      | run         | a duplicated opening marker         | `givn readme check`                 |
      | run         | a duplicated closing marker         | `givn readme check`                 |
      | run         | two complete marker pairs           | `givn readme check`                 |
      | development | a closing marker before its opening | `givn archive --change readme-demo` |
      | development | a duplicated opening marker         | `givn archive --change readme-demo` |
      | development | a duplicated closing marker         | `givn archive --change readme-demo` |
      | development | two complete marker pairs           | `givn archive --change readme-demo` |
      | run         | a closing marker before its opening | `givn archive --change readme-demo` |
      | run         | a duplicated opening marker         | `givn archive --change readme-demo` |
      | run         | a duplicated closing marker         | `givn archive --change readme-demo` |
      | run         | two complete marker pairs           | `givn archive --change readme-demo` |
