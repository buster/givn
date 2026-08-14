# User Interaction Inventory:
# - Run `givn check review --change <id>` on a change with overlap findings

@givn.delta @review-disposition-gates

Feature: review-disposition-gates

  @givn.added @wip @e2e
  Scenario: Review passes when every shape match is disposed and reports the net-delta receipt
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Reviewed shape one
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
        Scenario: Reviewed shape two
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  the change's review.md contains a disposition row naming "Reviewed shape one" and "Reviewed shape two"
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  stdout contains "net delta"
    And  stdout contains "2 added"

  @givn.added
  Scenario: Review blocks when a shape match involving the change's delta is undisposed
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Undisposed shape one
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
        Scenario: Undisposed shape two
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  the change's review.md has no disposition entries
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "disposition"
    And  stderr contains "Undisposed shape one"
    And  stderr contains "Undisposed shape two"

  @givn.added
  Scenario: Review blocks when a long scenario in the delta has no split-or-keep entry
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: The undisposed marathon
          Given a project root with givn/ initialised
          And step one
          And step two
          And step three
          And step four
          And step five
          And step six
          And step seven
          And step eight
          And step nine
          And step ten
          And step eleven
          And step twelve
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  the change's review.md has no disposition entries
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "split-or-keep"
    And  stderr contains "The undisposed marathon"

  @givn.added
  Scenario: Review passes when a long scenario has a split-or-keep entry
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: The disposed marathon
          Given a project root with givn/ initialised
          And step one
          And step two
          And step three
          And step four
          And step five
          And step six
          And step seven
          And step eight
          And step nine
          And step ten
          And step eleven
          And step twelve
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  the change's review.md contains a split-or-keep row for "The disposed marathon"
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  @givn.added
  Scenario: Review blocks when a delta scenario shape-matches a permanent scenario without disposition
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/specs/ex/ex.feature" with:
      """
      Feature: Ex
        Scenario: Permanent twin
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Delta twin
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  the change's review.md has no disposition entries
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "Permanent twin"
    And  stderr contains "Delta twin"

  @givn.added
  Scenario: Review passes when a shape match exists only between permanent scenarios
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/specs/one/one.feature" with:
      """
      Feature: One
        Scenario: Permanent shape one
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  a .feature file at "givn/specs/two/two.feature" with:
      """
      Feature: Two
        Scenario: Permanent shape two
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Solo delta
          Given x
          When y
          Then nothing asserted
      """
    And  the change's review.md has no disposition entries
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0

  @givn.added @wip
  Scenario: Review blocks with a clear message when review.md is missing
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Reviewless shape one
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
        Scenario: Reviewless shape two
          Given a project root with givn/ initialised
          When the user runs `givn lint`
          Then the command exits 0
      """
    When  the user runs `givn check review --change test-change`
    Then  the command exits non-zero
    And  stderr contains "review.md"

  @givn.added
  Scenario: The net-delta receipt counts added, modified, and removed scenarios
    Given  a givn project with a change that has passed design-review
    And  a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        Scenario: Receipt added
          Given x
      """
    And  a .feature file at "givn/changes/test-change/specs/other/other.feature" with:
      """
      Feature: Other
        @givn.modified
        Scenario: Receipt modified
          Given x
        @givn.removed
        Scenario: Receipt removed
          Given removed placeholder
      """
    When  the user runs `givn check review --change test-change`
    Then  the command exits 0
    And  stdout contains "net delta"
    And  stdout contains "1 added"
    And  stdout contains "1 modified"
    And  stdout contains "1 removed"
