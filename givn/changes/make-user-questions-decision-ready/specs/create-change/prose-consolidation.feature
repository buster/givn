@givn.delta @prose-consolidation
Feature: One effective dialog policy across generated authoring guidance

  # Interaction inventory: specs/create-change/usecase.md.
  # Inspect synchronized guidance -> Generated guidance points to the canonical E2E policy
  # This existing action covers generated dialog references too; addon and delegation cases are invariant variants.

  @givn.modified @e2e Scenario: Generated guidance points to the canonical E2E policy
    Given a Givn project with all agent skill targets and optional guidance enabled
    When the consumer synchronizes the selected agent guidance through the Givn CLI
    Then every emitted skill and command requires the effective dialog policy before its context asks a user-facing question
    And direct skill invocation has the same obligation as session-start invocation
    And reference skills do not narrow the obligation to one workflow phase
    And the generated guidance preserves its references to the canonical interaction and E2E policy
    And detailed dialog rules and worked examples remain in the effective dialog instructions rather than competing generated copies
    And the synchronization result reports the selected guidance successfully

  @givn.removed Scenario: Generated interactive guidance points to the canonical dialog policy
    Given the limited dialog-reference assertion is superseded by the generated-guidance consumer contract

  @givn.added Scenario: Disabling optional addons does not disable the dialog obligation
    Given a Givn project with all agent skill targets selected and every optional addon disabled
    When the author synchronizes the selected guidance through the Givn CLI
    Then every available core skill and command still requires the effective dialog policy for every user-facing question
    And guidance for disabled addons is not installed
    And the core dialog obligation does not depend on the optional tone policy
    And the selected targets and addon choices remain unchanged

  @givn.added Scenario: Delegated question preparation carries the policy without assuming inherited context
    Given a Givn project with delegated authoring and review guidance enabled
    When the author reads the synchronized delegation instructions
    Then every question-preparing delegation explicitly requires the effective dialog policy in the fresh context
    And the delegation instructions do not assume the parent's loaded policy is inherited
    And the presenting agent remains responsible for the question before it reaches the person
    And a question-tool recommendation or option label does not replace context and answer consequences

  @givn.added Scenario: Successful project skill and command replacements are preserved without policy injection
    Given a Givn project with all agent skill targets selected
    And valid project-owned skill and command replacements intentionally omit the bundled dialog obligation
    When the author synchronizes the selected guidance through the Givn CLI
    Then the project-owned skill and command source bytes remain unchanged
    And every selected skill consumer receives the authoritative rendered replacement without a bundled prefix
    And the opencode command consumer receives the authoritative rendered replacement without a bundled prefix
    And the managed project-start guidance still references the effective dialog policy
