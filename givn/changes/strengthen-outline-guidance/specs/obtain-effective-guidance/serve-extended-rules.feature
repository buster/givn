@givn.delta @serve-extended-rules
Feature: The served instruction carries the built-in rules and the project's extension

  @givn.modified @wip
  Scenario: Refactor guidance uses the outline once for supported changed components and their neighbors
    Given a behavior batch changes supported and unsupported components
    When the author reads the REFACTOR guidance
    Then supported changed components receive one advisory outline inspection per batch
    And the assessment discloses which languages and components the outline covers and which it omits
    And unsupported components receive direct source assessment without forced outline analysis
    And the assessment asks whether responsibilities are cohesive public interfaces narrow dependencies correctly directed and decomposition understandable
    And suspected problems are confirmed against source before conclusions or edits
    And structural call order is not represented as a complete runtime trace
    And uncertain or omitted edges limit the conclusions
    And a later structural edit is the reason to regenerate a stale outline rather than every small edit
    And the report is reused until structural changes make its relevant evidence stale
    And nonstructural edits alone do not require rerunning the outline

  @givn.modified @wip
  Scenario: Unsupported outline languages receive source assessment without a waiver
    Given a behavior batch changes unsupported components
    When the author reads the REFACTOR guidance
    Then the same responsibility interface dependency and decomposition questions are assessed directly against source
    And unavailable outline support is not failure or a reason for a waiver
    And no outline verdict metric waiver or forced reviewer context is required

  @givn.added @wip
  Scenario: Refactor guidance treats an unreadable changed unit as a finding
    Given a behavior batch changes a unit whose outline cannot carry its story
    When the author reads the REFACTOR guidance
    Then the outline must state the unit's responsibility and ordered call flow without bodies or comments
    And unit type method parameter return and called-type names must state the domain operation
    And generic machinery names are readability findings
    And metrics and classifications never overrule an unreadable outline
    And source confirms a suspected problem rather than substituting for the outline story
    And an unreadable changed unit is fixed in REFACTOR or recorded as a substantive review finding

  @givn.added @wip
  Scenario: Implementation guidance routes to the shared step mechanics
    Given a Givn project with an active change
    When the author requests the tasks instruction
    Then the served guidance names givn instructions steps as the step mechanics
