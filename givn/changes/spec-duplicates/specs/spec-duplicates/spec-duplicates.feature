# User Interaction Inventory:
# - invoke `givn spec duplicates` and consume the complete advisory audit, including its JSON output variant

@givn.delta @spec-duplicates

Feature: Audit duplicate and related permanent scenarios

  @givn.added @e2e @wip
  Scenario: Maintainer inspects the complete permanent scenario audit
    Given an end-to-end project with permanent scenarios containing duplicate and related behaviour
    When the end-to-end user runs `givn spec duplicates`
    Then the command exits 0
    And stdout identifies every duplicate and related-scenario candidate

  @givn.added
  Scenario: Exact duplicates in one feature are reported together
    Given a permanent feature contains two scenarios with identical behaviour
    When the user runs `givn spec duplicates`
    Then the audit classifies the pair as an exact duplicate
    And the audit identifies both scenarios

  @givn.added @wip
  Scenario: Exact duplicates across features are reported together
    Given two permanent features contain scenarios with identical behaviour
    When the user runs `givn spec duplicates`
    Then the audit classifies the pair as an exact duplicate
    And the audit identifies both feature files

  @givn.added @wip
  Scenario: Structurally overlapping scenarios retain their distinguishing facts
    Given permanent scenarios share an action structure but assert different facts
    When the user runs `givn spec duplicates`
    Then the audit classifies the pair as a structural overlap
    And the audit shows the facts that distinguish the scenarios

  @givn.added @wip
  Scenario: Semantically related scenarios are reported as candidates
    Given permanent scenarios describe closely related behaviour with different wording
    When the user runs `givn spec duplicates`
    Then the audit classifies the pair as a semantic candidate
    And the audit identifies both scenarios for maintainer review

  @givn.added @wip
  Scenario: The audit reports candidates beyond the interactive top five
    Given the permanent corpus contains more than five duplicate and related-scenario candidates
    When the user runs `givn spec duplicates`
    Then the audit reports every candidate in the corpus
    And the audit includes a candidate beyond rank 5

  @givn.added @wip
  Scenario: Repeating an unchanged audit produces the same report
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    When the user runs `givn spec duplicates` twice without changing the corpus
    Then both audit reports have the same ordering and content

  @givn.added @wip
  Scenario: JSON output identifies every audit candidate
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    When the user runs `givn spec duplicates --format json`
    Then stdout is a valid JSON audit document
    And every JSON candidate has a classification and both scenario identities
    And the JSON candidates follow the deterministic audit ordering

  @givn.added @wip
  Scenario: Auditing leaves permanent specifications unchanged
    Given a permanent scenario corpus with duplicate and related-scenario candidates
    And the permanent specifications are snapshotted
    When the user runs `givn spec duplicates`
    Then the permanent specifications are unchanged

  @givn.added @wip
  Scenario: Active change scenarios are not part of the permanent audit
    Given a permanent scenario corpus with one related scenario
    And an active change contains another scenario with related behaviour
    When the user runs `givn spec duplicates`
    Then the audit reports only relationships between permanent scenarios
    And the active change scenario is absent from the audit

  @givn.added @wip
  Scenario: An audit with no candidates reports an empty result
    Given a permanent scenario corpus with no duplicate or related behaviour
    When the user runs `givn spec duplicates`
    Then the command exits 0
    And the audit reports that no candidates were found
