# Review: cover-every-agent-target

## Fabrication Audit (do this first)

**@e2e tag integrity.** `git log --all -p -- givn/changes/cover-every-agent-target/specs | grep -c "@e2e"` returns 0: neither delta entry ever carried an `@e2e` tag, so none was removed, added, or edited. Clean.

**Step definition scan.** The four new steps in
`tests/steps/instruction_integrity_steps.rs` carry real assertions or real
fixture work; 0 empty bodies; no `unimplemented!()` or `todo!()`; no bare
`return;` in any touched step file. The `{}` grep hits are format-string
placeholders inside panic messages. The three modified assertion loops
(fixture derivations, coverage guard) were exercised by the scenarios
recorded in `tasks.md`.

**Commit cross-check.**

| Scenario | Commit hash | Touches production artifact? |
|---|---|---|
| 1 — The characterization guidance requests the reverse review from Givn (modified) | `8dfa6373` | Yes — the four production-derived fixtures, the two derived loops, `owns_commands`, and the new command-owning step |
| 2 — The generated guidance references resolve with the default configuration (modified) | `3caeae8a` | Yes — `recognizes_target_dir`/`uncovered_targets` in the check, the strengthened positive, the coverage guard step |
| 3 — A configured target without synchronized guidance fails the check (added) | `9cd93e96` | Yes — the removal When and the failure Then over `uncovered_targets` |

No production `src/` file is touched by design: the change hardens test
support.

**Promised-architecture cross-check.**

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Production-derived recognition + `uncovered_targets` | Yes | `tests/common/reference_check.rs` |
| Four production-derived fixtures | Yes | `prose_consolidation_steps.rs` (×2), `prose_consolidation_e2e_steps.rs`, `instruction_integrity_steps.rs` |
| Derived loops and expectation | Yes | `core_entries_require_dialog_policy`, `skill_consumers_receive_replacement`, `selected_targets_and_addons_unchanged` |
| Four new steps | Yes | `instruction_integrity_steps.rs` |
| Durable records | Yes | `docs/arc42/10-quality-requirements.md` (Q-97, 10.3), `docs/arc42/11-risks-and-technical-debt.md` (D-21) |

**Strict-mode proof.** `tests/features_runner.rs` calls `.fail_on_skipped()`
and exits non-zero on failed or skipped scenarios. Before implementation the
change scope failed 3/3 with undefined steps (`exit=100`), recorded in
`tasks.md` Setup; each scenario then failed RED on its undefined step by name
(`Step doesn't match any function`).

**Downgraded e2e scenario scan.** No delta `@e2e` scenarios — nothing to scan.
The change adds no consumer action and does not modify the pre-existing
`@e2e` scenarios.

**E2e scope check.**

| Capability | Delta @e2e scenario count | Distinct new actions? | Over-produced? |
|---|---|---|---|
| instruction-integrity | 0 | None — no new consumer action | No |

**Duplicate-implementation check.** One `features_runner` harness
(`Cargo.toml`, `harness = false`) plus `ready_fixture` (fixture builder, not
an alternate e2e implementation). `run-tests.sh`, `verify.e2e_command`, and
`measure-coverage.sh` drive the same runner and tag selection. No second or
weaker implementation exists.

**Local environment check.**

```
GIVN_FEATURES=givn/changes/cover-every-agent-target/specs ./run-tests.sh  →  3 tests run: 3 passed, 0 skipped
./run-tests.sh                                                           →  605 tests run: 605 passed, 0 skipped
./run-tests.sh --e2e                                                     →  90 tests run: 90 passed, 515 skipped
givn lint --change cover-every-agent-target                              →  1 file(s) checked — clean
```

**E2e isolation check.**

| verify.command | verify.e2e_command | Identical? | Regular scope count | E2E scope count | E2E strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 605 | 90 | Yes — 90 < 605 |

**Design conformance diff.**

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Production-derived recognition | `recognizes_target_dir` over `AgentTarget::ALL`; `AGENT_TARGETS` deleted | Implemented; `is_candidate` and `skill_reference` use the predicate | Yes |
| Executable coupling guard | The coverage step asserts recognition for every production target | `every_agent_target_is_covered` asserts recognition and per-target sources | Yes |
| Four production-derived fixtures | Prompt and `--agents` fixtures plus the E2E and addon-free fixtures select `AgentTarget::ALL` | All four build their selection from `AgentTarget::ALL` | Yes |
| Derived expectation and loops | `selected_targets_and_addons_unchanged` and the two "every target" loops follow configuration/production | Implemented; the full suite proves the fixture blast radius | Yes |
| Installed positive strengthened | `uncovered_targets` asserted empty; printed positive exempt | Implemented; the no-addons fixture passes with an empty uncovered list | Yes |
| Fail-closed scenario | Remove a target's skills, expect the target named | Implemented; the helper returns the directory form (`.claude`) after the first run exposed the bare-name mismatch — a clarification inside the design's frame, reflected in the design snippet | Yes |
| Characterization assertions | New production command-owning step; existing content assertion uses `owns_commands` and fails on a missing file | Implemented in both places | Yes |
| Out-of-scope literals | `enable-optional-addons` helper and other judged-deliberate lists untouched | Untouched | Yes |
| Runner | `verify.command ./run-tests.sh`; strict mode; single-scenario command from design.md | Used throughout; no configuration change | Yes |

**Interaction coverage verification.** No `.feature` file in this repository
carries a `# User Interaction Inventory:` block; the canonical policy keeps
the inventory in the owning use-case document. Cross-referenced against
`givn/specs/fragments/fragment.md` and the design's matrix:

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| instruction-integrity: request effective instructions by id | Yes | Yes — "An agent requests machine-readable artifact guidance" | Real `givn instructions` subprocess | Existing e2e steps run the compiled binary via `common::run_givn` | Yes |
| instruction-integrity: request normal session-start instructions | Yes | Yes — "Author starts a session with the universal dialog obligation" | Real `givn instructions` subprocess | Same real-binary driver | Yes |

## Use-Case And Persona Conformance

Owning contract: fragment `support`; capability `instruction-integrity`; no
change seed exists (direct operator follow-up).

| Contract element | Where realized | Evidence |
|---|---|---|
| Guarantee — guidance stays complete and trustworthy for every supported harness | Production-derived recognition; all-targets fixtures and loops; per-target coverage assertion; fail-closed scenario | Delta scenarios 1–3 |
| Capability — instruction-integrity owns the graded guidance | `tests/common/reference_check.rs`, `tests/steps/instruction_integrity_steps.rs`, the affected fixtures | Scenarios 1–3 |

| Persona | Goal | Finding | Disposition |
|---|---|---|---|
| givn-maintainer--owner | Keep the suite honest about the target list | None — the suite now follows production and fails when a configured target is empty | Passes |
| agent--technical-role | Have its harnesses graded, including future ones | None — recognition and fixtures derive from `AgentTarget::ALL` | Passes |

No Persona was invented or auto-promoted.

PERSONAS: givn-maintainer--owner
PERSONAS: agent--technical-role

## Deferred mandatory checks

None. No mandatory check is deferred.

## Visual Review

- Interface: terminal — the change is test support and Gherkin consumed
  through the terminal. The opposite classification — a browser-rendered Web
  UI — does not hold: no changed path renders HTML, CSS, or JavaScript.
- Viewports: not applicable — no Web UI.
- The change adds no delta `@e2e` scenario and no new UI state; no transcript
  is owed for a delta scenario.

| @e2e Scenario | Evidence kind | Evidence file | Present and non-empty? |
|---|---|---|---|
| (none — no delta `@e2e` scenario) | transcript | — | N/A — no transcript is required for a scenario that does not exist |

| Persona | Goal | Finding | Disposition |
|---|---|---|---|
| givn-maintainer--owner | Read honest suite output | None — passes; failure messages name the uncovered target | Passes |
| agent--technical-role | Rely on its harnesses being graded | None — passes; no consumer-facing text changed | Passes |

**Baseline (terminal):** headings identify their subject; sentences are
complete; references are backticked inline; motion, focus, hover, and
viewport rules are nonapparent to static terminal content by construction.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires no scenario, asset, or behavior.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | arc42.md claim | design.md treatment | tasks.md treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Chapter 10 — Q-97 guidance integrity | `docs/arc42/10-quality-requirements.md` | Row 10 — Yes | Q-97 row gains the no-guidance clause; 10.3 refined | Durable edits in the plan commit | Q-97 reads "…fails the first reference that resolves nowhere, and a configured target with no synchronized guidance fails the check"; 10.3 names the production-derived coverage; the scenario proves the failure | Yes |
| Chapter 10 — 10.3 verification signal | `docs/arc42/10-quality-requirements.md` | Row 10 — Yes | Named | Plan commit | The bullet states the check covers every production agent target and requires synchronized guidance from each configured one | Yes |
| Chapter 11 — D-21 coverage granularity | `docs/arc42/11-risks-and-technical-debt.md` | Row 11 — Yes | D-21 recorded | Plan commit | D-21 names the "at least one source per configured target" limit; `uncovered_targets` implements exactly that granularity | Yes |
| Chapter 8 — 8.7 resolution rule | `docs/arc42/08-crosscutting-concepts.md` | Row 8 — No | Explicitly unaffected | — | The rule text is unchanged; the suite's target derivation is a check property | Yes |
| Chapter 12 — `Agent target` | `docs/arc42/12-glossary.md` | Row 12 — No | No new term | — | The glossary row already defines all four destinations used by the derivation | Yes |
| Chapter 9 — architecture decisions | `docs/arc42/adr/` | Row 9 — No; routing `CANONICAL_ARTIFACT` to 10.3 | Verdict recorded with falsification tests | — | No new MADR entry; register unchanged | Yes |

- [x] `arc42.md` claims match the implementation
- [x] Every affected chapter was re-derived independently
- [x] The durable docs carry the implemented guarantee
- [x] No qualified ADR was required (canonical-artifact routing)
- [x] No chapter is contradicted or left stale

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt exists yet (the change is not archived). Classification
from static inspection: the change adds no executable production line — it
edits test-support Rust and Gherkin. Coverage measurement applies to
production source, which is unchanged; the archive receipt will carry the
measured result.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `tests/common/reference_check.rs` (recognition + `uncovered_targets`) | ~30 | Test support executed by delta scenarios 2–3 | None needed |
| `tests/steps/*` (four fixtures, two loops, expectation, four steps) | ~120 | Test support executed by delta scenarios 1–3 and the permanent suites | None needed |
| Dead code introduced | — | None | Nothing to delete |

## Scenario execution evidence

- `givn lint --change cover-every-agent-target` → `1 file(s) checked — clean`
- Change delta — `GIVN_FEATURES=givn/changes/cover-every-agent-target/specs ./run-tests.sh` → `3 tests run: 3 passed, 0 skipped`
- Full suite — `./run-tests.sh` → `605 tests run: 605 passed, 0 skipped` (46.129s)
- E2E suite — `./run-tests.sh --e2e` → `90 tests run: 90 passed, 515 skipped` (7.678s)

**Receipt status:** not archived yet.

## Overlap dispositions

No shape-match findings to dispose; the lint's one `[SUBST]` advisory is a
subset relation on the same assertion shape: the no-addons scenario asserts
the reference-resolution property over a different configuration
(opencode-only) and is the only scenario that exercises the resolve-skill-by-
name rule for `.agents/skills/givn-steps/SKILL.md`. Merging would drop the
configuration branch; the same disposition pattern was accepted for the same
pair in `validate-generated-references`.

## Split-or-keep

None — the lint reported no long-scenario finding; no scenario was flagged
for splitting.

## Sign-off

- [x] Fabrication audit clean (tag integrity, step bodies, commit cross-check, promised architecture, strict-mode proof)
- [x] Every checked task maps to a production artifact (the check and fixtures)
- [x] Components named by `design.md` exist in the tree
- [x] Strict-mode proof recorded with output
- [x] E2E isolation: `verify.e2e_command` distinct; counts 90 < 605
- [x] Coverage classified (static inspection; no executable production line changed)
- [x] No dead code introduced
- [x] No `@wip` scenario remains (lint clean)
- [x] Canonical E2E policy satisfied (no new action; existing `@e2e` coverage via the real binary)
- [x] Local run commands verified (delta, full, e2e, lint)
- [x] Implementation matches the reviewed design (no silent deviation; the helper's name form was clarified in design.md)
- [x] Interaction coverage complete (fragment inventory rows against the existing `@e2e` scenarios)

README-IMPACT: none

README.md describes installation, the lifecycle, and the run declaration; it
does not describe the acceptance suite's harness coverage, so the change does
not touch its public face.

**REVIEW: PASS**
