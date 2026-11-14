@spec-gap
Feature: spec-gap

  @e2e @spec-gap
  Scenario: The review guidance requires one spec gap outcome before any fix
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the review guidance for an active change
    Then  the command exits 0
    And  the served review guidance requires exactly one spec gap outcome before any fix
    And  the served review guidance names "E2E test defect", "implementation defect", "specification defect", and "acceptable undocumented change"
    And  the served review guidance states that only "specification defect" is a gap in the Specification
    And  the terminal transcript is captured for this scenario

  @spec-gap
  Scenario: The proposal guidance names the corrective route for each outcome
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the proposal guidance for an active change
    Then  the command exits 0
    And  the served proposal guidance names the record and its outcome for a corrective change
    And  the served proposal guidance sends "implementation defect" back to the builder with no corpus edit
    And  the served proposal guidance carries the corpus or E2E suite edit for "specification defect" and "E2E test defect"
    And  the served proposal guidance records "acceptable undocumented change" in the corpus

  @spec-gap
  Scenario: A spec gap record names the artifact rather than its finder
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the proposal guidance for an active change
    Then  the command exits 0
    And  the served proposal guidance requires the record to name the wrong artifact, not who found it

  @spec-gap
  Scenario: An E2E test defect stops the Ise Test until the test is corrected
    Given  a Givn project with the ise addon enabled and the skills synchronized
    When  the consumer requests the review guidance for an active change
    Then  the command exits 0
    And  the served review guidance stops an Ise Test on "E2E test defect" and restarts it after the test is corrected
    And  the served review guidance states that the corrected test is a Specification correction, not an exception to the freeze
