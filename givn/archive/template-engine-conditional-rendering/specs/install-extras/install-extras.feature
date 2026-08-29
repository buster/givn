@givn.delta @install-extras

Feature: install-extras

  @givn.modified
  Scenario: givn addons enable generates skill and command files
    Given  a project root with givn/ initialised without addons
    And  givn addon "domain_modeling" is "false"
    When  the user runs `givn addons enable domain_modeling`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" does not exist
    When  the user runs `givn addons enable arc42`
    Then  the command exits 0
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/commands/grill-me.md" does not exist
    And  the directory ".agents/skills/givn-domain-modeling" does not exist
