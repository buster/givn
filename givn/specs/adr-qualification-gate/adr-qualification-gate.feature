Feature: Strict qualification and canonical routing of architecture decisions

  Scenario: The guidance defines strict qualification as an all-gates contract
    Given  the ADR qualification guidance is available
    When  an author reads the qualification gate
    Then  the guidance requires materially different alternatives
    And  the guidance requires architectural impact
    And  the guidance requires a durable consequence
    And  the guidance requires correct lower-level artifact classification
    And  the guidance requires a completed existing-ADR or duplicate check
    And  the guidance defines qualification as Boolean AND across all mandatory gates

  Scenario: Shared guidance alone does not qualify an ADR
    Given  the ADR qualification guidance is available
    When  an author reads the qualification criteria
    Then  the guidance says `Must be shared` is supporting evidence only
    And  the guidance says shared evidence cannot qualify a candidate by itself
    And  the guidance routes a non-qualifying candidate to one canonical artifact

  Scenario: The guidance routes normal design decisions to one canonical artifact
    Given  the ADR qualification guidance is available
    When  an author reads the lower-level artifact rules
    Then  the guidance distinguishes a feature specification from technical design
    And  the guidance distinguishes Arc42 documentation from implementation code
    And  the guidance distinguishes process guidance from architecture rationale
    And  the guidance requires exactly one canonical target for a non-ADR decision

  Scenario: The guidance requires an existing-ADR check before drafting
    Given  the ADR qualification guidance is available
    When  an author reads the duplicate and relationship rules
    Then  the guidance requires searching existing ADRs before drafting
    And  the guidance requires an explicit new amend supersede or canonical-artifact route
    And  the guidance rejects a new ADR when the existing-ADR check is incomplete

  Scenario: The structured verdict has fixed gate and route fields
    Given  the ADR qualification guidance is available
    When  an author reads the structured verdict contract
    Then  the verdict contract names all five mandatory gate fields
    And  the verdict contract names the supporting-only `must_be_shared` field
    And  the verdict contract names `qualification` and `routing`
    And  the verdict contract names `canonical_artifact`, `target_adr`, and `replacement_adr`
    And  the guidance defines the allowed qualification gate and routing values

  Scenario: Route targets are conditional and unused targets stay empty
    Given  the ADR qualification guidance is available
    When  an author reads the route-target rules
    Then  `CANONICAL_ARTIFACT` requires a `canonical_artifact` target
    And  `AMEND_ADR` requires a `target_adr` target
    And  `SUPERSEDE_ADR` requires a `replacement_adr` target
    And  unused route-target fields are required to remain null

  Scenario: The guidance forbids invented ADR evidence
    Given  the ADR qualification guidance is available
    When  an author reads the evidence rules
    Then  the guidance requires evidence supplied by the author or source artifacts
    And  the guidance says missing evidence remains empty
    And  the guidance forbids inventing alternatives impact consequences or participants

  Scenario: An ADR refinement is routed to amendment without duplication
    Given  the ADR qualification guidance is available
    When  an author reads the refinement rules
    Then  the guidance routes a refinement to `AMEND_ADR`
    And  the guidance requires the target ADR to be named
    And  the guidance forbids a parallel ADR for the same decision boundary

  Scenario: An architectural replacement preserves supersession history
    Given  the ADR qualification guidance is available
    When  an author reads the replacement rules
    Then  the guidance routes a replacement to `SUPERSEDE_ADR`
    And  the guidance requires the replacement ADR to be named
    And  the guidance requires the old architectural record to keep an explicit replacement link

  Scenario: Arc42 impact does not by itself require an ADR
    Given  the ADR qualification guidance is available
    When  an author reads the Arc42 impact rules
    Then  the guidance distinguishes chapter impact from ADR qualification
    And  the guidance requires an affected chapter to be updated when appropriate
    And  the guidance does not require a new ADR solely because a chapter is affected

  Scenario: The change assessment records ADR disposition separately from chapter impact
    Given  the change-level Arc42 assessment template is available
    When  a maintainer reads the ADR disposition marker
    Then  the marker distinguishes chapter impact from ADR action
    And  the marker allows `NONE` with target `none`
    And  the marker does not duplicate the complete MADR procedure

  Scenario: The ADR register has one canonical ownership contract
    Given  the consolidated ADR register guidance is available
    When  a maintainer reads the register rules
    Then  the active register contains accepted non-superseded architectural decisions only
    And  non-ADR information is not copied into the ADR archive
    And  current architecture facts are owned by Arc42 or the implementation
    And  an ADR retains rationale and links instead of duplicating current facts

  Scenario: All effective guidance sources use the same ADR policy
    Given  the canonical Arc42 qualification procedure and design-review guidance are available
    When  an author compares the effective and generated guidance sources
    Then  the canonical source uses strict AND qualification semantics
    And  all sources that state qualification treat `Must be shared` as supporting evidence only
    And  all sources that state routing use the same routing values and route targets
    And  derived sources defer to the canonical procedure instead of copying its schema
    And  no source claims that runtime validation enforces the policy
    And  a project override remains the effective source when configured

  Scenario: The ADR guidance requires an adversarial qualification record
    Given  the ADR qualification guidance is available
    When  an author prepares an ADR candidate
    Then  the guidance requires real alternatives and supplied evidence
    And  the guidance requires a decision classification and one canonical target
    And  the guidance requires an explicit duplicate amendment or supersession check
    And  the MADR checklist is evidence for all mandatory gates rather than an at-least-one escape hatch

  @e2e
  Scenario: An author requests the strict ADR guidance through the Givn CLI
    Given  an initialized ADR qualification project with active change `adr-qualification-gate`
    When  the author requests the Arc42 ADR guidance through the Givn CLI
    Then  the CLI exits with status 0
    And  the CLI output contains the strict AND qualification gate
    And  the CLI output says that `Must be shared` is supporting evidence only
    And  the CLI output distinguishes `NEW_ADR`, `AMEND_ADR`, `SUPERSEDE_ADR`, and `CANONICAL_ARTIFACT`

  @e2e
  Scenario: A reviewer requests design-review guidance through the Givn CLI
    Given  an initialized ADR qualification project with active change `adr-qualification-gate`
    When  the reviewer requests the `design-review` guidance through the Givn CLI
    Then  the CLI exits with status 0
    And  the CLI output requires every mandatory qualification dimension to pass
    And  the CLI output requires the existing-ADR and duplicate check
    And  the CLI output says that qualification policy is guidance rather than runtime enforcement
  Scenario: The migration preserves independent architecture history across three proposed decisions
    Given  the consolidated ADR register and canonical sources are available
    When  a maintainer compares the register with the canonical sources
    Then  the final register has 36 accepted decisions and three proposed decisions
    And  the change's migration manifest gives every assigned identifier exactly one disposition
    And  reserved identifiers `ADR-050` and `ADR-051` have no record
    And  non-ADR records are removed from the active register and ADR archive
    And  absorbed records are absent and their refinements are in the target ADRs
    And  superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
    And  `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
    And  independent architecture decisions remain separate
    And  `ADR-063` remains proposed and owned by its active change
