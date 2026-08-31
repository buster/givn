# User Interaction Inventory:
# - initialise a project with givn (scaffolds the README via the readme addon)
# - run `givn readme check`
# - run `givn check review --change <id>`
# - run `givn archive --change <id>`
# - run `givn instructions`

@givn.delta @readme-enforcement

Feature: README enforcement

  @givn.added @e2e @wip
  Scenario: Consumer scaffolds a project and the README marks its unfilled sections
    Given a project root without a givn/ directory
    When the user runs `givn init`
    Then the file "README.md" exists
    And the README contains an open placeholder for the description
    And the README contains an open placeholder for the license
    And the README contains the development managed block naming the configured workflow commands

  @givn.added
  Scenario: README check reports every open placeholder and its expectation
    Given a project root with givn/ initialised
    And a scaffolded README with open placeholders
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "Quick start"
    And stdout mentions "License"
    And stdout mentions "Exact commands to clone"

  @givn.added
  Scenario: README check reports a legacy scaffold README as needing migration
    Given a project root with givn/ initialised
    And a README with the previous scaffold style
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "migration"

  @givn.added
  Scenario: README check reports a damaged managed block
    Given a project root with givn/ initialised
    And a README with a broken development managed block
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "development managed block"

  @givn.added
  Scenario: README check reports a missing README
    Given a project root with givn/ initialised
    And the README is missing
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "README.md is missing"

  @givn.added
  Scenario: README check reports a hand-written README without givn markers as unmanaged
    Given a project root with givn/ initialised
    And a hand-written README without any givn markers
    When the user runs `givn readme check`
    Then the command exits non-zero
    And stdout mentions "adapt the README or disable the readme addon"

  @givn.added @e2e @wip
  Scenario: Consumer runs the README check on a complete README and it passes
    Given a project root with givn/ initialised
    And a complete README with the development managed block
    When the user invokes `givn readme check`
    Then the command exits 0

  @givn.added
  Scenario: Review refuses sign-off without a recorded README impact decision
    Given a project root with givn/ initialised
    And the project's verify commands are no-ops
    And a reviewable change "readme-demo"
    When the user runs `givn check review --change readme-demo`
    Then the command exits non-zero
    And stderr contains "README impact decision"

  @givn.added
  Scenario: Review accepts an updated README impact decision naming the sections
    Given a project root with givn/ initialised
    And the project's verify commands are no-ops
    And a reviewable change "readme-demo"
    And the review of change "readme-demo" records the README impact decision "updated - Usage"
    When the user runs `givn check review --change readme-demo`
    Then the command exits 0

  @givn.added
  Scenario: A project without the readme addon needs no README impact decision
    Given a project root with givn/ initialised
    And givn addon "readme" is "false"
    And the project's verify commands are no-ops
    And a reviewable change "no-addon"
    When the user runs `givn check review --change no-addon`
    Then the command exits 0

  @givn.added
  Scenario: An unknown README impact value is refused
    Given a project root with givn/ initialised
    And the project's verify commands are no-ops
    And a reviewable change "readme-demo"
    And the review of change "readme-demo" records the README impact decision "maybe"
    When the user runs `givn check review --change readme-demo`
    Then the command exits non-zero

  @givn.added
  Scenario: Archive blocks while the README has open placeholders
    Given a project root with givn/ initialised
    And a scaffolded README with open placeholders
    And the project's verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo" with all required artifacts
    And the semantic gate fixtures are offline
    When the user runs `givn archive --change readme-demo`
    Then the command exits non-zero
    And stdout mentions "Quick start"
    And the directory givn/changes/readme-demo/ exists

  @givn.added
  Scenario: Archive blocks when the README is missing
    Given a project root with givn/ initialised
    And the README is missing
    And the project's verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo" with all required artifacts
    And the semantic gate fixtures are offline
    When the user runs `givn archive --change readme-demo`
    Then the command exits non-zero
    And the directory givn/changes/readme-demo/ exists

  @givn.added
  Scenario: Archive blocks for a hand-written README without givn markers
    Given a project root with givn/ initialised
    And a hand-written README without any givn markers
    And the project's verify commands are no-ops
    And givn addon "coverage" is "false"
    And a completed change "readme-demo" with all required artifacts
    And the semantic gate fixtures are offline
    When the user runs `givn archive --change readme-demo`
    Then the command exits non-zero
    And the directory givn/changes/readme-demo/ exists

  @givn.added @wip
  Scenario: Archive does not re-insert a deleted development managed block while the README stays managed
    Given a project root with givn/ initialised
    And a complete README whose only managed block is the coverage badge
    And givn addon "coverage" is "false"
    And the project's verify commands are no-ops
    And a completed change "readme-demo" with all required artifacts
    And the semantic gate fixtures are offline
    When the user runs `givn readme check`
    Then the command exits 0
    When the user runs `givn archive --change readme-demo`
    Then the command exits 0
    And the README still has no development managed block

  @givn.added @e2e @wip
  Scenario: Consumer archives a change and the development managed block refreshes
    Given a project root with givn/ initialised
    And a complete README whose development managed block is stale
    And a givn project providing its own workflow scripts
    And a completed change "readme-demo" with all required artifacts
    And the semantic gate fixtures are offline
    When the user invokes `givn archive --change readme-demo`
    Then the command succeeds
    And the development managed block lists the current configured workflow commands
    And the directory givn/archive/readme-demo/ exists

  @givn.added @e2e @wip
  Scenario: Consumer checks a review with a recorded README impact decision and it passes
    Given a project root with givn/ initialised
    And the project's verify commands are no-ops
    And a reviewable change "readme-demo"
    And the review of change "readme-demo" records the README impact decision "none"
    When the user invokes `givn check review --change readme-demo`
    Then the command succeeds

  @givn.added @e2e @wip
  Scenario: Consumer asks for instructions and the unfinished README is surfaced
    Given a project root with givn/ initialised
    And a scaffolded README with open placeholders
    When the user invokes `givn instructions`
    Then the instructions mention the unfinished README and `givn readme check`

  @givn.added @wip
  Scenario: Instructions stay silent about the README when it is complete
    Given a project root with givn/ initialised
    And a complete README with the development managed block
    When the user runs `givn instructions`
    Then stdout does not contain "givn readme check"
