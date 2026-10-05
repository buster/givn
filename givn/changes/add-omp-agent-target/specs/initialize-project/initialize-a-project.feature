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

  @givn.modified @wip
  Scenario: givn skills targets add accepts "all" as a shorthand for every target
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode"
    When the user runs `givn skills targets add all`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude, omp"
    And the file ".agents/skills/givn-propose/SKILL.md" exists
    And the file ".claude/skills/givn-propose/SKILL.md" exists
    And the file ".omp/skills/givn-propose/SKILL.md" exists

  @givn.added @wip
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

  @givn.added @wip
  Scenario: The --agents flag accepts the dotted directory form for the omp target
    Given a project root without a givn/ directory
    When the user runs `givn init --agents .omp`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "omp"
    And the file ".omp/skills/givn-propose/SKILL.md" exists
