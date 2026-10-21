# Architecture Principles addon

Optional, default-off guidance for designing replaceable components: **design
the consumer's contract now, defer the implementation split.**

Canonical rule: `givn instructions architecture-principles`.

## What it does

- A **contract** is the consumer's use case, expressed as the narrowest callable
  surface. It uses the consumer's language, parameters, and return values and
  contains only what the current use case needs.
- A **cell** is the implementation behind one or more contracts. Its internal
  methods are free.
- **Interface Segregation splits contracts now.** `BookLending` and `CdLending`
  are separate contracts even when one `LendingDesk` implements both. A consumer
  never depends on a method it does not use.
- **Single Responsibility governs the cell.** If book and CD policies later
  diverge, split `LendingDesk` into separate cells. The contracts already exist,
  so no consumer changes.
- Consumers use only the contract. The contract never imports the
  implementation; only the composition root names implementations.
- A contract is not deferred. The last responsible moment applies to the
  implementation split, technology choices, and business rules.
- YAGNI and KISS apply to the contract's content: no future-use-case methods,
  no generic `lendMedia`, no second implementation now.

## Example

```text
BookLending.lendBook(bookId)   implemented by LendingDesk
CdLending.lendCd(cdId)         implemented by LendingDesk
```

Later, when the policies diverge:

```text
BookLendingDesk implements BookLending
CdLendingDesk   implements CdLending
```

`BookDesk` and `CdDesk` do not change.

A cell is a replacement boundary, not a failure-isolation boundary. Do not add
retries, circuit breakers, process isolation, or messaging in its name.

## Where it applies

| Phase | Effect when enabled |
|---|---|
| Proposal | Classifies open questions; a contract is not an open question — it follows the consumer use case. |
| Design | Adds the `### Contracts and Cells` table under Architecture Impact and the carried-decision fields. |
| Design review | Adds the “Contracts and cells” and “Decision timing” branches. |
| Tasks / implementation | A scenario touching a cell names the contract it uses; the implementer never imports the implementing cell. |
| Review | Checks that every consumer uses only its declared contract. |
| Dialog | Points architecture questions at the canonical instruction. |

The addon adds no artifact, generated skill, or archive gate. A durable contract
is recorded in the building-block view when arc42 is enabled; ADR qualification
is unchanged.

## Enable and disable

```sh
givn addons enable architecture_principles
givn addons disable architecture_principles
```

`givn init` lists it under `addons available (default off)`. When disabled, the
workflow pointers and gate checks above are omitted. The instruction text
remains retrievable by explicit id, matching the existing policy addons.
