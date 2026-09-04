@givn.delta @release-0-3-0
Feature: Givn 0.3.0 release and legacy project migration

  @givn.modified
  Scenario: Cargo installation enables every declared feature by default
    Given  the Cargo feature metadata for the Givn release
    When  a maintainer inspects the default Cargo installation features
    Then  the default feature set is empty
    And  every optional model feature is declared for explicit opt-in
    And  a feature-free minimal build remains explicitly available
