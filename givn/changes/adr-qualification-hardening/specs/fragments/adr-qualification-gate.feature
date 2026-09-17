@givn.delta @adr-qualification-gate

Feature: Strict qualification and canonical routing of architecture decisions

  @givn.added
  Scenario: The guidance carries the three falsification tests
    Given the ADR qualification guidance is available
    When an author reads the falsification tests
    Then the guidance fails a single-change reversible decision
    And the guidance fails a decision that constrains no future change
    And the guidance fails workflow behavior unless it also moves a boundary
    And the guidance names the misplaced-decision non-examples
    And the guidance still qualifies a genuine boundary decision
    And the guidance classifies by kind rather than count
    And the guidance states there is no numeric ADR cap

  @givn.added
  Scenario: The design template keeps decision justification in the design
    Given the design template is available
    When an author reads the Technology Decisions guidance
    Then the template says justification belongs in the design
    And the template forbids ADR candidacy unless the falsification tests pass

  @givn.added
  Scenario: The impact assessment records the verdict before drafting
    Given the change-level Arc42 assessment template is available
    When a maintainer reads the verdict placement rule
    Then the marker requires the structured verdict before drafting

  @givn.added
  Scenario: The ADR template ships with no pre-passed qualification values
    Given the ADR template is available
    When an author prepares to copy the template
    Then the template contains no pre-passed qualification verdict
    And the template instructs the author to fill the verdict from the record

  @givn.added
  Scenario: Design-review requires the cheaper-home challenge for a new ADR
    Given the design-review instruction is available
    When a reviewer reads the ADR branch
    Then the instruction requires the cheaper-home challenge for every proposed new ADR
    And the design-review command requires the cheaper-home challenge
    And the design-review skill points at the requirement

  @givn.added @wip
  Scenario: The chapter-selection guidance does not normalize row 9
    Given the ADR qualification guidance is available
    When an author reads the chapter-selection guidance
    Then the guidance treats correlated rows as a double-check
    And the guidance says an affected chapter does not by itself qualify an ADR
    And the guidance does not normalize an ADR per change
