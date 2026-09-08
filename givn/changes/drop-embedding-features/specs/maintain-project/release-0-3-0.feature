@givn.delta @release-0-3-0

Feature: Givn 0.3.0 release and legacy project migration

  @givn.modified
  Scenario: Cargo installation enables every declared feature by default
    Given the Cargo feature metadata for the Givn release
    When a maintainer inspects the default Cargo installation features
    Then the default feature set is empty
    And no optional model features are declared

  @givn.added @e2e @wip
  Scenario: A consumer invoking a retired engine command is told it does not exist
    Given an installed Givn binary
    When the consumer runs `givn spec index`
    Then the command exits non-zero
    And the error reports "spec index" is not an available command
    And stdout stays free of "embedding" and "download" wording
