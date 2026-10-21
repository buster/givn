@givn.delta @givn-workflow-enforcement

Feature: Review use-case diagnostic corrections

  @givn.added
  Scenario: Review corrects a non-kebab-case use-case title to its canonical identifier
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and has an unknown "## Guarantees" heading
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr says headings are case-sensitive
    And stderr suggests exactly "# Use case: lending-register"
    And stderr contains "unknown heading '## Guarantees'"

  @givn.added
  Scenario: Review does not accept a non-kebab-case title as the use-case identifier
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register" and whose capability list is a table
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr does not contain "leaf use case 'Lending Register'"
    And stderr does not contain "did you mean '# Use case: Lending Register'"
    And stderr does not contain "does not declare capability"
    And stderr does not contain "has no interaction for capability"

  @givn.added
  Scenario: Review corrects an exact-prefix non-kebab-case title with the canonical identifier
    Given a clean committed change whose use-case contract starts with "# Use case: Lending Register"
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr contains "must be a stable kebab-case identifier"
    And stderr suggests exactly "# Use case: lending-register"
    And stderr does not contain "document is missing '# Use case:"

  @givn.added
  Scenario: A malformed title does not fabricate missing-section or empty-section diagnostics
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register", omits "## Rules", and leaves "## Examples" empty
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr suggests exactly "# Use case: lending-register"
    And stderr does not contain "document schema does not recognise '## Rules'"
    And stderr does not contain "section '## Examples' must not be empty"

  @givn.added
  Scenario: A malformed Level does not fabricate missing-section or leaf-capability diagnostics
    Given a clean committed change whose use-case contract has a malformed Level and omits "## Examples"
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr contains "use-case Level must be '!' or '+'"
    And stderr does not contain "document schema does not recognise '## Examples'"
    And stderr does not contain "must declare at least one capability"

  @givn.added
  Scenario: Independent local findings are still reported with a malformed title
    Given a clean committed change whose use-case contract starts with "# Use Case: Lending Register", has a rule line that is not a bullet, and has an interaction that names an undeclared capability
    When the developer runs `givn check review --change review-diagnostics-demo`
    Then the command exits non-zero
    And stderr contains "section '## Rules' must contain one rule per bullet"
    And stderr does not contain "interaction references undeclared capability"
    And stderr suggests exactly "# Use case: lending-register"

  @givn.added
  Scenario: A valid kebab-case use-case title remains accepted
    Given a clean committed change with a green change verification receipt recorded at one revision
    When the developer runs `givn check review --change review-proof-demo`
    Then the command exits 0
    And stderr does not contain "expected '# Use case:"
    And stderr does not contain "document schema does not recognise"
