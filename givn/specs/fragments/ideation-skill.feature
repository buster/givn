Feature: ideation-skill

  Scenario: The ideation skill ships without any addon enabled
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Frame"

  Scenario: The ideation guidance carries the phase ladder and the one-question rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Event Storming"
    And  the generated givn-ideation guidance contains "Use Case Definition"
    And  the generated givn-ideation guidance contains "Use Case Elaboration"
    And  the generated givn-ideation guidance contains "Handoff"
    And  the generated givn-ideation guidance contains "one question at a time"
    And  the generated givn-ideation guidance contains "quality review"
    And  the generated givn-ideation guidance contains "givn instructions dialog"
    And  the generated givn-ideation guidance contains "In practice:"
    And  the generated givn-ideation guidance contains "Rejected:"

  Scenario: The ideation guidance binds personas to the artifacts that affect them
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "stakeholder type"
    And  the generated givn-ideation guidance contains "persona segment"
    And  the generated givn-ideation guidance contains "persona promotion"
    And  the generated givn-ideation guidance contains "every artifact that affects"

  Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "givn/ideation/"
    And  the generated givn-ideation guidance contains "open question"
    And  the generated givn-ideation guidance contains "change seed"
    And  the generated givn-ideation guidance contains "session.md"
    And  the generated givn-ideation guidance contains "docs/ideation/"
    And  the generated givn-ideation guidance contains "Handoff never starts a change"

  Scenario: The explore guidance points to ideation for structured domain work
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-explore guidance contains "givn-ideation"
    And  the generated givn-explore command contains "givn-ideation"

  @e2e
  Scenario: A consumer synchronizes skills and every agent target receives the ideation facilitation skill
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  stdout contains "Syncing skills and agent commands..."
    And  stdout contains "synced skill: givn-ideation"
    And  the generated givn-ideation guidance contains "Frame"
    And  the generated givn-ideation command contains "ideation"
  Scenario: The ideation guidance carries the seed schema and its status values
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "change seed"
    And  the generated givn-ideation guidance contains "Status: ready"
    And  the generated givn-ideation guidance contains "proposed by"
    And  the generated givn-ideation guidance contains "implemented in"
    And  the generated givn-ideation guidance contains "superseded by"
    And  the generated givn-ideation guidance contains "withdrawn"
    And  the generated givn-ideation guidance contains "Accepted decisions"
    And  the generated givn-ideation guidance contains "Suggested start"
  Scenario: The ideation guidance carries the term schema and the collision procedure
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "domain-terms.md"
    And  the generated givn-ideation guidance contains "`candidate`, `confirmed`, or `rejected`"
    And  the generated givn-ideation guidance contains "Contrast"
    And  the generated givn-ideation guidance contains "First use"
    And  the generated givn-ideation guidance contains "definition and first use"
    And  the generated givn-ideation guidance contains "recorded collision check"
    And  the generated givn-ideation guidance contains "only `confirmed` terms"
    And  the generated givn-ideation guidance contains "never deleted"
    And  the generated givn-ideation guidance contains "givn-domain-modeling"
    And  the generated givn-ideation guidance contains "Collision check"
    And  the generated givn-ideation guidance contains "Grep the durable glossary"
    And  the generated givn-ideation guidance contains "without recorded commands is not a record"
