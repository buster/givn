@givn.delta @rules-policy

Feature: Rules policy

  @givn.added
  Scenario: A fresh project resolves the rules policy to local-only
    Given  a project root without a givn/ directory
    When  the user runs `givn init`
    Then  the command exits 0
    And  the file "givn/config.yaml" contains "rules: local-only"
    And  stdout contains "rules: local-only"

  @givn.added
  Scenario: An explicit versioned policy is preserved and named
    Given  a committed project root with givn/ initialised
    And  a givn/config.yaml with rules "versioned"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  stdout contains "rules: versioned"
    And  the tracked ignore block does not contain ".opencode/skills/givn-propose/SKILL.md"

  @givn.added
  Scenario: An unrecognized rules value warns and continues as local-only
    Given  a committed project root with givn/ initialised
    And  a givn/config.yaml with rules "localonly"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  stderr contains "unrecognized rules policy"
    And  stdout contains "rules: local-only"
