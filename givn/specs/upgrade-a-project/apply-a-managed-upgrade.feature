Feature: Managed upgrade and legacy project migration

  @e2e
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
      | Git |
      | Fossil |

  @e2e
  Scenario: A clean project without migration requirements receives one managed upgrade commit
    Given a clean Git project with current Givn files and a stale generated integration
    When the consumer invokes `givn upgrade` for migration-free refresh
    Then the command exits successfully
    And the project config identifies the current Givn version
    And one managed upgrade commit exists
    And no migration change exists

  Scenario: An unclean project is protected from automatic migration commits
    Given a Givn 0.2.0 project with uncommitted user changes
    When the consumer invokes `givn upgrade` for migration
    Then the command fails before changing project files
    And no commit is created
    And the user changes remain byte-for-byte unchanged

  Scenario: Repeating the upgrade does not duplicate the migration
    Given a project that already has the current-version migration change
    When the consumer runs `givn upgrade` again
    Then the command does not overwrite the migration change
    And no second migration commit is created
    And the command reports that the project is already prepared for migration

  Scenario: A project outside Git is protected from a partial automatic migration
    Given  a Givn 0.2.0 project that is not a Git repository
    When  the consumer invokes `givn upgrade` for migration
    Then  the command fails before changing project files
    And  the command explains that a version-controlled project is required for the automatic migration

  Scenario: An existing migration plan is never overwritten
    Given a Givn 0.2.0 project with an existing "migrate-0-2-0-to-0-4-0" change
    When the consumer invokes `givn upgrade` for migration
    Then the command does not overwrite the existing migration proposal or design
    And the command reports the existing migration change

  Scenario: The migration change remains honest until the project review passes
    Given an automatically created current-version migration change
    When an LLM opens the migration change
    Then the change asks for project-specific task derivation
    And it does not contain a fabricated design-review pass
    And it does not contain a fabricated implementation commit
    And the normal Givn review and archive gates remain required

  @e2e
  Scenario: A completed maintenance migration can be archived without a product specification
    Given a completed automatic current-version migration change
    When the consumer archives the maintenance migration
    Then the maintenance migration is archived successfully
    And the permanent specification tree contains no migration feature
  @e2e
  Scenario: A consumer invoking a retired engine command is told it does not exist
    Given  an installed Givn binary
    When  the consumer invokes the retired `givn spec index` command
    Then  the command exits non-zero
    And  the error reports "spec index" is not an available command
    And  the error output stays free of "embedding" and "download" wording
  @e2e
  Scenario: A clean Fossil project receives the same managed upgrade revision shape
    Given  a clean Fossil project with current Givn files and a stale generated integration
    When  the consumer invokes `givn upgrade` for migration-free refresh
    Then  the command exits successfully
    And  the project config identifies the current Givn version
    And  one managed upgrade revision exists
    And  no migration change exists

  Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits
    Given  a Givn 0.2.0 Fossil project with uncommitted user changes
    When  the consumer invokes `givn upgrade` for migration
    Then  the command fails before changing project files
    And  no revision is created
    And  the user changes remain byte-for-byte unchanged
