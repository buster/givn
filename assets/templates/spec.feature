@givn.delta @{{CAPABILITY}}
Feature: {{FEATURE_TITLE}}

  # Write scenarios in domain language (Given/When/Then).
  # Each scenario tests ONE observable behaviour from a user or system perspective.
  # Do NOT mention: class names, function signatures, HTTP routes, DB schemas,
  # framework details, or step-definition mechanics.
  # Tag new scenarios @givn.added, changed ones @givn.modified, deleted ones @givn.removed.
  # Tag @wip while step definitions are not yet implemented.
  # Tag @e2e for end-to-end smoke tests that exercise the real interaction surface.
  # One @e2e scenario per distinct happy-path action — not one per variant/error case.

  @givn.added @wip
  Scenario: {{SCENARIO_TITLE}}
    Given {{context}}
    When {{action}}
    Then {{observable_outcome}}

  # End-to-end smoke test — exercises the real interaction surface (browser, CLI, API).
  # Uses verify.e2e_command runner and e2e-specific step definitions.

  @givn.added @e2e @wip
  Scenario: {{E2E_SCENARIO_TITLE}}
    Given {{e2e_context}}
    When {{e2e_action}}
    Then {{e2e_observable_outcome}}
