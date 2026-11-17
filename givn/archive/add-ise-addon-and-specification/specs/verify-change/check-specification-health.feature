@givn.delta @check-specification-health

Feature: check-specification-health

  @givn.added
  Scenario: An absent declaration is reported by name and no advisory finding is computed
    Given an ise-enabled project selecting opencode with no declaration file
    When the user runs `givn lint`
    Then the command exits non-zero
    And stderr names "givn/specification.yaml"
    And stderr contains "checks were not performed"

  @givn.added
  Scenario: A declared path missing from the tree is reported as advisory
    Given an ise-enabled project selecting opencode whose limits role declares "docs/performance.md"
    And the path "docs/performance.md" does not exist
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names "docs/performance.md"

  @givn.added
  Scenario: One path declared in two roles is reported as advisory
    Given an ise-enabled project selecting opencode whose structure and behavior roles both declare "givn/specs/**"
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names "givn/specs/**"
    And stderr reports the duplicate declaration once

  @givn.added
  Scenario: A .feature file outside the behavior globs is reported as advisory
    Given an ise-enabled project selecting opencode whose behavior role declares only "givn/specs/orders/**"
    And a .feature file at "givn/specs/billing/invoice.feature"
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names "givn/specs/billing/invoice.feature"

  @givn.added
  Scenario: A declared e2e package missing from the tree is reported as advisory
    Given an ise-enabled project selecting opencode whose e2e role declares "tests/e2e"
    And the path "tests/e2e" does not exist
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names "tests/e2e"
    And stderr reports the missing e2e package once

  @givn.added
  Scenario: A declared e2e package with a dependency edge into source is reported as advisory
    Given an ise-enabled project selecting opencode whose e2e package depends on the source package
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names the e2e package and the dependency edge into "source"

  @givn.added
  Scenario: A declared e2e package with a transitive dependency edge into source is reported as advisory
    Given an ise-enabled project selecting opencode whose e2e package depends transitively on the source package
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names the e2e package and the dependency edge into "source"

  @givn.added
  Scenario: A declared e2e path with no package manifest is reported by the boundary finding
    Given an ise-enabled project selecting opencode whose e2e role declares a package with no Cargo manifest
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains "advisory"
    And stderr names "tests/e2e" and the reason "no package manifest"

  @givn.added
  Scenario: A malformed package manifest skips the dependency-edge finding
    Given an ise-enabled project selecting opencode whose e2e package manifest is malformed
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains no dependency-edge finding

  @givn.added
  Scenario: The JSON path exposes declaration findings with advisory severity
    Given an ise-enabled project selecting opencode whose limits role declares "docs/performance.md"
    And the path "docs/performance.md" does not exist
    When the user runs `givn lint --json`
    Then the command exits 0
    And stdout reports a finding with severity "advisory" naming "docs/performance.md"

  @givn.added
  Scenario: Change-scoped lint does not read the declaration
    Given an ise-enabled project selecting opencode with a malformed declaration
    And an active change with one clean delta feature
    When the user runs `givn lint --change add-declaration-check`
    Then the command exits 0
    And stderr contains no declaration finding

  @givn.added
  Scenario: An empty role list retains nothing and produces no finding
    Given an ise-enabled project selecting opencode with the scaffolded declaration
    When the user runs `givn lint`
    Then the command exits 0
    And stderr contains no declaration finding
