Feature: Review architecture documentation

  @e2e @review-architecture-documentation
  Scenario: Arc42-enabled review instructions require conformance evidence
    Given  a project root with givn/ initialised
    When  the user runs `givn new arc42-review-alignment-test`
    And  the user runs `givn instructions review --change arc42-review-alignment-test`
    Then  the command succeeds
    And  the returned guidance compares Arc42 design tasks and completed implementation against available evidence
    And  the guidance requires recording and resolving material conformance mismatches
    And  the guidance does not require an unwritten review verdict or future archive receipt as evidence
    And  the terminal transcript is captured for this scenario

  @e2e @review-architecture-documentation
  Scenario: The review template includes the Arc42 conformance block
    Given  a project root with givn/ initialised
    When  the user runs `givn eject review:template`
    Then  the command succeeds
    And  the ejected template records Arc42 implementation conformance against design tasks and completed implementation
    And  the template records evidence and material mismatches
    And  the template contains no outline verdict phrase
    And  the terminal transcript is captured for this scenario

  @e2e
  Scenario: Generated review guidance stays aligned after skill sync
    Given  a project root with givn/ initialised
    And  an existing change named "review-alignment"
    When  the user runs `givn skills sync`
    Then  the command succeeds
    And  the generated review skill directs the reviewer to the review instruction
    And  the generated review skill carries no Arc42 implementation conformance section
    And  the served review instruction contains "Arc42 implementation conformance"

  Scenario: Arc42-disabled review instructions mark conformance as not applicable
    Given a project root with givn/ initialised without addons
    When the user runs `givn new arc42-review-alignment-test`
    And the user runs `givn instructions review --change arc42-review-alignment-test`
    Then the command succeeds
    And stdout contains "N/A when the arc42 addon is disabled"

  @review-architecture-documentation
  Scenario: The first completed Arc42 change establishes a real twelve-chapter baseline
    Given  a project using the Arc42 addon without a completed architecture baseline
    When  the author reads the architecture documentation guidance
    Then  the first completed change must establish all twelve chapters proportionate to the real project
    And  unfinished scaffold and fabricated completeness do not satisfy the baseline
    And  specific explicit unknowns and explained inapplicability are acceptable content

  @review-architecture-documentation
  Scenario: Later Arc42 changes maintain affected chapters rather than reapprove the baseline
    Given  a project with a completed twelve-chapter Arc42 baseline
    When  the author reads the architecture maintenance guidance for a later change
    Then  affected chapters must reflect the implemented change
    And  unchanged chapters do not require repeated whole-baseline approval

  @review-architecture-documentation
  Scenario: Architecture review checks substantive boundaries from the first implementation
    Given  a change's first implementation is ready for review
    When  the reviewer reads the architecture conformance guidance
    Then  review checks cohesive responsibilities narrow consumer contracts external adapters separate composition and explicit dependency direction
    And  review confirms actual imports and calls rather than inferring boundaries from filenames
    And  narrow module APIs may satisfy a contract while explicit interfaces are required where isolation or substitution needs them
    And  confirmed Personas remain stakeholder review lenses without invented identities or an internal-change exemption

  @review-architecture-documentation
  Scenario: Review closure is bounded to findings and regressions from their fixes
    Given  one substantive review has recorded material findings and minor naming observations
    When  the reviewer reads the closure guidance
    Then  one targeted closure checks the findings and material regressions introduced by their fixes
    And  critical defects remain repairable blockers
    And  minor naming style polish and unrelated findings go to the existing backlog or a separate follow-up batch
    And  closure does not initiate another open-ended audit or redundant full-suite run

  @review-architecture-documentation
  Scenario: Final architecture review uses the advisory outline without a second verdict
    Given  an outline report is available for changed components
    When  the reviewer reads the final architecture review guidance
    Then  final review reuses relevant outline evidence from implementation unless structural changes have made it stale
    And  only structurally stale relevant outline evidence requires rerunning the outline
    And  the outline supports substantive source-confirmed architecture review
    And  mixed-language coverage is disclosed without forcing outline analysis of unsupported components
    And  unsupported components receive direct source assessment using the same architecture questions
    And  no outline verdict metric waiver forced reviewer context or independent outline review is required
    And  the outline alone cannot block review or archive

  @review-architecture-documentation
  Scenario: A changed unit unreadable from its outline rejects PASS
    Given  an outline report is available for changed components
    When  the reviewer reads the final architecture review guidance
    Then  the reviewer must reconstruct responsibility and ordered call flow from the outline alone
    And  names parameters return types and type names must state the domain operation
    And  a generic or machinery name is a substantive readability finding
    And  an unreadable changed unit rejects PASS until made readable
    And  the report itself supplies no verdict metric waiver or second review context
