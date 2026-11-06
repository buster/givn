@givn.delta @check-specification-health

Feature: check-specification-health

  @givn.added
  Scenario: Change-scoped lint reports an E2E scenario the projected inventory does not declare
    Given a project with a permanent fragment whose interaction inventory declares "Consumer submits an order"
    And a change "projected-change" whose delta adds an E2E scenario "Consumer cancels an order" without an interaction row
    When the user runs `givn lint --change projected-change`
    Then the command exits 1
    And stderr contains "interaction inventory mismatch"
    And stderr contains "use case 'fragments'"
    And stderr contains "capability 'place-order'"
    And stderr contains "unlisted @e2e scenario 'Consumer cancels an order'"

  @givn.added
  Scenario: Change-scoped lint reports a declared interaction whose projected corpus has no E2E scenario
    Given a project with a permanent fragment whose interaction inventory declares "Consumer submits an order"
    And a change "projected-change" whose delta removes the only E2E scenario for "Consumer submits an order"
    When the user runs `givn lint --change projected-change`
    Then the command exits 1
    And stderr contains "interaction inventory mismatch"
    And stderr contains "use case 'fragments'"
    And stderr contains "capability 'place-order'"
    And stderr contains "missing @e2e scenario 'Consumer submits an order'"

  @givn.added
  Scenario: Change-scoped lint reports a feature the projected inventory does not declare
    Given a project with a permanent fragment whose interaction inventory declares "Consumer submits an order"
    And a change "projected-change" whose delta adds an E2E scenario "Consumer cancels an order" under an undeclared capability "cancellations"
    When the user runs `givn lint --change projected-change`
    Then the command exits 1
    And stderr contains "use case 'fragments'"
    And stderr contains "undeclared capability 'cancellations'"
    And stderr contains "Consumer cancels an order"

  @givn.added
  Scenario: Default lint reports an undeclared capability
    Given a project with a permanent fragment whose interaction inventory declares "Consumer submits an order"
    And the permanent corpus contains an E2E scenario "Consumer cancels an order" in an undeclared capability "cancellations"
    When the user runs `givn lint`
    Then the command exits 1
    And stderr contains "undeclared capability 'cancellations'"

  @givn.added
  Scenario: Change-scoped lint reports a delta capability with no projected owner
    Given a project root with givn/ initialised
    And a raw delta feature at "givn/changes/projected-change/specs/cap/cap.feature" with:
      """
      @givn.delta @cap
      Feature: Cap
        @givn.added @cap
        Scenario: Unplaced
          Given x
      """
    When the user runs `givn lint --change projected-change`
    Then the command exits 1
    And stderr contains "cannot be placed in the projected corpus"
    And stderr contains "capability 'cap'"

  @givn.added
  Scenario: Change-scoped lint accepts a delta that keeps the projected inventory consistent
    Given a project with a permanent fragment whose interaction inventory declares "Consumer submits an order"
    And a change "projected-change" whose delta adds an E2E scenario "Consumer cancels an order" with its interaction row
    When the user runs `givn lint --change projected-change`
    Then the command exits 0
    And stdout contains "clean"

  @givn.added
  Scenario: Change-scoped lint reports a pre-existing inventory mismatch
    Given a project with a permanent fragment whose interaction inventory declares "Consumer submits an order"
    And the permanent corpus contains an unlisted E2E scenario "Consumer cancels an order"
    And a change "projected-change" with no delta features
    When the user runs `givn lint --change projected-change`
    Then the command exits 1
    And stderr contains "unlisted @e2e scenario 'Consumer cancels an order'"
