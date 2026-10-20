@givn.delta @readability-review-gate

Feature: Readability reviewer independence

  @givn.added
  Scenario: A rationale with an independent reviewer passes
    Given a Givn project with the readability experiment enabled
    And a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And the change grows the type to 700 code lines at head
    And the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And a rationale for "type:src/engine.rs::Engine" has an independent reviewer
    And the change's review record records "READABILITY: PASS"
    When the readability review assesses the change
    Then the review passes
    And the review reports "JUSTIFIED type:src/engine.rs::Engine"

  @givn.added
  Scenario: A rationale without a reviewer fails
    Given a Givn project with the readability experiment enabled
    And a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And the change grows the type to 700 code lines at head
    And the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And a rationale for "type:src/engine.rs::Engine" omits "Reviewed-by:"
    And the change's review record records "READABILITY: PASS"
    When the readability review assesses the change
    Then the review fails
    And the review says "missing Reviewed-by:"

  @givn.added
  Scenario: A rationale reviewed by its author fails
    Given a Givn project with the readability experiment enabled
    And a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And the change grows the type to 700 code lines at head
    And the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And a rationale for "type:src/engine.rs::Engine" records "Reviewed-by: agent"
    And the change's review record records "READABILITY: PASS"
    When the readability review assesses the change
    Then the review fails
    And the review says "Reviewed-by must differ from Author"

  @givn.added
  Scenario: A rationale with a reject verdict fails
    Given a Givn project with the readability experiment enabled
    And a committed change whose base has a god type of 620 code lines split across files in "src/engine.rs"
    And the change grows the type to 700 code lines at head
    And the change's review record records "READABILITY: JUSTIFIED type:src/engine.rs::Engine"
    And a rationale for "type:src/engine.rs::Engine" records "Review verdict: reject"
    And the change's review record records "READABILITY: PASS"
    When the readability review assesses the change
    Then the review fails
    And the review says "Review verdict 'reject' is not accept"
