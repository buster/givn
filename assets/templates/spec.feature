@givn.delta @{{CAPABILITY}}
Feature: {{FEATURE_TITLE}}

  # Write scenarios in domain language (Given/When/Then).
  # Each scenario tests ONE observable behaviour from a user or system perspective.
  # Do NOT mention: class names, function signatures, HTTP routes, DB schemas,
  # framework details, or step-definition mechanics.
  # Tag new scenarios @givn.added, changed ones @givn.modified, deleted ones @givn.removed.
  # Tag @wip while step definitions are not yet implemented.

  @givn.added @wip
  Scenario: {{SCENARIO_TITLE}}
    Given {{context}}
    When {{action}}
    Then {{observable_outcome}}
