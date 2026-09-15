@givn.delta @release-0-3-0
Feature: Givn 0.3.0 release and legacy project migration

  @givn.modified @e2e
  Scenario Outline: A clean 0.2.0 project receives a reviewable aggregate migration plan
    Given  a clean <backend> project created with Givn 0.2.0
    When  the consumer invokes `givn upgrade` for migration
    Then  the command exits successfully
    And  the project config identifies the current Givn version
    And  one commit contains only the managed scaffold and generated agent integration changes
    And  a migration change for the current version exists
    And  the migration change records the managed upgrade commit
    And  the migration change contains a proposal and detailed design
    And  the migration change intentionally has no product specification

    Examples:
      | backend |
      | Git     |
      | Fossil  |

  @givn.added @e2e
  Scenario: A clean Fossil project receives the same managed upgrade revision shape
    Given  a clean Fossil project with current Givn files and a stale generated integration
    When  the consumer invokes `givn upgrade` for migration-free refresh
    Then  the command exits successfully
    And  the project config identifies the current Givn version
    And  one managed upgrade revision exists
    And  no migration change exists

  @givn.added
  Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits
    Given  a Givn 0.2.0 Fossil project with uncommitted user changes
    When  the consumer invokes `givn upgrade` for migration
    Then  the command fails before changing project files
    And  no revision is created
    And  the user changes remain byte-for-byte unchanged
