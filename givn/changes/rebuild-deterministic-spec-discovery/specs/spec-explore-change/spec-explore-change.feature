@givn.delta @spec-explore-change
Feature: Explore scenarios in the current change

  @givn.removed
  Scenario: Change-aware file exploration finds permanent and active change candidates

  @givn.removed
  Scenario: Change-aware JSON exploration labels candidate sources

  @givn.removed
  Scenario: Removed change scenarios are absent from exploration results

  @givn.removed
  Scenario: Rule-nested active change scenarios are searchable

  @givn.removed
  Scenario: A modified change scenario is not reported against itself from its delta source

  @givn.removed
  Scenario: Change exploration does not persist delta scenarios in the permanent index

  @givn.removed
  Scenario: An added change scenario is not reported against itself from its delta source

  @givn.removed
  Scenario: A modified change scenario is not reported against itself through its projected permanent target

  @givn.removed
  Scenario: Change-aware exploration normalizes relative and absolute scenario identities

  @givn.removed
  Scenario: Change-aware standard input returns the same ranked candidates as file input

  @givn.removed
  Scenario: Permanent exploration text identifies permanent candidate sources

  @givn.removed
  Scenario: Permanent exploration JSON adds source metadata while retaining legacy result fields

  @givn.removed
  Scenario: Unsafe change identifiers are rejected without changing the permanent index

  @givn.removed
  Scenario: A missing change is rejected without changing the permanent index

  @givn.removed
  Scenario: A change without a specs directory is rejected without changing the permanent index

  @givn.removed
  Scenario: A malformed delta feature is rejected without changing the permanent index

  @givn.added @e2e
  Scenario: Routing a proposal prints ranked candidates with evidence
    Given  a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And    an active change whose proposal describes extending the lint command
    When   the user routes the change with `givn spec route --change`
    Then   the command exits 0
    And    stdout contains "ranked candidates"
    And    stdout contains the capability that invokes `givn lint`
    And    stdout contains an evidence line

  @givn.added
  Scenario: A proposal matching an existing capability surface recommends EXTEND
    Given  a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And    an active change whose proposal describes extending the lint command
    When   the user routes the change with `givn spec route --change`
    Then   the command exits 0
    And    stdout contains "recommendation: EXTEND"

  @givn.added
  Scenario: A proposal with no matching surface recommends NEW
    Given  a spec discovery project whose corpus contains a capability that invokes `givn lint`
    And    an active change whose proposal describes internal widget plumbing with no command surface
    When   the user routes the change with `givn spec route --change`
    Then   the command exits 0
    And    stdout contains "recommendation: NEW"

  @givn.added
  Scenario: Routing requires exactly one input
    Given  a spec discovery project with a non-empty corpus and an active change
    When   the user routes the change with both `--change` and `--file`
    Then   the command exits non-zero
    And    stderr contains "exactly one input"

  @givn.added
  Scenario: Routing never modifies permanent files
    Given  a spec discovery project with a non-empty corpus and an active change
    And    the permanent specifications are snapshotted
    When   the user routes the change with `givn spec route --change`
    Then   the command exits 0
    And    the permanent specifications are unchanged
  @givn.removed
  Scenario: Provider selection rejects an unknown provider

  @givn.removed
  Scenario: Removed provider behavior
