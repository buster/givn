@givn.delta @generate-ideation-guidance

Feature: Generate ideation guidance

  @givn.modified
  Scenario: The ideation skill ships without any addon enabled
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn skills sync`
    Then  the served ideation instruction contains "Frame"
    And  the generated givn-ideation guidance directs the ideator to the ideation instruction

  @givn.modified
  Scenario: The ideation guidance carries the phase ladder and the one-question rule
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "Event Storming"
    And  the served ideation instruction contains "Use Case Definition"
    And  the served ideation instruction contains "Use Case Elaboration"
    And  the served ideation instruction contains "Handoff"
    And  the served ideation instruction contains "one question at a time"
    And  the served ideation instruction contains "quality review"
    And  the served ideation instruction contains "givn instructions dialog"
    And  the served ideation instruction contains "In practice:"
    And  the served ideation instruction contains "Rejected:"

  @givn.modified
  Scenario: The ideation guidance binds personas to the artifacts that affect them
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "stakeholder type"
    And  the served ideation instruction contains "persona segment"
    And  the served ideation instruction contains "persona promotion"
    And  the served ideation instruction contains "every artifact that affects"

  @givn.modified
  Scenario: The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "givn/ideation/"
    And  the served ideation instruction contains "open question"
    And  the served ideation instruction contains "change seed"
    And  the served ideation instruction contains "session.md"
    And  the served ideation instruction contains "docs/ideation/"
    And  the served ideation instruction contains "Handoff never starts a change"

  @givn.modified
  Scenario: The explore guidance points to ideation for structured domain work
    Given  a Givn project
    When  the agent requests the explore instructions through the Givn CLI
    Then  the served explore instruction contains "givn-ideation"

  @givn.modified @e2e @wip
  Scenario: A consumer synchronizes skills and every agent target receives the ideation facilitation skill
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  stdout contains "Syncing skills and agent commands..."
    And  stdout contains "synced skill: givn-ideation"
    And  the served ideation instruction contains "Frame"
    And  the generated givn-ideation command directs the ideator to the ideation instruction

  @givn.modified @wip
  Scenario: The ideation guidance carries the seed schema and its status values
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "change seed"
    And  the served ideation instruction contains "Status: ready"
    And  the served ideation instruction contains "proposed by"
    And  the served ideation instruction contains "implemented in"
    And  the served ideation instruction contains "superseded by"
    And  the served ideation instruction contains "withdrawn"
    And  the served ideation instruction contains "Accepted decisions"
    And  the served ideation instruction contains "Suggested start"

  @givn.modified @wip
  Scenario: The ideation guidance carries the term schema and the collision procedure
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "domain-terms.md"
    And  the served ideation instruction contains "`candidate`, `confirmed`, or `rejected`"
    And  the served ideation instruction contains "Contrast"
    And  the served ideation instruction contains "First use"
    And  the served ideation instruction contains "definition and first use"
    And  the served ideation instruction contains "recorded collision check"
    And  the served ideation instruction contains "only `confirmed` terms"
    And  the served ideation instruction contains "never deleted"
    And  the served ideation instruction contains "givn-domain-modeling"
    And  the served ideation instruction contains "Collision check"
    And  the served ideation instruction contains "Grep the durable glossary"
    And  the served ideation instruction contains "without recorded commands is not a record"

  @givn.modified @wip
  Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "contract between one primary actor and the system"
    And  the served ideation instruction contains "One goal, one primary actor"
    And  the served ideation instruction contains "Goal-shaped, never component-shaped"
    And  the served ideation instruction contains "Level discipline"
    And  the served ideation instruction contains "promotion rule"
    And  the served ideation instruction contains "three or more distinct actor goals"
    And  the served ideation instruction contains "Placement test"
    And  the served ideation instruction contains "Mutation containment"
    And  the served ideation instruction contains "Extension discipline"
    And  the served ideation instruction contains "Precondition and flow consistency"
    And  the served ideation instruction contains "Implementation independence"
    And  the served ideation instruction contains "Relationship semantics"

  @givn.modified @wip
  Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "the first actor is the primary actor"
    And  the served ideation instruction contains "Supporting:"
    And  the served ideation instruction contains "protects <guarantee>"
    And  the served ideation instruction contains "never an extension"
    And  the served ideation instruction contains "never to a capability inside another goal"

  @givn.modified @wip
  Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "authority boundary"
    And  the served ideation instruction contains "evaluation capability"
    And  the served ideation instruction contains "own vocabulary"
    And  the served ideation instruction contains "topic stance"
    And  the served ideation instruction contains "inventory"
    And  the served ideation instruction contains "Reuse:"
    And  the served ideation instruction contains "New:"
    And  the served ideation instruction contains "append-only"
    And  the served ideation instruction contains "one persona with three stance blocks"
    And  the served ideation instruction contains "record entries only"

  @givn.modified @wip
  Scenario: The ideation guidance keeps the confirmed persona behaviors
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "stakeholder type"
    And  the served ideation instruction contains "persona segment"
    And  the served ideation instruction contains "persona promotion"
    And  the served ideation instruction contains "every artifact that affects"
    And  the served ideation instruction contains "at most three"
    And  the served ideation instruction contains "never auto-promote"
    And  the served ideation instruction contains "confirms every persona"
    And  the served ideation instruction contains "arbitrates"
    And  the served ideation instruction contains "inventory"
    And  the served ideation instruction contains "match"
    And  the served ideation instruction contains "separate from Actors"

  @givn.modified @wip
  Scenario: The ideation guidance carries the open-question qualifier rule
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "(open: Q<N>)"
    And  the served ideation instruction contains "must not be presented as unconditional"
    And  the served ideation instruction contains "removes the qualifier"

  @givn.modified @wip
  Scenario: The ideation guidance carries the live-question standard
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "A bare option list is a defect"
    And  the served ideation instruction contains "the decision it blocks"
    And  the served ideation instruction contains "context that forces it"

  @givn.modified @wip
  Scenario: The ideation guidance stops questions at decided directions
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "never asked again"
    And  the served ideation instruction contains "applied and reported"
    And  the served ideation instruction contains "phase report"
    And  the served ideation instruction contains "only when a new fork"
    And  the served ideation instruction contains "share one direction are one question"

  @givn.modified @wip
  Scenario: The frame is confirmed once and later phase boundaries report
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "Confirm the Frame"
    And  the served ideation instruction contains "report what was recorded and proceed"

  @givn.modified @wip
  Scenario: Handoff cuts change seeds by deliverable guarantee
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "deliverable guarantee"
    And  the served ideation instruction contains "true end to end"
    And  the served ideation instruction contains "can ship alone"
    And  the served ideation instruction contains "may cover several use cases"
    And  the served ideation instruction contains "one use case may yield several seeds"
    And  the served ideation instruction contains "confirmed rule untrue"
    And  the served ideation instruction contains "dependencies and a suggested start order"
    And  the served ideation instruction contains "changes/<change-id>.md"

  @givn.modified @wip
  Scenario: The ideation command guardrails carry the same conduct rules
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "deliverable guarantee"
    And  the served ideation instruction contains "never asked again"

  @givn.modified @wip
  Scenario: The ideation guidance requires a canvas with distinguishable element kinds
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "one or more mermaid diagrams"
    And  the served ideation instruction contains "show every event, command, aggregate, and actor, with the kinds"
    And  the served ideation instruction contains "(per-node kind markers or a legend)"
    And  the served ideation instruction contains "the actor that issues it and the aggregate it changes"
    And  the served ideation instruction contains "A text chain without a canvas is an incomplete event storming"
    And  the served ideation instruction contains "the canvas must render"
    And  the served ideation instruction contains "drawn as a rendered mermaid canvas"
    And  the served ideation instruction contains "aggregate (the consistency boundary the command changes)"

  @givn.modified @wip
  Scenario: The ideation guidance keeps aggregates to declared consistency boundaries
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "Aggregates are given, not minted"
    And  the served ideation instruction contains "a domain-noun root that receives commands and emits events under one"
    And  the served ideation instruction contains "confirmed before the canvas relies on it"
    And  the served ideation instruction contains "Reuse the aggregates the project already declares"
    And  the served ideation instruction contains "never an aggregate"
    And  the served ideation instruction contains "Name the kind"

  @givn.modified @wip
  Scenario: The event storming review checks canvas coverage, legality, agreement, and rendering
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "canvas that shows every event, command, aggregate, and actor with its kinds"
    And  the served ideation instruction contains "every aggregate a declared consistency boundary or a"
    And  the served ideation instruction contains "drawn as an aggregate is a defect"
    And  the served ideation instruction contains "the lines and the picture naming the"
    And  the served ideation instruction contains "a canvas that renders"

  @givn.modified @wip
  Scenario: The ideation command guardrails require the canvas and legal aggregates
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "rendered mermaid canvas naming every event"
    And  the served ideation instruction contains "every aggregate is a declared"
    And  the served ideation instruction contains "never a record or a specification"

  @givn.modified @wip
  Scenario: The ideation guidance binds every confirmed term to the user's verified name
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "the user's verified name"

  @givn.modified @wip
  Scenario: The ideation guidance asks the naming question as soon as a name fails
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "generic, overloaded"
    And  the served ideation instruction contains "cannot fill its genus slot"
    And  the served ideation instruction contains "ask the naming question immediately"
    And  the served ideation instruction contains "is collision-checked like any other"

  @givn.modified @wip
  Scenario: The ideation guidance asks the term check before a phase closes
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "The user verifies every new core term before it is used"
    And  the served ideation instruction contains "Term check — <phase>"
    And  the served ideation instruction contains "Confirm all, or name the terms to revisit"
    And  the served ideation instruction contains "one-sentence description"
    And  the served ideation instruction contains "Confirm all makes them `confirmed`"
    And  the served ideation instruction contains "naming dialog, one at a time, before confirmation"
    And  the served ideation instruction contains "the other terms stand"
    And  the served ideation instruction contains "a replaced name becomes an anti-term"

  @givn.modified @wip
  Scenario: The verified-name gate holds without the addon
    Given  a project root with givn/ initialised without addons
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "the user's verified name"
    And  the served ideation instruction contains "Term check — <phase>"

  @givn.modified @wip
  Scenario: The ideation command guardrails carry the verified-name rule
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "verifies every new core term"
    And  the served ideation instruction contains "phase term check"
    And  the served ideation instruction contains "naming dialog"
    And  the served ideation instruction contains "before the canvas, use cases, or seeds rely on it"

  @givn.modified @wip
  Scenario: The ideation guidance carries the persona schema
    Given  a Givn project
    When  the agent requests the ideation instructions through the Givn CLI
    Then  the served ideation instruction contains "Stable core"
    And  the served ideation instruction contains "Authority boundary"
    And  the served ideation instruction contains "Evaluation capability"
    And  the served ideation instruction contains "Own vocabulary"
    And  the served ideation instruction contains "Success criterion"
    And  the served ideation instruction contains "Running record"
    And  the served ideation instruction contains "append-only"
    And  the served ideation instruction contains "Topic stances"

  @givn.added @wip
  Scenario: The generated ideation files direct the agent to the ideation instruction
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance directs the ideator to the ideation instruction
    And  the generated givn-ideation command directs the ideator to the ideation instruction
