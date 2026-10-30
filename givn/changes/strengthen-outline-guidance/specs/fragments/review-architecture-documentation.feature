@givn.delta @review-architecture-documentation
Feature: Review architecture documentation

  @givn.modified @e2e
  Scenario: The review template includes the Arc42 conformance block
    Given a project root with givn/ initialised
    When the user runs `givn eject review:template`
    Then the command succeeds
    And the ejected template records Arc42 implementation conformance against design tasks and completed implementation
    And the template records evidence and material mismatches
    And the template contains no outline verdict phrase
    And the terminal transcript is captured for this scenario

  @givn.modified @wip
  Scenario: Final architecture review uses the advisory outline without a second verdict
    Given an outline report is available for changed components
    When the reviewer reads the final architecture review guidance
    Then final review reuses relevant outline evidence from implementation unless structural changes have made it stale
    And only structurally stale relevant outline evidence requires rerunning the outline
    And the outline supports substantive source-confirmed architecture review
    And mixed-language coverage is disclosed without forcing outline analysis of unsupported components
    And unsupported components receive direct source assessment using the same architecture questions
    And no outline verdict metric waiver forced reviewer context or independent outline review is required
    And the outline alone cannot block review or archive

  @givn.added @wip
  Scenario: A changed unit unreadable from its outline rejects PASS
    Given an outline report is available for changed components
    When the reviewer reads the final architecture review guidance
    Then the reviewer must reconstruct responsibility and ordered call flow from the outline alone
    And names parameters return types and type names must state the domain operation
    And a generic or machinery name is a substantive readability finding
    And an unreadable changed unit rejects PASS until made readable
    And the report itself supplies no verdict metric waiver or second review context
