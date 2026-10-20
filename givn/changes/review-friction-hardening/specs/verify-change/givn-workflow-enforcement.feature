@givn.delta @givn-workflow-enforcement

Feature: Review diagnostics and proof reuse

  @givn.added @e2e
  Scenario: Consumer runs review and receives every use-case schema problem in one pass
    Given a clean committed change whose delta capability is routed "NEW in review-diagnostics-demo"
    And the use-case contract has a case-mismatched title, an unknown "## Guarantees" heading, a capability line that is not a bullet, an interaction that names an undeclared capability, and an example line that is not a bullet
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr reports the title, the unknown heading, the capability line, the undeclared interaction, and the example line together in one review run
    And stderr reports each distinct document problem once

  @givn.added @wip
  Scenario: A case-mismatched document title is refused with the exact correction
    Given a clean committed change whose use-case contract starts with "# Use Case: review-diagnostics-demo"
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr says headings are case-sensitive
    And stderr suggests "# Use case: review-diagnostics-demo"

  @givn.added
  Scenario: A near-miss section heading is refused with the exact accepted spellings
    Given a clean committed change whose use-case contract has an unknown "## Guarantees" heading
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr suggests "## Minimal guarantee"
    And stderr suggests "## Success guarantee"

  @givn.added
  Scenario: Missing required sections and a malformed interaction table are reported together
    Given a clean committed change whose use-case contract omits "## Minimal guarantee" and "## Success guarantee"
    And the interaction table separator row has the wrong column count
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr names both missing sections and the malformed interaction table together

  @givn.added @wip
  Scenario: One use-case parse failure is reported once across several delta capabilities
    Given a clean committed change with three delta capabilities routed "NEW in review-diagnostics-demo"
    And the use-case contract has a case-mismatched title and an unknown "## Guarantees" heading
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr names the case-mismatched title once
    And stderr names the unknown heading once

  @givn.added @wip
  Scenario: Review validates an active delta use-case narrative even for an extension
    Given a clean committed change that replaces an existing use-case narrative and whose routing decision extends that use case's capability
    And the replacement narrative has a case-mismatched title and an unknown "## Guarantees" heading
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr names the replacement narrative's title and unknown heading

  @givn.added @wip
  Scenario: Review accepts a green proof after documentation-only review edits
    Given a clean committed change with a green change verification receipt recorded at one revision
    And the developer commits only proposal, design, tasks, review, arc42, and use-case narrative edits after that revision
    When the developer runs `givn check review --change review-proof-demo`
    Then the command exits 0
    And the change suite did not run again
    And stdout says the change verification receipt was reused
