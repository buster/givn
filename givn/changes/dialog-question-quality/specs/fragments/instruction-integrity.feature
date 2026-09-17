@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The canonical dialog policy is retrievable through the Givn CLI
    Given a Givn project with an active change for the dialog policy
    When the author requests the dialog policy through the Givn CLI
    Then the CLI output contains the named-artifact rule
    And the CLI output contains the option treatment rule
    And the CLI output contains the standalone-answerability rule
    And the CLI output contains the rejected-alternative rule

  @givn.added
  Scenario: The canonical dialog policy demonstrates the full question record
    Given the canonical dialog policy is requested
    When an author reads the option treatment and the record shape
    Then the policy requires every option's meaning gain cost and concrete example
    And the policy requires the rejected alternatives with their reasons
    And the policy rejects an outcome-only resolution
    And the policy requires a gloss for every cross-reference

  @givn.added @wip
  Scenario: The dialog policy is retrievable without an active change
    Given a project root with givn/ initialised
    When the author requests the dialog policy through the Givn CLI without a change
    Then the command exits 0
    And the CLI output contains the named-artifact rule

  @givn.added @wip
  Scenario: An unresolvable instruction id keeps the known-artifact error
    Given a project root with givn/ initialised
    When the author requests an unknown instruction through the Givn CLI
    Then the command fails
    And stderr contains "unknown artifact"

  @givn.added @wip
  Scenario: The design-review template carries the per-question record shape
    Given a givn project
    When the user runs `givn eject design-review:template`
    Then the ejected design-review template contains "Branch"
    And the ejected design-review template contains "Outcome"
    And the ejected design-review template contains "In practice:"
    And the ejected design-review template contains "Recommended:"
    And the ejected design-review template contains "Rejected:"
