# Existing inventory entry:
# givn-workflow-enforcement | run the review check on a change with use-case
# schema errors | Consumer runs review and receives every use-case schema
# problem in one pass.
# This delta adds error-case scenarios for that same review action; it does not
# introduce a new interaction or a second E2E evidence scenario.

@givn.delta @givn-workflow-enforcement

Feature: Review use-case diagnostic corrections

  @givn.added @wip
  Scenario: Review corrects a non-kebab-case use-case title to its canonical identifier
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register"
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr says headings are case-sensitive
    And stderr suggests "# Use case: lending-register"

  @givn.added @wip
  Scenario: Review does not accept a non-kebab-case title as the use-case identifier
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and whose capability list is a table
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr does not contain "leaf use case 'Lending Register'"
    And stderr does not contain "did you mean '# Use case: Lending Register'"

  @givn.added @wip
  Scenario: A malformed title does not fabricate missing-section diagnostics
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and omits "## Rules"
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr suggests "# Use case: lending-register"
    And stderr does not contain "document schema does not recognise '## Rules'"

  @givn.added @wip
  Scenario: A malformed Level does not fabricate missing-section diagnostics
    Given a clean committed change whose use-case contract has a malformed Level and omits "## Examples"
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr contains "use-case Level must be '!' or '+'"
    And stderr does not contain "document schema does not recognise '## Examples'"

  @givn.added @wip
  Scenario: A genuinely unknown heading is still reported with a malformed title
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and has an unknown "## Guarantees" heading
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr contains "unknown heading '## Guarantees'"
    And stderr suggests "# Use case: lending-register"

  @givn.added @wip
  Scenario: A valid kebab-case use-case title remains accepted
    Given a clean committed change with a green change verification receipt recorded at one revision
    When the developer runs `givn check review --change review-proof-demo`
    Then the command exits 0
    And stderr does not contain "expected '# Use case:"
    And stderr does not contain "document schema does not recognise"
