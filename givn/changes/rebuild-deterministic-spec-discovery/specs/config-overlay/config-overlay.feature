@givn.delta @config-overlay
Feature: Configuration overlay customizes the effective manifest

  @givn.modified
  Scenario: Overlay removes a leaf artifact via remove true
    When  the overlay "artifacts:\n  - id: design-review\n    remove: true\n  - id: tasks\n    requires: [specs]\napply:\n  requires:\n    - specs" is applied to the default manifest
    Then  the effective manifest does not have an artifact "design-review"

  @givn.added
  Scenario: Overlay refuses to remove a protected gate artifact
    When  the overlay "artifacts:\n  - id: review\n    remove: true\napply:\n  requires:\n    - tasks" is applied to the default manifest
    Then  the overlay application fails with "protected"
