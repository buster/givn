Feature: README enforcement

  @e2e
  Scenario: Scaffolding marks the unfilled sections
    Given  a project root without a givn/ directory
    When  the user runs `givn init --agents opencode`
    Then  the file "README.md" exists
    And  the README has the description open placeholder
    And  the README has the license open placeholder
    And  the README has the development managed block

  Scenario: The check names every open placeholder
    Given a project root with givn/ initialised
    And a scaffolded README
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "Quick start"
    And stdout mentions "License"
    And stdout mentions "Exact commands to clone"

  Scenario: A legacy README is reported for migration
    Given a project root with givn/ initialised
    And a legacy scaffold README
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "migration"

  Scenario: The check names a damaged managed block
    Given a project root with givn/ initialised
    And a README with a broken development managed block
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "development managed block"

  Scenario: The check reports a missing README
    Given a project root with givn/ initialised
    And the README is missing
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "README.md is missing"

  Scenario: A marker-less README is unmanaged
    Given a project root with givn/ initialised
    And a hand-written README
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "adapt the README or disable the readme addon"

  @e2e
  Scenario: The check passes on a complete README
    Given a project root with givn/ initialised
    And a complete README
    When the user invokes `givn readme check`
    Then the command exits 0

  Scenario: Missing impact decision refuses the review
    Given a project root with givn/ initialised
    And verify commands are no-ops
    And a reviewable change "readme-demo"
    And the project declares a complete run declaration
    When the user runs `givn check review --change readme-demo`
    Then the command exits non-zero
    And stderr contains "README impact decision"

  Scenario: Updated impact decision passes the review
    Given a project root with givn/ initialised
    And verify commands are no-ops
    And a reviewable change "readme-demo"
    And its review records the impact "updated - Usage"
    And the project declares a complete run declaration
    When the user runs `givn check review --change readme-demo`
    Then the command exits 0

  Scenario: A disabled addon needs no impact decision
    Given a project root with givn/ initialised
    And givn addon "readme" is "false"
    And verify commands are no-ops
    And a reviewable change "no-addon"
    And the project declares a complete run declaration
    When the user runs `givn check review --change no-addon`
    Then the command exits 0

  Scenario: An unknown impact value is refused
    Given a project root with givn/ initialised
    And verify commands are no-ops
    And a reviewable change "readme-demo"
    And its review records the impact "maybe"
    And the project declares a complete run declaration
    When the user runs `givn check review --change readme-demo`
    Then the command exits non-zero

  Scenario: Open placeholders block the archive
    Given a project root with givn/ initialised
    And a scaffolded README
    And verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo"
    And the project declares a complete run declaration
    When the user runs `givn archive --change readme-demo`
    Then the command exits non-zero
    And stdout mentions "Quick start"
    And the directory givn/changes/readme-demo/ exists

  Scenario: A missing README blocks the archive
    Given a project root with givn/ initialised
    And the README is missing
    And verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo"
    And the project declares a complete run declaration
    When the user runs `givn archive --change readme-demo`
    Then the command exits non-zero
    And the directory givn/changes/readme-demo/ exists

  Scenario: An unmanaged README blocks the archive
    Given a project root with givn/ initialised
    And a hand-written README
    And verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo"
    And the project declares a complete run declaration
    When the user runs `givn archive --change readme-demo`
    Then the command exits non-zero
    And the directory givn/changes/readme-demo/ exists

  Scenario: A deleted development block is not re-inserted
    Given a project root with givn/ initialised
    And a badge-only README
    And verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo"
    And the project declares a complete run declaration
    When the user runs `givn readme check`
    Then the command exits 0
    When the user runs `givn archive --change readme-demo`
    Then the command exits 0
    And the development block stays deleted

  @e2e
  Scenario: Archiving refreshes the development block
    Given a project root with givn/ initialised
    And a stale development managed block
    And a givn project providing its own workflow scripts
    And a completed change "readme-demo"
    And the project declares a complete run declaration
    When the user invokes `givn archive --change readme-demo`
    Then the command succeeds
    And the development block lists the configured commands
    And the archived change directory exists

  @e2e
  Scenario: A recorded impact decision passes the review
    Given a project root with givn/ initialised
    And verify commands are no-ops
    And a reviewable change "readme-demo"
    And its review records the impact "none"
    And the project declares a complete run declaration
    When the user invokes `givn check review --change readme-demo`
    Then the command exits 0

  @e2e
  Scenario: Instructions surface an unfinished README
    Given a project root with givn/ initialised
    And a scaffolded README
    When the user invokes `givn instructions`
    Then the instructions mention the unfinished README

  Scenario: Instructions stay silent on a complete README
    Given a project root with givn/ initialised
    And a complete README
    When the user runs `givn instructions`
    Then stdout does not contain "givn readme check"
