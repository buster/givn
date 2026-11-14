@givn.delta @apply-a-managed-upgrade

Feature: apply-a-managed-upgrade

  @givn.added @wip
  Scenario: An upgrade writes and stages the declaration when ise is effective and it is absent
    Given  a clean Git project with a committed checkpoint and ise effective and no declaration
    When  the consumer runs `givn upgrade`
    Then  the command exits successfully and reports the managed upgrade revision
    And  exactly one allowlist-only managed revision exists and no migration change is created
    And  the revision contains the path "givn/specification.yaml"
    And  the declaration names exactly the five roles and one source list
    And  the report names the declaration as written

  @givn.added @wip
  Scenario: An upgrade preserves an edited declaration
    Given  a clean Git project with a committed checkpoint and ise effective and a project-edited declaration whose bytes are recorded
    When  the consumer runs `givn upgrade`
    Then  the command exits successfully and reports the managed upgrade revision
    And  the revision carries no diff for the path "givn/specification.yaml"
    And  the declaration bytes equal the recorded bytes
    And  the report names the declaration as project-owned

  @givn.added @wip
  Scenario: An upgrade writes no declaration when ise is not effective
    Given  a clean Git project with a committed checkpoint and ise disabled and a leftover declaration whose bytes are recorded
    When  the consumer runs `givn upgrade`
    Then  the command exits successfully and reports the managed upgrade revision
    And  the revision contains no path "givn/specification.yaml"
    And  the declaration bytes equal the recorded bytes

  @givn.added @wip
  Scenario: Repeating an upgrade after the declaration scaffold creates no second revision
    Given  a clean Git project with a committed checkpoint and ise effective and an upgrade-written declaration
    When  the consumer runs `givn upgrade`
    Then  the command reports the project is current
    And  no second managed revision is created
    And  the declaration bytes are unchanged

  @givn.added @wip
  Scenario: A declaration scaffold write failure reports the path and the completed writes
    Given  a clean Git project with a committed checkpoint and ise effective and the path "givn/specification.yaml" is a directory
    When  the consumer runs `givn upgrade`
    Then  the command exits nonzero and names "givn/specification.yaml"
    And  the report identifies the completed writes and the checkpoint
    And  no complete rollback is promised
