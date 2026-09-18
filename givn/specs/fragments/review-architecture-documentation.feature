Feature: Review architecture documentation

  @e2e
  Scenario: Arc42-enabled review instructions require conformance evidence
    Given a project root with givn/ initialised
    When the user runs `givn new arc42-review-alignment-test`
    And the user runs `givn instructions review --change arc42-review-alignment-test`
    Then the command succeeds
    And stdout contains "Arc42 implementation conformance"
    And stdout contains "design.md"
    And stdout contains "tasks.md"
    And stdout contains "completed implementation"
    And stdout contains "mismatch"
    And stdout contains "ARC42 CONFORMANCE: CLEAN"

  @e2e
  Scenario: The review template includes the Arc42 conformance block
    Given a project root with givn/ initialised
    When the user runs `givn eject review:template`
    Then the command succeeds
    And the file "givn/artifacts/review/review.md" exists
    And the ejected review template contains "Arc42 implementation conformance"
    And the ejected review template contains "design.md"
    And the ejected review template contains "tasks.md"
    And the ejected review template contains "ARC42 CONFORMANCE"

  @e2e
  Scenario: Generated review guidance stays aligned after skill sync
    Given a project root with givn/ initialised
    When the user runs `givn skills sync`
    Then the command succeeds
    And the generated review guidance contains "Arc42 implementation conformance"

  Scenario: Arc42-disabled review instructions mark conformance as not applicable
    Given a project root with givn/ initialised without addons
    When the user runs `givn new arc42-review-alignment-test`
    And the user runs `givn instructions review --change arc42-review-alignment-test`
    Then the command succeeds
    And stdout contains "N/A when the arc42 addon is disabled"
