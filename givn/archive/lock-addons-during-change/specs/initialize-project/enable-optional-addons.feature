@givn.delta @enable-optional-addons

Feature: enable-optional-addons

  @givn.added
  Scenario: A new change records the operator add-on configuration
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    Then the file "givn/changes/addon-lock/config-lock.yaml" exists
    And the file "givn/changes/addon-lock/config-lock.yaml" contains "coverage: true"

  @givn.added
  Scenario: An active change refuses add-on mutation
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    And the user runs `givn addons disable coverage`
    Then the command exits non-zero
    And stderr contains "operator-owned"
    And stderr contains "coverage"
    And stderr contains "environment"
    And the givn config keeps addon "coverage" enabled

  @givn.added
  Scenario: A non-interactive operator override is refused
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    And the user runs `givn addons disable coverage --operator --reason "missing dependency"`
    Then the command exits non-zero
    And stderr contains "interactive terminal"
    And the givn config keeps addon "coverage" enabled

  @givn.added
  Scenario: A forced init cannot change add-ons during a locked change
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    And the user runs `givn init --force --no-addons --agents opencode`
    Then the command exits non-zero
    And stderr contains "add-on configuration is locked"
    And stderr contains "coverage"
    And the givn config keeps addon "coverage" enabled

  @givn.added
  Scenario: A forced init that preserves add-ons is allowed during a locked change
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    And the user runs `givn init --force --agents opencode`
    Then the command exits 0
    And the givn config keeps addon "coverage" enabled

  @givn.added
  Scenario: An operator override on a real terminal is recorded
    Given a project root with givn/ initialised
    When the user runs `givn new addon-lock`
    And the operator disables coverage with an override in the real terminal
    Then the command exits 0
    And the givn config keeps addon "coverage" disabled
    And the change lock records the operator override reason

  @givn.added
  Scenario: Add-ons remain changeable without an active change
    Given a project root with givn/ initialised
    When the user runs `givn addons disable coverage`
    Then the command exits 0
    And the givn config keeps addon "coverage" disabled
