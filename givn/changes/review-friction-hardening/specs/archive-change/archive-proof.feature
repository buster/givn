@givn.delta @archive-proof

Feature: Archive proof reuse across documentation-only revisions

  @givn.added @e2e @wip
  Scenario: Consumer archives a change after documentation-only review edits and the proof is reused
    Given a clean committed change with a green change verification receipt recorded at an earlier revision
    And the developer commits only proposal, design, design-review, tasks, review, arc42, use-case narrative, README, and evidence edits after that revision
    And the verify command records every invocation with its scope and features path
    When the developer runs `givn archive --change review-proof-demo`
    Then the command exits 0
    And the archive is proven
    And the change suite was not invoked again
    And the merged regular scope ran once with features path "givn/specs"
    And stdout says the change verification receipt was reused

  @givn.added
  Scenario: Archive reuses a proof committed with its receipt
    Given a clean committed change with a green change verification receipt recorded at an earlier revision
    And the developer commits only the verification receipt after that revision
    And the verify command records every invocation with its scope and features path
    When the developer runs `givn archive --change review-proof-demo`
    Then the command exits 0
    And the archive is proven
    And the change suite was not invoked again

  @givn.added @wip
  Scenario Outline: Archive refuses a proof after tested content changed
    Given a clean committed change with a green change verification receipt recorded at an earlier revision
    And the <changed content> is changed and committed after that revision
    When the developer runs `givn archive --change review-proof-demo`
    Then the command exits non-zero
    And stderr says the verification receipt is stale
    And stderr names the changed tested content
    And the change directory still exists under givn/changes/
    And no archive candidate exists for "review-proof-demo"

    Examples:
      | changed content |
      | delta feature   |
      | verify command  |
      | source code     |
