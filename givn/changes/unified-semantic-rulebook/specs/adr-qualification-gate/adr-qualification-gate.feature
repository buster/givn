# User Interaction Inventory:
# - (inventory owned by the adr-qualification-gate capability; this delta only retires and re-adds one register-count assertion)

@givn.delta @adr-qualification-gate

Feature: adr-qualification-gate

  @givn.removed
  Scenario: The migration removes non-ADRs and preserves independent architecture history
    Given  a project root with givn/ initialised

  @givn.added
  Scenario: The migration preserves independent architecture history across three proposed decisions
    Given  the consolidated ADR register and canonical sources are available
    When  a maintainer compares the register with the canonical sources
    Then  the final register has 35 accepted decisions and three proposed decisions
    And  the change's migration manifest gives every assigned identifier exactly one disposition
    And  reserved identifiers `ADR-050` and `ADR-051` have no record
    And  non-ADR records are removed from the active register and ADR archive
    And  absorbed records are absent and their refinements are in the target ADRs
    And  superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
    And  `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
    And  independent architecture decisions remain separate
    And  `ADR-063` remains proposed and owned by its active change
