@givn.delta @refresh-generated-guidance
Feature: Refresh generated guidance

  @givn.added @wip
  Scenario: Generated implementation guidance routes to the task and step instructions
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-implement skill names both givn instructions tasks and givn instructions steps
    And the served tasks instruction names givn instructions steps
