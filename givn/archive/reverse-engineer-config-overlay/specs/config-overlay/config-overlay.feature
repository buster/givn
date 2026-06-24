@givn.delta @config-overlay
Feature: config-overlay

  The black-box boundary for this capability is: module boundary — `overlay::apply(base: Manifest, overlay_yaml: &str) -> Result<Manifest>`.

  The overlay engine applies user config from `givn/config.yaml` onto the
  embedded default manifest with deep-merge semantics. It supports artifact
  insertion/removal/modification, DAG validation (cycle and dangling-reference
  detection), addon expansion (arc42), and extras flag tracking.

  Suspected bugs: none.

  @givn.added
  Scenario: Empty overlay returns the base manifest unchanged
    When the overlay "{}" is applied to the default manifest
    Then the effective manifest equals the default manifest

  @givn.added
  Scenario: Overlay replaces the verify command
    When the overlay "verify:\n  command: my-runner" is applied to the default manifest
    Then the effective manifest has verify command "my-runner"

  @givn.added
  Scenario: Overlay replaces apply requires
    When the overlay "apply:\n  requires:\n    - review" is applied to the default manifest
    Then the effective manifest has apply requires exactly "review"

  @givn.added
  Scenario: Overlay adds a new artifact
    When the overlay "artifacts:\n  - id: risk-assessment\n    generates:\n      - risk.md\n    requires:\n      - design" is applied to the default manifest
    Then the effective manifest has an artifact "risk-assessment" that requires "design"

  @givn.added
  Scenario: Overlay removes a leaf artifact via remove true
    When the overlay "artifacts:\n  - id: review\n    remove: true\napply:\n  requires:\n    - tasks" is applied to the default manifest
    Then the effective manifest does not have an artifact "review"

  @givn.added
  Scenario: Overlay remove is rejected when artifact has dependents
    When the overlay "artifacts:\n  - id: design-review\n    remove: true" is applied to the default manifest
    Then the overlay application fails with "does not exist"

  @givn.added
  Scenario: Overlay marks an existing artifact as optional
    When the overlay "artifacts:\n  - id: proposal\n    optional: true" is applied to the default manifest
    Then the effective manifest has an artifact "proposal" that is optional

  @givn.added
  Scenario: Overlay deep-merges fields on an existing artifact
    When the overlay "artifacts:\n  - id: proposal\n    generates:\n      - plan.md" is applied to the default manifest
    Then the effective manifest has an artifact "proposal" that generates "plan.md"

  @givn.added
  Scenario: Arc42 addon expands to arc42-docs artifact when enabled
    When the overlay "addons:\n  arc42: true" is applied to the default manifest
    Then the effective manifest has an artifact "arc42-docs" that is optional

  @givn.added
  Scenario: Arc42 addon is idempotent when already enabled
    When the overlay "addons:\n  arc42: true" is applied twice to the default manifest
    Then the effective manifest has exactly one artifact "arc42-docs"

  @givn.added
  Scenario: Extras flag is propagated from overlay
    When the overlay "extras: true" is applied to the default manifest
    Then the effective manifest has extras enabled

  @givn.added
  Scenario: Cycle in dependency graph is rejected
    When the overlay "artifacts:\n  - id: a\n    requires:\n      - b\n  - id: b\n    requires:\n      - a" is applied to the default manifest
    Then the overlay application fails with "cycle"

  @givn.added
  Scenario: Dangling require reference is rejected
    When the overlay "artifacts:\n  - id: my-artifact\n    requires:\n      - nosuch" is applied to the default manifest
    Then the overlay application fails with "does not exist"

  @givn.added
  Scenario: apply.requires referencing non-existent artifact is rejected
    When the overlay "apply:\n  requires:\n    - nosuch" is applied to the default manifest
    Then the overlay application fails with "does not exist"