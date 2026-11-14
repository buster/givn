@givn.delta @prose-consolidation

Feature: prose-consolidation

  @givn.modified
  Scenario: Canonical policy accepts the library public API as its real interface
    Given a Givn project with ise effective whose product is a library without a service or browser
    When the author reads the canonical E2E guidance
    Then consumer calls to the public library API may run in process only from the declared e2e package as real-interface evidence
    And a call that needs production source becomes a component scenario
    And the policy does not require artificial services containers or browser infrastructure

  @givn.modified
  Scenario: Canonical policy selects E2E scope explicitly rather than by command shape
    Given regular and E2E verification share one runner command
    When the author reads the canonical verification guidance
    Then explicit scope selection such as GIVN_RESULT_SCOPE distinguishes the executions
    And equal test counts or command strings do not invalidate honestly isolated scopes
    And the E2E execution runs the declared e2e package which links no production source
    And required executions and zero failed or skipped tests remain required without an extra isolation run

  @givn.added
  Scenario: Canonical policy names the declared e2e package as the E2E target
    Given a Givn project with ise effective
    When the author reads the canonical E2E guidance
    Then the policy states that the declared e2e package holds the E2E runner and every E2E step definition
    And the policy states that the package links no production source
    And the policy states that an unbound or skipped step fails the E2E run
    And the policy states that feature files stay shared

  @givn.added
  Scenario: Served ise guidance keeps component scenarios in the Specification
    Given a Givn project with ise effective
    When the author reads the served ise guidance
    Then the guidance states that component scenarios stay in the Specification unchanged
    And the guidance states that only their step definitions and fakes are implementation
    And the guidance keeps the two-layer test boundary

  @givn.added
  Scenario: Canonical policy keeps the in-process allowance while ise is off
    Given a Givn project with ise off whose product is a library without a service or browser
    When the author reads the canonical E2E guidance
    Then consumer calls to the public library API may run in process as real-interface evidence
    And the policy does not require artificial services containers or browser infrastructure
