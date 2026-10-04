@givn.delta @describe-the-release

Feature: Givn release metadata and migration route

  @givn.added
  Scenario: A release section lists only the user-visible change groups
    Given a release range with a feature, a bug fix, a performance change, a revert, and a breaking change
    And a test-harness bug fix and an uncategorised revision in the same range
    When the maintainer generates the changelog for the release range
    Then the changelog lists the "Breaking Changes", "Features", "Bug Fixes", "Performance", and "Reverts" groups
    And the changelog lists no "Other Changes" group
    And the release section lists no test-harness bug fix

  @givn.added
  Scenario: A release range without user-visible change has no release section
    Given a release range with only documentation, refactoring, test, chore, and uncategorised revisions
    When the maintainer generates the changelog for the release range
    Then the release section lists no change group

  @givn.added
  Scenario: A release cannot be prepared without a tag for the current version
    Given a repository whose current version is "0.8.0"
    And whose newest version tag is "v0.4.0"
    When the maintainer verifies the release base
    Then the check fails naming the missing tag "v0.8.0"
    And the check accepts the repository once the newest version tag is "v0.8.0"

  @givn.added
  Scenario: A release section without user-visible change stops the release
    Given a generated changelog whose release section for "0.9.0" holds no change group
    When the maintainer verifies the release section for "0.9.0"
    Then the check fails naming the release version "0.9.0"
    And the check accepts a release section holding a change group

  @givn.added @e2e
  Scenario: Repeated revisions of one change yield one changelog entry
    Given a release range with three feature revisions of one change that share the release note "Givn reports the active SCM backend in givn status"
    And a documentation revision in the same range
    When the maintainer generates the changelog for the release range
    Then the changelog lists "Givn reports the active SCM backend in givn status" exactly once
    And the changelog lists no documentation entry
