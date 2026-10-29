@givn.delta @describe-the-release
Feature: Givn release metadata and migration route

  @givn.modified
  Scenario: The migration catalog defines complete release-owned bundles
    Given the embedded Givn migration catalog with 1.0 as the supported migration baseline
    When a maintainer inspects the migration catalog
    Then it declares a catalog schema and a route collection that may be empty
    And it contains no embedded pre-1.0 migration routes
    And every declared migration bundle has a source, target, order, proposal asset, and design asset

  @givn.added
  Scenario Outline: Synthetic future migration bundles retain complete metadata validation
    Given a synthetic future supported migration catalog fixture starting at or after the 1.0 baseline
    And its migration bundle omits only the "<field>" field
    When a maintainer validates the migration catalog fixture
    Then validation fails and names the incomplete bundle and missing "<field>"
    And the same fixture validates once the missing field is restored

    Examples:
      | field          |
      | source         |
      | target         |
      | order          |
      | proposal asset |
      | design asset   |

  @givn.removed
  Scenario: The migration guide is an executable prompt sequence
    Given this scenario is removed
