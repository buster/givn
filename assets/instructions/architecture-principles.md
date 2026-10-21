# Instruction: architecture-principles

Answer questions at their last responsible moment. Design the consumer's
contract now; defer the component split.

**Opt-in.** This guidance applies only while `addons.architecture_principles`
is enabled in `givn/config.yaml`. The workflow pointers and gate checks are
hidden otherwise.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first question.

**Terminology.** This instruction uses *contract* for the callable surface
between a consumer and a component. The product's external interface (Web UI,
CLI, HTTP API, library) is a different thing. It uses *component* for the
replaceable unit of implementation behind one or more contracts, and
*implementation* for the code inside it. "Cell-based architecture" is an
informal name for the same pattern.

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

A component split is the canonical carried decision: defer it until the
component's responsibilities actually diverge. The contracts already exist, so
the split changes no consumer.

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

## Components: one unit, many contracts

A consumer is any caller with a distinct use case: another component, a UI, an
external system, or a test harness. A component is a replaceable unit of
implementation behind one or more contracts. Its internal methods are free. A
component may implement several contracts.

- **ISP splits contracts now.** One contract per consumer use case or consumer
  role. `BookLending` and `CdLending` are separate even when one component
  implements both. A consumer must not depend on a method it does not use.
- **SRP governs the component.** One named responsibility. A component with two
  current responsibilities is split now. A component with one responsibility may
  implement several contracts; when its responsibilities actually diverge, split
  it then. The contracts already exist, so the split changes no consumer.
- **Use only the contract.** Consumers import contracts, never implementations.
  The contract never imports the implementation. Only the composition root
  names implementations. The implementation's own tests may use it; consumer
  tests use the contract.
- **Co-locate, separate files.** Put the contract and the first implementation
  in the same directory, preferably in separate files. Follow the project's
  existing boundary convention; do not add a second one.
- **Enforce mechanically where possible.** Use the language's visibility rules
  (internal package, private module, non-exported implementation). If the
  language cannot enforce it, add a mechanical import check or a review check.
- **Not a bulkhead.** A component is a replacement boundary, not a
  failure-isolation boundary. Do not add retries, circuit breakers, process
  isolation, or messaging in its name.

A unit is a component when it has a named responsibility and a consumer outside
itself. Internal helpers inside a component are not separate components and
need no contract.

## Recording

- Record contracts in `design.md`: contract, consumer use case, consumers,
  implementing component today, split trigger.
- When the arc42 addon is enabled, a durable contract belongs in the
  building-block view; a change-local contract stays in `design.md`.
- When the arc42 addon is enabled, a durable contract that passes the ADR
  qualification gate is recorded as an ADR. Settling a contract early does not
  make it an ADR.
- Carrying a decision is not deferring a mandatory check; evidence gates stay
  mandatory.
