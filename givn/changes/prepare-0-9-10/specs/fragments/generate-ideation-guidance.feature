@givn.delta @generate-ideation-guidance
Feature: Generate ideation guidance

  @givn.modified
  Scenario: A consumer synchronizes skills and every agent target receives the ideation facilitation skill
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then stdout contains "Syncing skills..."
    And stdout contains "synced skill: givn-ideation"
    And the served ideation instruction contains "Frame"
    And the generated givn-ideation guidance directs the ideator to the ideation instruction

  @givn.modified
  Scenario: The generated ideation files direct the agent to the ideation instruction
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-ideation guidance directs the ideator to the ideation instruction
    And no separately authored agent command is generated
