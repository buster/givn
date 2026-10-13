@givn.delta @apply-a-managed-upgrade

Feature: Managed upgrade and legacy project migration

  @givn.added @wip
  Scenario: An upgrade reconciles full-copy guidance to the direction
    Given a committed project root with givn/ initialised
    And a tracked full-copy generated rule at ".opencode/skills/givn-propose/SKILL.md"
    When the user runs `givn upgrade`
    Then the command exits 0
    And the revision shows ".opencode/skills/givn-propose/SKILL.md" rewritten to the direction

  @givn.added @wip
  Scenario: An upgrade leaves the project's extensions untouched
    Given a committed project root with givn/ initialised
    And the project records an extension for the proposal instruction
    When the user runs `givn upgrade`
    Then the command exits 0
    And the upgrade revision carries no extension content
    And the recorded extension is unchanged

  @givn.added @wip
  Scenario: A project-owned replacement stays byte-for-byte across an upgrade
    Given a committed project root with givn/ initialised
    And a project-owned replacement for the proposal instruction
    When the user runs `givn upgrade`
    Then the command exits 0
    And the project-owned replacement is byte-for-byte unchanged
    And the upgrade revision carries no injected text for the replacement
