# Arc42 impact: ise-ship-component-tier

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | 01 introduction-and-goals | No | The project goals and functional requirements are unchanged; the change tightens where an existing requirement is proven. |
| 2 | 02 architecture-constraints | Yes | TC-54 records the ship component tier: the design declaration the gate reads, the exact declared count, and the retained `givn/specs/**` subject. |
| 3 | 03 context-and-scope | No | No new external interface or consumer surface; the existing `givn check` commands gain a refusal, and the Ise declarations are unchanged. |
| 4 | 04 solution-strategy | Yes | The verification strategy row now names the two-tier evidence for an Ise ship and the build-and-test tool boundary the component run uses. |
| 5 | 05 building-block-view | Yes | `ise::ship` is a new block beside `ise::contract`; `proof` gains the component evidence record, and `cli::check`/`cli::archive`/`state` call the one required-evidence predicate. |
| 6 | 06 runtime-view | Yes | The change-verification flow gains the subject selection and the component run, and the review and archive freshness flow gains the component stale reasons. |
| 7 | 07 deployment-view | No | No deployment topology or runtime host change; the same binary and the same project runner execute. |
| 8 | 08 crosscutting-concepts | Yes | The runner-result section now states that a `verify` hook writes and validates a result, and that the receipt binds the executed subject. |
| 9 | 09 architecture-decisions | Yes | ADR-070 is amended: applicability of the change verification receipt now excludes an Ise ship with a declared component corpus. |
| 10 | 10 quality-requirements | No | No new quality scenario; the refusal is an existing correctness requirement of the change gate. |
| 11 | 11 risks-and-technical-debt | Yes | R-174 records the count-only component evidence limit: a runner can report the declared count without binding the scenario titles. |
| 12 | `docs/glossary.md` | Yes | **Component evidence** is added to the existing receipt vocabulary; no new domain entity is introduced. |

## ADR qualification

- **Architectural commitment**: the change verification receipt is a durable
  contract between Givn and every project runner; harnesses and external
  projects read the receipt and rely on the applicability rule.
- **Real alternatives**: keep the not-applicable branch for every
  `Behavior change: none` change versus require the retained corpus for an Ise
  ship; add a separate `givn ise verify-ship` command versus reuse the
  change-scope command and receipt.
- **Durable consequence**: reversing the choice changes which plans pass
  review and archive, and requires migrating receipts that lack the component
  field; a project runner that writes no result would fail a scope it passed
  before.
- **Existing decisions**: ADR-070 already owns the change verification receipt,
  its applicability rule, and its subject binding. Route: `AMEND_ADR`
  (ADR-070), not a new record; ADR-076 owns the spec-copy trial record and is
  not amended because the ship gate does not write a trial record.

## Status

STATUS: DONE
