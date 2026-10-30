@givn.delta @review-disposition-gates
Feature: Review disposition gates

  @givn.removed
  Scenario: Review passes when every shape match is disposed and reports the net-delta receipt
    Given this scenario is removed

  @givn.removed
  Scenario: Review blocks when a shape match involving the change's delta is undisposed
    Given this scenario is removed

  @givn.removed
  Scenario: Review blocks when a delta scenario shape-matches a permanent scenario without disposition
    Given this scenario is removed

  @givn.added
  Scenario: Consumer passes review without shape dispositions and receives the net-delta receipt
    Given a change has two short added scenarios with matching shapes and distinct titles
    And the completed review has no shape disposition table
    And applicable verification and integrity evidence passes
    When the consumer invokes `givn check review --change test-change`
    Then the command succeeds
    And terminal output reports a net delta of 2 added scenarios

  @givn.added
  Scenario Outline: Shape matches do not block review or archive
    Given an otherwise valid completed change has a short delta scenario shape-matching <matching scenario>
    And its review has no shape disposition entries
    When the developer runs <command>
    Then the command succeeds without requiring a shape disposition
    Examples:
      | matching scenario | command |
      | another delta scenario | `givn check review --change test-change` |
      | a permanent scenario | `givn check review --change test-change` |
      | another delta scenario | `givn archive --change test-change` |
      | a permanent scenario | `givn archive --change test-change` |

  @givn.modified
  Scenario: Review blocks with a clear message when review.md is missing
    Given a change has current applicable verification evidence and passes all other applicable integrity checks
    And the change has no review.md and therefore no required review verdict evidence
    When the developer runs `givn check review --change test-change`
    Then the command exits non-zero and names review.md and the missing required review verdict evidence
    And no shape disposition is required

  @givn.modified
  Scenario: Review passes when a long scenario has a split-or-keep entry
    Given a change contains a scenario exceeding the scenario-length limit
    And its review explicitly records "keep" for that scenario with the brief reason "one atomic import requires all setup rows"
    And all other applicable review evidence passes
    When the developer runs `givn check review --change test-change`
    Then the command succeeds

  @givn.added
  Scenario Outline: A keep disposition must contain an explicit decision and a reason
    Given a change contains a scenario exceeding the scenario-length limit
    And its review contains <entry> for that scenario
    When the developer runs `givn check review --change test-change`
    Then the command exits non-zero and explains the missing split-or-keep evidence
    Examples:
      | entry |
      | a scenario name without a split or keep decision |
      | a keep decision without a brief reason |

  @givn.added
  Scenario: A promised split cannot pass review while the long scenario remains unsplit
    Given a change contains a scenario exceeding the scenario-length limit
    And its review records a split decision for that scenario
    And the chosen split is not completed in the delta specs
    When the developer runs `givn check review --change test-change`
    Then the command exits non-zero and names the incomplete split

  @givn.added
  Scenario: A completed split satisfies the scenario-length disposition
    Given a review records a split decision for a long scenario
    And the delta specs complete that split into independently focused scenarios within the length limit
    And all other applicable review evidence passes
    When the developer runs `givn check review --change test-change`
    Then the command succeeds without requiring a keep reason for the completed split
