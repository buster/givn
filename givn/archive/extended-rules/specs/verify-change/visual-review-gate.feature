@givn.delta @visual-review-gate

Feature: Visual Review Gate

  @givn.modified
  Scenario: Synchronized review guidance carries the visual block
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-review guidance directs the reviewer to the review instruction
    And the served review instruction contains "VISUAL-REVIEW: PASS"
    And the served review instruction contains "confirmed Persona"
    And the served review instruction contains "no pixel-level"
