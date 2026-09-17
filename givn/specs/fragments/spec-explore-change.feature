Feature: Explore scenarios in the current change

  @e2e
  Scenario: Routing a proposal prints ranked candidates with evidence
    Given a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And an active change whose proposal describes extending the lint command
    When the user routes the change with `givn spec route --change`
    Then the command exits 0
    And stdout contains "ranked candidates"
    And stdout contains the capability that invokes `givn lint`
    And stdout contains an evidence line

  Scenario: A proposal matching an existing capability surface recommends EXTEND
    Given a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And an active change whose proposal describes extending the lint command
    When the user routes the change with `givn spec route --change`
    Then the command exits 0
    And stdout contains "recommendation: EXTEND"

  Scenario: Routing requires exactly one input
    Given a spec discovery project with a non-empty corpus and an active change
    When the user routes the change with both `--change` and `--file`
    Then the command exits non-zero
    And stderr contains "exactly one input"

  Scenario: Routing never modifies permanent files
    Given a spec discovery project with a non-empty corpus and an active change
    And the permanent specifications are snapshotted
    When the user routes the change with `givn spec route --change`
    Then the command exits 0
    And the permanent specifications are unchanged
  Scenario: A tied route reports no signal
    Given  a spec discovery project whose corpus contains two capabilities that invoke `givn lint`
    And  an active change whose proposal describes extending the lint command
    When  the user routes the change with `givn spec route --change`
    Then  the command exits 0
    And  stdout contains "no signal"
    And  stdout contains "tie"
    And  stdout does not contain "ranked candidates"

  Scenario: A goal-aligned capability ranks above a lexically similar candidate
    Given  a spec discovery project whose corpus contains a goal-aligned and a goal-mismatched capability
    And  an active change whose proposal describes coverage report validation
    When  the user routes the change with `givn spec route --change`
    Then  the command exits 0
    And  stdout contains "recommendation: EXTEND coverage-tool"
    And  stdout contains "confidence: signal"

  Scenario: A proposal with no matching surface reports no signal
    Given  a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And  an active change whose proposal describes internal widget plumbing with no command surface
    When  the user routes the change with `givn spec route --change`
    Then  the command exits 0
    And  stdout contains "recommendation: no signal"
