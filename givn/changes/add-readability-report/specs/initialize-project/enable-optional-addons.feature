@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.removed @wip
  Scenario: givn init defaults to all addons enabled and generates their files
    Given the scenario is removed

  @givn.added
  Scenario: givn init enables only default-on addons
    Given a project root without a givn/ directory
    When the user runs "givn init --agents opencode"
    Then the command exits 0
    And the file ".opencode/skills/givn-domain-modeling/SKILL.md" exists
    And the file ".opencode/skills/givn-dev-principles/SKILL.md" exists
    And the file ".opencode/commands/grill-me.md" exists
    And the file ".opencode/commands/grill-with-docs.md" exists
    And the readability addon is disabled

  @givn.added
  Scenario: givn init lists default-off addons and how to enable them
    Given a project root without a givn/ directory
    When the user runs "givn init --agents opencode"
    Then the command exits 0
    And stdout contains "addons available (default off): readability"
    When the user runs "givn addons list"
    Then stdout contains "[disabled]  readability"
    And stdout contains "givn addons enable"

  @givn.added @e2e @wip
  Scenario: Consumer enables the readability addon and it is listed as enabled
    Given a project root with givn/ initialised without addons
    And the readability addon is disabled
    When the user runs "givn addons enable readability"
    Then the command exits 0
    And stdout contains "enabled addon 'readability'"
    When the user runs "givn addons list"
    Then stdout contains "[enabled ]  readability"
