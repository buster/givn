Feature: Rewrite history

  Scenario: The rewrite purges reachable history and keeps the rule files
    Given  a committed project root with givn/ initialised
    And  a generated rule committed in an earlier revision at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn rewrite-history --yes`
    Then  the command exits 0
    And  stdout contains "force-push"
    And  the reachable history does not contain ".opencode/skills/givn-propose/SKILL.md"
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists on disk

  Scenario: A dirty checkout is refused before any write
    Given  a committed project root with givn/ initialised
    And  a generated rule committed in an earlier revision at ".opencode/skills/givn-propose/SKILL.md"
    And  an untracked project file at "src/lib.rs"
    When  the user runs `givn rewrite-history --yes`
    Then  the command exits non-zero
    And  stderr contains "clean checkout"
    And  the reachable history contains ".opencode/skills/givn-propose/SKILL.md"

  Scenario: Piped input without the confirmation flag refuses
    Given  a committed project root with givn/ initialised
    And  a generated rule committed in an earlier revision at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn rewrite-history`
    Then  the command exits non-zero
    And  stderr contains "--yes"

  Scenario: The versioned policy is refused
    Given  a committed project root with givn/ initialised
    And  a givn/config.yaml with rules "versioned"
    When  the user runs `givn rewrite-history --yes`
    Then  the command exits non-zero
    And  stderr contains "versioned"

  Scenario: The Fossil backend is refused by name
    Given  an initialized Fossil project
    When  the user runs `givn rewrite-history --yes`
    Then  the command exits non-zero
    And  stderr contains "Fossil"
