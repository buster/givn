# User Interaction Inventory:
# TODO: Enumerate normalized distinct user actions against the real interface.
# Read `givn instructions specs --change <id>` for the canonical E2E policy.
#
# Web UI: every button click, form submission, nav link, SSE/WS event.
# CLI: every distinct action; output-affecting flags are variants unless they
# change the action or side effect.
# HTTP API: every endpoint, every side-effect-permitting operation.
# Library: every public function/method a consumer invokes.
# Message bus: every message type consumed, every side-effect-producing message.
#
# Example:
# - submit "Add item" form on list page
# - click "Check" button on a list item
# - click "Delete" button on a list item

@givn.delta @{{CAPABILITY}}
Feature: {{FEATURE_TITLE}}

  # Write scenarios in domain language (Given/When/Then).
  # Each scenario tests ONE observable behaviour from a user or system perspective.
  # Do NOT mention: class names, function signatures, HTTP routes, DB schemas,
  # framework details, or step-definition mechanics.
  # Tag new scenarios @givn.added, changed ones @givn.modified, deleted ones @givn.removed.
  # Tag @wip while step definitions are not yet implemented.
  # Tag @e2e for end-to-end smoke tests that exercise the real interaction surface.
# One @e2e scenario per distinct real user action, as defined by the canonical
# specs instruction — not one per variant/error case.

  @givn.added @wip
  Scenario: {{SCENARIO_TITLE}}
    Given {{context}}
    When {{action}}
    Then {{observable_outcome}}

  # @e2e smoke tests — one per normalized inventory entry; use verify.e2e_command + e2e step definitions.

  @givn.added @e2e @wip
  Scenario: {{E2E_SCENARIO_TITLE}}
    Given {{e2e_context}}
    When {{e2e_action}}
    Then {{e2e_observable_outcome}}
