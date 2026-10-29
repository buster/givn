@givn.delta @review-architecture-documentation
Feature: Review architecture documentation

  @givn.modified @e2e @wip
  Scenario: Arc42-enabled review instructions require conformance evidence
    Given a project root with givn/ initialised
    When the user runs `givn new arc42-review-alignment-test`
    And the user runs `givn instructions review --change arc42-review-alignment-test`
    Then the command succeeds
    And the returned guidance compares Arc42 design tasks and completed implementation against available evidence
    And the guidance requires recording and resolving material conformance mismatches
    And the guidance does not require an unwritten review verdict or future archive receipt as evidence

  @givn.modified @e2e @wip
  Scenario: The review template includes the Arc42 conformance block
    Given a project root with givn/ initialised
    When the user runs `givn eject review:template`
    Then the command succeeds
    And the ejected template records Arc42 implementation conformance against design tasks and completed implementation
    And the template records evidence and material mismatches without an outline verdict or metric waiver

  @givn.added @wip
  Scenario: The first completed Arc42 change establishes a real twelve-chapter baseline
    Given a project using the Arc42 addon without a completed architecture baseline
    When the author reads the architecture documentation guidance
    Then the first completed change must establish all twelve chapters proportionate to the real project
    And unfinished scaffold and fabricated completeness do not satisfy the baseline
    And specific explicit unknowns and explained inapplicability are acceptable content

  @givn.added @wip
  Scenario: Later Arc42 changes maintain affected chapters rather than reapprove the baseline
    Given a project with a completed twelve-chapter Arc42 baseline
    When the author reads the architecture maintenance guidance for a later change
    Then affected chapters must reflect the implemented change
    And unchanged chapters do not require repeated whole-baseline approval

  @givn.added @wip
  Scenario: Architecture review checks substantive boundaries from the first implementation
    Given a change's first implementation is ready for review
    When the reviewer reads the architecture conformance guidance
    Then review checks cohesive responsibilities narrow consumer contracts external adapters separate composition and explicit dependency direction
    And review confirms actual imports and calls rather than inferring boundaries from filenames
    And narrow module APIs may satisfy a contract while explicit interfaces are required where isolation or substitution needs them
    And confirmed Personas remain stakeholder review lenses without invented identities or an internal-change exemption

  @givn.added @wip
  Scenario: Review closure is bounded to findings and regressions from their fixes
    Given one substantive review has recorded material findings and minor naming observations
    When the reviewer reads the closure guidance
    Then one targeted closure checks the findings and material regressions introduced by their fixes
    And critical defects remain repairable blockers
    And minor naming style polish and unrelated findings go to the existing backlog or a separate follow-up batch
    And closure does not initiate another open-ended audit or redundant full-suite run

  @givn.added @wip
  Scenario: Final architecture review uses the advisory outline without a second verdict
    Given an outline report is available for changed components
    When the reviewer reads the final architecture review guidance
    Then final review reuses relevant outline evidence from implementation unless structural changes have made it stale
    And only structurally stale relevant outline evidence requires rerunning the outline
    And the outline supports substantive source-confirmed architecture review
    And mixed-language coverage is disclosed without forcing outline analysis of unsupported Go or Java components
    And unsupported components receive direct source assessment using the same architecture questions
    And no outline verdict metric waiver forced reviewer context or independent outline review is required
    And the outline alone cannot block review or archive
