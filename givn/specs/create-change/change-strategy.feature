@change-strategy
Feature: change-strategy

  @e2e @change-strategy
  Scenario: The design guidance requires one recorded change strategy
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design guidance for an active change
    Then  the command exits 0
    And  the served design guidance requires exactly one change strategy as a `Decides D` entry
    And  the served design guidance names "incremental modification", "Constant Ise", and a whole-product rebuild
    And  the served design guidance requires the affected consumers, the persisted state, and the rebuild boundary
    And  the terminal transcript is captured for this scenario

  @change-strategy
  Scenario: The design-review guidance keeps a rebuild without an Ise Test mode a finding
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the design-review guidance
    Then  the command exits 0
    And  the served design-review guidance keeps a rebuilding change with no recorded Ise Test mode a finding
    And  the served design-review guidance keeps an unresolved change strategy a finding
