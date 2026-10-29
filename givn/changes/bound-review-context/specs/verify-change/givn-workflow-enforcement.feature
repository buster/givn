@givn.delta @givn-workflow-enforcement

Feature: givn Workflow Enforcement

  A change records how its review was performed. The default is a self-review
  in the current context. A major or user-requested change may record exactly
  one independent review context, which covers both the design review and the
  final review. Existing design-review and review markers stay required.

  @givn.added @wip
  Scenario: A recorded self-review is accepted
    Given a givn project with a change that has passed design-review
    And the change's proposal records review kind "self-review"
    And the change's design-review records review kind "self-review" by author "agent" in reviewer context "agent"
    And the change's review records review kind "self-review" by author "agent" in reviewer context "agent"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added @wip
  Scenario: An independent review by a different context is accepted
    Given a givn project with a change that has passed design-review
    And the change's proposal records review kind "independent"
    And the change's design-review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And the change's review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added @wip
  Scenario: An independent review whose reviewer is the author is refused
    Given a givn project with a change that has passed design-review
    And the change's proposal records review kind "independent"
    And the change's design-review records review kind "independent" by author "agent" in reviewer context "agent"
    And the change's review records review kind "independent" by author "agent" in reviewer context "agent"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "Reviewer context"
    And stderr contains "must differ from the author"

  @givn.added @wip
  Scenario: A self-review cannot satisfy a recorded independent request
    Given a givn project with a change that has passed design-review
    And the change's proposal records review kind "independent"
    And the change's design-review records review kind "self-review" by author "agent" in reviewer context "agent"
    And the change's review records review kind "self-review" by author "agent" in reviewer context "agent"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "independent review is required"
    And stderr contains "Review kind: self-review"

  @givn.added @wip
  Scenario: The recorded independent choice is authoritative from the proposal
    Given a givn project with a change that has passed design-review
    And the change's proposal records review kind "independent"
    And the change's design-review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And the change's review records review kind "self-review" by author "agent" in reviewer context "agent"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change` twice
    Then both runs exit non-zero
    And stderr contains "independent review is required"

  @givn.added @wip
  Scenario: Design review and final review share one independent context
    Given a givn project with a change that has passed design-review
    And the change's proposal records review kind "independent"
    And the change's design-review records review kind "independent" by author "agent" in reviewer context "reviewer-one"
    And the change's review records review kind "independent" by author "agent" in reviewer context "reviewer-two"
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits non-zero
    And stderr contains "one review context"
    And stderr contains "reviewer-one"
    And stderr contains "reviewer-two"

  @givn.added @wip
  Scenario: A legacy review record without a kind keeps its existing gate result
    Given a givn project with a change that has passed design-review
    And the project declares a complete run declaration
    When the user runs `givn check review --change test-change`
    Then the command exits 0

  @givn.added @wip
  Scenario: The review instruction carries the one-context policy
    Given a givn project
    When the developer reads the review instruction
    Then the review instruction says the default is `Review kind: self-review`
    And the review instruction permits exactly one fresh context for a major change
    And the review instruction says one context covers design review and final review
    And the review instruction forbids more than one review context
    And the review instruction says the reviewer must not be the author

  @givn.added @wip
  Scenario: Guidance assets stop prescribing subagent orchestration
    When the guidance assets are scanned for subagent vocabulary
    Then only assets/instructions/review.md names a review subagent
    And no other guidance asset prescribes a separate agent, session, architect, or reviewer to execute a step
