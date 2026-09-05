@givn.delta @ideation-skill

Feature: ideation-skill
  The ideation facilitation skill ships through skill synchronization.

  @givn.added
  Scenario: The ideation skill ships without any addon enabled
    Given a project root with givn/ initialised without addons
    When the user runs `givn skills sync`
    Then the generated givn-ideation guidance contains "Frame"

  @givn.added
  Scenario: The ideation guidance carries the phase ladder and the one-question rule
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-ideation guidance contains "Event Storming"
    And the generated givn-ideation guidance contains "Use Case Definition"
    And the generated givn-ideation guidance contains "Use Case Elaboration"
    And the generated givn-ideation guidance contains "Handoff"
    And the generated givn-ideation guidance contains "one question at a time"
    And the generated givn-ideation guidance contains "quality review"

  @givn.added
  Scenario: The ideation guidance binds personas to the artifacts that affect them
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-ideation guidance contains "stakeholder type"
    And the generated givn-ideation guidance contains "persona segment"
    And the generated givn-ideation guidance contains "persona promotion"
    And the generated givn-ideation guidance contains "every artifact that affects"

  @givn.added
  Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-ideation guidance contains "givn/ideation/"
    And the generated givn-ideation guidance contains "open question"
    And the generated givn-ideation guidance contains "change seed"
    And the generated givn-ideation guidance contains "session.md"
    And the generated givn-ideation guidance contains "docs/ideation/"
    And the generated givn-ideation guidance contains "Handoff never starts a change"

  @givn.added @wip
  Scenario: The explore guidance points to ideation for structured domain work
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then the generated givn-explore guidance contains "givn-ideation"
    And the generated givn-explore command contains "givn-ideation"

  @givn.added @e2e @wip
  Scenario: A consumer synchronizes skills and every agent target receives the ideation facilitation skill
    Given a givn project with all agent skill targets enabled
    When the user runs `givn skills sync`
    Then stdout contains "Syncing skills and agent commands..."
    And stdout contains "synced skill: givn-ideation"
    And the generated givn-ideation guidance contains "Frame"
    And the generated givn-ideation command contains "ideation"