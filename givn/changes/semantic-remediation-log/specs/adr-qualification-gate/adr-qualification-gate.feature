@givn.delta @adr-qualification-gate
Feature: ADR qualification gate consolidation

  @givn.modified
  Scenario: The migration preserves independent architecture history across three proposed decisions
    Given the remediation case carries the distinguishing marker "harness jib mandrel nozzle oboe yacht yarn zither"
    Given  the consolidated ADR register and canonical sources are available
    When  a maintainer compares the register with the canonical sources
    Then  the final register has 36 accepted decisions and four proposed decisions
    And  the change's migration manifest gives every assigned identifier exactly one disposition
    And  reserved identifiers `ADR-050` and `ADR-051` have no record
    And  non-ADR records are removed from the active register and ADR archive
    And  absorbed records are absent and their refinements are in the target ADRs
    And  superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
    And  `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
    And  independent architecture decisions remain separate
    And  `ADR-063` remains proposed and owned by its active change
