Feature: Strict qualification and canonical routing of architecture decisions

  @adr-qualification-gate
  Scenario: The guidance defines strict qualification as an all-gates contract
    Given  the ADR qualification guidance is available
    When  an author reads the qualification gate
    Then  an ADR requires an architectural commitment with real alternatives and a lasting boundary or consequence
    And  the author must search existing ADRs before deciding whether to create amend or supersede a record
    And  an ordinary implementation choice belongs in its appropriate existing artifact
    And  the guidance requires one concise qualification in the change assessment rather than repeated gate worksheets

  @adr-qualification-gate
  Scenario: Shared guidance alone does not qualify an ADR
    Given  the ADR qualification guidance is available
    When  an author reads the qualification criteria
    Then  the guidance labels `Must be shared` as supporting evidence only
    And  the guidance says shared evidence alone never qualifies a candidate
    And  the guidance routes a non-qualifying candidate to its appropriate existing artifact

  @adr-qualification-gate
  Scenario: The guidance routes normal design decisions to one canonical artifact
    Given  the ADR qualification guidance is available
    When  an author reads the lower-level artifact rules
    Then  the guidance names the existing artifacts that can own an ordinary decision
    And  the guidance requires the decision to have one complete home
    And  the guidance keeps the decision rationale in the ADR when it qualifies

  @adr-qualification-gate
  Scenario: The guidance requires an existing-ADR check before drafting
    Given  the ADR qualification guidance is available
    When  an author reads the duplicate and relationship rules
    Then  the guidance requires searching the active and archived ADR registers before choosing a route
    And  the guidance requires exactly one of NEW_ADR AMEND_ADR SUPERSEDE_ADR or an existing artifact
    And  the guidance forbids drafting an ADR before the existing-decision search

  @adr-qualification-gate
  Scenario: The guidance forbids invented ADR evidence
    Given  the ADR qualification guidance is available
    When  an author reads the evidence rules
    Then  the guidance requires facts established by the author or source artifacts
    And  the guidance forbids inventing alternatives impact consequences participants or provenance

  @adr-qualification-gate
  Scenario: An ADR refinement is routed to amendment without duplication
    Given  the ADR qualification guidance is available
    When  an author reads the refinement rules
    Then  the guidance says a refinement amends the existing ADR
    And  the guidance requires the existing record to be identified before amending
    And  the guidance forbids a duplicate qualification worksheet in the amended record

  @adr-qualification-gate
  Scenario: An architectural replacement preserves supersession history
    Given  the ADR qualification guidance is available
    When  an author reads the replacement rules
    Then  the guidance says a replacement supersedes the existing ADR
    And  the guidance requires an explicit replacement link from the old record
    And  the guidance keeps the superseded record's history in the archive

  Scenario: Arc42 impact does not by itself require an ADR
    Given the ADR qualification guidance is available
    When an author reads the Arc42 impact rules
    Then the guidance distinguishes chapter impact from ADR qualification
    And the guidance requires an affected chapter to be updated when appropriate
    And the guidance does not require a new ADR solely because a chapter is affected

  Scenario: The change assessment records ADR disposition separately from chapter impact
    Given the change-level Arc42 assessment template is available
    When a maintainer reads the ADR disposition marker
    Then the marker distinguishes chapter impact from ADR action
    And the marker allows `NONE` with target `none`
    And the marker does not duplicate the complete MADR procedure

  Scenario: The ADR register has one canonical ownership contract
    Given the consolidated ADR register guidance is available
    When a maintainer reads the register rules
    Then the active register contains accepted non-superseded architectural decisions only
    And non-ADR information is not copied into the ADR archive
    And current architecture facts are owned by Arc42 or the implementation
    And an ADR retains rationale and links instead of duplicating current facts

  @adr-qualification-gate
  Scenario: All effective guidance sources use the same ADR policy
    Given  the canonical Arc42 qualification procedure and design-review guidance are available
    When  an author compares the effective and generated guidance sources
    Then  each source defers to the same concise qualification and existing-ADR search
    And  shared guidance alone does not qualify an ADR
    And  derived sources do not copy qualification worksheets
    And  no source claims runtime enforcement of the qualification policy
    And  a configured project override remains effective

  @adr-qualification-gate
  Scenario: The ADR guidance requires an adversarial qualification record
    Given  the ADR qualification guidance is available
    When  an author prepares an ADR candidate
    Then  the guidance requires one concise assessment of the commitment real alternatives durable consequence and existing decisions
    And  the ADR owns the decision rationale alternatives and consequences
    And  neither the ADR nor design-review repeats the assessment as another qualification worksheet

  @e2e @adr-qualification-gate
  Scenario: An author requests the strict ADR guidance through the Givn CLI
    Given  an initialized ADR qualification project with active change `adr-qualification-gate`
    When  the author requests the Arc42 ADR guidance through the Givn CLI
    Then  the CLI exits with status 0
    And  the output explains qualification by architectural commitment real alternatives lasting consequence and existing-ADR search
    And  the output routes the result to a new amended superseding or non-ADR artifact
    And  the output requires qualification once in the change assessment
    And  the terminal transcript is captured for this scenario

  @adr-qualification-gate
  Scenario: A reviewer requests design-review guidance through the Givn CLI
    Given  an initialized ADR qualification project with active change `adr-qualification-gate`
    When  the reviewer requests the `design-review` guidance through the Givn CLI
    Then  the CLI exits with status 0
    And  the output directs the reviewer to the recorded qualification and existing-ADR search
    And  the output does not demand a duplicate qualification worksheet
    And  the output distinguishes authoring policy from runtime enforcement

  @adr-qualification-gate
  Scenario: The migration preserves independent architecture history across three proposed decisions
    Given  the remediation case carries the distinguishing marker "harness jib mandrel nozzle oboe yacht yarn zither"
    Given  the consolidated ADR register and canonical sources are available
    When  a maintainer compares the register with the canonical sources
    Then  the final register has 30 accepted decisions and no proposed decisions
    And  the change's migration manifest gives every assigned identifier exactly one disposition
    And  reserved identifiers `ADR-050` and `ADR-051` have no record
    And  non-ADR records are removed from the active register and ADR archive
    And  absorbed records are absent and their refinements remain readable
    And  superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
    And  `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
    And  the superseded semantic-engine records remain archived with replacement links to their replacement record

  @adr-qualification-gate
  Scenario: The guidance carries the three falsification tests
    Given  the ADR qualification guidance is available
    When  an author reads the qualification examples
    Then  reversible implementation choices and workflow-only changes without architectural consequence remain non-ADR examples
    And  a genuine lasting boundary decision with real alternatives may qualify
    And  examples explain the concise qualification rather than require three additional recorded tests
    And  qualification is by kind rather than a numeric ADR cap

  @adr-qualification-gate
  Scenario: The design template keeps decision justification in the design
    Given  the design template is available
    When  an author reads the Technology Decisions guidance
    Then  ordinary technical justification remains in the design
    And  ADR candidates refer to the concise change assessment instead of repeating qualification tests

  @adr-qualification-gate
  Scenario: The impact assessment records the verdict before drafting
    Given  the change-level Arc42 assessment template is available
    When  a maintainer reads the verdict placement rule
    Then  the assessment records the concise qualification and chosen route before ADR drafting
    And  a second structured verdict is not required in the ADR

  @adr-qualification-gate
  Scenario: The ADR template ships with no pre-passed qualification values
    Given  the ADR template is available
    When  an author prepares to copy the template
    Then  the template supplies valid frontmatter without pre-passed qualification claims
    And  the template records decision rationale alternatives and consequences without another qualification worksheet

  @adr-qualification-gate
  Scenario: Design-review requires the cheaper-home challenge for a new ADR
    Given  the design-review instruction is available
    When  a reviewer reads the ADR branch
    Then  the reviewer checks whether the recorded commitment belongs in an ADR or an existing lower-level artifact
    And  the reviewer uses the recorded qualification without another mandatory worksheet
    And  the design-review skill points to the canonical instruction

  Scenario: The chapter-selection guidance does not normalize row 9
    Given  the ADR qualification guidance is available
    When  an author reads the chapter-selection guidance
    Then  the guidance treats correlated rows as a double-check
    And  the guidance says an affected chapter does not by itself qualify an ADR
    And  the guidance does not normalize an ADR per change

  @adr-qualification-gate
  Scenario: Ordinary ADR downsides are not automatically risk debt
    Given  an architectural decision with a routine tradeoff and an actionable unresolved downside
    When  the author reads the consequence and risk-recording guidance
    Then  the ADR retains both consequences
    And  only the actionable unresolved downside requires risk or debt tracking
