@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.modified
  Scenario: givn init defaults to all addons enabled and generates their files
    Given a project root without a givn/ directory
    When the user runs `givn init --agents opencode`
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".opencode/skills/givn-dev-principles/SKILL.md" exists
    And the file ".opencode/commands/grill-me.md" exists
    And the file ".opencode/commands/grill-with-docs.md" exists
    And the directory ".agents/skills/givn-domain-modeling" does not exist

  @givn.modified @wip
  Scenario: givn init --no-addons produces no skill or command addon files
    Given a project root without a givn/ directory
    When the user runs `givn init --no-addons --agents opencode`
    Then the command exits 0
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    And the directory ".agents/skills/givn-dev-principles" does not exist
    And the file ".opencode/commands/grill-me.md" does not exist
    And the file ".opencode/commands/grill-with-docs.md" does not exist
