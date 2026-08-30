# User Interaction Inventory:
# - invoke `givn check review --change <id>` and inspect remediation verification of a delta-to-delta candidate in a fresh repository without permanent specification history

@givn.delta @semantic-review
Feature: Verify delta-to-delta semantic remediation

  @givn.added
  Scenario: An unclassified delta-to-delta pair blocks review with a current-change scope
    Given the remediation case carries the distinguishing marker "the polar research outpost logs the same geomagnetic surge onto the east-facing chart roll and the west-facing chart roll of the calibration change in a repository that owns no permanent specification history"
    Given a project prepared for semantic remediation with a delta-to-delta "DUPLICATE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the semantic review report contains scope "delta-to-delta"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: A DUPLICATE delta-to-delta classification without removal remains blocked
    Given the remediation case carries the distinguishing marker "the mountain locomotive division book-keeps one identical boiler inspection entry on both the morning crew roster and the evening crew roster of the engine overhaul change while the running change holds its first delta"
    Given a project prepared for semantic remediation with a delta-to-delta "DUPLICATE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLICATE"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "DUPLICATE remediation requires removing the redundant scenario"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Removing the redundant delta-to-delta scenario verifies duplicate remediation
    Given the remediation case carries the distinguishing marker "the citrus export cooperative stamps the same crate batch code into the dispatch column and the receipt column of the packing ledger across the two delta endpoints of the running change"
    Given a project prepared for semantic remediation with a delta-to-delta "DUPLICATE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLICATE"
    And the delta-to-delta duplicate scenario is removed from the active change
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added
  Scenario: A delta-to-delta DUPLICATE pair reworded below the boundary verifies without removal
    Given the remediation case carries the distinguishing marker "the coral reef biodiversity census registers the very same polyp cohort twice, once in the shallow survey quadrant and once in the deep survey quadrant, inside a fresh workspace whose only feature file is the current change"
    Given a project prepared for semantic remediation with a delta-to-delta "DUPLICATE" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLICATE"
    And the delta-to-delta duplicate pair is reworded below the boundary without removal
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added
  Scenario: A VARIANT delta-to-delta classification without consolidation remains blocked
    Given the remediation case carries the distinguishing marker "the coastal lighthouse keeper repeats the identical fog-signal watch entry on the dusk page and the midnight page of the guard log, and the active change has no permanent baseline to draw from"
    Given a project prepared for semantic remediation with a delta-to-delta "VARIANT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANT"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VARIANT remediation requires consolidation"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Consolidating a delta-to-delta variant into a Scenario Outline verifies remediation
    Given the remediation case carries the distinguishing marker "the beekeepers cooperative lists the same hive inspection stamp on both the spring inspection sheet and the autumn inspection sheet while the running change carries two first-time delta scenarios"
    Given a project prepared for semantic remediation with a delta-to-delta "VARIANT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANT"
    And the delta-to-delta variant is consolidated into a parameterized Scenario Outline
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added
  Scenario: A VALID-BOUNDARY delta-to-delta classification without rewording remains blocked
    Given the remediation case carries the distinguishing marker "the tide-driven mill records the same sluice gate reading on the upstream rotation table and the downstream rotation table of the gearbox repair change inside a repository that just started its first change"
    Given a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Sharpening a delta-to-delta boundary below the similarity boundary verifies remediation
    Given the remediation case carries the distinguishing marker "the glacier mapping crew writes the same crevasse-stability fix into the inner traverse notebook and the outer traverse notebook of the survey change, with no permanent corpus to anchor either scenario"
    Given a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the delta-to-delta valid boundary is sharpened with an explicit distinction below the similarity boundary
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
    And the semantic review report candidate count is 0

  @givn.added
  Scenario: A delta-to-delta VALID-BOUNDARY without an explicit distinction remains blocked
    Given the remediation case carries the distinguishing marker "the munitions depot manifest repeats the identical shell-lot count on the intake register and the dispatch register of the inventory change while the change's two own scenarios remain the only specifications in the repository"
    Given a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the delta-to-delta valid boundary is reworded below the boundary without an explicit distinction
    And the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"

  @givn.added @e2e
  Scenario: Real CLI review verifies a delta-to-delta duplicate remediation
    Given the remediation case carries the distinguishing marker "the chamber choir binder books the same hymn arrangement onto the alto practice sheet and the tenor practice sheet ahead of the season's first rehearsal change"
    Given an end-to-end project prepared for a delta-to-delta duplicate remediation
    When the remediation E2E user runs change review for "running-change"
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"