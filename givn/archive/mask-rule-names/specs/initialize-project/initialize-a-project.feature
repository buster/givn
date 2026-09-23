@givn.delta @initialize-a-project

Feature: Initialize a project

  @givn.modified
  Scenario: Initialization keeps the generated rules out of the repository
    Given  a committed project root with givn/ initialised
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the revision does not track ".opencode/skills/givn-propose/SKILL.md"
    And  the tracked ignore block contains ".opencode/skills/givn-*/"
    And  the tracked ignore block masks the rule names
    And  the local exclude contains ".opencode/skills/givn-*/"
