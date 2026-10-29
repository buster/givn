@givn.delta @adr-qualification-gate
Feature: Concise qualification and canonical routing of architecture decisions

  @givn.modified @wip
  Scenario: The guidance defines strict qualification as an all-gates contract
    Given the ADR qualification guidance is available
    When an author reads the qualification gate
    Then an ADR requires an architectural commitment with real alternatives and a lasting boundary or consequence
    And the author must search existing ADRs before deciding whether to create amend or supersede a record
    And an ordinary implementation choice belongs in its appropriate existing artifact
    And the guidance requires one concise qualification in the change assessment rather than repeated gate worksheets

  @givn.removed
  Scenario: The structured verdict has fixed gate and route fields
    Given this scenario is removed

  @givn.removed
  Scenario: Route targets are conditional and unused targets stay empty
    Given this scenario is removed

  @givn.modified @wip
  Scenario: All effective guidance sources use the same ADR policy
    Given the canonical Arc42 qualification procedure and design-review guidance are available
    When an author compares the effective and generated guidance sources
    Then each source defers to the same concise qualification and existing-ADR search
    And shared guidance alone does not qualify an ADR
    And derived sources do not copy qualification worksheets
    And no source claims runtime enforcement of the qualification policy
    And a configured project override remains effective

  @givn.modified @wip
  Scenario: The ADR guidance requires an adversarial qualification record
    Given the ADR qualification guidance is available
    When an author prepares an ADR candidate
    Then the guidance requires one concise assessment of the commitment real alternatives durable consequence and existing decisions
    And the ADR owns the decision rationale alternatives and consequences
    And neither the ADR nor design-review repeats the assessment as another qualification worksheet

  @givn.modified @e2e @wip
  Scenario: An author requests the strict ADR guidance through the Givn CLI
    Given an initialized ADR qualification project with active change `adr-qualification-gate`
    When the author requests the Arc42 ADR guidance through the Givn CLI
    Then the CLI exits with status 0
    And the output explains qualification by architectural commitment real alternatives lasting consequence and existing-ADR search
    And the output routes the result to a new amended superseding or non-ADR artifact
    And the output requires qualification once in the change assessment

  @givn.modified @wip
  Scenario: A reviewer requests design-review guidance through the Givn CLI
    Given an initialized ADR qualification project with active change `adr-qualification-gate`
    When the reviewer requests the `design-review` guidance through the Givn CLI
    Then the CLI exits with status 0
    And the output directs the reviewer to the recorded qualification and existing-ADR search
    And the output does not demand a duplicate qualification worksheet
    And the output distinguishes authoring policy from runtime enforcement

  @givn.modified @wip
  Scenario: The impact assessment records the verdict before drafting
    Given the change-level Arc42 assessment template is available
    When a maintainer reads the verdict placement rule
    Then the assessment records the concise qualification and chosen route before ADR drafting
    And a second structured verdict is not required in the ADR

  @givn.modified @wip
  Scenario: The ADR template ships with no pre-passed qualification values
    Given the ADR template is available
    When an author prepares to copy the template
    Then the template supplies valid frontmatter without pre-passed qualification claims
    And the template records decision rationale alternatives and consequences without another qualification worksheet

  @givn.modified @wip
  Scenario: The design template keeps decision justification in the design
    Given the design template is available
    When an author reads the Technology Decisions guidance
    Then ordinary technical justification remains in the design
    And ADR candidates refer to the concise change assessment instead of repeating qualification tests

  @givn.modified @wip
  Scenario: Design-review requires the cheaper-home challenge for a new ADR
    Given the design-review instruction is available
    When a reviewer reads the ADR branch
    Then the reviewer checks whether the recorded commitment belongs in an ADR or an existing lower-level artifact
    And the reviewer uses the recorded qualification without another mandatory worksheet
    And the design-review skill points to the canonical instruction

  @givn.added @wip
  Scenario: Ordinary ADR downsides are not automatically risk debt
    Given an architectural decision with a routine tradeoff and an actionable unresolved downside
    When the author reads the consequence and risk-recording guidance
    Then the ADR retains both consequences
    And only the actionable unresolved downside requires risk or debt tracking

  @givn.modified @wip
  Scenario: The guidance carries the three falsification tests
    Given the ADR qualification guidance is available
    When an author reads the qualification examples
    Then reversible implementation choices and workflow-only changes without architectural consequence remain non-ADR examples
    And a genuine lasting boundary decision with real alternatives may qualify
    And examples explain the concise qualification rather than require three additional recorded tests
    And qualification is by kind rather than a numeric ADR cap
