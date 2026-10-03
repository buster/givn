@givn.delta @initialize-a-project

Feature: initialize-a-project

  @givn.removed @wip
  Scenario: Non-interactive init installs skills to opencode only
    Given a placeholder step

  @givn.removed @wip
  Scenario: Interactive init accepts the default selection with an empty answer
    Given a placeholder step

  @givn.removed @wip
  Scenario: Non-interactive init persists opencode as the default target
    Given a placeholder step

  @givn.removed @wip
  Scenario: An answer without any known agent falls back to the default
    Given a placeholder step

  @givn.removed @wip
  Scenario: givn init --force re-applies the default selection
    Given a placeholder step

  @givn.added
  Scenario: Non-interactive init without an agent target refuses initialization
    Given a project root without a givn/ directory
    When the user runs `givn init`
    Then the command exits non-zero
    And stderr contains "no agent target selected"
    And stderr contains ".opencode"
    And stderr contains ".claude"
    And stderr contains "--agents"
    And the directory "givn" does not exist

  @givn.added
  Scenario: The interactive prompt refuses a confirmation without a marked target
    Given a project root without a givn/ directory
    When the user runs `givn init` in a terminal and confirms without marking a target
    Then the command exits non-zero
    And stderr contains "no agent target selected"
    And the directory "givn" does not exist

  @givn.added
  Scenario: An empty answer refuses initialization
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with ""
    Then the command exits non-zero
    And stderr contains "no agent target selected"
    And the directory "givn" does not exist

  @givn.added
  Scenario: An answer without any known agent refuses initialization
    Given a project root without a givn/ directory
    When the user runs `givn init` and answers the agent prompt with "nope"
    Then the command exits non-zero
    And stderr contains "no agent target selected"
    And stderr contains ".agents"
    And the directory "givn" does not exist

  @givn.added
  Scenario: An unknown-only --agents value refuses initialization
    Given a project root without a givn/ directory
    When the user runs `givn init --agents nope`
    Then the command exits non-zero
    And stderr contains "no agent target selected"
    And the directory "givn" does not exist

  @givn.added
  Scenario: givn init --force re-applies the explicitly given selection
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    When the user runs `givn init --force --agents opencode`
    Then the command exits 0
    And givn skills targets in givn/config.yaml equal "opencode"

  @givn.added
  Scenario: givn init --force without a selection keeps the persisted selection
    Given a project root with givn/ initialised
    And givn skills targets in givn/config.yaml equal "opencode, claude"
    When the user runs `givn init --force`
    Then the command exits non-zero
    And stderr contains "no agent target selected"
    And givn skills targets in givn/config.yaml equal "opencode, claude"

  @givn.modified @e2e @wip
  Scenario: Consumer runs init non-interactively and gets opencode skills
    Given a project root without a givn/ directory
    When the consumer runs `givn init --agents opencode`
    Then the command exits 0
    And the file ".opencode/skills/givn-propose/SKILL.md" exists
    And the directory ".claude/skills/givn-propose" does not exist
