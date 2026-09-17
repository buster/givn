@givn.delta @spec-explore-change

Feature: Explore scenarios in the current change

  @givn.added
  Scenario: A tied route reports no signal
    Given a spec discovery project whose corpus contains two capabilities that invoke `givn lint`
    And an active change whose proposal describes extending the lint command
    When the user routes the change with `givn spec route --change`
    Then the command exits 0
    And stdout contains "no signal"
    And stdout contains "tie"
    And stdout does not contain "ranked candidates"

  @givn.added
  Scenario: A goal-aligned capability ranks above a lexically similar candidate
    Given a spec discovery project whose corpus contains a goal-aligned and a goal-mismatched capability
    And an active change whose proposal describes coverage report validation
    When the user routes the change with `givn spec route --change`
    Then the command exits 0
    And stdout contains "recommendation: EXTEND coverage-tool"
    And stdout contains "confidence: signal"

  @givn.removed
  Scenario: A proposal with no matching surface recommends NEW
    Given placeholder

  @givn.added
  Scenario: A proposal with no matching surface reports no signal
    Given a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And an active change whose proposal describes internal widget plumbing with no command surface
    When the user routes the change with `givn spec route --change`
    Then the command exits 0
    And stdout contains "recommendation: no signal"
