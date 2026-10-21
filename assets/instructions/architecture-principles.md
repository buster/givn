# Instruction: architecture-principles

Design the consumer's contract now. Defer the implementation split.

**Opt-in.** This guidance applies only while `addons.architecture_principles`
is enabled in `givn/config.yaml`. The workflow pointers and gate checks are
hidden otherwise.

For every user-facing question, follow the effective policy from
`givn instructions dialog`; load it once per context before the first question.

**Terminology.** This instruction uses *contract* for the callable surface
between a consumer and a cell. The product's external interface (Web UI, CLI,
HTTP API, library) is a different thing. It uses *cell* for the implementation
unit behind one or more contracts, and *component* for the published unit: a
cell plus its contracts.

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

A contract is not deferred. It is the answer to the current use case, not a
question about the future. The consumer's use case is known now. If the use
case changes later, the contract and the consumer change; that cost is the same
as without a contract. The contract protects against changing implementations,
not against changing requirements.

## Cells: one implementation, many contracts

A consumer is any caller with a distinct use case: another cell, a UI, an
external system, or a test harness. A cell is the implementation unit behind one
or more contracts. Its internal methods are free. A cell may implement several
contracts.

- **ISP splits contracts now.** One contract per consumer use case or consumer
  role. `BookLending` and `CdLending` are separate even when one cell implements
  both. A consumer must not depend on a method it does not use.
- **SRP governs the cell.** One named responsibility. A cell with two current
  responsibilities is split now. A cell with one responsibility may implement
  several contracts; when its responsibilities actually diverge, split it then.
  The contracts already exist, so the split changes no consumer.
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
- **Not a bulkhead.** A cell is a replacement boundary, not a failure-isolation
  boundary. Do not add retries, circuit breakers, process isolation, or
  messaging in its name.

A unit is a component when it has a named responsibility used from outside its
own implementation. Internal helpers inside a cell are not components and need
no contract.

## What is deferred

The last responsible moment applies to implementations, technology, and
business rules — never to the consumer contract.

- **Implementation split** — defer until the cell's responsibilities actually
  diverge. The contracts make the split an implementation change only.
- **Technology choice** — answer now when reversal needs a migration, a broken
  contract, or a rewrite; carry it when switching is cheap and usage supplies
  the answer.
- **Business rule** — answer now when it defines the product, a differentiator,
  or an enterprise rule; carry it when usage supplies the answer.

A carried decision names its default, the trigger that forces the answer, why
switching is cheap, and its owner. A carried decision is not an unowned
question. If a scenario in the current change needs the answer, it is not
carried; answer it now.

## Recording

- Record contracts in `design.md`: contract, consumer use case, consumers,
  implementing cell today, split trigger.
- When the arc42 addon is enabled, a durable contract belongs in the
  building-block view; a change-local contract stays in `design.md`.
- When the arc42 addon is enabled, a durable contract that passes the ADR
  qualification gate is recorded as an ADR. Settling a contract early does not
  make it an ADR.
- Carrying a decision is not deferring a mandatory check; evidence gates stay
  mandatory.
