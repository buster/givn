Feature: enable-optional-addons

  Scenario: givn init defaults to all addons enabled and generates their files
    Given a project root without a givn/ directory
    When the user runs `givn init`
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".opencode/skills/givn-dev-principles/SKILL.md" exists
    And the file ".opencode/commands/grill-me.md" exists
    And the file ".opencode/commands/grill-with-docs.md" exists
    And the directory ".agents/skills/givn-domain-modeling" does not exist

  Scenario: givn init --no-addons produces no skill or command addon files
    Given a project root without a givn/ directory
    When the user runs `givn init --no-addons`
    Then the command exits 0
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    And the directory ".agents/skills/givn-dev-principles" does not exist
    And the file ".opencode/commands/grill-me.md" does not exist
    And the file ".opencode/commands/grill-with-docs.md" does not exist

  Scenario: givn addons enable generates skill and command files
    Given a project root with givn/ initialised without addons
    And givn addon "domain_modeling" is "false"
    When the user runs `givn addons enable domain_modeling`
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" does not exist
    When the user runs `givn addons enable arc42`
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".opencode/commands/grill-me.md" does not exist
    And the directory ".agents/skills/givn-domain-modeling" does not exist

  Scenario: givn addons disable removes generated skill files
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "true"
    And the skill "givn-domain-modeling" exists in all targets
    When the user runs `givn addons disable domain_modeling`
    Then the command exits 0
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    And the directory ".claude/skills/givn-domain-modeling" does not exist
    And the directory ".opencode/skills/givn-domain-modeling" does not exist

  Scenario: givn addons disable removes generated command files
    Given a project root with givn/ initialised
    And givn addon "grill_me" is "true"
    And the command "grill-me" exists in .opencode/commands
    When the user runs `givn addons disable grill_me`
    Then the command exits 0
    And the file ".opencode/commands/grill-me.md" does not exist

  Scenario: givn addons disable when already disabled is idempotent
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "false"
    When the user runs `givn addons disable domain_modeling`
    Then the command exits 0

  Scenario: givn skills sync includes skill and command addons when enabled
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "true"
    And givn addon "dev_principles" is "true"
    And givn addon "grill_me" is "true"
    And givn addon "grill_with_docs" is "true"
    When the user runs `givn skills sync`
    Then the command exits 0
    And stdout contains a line "synced skill: givn-domain-modeling"
    And stdout contains a line "synced skill: givn-dev-principles"
    And stdout contains a line "synced command: /grill-me"
    And stdout contains a line "synced command: /grill-with-docs"

  Scenario: givn skills sync excludes addon skills and commands when disabled
    Given  a project root with givn/ initialised
    And  givn addon "domain_modeling" is "false"
    And  givn addon "grill_me" is "false"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the directory ".opencode/skills/givn-domain-modeling" does not exist
    And  stdout does not contain "synced skill: givn-domain-modeling"
    And  stdout does not contain "synced command: /grill-me"

  Scenario: givn addons enable errors when givn/ is not initialised
    Given a project root without a givn/ directory
    When the user runs `givn addons enable domain_modeling`
    Then the command exits non-zero
    And stderr contains "givn/config.yaml not found"

  Scenario: givn addons disable errors when givn/ is not initialised
    Given a project root without a givn/ directory
    When the user runs `givn addons disable domain_modeling`
    Then the command exits non-zero
    And stderr contains "givn/config.yaml not found"
  Scenario: addon skills are written to the default agent target after sync
    Given a project root with givn/ initialised
    And givn addon "domain_modeling" is "true"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the directory ".agents/skills/givn-domain-modeling" does not exist
    And the directory ".claude/skills/givn-domain-modeling" does not exist

  Scenario: addon skills are written to every configured agent target after sync
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, agents, claude"
    And givn addon "domain_modeling" is "true"
    When the user runs `givn skills sync`
    Then the command exits 0
    And the file ".agents/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".claude/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
  Scenario: The absolute-mode policy block carries the dialog context rule
    Given  a project root with givn/ initialised without addons
    And  givn addon "absolute_mode" is "false"
    When  the user runs `givn addons enable absolute_mode`
    Then  the command exits 0
    And  the file "AGENTS.md" contains "Context required to answer a question is not filler"
