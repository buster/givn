# Review: add-ideation-skill

## Fabrication Audit

**@e2e tag integrity**: the delivery scenario carries `@givn.added @e2e`;
git history shows the tag present since creation (commit `b81244e`) and never
removed. PASS.

**Step body scan**: `0` empty/no-op step bodies found across the touched step
definition files (`tests/steps/prose_consolidation_steps.rs` — real targets
parsing via the production loader; the two grep hits are panic-message strings
inside real assertions). No new step files were needed; all bindings are
existing, proven steps.

**Commit cross-check** (every checked task → commit → production diff):

| Task | Commit(s) | Touches production code? |
|---|---|---|
| Planning layer | `b93f2c5` | Docs + spec delta (planning layer, pre-implementation by design) |
| ships without any addon enabled | `06b1113` | Yes — `assets/skills/givn-ideation/SKILL.md.tmpl`, `src/skills/render.rs`, `tests/steps/prose_consolidation_steps.rs` |
| phase ladder | `02e8817` | Yes — template |
| persona binding | `f773046` + `cd37f6f` | Yes — template |
| topic state + handoff safety | `4b6b79c` | Yes — template |
| explore wiring | `3d9c53b` | Yes — both explore assets |
| delivery `@e2e` | `b81244e` | Yes — `assets/commands/givn-ideation.md`, `src/skills/targets.rs` |
| marker-ban fix (final verification) | `e7bf9e8` | Yes — template, design/design-review/tasks records |

**Promised components**: skill template, command template,
`default_skill_names()` entry, `default_command_names()` entry, explore
transition row and command offer line — all present in the tree.

**Strict-mode proof**: present in `tasks.md` — undefined-step probe failed
with exit 101 ("Step doesn't match any function"); `.fail_on_skipped()` in
`tests/features_runner.rs:20`.

**E2E downgrade check**: the delivery scenario's Then steps assert CLI stdout
("Syncing skills and agent commands...", "synced skill: givn-ideation") and
generated-file content through the real `givn skills sync` subprocess — the
real interface, not repository-only. Not a browser capability (steps 5/6
N/A).

**verify.e2e_command identity and isolation**: `./run-tests.sh --e2e` vs
`./run-tests.sh` — different strings, tag-filtered (`@e2e`) subset. Counts
proven in `tasks.md` Setup: 1 e2e scenario < 6 total (full suite: 354/1).
Exact invoked surface: `run-tests.sh --e2e` →
`cargo test --test features_runner -- --tags '… @e2e'`. Tree search found no
second/parallel e2e implementation for this capability (the capability binds
generic parameterized steps; `grep -rln ideation tests/` returns nothing).

**Design conformance**: as-built matches design.md — the two registration
lists, both assets, the declared shared-helper change, and the runner
commands. The one evolution (generated-content marker ban, state file renamed
`glossary-candidates.md` → `domain-terms.md`) went through the
never-silently-deviate procedure: design.md amended, fresh-context design
re-review passed (recorded in `design-review.md`, "Amendment re-pass").

**Interaction coverage cross-reference**:

| Inventory entry (support/group.md) | Matrix row (design.md) | @e2e scenario | Driving mechanism verified |
|---|---|---|---|
| A consumer synchronizes skills and every agent target receives the ideation facilitation skill | same row, present | same title, `@e2e` in delta | Real `givn skills sync` subprocess via the generic runner step; stdout + all configured targets + command file asserted |

**Result:** CLEAN — 0 findings.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| New user-facing capability in scope | `docs/arc42/03-context-and-scope.md` In-scope bullet (synchronized `givn-ideation` skill) | Row 3 Yes | Skill Content + registration sections | Scenario 1/6 tasks | `givn skills sync` renders the skill to all three targets + command file | Yes |
| Risk R-46 (unenforced ideation/persona state convention) | `docs/arc42/11-risks-and-technical-debt.md` R-46 | Row 11 Yes | Marker-ban amendment section; "no machine enforcement" boundary | Scenario tasks assert contract phrases only | Specs assert generated content; no binary validation exists — as documented | Yes |
| Ideation vocabulary (14+1 terms) | `docs/arc42/12-glossary.md` Ideation block + Domain terms | Row 12 Yes | Contract-phrase table uses exactly these terms | Scenarios assert the phrases | Rendered guidance contains them (all six scenarios green) | Yes |
| No qualified ADR | chapter 09 register unchanged | Row 9 No (routing CANONICAL_ARTIFACT) | Version Freshness + amendment sections record the non-qualification | n/a | No ADR file created | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

All spec and design terms are glossary terms with consistent usage: Ideation,
Frame, Event Storming, Use Case Definition, Use Case Elaboration, Handoff,
Persona, Stakeholder type, Persona segment, Persona promotion, Change seed,
Quality review, Open question. Anti-terms respected ("quality review", never
"gate"; "open question", never "blocker"). One gap found and fixed during this
review: the design amendment's per-topic candidates file (`domain-terms.md`)
was not a glossary term — recorded now as **Domain terms** in
`docs/arc42/12-glossary.md` (with the marker-ban rationale as its anti-term
note). `docs/reference.md` agent-skills table updated with the `givn-ideation`
row (skills registry; the README points to it and itself stays unchanged).

## Coverage

Measurement: `GIVN_FEATURES=givn/changes/add-ideation-skill/specs
./measure-coverage.sh` (exit 0, llvm-cov replay of the change's scenarios)
followed by `./merge-coverages.sh` (exit 0, fresh merged report
`coverage/cobertura-coverage.xml`, 16.7 MB, produced at review time).

Production regions touched by this change:

- `src/skills/render.rs` `default_skill_names()` (lines 69–84): every
  statement region hit ×12 (12 sync test invocations exercise the list).
- `src/skills/targets.rs` `default_command_names()` (lines 321–336): every
  statement region hit ×12; line 340 (following function) hit ×176.

Closing-delimiter lines (render.rs:84, targets.rs:336) report zero hits —
an llvm-cov region-counting artifact on `]`/`}` delimiters; every statement
region in both functions is exercised. Not a coverage gap.

**COVERAGE MEASUREMENT: VALID** — instrumented, fresh, merged, with a
non-zero exercised production path.

## Coverage gap classification

1. Dead code: none — nothing to delete.
2. Missing test coverage: none — the only production lines this change adds
   are the two registration entries, both exercised ×12; all behavior is
   asserted through the six `.feature` scenarios at the CLI boundary.
3. Legitimately hard to test: none.

Redundant unit tests: none added; no duplicates removed.

## Deterministic gate dispositions

### Overlap dispositions

| Scenario A | Scenario B | Decision | Rationale |
|---|---|---|---|
| `The ideation guidance carries the phase ladder and the one-question rule` | `The ideation guidance persists topic state, parks unknowns, and keeps handoff from starting changes` | boundary | Same Given/When (skills sync) but distinct behavior facets of one generated artifact: phase ladder and interaction rule vs. topic state layout and handoff safety. Assertion sets are disjoint (phase names and the one-question rule vs. state paths, change seed, and the handoff safety sentence); each scenario fails independently on its own phrases. |

The remaining lint findings are assertion-subset (SUBST) relations, not
shape matches, and are inherent to a shared-content-contract surface: each
scenario pins a different facet of the same generated guidance, so earlier
facets are assertion-subsets of broader ones (`ships without any addon
enabled` ⊂ delivery e2e; `ships without any addon enabled` and `binds
personas` ⊂ phase ladder and topic state). Each subset scenario still fails
independently in RED on its own phrases and pins a contract facet no other
scenario pins — kept deliberately as one-behavior-per-scenario.

No long scenarios (max 8 steps; threshold 14) — no Split-or-keep entries. No
removed+added pairs.

## Test suite result

```
./run-tests.sh (full, permanent corpus): 354 scenarios (354 passed); exit 0
GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh: 35 steps (35 passed); exit 0
GIVN_FEATURES=givn/changes/add-ideation-skill/specs ./run-tests.sh --e2e: 1 scenario (1 passed); exit 0
cargo test --lib: 241 passed (pairing test all_command_templates_are_embedded included)
cargo fmt + cargo clippy --workspace --all-targets -- -D warnings: clean
```

## README impact decision

README-IMPACT: none

The README does not enumerate skills (it points to `docs/reference.md`); that
registry was updated with the `givn-ideation` row as part of this review's
ubiquitous-language pass. The README itself needed no section refresh.

## Sign-off checklist

- [x] Fabrication audit: CLEAN (0 empty bodies, commits verified, single e2e
      implementation, tag integrity, design conformance).
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing (exit 101).
- [x] `verify.command` and `verify.e2e_command` both exit 0.
- [x] Coverage measured (instrumented llvm-cov, merged, fresh) with a
      non-zero exercised production path.
- [x] Every coverage gap classified (none in buckets 1–3).
- [x] No `@wip` tags remain (lint exit 0); no implementation detail in the
      spec.
- [x] Canonical E2E policy applied: one normalized action, one `@e2e`
      scenario, variants are regular scenarios.
- [x] The e2e scenario asserts through the real interface (CLI subprocess,
      stdout + generated files); no browser surface exists (N/A).
- [x] Local runnability: the capability needs no server/browser/database/twin;
      the CLI subprocess harness is the full stack.
- [x] `verify.e2e_command` ≠ `verify.command`; scenario count strictly
      smaller (1 < 6, and 1 < 354 at full scope).
- [x] Implementation matches design.md; the single evolution went through
      design amendment + re-review.
- [x] Interaction coverage verified end-to-end (inventory ↔ matrix ↔ scenario
      ↔ steps).
- [x] Ubiquitous language conformance: complete (Domain terms recorded;
      reference.md registry updated).
- [x] No invented classification anywhere in this report.

REVIEW: PASS