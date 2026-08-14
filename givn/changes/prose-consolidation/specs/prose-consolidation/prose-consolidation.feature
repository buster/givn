# User Interaction Inventory:
# - Run `givn instructions specs --change prose-consolidation` to read the canonical authoring policy
# - Run `givn skills sync` to receive generated guidance that follows the canonical policy
# Project-specific instruction overrides are configuration variants of the
# `givn instructions` interaction and are covered by the regular scenario below.

@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.added @e2e @wip
  Scenario: Canonical E2E policy is emitted by specs instructions
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "one E2E evidence scenario per distinct real user action"
    And stdout does not contain "one `@e2e` scenario per User Interaction Inventory entry"

  @givn.added @e2e @wip
  Scenario: Generated guidance points to the canonical E2E policy
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then stdout contains "Syncing skills and agent commands..."
    And the generated givn-spec guidance contains "givn instructions specs"
    And the generated givn-design guidance contains "givn instructions specs"
    And the generated givn-review guidance contains "givn instructions specs"
    And the generated givn-spec command contains "givn instructions specs"
    And the generated givn-steps guidance contains "givn instructions specs"
    And the generated givn-tasks guidance contains "givn instructions specs"
    And the generated givn-implement guidance contains "givn instructions specs"
    And the generated givn-design-review guidance contains "givn instructions specs"
    And generated guidance does not contain "one `@e2e` scenario per User Interaction Inventory entry"
    And generated guidance does not contain "one @e2e scenario per distinct happy-path action"

  @givn.added @wip
  Scenario: Project override replaces the canonical specs instruction
    Given a givn project
    And a project override for specs instructions containing "Project-specific E2E policy"
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "Project-specific E2E policy"
    And stdout does not contain "one E2E evidence scenario per distinct real user action"

  @givn.added @wip
  Scenario: Canonical policy classifies output-affecting flags as variants
    Given a givn project
    When the user runs `givn instructions specs --change prose-consolidation`
    Then stdout contains "output-affecting flags are recorded as variants"
    And stdout does not contain "Every subcommand, every flag combination that produces different output"

  @givn.added @wip
  Scenario: Design and review instructions defer to the canonical policy
    Given a givn project
    When the user reads the design and review instructions for prose-consolidation
    Then combined instruction output contains "givn instructions specs"
    And combined instruction output does not contain "one `@e2e` scenario per User Interaction Inventory entry"
