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

  @describe-the-release
  Scenario: The migration catalog defines complete release-owned bundles
    Given  the embedded Givn migration catalog with 1.0 as the supported migration baseline
    When  a maintainer inspects the migration catalog
    Then  it declares a catalog schema and a route collection that may be empty
    And  it contains no embedded pre-1.0 migration routes
    And  every declared migration bundle has a source, target, order, proposal asset, and design asset

  Scenario: A release section lists only the user-visible change groups
    Given  a release range with a feature, a bug fix, a performance change, a revert, and a breaking change
    And  a test-harness bug fix and an uncategorised revision in the same range
    When  the maintainer generates the changelog for the release range
    Then  the changelog lists the "Breaking Changes", "Features", "Bug Fixes", "Performance", and "Reverts" groups
    And  the changelog lists no "Other Changes" group
    And  the release section lists no test-harness bug fix

  Scenario: A release range without user-visible change has no release section
    Given  a release range with only documentation, refactoring, test, chore, and uncategorised revisions
    When  the maintainer generates the changelog for the release range
    Then  the release section lists no change group

  Scenario: A release cannot be prepared without a tag for the current version
    Given  a repository whose current version is "0.8.0"
    And  whose newest version tag is "v0.4.0"
    When  the maintainer verifies the release base
    Then  the check fails naming the missing tag "v0.8.0"
    And  the check accepts the repository once the newest version tag is "v0.8.0"

  Scenario: A release section without user-visible change stops the release
    Given  a generated changelog whose release section for "0.9.0" holds no change group
    When  the maintainer verifies the release section for "0.9.0"
    Then  the check fails naming the release version "0.9.0"
    And  the check accepts a release section holding a change group

  @e2e
  Scenario: Repeated revisions of one change yield one changelog entry
    Given  a release range with three feature revisions of one change that share the release note "Givn reports the active SCM backend in givn status"
    And  a documentation revision in the same range
    When  the maintainer generates the changelog for the release range
    Then  the changelog lists "Givn reports the active SCM backend in givn status" exactly once
    And  the changelog lists no documentation entry

  @describe-the-release
  Scenario Outline: Synthetic future migration bundles retain complete metadata validation
    Given  a synthetic future supported migration catalog fixture starting at or after the 1.0 baseline
    And  its migration bundle omits only the "<field>" field
    When  a maintainer validates the migration catalog fixture
    Then  validation fails and names the incomplete bundle and missing "<field>"
    And  the same fixture validates once the missing field is restored
    Examples:
      | field |
      | source |
      | target |
      | order |
      | proposal asset |
      | design asset |
