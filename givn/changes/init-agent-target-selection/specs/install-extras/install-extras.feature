@givn.delta @install-extras
Feature: install-extras

  @givn.modified
  Scenario: givn init defaults to all addons enabled and generates their files
    Given  a project root without a givn/ directory
    When  the user runs `givn init`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/skills/givn-dev-principles/SKILL.md" exists
    And  the file ".opencode/commands/grill-me.md" exists
    And  the file ".opencode/commands/grill-with-docs.md" exists
    And  the directory ".agents/skills/givn-domain-modeling" does not exist

  @givn.modified
  Scenario: givn addons enable generates skill and command files
    Given  a project root with givn/ initialised without addons
    And  givn addon "domain_modeling" is "false"
    When  the user runs `givn addons enable domain_modeling`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/commands/grill-me.md" does not exist
    And  the directory ".agents/skills/givn-domain-modeling" does not exist

  @givn.removed @wip
  Scenario: addon skills are written to all target directories
    Given  a placeholder step

  @givn.added
  Scenario: addon skills are written to the default agent target after sync
    Given  a project root with givn/ initialised
    And  givn addon "domain_modeling" is "true"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And  the directory ".agents/skills/givn-domain-modeling" does not exist
    And  the directory ".claude/skills/givn-domain-modeling" does not exist

  @givn.added
  Scenario: addon skills are written to every configured agent target after sync
    Given  a project root with givn/ initialised
    And  givn skills targets in givn/config.yaml equal "opencode, agents, claude"
    And  givn addon "domain_modeling" is "true"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the file ".agents/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".claude/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists