# User Interaction Inventory:
# - invoke `givn check review --change <id>` and inspect remediation verification of a delta-to-delta candidate in a fresh repository without permanent specification history

@givn.delta @semantic-review
Feature: Verify delta-to-delta semantic remediation

  @givn.added
  Scenario: A first change without permanent specs is gated on its own duplicated pair
    Given the remediation case carries the distinguishing marker "the polar research outpost logs the same geomagnetic surge onto the east-facing chart roll and the west-facing chart roll of the calibration change in a repository that owns no permanent specification history"
    Given a project prepared for semantic remediation with a delta-to-delta "DUPLICATE" candidate on change "rc-unclassified"
    Given the remediation notes record that "the calibration bench ran unattended for eleven full seasons, and the east-tower duty officer transcribed the geomagnetic surge reading onto both chart rolls only because the west-tower gauge unit had been disconnected for a recalibration that nobody scheduled; the archive clerk later stamped both copies with the same observation time despite the fact that the two instruments measure orthogonal field components, a discrepancy the shift supervisor attributed to the storm watch rather than to any deliberate duplication"
    When the user runs `givn check review --change rc-unclassified`
    Then the command exits non-zero
    And the semantic review report contains scope "delta-to-delta"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: A VARIANT pairing stays blocked until the ending is consolidated away
    Given the remediation case carries the distinguishing marker "the coastal lighthouse keeper repeats the identical fog-signal watch entry on the dusk page and the midnight page of the guard log, and the active change has no permanent baseline to draw from"
    Given a project prepared for semantic remediation with a delta-to-delta "VARIANT" candidate on change "rc-var-blocked"
    Given the remediation notes record that "the lighthouse keeper copied the fog-signal watch entry from the dusk page onto the midnight page and initialled both before climbing down from the lantern room, using the same lamp count, the same sounding interval, and the same duty code, while the guard log otherwise would have shown an empty midnight watch for the month"
    When the user runs `givn check review --change rc-var-blocked`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANT"
    And the user runs `givn check review --change rc-var-blocked`
    Then the command exits non-zero
    And stderr contains "VARIANT remediation requires consolidation"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: Folding the variant body into a Scenario Outline passes the first-change gate
    Given the remediation case carries the distinguishing marker "the beekeepers cooperative lists the same hive inspection stamp on both the spring inspection sheet and the autumn inspection sheet while the running change carries two first-time delta scenarios"
    Given a project prepared for semantic remediation with a delta-to-delta "VARIANT" candidate on change "rc-var-consolidated"
    Given the remediation notes record that "the beekeepers ran the spring and autumn hive inspections on the same calendar day because only one inspector held the service key, pressing the self-same inspection stamp onto both record sheets, with the same brood-frame count and the same queen condition noted twice, so the cooperative's ledger contained two entries differing in nothing but the heading of the sheet"
    When the user runs `givn check review --change rc-var-consolidated`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VARIANT"
    And the delta-to-delta variant is consolidated into a parameterized Scenario Outline
    And the user runs `givn check review --change rc-var-consolidated`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added
  Scenario: A boundary pairing without rewording still fails review on the first change
    Given the remediation case carries the distinguishing marker "the tide-driven mill records the same sluice gate reading on the upstream rotation table and the downstream rotation table of the gearbox repair change inside a repository that just started its first change"
    Given a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate on change "rc-boundary-blocked"
    Given the remediation notes record that "the millwright recorded the sluice gate reading on the upstream rotation table, then matched the identical reading onto the downstream rotation table after the gearbox repair had been signed off by the foreman, copying the same water height, the same bearing temperature, and the same rotation speed onto both tables without any note explaining why one station measured twice"
    When the user runs `givn check review --change rc-boundary-blocked`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the user runs `givn check review --change rc-boundary-blocked`
    Then the command exits non-zero
    And stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And the semantic review report status is "BLOCKED"

  @givn.added
  Scenario: An explicit distinction below the boundary lets the first change pass
    Given the remediation case carries the distinguishing marker "the glacier mapping crew writes the same crevasse-stability fix into the inner traverse notebook and the outer traverse notebook of the survey change, with no permanent corpus to anchor either scenario"
    Given a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate on change "rc-boundary-sharpened"
    Given the remediation notes record that "the glacier mapping crew wrote the crevasse-stability value in the inner traverse notebook on the night before the final survey flight, then transcribed the very same number and the same ice-thickness estimate into the outer notebook on the morning of departure, keeping the two notebooks separate but the recorded figures indistinguishable between the two delta endpoints"
    When the user runs `givn check review --change rc-boundary-sharpened`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And the delta-to-delta valid boundary is sharpened with an explicit distinction below the similarity boundary
    And the user runs `givn check review --change rc-boundary-sharpened`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
    And the semantic review report candidate count is 0

  @givn.added @e2e
  Scenario: Real CLI review verifies a duplicate remediated within the first change
    Given the remediation case carries the distinguishing marker "the chamber choir binder books the same hymn arrangement onto the alto practice sheet and the tenor practice sheet ahead of the season's first rehearsal change"
    Given an e2e project staged for a delta-to-delta duplicate remediation on change "rc-e2e"
    Given the remediation notes record that "the choir binder transcribed the hymn arrangement onto the alto practice sheet first, then set the same notes onto the tenor practice sheet while the copy room printer jammed, so the two sheets ended up carrying the same bar numbers, the same tempo marking, and the same rehearsal repeats ahead of the season's first performance"
    When the remediation E2E user runs change review for "rc-e2e"
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
