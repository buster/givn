@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.removed
  Scenario: givn init enables only default-on addons
    Given a placeholder step

  @givn.added
  Scenario: givn init enables every addon by default
    Given  a project root without a givn/ directory
    When  the user runs "givn init --agents opencode"
    Then  the command exits 0
    And  the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And  the file ".opencode/skills/givn-dev-principles/SKILL.md" exists
    And  the file ".opencode/commands/grill-me.md" exists
    And  the file ".opencode/commands/grill-with-docs.md" exists
    When  the user runs "givn addons list"
    Then  stdout contains "[enabled ]  outline"
    And  stdout contains "[enabled ]  architecture_principles"

  @givn.removed
  Scenario: givn init lists default-off addons and how to enable them
    Given a placeholder step

  @givn.added
  Scenario: givn addons list reports every addon enabled by default
    Given  a project root without a givn/ directory
    When  the user runs "givn init --agents opencode"
    Then  the command exits 0
    And  stdout contains "addons:  enabled"
    When  the user runs "givn addons list"
    Then  stdout contains "[enabled ]  outline"
    And  stdout contains "[enabled ]  architecture_principles"
    And  stdout does not contain "addons available (default off)"

  @givn.added
  Scenario: givn init --no-addons writes an explicit opt-out for every addon
    Given  a project root without a givn/ directory
    When  the user runs "givn init --no-addons --agents opencode"
    Then  the command exits 0
    And  givn addon "outline" is "false"
    And  givn addon "architecture_principles" is "false"
    And  givn addon "domain_modeling" is "false"
    When  the user runs "givn addons list"
    Then  stdout contains "[disabled]  outline"
    And  stdout contains "[disabled]  architecture_principles"

  @givn.added
  Scenario: a partial addons block keeps unnamed addons enabled
    Given  a project root without a givn/ directory
    When  the user runs "givn init --no-addons --agents opencode"
    Then  the command exits 0
    When  the user writes the addons block "addons:\n  outline: true"
    And  the user runs "givn addons list"
    Then  stdout contains "[enabled ]  outline"
    And  stdout contains "[enabled ]  domain_modeling"
    And  stdout contains "[enabled ]  coverage"
