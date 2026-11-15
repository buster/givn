@givn.delta @instruction-integrity

Feature: Instruction and skill guidance remains complete and machine-readable

  @givn.added @wip
  Scenario: Every instruction id a served instruction names resolves
    Given  a Givn project with an active change and the guidance reference check available
    When  the maintainer runs the guidance reference check over every served instruction
    Then  every instruction id a served instruction names resolves in the context that instruction needs
    And  every instruction id an effective addon instruction names resolves
    And  the check names no unresolved instruction id

  @givn.added @wip
  Scenario: A dangling stack instruction id fails the reference check by name
    Given  a Givn project with an active change
    And  a steps instruction override that names "steps-rustt"
    When  the maintainer runs the guidance reference check over the served steps guidance
    Then  the check fails
    And  it names "steps-rustt"

  @givn.added @wip
  Scenario: An artifact instruction named without its change context is a finding
    Given  a Givn project with an active change and the guidance reference check available
    And  a served instruction names "proposal" without a change context
    When  the maintainer runs the guidance reference check over the served guidance
    Then  the check fails
    And  it names "proposal" and the missing change context

  @givn.added @wip
  Scenario: A policy instruction named with a change context is a finding
    Given  a Givn project with an active change and the guidance reference check available
    And  a served instruction names "steps" with a change context
    When  the maintainer runs the guidance reference check over the served guidance
    Then  the check fails
    And  it names "steps" and the extra change context

  @givn.added @wip
  Scenario: The ise instruction names only instruction ids that resolve
    Given  a Givn project with the ise addon enabled and the guidance reference check available
    When  the maintainer requests the ise instruction and runs the reference check over it
    Then  the served text names "steps" as the mapping for the change's stack mechanics
    And  the served text names "steps-rust" as one stack instruction example
    And  the served text names the artifact instruction that owns the change-strategy record and the one that owns the spec-gap record
    And  every named instruction id resolves

  @givn.added @wip
  Scenario: The review guidance keeps every surviving obligation
    Given  a Givn project with an active change
    When  the maintainer requests the review instructions through the Givn CLI
    Then  the served text carries "`@e2e` tag integrity"
    And  the served text carries "explicit E2E scope selection"
    And  the served text carries "receipt reading"
    And  the served text carries "opposite classification"
    And  the served text carries "local run command"
    And  the served text carries "second `@e2e` file"
    And  the served text carries "Design match:"
    And  the served text carries "Interaction coverage maps"
    And  the served text carries "no fourth bucket"
