@givn.delta @givn-workflow-enforcement
Feature: Ise ship component tier

  @givn.added
  Scenario: A whole-system Ise ship executes the retained component corpus
    Given an ise-effective ship change declaring two retained component scenarios and a runner that reports every declared scenario
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits 0
    And stdout contains "component_executed=2/2"
    And the change verification receipt records the retained component corpus at "givn/specs" with two scenarios

  @givn.added
  Scenario: An Ise ship with a delta also executes the retained component corpus
    Given an ise-effective ship change that keeps one delta scenario and two retained component scenarios
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits 0
    And the change verification receipt records a delta result of one scenario
    And the change verification receipt records a component result of two scenarios

  @givn.added
  Scenario: A whole-system Ise ship without a runner result is refused
    Given an ise-effective ship change declaring two retained component scenarios and a runner that exits 0 without a result file
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names "component_executed=0/2"

  @givn.added
  Scenario: A whole-system Ise ship that executes fewer component scenarios is refused
    Given an ise-effective ship change declaring two retained component scenarios and a runner that reports one executed scenario
    When the developer runs `givn check tasks --change ise-boundary`
    Then the command exits non-zero
    And stderr names "component_executed=1/2"

  @givn.added
  Scenario: Review refuses an Ise ship without a component receipt
    Given an ise-effective ship change declaring two retained component scenarios and a runner that reports every declared scenario
    When the developer runs `givn check review --change ise-boundary`
    Then the command exits non-zero
    And stderr names "change verification receipt missing"
