Feature: review-disposition-gates

  Scenario: Review blocks when a long scenario in the delta has no split-or-keep entry
    Given a givn project with a change that has passed design-review
    And a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
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
    And the change's review.md has no disposition entries
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "split-or-keep"
    And stderr contains "The undisposed marathon"

  @review-disposition-gates
  Scenario: Review passes when a long scenario has a split-or-keep entry
    Given  a change contains a scenario exceeding the scenario-length limit
    And  its review explicitly records "keep" for that scenario with the brief reason "one atomic import requires all setup rows"
    And  all other applicable review evidence passes
    When  the developer runs `givn check review --change test-change`
    Then  the command succeeds

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
        @e2e
        Scenario: Solo delta
          Given x
          When y
          Then nothing asserted
      """
    And the change's review.md has no disposition entries
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @review-disposition-gates
  Scenario: Review blocks with a clear message when review.md is missing
    Given  a change has current applicable verification evidence and passes all other applicable integrity checks
    And  the change has no review.md and therefore no required review verdict evidence
    When  the developer runs `givn check review --change test-change`
    Then  the command exits non-zero and names review.md and the missing required review verdict evidence
    And  no shape disposition is required

  Scenario: The net-delta receipt counts added, modified, and removed scenarios
    Given a givn project with a change that has passed design-review
    And a .feature file at "givn/changes/test-change/specs/cap/cap.feature" with:
      """
      Feature: Cap
        @e2e
        Scenario: Receipt added
          Given x
      """
    And a .feature file at "givn/specs/other/other.feature" with:
      """
      Feature: Other
        Scenario: Receipt modified
          Given x
        Scenario: Receipt removed
          Given x
      """
    And a .feature file at "givn/changes/test-change/specs/other/other.feature" with:
      """
      Feature: Other
        @givn.modified
        Scenario: Receipt modified
          Given x
        @givn.removed
        Scenario: Receipt removed
          Given removed placeholder
      """
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0
    And stdout contains "net delta"
    And stdout contains "1 added"
    And stdout contains "1 modified"
    And stdout contains "1 removed"

  @review-disposition-gates
  Scenario: Consumer passes review without shape dispositions and receives the net-delta receipt
    Given  a change has two short added scenarios with matching shapes and distinct titles
    And  the completed review has no shape disposition table
    And  applicable verification and integrity evidence passes
    When  the consumer invokes `givn check review --change test-change`
    Then  the command succeeds
    And  terminal output reports a net delta of 2 added scenarios

  @review-disposition-gates
  Scenario Outline: Shape matches do not block review or archive
    Given  an otherwise valid completed change has a short delta scenario shape-matching <matching scenario>
    And  its review has no shape disposition entries
    When  the developer runs <command>
    Then  the command succeeds without requiring a shape disposition
    Examples:
      | matching scenario | command |
      | another delta scenario | `givn check review --change test-change` |
      | a permanent scenario | `givn check review --change test-change` |
      | another delta scenario | `givn archive --change test-change` |
      | a permanent scenario | `givn archive --change test-change` |

  @review-disposition-gates
  Scenario Outline: A keep disposition must contain an explicit decision and a reason
    Given  a change contains a scenario exceeding the scenario-length limit
    And  its review contains <entry> for that scenario
    When  the developer runs `givn check review --change test-change`
    Then  the command exits non-zero and explains the missing split-or-keep evidence
    Examples:
      | entry |
      | a scenario name without a split or keep decision |
      | a keep decision without a brief reason |

  @review-disposition-gates
  Scenario: A promised split cannot pass review while the long scenario remains unsplit
    Given  a change contains a scenario exceeding the scenario-length limit
    And  its review records a split decision for that scenario
    And  the chosen split is not completed in the delta specs
    When  the developer runs `givn check review --change test-change`
    Then  the command exits non-zero and names the incomplete split

  @review-disposition-gates
  Scenario: A completed split satisfies the scenario-length disposition
    Given  a review records a split decision for a long scenario
    And  the delta specs complete that split into independently focused scenarios within the length limit
    And  all other applicable review evidence passes
    When  the developer runs `givn check review --change test-change`
    Then  the command succeeds without requiring a keep reason for the completed split
