@givn.delta @lint-command

Feature: lint-command

  @givn.added
  Scenario: A change seed without a status line is reported as an advisory
    Given a project root with givn/ initialised
    And a change seed at "givn/ideation/topic/changes/seed-one.md" with:
      """
      # Change seed: add-widget

      Use-case ID: add-widget
      """
    And a change seed at "givn/ideation/topic/changes/seed-clean.md" with:
      """
      # Change seed: add-gadget

      Status: ready
      """
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[SEED ]"
    And stderr contains "no valid status line"
    And stderr does not contain "seed-clean"

  @givn.added
  Scenario: A change seed proposed by a missing change is reported as an advisory
    Given a project root with givn/ initialised
    And an existing change named "active-change"
    And a change seed at "givn/ideation/topic/changes/seed-two.md" with:
      """
      # Change seed: add-widget

      Status: proposed by ghost-change
      """
    And a change seed at "givn/ideation/topic/changes/seed-active.md" with:
      """
      # Change seed: add-gadget

      Status: proposed by active-change
      """
    When the user runs `givn lint`
    Then the command exits 0
    And stdout contains a line "givn lint: no .feature files found under givn/"
    And stderr contains "[SEED ]"
    And stderr contains "ghost-change"
    And stderr does not contain "seed-active"

  @givn.added
  Scenario: A change seed proposed by an archived change is reported as stale
    Given a project root with givn/ initialised
    And a change "done-change" exists in archive
    And a change seed at "givn/ideation/topic/changes/seed-three.md" with:
      """
      # Change seed: add-widget

      Status: proposed by done-change
      """
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[SEED ]"
    And stderr contains "already archived"

  @givn.added
  Scenario: An unrecognised seed status is reported and a status after prose is honoured
    Given a project root with givn/ initialised
    And a change seed at "givn/ideation/topic/changes/seed-unrecognised.md" with:
      """
      # Change seed: add-widget

      Notes about the handoff.

      Status: done
      """
    And a change seed at "givn/ideation/topic/changes/seed-prose.md" with:
      """
      # Change seed: add-gadget

      Notes about the handoff.

      Status: ready
      """
    And a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Bar
          Given x
      """
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[SEED ]"
    And stderr contains "no valid status line"
    And stderr does not contain "seed-prose"

  @givn.added
  Scenario: An advisory seed finding is counted in the summary and still exits 0
    Given a project root with givn/ initialised
    And a change seed at "givn/ideation/topic/changes/seed-five.md" with:
      """
      # Change seed: add-widget

      Status: proposed by ghost-change
      """
    And a .feature file at "givn/specs/foo/foo.feature" with:
      """
      Feature: Foo
        Scenario: Bar
          Given x
      """
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "[SEED ]"
    And stderr contains "givn lint: 1 file(s) checked, 1 finding(s)"
