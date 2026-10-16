@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.modified
  Scenario: givn skills targets add accepts "all" as a shorthand for every target
    Given  a project root with givn/ initialised
    And  givn skills targets in givn/config.yaml equal "opencode"
    When  the user runs `givn skills targets add all`
    Then  the command exits 0
    And  givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp, maki"
    And  the file ".agents/skills/givn-propose/SKILL.md" exists
    And  the file ".claude/skills/givn-propose/SKILL.md" exists
    And  the file ".omp/skills/givn-propose/SKILL.md" exists

  @givn.modified
  Scenario: The --agents flag accepts "all" as a shorthand for every target
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents all`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-propose/SKILL.md" exists
    And  the file ".claude/skills/givn-propose/SKILL.md" exists
    And  the file ".agents/skills/givn-propose/SKILL.md" exists
    And  the file ".omp/skills/givn-propose/SKILL.md" exists
    And  givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp, maki"
