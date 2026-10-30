@givn.delta @refresh-generated-guidance

Feature: Refresh generated guidance

  @givn.added
  Scenario: Fresh initialization generates start instead of explore
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the file ".opencode/skills/givn-start/SKILL.md" exists
    And the directory ".opencode/skills/givn-explore" does not exist

  @givn.added
  Scenario: Synchronization removes a recognizable generated explore skill
    Given an initialized addon-free project selecting opencode
    And a recognizable generated explore skill at ".opencode/skills/givn-explore/SKILL.md"
    When the user runs `givn skills sync`
    Then the directory ".opencode/skills/givn-explore" does not exist

  @givn.added
  Scenario: Synchronization preserves a custom explore skill and reports a manual transition
    Given an initialized addon-free project selecting opencode
    And a custom explore skill at ".opencode/skills/givn-explore/SKILL.md" with content "# Custom explore"
    When the user runs `givn skills sync`
    Then the file ".opencode/skills/givn-explore/SKILL.md" contains "# Custom explore"
    And stdout contains "manual transition"
    And stdout contains "givn-explore"

  @givn.added
  Scenario: Synchronization still removes a recognizable generated explore command
    Given an initialized addon-free project selecting opencode
    And a recognizable generated explore command at ".opencode/commands/givn-explore.md"
    When the user runs `givn skills sync`
    Then the file ".opencode/commands/givn-explore.md" does not exist



  @givn.added
  Scenario: Synchronization reports a project-owned explore override without changing it
    Given an initialized addon-free project selecting opencode
    And a project file "givn/skills/givn-explore/SKILL.md.tmpl" with content "# Custom source"
    When the user runs `givn skills sync`
    Then the file "givn/skills/givn-explore/SKILL.md.tmpl" contains "# Custom source"
    And stdout contains "manual transition"
    And stdout contains "givn-explore"

  @givn.added
  Scenario: Upgrade removes a tracked generated explore skill under the versioned rules policy
    Given a committed project with a tracked generated explore skill and the versioned rules policy
    When the user runs `givn upgrade`
    Then the file ".opencode/skills/givn-explore/SKILL.md" does not exist
    And the latest revision does not contain ".opencode/skills/givn-explore/SKILL.md"
