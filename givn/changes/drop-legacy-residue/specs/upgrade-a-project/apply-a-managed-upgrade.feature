@givn.delta @apply-a-managed-upgrade

Feature: Managed upgrade and legacy project migration

  @givn.removed
  Scenario: A clean 0.2.0 project receives a reviewable aggregate migration plan
    Given a placeholder step

  @givn.added @e2e @wip
  Scenario Outline: A clean 0.7.0 project receives a reviewable aggregate migration plan
    Given  a clean <backend> project created with Givn 0.7.0
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

  @givn.added
  Scenario: A project below the oldest migration route receives no migration change
    Given a clean Git project created with Givn 0.2.0
    When the consumer invokes `givn upgrade` for migration
    Then the command exits successfully
    And the project config identifies the current Givn version
    And one managed upgrade commit exists
    And no migration change exists

  @givn.modified
  Scenario: An unclean project is protected from automatic migration commits
    Given a Givn 0.7.0 project with uncommitted user changes
    When the consumer invokes `givn upgrade` for migration
    Then the command fails before changing project files
    And no commit is created
    And the user changes remain byte-for-byte unchanged

  @givn.modified
  Scenario: A project outside Git is protected from a partial automatic migration
    Given  a Givn 0.7.0 project that is not a Git repository
    When  the consumer invokes `givn upgrade` for migration
    Then  the command fails before changing project files
    And  the command explains that a version-controlled project is required for the automatic migration

  @givn.modified @wip
  Scenario: An existing migration plan is never overwritten
    Given a Givn 0.7.0 project with an existing "migrate-0-7-0-to-0-8-0" change
    When the consumer invokes `givn upgrade` for migration
    Then the command does not overwrite the existing migration proposal or design
    And the command reports the existing migration change

  @givn.modified @wip
  Scenario: A Fossil project with uncommitted changes is protected from automatic migration commits
    Given  a Givn 0.7.0 Fossil project with uncommitted user changes
    When  the consumer invokes `givn upgrade` for migration
    Then  the command fails before changing project files
    And  no revision is created
    And  the user changes remain byte-for-byte unchanged

  @givn.added @e2e @wip
  Scenario: A consumer invoking the retired history rewrite command is told it does not exist
    Given  an installed Givn binary
    When  the consumer invokes the retired `givn rewrite-history` command
    Then  the command exits non-zero
    And  the error reports "rewrite-history" is not an available command
