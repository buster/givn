@givn.delta @local-only-transition

Feature: Local-only transition

  @givn.added
  Scenario: A tracked generated rule becomes local-only without deleting the file
    Given  a committed project root with givn/ initialised
    And  a tracked generated rule at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-propose/SKILL.md"
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists on disk
    And  ".opencode/skills/givn-propose/SKILL.md" is staged for removal

  @givn.added
  Scenario: The transition is idempotent after the removal is committed
    Given  a committed project root with givn/ initialised
    And  a tracked generated rule at ".opencode/skills/givn-propose/SKILL.md"
    When  the user runs `givn skills sync`
    And  the staged removals are committed
    And  the user runs `givn skills sync`
    Then  the command exits 0
    And  stdout contains "no tracked rule in the current revision"
