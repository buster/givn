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
