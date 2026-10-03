# Review: harden-event-storming

## Fabrication Audit (do this first)

**@e2e tag integrity** — the delta adds no `@e2e` scenario and removes none;
the capability's existing `@e2e` scenario is untouched, so no planned e2e
coverage was dropped.

**Step definition scan** — no step definition file was added or changed
(`git diff --name-only 52061dbf..HEAD -- tests/` is empty). The four delta
scenarios reuse the existing generic steps in
`tests/steps/prose_consolidation_steps.rs` (`generated givn-<skill>
guidance/command contains`), whose bodies assert with `contains` and panic on
a missing phrase. **0 empty step bodies found.**

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (embedded assets and durable
records):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Planning | `cde2c5d8` | Yes — proposal, spec delta, design, design-review, arc42, tasks |
| canvas with distinguishable kinds | `e3c103c3` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl` |
| aggregate legality | `3a2ea1fe` | Yes — template, `docs/workflow.md`, `docs/arc42/12-glossary.md` |
| review lens | `7fb1d8c6` | Yes — template |
| command guardrails | `d7852123` | Yes — `assets/commands/givn-ideation.md` |
| Evidence record | `4e1b9d40` | Yes — tasks.md evidence and commit hashes |

**Promised-architecture cross-check** — every component design.md named
exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Ideation skill template | Yes | `assets/skills/givn-ideation/SKILL.md.tmpl` |
| Ideation command guardrails | Yes | `assets/commands/givn-ideation.md` |
| Workflow phase table | Yes | `docs/workflow.md` |
| Glossary "Event Storming" row | Yes | `docs/arc42/12-glossary.md` |

**Strict-mode proof** — present: `tests/features_runner.rs` carries
`.fail_on_skipped()`. Every scenario's RED run was targeted and exited
non-zero (`1 scenario (1 failed)`, exit 101) before its GREEN, recorded per
task in tasks.md; scenario 3's RED was re-proven for the corrected assertion
set.

**Downgraded e2e scenario scan** — the delta adds no `@e2e` scenario; the
capability's existing `@e2e` scenario is untouched and drives the real
interface:

| @e2e Scenario | Real-interface assertion present? | Driven via real browser (if UI)? | Downgraded? |
|---|---|---|---|
| A consumer synchronizes skills and every agent target receives the ideation facilitation skill | Yes — a real `givn skills sync` subprocess runs and the rendered `.opencode`/`.agents`/`.claude` files are asserted | N/A — CLI, no browser | No |

**E2e scope check** — one `@e2e` per distinct action:

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| generate-ideation-guidance | 1 | Yes — synchronize skills | No |

**Duplicate-implementation check** — `verify.e2e_command` invokes
`./run-tests.sh --e2e` (tag-scoped to `@e2e`). No parallel e2e implementation
for this capability exists in the tree, tracked or untracked; the delta adds
none.

**Local environment check** — no stack and no digital twins; the local run is
the runner:

```
./run-tests.sh          -> Summary [36.960s] 569 tests run: 569 passed, 0 skipped
GIVN_FEATURES=givn/changes/harden-event-storming/specs ./run-tests.sh
                        -> Summary [0.415s] 4 tests run: 4 passed, 0 skipped
```

**E2e isolation check** — the commands differ and the e2e scope is strictly
smaller (counts from the change's verification runs; the archive receipt will
carry the measured result):

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2E scope scenario count | E2E count strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 569 | 91 | Yes (478 skipped) |

**Design conformance diff** — the implementation matches the commands, file
layout, and mechanism design.md named; one wording deviation is recorded and
resolved:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: design.md updated + design-review re-run? |
|---|---|---|---|---|
| Canvas bullets and topic-state line | the design's six-bullet block and rendered-canvas line | Same wording in `assets/skills/givn-ideation/SKILL.md.tmpl` | Yes | N/A |
| Aggregate bullet conditional | `{{#if addons.arc42}}` glossary reference with an else branch | Same; both branches verified in scratch projects | Yes | N/A |
| Review lens | the design's lens | Same, except the noun phrase described below | No | `design.md` updated; `design-review.md` carries the finding; RED/GREEN re-proven |
| Command guardrail | one line, exact wording | Same | Yes | N/A |
| Durable rows | exact texts, review-verified | Same | Yes | N/A |
| Step definitions | reuse existing content-assertion steps | No new steps | Yes | N/A |

Deviation detail: the lens first carried the proposal's "declared glossary
aggregate". The full suite's `match-guidance-to-enabled-addons` scenario
(`@e2e`, mandatory) proved that a project with the addons disabled may not
mention "glossary" case-insensitively. The lens now reads "a declared
consistency boundary" — the term the aggregate bullet defines; the delta
assertion and `design.md` follow, `design-review.md` records the finding, and
the no-addon render of the skill and command was verified to contain none of
`arc42`, `glossary`, `ubiquitous`, `domain-modeling`.

**Interaction coverage verification** — the fragment carries no user
interaction inventory (`fragment.md`: `Interactions: none`), so the matrix row
records the capability's one real action:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| generate-ideation-guidance: synchronize skills (fragment; no inventory) | Yes | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | Real `givn skills sync` subprocess; content asserted on rendered files | The step spawns `CARGO_BIN_EXE_givn` and reads the rendered files | Yes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | Fragment `support` (`givn/specs/fragments/fragment.md`) still owns `generate-ideation-guidance`; its guarantees are unchanged, only its guidance text is amended | Yes |
| Capability ownership and typed relationships preserved | No capability added, moved, or retired | Yes |
| Confirmed Persona-relevant outcomes preserved | The reading human gets a reviewable canvas with legal elements; the running agent gets an explicit legality rule; the maintainer's durable records stay truthful | Yes |
| Actors remain distinct from Persona biographies | Delta scenarios name the consumer action; Personas only in design/review context | Yes |
| Migration ledger and coverage evidence complete, if applicable | N/A — no migration | Yes |

PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Deferred mandatory checks

None.

## Visual Review

N/A — the change's design declares no visual interface (`## Visual Design
Contract` absent); the only surface is generated guidance text. The opposite
classification does not hold: nothing renders a page, no HTML, CSS, or JS is
produced for a browser.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires nothing.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| 12 Glossary "Event Storming" | `docs/arc42/12-glossary.md` | Affected | Durable-row replacement named | Scenario 2 edit | Row now defines the phase as the rendered canvas whose aggregates are declared consistency boundaries | Match |
| 08 crosscutting-concepts (no section owns phase conduct) | `docs/arc42/08-crosscutting-concepts.md` | Not affected | Guidance-only change; no 8.x section restates the rules | No 8.x task | The rules live only in the generated guidance; no 8.x text contradicts them | Match |
| 11 risks-and-technical-debt R-46 | `docs/arc42/11-risks-and-technical-debt.md` | Not affected | No new check; the review lens strengthened | No task | R-46's statement (convention reviewed at the phase boundary) still holds | Match |

- [x] Every chapter marked affected was opened and checked for stale content.
- [x] Every relevant Arc42 fact is represented in design.md and mapped to tasks.
- [x] Implementation follows those facts; evidence names the files.
- [x] No architecture change was discovered that is not recorded.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt exists yet for this change, so there is no measured
coverage evidence to classify against; the archive will measure coverage
against the committed candidate and publish it in the receipt. Static
inspection: the change adds no executable Rust production line — it edits
embedded text assets and durable documentation — and the four delta scenarios
fully exercise the changed surface (rendered guidance text), green at 4/4.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| No changed executable line exists in this change | 0 | N/A — the change adds no uncovered production line | None required; the merged report's uncovered regions are the pre-existing project baseline, untouched by this change |

## Scenario execution evidence

Unproven candidate — no archive receipt yet. Per-scope counts from the
change's verification runs are recorded in tasks.md (4/4 delta, 569/569
regular, 91/91 e2e).

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes | The ideation guidance keeps aggregates to declared consistency boundaries | `variant` |
| The ideation guidance carries the seed schema and its status values | The ideation guidance requires a canvas with distinguishable element kinds | `variant` |
| The ideation guidance carries the authoring requirements and the handoff mapping | The event storming review checks canvas coverage, legality, agreement, and rendering | `variant` |
| The ideation guidance stops questions at decided directions | The event storming review checks canvas coverage, legality, agreement, and rendering | `variant` |
| Handoff cuts change seeds by deliverable guarantee | The ideation guidance requires a canvas with distinguishable element kinds | `variant` |

The remaining 45 advisory subset findings are the same sync-and-contains
pattern across the capability's permanent and delta scenarios; each is a
`variant` (distinct asserted content, same capability action). No
`duplicate`, no `boundary` finding.

## Split-or-keep

None — `givn lint` reports no long-scenario finding for this delta.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies, all commits verified, all promised components exist, no downgraded e2e scenarios, no over-produced e2e scope).
- [x] Scenario execution evidence recorded per scope; review does not re-run the suite.
- [x] Every coverage gap is classified and resolved (or justified) from the most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts cleanly; no digital twins.
- [x] The exact file(s) `verify.e2e_command` invokes were identified; the tree was searched for a parallel e2e implementation.
- [x] `verify.e2e_command` is not identical to `verify.command`; counts prove isolation.
- [x] Implementation matches design.md as reviewed (one wording deviation recorded, design.md updated, design-review.md finding added, RED/GREEN re-proven).
- [x] Interaction coverage verified.
- [x] Design.md's Internal Primitives section states none (correct for a guidance-only change).
- [x] arc42 slug mapping verified: the glossary slug maps to a real file.
- [x] No finding is excused with a classification outside the three buckets.

README-IMPACT: none

**REVIEW: PASS**
