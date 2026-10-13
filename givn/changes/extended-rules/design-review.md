# Design review: extended-rules

## Grilling record

| Branch | Outcome |
|---|---|
| Scope | The proposal's scope is unchanged and every finding lands in planning artifacts: one serving path, one extension map, no new command surface, no workflow configuration. Done is unambiguous: the delta features under `specs/` lock the observable behaviour. |
| Tech choices | The `extensions:` config map with validation when guidance is resolved is retained; the parse boundary (empty list valid, whitespace-only skipped with a warning, repeated instruction key last-wins, deterministic one-warning-per-entry order) is now recorded; no render cache and no new command surface. |
| Missing scenarios | Four scenarios added: empty extension list, whitespace-only rule, repeated instruction key (last-wins), and the direction-context integrity check. |
| Testability | Every new scenario is observable through a real `givn` CLI subprocess; the terminal evidence set is capturable with the established archive transcript pattern; `givn lint` reports advisories only (exit 2). |
| Risk | The most likely implementation risk remains a generated file keeping a rule copy under the asset rewrite; it is bounded by the no-duplication marker-list step plus the served-instruction content assertions. The direction-context defect class (id resolves syntactically but cannot be invoked) is now graded. |
| Use-case and Persona context | The two new use cases keep one actor goal each; the extension-recording contract now models project-configuration maintenance instead of a phantom command; the three confirmed Personas are reused and actors stay distinct from Personas. |
| Decision ledger | D1–D4 resolve with their seed references preserved (`Decides D1 (seed Q2)` … `D4 (seed Q7)`), and the design decisions are recorded; no unresolved, unrecorded item remains. |
| Visual design contract | `Interface: terminal`; the contract now names the transcript evidence path, the capture procedure, and the 12-transcript expected evidence set; the earlier "assertions alone are the evidence" wording is gone. |
| Deferrals | None. The terminal transcript obligation is a confirmed rule and is recorded as required evidence, not deferred. |
| Persona disposition | All three confirmed Personas are relevant and reused; the user-visible CLI surface is covered by `developer--primary-user` and `agent--technical-role`, with `givn-maintainer--owner` owning the single canonical asset per rule. |
| Interface classification | Terminal was tested against the opposite classification: nothing renders HTML, CSS, or JavaScript in a browser, so a browser driver would be wrong; the contract line parses as `terminal`. |
| Interaction Coverage | The matrix now carries all 13 inventory rows (the four affected use cases plus the support fragment) with exact `@e2e` titles and CLI subprocess mechanisms; the fragment relocation and the session-start placement are recorded; no inventory entry lacks a row. |
| ADR qualification | The verdict stands: NOT_QUALIFIED with the canonical artifact `docs/arc42/08-crosscutting-concepts.md` §§8.3a/8.7; the existing-ADR check (ADR-005, ADR-013, ADR-048) is unchanged. |
| Architecture documentation | The 05, 06, 08, 10, and 11 rows were re-verified against the working-tree diffs; chapter 12's `No` is reconciled with the ideation-handoff note so the file-change accounting matches the actual diff. |
| Ubiquitous language | "directing form", "resident obligations", "serve-time", "effective manifest", and "whole-facet replacement" are replaced with descriptive prose or existing glossary terms (`Extended rule`, `Facet`, `Cascade`, `Generated guidance`); no new durable term and no naming blocker. |

## Operator questions

The grilling surfaced two candidate questions; both are resolved without operator input because the answer is fixed by a confirmed rule or decision:

1. **Terminal transcripts.** Whether this terminal change must commit captured transcripts. Fixed by the confirmed evidence rule: the canonical specs instruction states "For a terminal interface, each `@e2e` scenario commits a captured transcript instead of screenshots, and the same baseline applies to the text the consumer reads"; the `visual-review-gate` feature proves a terminal change passes review only "with a transcript", and the design instruction fixes the capture path ("a transcript under `.../transcript.txt` for a terminal interface. Intermediate captures live under `reports/visual/` and are not review evidence"). No project constraint conflicts with the capture, so no deferral exists and no operator question is required. The design now declares the contract and the evidence set.
2. **Extension-recording mechanism.** Whether recording should go through a Givn command instead of editing configuration. Fixed by the design's Q4 decision (`Decides D2 (seed Q4)`: "the extension lives in a new config-map key `extensions:` in `givn/config.yaml`", validated when guidance is resolved) and by the user's minimal-surface direction (Q1 answered branch-only: no setting, and the design decision "no render cache and no new command surface"; the proposal's out-of-scope records no new configuration surface beyond the extension map). A recording command would reintroduce the command product surface the user rejected, so the E2E action is configuration maintenance (fixture) plus a real `givn instructions` subprocess. No operator question is required.

Neither candidate remains open; neither is a deferral.

## Findings and resolutions

1. **Interaction coverage matrix incomplete.** `design.md`'s Interaction Coverage Matrix now lists every inventory entry of the four affected use-case documents plus the `support` fragment document — 13 rows — each with its exact `@e2e` scenario title and a concrete CLI driving mechanism (real `givn` subprocess). Every title was cross-checked against its `.feature` file (delta and permanent); this change carries no separate inventory comments in `.feature` files, and the `usecase.md`/`fragment.md` inventories agree with the matrix.
2. **Extension recording contract vs implementation.** The contract no longer implies a recording command. `specs/extend-an-instruction/usecase.md` now models: the developer edits the project-owned `extensions:` map; Givn loads and validates it when it resolves guidance; rejected entries warn and are skipped; the request serves the built-in rules alone for them. The E2E scenario keeps its title and drives a configured extension (fixture) through a real `givn instructions` subprocess; the actor's action is maintaining project configuration.
3. **Terminal transcripts missing.** The Visual Design Contract now declares the terminal interface, the evidence path `givn/changes/extended-rules/evidence/visual/<scenario-slug>/transcript.txt`, the capture procedure (scenario title, exact `argv`, working-directory disposition, exit code, raw stdout/stderr; intermediate captures under `reports/visual/`), and the expected evidence set of 12 transcripts, one per delta `@e2e` scenario. The "assertions alone are the evidence" wording was removed; no deferral is recorded.
4. **`fragment.md` ownership update.** Verified: the change-tree `specs/fragments/fragment.md` no longer carries "request effective instructions by id" and keeps the resolution/machine-contract row with its retained `@e2e` scenario "An agent requests machine-readable artifact guidance"; the relocated action is `obtain-effective-guidance`'s inventory row (its wording restored to "request effective instructions by id (artifact, general policy, JSON, and change context are variants)"), whose matching `@e2e` scenario is "An agent receives the extended rules through the Givn CLI". The session-start row stays under `instruction-integrity` because the fragment owns the startup instruction's resolution and dialog obligation and its feature carries the matching `@e2e` scenario, while `inspect-change-state` owns the change-state reporting and carries no separate session-start row; the rationale is recorded in `design.md`'s matrix notes. `fragment.md` itself needed no row change.
5. **Validation boundaries underspecified.** `design.md` records deterministic behaviour: an empty list is valid and adds nothing; a non-string, empty, or whitespace-only rule warns and is skipped; a repeated instruction key keeps the last occurrence (recorded YAML behaviour — the pinned parser rejects duplicate keys inside a `Value` mapping, so the map is read as an entry list); warnings are one line per rejected entry in configuration document order. Scenarios added: empty extension list, whitespace-only rule, repeated instruction key.
6. **"Nothing changes" wording.** The minimal guarantees of `extend-an-instruction` and `obtain-effective-guidance` now separate "no project-state mutation" from "warn and serve the built-in rules alone"; the mirrored example text ("nothing changes") was corrected to "no project state changes".
7. **Design vocabulary vs durable glossary.** In `design.md`, "directing form", "resident obligations", "serve-time", "effective manifest", and "whole-facet replacement" were rewritten into descriptive prose or existing terms (`Extended rule`, `Facet`, `Cascade`, `Generated guidance`); the same wording was aligned in the touched `usecase.md`/`.feature` narrative and in the affected `docs/arc42/` sections. No new durable term was added and no blocker arose.
8. **Use-case boundary for configuration editing.** Applied with finding 2: the main flow names the developer maintaining project-owned configuration and Givn loading/serving it; malformed and unknown behaviour stays on the request path; the diagram reflects the same boundary.
9. **arc42 chapter 12 reconciliation.** `arc42.md` now states that the working-tree `docs/arc42/12-glossary.md` edits (the `Extended rule` row and the `Givn rule` disambiguation) are ideation-handoff material that predates this change, so chapter 12's `No` is correct and the assessment's file-change accounting matches the actual diff. The 05, 06, 08, 10, and 11 rows were re-verified against the working-tree diffs (all `Yes`, all matching); row 6's "serve-time" was rewritten to "request-time".
10. **Directions usable in context.** `design.md`'s integrity decision now distinguishes the two classes: a direction to an artifact instruction carries a resolvable change context; a direction to a policy or step instruction carries none; a syntactically resolvable id that cannot be invoked in its documented context is a defect. A new scenario, "Every generated direction is invokable in its documented context", asserts this through `givn skills sync` and per-context CLI resolution. `docs/arc42/08` §8.7 and `docs/arc42/10` Q-97 / §10.3 were aligned to the two-class rule.
11. **Duplicate asset row.** The second `assets/instructions/ideation.md` row was removed from `design.md`'s exact-file table; the ideation-specific detail was merged into the authoritative new-assets row.

## Hardening edits

| File | Change |
|---|---|
| `givn/changes/extended-rules/design.md` | Complete interaction matrix (13 rows) and ownership notes; terminal transcript contract with evidence set and capture procedure; validation boundaries and last-wins duplicate record; lenient overlay entry-list mechanism; two-class integrity decision; vocabulary rewrite; duplicate asset row merged; arc42 impact row alignment. |
| `givn/changes/extended-rules/arc42.md` | Chapter-12 reconciliation note; row 6 "request-time" wording. |
| `givn/changes/extended-rules/specs/extend-an-instruction/usecase.md` | Main flow, extensions, rules, examples, minimal guarantee, diagram: configuration-maintenance contract. |
| `givn/changes/extended-rules/specs/extend-an-instruction/instruction-extension.feature` | E2E and regular steps rewritten to the configured-extension mechanism; three boundary scenarios added. |
| `givn/changes/extended-rules/specs/obtain-effective-guidance/usecase.md` | Request-path rejection wording; no-mutation vs warn-and-serve distinction; relocated inventory row; facet wording. |
| `givn/changes/extended-rules/specs/fragments/instruction-integrity.feature` | New direction-context scenario. |
| `givn/changes/extended-rules/specs/initialize-project/usecase.md` | Direction/obligation vocabulary alignment. |
| `givn/changes/extended-rules/specs/initialize-project/initialize-a-project.feature` | Scenario title rewritten to the direction. |
| `givn/changes/extended-rules/specs/upgrade-a-project/usecase.md` | Direction vocabulary alignment. |
| `givn/changes/extended-rules/specs/upgrade-a-project/apply-a-managed-upgrade.feature` | Scenario title and step rewritten to the direction. |
| `docs/arc42/08-crosscutting-concepts.md` | Request-time wording; direction-only form wording; two-class id/context rule. |
| `docs/arc42/10-quality-requirements.md` | Q-97 and §10.3 two-class context rule. |

`proposal.md` is unchanged; `specs/fragments/fragment.md` was verified without a row change; no `tasks.md` was written. No `@e2e` tag was removed from any scenario.

## Verification

`givn lint --change extended-rules` — exit 2, summary `12 file(s) checked, 68 finding(s)`, all `WIP` findings and `SHAPE`/`SUBST` advisories; no parse error, duplicate title, or interaction mismatch. This is the expected pre-implementation baseline (the new scenarios are deliberately `@wip` until their steps exist).

## Post-review correction (2026-10-13)

The tasks pass flagged a step-file inconsistency in `design.md`: the
§Exact file changes cell for `tests/steps/instruction_integrity_steps.rs`
claimed "served-instruction steps used by the ideation feature", contradicting
§Step definitions (which maps `generate-ideation-guidance` to the shared
served-instruction step in `tests/steps/serve_extended_rules_steps.rs`). A
fresh-context re-verification confirmed the contradiction and found three
related gaps. All corrections are applied and re-checked against the delta
features:

- The `instruction_integrity_steps.rs` cell now points at the shared step in
  `serve_extended_rules_steps.rs`.
- The visual step files' cell now records that they add no content bindings
  and keep their existing eject and fixture steps (avoiding a second binding
  of `the served <id> instruction contains …`).
- The step-shape split is pinned: the shared content step takes a quoted
  string; the addon-condition steps keep their literal, unquoted form.
- The shared extension-serving assertions are single-homed in
  `serve_extended_rules_steps.rs`; `instruction_extension_steps.rs` holds
  only mutation, validation, and rejection steps.
- The exact-file-changes table gained the
  `tests/steps/initialize_a_project_e2e_steps.rs` row.

Nothing else in the review record changed; the verdict stands.

## Status

DESIGN-REVIEW: PASS
