@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added
  Scenario: The proposal instruction carries the question disposition rule
    Given the proposal instructions are requested
    When an author reads the question disposition rule
    Then the guidance requires each intersecting seed question to be dispositioned
    And the guidance names the answered carried and out-of-scope dispositions
    And the guidance rejects a bare deferral to the design
    And the guidance assigns stable question IDs
    And the guidance marks the disposition rule advisory without an ideation source

  @givn.added
  Scenario: The design and design-review guidance carry the decision ledger
    Given the design and design-review instructions are requested
    When a maintainer reads the ledger contract
    Then the guidance requires a decisions ledger mapping each question ID
    And the guidance records decisions as "Decides D" with their realization
    And the guidance records the seed question reference beside the ledger ID
    And the design-review guidance treats an unresolved unrecorded question as a finding

  @givn.added @wip
  Scenario: The review instruction rejects an unqualified guarantee with an open prerequisite
    Given the review instructions are requested
    When the reviewer reads the use-case scope criteria
    Then the guidance rejects an unqualified guarantee whose prerequisite is open

  @givn.added @wip
  Scenario: The proposal design and design-review templates carry the ledger shapes
    Given a givn project
    When the user runs `givn eject proposal:template`
    And the user runs `givn eject design:template`
    And the user runs `givn eject design-review:template`
    Then the file "givn/artifacts/proposal/proposal.md" contains "Disposition"
    And the file "givn/artifacts/design/design.md" contains "## Decisions"
    And the file "givn/artifacts/design/design.md" contains "Decides D"
    And the file "givn/artifacts/design-review/design-review.md" contains "ledger"
