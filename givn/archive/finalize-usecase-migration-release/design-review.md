# Design Review: finalize-usecase-migration-release

## Scope

The change contains only release-closure work: migration gate scenarios,
complete active use-case ownership, release bundle finalization, and durable
evidence documentation. It does not alter product behavior, historical
archives, or Kiekste Handoff state.

## Grilling Outcomes

| Branch | Finding | Resolution |
|---|---|---|
| Scope | The proposal requires executable regression evidence and a concrete 0.6.0 bundle. | Seven migration scenarios cover pass, missing behavior, lost E2E, lower interaction, lower line, lower branch, and real CLI smoke behavior. |
| Technical choice | The existing Rust/Cucumber runner and CLI subprocess are the project-standard boundary. | Retained; no new framework or fixture protocol is introduced. |
| Missing scenarios | A migration can fail independently on behavior identity, E2E state, interactions, line ratio, or branch ratio. | Each failure dimension has a separate scenario. The passing receipt and E2E action are also covered. |
| Testability | Every scenario can fail from fixture evidence alone before production changes. | Temporary initialized projects write deterministic JSON evidence and invoke the actual migration command. |
| Risk | The most likely failure is a fixture that exercises only the in-process comparison and not the CLI receipt path. | All scenarios invoke `givn check migration`; the tagged smoke scenario uses a separate real-subprocess step definition. |
| Use-case and Persona context | The change extends `verify-change`; no Persona or ideation source applies. | Capability ownership is corrected in both affected use-case documents. Kiekste Handoff remains untouched. |
| ADR qualification | The change does not create a durable architectural boundary. | Routed to feature, process, Arc42 quality, and release assets. No ADR is created. |
| Arc42 | Quality, cross-cutting evidence, risk, and glossary content are affected. | All twelve chapters were assessed; chapters 8, 10, 11, and 12 were updated. |

## Evidence Boundary

The migration checker compares behavior identities using behavior hash plus
scenario title, preserves E2E state and interaction counts, and compares source
coverage ratios. The scenarios assert the observable command exit, output, and
receipt content rather than private implementation calls.

## Review Checklist

- [x] Strict runner remains `.fail_on_skipped()`.
- [x] The E2E action uses the real `givn check migration` subprocess.
- [x] No generated guidance is hand-edited.
- [x] No historical archive is rewritten.
- [x] No Persona is invented or promoted.
- [x] The target release is explicitly `0.6.0`.

DESIGN-REVIEW: PASS
