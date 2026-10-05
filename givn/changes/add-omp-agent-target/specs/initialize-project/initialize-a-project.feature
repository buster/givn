@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.modified
  Scenario: The --agents flag accepts "all" as a shorthand for every target
    Given a project root without a givn/ directory
    When the user runs `givn init --agents all`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the file ".agents/skills/givn-propose/SKILL.md" exists
    And the file ".omp/skills/givn-propose/SKILL.md" exists
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp"

  @givn.modified
  Scenario: givn skills targets add accepts "all" as a shorthand for every target
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode"
    When the user runs `givn skills targets add all`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp"
    And the file ".agents/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the file ".omp/skills/givn-propose/SKILL.md" exists

  @givn.added
  Scenario: The omp target receives its own guidance and attribution integration
    Given a project root without a givn/ directory
    When the user runs `givn init --agents omp`
    Then the command exits 0
    And the file ".omp/skills/givn-propose/SKILL.md" exists
    And the file ".omp/commands/givn-propose.md" exists
    And the file ".omp/hooks/pre/givn-attribution.js" exists
    And the directory ".opencode/skills/givn-propose" does not exist
    And the directory ".claude/skills/givn-propose" does not exist
    And givn skills targets in givn/config.yaml equal "omp"

  @givn.added
  Scenario: An omp project keeps the generated omp rules out of the repository
    Given a committed project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "omp"
    And a tracked generated rule at ".omp/skills/givn-propose/SKILL.md"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the revision does not track ".omp/skills/givn-propose/SKILL.md"
    And the revision does not track ".omp/hooks/pre/givn-attribution.js"
    And the tracked ignore block contains ".omp/skills/givn-*/"
    And the tracked ignore block contains ".omp/commands/givn-*.md"
    And the tracked ignore block contains ".omp/hooks/pre/givn-attribution.js"
    And the local exclude contains ".omp/skills/givn-*/"

  @givn.added
  Scenario: The --agents flag accepts the dotted directory form for the omp target
    Given a project root without a givn/ directory
    When the user runs `givn init --agents .omp`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "omp"
    And the file ".omp/skills/givn-propose/SKILL.md" exists
