@givn.delta @match-guidance-to-enabled-addons
Feature: Match guidance to enabled addons

  @givn.added
  Scenario Outline: Outline guidance follows the opt-in addon
    Given a Givn project with the outline addon <state> and the skills synchronized
    Then the served steps instruction <outcome> the advisory outline section
    And the served review instruction <outcome> the advisory outline section
    And no generated skill carries outline guidance
    Examples:
      | state    | outcome  |
      | enabled  | contains |
      | disabled | omits    |
