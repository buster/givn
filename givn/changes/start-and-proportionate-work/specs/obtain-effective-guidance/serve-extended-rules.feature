@givn.delta @serve-extended-rules

Feature: The served instruction carries the built-in rules and the project's extension

  @givn.added @wip
  Scenario: The start guidance recommends a proportionate outcome
    Given a Givn project
    Then the served start instruction contains "Answer and stop"
    And the served start instruction contains "Direct work without a Givn change"
    And the served start instruction contains "givn-ideation"
    And the served start instruction contains "givn-propose"
    And the served start instruction contains "proportionality"
    And the served start instruction contains "creates no workflow artifacts"
    And the served start instruction contains "do not assume every modification requires a Givn change"

  @givn.added @wip
  Scenario: The start guidance preserves direct-work obligations
    Given a Givn project
    Then the served start instruction contains "Read the relevant code"
    And the served start instruction contains "focused correction"
    And the served start instruction contains "regression coverage"
    And the served start instruction contains "Report the modification and verification evidence"
    And the served start instruction contains "Do not create a proposal, design, task list, or archive ceremony"
    And the served start instruction contains "authorization to edit is not authorization to commit"

  @givn.added @wip
  Scenario: The start guidance keeps the handoff with the user
    Given a Givn project
    Then the served start instruction contains "wait unless continuation is already authorized"
    And the served start instruction contains "An ordinary request to discuss is not authorization"
    And the served start instruction contains "Direct entry remains available"

  @givn.added @wip
  Scenario: The start guidance is available without optional addons
    Given a project root with givn/ initialised without addons
    Then the served start instruction contains "Answer and stop"
    And the served start instruction contains "Direct work without a Givn change"

  @givn.added @wip
  Scenario: The retired explore instruction has no callable alias
    Given a Givn project
    When the user runs `givn instructions explore`
    Then the command fails
    And stderr contains "explore"

  @givn.added @wip
  Scenario: A project-owned explore override cannot restore a callable alias
    Given a Givn project with a project-owned explore instruction override
    When the user runs `givn instructions explore`
    Then the command fails
    And stderr contains "givn instructions start"

  @givn.modified
  Scenario: The served implementation guidance requires behavioral RED before production edits
    Given a coherent named behavior batch is planned in tasks with scenarios validation and an intended commit boundary
    When the author requests the implementation and steps guidance
    Then tests and bindings must run before production edits for new behavior
    And the observed RED must be a meaningful behavioral failure rather than missing bindings syntax registration fixtures or environment failure
    And the actual command and short observed failure are recorded compactly in the batch task as they occur
    And minimum implementation is followed by actual GREEN and REFACTOR checks recorded briefly
