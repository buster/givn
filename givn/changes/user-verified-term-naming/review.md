# Review: user-verified-term-naming

## Fabrication Audit (do this first)

**@e2e tag integrity.** `grep -n "@e2e"
givn/changes/user-verified-term-naming/specs/fragments/*.feature` reports no
matches: both delta features carry only `@givn.delta` and `@givn.added` tags
(the two `@wip` tags were removed when their steps landed). No `@e2e` tag was
removed, added, or edited. Clean.

**Step definition scan.** All eight new steps in
`tests/steps/fix_domain_modeling_skill_trigger_steps.rs` carry `assert!`
bodies with failure messages; 0 empty step bodies; no `unimplemented!()` or
`todo!()` in the file. Two hits of the emptiness grep were excluded as
non-steps: `tests/steps/archive_proof_support.rs:158` (a `printf` string
inside fixture setup) and `tests/steps/llm_attribution_e2e_steps.rs:364`
(`fn make_executable(_path: &std::path::Path) {}` — a pre-existing no-op
helper, not a step definition).

**Commit cross-check.**

| Scenario | Commit hash | Touches production code? |
|---|---|---|
| 1 — verified name in the `confirmed` clause | `3b4bc796` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl` |
| 2 — asks the naming question as soon as a name fails | `74d11d21` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl` |
| 3 — asks the term check before a phase closes | `c8300a22` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl` |
| 4 — verified-name gate holds without the addon | (none) | Placement pin — the production text is delivered by `3b4bc796`, `74d11d21`, `c8300a22`; the checked scenario passes against that production source. Not fabricated: no new production change exists to commit, and the rationale is recorded in `tasks.md` |
| 5 — command guardrails carry the verified-name rule | `d066cea9` | Yes — `assets/commands/givn-ideation.md` |
| 6 — challenge bullet hands the name choice to the user | `57375275` | Yes — `assets/skills/givn-domain-modeling/SKILL.md.tmpl`, `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` |
| 7 — challenge bullet shows a naming question | `486ede9b` | Yes — `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` (the delta `@wip` removal rode this commit) |

**Promised-architecture cross-check.**

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Domain-modeling skill template (challenge bullet) | Yes | `assets/skills/givn-domain-modeling/SKILL.md.tmpl` |
| Ideation skill template (term record + term check) | Yes | `assets/skills/givn-ideation/SKILL.md.tmpl` |
| Ideation command guardrails | Yes | `assets/commands/givn-ideation.md` |
| Naming-dialog step definitions | Yes | `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` |
| Durable records | Yes | `docs/workflow.md`, `docs/arc42/08-crosscutting-concepts.md`, `docs/arc42/12-glossary.md` |

**Strict-mode proof.** The runner is strict:
`tests/features_runner.rs` calls `.fail_on_skipped()` (line 55) and exits
non-zero when `failed > 0` or `skipped > 0` (line 64). Before implementation,
the change delta ran `7 tests run: 0 passed, 7 failed, 0 skipped` (exit
non-zero) — the two `@wip` scenarios failed with undefined steps under the
strict runner. Recorded in `tasks.md` Setup.

**Downgraded e2e scenario scan.** No delta `@e2e` scenarios — nothing to scan.
The change adds no consumer action, and the pre-existing `@e2e` scenarios are
unmodified.

**E2e scope check.**

| Capability | Delta @e2e scenario count | Distinct new actions? | Over-produced? |
|---|---|---|---|
| generate-ideation-guidance | 0 | None — no new consumer action | No |
| guide-domain-modeling | 0 | None — no new consumer action | No |

**Duplicate-implementation check.** `Cargo.toml` declares one test harness for
the features (`features_runner`, `harness = false`) plus `ready_fixture` (the
fixture builder, not an alternate e2e implementation). `run-tests.sh` and
`measure-coverage.sh` drive the same runner with the same tag selection. No
second or weaker e2e implementation exists.

**Local environment check.**

```
GIVN_FEATURES=givn/changes/user-verified-term-naming/specs ./run-tests.sh  →  7 tests run: 7 passed, 0 skipped
./run-tests.sh                                                        →  591 tests run: 591 passed, 0 skipped
./run-tests.sh --e2e                                                  →  90 tests run: 90 passed, 501 skipped
givn lint --change user-verified-term-naming                           →  2 file(s) checked — clean
```

**E2e isolation check.**

| verify.command | verify.e2e_command | Identical? | Regular scope count | E2E scope count | E2E strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 591 | 90 | Yes — 90 < 591 |

**Design conformance diff.**

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| D1 — two-tier verification | Immediate naming dialog for failed names; one combined term check at each phase's closing checkpoint | The ideation template carries the immediate-question bullet and the checkpoint question with confirm/revisit outcomes | Yes |
| D2 — single-source the dialog shape | The challenge bullet in `givn-domain-modeling` owns the dialog; ideation binds it | The 9-line challenge bullet carries the concept sentence, candidates, collisions, and free-form; ideation text names the dialog without restating its shape | Yes |
| Unconditional placement | The `confirmed` clause and the term-check section sit outside the addon conditional; only the term-test pointer stays wrapped | Verified against the no-addon fixture (scenario 4) | Yes |
| Durable records | `docs/workflow.md`, `docs/arc42/08-crosscutting-concepts.md` §8.20, `docs/arc42/12-glossary.md` amended, review-verified | Present in tree; committed with the plan (`2bc72212`) | Yes |
| Step definitions | One file per capability; dialog steps in `fix_domain_modeling_skill_trigger_steps.rs` | 8 steps appended there; the ideation delta reuses the shared steps in `prose_consolidation_steps.rs` | Yes |
| Runner | `verify.command ./run-tests.sh`; strict mode; single-scenario command | Used throughout; the design's single-scenario example shows the libtest form while `run-tests.sh` defaults to nextest — same `features_runner` target and selection, documented in `tasks.md`; no behavioral deviation | Yes |

**Interaction coverage verification.** The delta features declare no
`# User Interaction Inventory` block because the change adds no consumer
action; the design's matrix records the two capabilities as existing coverage.

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| No new action — synchronizing skills already inventoried | Yes — "Existing coverage (no new action)" | Yes — "A consumer synchronizes skills and every agent target receives the ideation facilitation skill" | Real `givn skills sync` subprocess | The existing `@e2e` scenario drives the real binary (compile-time `CARGO_BIN_EXE_givn`) and asserts the rendered guidance | Yes |
| No new action — synchronizing skills already inventoried | Yes — "Existing coverage (no new action)" | Yes — "An agent synchronizes skills and the domain skill is routed" | Real `givn skills sync` subprocess | Same real-binary driver, asserting the routed domain skill | Yes |

## Use-Case And Persona Conformance

Owning contract: fragment `support`; capabilities `generate-ideation-guidance`
and `guide-domain-modeling`; no change seed exists (direct operator feedback).

| Contract element | Where realized | Evidence |
|---|---|---|
| Rule — the user verifies every new core term before it is used | Ideation term-check section, challenge bullet, command guardrail | Scenarios 1–7 |
| Capability — generate-ideation-guidance owns the rendered ideation skill and command | `assets/skills/givn-ideation/SKILL.md.tmpl`, `assets/commands/givn-ideation.md` | Scenarios 1–5 |
| Capability — guide-domain-modeling owns the skill's naming discipline | `assets/skills/givn-domain-modeling/SKILL.md.tmpl` | Scenarios 6–7 |

| Persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | Name the concept, not the tool | None — the question carries the concept description, up to three candidates with their collisions, and a free-form answer | Passes |
| agent--technical-role | Ask only what the user must decide | None — the triggers and the checkpoint question are explicit; no judgment is delegated to the agent | Passes |
| givn-maintainer--owner | One vocabulary across guidance, specs, and durable records | None — the dialog shape is single-sourced; the durable rows carry the same rule | Passes |

No Persona was invented or auto-promoted.

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Deferred mandatory checks

None. The proposal's "no machine enforcement" boundary is an accepted scope
decision, not a deferred mandatory check.

## Visual Review

- Interface: terminal — the change's user-visible surface is generated
  Markdown guidance and one command file consumed through the terminal. The
  opposite classification — a browser-rendered Web UI — does not hold: no
  changed path renders HTML, CSS, or JavaScript, and the `givn skills sync`
  terminal contract is unchanged.
- Viewports: not applicable — no Web UI.
- The change adds no delta `@e2e` scenario and no new UI state; no transcript
  is owed for a delta scenario, and the rendered guidance is produced by the
  existing `givn skills sync` action whose terminal surface already has
  `@e2e` coverage.

| @e2e Scenario | Evidence kind | Evidence file | Present and non-empty? |
|---|---|---|---|
| (none — no delta `@e2e` scenario) | transcript | — | N/A — no transcript is required for a scenario that does not exist |

| Persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | Read the question and answer it | None — passes; the question is complete sentences with inline-code terms | Passes |
| agent--technical-role | Present the question accurately | None — passes; the wording delegates no naming judgment to the agent | Passes |
| givn-maintainer--owner | Keep the vocabulary consistent | None — passes; the guidance uses the agreed terms throughout | Passes |

**Baseline (terminal):** headings identify their subject; sentences are
complete; terms are backticked inline; motion, focus, hover, and viewport
rules are nonapparent to static terminal content by construction, not
deferred.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires nothing.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | arc42.md claim | design.md treatment | tasks.md treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Chapter 8 — §8.20 Term Introduction Discipline | `docs/arc42/08-crosscutting-concepts.md:698-719` | Row 8 — Yes | First paragraph amended | Durable edits in the plan commit; convention text exercised by scenarios 1–3 | Line 705: "a recorded search-before-coin check, and the user's verified name (asked immediately when the name fails, otherwise at the phase's term check)" | Yes |
| Chapter 12 — Term record row | `docs/arc42/12-glossary.md:147` | Row 12 — Yes | Term record row amended | Plan commit | "carries the user's verified name — asked immediately when the name fails, otherwise at the phase's term check." | Yes |
| Durable workflow record — term record paragraph | `docs/workflow.md:85-93` | Recorded in the Arc42 impact | Amended | Plan commit | Line 90: "verified name — asked immediately when the name fails, otherwise at the phase's term check." | Yes |
| Chapter 9 — architecture decisions | `docs/arc42/adr/` | Not affected — no qualified ADR; routing `CANONICAL_ARTIFACT` | ADR verdict recorded | — | No new MADR entry; register unchanged | Yes |
| Chapter 11 — risks | R-68 | Not affected — row 11 stays No | Risks unchanged | — | R-68 text untouched | Yes |

- [x] `arc42.md` claims match the implementation
- [x] Every affected chapter was re-derived independently
- [x] The durable docs carry the implemented convention
- [x] No qualified ADR was required (canonical-artifact routing)
- [x] No chapter is contradicted or left stale

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt exists yet (the change is not archived). Classification
from static inspection: the change adds no executable production line — it
edits embedded Markdown guidance assets consumed by the existing Rust binary.
There is no script, binary, or instrumented process path to measure; coverage
measurement is not applicable to text assets.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `assets/skills/givn-domain-modeling/SKILL.md.tmpl` (changed bullet) | ~9 | Covered — asserted by delta scenarios 6–7 | None needed |
| `assets/skills/givn-ideation/SKILL.md.tmpl` (term record + verification section) | ~20 | Covered — asserted by delta scenarios 1–4 | None needed |
| `assets/commands/givn-ideation.md` (guardrail bullet) | 2 | Covered — asserted by delta scenario 5 | None needed |
| `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` (new steps) | ~69 | Covered — executed by delta scenarios 6–7 | None needed |
| Dead code introduced | — | None | Nothing to delete |

## Scenario execution evidence

- `givn lint --change user-verified-term-naming` → `2 file(s) checked — clean`
- Change delta — `GIVN_FEATURES=givn/changes/user-verified-term-naming/specs ./run-tests.sh` → `7 tests run: 7 passed, 0 skipped`
- Full suite — `./run-tests.sh` → `591 tests run: 591 passed, 0 skipped` (44.451s)
- E2E suite — `./run-tests.sh --e2e` → `90 tests run: 90 passed, 501 skipped` (7.453s)

**Receipt status:** not archived yet.

## Overlap dispositions

| Scenario A | Scenario B | Decision |
|---|---|---|
| The ideation skill ships without any addon enabled | The ideation guidance binds every confirmed term to the user's verified name | variant |
| The ideation guidance binds personas to the artifacts that affect them | The ideation guidance asks the naming question as soon as a name fails | variant |
| The ideation guidance carries the seed schema and its status values | The ideation guidance asks the term check before a phase closes | variant |
| The frame is confirmed once and later phase boundaries report | The verified-name gate holds without the addon | variant |
| Handoff cuts change seeds by deliverable guarantee | The ideation guidance asks the term check before a phase closes | variant |
| The ideation guidance requires a canvas with distinguishable element kinds | The ideation guidance asks the term check before a phase closes | variant |

All six shape matches are `variant`: the same action shape (a generated-
guidance assertion over the same skill or command file) with distinct
asserted content; merging would collapse different behaviours into one
scenario. The subset advisories on the same features carry the same
disposition — each delta scenario asserts a distinct subset of the guidance;
none duplicates another scenario's assertions wholesale.

## Split-or-keep

None — the lint reported no long-scenario finding; no scenario was flagged
for splitting.

## Sign-off

- [x] Fabrication audit clean (tag integrity, step bodies, commit cross-check, promised architecture, strict-mode proof)
- [x] Every checked task maps to production source (scenario 4 is a verified placement pin)
- [x] Components named by `design.md` exist in the tree
- [x] Strict-mode proof recorded with output
- [x] E2E isolation: `verify.e2e_command` distinct; counts 90 < 591
- [x] Coverage classified (static inspection; no executable production line)
- [x] No dead code introduced
- [x] No `@wip` scenario remains (lint clean)
- [x] Canonical E2E policy satisfied (no new action; existing `@e2e` coverage via the real binary)
- [x] Local run commands verified (delta, full, e2e)
- [x] Implementation matches the reviewed design (no silent deviation)
- [x] Interaction coverage complete (matrix rows against the existing `@e2e` scenarios)

README-IMPACT: none

README.md does not describe the generated guidance content or the naming
flow; the changed surfaces are the skill and command assets and the durable
records.

**REVIEW: PASS**
