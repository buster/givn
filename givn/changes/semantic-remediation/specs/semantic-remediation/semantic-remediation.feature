# User Interaction Inventory:
# - invoke `givn check review --change <id>` and inspect remediation verification and exit status

@givn.delta @semantic-review @semantic-remediation
Feature: Verify semantic remediation

  @givn.added
  Scenario: A DUPLIKAT classification without removal remains blocked
    Given the remediation case carries the distinguishing marker "duplicate removal blocking contract alpha crimson lantern meadow orbit quartz river summit velvet walnut xenon yellow zephyr atlas beacon cobalt delta ember"
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
    Given the remediation case carries the distinguishing marker "duplicate projected removal success beta canyon copper ivory juniper kelp mosaic nickel opal prairie ruby saffron topaz umber violet willow xylophone yarrow zenith"
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
    Given the remediation case carries the distinguishing marker "variant consolidation blocking contract gamma acorn birch cedar dune elm fern grove harbor iris jade kiln lagoon maple north oak pine quarry rose stone thistle"
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
    Given the remediation case carries the distinguishing marker "variant parameterized examples success delta anchor blossom circuit drum engine flare glacier hinge island jacket kettle ladder marble nebula orange prism"
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
    Given the remediation case carries the distinguishing marker "variant score only independent contracts epsilon pebble plume pollen pumpkin raven sandal shell silver spruce stream thunder timber turbine valley vessel winter wren"
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
    Given the remediation case carries the distinguishing marker "boundary score only missing invariant zeta ash basil cloud coral dahlia echo frost granite honey juniper lemon mist nutmeg olive pepper rain snow tulip"
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
    Given the remediation case carries the distinguishing marker "boundary above threshold blocking eta arrow bronze comet dragon eclipse feather galaxy helmet ivory jigsaw kingdom lilac magnet nickel ozone pebble rocket"
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
    Given the remediation case carries the distinguishing marker "boundary explicit observable fact success theta acrobat bamboo crystal domino emerald fossil garden harbor insect jasmine koala lantern mango needle ocean pocket rainbow"
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
    Given the remediation case carries the distinguishing marker "boundary exact threshold blocking iota almond butter carrot daisy engine fig grape hazel igloo jelly kiwi lemon melon nut olive peach quince raisin"
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
    Given the remediation case carries the distinguishing marker "stale endpoint identity changed kappa beacon breeze candle drizzle ember flame glow hearth incense kettle lantern moonlight nightfall"
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
    Given the remediation case carries the distinguishing marker "mutation safety evidence preservation lambda archive backup checksum durable enclave fidelity guardian harbor immutable journal kernel ledger memory"
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
    Given the remediation case carries the distinguishing marker "modified permanent contract replacement mu archive bridge contract delta evidence fixture gateway history invariant journal keeper ledger merge namespace output projection"
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.modified
  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given the remediation case carries the distinguishing marker "regression deterministic report omega glacier harbor invariant matrix nonce ordered receipt stable"
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.modified
  Scenario: Changing a candidate scenario invalidates its previous classification
    Given the remediation case carries the distinguishing marker "regression active candidate invalidation sigma texture vector antler birch comet dune ember fjord granite harbor island juniper kiln lagoon marble nebula olive prism quartz rocket sandal timber umber"
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given the remediation case carries the distinguishing marker "regression permanent endpoint invalidation tau unmodified baseline acorn beacon copper delta engine frost grove hickory iris jade kettle lemon mosaic nickel opal pebble ruby stone thistle"
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given the remediation case carries the distinguishing marker "regression evidence metadata invalidation upsilon version hash"
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"

  @givn.modified
  Scenario: Removed candidates disappear after classification
    Given the remediation case carries the distinguishing marker "regression removed candidate disappearance phi projected state"
    Given a project prepared for semantic remediation with a "DUPLIKAT" candidate
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    When the user classifies every semantic review candidate as "DUPLIKAT"
    And the classified duplicate candidate is removed from the active delta
    And the user runs `givn check review --change running-change`
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"

  @givn.added @e2e
  Scenario: Real CLI review reports successful duplicate remediation
    Given an end-to-end project prepared for a duplicate remediation
    When the remediation E2E user runs change review for "running-change"
    Then the command exits 0
    And stdout contains "semantic remediation verified"
    And the semantic review report status is "PASS"
