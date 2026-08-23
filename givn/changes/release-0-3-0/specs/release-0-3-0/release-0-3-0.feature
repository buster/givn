# User Interaction Inventory:
# - run `givn upgrade` in a clean project created with Givn 0.2.0

@givn.delta @release-0-3-0
Feature: Givn 0.3.0 release and legacy project migration

  The 0.3.0 release must describe the complete stacked change set and give
  projects created from the 0.2.0 master branch a safe, reviewable migration.

  @givn.added @wip
  Scenario: The release identifies itself as its package version
    Given the Givn release metadata
    When a consumer asks for the Givn version
    Then the reported version matches the package version

  @givn.added @wip
  Scenario: The changelog describes the complete 0.3.0 release
    Given the Givn 0.3.0 release files
    When a maintainer reads the changelog
    Then it contains the user-visible changes from the stacked release
    And it contains the 0.2.0 to 0.3.0 migration entry point

  @givn.added @wip
  Scenario: The migration guide is an executable prompt sequence
    Given the 0.2.0 to 0.3.0 migration guide
    When an LLM reads the guide from a project using Givn
    Then it finds ordered prompts for inventory, upgrade, synchronization, review, and verification
    And each prompt tells the LLM which project evidence to inspect before editing
    And the guide forbids automatic changes to domain specifications and architecture decisions

  @givn.added @e2e @wip
  Scenario: A clean 0.2.0 project receives a reviewable 0.3.0 migration plan
    Given a clean Git project created with Givn 0.2.0
    When the consumer invokes `givn upgrade` for migration
    Then the command exits successfully
    And the project config identifies Givn 0.3.0
    And one commit contains only the managed scaffold and generated agent integration changes
    And a migration change named "migrate-0-2-0-to-0-3-0" exists
    And the migration change records the managed upgrade commit
    And the migration change contains a proposal and detailed design
    And the migration change intentionally has no product specification

  @givn.added @wip
  Scenario: An unclean project is protected from automatic migration commits
    Given a Givn 0.2.0 project with uncommitted user changes
    When the consumer runs `givn upgrade`
    Then the command fails before changing project files
    And no commit is created
    And the user changes remain byte-for-byte unchanged

  @givn.added @wip
  Scenario: Repeating the upgrade does not duplicate the migration
    Given a project that already has the 0.3.0 migration change
    When the consumer runs `givn upgrade` again
    Then the command does not overwrite the migration change
    And no second migration commit is created
    And the command reports that the project is already prepared for migration

  @givn.added @wip
  Scenario: A project outside Git is protected from a partial automatic migration
    Given a Givn 0.2.0 project that is not a Git repository
    When the consumer runs `givn upgrade`
    Then the command fails before changing project files
    And the command explains that a Git repository is required for the automatic migration

  @givn.added @wip
  Scenario: An existing migration plan is never overwritten
    Given a Givn 0.2.0 project with an existing "migrate-0-2-0-to-0-3-0" change
    When the consumer runs `givn upgrade`
    Then the command does not overwrite the existing migration proposal or design
    And the command reports the existing migration change

  @givn.added @wip
  Scenario: The migration change remains honest until the project review passes
    Given an automatically created 0.3.0 migration change
    When an LLM opens the migration change
    Then the change asks for project-specific task derivation
    And it does not contain a fabricated design-review pass
    And it does not contain a fabricated implementation commit
    And the normal Givn review and archive gates remain required
