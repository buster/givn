Feature: Generate ideation guidance

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
  Scenario: The ideation guidance carries the canonical definition and the eight scoping rules
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "contract between one primary actor and the system"
    And  the generated givn-ideation guidance contains "One goal, one primary actor"
    And  the generated givn-ideation guidance contains "Goal-shaped, never component-shaped"
    And  the generated givn-ideation guidance contains "Level discipline"
    And  the generated givn-ideation guidance contains "promotion rule"
    And  the generated givn-ideation guidance contains "three or more distinct actor goals"
    And  the generated givn-ideation guidance contains "Placement test"
    And  the generated givn-ideation guidance contains "Mutation containment"
    And  the generated givn-ideation guidance contains "Extension discipline"
    And  the generated givn-ideation guidance contains "Precondition and flow consistency"
    And  the generated givn-ideation guidance contains "Implementation independence"
    And  the generated givn-ideation guidance contains "Relationship semantics"

  Scenario: The ideation guidance carries the authoring requirements and the handoff mapping
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "the first actor is the primary actor"
    And  the generated givn-ideation guidance contains "Supporting:"
    And  the generated givn-ideation guidance contains "protects <guarantee>"
    And  the generated givn-ideation guidance contains "never an extension"
    And  the generated givn-ideation guidance contains "never to a capability inside another goal"
  Scenario: The ideation guidance carries the persona identity criteria and the reuse protocol
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "authority boundary"
    And  the generated givn-ideation guidance contains "evaluation capability"
    And  the generated givn-ideation guidance contains "own vocabulary"
    And  the generated givn-ideation guidance contains "topic stance"
    And  the generated givn-ideation guidance contains "inventory"
    And  the generated givn-ideation guidance contains "Reuse:"
    And  the generated givn-ideation guidance contains "New:"
    And  the generated givn-ideation guidance contains "append-only"
    And  the generated givn-ideation guidance contains "one persona with three stance blocks"
    And  the generated givn-ideation guidance contains "record entries only"

  Scenario: The ideation guidance keeps the confirmed persona behaviors
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "stakeholder type"
    And  the generated givn-ideation guidance contains "persona segment"
    And  the generated givn-ideation guidance contains "persona promotion"
    And  the generated givn-ideation guidance contains "every artifact that affects"
    And  the generated givn-ideation guidance contains "at most three"
    And  the generated givn-ideation guidance contains "never auto-promote"
    And  the generated givn-ideation guidance contains "confirms every persona"
    And  the generated givn-ideation guidance contains "arbitrates"
    And  the generated givn-ideation command contains "inventory"
    And  the generated givn-ideation command contains "match"
    And  the generated givn-ideation command contains "separate from Actors"

  Scenario: The persona template carries the stable core and the running record
    Given  the persona template is inspected
    When  an author reads the persona schema
    Then  the template names the authority boundary and evaluation capability
    And  the template names the own vocabulary and success criterion
    And  the template carries an append-only running record
    And  the template carries a dated topic stances appendix
  Scenario: The ideation guidance carries the open-question qualifier rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "(open: Q<N>)"
    And  the generated givn-ideation guidance contains "must not be presented as unconditional"
    And  the generated givn-ideation guidance contains "removes the qualifier"
  Scenario: The ideation guidance carries the live-question standard
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "A bare option list is a defect"
    And  the generated givn-ideation guidance contains "the decision it blocks"
    And  the generated givn-ideation guidance contains "context that forces it"

  Scenario: The ideation guidance stops questions at decided directions
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "never asked again"
    And  the generated givn-ideation guidance contains "applied and reported"
    And  the generated givn-ideation guidance contains "phase report"
    And  the generated givn-ideation guidance contains "only when a new fork"
    And  the generated givn-ideation guidance contains "share one direction are one question"

  Scenario: The frame is confirmed once and later phase boundaries report
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Confirm the Frame"
    And  the generated givn-ideation guidance contains "report what was recorded and proceed"

  Scenario: Handoff cuts change seeds by deliverable guarantee
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "deliverable guarantee"
    And  the generated givn-ideation guidance contains "true end to end"
    And  the generated givn-ideation guidance contains "can ship alone"
    And  the generated givn-ideation guidance contains "may cover several use cases"
    And  the generated givn-ideation guidance contains "one use case may yield several seeds"
    And  the generated givn-ideation guidance contains "confirmed rule untrue"
    And  the generated givn-ideation guidance contains "dependencies and a suggested start order"
    And  the generated givn-ideation guidance contains "changes/<change-id>.md"

  Scenario: The ideation command guardrails carry the same conduct rules
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation command contains "deliverable guarantee"
    And  the generated givn-ideation command contains "never asked again"
  Scenario: The ideation guidance requires a canvas with distinguishable element kinds
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "one or more mermaid diagrams"
    And  the generated givn-ideation guidance contains "show every event, command, aggregate, and actor, with the kinds"
    And  the generated givn-ideation guidance contains "(per-node kind markers or a legend)"
    And  the generated givn-ideation guidance contains "the actor that issues it and the aggregate it changes"
    And  the generated givn-ideation guidance contains "A text chain without a canvas is an incomplete event storming"
    And  the generated givn-ideation guidance contains "the canvas must render"
    And  the generated givn-ideation guidance contains "drawn as a rendered mermaid canvas"
    And  the generated givn-ideation guidance contains "aggregate (the consistency boundary the command changes)"

  Scenario: The ideation guidance keeps aggregates to declared consistency boundaries
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "Aggregates are given, not minted"
    And  the generated givn-ideation guidance contains "a domain-noun root that receives commands and emits events under one"
    And  the generated givn-ideation guidance contains "confirmed before the canvas relies on it"
    And  the generated givn-ideation guidance contains "Reuse the aggregates the project already declares"
    And  the generated givn-ideation guidance contains "never an aggregate"
    And  the generated givn-ideation guidance contains "Name the kind"

  Scenario: The event storming review checks canvas coverage, legality, agreement, and rendering
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "canvas that shows every event, command, aggregate, and actor with its kinds"
    And  the generated givn-ideation guidance contains "every aggregate a declared consistency boundary or a"
    And  the generated givn-ideation guidance contains "drawn as an aggregate is a defect"
    And  the generated givn-ideation guidance contains "the lines and the picture naming the"
    And  the generated givn-ideation guidance contains "a canvas that renders"

  Scenario: The ideation command guardrails require the canvas and legal aggregates
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation command contains "rendered mermaid canvas naming every event"
    And  the generated givn-ideation command contains "every aggregate is a declared"
    And  the generated givn-ideation command contains "never a record or a specification"
  Scenario: The ideation guidance binds every confirmed term to the user's verified name
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "the user's verified name"

  Scenario: The ideation guidance asks the naming question as soon as a name fails
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "generic, overloaded"
    And  the generated givn-ideation guidance contains "cannot fill its genus slot"
    And  the generated givn-ideation guidance contains "ask the naming question immediately"
    And  the generated givn-ideation guidance contains "is collision-checked like any other"

  Scenario: The ideation guidance asks the term check before a phase closes
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "The user verifies every new core term before it is used"
    And  the generated givn-ideation guidance contains "Term check — <phase>"
    And  the generated givn-ideation guidance contains "Confirm all, or name the terms to revisit"
    And  the generated givn-ideation guidance contains "one-sentence description"
    And  the generated givn-ideation guidance contains "Confirm all makes them `confirmed`"
    And  the generated givn-ideation guidance contains "naming dialog, one at a time, before confirmation"
    And  the generated givn-ideation guidance contains "the other terms stand"
    And  the generated givn-ideation guidance contains "a replaced name becomes an anti-term"

  Scenario: The verified-name gate holds without the addon
    Given  a project root with givn/ initialised without addons
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation guidance contains "the user's verified name"
    And  the generated givn-ideation guidance contains "Term check — <phase>"

  Scenario: The ideation command guardrails carry the verified-name rule
    Given  a givn project with all agent skill targets enabled
    When  the user runs `givn skills sync`
    Then  the generated givn-ideation command contains "verifies every new core term"
    And  the generated givn-ideation command contains "phase term check"
    And  the generated givn-ideation command contains "naming dialog"
    And  the generated givn-ideation command contains "before the canvas, use cases, or seeds rely on it"
