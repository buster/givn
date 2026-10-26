# Architecture Principles addon

**TL;DR:** Answer questions at their last responsible moment — late enough for
evidence, early enough to stay cheap. Treat each consumer use case as a contract;
one component implements that contract. Split a component when it owns more than
one current responsibility. Use ports and adapters for external boundaries in a
complex change; keep a minimal change simple.

Canonical rule: `givn instructions architecture-principles`.

## Status

Feature-gated opt-in experiment. It is default-off, adds no archive gate and no
generated skill, and its rules are provisional until the expected outcome below
is observed in real changes. Disabling it removes every workflow pointer and
gate check without deleting project content.

## What it does

- A **contract** is the consumer's use case, expressed as the narrowest callable
  surface. It uses the consumer's language, parameters, and return values and
  contains only what the current use case needs.
- A **component** is the replaceable unit of implementation behind one contract.
  Its internal methods are free.
- A **port** is the narrow callable interface one consumer uses. An **adapter**
  translates between a port and one external mechanism: CLI, HTTP service,
  database/file, CSV import, or equivalent. An adapter contains no business
  rules.
- **Interface Segregation splits contracts now.** `BookLending` and `CdLending`
  are separate contracts. A consumer never depends on a method it does not use.
- **Single Responsibility governs the component.** One component implements one
  contract and one named responsibility. A unit that owns more than one
  contract, capability, or external adapter is split now.
- Consumers use only the contract. The contract never imports the
  implementation; only the composition root names implementations.
- A contract is not deferred. It is the current consumer use case.
- YAGNI and KISS apply to the contract's content: no future-use-case methods,
  no generic `lendMedia`, no second implementation now.
- A minimal change with one external dependency and no complex business rules
  may stay in one file; do not add ports and adapters for it. A change with an
  external service or complex rules uses ports and adapters.

## Decision timing

The rule is not "decide everything now." It is:

- **Foundational — answer now.** Language/runtime, internationalization,
  multi-tenancy, legal constraints, the differentiators, or a decision whose
  reversal needs a migration or a broken contract. Example: how the library
  handles overdue notices is a business rule, not a screen detail.
- **Carried — answer on a trigger.** The change needs a default, but usage will
  supply the real answer. Example: the overdue-notice layout; choose a simple
  layout now, change it when usability evidence arrives.
- **Out of scope — do not decide it here.** The feature was not requested.
  Example: a deputy rule or digital-media support; record it as future scope,
  do not design for it.

A contract is not carried; it is the current use case. A component split is not
carried either: when a unit owns more than one current responsibility, split it
now.

## Example

```text
BookLending.lendBook(bookId)   implemented by BookLendingDesk
CdLending.lendCd(cdId)         implemented by CdLendingDesk
```

The two contracts and components are separate from the start. `BookDesk` and
`CdDesk` depend only on their contract.

A component is a replacement boundary, not a failure-isolation boundary. Do not
add retries, circuit breakers, process isolation, or messaging in its name.

## Expected outcome

The experiment succeeds if it makes replacing or splitting a component cheaper
without making the code harder to read. Concretely:

- **Cheaper replacement.** Replacing or splitting a component changes the
  implementation and the composition root; consumer code and consumer tests do
  not change.
- **Consumer-shaped contracts.** Contracts are smaller than the implementation
  surface, use the consumer's language, and contain no implementation or
  framework types. A consumer can work against the contract without reading the
  implementation.
- **Right granularity.** Contracts correspond to consumer use cases, not to
  modules or files. One contract has one implementing component and one named
  responsibility; a unit with more is split. Internal helpers stay internal.
  A minimal change with one external dependency and simple rules may stay in
  one file.
- **Correct timing.** Foundational business and technology questions are
  answered now. Carried decisions name a default, trigger, switch cost, and
  owner. Component splits are not carried; split when the second current
  responsibility appears.
- **No readability tax.** The contract table and review branches are used only
  where a component exists. Contracts reduce the surface a consumer must
  understand; they do not add a layer to helpers.
- **Real use.** Design tables are filled, review branches produce findings, and
  the findings are resolved. At least one change shows a replacement or split
  that stayed inside the implementation.

## Signals that the rules need revision

- **Contract sprawl.** A contract per module or helper, many one-method
  contracts with one consumer and no replacement trigger, or readers jumping
  through files for no benefit.
- **Implementation-shaped contracts.** Contracts mirror internal methods, expose
  framework, database, AST, or HTTP types, or change whenever the implementation
  changes. Consumers still need implementation knowledge.
- **Speculative design.** Methods for future use cases, a generic `lendMedia`,
  or a second implementation built now. YAGNI and KISS are being violated.
- **Decorative contracts.** The table is filled but consumers still import
  implementations, the composition-root exception is used everywhere, or tests
  bypass the contract.
- **SRP erosion.** One unit owns several capabilities or external adapters
  without a split, or one component implements more than one contract.
- **Deferred foundation.** A business or enterprise rule is carried without a
  trigger or owner, or a scenario depends on a carried answer.
- **No cheaper change.** When a component was actually replaced or split,
  consumers still had to change. The core promise failed.
- **Process overhead.** Design reviews take longer without finding real boundary
  problems, the table becomes boilerplate, or teams disable the addon.
- **Duplicate record.** Contracts duplicate arc42 or ADR content and drift.
- **Scope creep.** The component boundary is used to justify retries, circuit
  breakers, process isolation, or to force premature decisions on layout and
  product scope.
- **Readability regression.** More indirection, longer design documents, or a
  larger surface for the consumer to understand.

Any of these is evidence to narrow, revise, or retire the rule — not a reason
to add more abstraction.

## Where it applies

| Phase | Effect when enabled |
|---|---|
| Proposal | Classifies open questions; a contract is not an open question — it follows the consumer use case. |
| Design | Adds the `### Contracts and Components` table under Architecture Impact and the carried-decision fields. |
| Design review | Adds the “Contracts and components” and “Decision timing” branches. |
| Tasks / implementation | A scenario touching a component names the contract it uses; the implementer never imports the component's implementation. |
| Review | Checks that every consumer uses only its declared contract. |
| Dialog | Points architecture questions at the canonical instruction. |

The addon adds no artifact, generated skill, or archive gate. A durable contract
is recorded in the building-block view when arc42 is enabled. Port/adapter
boundaries, a component split, and the durable storage mechanism are ADR
candidates.

## Enable and disable

```sh
givn addons enable architecture_principles
givn addons disable architecture_principles
```

`givn init` lists it under `addons available (default off)`. When disabled, the
workflow pointers and gate checks above are omitted. The instruction text
remains retrievable by explicit id, matching the existing policy addons.
