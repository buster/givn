Feature: Choose Project License

  Scenario: Cargo.toml declares dual MIT OR Apache-2.0 license
    Given a project root containing the givn source checkout
    When the user reads Cargo.toml
    Then the license field is "MIT OR Apache-2.0"
    And the license field is no longer "TBD"

  Scenario: README.md declares the project license
    Given a project root containing the givn source checkout
    When the user reads README.md
    Then the license section names a specific license
    And the license section does not say "not yet chosen"

  Scenario: Consumer inspects the project license in Cargo.toml and README
    Given a checkout of the givn source repository
    When the user reads Cargo.toml
    Then the license field is a valid SPDX expression
    When the user reads README.md
    Then the license section names the same license
