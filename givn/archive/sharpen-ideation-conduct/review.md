# Review: sharpen-ideation-conduct

## Fabrication Audit (do this first)

**Step definition scan** — no step definition file was added or changed by
this change. The delta reuses the existing generic steps in
`tests/steps/prose_consolidation_steps.rs` (`generated givn-<skill>
guidance/command contains`), whose bodies carry real assertions and panic on a
missing phrase. No empty/no-op bodies exist in the reused steps.

**Commit cross-check** — every checked-off `[x]` task in tasks.md has a
matching commit that touches production source (embedded assets and durable
records):

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| Planning | `76192275` | Yes — proposal/spec/design/design-review/arc42/tasks |
| live-question standard | `255ec0c6` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl` |
| stops questions at decided directions | `496a02c9` | Yes — same template |
| frame confirmed once | `95cef964` | Yes — same template |
| Handoff cuts seeds by deliverable guarantee | `27e7eefc` | Yes — template, arc42 8.19, glossary |
| command guardrails | `cc767cf1` | Yes — `assets/commands/givn-ideation.md` |
| Verification | `a9c01f3f` | Yes — tasks evidence record |

**Promised-architecture cross-check** — every component design.md named exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Ideation skill template | Yes | `assets/skills/givn-ideation/SKILL.md.tmpl` |
| Ideation command guardrails | Yes | `assets/commands/givn-ideation.md` |
| arc42 8.19 seed lifecycle | Yes | `docs/arc42/08-crosscutting-concepts.md` |
| Glossary "Change seed" row | Yes | `docs/arc42/12-glossary.md` |

**Strict-mode proof** — present: `tests/features_runner.rs` carries
`.fail_on_skipped()` (lines 55 and 129). Every scenario's RED run was
targeted and exited non-zero (`1 scenario (1 failed)`, exit 101) before its
GREEN, recorded per task in tasks.md.

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
./run-tests.sh          -> Summary [34.609s] 539 tests run: 539 passed, 0 skipped
GIVN_FEATURES=givn/changes/sharpen-ideation-conduct/specs ./run-tests.sh
                        -> Summary [0.511s] 5 tests run: 5 passed, 0 skipped
```

**E2e isolation check** — the commands differ and the e2e scope is strictly
smaller (counts from the change's verification runs; the archive receipt will
carry the measured result):

| `verify.command` value | `verify.e2e_command` value | Identical? | Regular scope scenario count | E2E scope scenario count | E2E count strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 539 | 88 | Yes (451 skipped) |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | Fragment `support` (`givn/specs/fragments/fragment.md`) still owns `generate-ideation-guidance`; the capability's guarantees are unchanged, only its guidance text | Yes |
| Capability ownership and typed relationships preserved | No capability added, moved, or retired | Yes |
| Confirmed Persona-relevant outcomes preserved | The change reduces re-asking and gives the answering human full context; the asking agent gets a deliverable-based seed cut; the maintainer's durable records stay truthful | Yes |
| Actors remain distinct from Persona biographies | Delta scenarios name the consumer action; Personas only in this review/design context | Yes |
| Migration ledger and coverage evidence complete, if applicable | N/A — no migration | Yes |

PERSONAS: developer--primary-user, agent--technical-role, givn-maintainer--owner

## Deferred mandatory checks

None.

**Design.md conformance check** — implementation matches what design.md named:

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: design.md updated + design-review re-run? |
|---|---|---|---|---|
| Skill template rewrites | question section, Frame paragraph, Handoff bullet, topic-state entry, seed schema | Same files, same wording | Yes | N/A |
| Command guardrails | three exact lines | Same three lines | Yes | N/A |
| arc42 and glossary | 8.19 and "Change seed" row | Same | Yes | N/A |
| Step definitions | reuse existing content-assertion steps | No new steps | Yes | N/A |

Note: the assertion "never asked again" moved from delta scenario 1 to
scenario 2 during implementation so each scenario turns GREEN with its own
paragraph edit; this is spec-internal ordering, not a design deviation.
During review, the routing gate corrected the fragment convention: the delta
now lives under `specs/fragments/` and the proposal records
`NEW in fragments`; the delta suite was re-run green (5/5).

**Interaction coverage verification** — the fragment carries no user
interaction inventory (`fragment.md`: `Interactions: none`), so the matrix row
records the capability's one real action:

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism (from step defs) | Match? |
|---|---|---|---|---|---|
| generate-ideation-guidance: synchronize skills (fragment; no inventory) | Yes | A consumer synchronizes skills and every agent target receives the ideation facilitation skill | Real `givn skills sync` subprocess; content asserted on rendered files | `tests/steps/...` spawns the real binary and reads the rendered files | Yes |

## Visual Review

N/A — the change's design declares no visual interface (`## Visual Design
Contract` absent); the only surface is generated guidance text. The opposite
classification does not hold: nothing renders a page.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires nothing.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? / finding |
|---|---|---|---|---|---|---|
| 08.19 Change Seed Lifecycle | `docs/arc42/08-crosscutting-concepts.md` | Affected | Handoff/seed cut rules | Scenario 4 edit | Section now cuts seeds by deliverable guarantee, allows several use cases, forbids lying intermediate states, names the file by change identity | Match |
| 12 Glossary "Change seed" | `docs/arc42/12-glossary.md` | Affected | Seed cut wording | Scenario 4 edit | Row now per-change, deliverable cut, both directions | Match |

- [x] Every chapter marked affected was opened and checked for stale content.
- [x] Every relevant Arc42 fact is represented in design.md and mapped to tasks.
- [x] Implementation follows those facts; evidence names the files.
- [x] No architecture change was discovered that is not recorded.
- [x] No mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

No instrumented pass is authoritative yet (not archived). The merged report
was produced during verification:
`coverage/cobertura-coverage.xml` — lines 7655/9684 (79.05%). The change
modifies no executable Rust production code; its production surface (embedded
guidance assets and durable docs) is fully exercised by the delta suite,
which is green (5/5).

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| No changed executable line exists in this change | 0 | N/A — the change adds no uncovered production line | None required; the merged report's uncovered regions are the pre-existing project baseline, untouched by this change |

## Scenario execution evidence

Unproven candidate — no archive receipt yet. Per-scope counts from the
change's verification runs are recorded in tasks.md (5/5 delta, 539/539
regular, 88/88 e2e).

**Receipt status:** not archived yet

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| The ideation guidance carries the seed schema and its status values | Handoff cuts change seeds by deliverable guarantee | `variant` |
| The ideation guidance carries the authoring requirements and the handoff mapping | The ideation guidance stops questions at decided directions | `variant` |
| The ideation guidance carries the open-question qualifier rule | The ideation guidance carries the live-question standard | `variant` |

The remaining 48 advisory subset findings are the same sync-and-contains
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
- [x] Implementation matches design.md as reviewed.
- [x] Interaction coverage verified.
- [x] Design.md's Internal Primitives section states none (correct for a guidance-only change).
- [x] arc42 slug mapping verified: 8.19 and the glossary slug map to real files.
- [x] No finding is excused with a classification outside the three buckets.

README-IMPACT: none

**REVIEW: PASS**
