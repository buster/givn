Feature: Refresh generated guidance

  @e2e @refresh-generated-guidance
  Scenario: Synchronized guidance explains how to resolve the deterministic gates
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  synchronization reports skills rather than skills and agent commands
    And  generated review design and steps skills direct readers to their canonical instructions
    And  the served review instruction retains split-or-keep while treating shape matches as advisory
    And  the served design and steps instructions retain black-box behavior and distinct requirement coverage
    And  generated skills contain no repeated dialog-loading or delegation policy blocks
    And  no separately authored agent commands are generated
    And  the terminal transcript is captured for this scenario

  @refresh-generated-guidance
  Scenario: Review instructions require deterministic gate dispositions
    Given  a givn project
    When  the user runs `givn instructions review --change skill-rewrites`
    Then  the guidance requires split-or-keep and a brief reason for keeping a long scenario
    And  the guidance requires completing a chosen split before sign-off
    And  shape-match findings do not require blocking dispositions

  Scenario: Design and step instructions require the Black-Box-First decision
    Given a givn project
    When the user reads the design and steps instructions for skill-rewrites
    Then combined instruction output contains "Black-Box-First"
    And combined instruction output contains "which case this test covers"

  Scenario: Spec instructions use deterministic gates before retrieval exists
    Given a givn project
    When the user runs `givn instructions specs --change skill-rewrites`
    Then stdout contains "User Interaction Inventory"
    And stdout does not contain "Run `givn spec index`"

  @refresh-generated-guidance
  Scenario: Generated guidance carries no copy of a built-in rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  every generated skill carries trigger metadata and a direction to its served instruction
    And  no generated skill copies built-in instruction rules
    And  dialog policy and delegation boilerplate are not repeated in thin skills
    And  no separately authored agent command is generated

  @refresh-generated-guidance
  Scenario: Generated implementation guidance routes to the task and step instructions
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-implement skill names both givn instructions tasks and givn instructions steps
    And  the served tasks instruction names givn instructions steps

  @refresh-generated-guidance
  Scenario: Fresh initialization generates start instead of explore
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  the file ".opencode/skills/givn-start/SKILL.md" exists
    And  the directory ".opencode/skills/givn-explore" does not exist

  @refresh-generated-guidance
  Scenario: Synchronization removes a recognizable generated explore skill
    Given  an initialized addon-free project selecting opencode
    And  a recognizable generated explore skill at ".opencode/skills/givn-explore/SKILL.md"
    When  the user runs `givn skills sync`
    Then  the directory ".opencode/skills/givn-explore" does not exist

  @refresh-generated-guidance
  Scenario: Synchronization preserves a custom explore skill and reports a manual transition
    Given  an initialized addon-free project selecting opencode
    And  a custom explore skill at ".opencode/skills/givn-explore/SKILL.md" with content "# Custom explore"
    When  the user runs `givn skills sync`
    Then  the file ".opencode/skills/givn-explore/SKILL.md" contains "# Custom explore"
    And  stdout contains "manual transition"
    And  stdout contains "givn-explore"

  @refresh-generated-guidance
  Scenario: Synchronization still removes a recognizable generated explore command
    Given  an initialized addon-free project selecting opencode
    And  a recognizable generated explore command at ".opencode/commands/givn-explore.md"
    When  the user runs `givn skills sync`
    Then  the file ".opencode/commands/givn-explore.md" does not exist

  @refresh-generated-guidance
  Scenario: Synchronization reports a project-owned explore override without changing it
    Given  an initialized addon-free project selecting opencode
    And  a project file "givn/skills/givn-explore/SKILL.md.tmpl" with content "# Custom source"
    When  the user runs `givn skills sync`
    Then  the file "givn/skills/givn-explore/SKILL.md.tmpl" contains "# Custom source"
    And  stdout contains "manual transition"
    And  stdout contains "givn-explore"

  @refresh-generated-guidance
  Scenario: Upgrade removes a tracked generated explore skill under the versioned rules policy
    Given  a committed project with a tracked generated explore skill and the versioned rules policy
    When  the user runs `givn upgrade`
    Then  the file ".opencode/skills/givn-explore/SKILL.md" does not exist
    And  the latest revision does not contain ".opencode/skills/givn-explore/SKILL.md"
