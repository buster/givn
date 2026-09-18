Feature: Givn release metadata and migration route

  Scenario: The release identifies itself as its package version
    Given the Givn release metadata
    When a consumer asks for the Givn version
    Then the reported version matches the package version

  Scenario: Cargo installation enables every declared feature by default
    Given  the Cargo feature metadata for the Givn release
    When  a maintainer inspects the default Cargo installation features
    Then  the default feature set is empty
    And  no optional model features are declared

  Scenario: The changelog describes the complete 0.3.0 release
    Given the Givn 0.3.0 release files
    When a maintainer reads the changelog
    Then it contains the user-visible changes from the stacked release
    And it contains the 0.2.0 to 0.3.0 migration entry point

  Scenario: The migration guide is an executable prompt sequence
    Given the 0.2.0 to 0.3.0 migration guide
    When an LLM reads the guide from a project using Givn
    Then it finds ordered prompts for inventory, upgrade, synchronization, review, and verification
    And each prompt tells the LLM which project evidence to inspect before editing
    And the guide forbids automatic changes to domain specifications and architecture decisions

  Scenario: The migration catalog defines complete release-owned bundles
    Given the embedded Givn migration catalog
    When a maintainer inspects the migration catalog
    Then it declares a catalog schema and migration bundle entries
    And every migration bundle has a source, target, order, proposal asset, and design asset
