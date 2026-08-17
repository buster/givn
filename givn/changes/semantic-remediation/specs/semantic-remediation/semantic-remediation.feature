# User Interaction Inventory:
# - invoke `givn check review --change <id>` and inspect remediation verification and exit status

@givn.delta @semantic-review @semantic-remediation
Feature: Verify semantic remediation

  @givn.added
  Scenario: A DUPLIKAT classification without removal remains blocked
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "DUPLIKAT remediation requires removing the redundant scenario"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Removing a DUPLIKAT candidate verifies the projected remediation
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    And the permanent specifications are snapshotted
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
    And the permanent specifications are unchanged

  @givn.added
  Scenario: A VARIANTE classification without consolidation remains blocked
    Given a project prepared for semantic remediation with a "VARIANTE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANTE"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VARIANTE remediation requires consolidation"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Consolidating a VARIANTE candidate verifies the projected remediation
    Given a project prepared for semantic remediation with a "VARIANTE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANTE"
    And the active variant is consolidated with an explicit parameter
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added
  Scenario: A VARIANTE score below the boundary without consolidation remains blocked
    Given a project prepared for semantic remediation with a "VARIANTE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANTE"
    And the active variant is rewritten below the boundary without consolidation
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VARIANTE remediation requires consolidation"

  @givn.added
  Scenario: A VALID-BOUNDARY score below the boundary without distinction remains blocked
    Given a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the valid boundary is rewritten below the boundary without an explicit distinction
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"

  @givn.added
  Scenario: A VALID-BOUNDARY classification above the boundary remains blocked
    Given a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation
    Given a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the valid boundary is rewritten with an explicit distinction below the similarity boundary
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added
  Scenario: A VALID-BOUNDARY score exactly at the boundary remains blocked
    Given a project prepared for semantic remediation with a "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the valid boundary is rewritten with an explicit distinction at the similarity boundary
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"

  @givn.added
  Scenario: Changing a classified endpoint before remediation makes the decision stale
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the active classified endpoint changes without remediation
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "semantic classification is stale"

  @givn.added
  Scenario: Failed remediation verification does not mutate project evidence
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    And the permanent specifications are snapshotted
    And the active feature files, review dispositions, and index boundary are snapshotted
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "DUPLIKAT remediation requires removing the redundant scenario"
    And the active feature files are unchanged
    And the permanent specifications are unchanged
    And the review dispositions are unchanged
    And the permanent index contains no delta vectors, classifications, or report state

  @givn.modified
  Scenario: A complete classification allows change review and survives regeneration
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added @e2e @wip
  Scenario: Real CLI review reports successful duplicate remediation
    Given an end-to-end project prepared for a duplicate remediation
    When the end-to-end user runs change review for "running-change"
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
