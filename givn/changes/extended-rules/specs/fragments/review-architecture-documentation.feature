@givn.delta @review-architecture-documentation

Feature: Review architecture documentation

  @givn.modified
  Scenario: Generated review guidance stays aligned after skill sync
    Given a project root with givn/ initialised
    And an existing change named "review-alignment"
    When the user runs `givn skills sync`
    Then the command succeeds
    And the generated review skill directs the reviewer to the review instruction
    And the generated review skill carries no Arc42 implementation conformance section
    And the served review instruction contains "Arc42 implementation conformance"
