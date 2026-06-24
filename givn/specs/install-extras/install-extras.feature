Feature: install-extras

  Scenario: givn init defaults to extras enabled and generates extras skills
    Given  a project root without a givn/ directory
    When  the user runs `givn init`
    Then  the command exits 0
    And  the file ".agents/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".agents/skills/givn-dev-principles/SKILL.md" exists
    And  the file ".opencode/commands/grill-me.md" exists
    And  the file ".opencode/commands/grill-with-docs.md" exists
    And  givn config key "extras" is "true"

  Scenario: givn init --no-extras produces no extras skills or commands
    Given  a project root without a givn/ directory
    When  the user runs `givn init --no-extras`
    Then  the command exits 0
    And  the directory ".agents/skills/givn-domain-modeling" does not exist
    And  the directory ".agents/skills/givn-dev-principles" does not exist
    And  the file ".opencode/commands/grill-me.md" does not exist
    And  the file ".opencode/commands/grill-with-docs.md" does not exist
    And  givn config key "extras" is "false"

  Scenario: givn extras install enables extras and syncs skills
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "false"
    When  the user runs `givn extras install`
    Then  the command exits 0
    And  givn config key "extras" is "true"
    And  the file ".agents/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/commands/grill-me.md" exists

  Scenario: givn extras install when already enabled is idempotent
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "true"
    And  extras skills and commands are already generated
    When  the user runs `givn extras install`
    Then  the command exits 0
    And  givn config key "extras" is "true"
    And  the file ".agents/skills/givn-domain-modeling/SKILL.md" exists

  Scenario: givn extras uninstall disables extras and removes generated files
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "true"
    And  extras skills and commands are already generated
    When  the user runs `givn extras uninstall`
    Then  the command exits 0
    And  givn config key "extras" is "false"
    And  the directory ".agents/skills/givn-domain-modeling" does not exist
    And  the directory ".agents/skills/givn-dev-principles" does not exist
    And  the file ".opencode/commands/grill-me.md" does not exist
    And  the file ".opencode/commands/grill-with-docs.md" does not exist

  Scenario: givn extras uninstall when already disabled is idempotent
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "false"
    When  the user runs `givn extras uninstall`
    Then  the command exits 0
    And  givn config key "extras" is "false"

  Scenario: givn skills sync includes extras skills and commands when enabled
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "true"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  stdout contains a line "synced skill: givn-domain-modeling"
    And  stdout contains a line "synced skill: givn-dev-principles"
    And  stdout contains a line "synced command: /grill-me"
    And  stdout contains a line "synced command: /grill-with-docs"

  Scenario: givn skills sync excludes extras skills and commands when disabled
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "false"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  stdout does not contain "givn-domain-modeling"
    And  stdout does not contain "grill-me"

  Scenario: extras skills are written to all target directories
    Given  a project root with givn/ initialised
    And  givn config key "extras" is "true"
    When  the user runs `givn skills sync`
    Then  the command exits 0
    And  the file ".agents/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".claude/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists

  Scenario: givn extras install errors when givn/ is not initialised
    Given  a project root without a givn/ directory
    When  the user runs `givn extras install`
    Then  the command exits non-zero
    And  stderr contains "givn/ not found"

  Scenario: givn extras uninstall errors when givn/ is not initialised
    Given  a project root without a givn/ directory
    When  the user runs `givn extras uninstall`
    Then  the command exits non-zero
    And  stderr contains "givn/ not found"
