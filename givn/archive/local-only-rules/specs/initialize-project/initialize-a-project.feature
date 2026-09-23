@givn.delta @initialize-a-project

Feature: Initialize a project

  @givn.added
  Scenario: Initialization keeps the generated rules out of the repository
    Given  a committed project root with givn/ initialised
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-propose/SKILL.md"
    And  the tracked ignore block contains ".opencode/skills/givn-propose/SKILL.md"
    And  the local exclude contains ".opencode/skills/givn-propose/SKILL.md"

  @givn.added
  Scenario: Initialization keeps a rule from a disabled addon out of the repository
    Given  a committed project root with givn/ initialised
    And  a tracked orphan rule at ".opencode/skills/givn-stale/SKILL.md"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-stale/SKILL.md"
    And  the file ".opencode/skills/givn-stale/SKILL.md" exists on disk
