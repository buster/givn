@givn.delta @require-durable-retained-paths
Feature: Require Durable Retained Paths

  @givn.added @e2e
  Scenario: Consumer archives a change whose declared retained paths survive the move
    Given a givn project with a completed change "durable-paths" ready to archive
    And  an effective declaration whose limits role names "givn/archive/durable-paths/design.md"
    When the user runs `givn archive --change durable-paths`
    Then the command exits 0
    And  the change directory has been moved to givn/archive/
    And  the archived change contains "design.md"

  @givn.added
  Scenario Outline: Archive is blocked when a declared retained path moves with the change
    Given a givn project with a completed change "moved-limits" ready to archive
    And  an effective declaration whose limits role names "<entry>"
    When the user runs `givn archive --change moved-limits`
    Then the command exits non-zero
    And  stderr names "<entry>"
    And  stderr names "<moved>"
    And  stderr names "<archived>"
    And  stderr says the archive moves a declared retained path
    And  the change directory still exists under givn/changes/
    And  the change directory has no pending archive receipt
    And  no archive candidate exists for "moved-limits"

    Examples:
      | entry                                | moved                                | archived                            |
      | givn/changes/moved-limits/design.md  | givn/changes/moved-limits/design.md  | givn/archive/moved-limits/design.md |
      | givn/changes/**                      | givn/changes/moved-limits            | givn/archive/**                     |

  @givn.added
  Scenario: The refusal reports at most twenty isolated retained paths
    Given a givn project with a completed change "many-limits" ready to archive
    And  an effective declaration whose limits role names 21 paths under "givn/changes/many-limits/"
    When the user runs `givn archive --change many-limits`
    Then the command exits non-zero
    And  stderr lists 20 declared retained paths
    And  stderr says "+1 more"
    And  the change directory still exists under givn/changes/

  @givn.added
  Scenario: Archive is blocked when the declaration cannot be loaded
    Given a givn project with a completed change "broken-declaration" ready to archive
    And  an effective but malformed declaration
    When the user runs `givn archive --change broken-declaration`
    Then the command exits non-zero
    And  stderr names "givn/specification.yaml"
    And  stderr names "unknown key"
    And  the change directory still exists under givn/changes/
    And  no archive candidate exists for "broken-declaration"

  @givn.added
  Scenario: A retained root that also covers the archive location is unaffected
    Given a givn project with a completed change "broad-limits" ready to archive
    And  an effective declaration whose limits role names "givn/**"
    When the user runs `givn archive --change broad-limits`
    Then the command exits 0
    And  the change directory has been moved to givn/archive/

  @givn.added
  Scenario: A project with ise off archives without reading the declaration
    Given a givn project with a completed change "ise-off" ready to archive
    And  a declaration whose limits role names "givn/changes/ise-off/design.md" while ise is off
    When the user runs `givn archive --change ise-off`
    Then the command exits 0
    And  the change directory has been moved to givn/archive/
