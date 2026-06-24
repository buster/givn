Feature: config-overlay

  Scenario: Empty overlay returns the base manifest unchanged
    When  the overlay "{}" is applied to the default manifest
    Then  the effective manifest equals the default manifest

  Scenario: Overlay replaces the verify command
    When  the overlay "verify:\n  command: my-runner" is applied to the default manifest
    Then  the effective manifest has verify command "my-runner"

  Scenario: Overlay replaces apply requires
    When  the overlay "apply:\n  requires:\n    - review" is applied to the default manifest
    Then  the effective manifest has apply requires exactly "review"

  Scenario: Overlay adds a new artifact
    When  the overlay "artifacts:\n  - id: risk-assessment\n    generates:\n      - risk.md\n    requires:\n      - design" is applied to the default manifest
    Then  the effective manifest has an artifact "risk-assessment" that requires "design"

  Scenario: Overlay removes a leaf artifact via remove true
    When  the overlay "artifacts:\n  - id: review\n    remove: true\napply:\n  requires:\n    - tasks" is applied to the default manifest
    Then  the effective manifest does not have an artifact "review"

  Scenario: Overlay remove is rejected when artifact has dependents
    When  the overlay "artifacts:\n  - id: design-review\n    remove: true" is applied to the default manifest
    Then  the overlay application fails with "does not exist"

  Scenario: Overlay marks an existing artifact as optional
    When  the overlay "artifacts:\n  - id: proposal\n    optional: true" is applied to the default manifest
    Then  the effective manifest has an artifact "proposal" that is optional

  Scenario: Overlay deep-merges fields on an existing artifact
    When  the overlay "artifacts:\n  - id: proposal\n    generates:\n      - plan.md" is applied to the default manifest
    Then  the effective manifest has an artifact "proposal" that generates "plan.md"

  Scenario: Arc42 addon expands to arc42-docs artifact when enabled
    When  the overlay "addons:\n  arc42: true" is applied to the default manifest
    Then  the effective manifest has an artifact "arc42-docs" that is optional

  Scenario: Arc42 addon is idempotent when already enabled
    When  the overlay "addons:\n  arc42: true" is applied twice to the default manifest
    Then  the effective manifest has exactly one artifact "arc42-docs"

  Scenario: Extras flag is propagated from overlay
    When  the overlay "extras: true" is applied to the default manifest
    Then  the effective manifest has extras enabled

  Scenario: Cycle in dependency graph is rejected
    When  the overlay "artifacts:\n  - id: a\n    requires:\n      - b\n  - id: b\n    requires:\n      - a" is applied to the default manifest
    Then  the overlay application fails with "cycle"

  Scenario: Dangling require reference is rejected
    When  the overlay "artifacts:\n  - id: my-artifact\n    requires:\n      - nosuch" is applied to the default manifest
    Then  the overlay application fails with "does not exist"

  Scenario: apply.requires referencing non-existent artifact is rejected
    When  the overlay "apply:\n  requires:\n    - nosuch" is applied to the default manifest
    Then  the overlay application fails with "does not exist"
