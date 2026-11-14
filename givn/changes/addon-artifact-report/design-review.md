Review kind: self-review
Author: addon-artifact-report author
Reviewer context: same context as the author (self-review)

# Design Review: addon-artifact-report

## Phase 1 — Grilling

Scope: proposal, `specs/initialize-project/enable-optional-addons.feature`, `design.md`, `arc42.md`, and the touched chapters were read in full. Branches below record either a finding or "no issue found".

| # | Branch | Outcome |
|---|---|---|
| G1 | Scope | No issue found. The spec covers the proposal's table exactly: setting, materialized files, per-difference suggestion, managed-block case, exit status. Nothing extra: no `--fix`, no new command, no binary-version comparison. |
| G2 | Tech choices | No issue found with one correction recorded below: the proposal's `coverage` row assumed a scaffold. `coverage` writes no files (`src/cli/addons.rs:174`), so it declares `None`; the design carries the correction and no scenario depends on it. |
| **G3** | Decision ledger / suggestions | **Finding.** `enable_scaffold` (`src/cli/addons.rs:129`) runs `set_addon(true)` before scaffolding, and `enable`/`disable`/`skills sync` all refuse under `guard_addon_mutation` (`src/cli/addons.rs:318`). The design printed suggestions with no mention of the lock, so a suggestion run during an active change fails. Resolved by the operator: suggestions name the holding change. Hardened in `design.md` and the scenario "A locked change adds the lock to each suggestion". |
| **G4** | Shared paths | **Finding.** `docs/glossary.md` is scaffolded by both `arc42` (`src/cli/addons.rs:145`) and `domain_modeling` (`src/cli/addons.rs:155`). Declaring it under `domain_modeling` reports `partial` on every project that kept the glossary for `arc42` and dropped the domain-modeling skill. Resolved by the operator: a shared path is declared under exactly one addon, `docs/glossary.md` under `arc42`. Hardened in `design.md` (registry table, shared-path paragraph, registry uniqueness test) and the scenario "Disabling domain_modeling keeps the shared glossary from a partial report". |
| G5 | Missing scenarios | Covered and now complete: missing path, present-but-disabled, partial across targets, retired key, intact and damaged managed block, no-write, addon-free listing, shared path, lock suffix. No further observable behaviour is implied by the proposal. |
| G6 | Testability | No issue found. Every Then-step asserts concrete listing text or an exit code. `.fail_on_skipped()` on both `Cucumber` builders is the strict-mode mechanism; `@wip` tags mean undefined steps fail until bound. Single-scenario invocation recorded in the design. |
| G7 | E2E fidelity | No issue found. CLI system type; one `@e2e` scenario for the single consumer action (`givn addons list`); the remaining scenarios are input/state variants of that action, correctly not tagged `@e2e`. Primary assertion is the subprocess stdout. |
| G8 | Interaction Coverage | No issue found. The inventory row is in `givn/specs/initialize-project/usecase.md` §Interactions and matches the `@e2e` title "Consumer lists addons and sees each addon's materialized files reported". |
| G9 | Risk | The most likely failure: the registry table drifts from the scaffold routines even with the shared declaration, because four entries declare `None` by judgement. Mitigation: the registry uniqueness test for shared paths, plus scenario coverage of each declared path in both directions. |
| G10 | Use-case and Persona context | No issue found. `initialize-project` is the owning use case; the capability is `EXTEND enable-optional-addons`; no Persona is invented. Terminal output is a user-visible surface, so the proposal records "none" with its reason and the operator confirmation at proposal time. |
| G11 | Contracts and components | No issue found. One contract (`AddonMaterializationReport`), one component (`src/cli/addon_report.rs`), one external dependency (filesystem), single-file implementation accepted. `cli::addons` is the composition root. |
| G12 | Decision timing | No issue found. D1 is foundational and is answered in the design; D2 is decided; G3 and G4 are decided at this review. No scenario depends on a carried answer. |
| G13 | Visual design contract | No issue found. The change adds CLI text lines to an existing command and introduces no screen; there is no visual design contract to record and no `Interface:` line is required. The terminal-output classification is stated in the design. |
| G14 | Deferrals | No issue found. No mandatory check is deferred. The `@e2e` tag stays on its scenario. |
| G15 | Interface classification | No issue found. Not a browser surface: the interaction is a subprocess CLI invocation and the driver is a subprocess, not a browser. |
| G16 | ADR qualification | No issue found. `arc42.md` records one qualification. The cheaper home is named: the `cli::addons` row in chapter 05 plus the glossary terms. Rejected as sufficient because a chapter row cannot make an undeclared addon fail to build, and the boundary is shared by two consumers. NEW_ADR for ADR-076 stands. |
| G17 | Architecture documentation | Re-derived the 12-row table independently before opening `arc42.md`. Agreement on every row: 05 Yes, 06 Yes, 09 Yes, 11 Yes, 12 Yes, all others No. Chapters 05, 06, 09, and glossary checked against `design.md`; content matches. Chapter 11 consequence present in ADR-076 ("Bad, because ... four registry entries must declare `None`"). All 12 chapter files exist and carry content; no ASCII art found. |
| G18 | Ubiquitous language | Fixed during hardening. "Addon materialization" and "Materialization state" were recorded in `docs/glossary.md` before sign-off; "installation" is the recorded anti-term and the scenarios use `present`/`missing`/`partial`/`none`. |
| G19 | Structural model | `docs/domain-model.md` "Project configuration" now records that an addon's materialization is observed separately from its flag. |

## Phase 2 — Hardening applied

- `design.md`: registry table corrected (`coverage` -> `None`, `architecture_principles` -> managed block, glossary under `arc42`, `domain_modeling` declares `docs/domain-model.md` only); shared-path rule with a uniqueness test; lock-suffix rule; `none` added to the state table; external-boundary rows added; F1 and F2 added to the Decisions ledger.
- `specs/initialize-project/enable-optional-addons.feature`: two scenarios added (`@givn.added @wip`); lint reports only the expected `@wip` findings.
- `docs/glossary.md`, `docs/domain-model.md`, `docs/arc42/05,06,09`, `docs/arc42/adr/ADR-076`, `docs/arc42/adr/README.md`: written before this review and verified here.
- No `@e2e` tag removed. No `tasks.md` written.

## Remaining questions

None.

DESIGN-REVIEW: PASS
