# Review: validate-generated-references

## Fabrication Audit (do this first)

**@e2e tag integrity.** `git log --all -p -- givn/changes/validate-generated-references/specs | grep -c "@e2e"` returns 0: neither delta feature ever carried an `@e2e` tag, so none was removed, added, or edited. Clean.

**Step definition scan.** The nine new steps in
`tests/steps/instruction_integrity_steps.rs` carry real assertions or real
fixture work; 0 empty bodies; no `unimplemented!()` or `todo!()` remains. The
`{}` grep hits in that file are format-string placeholders inside panic
messages. The three guard `return;` statements live in the non-step helper
`tests/common/reference_check.rs` (`collect_skill_files`,
`collect_command_files`, `push_source`) and are real control flow, not empty
bodies. The classifier file itself has no stub left.

**Commit cross-check.**

| Scenario | Commit hash | Touches production artifact? |
|---|---|---|
| 1 — The ideation guidance carries the persona schema | `6224ef7f` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl` |
| 2 — The characterization guidance requests the reverse review from Givn | `e9d34960` | Yes — `assets/skills/givn-characterize/SKILL.md.tmpl`, `assets/commands/givn-characterize.md` |
| 3 — The generated guidance references resolve with the default configuration | `054825cf` | Yes — `tests/common/reference_check.rs` (the check is the deliverable), `assets/skills/givn-ideation/SKILL.md.tmpl`, `assets/skills/givn-spec/SKILL.md.tmpl`, `assets/commands/grill-with-docs.md` |
| 4 — The generated guidance references resolve without addons | `debeb675` | Yes — `tests/common/reference_check.rs` (skill-by-name resolution) |
| 5 — The printed instructions reference only resolvable files | `d0b83eb7` | Yes — `assets/instructions/specs.md`, `assets/instructions/review.md`, `tests/common/reference_check.rs` |
| 6 — An undeclared reference in an installed skill fails the check | `394ada3e` | Yes — the installed fail-closed path in `tests/steps/instruction_integrity_steps.rs` |
| 7 — A printed reference that resolves nowhere fails the check | `511ea8f1` | Yes — the printed fail-closed path in `tests/steps/instruction_integrity_steps.rs` |
| Post-verification regression repair | `98ae9173` | Yes — `assets/instructions/specs.md` (the placement-test sentence kept contiguous after the full suite caught the line-break regression) |

No production `src/` file is touched by design: the change is guidance
content plus the consumer-view check.

**Promised-architecture cross-check.**

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Candidate extraction + resolution tables (`GuidanceSource`, `Finding`, `installed_guidance`, `unresolved_references`, `describe`) | Yes | `tests/common/reference_check.rs` |
| Installed-corpus, printed-corpus, override, and failure steps | Yes | `tests/steps/instruction_integrity_steps.rs` |
| Module registration | Yes | `tests/common/mod.rs` (`pub mod reference_check;`) |
| Inline persona schema and dropped pointers | Yes | `assets/skills/givn-ideation/SKILL.md.tmpl`, `assets/skills/givn-spec/SKILL.md.tmpl` |
| CLI-routed reverse review in both surfaces | Yes | `assets/skills/givn-characterize/SKILL.md.tmpl`, `assets/commands/givn-characterize.md` |
| Reworded example | Yes | `assets/commands/grill-with-docs.md` |
| Dropped use-case-model pointers in the instructions | Yes | `assets/instructions/specs.md`, `assets/instructions/review.md` |
| Durable records | Yes | `docs/arc42/08-crosscutting-concepts.md` §8.7/§8.22, `10-quality-requirements.md` Q-97, `11-risks-and-technical-debt.md` D-20, `12-glossary.md` |

**Strict-mode proof.** `tests/features_runner.rs` calls `.fail_on_skipped()`
and exits non-zero on failed or skipped scenarios. Before implementation the
characterization scenario panicked on its stub: `exit=101`, `1 scenario
(1 failed)`, `Step panicked. Captured output: not implemented` — recorded in
`tasks.md` Setup.

**Downgraded e2e scenario scan.** No delta `@e2e` scenarios — nothing to scan.
The change adds no consumer action and does not modify the pre-existing
`@e2e` scenarios.

**E2e scope check.**

| Capability | Delta @e2e scenario count | Distinct new actions? | Over-produced? |
|---|---|---|---|
| instruction-integrity | 0 | None — no new consumer action | No |
| generate-ideation-guidance | 0 | None — no new consumer action | No |

**Duplicate-implementation check.** One `features_runner` harness
(`Cargo.toml`, `harness = false`) plus `ready_fixture` (fixture builder, not
an alternate e2e implementation). `run-tests.sh`, `verify.e2e_command`, and
`measure-coverage.sh` drive the same runner and tag selection. No second or
weaker implementation exists.

**Local environment check.**

```
GIVN_FEATURES=givn/changes/validate-generated-references/specs ./run-tests.sh  →  7 tests run: 7 passed, 0 skipped
./run-tests.sh                                                                 →  598 tests run: 598 passed, 0 skipped
./run-tests.sh --e2e                                                           →  90 tests run: 90 passed, 508 skipped
givn lint --change validate-generated-references                               →  2 file(s) checked — clean
```

**E2e isolation check.**

| verify.command | verify.e2e_command | Identical? | Regular scope count | E2E scope count | E2E strictly smaller? |
|---|---|---|---|---|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 598 | 90 | Yes — 90 < 598 |

**Design conformance diff.**

| Decision (from design.md) | design.md says | Actually built | Matches? |
|---|---|---|---|
| Reference classes | exists, workflow-created, rule-owned, skill-by-name; distribution paths never exempt | Implemented in `tests/common/reference_check.rs`; the two planted-reference scenarios prove fail-closed behaviour | Yes |
| Graded corpora | Installed skills/commands and every printed instruction (artifacts plus `review-reverse`, `specs-reverse`, `tasks-reverse`, `dialog`) | `installed_guidance` walks the configured targets; the printed When enumerates `givn graph --json` artifacts plus the four extra ids | Yes |
| Trailing-slash normalization and bare root candidates | Directory candidates resolve with or without a slash | `resolves` trims trailing slashes; the `givn/personas` table row gained its empty leaf after the first GREEN exposed the bare directory — within the design's stated matching rule, not a deviation | Yes |
| Rootless and fenced-example surface | Deliberate debt D-20 | Not graded; recorded in chapter 11 | Yes |
| Persona template asset | Stays; schema travels inline | Asset untouched; the gap-filling skill carries the schema; the permanent template scenario still executes | Yes |
| Check placement | Test support in `tests/common/reference_check.rs`, no `givn check` subcommand | No production module added | Yes |
| Step files | One file per capability; characterization and check steps in `instruction_integrity_steps.rs`; ideation reuses existing steps | As built | Yes |
| CLI route in both characterization surfaces | Skill and command | Both carry `` `givn instructions review-reverse` `` | Yes |
| Runner | `verify.command ./run-tests.sh`; strict mode already configured; single-scenario command from design.md | Used throughout; no configuration change was needed | Yes |

**Interaction coverage verification.** No `.feature` file in this repository
carries a `# User Interaction Inventory:` block — the canonical policy keeps
the inventory in the owning use-case document, and the review instruction's
step 13a wording predates that policy (recorded as an out-of-scope
discrepancy, not a finding against this change). Cross-referenced against
`givn/specs/fragments/fragment.md` and the design's matrix:

| Inventory entry | Matrix row exists? | @e2e scenario exists? | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| instruction-integrity: request effective instructions by id | Yes | Yes — "An agent requests machine-readable artifact guidance" | Real `givn instructions` subprocess | The existing e2e steps run the compiled binary via `common::run_givn` and assert stdout/JSON | Yes |
| instruction-integrity: request normal session-start instructions | Yes | Yes — "Author starts a session with the universal dialog obligation" | Real `givn instructions` subprocess | Same real-binary driver | Yes |
| generate-ideation-guidance: synchronize skills for the configured targets | Yes (annotated) | Yes — "A consumer synchronizes skills and every agent target receives the ideation facilitation skill" | Real `givn skills sync` subprocess | Same real-binary driver; the delta scenarios assert the rendered guidance | Yes |

## Use-Case And Persona Conformance

Owning contract: fragment `support`; capabilities `instruction-integrity`
and `generate-ideation-guidance`; no change seed exists (direct operator
investigation).

| Contract element | Where realized | Evidence |
|---|---|---|
| Guarantee — guidance stays complete and trustworthy; a dead reference is an incomplete instruction | The consumer-view check fails any rooted reference that resolves nowhere or lacks a creation owner; the dead pointers were removed | Delta scenarios 3–7 |
| Capability — instruction-integrity owns the graded guidance | `tests/common/reference_check.rs`, `tests/steps/instruction_integrity_steps.rs`, the four guidance assets | Scenarios 2–7 |
| Capability — generate-ideation-guidance owns the self-contained persona schema | `assets/skills/givn-ideation/SKILL.md.tmpl` inline schema block | Scenario 1 |

| Persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | Receive guidance whose referenced files exist in the project | None — the observed session's failure class is closed and guarded | Passes |
| agent--technical-role | Follow guidance references without hunting | None — no distribution path remains in any graded surface | Passes |
| givn-maintainer--owner | Keep the guidance corpus honest and enforced | None — the check grades installed and printed guidance, and the tables name each creator | Passes |

No Persona was invented or auto-promoted.

PERSONAS: developer--primary-user
PERSONAS: agent--technical-role
PERSONAS: givn-maintainer--owner

## Deferred mandatory checks

None. No mandatory check is deferred.

## Visual Review

- Interface: terminal — the change's user-visible surface is generated
  Markdown guidance and printed instruction text consumed through the
  terminal. The opposite classification — a browser-rendered Web UI — does
  not hold: no changed path renders HTML, CSS, or JavaScript, and the
  `givn skills sync` / `givn instructions` stdout contracts are unchanged.
- Viewports: not applicable — no Web UI.
- The change adds no delta `@e2e` scenario and no new UI state; no transcript
  is owed for a delta scenario, and the rendered guidance is produced by the
  existing `givn skills sync` and `givn instructions` actions whose terminal
  surfaces already have `@e2e` coverage.

| @e2e Scenario | Evidence kind | Evidence file | Present and non-empty? |
|---|---|---|---|
| (none — no delta `@e2e` scenario) | transcript | — | N/A — no transcript is required for a scenario that does not exist |

| Persona | Goal | Finding | Disposition |
|---|---|---|---|
| developer--primary-user | Read working guidance in the project | None — passes; the schema is self-contained and the pointers are gone | Passes |
| agent--technical-role | Follow references without searching the machine | None — passes; the check proves every graded reference resolves | Passes |
| givn-maintainer--owner | Keep the corpus and the check consistent | None — passes; the tables and the scenarios agree | Passes |

**Baseline (terminal):** headings identify their subject; sentences are
complete; references are backticked inline; motion, focus, hover, and
viewport rules are nonapparent to static terminal content by construction.

VISUAL-REVIEW: PASS

## Retirements

N/A — the change retires no scenario, asset, or behavior. The repository
persona template stays; only its generated-guidance references changed.

## Arc42 implementation conformance

| Arc42 chapter or architectural fact | Durable-doc source | arc42.md claim | design.md treatment | tasks.md treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| §8.7 — consumer-view resolution rule | `docs/arc42/08-crosscutting-concepts.md` (new paragraph) | Row 8 — Yes | Named the exact paragraph text | Durable edit in the plan commit | The paragraph reads "a named creation step … a reference into the Givn distribution is a defect"; the code implements exactly those classes | Yes |
| §8.22 — persona schema home | `docs/arc42/08-crosscutting-concepts.md` | Row 8 — Yes | Named the replacement sentence | Plan commit | "The ideation guidance carries the persona schema…" matches the inlined block | Yes |
| Chapter 10 — Q-97 guidance integrity | `docs/arc42/10-quality-requirements.md` | Row 10 — Yes | Q-97 recorded; 10.3 signal named | Plan commit; verified by scenarios 3–7 | Q-97 row and the 10.3 bullet match the implemented property | Yes |
| Chapter 11 — D-20 graded-surface limits | `docs/arc42/11-risks-and-technical-debt.md` | Row 11 — Yes | D-20 recorded | Plan commit | D-20 names the rooted-only extraction, fenced examples, and the table-upkeep obligation; the code matches | Yes |
| Chapter 12 — `Consumer project`, `Generated guidance` | `docs/arc42/12-glossary.md` | Row 12 — Yes | Terms used consistently | Plan commit | Both rows present; proposal, specs, design, and §8.7 use the same vocabulary | Yes |
| Chapter 9 — architecture decisions | `docs/arc42/adr/` | Row 9 — No; routing `CANONICAL_ARTIFACT` to §8.7 | Verdict recorded with falsification tests | — | No new MADR entry; register unchanged | Yes |
| §8.22 — repository template | `assets/templates/persona.md` | Row 8 — Yes | Kept; permanent scenario untouched | — | The file and its scenario still exist and pass in the full suite | Yes |

- [x] `arc42.md` claims match the implementation
- [x] Every affected chapter was re-derived independently
- [x] The durable docs carry the implemented rule and terms
- [x] No qualified ADR was required (canonical-artifact routing)
- [x] No chapter is contradicted or left stale

ARC42 CONFORMANCE: CLEAN

## Coverage

No archive receipt exists yet (the change is not archived). Classification
from static inspection: the change adds no executable production line. It
edits embedded Markdown guidance assets consumed by the existing Rust binary
and adds test-support Rust that the acceptance suite executes. Coverage
measurement applies to production source, which is unchanged; the archive
receipt will carry the measured result.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `assets/skills/givn-ideation/SKILL.md.tmpl` (schema + dropped pointer) | ~30 | Non-executable asset; asserted by delta scenarios 1 and 3 | None needed |
| `assets/skills/givn-characterize/SKILL.md.tmpl`, `assets/commands/givn-characterize.md` | 4 | Non-executable asset; asserted by delta scenario 2 | None needed |
| `assets/instructions/specs.md`, `assets/instructions/review.md` | ~10 | Non-executable asset; asserted by delta scenario 5 | None needed |
| `assets/commands/grill-with-docs.md` | 1 | Non-executable asset; asserted by delta scenario 3 | None needed |
| `tests/common/reference_check.rs` | ~300 | Test support executed by delta scenarios 3–7 | None needed |
| `tests/steps/instruction_integrity_steps.rs` (new steps) | ~120 | Test support executed by delta scenarios 2–7 | None needed |
| Dead code introduced | — | None | Nothing to delete |

## Scenario execution evidence

- `givn lint --change validate-generated-references` → `2 file(s) checked — clean`
- Change delta — `GIVN_FEATURES=givn/changes/validate-generated-references/specs ./run-tests.sh` → `7 tests run: 7 passed, 0 skipped`
- Full suite — `./run-tests.sh` → `598 tests run: 598 passed, 0 skipped` (45.609s)
- E2E suite — `./run-tests.sh --e2e` → `90 tests run: 90 passed, 508 skipped` (7.621s)

**Receipt status:** not archived yet.

## Overlap dispositions

| Scenario A | Scenario B | Decision |
|---|---|---|
| The generated guidance references resolve with the default configuration | The generated guidance references resolve without addons | variant |
| The ideation guidance carries the seed schema and its status values | The ideation guidance carries the persona schema | variant |
| Handoff cuts change seeds by deliverable guarantee | The ideation guidance carries the persona schema | variant |
| The ideation guidance requires a canvas with distinguishable element kinds | The ideation guidance carries the persona schema | variant |
| The ideation guidance asks the term check before a phase closes | The ideation guidance carries the persona schema | variant |

All five shape matches are `variant`: the same assertion shape over the same
generated-guidance files with distinct asserted facts. The default/no-addons
pair exercises distinct conditional renderings (arc42-on vs arc42-off
branches, all targets vs opencode-only), so merging would drop branch
coverage. The persona-schema scenario asserts the document skeleton headings
(`Stable core`, `Authority boundary`, `Evaluation capability`,
`Own vocabulary`, `Success criterion`, `Running record`, `append-only`,
`Topic stances`), which no other scenario asserts. The `[SUBST]` advisories —
the schema scenario reported as a subset of five permanent guidance
scenarios, and the reverse subset report for several permanent scenarios —
carry the same disposition: the relation is fingerprint-shape overlap on the
shared "generated guidance contains" step, not assertion duplication; each
scenario pins different content, and the schema scenario is the only pin for
the persona document shape after the pointer is gone.

## Split-or-keep

None — the lint reported no long-scenario finding; no scenario was flagged
for splitting.

## Sign-off

- [x] Fabrication audit clean (tag integrity, step bodies, commit cross-check, promised architecture, strict-mode proof)
- [x] Every checked task maps to a production artifact or the check itself
- [x] Components named by `design.md` exist in the tree
- [x] Strict-mode proof recorded with output
- [x] E2E isolation: `verify.e2e_command` distinct; counts 90 < 598
- [x] Coverage classified (static inspection; no executable production line changed)
- [x] No dead code introduced
- [x] No `@wip` scenario remains (lint clean)
- [x] Canonical E2E policy satisfied (no new action; existing `@e2e` coverage via the real binary)
- [x] Local run commands verified (delta, full, e2e, lint)
- [x] Implementation matches the reviewed design (no silent deviation)
- [x] Interaction coverage complete (fragment inventory rows against the existing `@e2e` scenarios)

README-IMPACT: none

README.md describes installation, the lifecycle, and the run declaration; it
does not describe the generated guidance content or reference-resolution
rules, so the change does not touch its public face.

**REVIEW: PASS**
