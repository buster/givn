# Review: use-case-model-scoping

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — the delta has no `@e2e` scenario and none was removed; the existing `@e2e` scenarios are untouched. |
| Empty/no-op step bodies | 0 found. `grep -n "unimplemented!\\|todo!()"` over both touched step files returns no matches. |
| Checked tasks have commits touching production | Clean — setup `fab75c6e` plus seven scenario commits (`f5f0abc1`, `31b6ceb8`, `9e67275e`, `208f8e43`, `a8486146`, `2f3e731c`, `d67d88d6`), each touching documents, assets, the template, or real step assertions. |
| Promised components exist | Clean — `docs/use-case-model.md`, the template guidance, the eight rules, the ten criteria, the design-review pointer, and the specs placement test all exist. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario. |
| `verify.e2e_command` isolation | `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e`. Counts: 507 regular vs 88 e2e — strictly fewer. |
| Implementation vs design.md | Matches; three asserted phrases needed rewrapping onto single lines during GREEN (placement test, the two criteria tie-ins, the handoff phrase), which is wording layout, not a decision change. |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| canonical-usecase-contract: inspect use-case tree | yes | `Use-case tree reports explicit use-case ownership` | Real `givn spec tree` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | yes | `A consumer synchronizes skills and every agent target receives the ideation facilitation skill` | Real `givn skills sync` subprocess |
| instruction-integrity: request artifact instructions (fragment; no inventory) | yes | `An agent requests machine-readable artifact guidance` | Real `givn instructions specs --json` subprocess |

No new `@e2e` scenario is added: no new consumer action exists, and the new
scenarios are guidance-content and document variants of the already-proven
actions. No browser UI exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Use-case model discipline | 08 §8.21 | Yes | Canonical Definition and Rules, Durable Document | canonical scenario tasks | `docs/use-case-model.md` + generated guidance | Yes |
| Definition and eight rules in the generated guidance | 08 §8.21 | Yes | Canonical Definition and Rules | ideation scenario tasks | `assets/skills/givn-ideation/SKILL.md.tmpl` | Yes |
| Ten rejection criteria | 08 §8.21 | Yes | Review Rejection Criteria | review scenario task | `assets/instructions/review.md` | Yes |
| Capability placement test | 08 §8.21 | Yes | Specs and Spec-Skill Guidance | specs scenario task | `assets/instructions/specs.md` + `assets/skills/givn-spec/SKILL.md.tmpl` | Yes |
| Template guidance | 08 §8.21 | Yes | Template Guidance | template scenario task | `src/gherkin/usecase_md.rs` | Yes |
| R-69 prose-only enforcement risk | 11 table | Yes | Risks | (covered by the criteria tasks) | criteria + mechanical tie-ins | Yes |
| `Use case`, `Primary actor`, `Summary use case` terms | 12 glossary | Yes | Canonical Definition and Rules | canonical scenario task | glossary rows | Yes |
| Chapter 10 unaffected | 10 (unchanged) | No | (not named) | (no task) | guidance-content scenarios only | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Use case` (now with the canonical definition), `Primary
actor`, `Summary use case`, and the existing `Capability` and `Fragment`
entries. Specs, design, documents, and code reuse those terms: the delta
scenarios say "primary actor", "placement test", "summary", and "leaf"; the
generated guidance uses the same vocabulary. No glossary term is reused
inconsistently and no undefined term appears in the spec. Clean.

## Visual review

The change declares no visual interface: it adds guidance text, one template
string, and one Markdown document, not a rendered screen. Opposite-
classification test: the generated guidance and the document are plain
Markdown read in a terminal; nothing renders HTML, CSS, or JavaScript in a
browser, so no browser driver applies. No `@e2e` scenario in this delta
requires a screenshot or transcript. The visual block is N/A.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `The ideation guidance carries the term schema and the collision procedure` | `The ideation guidance carries the canonical definition and the eight scoping rules` | boundary |

The pair shares the `givn skills sync` driver and a "guidance contains"
assertion shape but asserts two different content families (term records vs
use-case scoping), so it sits on a behavior boundary and is not a duplicate.
The deterministic scan additionally reports 12 assertion-subset warnings
between delta and permanent scenarios; all are structural overlaps on generic
shared assertions.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`The ideation guidance carries the canonical definition and the eight scoping
rules` at 14 steps). No split-or-keep decisions are required.

## Removed+added pairs

None. The delta adds 7 scenarios and modifies none; no scenario is removed.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- The durable document: asserted by the canonical scenario (definition, rules,
  criteria, both worked examples, references).
- The template guidance: asserted by the template scenario.
- The generated ideation guidance: asserted by the two ideation scenarios.
- The review, design-review, and specs guidance: asserted by the three
  instruction-integrity scenarios.
- The workflow and reference pointers: asserted by direct reads.

No dead code introduced; no missing coverage; no hard-to-test gap. The one
production change is a template string with no parser or lint code path, so
the existing lint suite is the evidence for the unchanged-behavior criterion.

## Use-case and Persona conformance

- `make-usecases-canonical` rules: a capability has exactly one owner and the
  corpus stays inspectable when a narrative is invalid; both are preserved
  (no parser or lint change).
- Support-fragment capabilities keep their fragment ownership.
- Confirmed Personas: `agent--technical-role` (generated guidance, template)
  and `givn-maintainer--owner` (model document, criteria). No Persona was
  invented or promoted; Gherkin actors remain the maintainer, reviewer,
  author, and agent.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 507 passed, e2e
      88 passed; delta 7 passed); the archive receipt will carry the measured
      coverage.
- [x] Coverage classified from inspection; no gap remains.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No redundant unit tests added.
- [x] No `@wip` tags remain in the delta; no implementation-layer detail in
      the spec.
- [x] Canonical E2E policy read; the new scenarios are regular variants of
      already-proven actions.
- [x] Every E2E scenario keeps its real-interface primary assertion.
- [x] Local run command starts the full stack: `./run-tests.sh`; no external
      dependency or digital twin exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; e2e scope is
      strictly smaller (88 vs 507).
- [x] Implementation matches design.md; the single-string template change is
      the only production edit.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
