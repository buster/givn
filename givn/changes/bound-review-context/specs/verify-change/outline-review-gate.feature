@givn.delta @outline-review-gate

Feature: Outline review gate

  @givn.added
  Scenario: A self-review outline uses the self context
    Given a Givn project with the outline experiment enabled
    And a committed change that adds a type of 40 code lines in "src/engine.rs"
    And the change's review record records "Review kind: self-review"
    And the change's review record records "Author: agent"
    And the change's review record records "Reviewer context: agent"
    And the change's review record records "OUTLINE: AUTHOR agent"
    And the change's review record records "OUTLINE: REVIEWER agent"
    And the change's review record records "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And the change's review record records "OUTLINE: PASS"
    When the outline review assesses the change
    Then the review passes

  @givn.added
  Scenario: An independent outline uses the recorded reviewer context
    Given a Givn project with the outline experiment enabled
    And a committed change that adds a type of 40 code lines in "src/engine.rs"
    And the change's review record records "Review kind: independent"
    And the change's review record records "Author: agent"
    And the change's review record records "Reviewer context: reviewer-one"
    And the change's review record records "OUTLINE: AUTHOR agent"
    And the change's review record records "OUTLINE: REVIEWER reviewer-one"
    And the change's review record records "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And the change's review record records "OUTLINE: PASS"
    When the outline review assesses the change
    Then the review passes

  @givn.added
  Scenario: An outline reviewer that differs from the recorded context is refused
    Given a Givn project with the outline experiment enabled
    And a committed change that adds a type of 40 code lines in "src/engine.rs"
    And the change's review record records "Review kind: self-review"
    And the change's review record records "Author: agent"
    And the change's review record records "Reviewer context: agent"
    And the change's review record records "OUTLINE: AUTHOR agent"
    And the change's review record records "OUTLINE: REVIEWER fresh-context-reviewer"
    And the change's review record records "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And the change's review record records "OUTLINE: PASS"
    When the outline review assesses the change
    Then the review fails
    And the review says "does not match the recorded Reviewer context"

  @givn.added
  Scenario: An outline author that differs from the recorded author is refused
    Given a Givn project with the outline experiment enabled
    And a committed change that adds a type of 40 code lines in "src/engine.rs"
    And the change's review record records "Review kind: self-review"
    And the change's review record records "Author: agent"
    And the change's review record records "Reviewer context: agent"
    And the change's review record records "OUTLINE: AUTHOR impostor"
    And the change's review record records "OUTLINE: REVIEWER agent"
    And the change's review record records "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And the change's review record records "OUTLINE: PASS"
    When the outline review assesses the change
    Then the review fails
    And the review says "does not match the recorded Author"

  @givn.added
  Scenario: A legacy outline without a review kind keeps the reviewer-differs rule
    Given a Givn project with the outline experiment enabled
    And a committed change that adds a type of 40 code lines in "src/engine.rs"
    And the change's review record records "OUTLINE: AUTHOR agent"
    And the change's review record records "OUTLINE: REVIEWER fresh-context-reviewer"
    And the change's review record records "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And the change's review record records "OUTLINE: PASS"
    When the outline review assesses the change
    Then the review passes

  @givn.added
  Scenario: A legacy outline with the author as reviewer is refused
    Given a Givn project with the outline experiment enabled
    And a committed change that adds a type of 40 code lines in "src/engine.rs"
    And the change's review record records "OUTLINE: AUTHOR agent"
    And the change's review record records "OUTLINE: REVIEWER agent"
    And the change's review record records "OUTLINE: PASS type:src/engine.rs::Engine | one responsibility; the calls are clear"
    And the change's review record records "OUTLINE: PASS"
    When the outline review assesses the change
    Then the review fails
    And the review says "must differ from the author"
