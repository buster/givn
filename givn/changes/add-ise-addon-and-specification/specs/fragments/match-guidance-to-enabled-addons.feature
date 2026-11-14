@givn.delta @match-guidance-to-enabled-addons

Feature: match-guidance-to-enabled-addons

  @givn.removed
  Scenario: Architecture principles guidance follows the opt-in addon
    Given a placeholder step

  @givn.added
  Scenario: Architecture principles guidance follows an enabled addon
    Given  a Givn project with the architecture-principles addon disabled and the skills synchronized
    When  a consumer enables the architecture-principles addon through the Givn CLI
    Then  the served design instruction contains the Contracts and components section
    And  the served design-review instruction contains the Contracts and components branch

  @givn.removed
  Scenario Outline: Outline guidance follows the opt-in addon
    Given a placeholder step

  @givn.added
  Scenario Outline: Outline guidance follows an enabled addon
    Given  a Givn project with the outline addon <state> and the skills synchronized
    Then  the served steps instruction <outcome> the advisory outline section
    And  the served review instruction <outcome> the advisory outline section
    And  no generated skill carries outline guidance
    Examples:
      | state | outcome |
      | enabled | contains |
      | disabled | omits |
