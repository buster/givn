# User Interaction Inventory:
# - invoke `givn spec explore --change <id> --file <draft.feature>` and read source-labeled text results
# - invoke `givn spec explore --change <id> --file <draft.feature> --format json` and consume source metadata

@givn.delta @spec-explore-change
Feature: Explore scenarios in the current change

  @givn.added @e2e @wip
  Scenario: Change-aware file exploration finds permanent and active change candidates
    Given an end-to-end project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the end-to-end user explores "draft.feature" against change "current-change"
    Then the command exits 0
    And stdout contains a permanent candidate titled "Provider name is unknown"
    And stdout contains a current-change candidate titled "Current change rejects an unknown provider"
    And stdout identifies the permanent candidate with scope "permanent"
    And stdout identifies the permanent candidate source as "givn/specs/provider.feature"
    And stdout identifies the current-change candidate with scope "current-change" and change "current-change"
    And stdout identifies the current-change candidate source as "givn/changes/current-change/specs/current-change/current-change.feature"

  @givn.added @e2e @wip
  Scenario: Change-aware JSON exploration labels candidate sources
    Given an end-to-end project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the end-to-end user explores "draft.feature" against change "current-change" with JSON output
    Then the command exits 0
    And stdout is a change exploration JSON document with permanent and current-change results
    And every JSON result carries its scope and owning change metadata

  @givn.added
  Scenario: Removed change scenarios are absent from exploration results
    Given a project prepared for current-change exploration with a removed scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Removed provider behavior
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" against change "current-change"
    Then the command exits 0
    And no ranked result is titled "Removed provider behavior"

  @givn.added @wip
  Scenario: A modified change scenario is not reported against itself from its delta source
    Given a project prepared for current-change exploration with a modified scenario
    When the user explores "givn/changes/current-change/specs/current-change/current-change.feature" against change "current-change"
    Then the command exits 0
    And no ranked result is titled "Provider name is unknown"
    And another eligible candidate remains visible

  @givn.added @wip
  Scenario: Change exploration does not persist delta scenarios in the permanent index
    Given a project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    And the permanent exploration index is built
    And the permanent exploration index is snapshotted
    When the user explores "draft.feature" against change "current-change"
    Then the command exits 0
    And the permanent exploration index is unchanged

  @givn.added @wip
  Scenario: An added change scenario is not reported against itself from its delta source
    Given a project prepared for current-change exploration with an active added scenario in a one-scenario delta file
    When the user explores "givn/changes/current-change/specs/current-change/added-provider.feature" against change "current-change"
    Then the command exits 0
    And no ranked result is titled "Current change rejects an unknown provider"
    And another eligible candidate remains visible

  @givn.added @wip
  Scenario: A modified change scenario is not reported against itself through its projected permanent target
    Given a project prepared for current-change exploration with a modified scenario and a separate eligible candidate
    When the user explores "givn/specs/current-change/current-change.feature" against change "current-change"
    Then the command exits 0
    And no ranked result is titled "Provider name is unknown"
    And another eligible candidate remains visible

  @givn.added @wip
  Scenario: Change-aware exploration normalizes relative and absolute scenario identities
    Given a project prepared for current-change exploration with a modified scenario
    When the user explores the modified delta scenario once by its relative path and once by its absolute project path against change "current-change"
    Then both requests exit 0
    And neither result is titled "Provider name is unknown"
    And both requests retain another eligible candidate

  @givn.added @wip
  Scenario: Change-aware standard input returns the same ranked candidates as file input
    Given a project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" against change "current-change" with the file input
    And the user explores the same draft against change "current-change" through standard input
    Then the standard-input ranked results equal the file ranked results

  @givn.added @wip
  Scenario: Permanent exploration text identifies permanent candidate sources
    Given a project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" with the file input
    Then the command exits 0
    And stdout contains a permanent candidate titled "Provider name is unknown"
    And stdout identifies the permanent candidate with scope "permanent"
    And stdout identifies the permanent candidate source as "givn/specs/provider.feature"

  @givn.added @wip
  Scenario: Permanent exploration JSON adds source metadata while retaining legacy result fields
    Given a project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    When the user explores "draft.feature" with JSON output
    Then the command exits 0
    And stdout is a permanent exploration JSON document
    And every JSON result retains rank title file and score fields in their PR1 order
    And every JSON result has scope "permanent" and no owning change

  @givn.added @wip
  Scenario: Unsafe change identifiers are rejected without changing the permanent index
    Given a project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    And the permanent exploration index is built
    And the permanent exploration index is snapshotted
    When the user explores "draft.feature" against unsafe change "../etc"
    Then the command exits non-zero
    And stderr clearly says the change id must be kebab-case
    And the permanent exploration index is unchanged

  @givn.added @wip
  Scenario: A missing change is rejected without changing the permanent index
    Given a project prepared for current-change exploration with an active added scenario
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    And no change named "missing-change" exists
    And the permanent exploration index is built
    And the permanent exploration index is snapshotted
    When the user explores "draft.feature" against change "missing-change"
    Then the command exits non-zero
    And stderr clearly says change "missing-change" was not found
    And the permanent exploration index is unchanged

  @givn.added @wip
  Scenario: A change without a specs directory is rejected without changing the permanent index
    Given a project prepared for current-change exploration with change "empty-change" but no specs directory
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    And the permanent exploration index is built
    And the permanent exploration index is snapshotted
    When the user explores "draft.feature" against change "empty-change"
    Then the command exits non-zero
    And stderr clearly says change "empty-change" has no specs directory
    And the permanent exploration index is unchanged

  @givn.added @wip
  Scenario: A malformed delta feature is rejected without changing the permanent index
    Given a project prepared for current-change exploration with a malformed delta feature
    And a .feature file at "draft.feature" with:
      """
      Feature: Draft exploration

        Scenario: Provider selection rejects an unknown provider
          Given a configured provider catalog
          When the user selects an unknown provider
          Then the command rejects the request
      """
    And the permanent exploration index is built
    And the permanent exploration index is snapshotted
    When the user explores "draft.feature" against change "current-change"
    Then the command exits non-zero
    And stderr clearly says the delta feature is not valid Gherkin
    And the permanent exploration index is unchanged
