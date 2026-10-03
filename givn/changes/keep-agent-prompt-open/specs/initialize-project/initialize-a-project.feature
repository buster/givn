@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.removed @wip
  Scenario: The interactive prompt refuses a confirmation without a marked target
    Given a placeholder step

  @givn.added
  Scenario: The interactive prompt accepts the pre-marked OpenCode default
    Given a project root without a givn/ directory
    When the user runs `givn init` in a terminal and confirms the prompt
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist
    And the directory ".claude/skills/givn-propose" does not exist

  @givn.added
  Scenario: The interactive prompt keeps asking until a target is marked
    Given a project root without a givn/ directory
    When the user runs `givn init` in a terminal, unmarks every target, and confirms twice
    Then the command exits 0
    And stderr contains "mark at least one agent target"
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".agents/skills/givn-propose" does not exist
