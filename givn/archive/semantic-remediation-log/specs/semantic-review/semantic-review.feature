@givn.delta @semantic-review
Feature: Semantic review consolidation

  @givn.added
  Scenario: Superseded permanent endpoints leave the semantic pair universe
    Given a project prepared for semantic review with a modified scenario and another candidate in the same target file
    And the project declares a complete run declaration
    When the user runs `givn check review --change running-change`
    Then the command exits non-zero
    And the report excludes the modified scenario's own projected target endpoint
    And the report still contains the other candidate in that target file
    And the report records the modified delta operation and projected target identity
    And the report pairs no live endpoint against a superseded permanent body


  @givn.removed
  Scenario: A complete classification allows change review and survives regeneration
    Given a placeholder step


  @givn.removed
  Scenario: A regenerated classified report is byte-for-byte deterministic
    Given a placeholder step


  @givn.removed
  Scenario: Changing a candidate scenario invalidates its previous classification
    Given a placeholder step


  @givn.removed
  Scenario: Changing a permanent endpoint invalidates its prior classification
    Given a placeholder step


  @givn.removed
  Scenario: Any changed endpoint evidence or prior-report metadata invalidates carry-forward
    Given a placeholder step


  @givn.removed
  Scenario: Removed candidates disappear after classification
    Given a placeholder step


  @givn.removed
  Scenario: A modified delta excludes only its own projected target
    Given a placeholder step


  @givn.removed
  Scenario: Consolidating a VARIANT candidate verifies the projected remediation
    Given a placeholder step


  @givn.removed
  Scenario: Rewriting a VALID-BOUNDARY candidate below the boundary verifies the projected remediation
    Given a placeholder step


  @givn.removed
  Scenario: Folding the variant body into a Scenario Outline passes the first-change gate
    Given a placeholder step


  @givn.modified
  Scenario: Removing a DUPLICATE candidate verifies the projected remediation
    Given the remediation fixture stages a "DUPLICATE" case
    And the permanent specs are snapshotted
    When the review gate runs
    Then the command exits non-zero
    When the resolved duplicate "removed" is recorded ["0a1b2c40"]
    And the classified duplicate is removed
    And the review gate reruns
    Then the review passes with the remediation receipt
    And the permanent specs are unchanged

  @givn.modified
  Scenario: A boundary pairing without rewording still fails review on the first change
    Given  the remediation case carries the distinguishing marker "the tide-driven mill records the same sluice gate reading on the upstream rotation table and the downstream rotation table of the gearbox repair change inside a repository that just started its first change"
    Given  a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate on change "rc-boundary-blocked"
    Given  the remediation notes record that "the millwright recorded the sluice gate reading on the upstream rotation table, then matched the identical reading onto the downstream rotation table after the gearbox repair had been signed off by the foreman, copying the same water height, the same bearing temperature, and the same rotation speed onto both tables without any note explaining why one station measured twice"
    And the project declares a complete run declaration
    When  the user runs `givn check review --change rc-boundary-blocked`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the reviewer logs the fix "kept the boundary pairing" and the commits ["2c3d4e5f"]
    And  the user runs `givn check review --change rc-boundary-blocked`
    Then  the command exits non-zero
    And  stderr contains "VALID-BOUNDARY remediation requires an explicit distinction below the similarity boundary"
    And  the semantic review report status is "BLOCKED"

  @givn.modified
  Scenario: An explicit distinction below the boundary lets the first change pass
    Given  the remediation case carries the distinguishing marker "the glacier mapping crew writes the same crevasse-stability fix into the inner traverse notebook and the outer traverse notebook of the survey change"
    Given  a project prepared for semantic remediation with a delta-to-delta "VALID-BOUNDARY" candidate on change "rc-boundary-sharpened"
    Given  the remediation notes record that "the glacier mapping crew wrote the crevasse-stability value in the inner traverse notebook, then transcribed the same number and the same ice-thickness estimate into the outer notebook on the morning of departure, keeping the recorded figures indistinguishable"
    And the project declares a complete run declaration
    When  the user runs `givn check review --change rc-boundary-sharpened`
    Then  the command exits non-zero
    When  the user classifies every semantic review candidate as "VALID-BOUNDARY"
    And  the delta-to-delta valid boundary is sharpened with an explicit distinction below the similarity boundary
    And  the reviewer logs the fix "sharpened the distinction" and the commits ["3d4e5f60"]
    And  the user runs `givn check review --change rc-boundary-sharpened`
    Then  the command exits 0
    And  stdout contains "semantic remediation verified"
    And  the semantic review report status is "PASS"
    And  the semantic review report candidate count is 0
