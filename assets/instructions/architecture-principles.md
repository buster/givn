# Instruction: architecture-principles

Answer questions at their last responsible moment. Design the consumer's
contract now. Split a component when it owns more than one current
responsibility; do not carry the split.

**Enabled by default.** This guidance applies while
`addons.architecture_principles` is enabled in `givn/config.yaml`. A project
may disable it with `givn addons disable architecture_principles`. The workflow
pointers and gate checks are hidden otherwise.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first question.

**Terminology.** This instruction uses *Component Interface* for the narrow
callable surface a consumer uses, and *contract* for the named agreement that
surface exposes. The product's external interface (Web UI, CLI, HTTP API,
library) is a different thing. It uses *Component* for the replaceable unit of
implementation behind one contract, and *implementation* for the code inside
it. "Cell-based architecture" is an informal name for the same pattern.

## Decision timing

Answer a question at its last responsible moment: late enough that use has
supplied the evidence, early enough that the answer is still cheap or possible.
Not the earliest moment someone is willing to choose, and not never.

- **Foundational — answer now.** A business or enterprise rule, a
  differentiator, a feasibility constraint, a reversal that needs a migration or
  a broken contract, or anything a scenario in this change depends on. Do not
  carry it and let usage decide by accident.
- **Carried — answer on a trigger.** The change needs a default, but usage will
  supply the real answer. Name the default, the trigger that forces the answer,
  why switching is cheap, and the owner. A carried decision is not an unowned
  question. If a scenario depends on the answer, it is foundational.
- **Out of scope — do not decide it here.** The change does not need it. Record
  it as future scope and do not design for it.

A component split is not a carried decision. Split now when a unit owns more
than one current responsibility. The contracts already exist, so the split
changes no consumer.

Technology choices and business rules follow the same test: answer now when
reversal needs a migration, a broken contract, or a rewrite; carry them when
usage supplies the answer.

A contract is not a deferred decision. It is the current consumer use case.
Decide narrowly: keep the commitment local and named once. A later change of
direction must not become a rewrite, but do not build the alternative now.

## Contracts are current requirements

A contract is the consumer's use case, expressed as the narrowest callable
surface. It is not a prediction about the implementation and not an abstraction
over it. Design it from the consumer's side:

- Use the consumer's language, parameters, and return values.
- Include only what the current use case needs. Do not add methods for imagined
  future use cases.
- Keep implementation types, framework types, and implementation-shaped
  methods out.
- The implementation adapts to the contract. The contract does not mirror the
  implementation's internal methods.

The consumer's use case is known now. If the use case changes later, the
contract and the consumer change; that cost is the same as without a contract.
The contract protects against changing implementations, not against changing
requirements.

## Components, Component Interfaces, and adapters

A consumer is any caller with a distinct use case: another Component, a UI, an
external system, or a test harness. A **Component Interface** is the narrow
callable surface a consumer uses. A **Component** is a replaceable unit of
implementation behind one contract. An **adapter** is a Component that
translates between a Component Interface and one external mechanism:
terminal/CLI, HTTP service, database or file, CSV import, message broker, or
equivalent. An adapter contains no business rules and exposes no mechanism
types to the domain.

- **One contract, one Component.** One contract per consumer use case or
  consumer role; one component implements that contract. A unit that implements
  more than one contract, owns more than one capability, or implements more than
  one external adapter is split now.
- **A responsibility is a cohesive capability or mechanism.** "Maintain the
  register" is not one responsibility when the same unit also parses commands,
  talks HTTP, persists state, and imports CSV. Those are separate
  responsibilities and separate components.
- **ISP splits contracts now.** One contract per consumer use case or consumer
  role. `BookLending` and `CdLending` are separate. A consumer must not depend
  on a method it does not use.
- **SRP governs the Component.** One named responsibility per Component. A
  Component with two current responsibilities is split now.
- **Use only the contract.** Consumers import contracts, never implementations.
  The contract never imports the implementation. Only the composition root
  names implementations. The implementation's own tests may use it; consumer
  tests use the contract.
- **Co-locate, separate files.** Put the contract and the first implementation
  in the same directory, preferably in separate files. Follow the project's
  existing boundary convention; do not add a second one.
- **Adapters and scope.** A minimal change with one external dependency and no
  complex business rules may stay in one file; do not add Component Interfaces
  and adapters for it. KISS and YAGNI apply. A change with an external service
  or complex business rules uses Component Interfaces and adapters: the domain
  Component imports only Component Interfaces, each adapter implements one
  Component Interface, and the composition root wires them.
- **Enforce mechanically where possible.** Use the language's visibility rules
  (internal package, private module, non-exported implementation). If the
  language cannot enforce it, add a mechanical import check or a review check.
- **Not a bulkhead.** A component is a replacement boundary, not a
  failure-isolation boundary. Do not add retries, circuit breakers, process
  isolation, or messaging in its name.

A unit is a Component when it has a named responsibility and a consumer outside
itself. Internal helpers inside a Component are not separate Components and
need no contract.

## Recording

- Record contracts in `design.md`: contract, consumer use case, consumers,
  implementing component today, split trigger.
- When the arc42 addon is enabled, a durable contract belongs in the
  building-block view; a change-local contract stays in `design.md`.
- When the arc42 addon is enabled, a durable contract that passes the ADR
  qualification gate is recorded as an ADR. An adapter/Component Interface boundary, a
  component split, and the durable storage mechanism are ADR candidates.
  Settling a contract early does not make it an ADR.
- Carrying a decision is not deferring a mandatory check; evidence gates stay
  mandatory.
