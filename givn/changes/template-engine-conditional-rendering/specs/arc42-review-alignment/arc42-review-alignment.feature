@givn.delta @arc42-review-alignment

Feature: Arc42 completion conformance review

  @givn.modified @e2e
  Scenario: Generated review guidance stays aligned after skill sync
    Given  a project root with givn/ initialised
    When  the user runs `givn skills sync`
    Then  the command succeeds
    And  the generated review guidance contains "Arc42 implementation conformance"
