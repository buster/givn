@givn.delta @readability-review-gate

Feature: Readability rationale accounting

  @givn.added
  Scenario: An incomplete rationale names the rationale path
    Given a Givn project with the readability experiment enabled
    And a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And the change grows the type to 700 code lines at head
    And the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And a rationale for "type:src/engine.rs::Engine" omits "Reviewed-by:"
    And the change's review record records "READABILITY: PASS"
    When the readability review assesses the change
    Then the review fails
    And the review says "rationale givn/changes/test-change/readability/rationales/type-src-engine-rs-engine.md: missing Reviewed-by:"
