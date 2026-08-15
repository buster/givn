@givn.delta @review-disposition-gates
Feature: review-disposition-gates

  @givn.modified @wip
  Scenario: Review passes when a shape match exists only between permanent scenarios
    Given a givn project with a change that has passed design-review
    And a .feature file at "givn/specs/one/one.feature" with:
      """
      Feature: One
        Scenario: Permanent shape one
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And a .feature file at "givn/specs/two/two.feature" with:
      """
      Feature: Two
        Scenario: Permanent shape two
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Solo delta
          Given x
          When y
          Then nothing asserted
      """
    And the change's review.md has no disposition entries
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "similarity gate failed"
