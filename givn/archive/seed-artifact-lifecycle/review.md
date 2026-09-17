# Review: seed-artifact-lifecycle

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — the delta has no `@e2e` scenario and none was removed. Running the static lint check is already driven end-to-end by the sibling `spec-overlap-gates` scenarios, recorded in the design matrix. |
| Empty/no-op step bodies | 0 found. `grep -n "unimplemented!\|todo!()" tests/steps/mod.rs` returns no matches. |
| Checked tasks have commits touching production | Clean — setup commit `d49e205c` plus seven scenario commits (`9b2a9a77`, `b5ed5fab`, `3496d62a`, `0ede1371`, `49b14d8f`, `80307b57`, `2d83f846`), each touching `src/cli/lint.rs`, assets, or real step assertions. |
| Promised components exist | Clean — `check_seed_status`, `parse_seed_status`, the `[SEED ]` render arm, the seed schema, the propose/archive stamping instructions, and the two new fixture steps all exist. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario; existing `@e2e` scenarios are untouched. |
| `verify.e2e_command` isolation | `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e`. Counts: 498 regular vs 88 e2e — strictly fewer. |
| Implementation vs design.md | Matches, with one recorded refinement: the standalone active-change scenario was folded into the dangling scenario during implementation (it could only pass vacuously against pre-change code); `design.md` and `design-review.md` were updated in the same commit (`0ede1371`). |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| lint-command: run the static lint check | yes (sibling coverage) | `A duplicate title across two permanent spec files is a hard error` | Real `givn lint` subprocess |
| lint-command: inspect lint JSON | yes (sibling coverage) | `The json output reports duplicate-title findings machine-readably` | Real `givn lint --json` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | yes | `A consumer synchronizes skills and every agent target receives the ideation facilitation skill` | Real `givn skills sync` subprocess |

No new `@e2e` scenario is added: running the static lint check is one
normalized action, already proven end-to-end by the sibling capability's
scenarios, and the new scenarios are regular variants (advisory findings,
guidance phrases). No browser UI exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `cli::lint` gains the advisory seed-status check | 05 §5.3 row | Yes | Advisory Seed-Status Check | lint scenario tasks | `src/cli/lint.rs::check_seed_status` | Yes |
| Seed scan, labels, exit-code neutrality in the lint flow | 06 §6.4 | Yes | Advisory Seed-Status Check | lint scenario tasks | `run_with_root` restructure + `[SEED ]` render arm | Yes |
| Seed lifecycle convention | 08 §8.19 | Yes | Seed Content Schema, Guidance Edits | ideation-skill, prose-consolidation tasks | skill schema + propose/archive instructions | Yes |
| R-46 mitigation names the advisory check | 11 table | Yes | Advisory Seed-Status Check | lint scenario tasks | the check exists and is asserted | Yes |
| `Seed status` term | 12 glossary | Yes | Seed Content Schema | ideation-skill task | glossary row + skill status values | Yes |
| Chapter 10 unaffected | 10 (unchanged) | No | (not named) | (no task) | functional scenarios only | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Change seed` and `Seed status`. Specs, design, and code
reuse those terms: the delta scenarios say "change seed" and "status line";
`parse_seed_status` and the skill schema use exactly the five status values.
No glossary term is reused inconsistently and no undefined term appears in the
spec. Clean.

## Visual review

The change declares no visual interface: it adds CLI text output and guidance
text, not a rendered screen. Opposite-classification test: the lint advisories
are plain terminal text; nothing renders HTML, CSS, or JavaScript in a
browser, so no browser driver applies. No `@e2e` scenario in this delta
requires a screenshot or transcript. The visual block is N/A.

## Overlap dispositions

Every non-contradicting shape match involving this change, with its decision.
The delta scenarios share a generic assertion shape (`the command exits 0`,
`stderr contains …`) with the lint-shape machinery scenarios; each pair sits
on a different behavior boundary, so no pair is a duplicate or needs a split.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A change seed without a status line is reported as an advisory` | `An unrecognised seed status is reported and a status after prose is honoured` | boundary |
| `A change seed proposed by an archived change is reported as stale` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `A change seed proposed by an archived change is reported as stale` | boundary |
| `A scenario whose assertion sequence is a subset of another produces a subset warning` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `A change seed proposed by an archived change is reported as stale` | boundary |
| `A scenario with more than 14 steps produces a long-scenario warning` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |
| `Consecutive And and But steps normalise to the same shape as plain Then steps` | `A change seed proposed by an archived change is reported as stale` | boundary |
| `Consecutive And and But steps normalise to the same shape as plain Then steps` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |
| `Quoted strings are masked in the shape comparison` | `A change seed proposed by an archived change is reported as stale` | boundary |
| `Quoted strings are masked in the shape comparison` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |
| `Scenario outlines sharing a template produce a shape-match warning` | `A change seed proposed by an archived change is reported as stale` | boundary |
| `Scenario outlines sharing a template produce a shape-match warning` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |
| `Docstrings are ignored in the shape comparison` | `A change seed proposed by an archived change is reported as stale` | boundary |
| `Docstrings are ignored in the shape comparison` | `An advisory seed finding is counted in the summary and still exits 0` | boundary |

Four further shape matches are contradiction findings (same shape, opposite
facts) between permanent lint-shape scenarios and the delta; the overlap gate
does not require dispositions for contradicted pairs, and none of them is a
behavioral duplicate. The deterministic scan additionally reports 203
assertion-subset warnings between delta and permanent scenarios, all
structural overlaps on generic shared assertions.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`An unrecognised seed status is reported and a status after prose is honoured`
at 9 steps). No split-or-keep decisions are required.

## Removed+added pairs

None. The delta adds 7 scenarios and modifies none; no scenario is removed.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- `check_seed_status` and `parse_seed_status`: covered by the five lint
  scenarios (missing/clean, dangling/active, stale, unrecognised/prose,
  mixed-run count). No uncovered region.
- The early-return rendering and the advisory count: covered by the dangling
  and mixed-run scenarios.
- The seed schema and stamping instructions: asserted by the ideation-skill
  and prose-consolidation scenarios.

No dead code introduced; no missing coverage; no hard-to-test gap.

## Use-case and Persona conformance

- `author-change` rules: generated guidance is synchronized from canonical
  assets and names only commands that work; both are asserted.
- `verify-change` rule: lint findings have a stable exit-code contract; the
  advisory scenarios assert exit 0.
- Support-fragment `ideation-skill` keeps its fragment ownership.
- Confirmed Personas: `agent--technical-role` (guidance and lint output) and
  `givn-maintainer--owner` (seed convention). No Persona was invented or
  promoted; Gherkin actors remain the Developer and the Authoring agent.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 498 passed, e2e
      88 passed; delta 7 passed); the archive receipt will carry the measured
      coverage.
- [x] Coverage classified from inspection; no gap remains.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No redundant unit tests added.
- [x] No `@wip` tags remain in the delta; no implementation-layer detail in
      the spec.
- [x] Canonical E2E policy read; every normalized inventory action has one
      `@e2e` scenario and the new scenarios are regular variants.
- [x] Every E2E scenario keeps its real-interface primary assertion.
- [x] Local run command starts the full stack: `./run-tests.sh`; no external
      dependency or digital twin exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; e2e scope is
      strictly smaller (88 vs 498).
- [x] Implementation matches design.md; the one refinement was recorded in
      design.md and design-review.md in the same commit.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
