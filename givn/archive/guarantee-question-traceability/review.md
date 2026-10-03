# Review: guarantee-question-traceability

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — the delta has no `@e2e` scenario and none was removed. |
| Empty/no-op step bodies | 0 found in `tests/steps/instruction_integrity_steps.rs`. |
| Checked tasks have commits touching production | Clean — setup `dc0e7c82` plus five scenario commits (`748c9243`, `f8af7398`, `67ceaee0`, `b32494bc`, `c5e3dc8c`). |
| Promised components exist | Clean — the qualifier rule, the disposition rule, the ledger contract, the review criterion, and all four template shapes exist. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario. |
| `verify.e2e_command` isolation | `./run-tests.sh` vs `./run-tests.sh --e2e`; counts 528 vs 88 — strictly fewer. |
| Implementation vs design.md | Matches; the templates scenario is a post-edit verification whose RED was reproduced against the setup revision, recorded in tasks.md. |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| ideation-skill: synchronize skills (fragment; no inventory) | yes | `A consumer synchronizes skills and every agent target receives the ideation facilitation skill` | Real `givn skills sync` subprocess |
| instruction-integrity: request artifact instructions (fragment; no inventory) | yes | `An agent requests machine-readable artifact guidance` | Real `givn instructions specs --json` subprocess |

No new `@e2e` scenario is added: the actions are already proven end-to-end and
the new scenarios are guidance-content variants. No browser UI exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Qualifier rule | 08 §8.23 | Yes | Open-Question Qualifier | ideation task | ideation skill bullet | Yes |
| Disposition rule | 08 §8.23 | Yes | Fail-Closed Question Disposition | proposal task | proposal instruction + command + template | Yes |
| Ledger contract | 08 §8.23 | Yes | Decision Ledger | ledger task | design instruction + template + design-review surfaces | Yes |
| Review open-prerequisite criterion | 08 §8.23 | Yes | Open-Question Qualifier | review task | `assets/instructions/review.md` | Yes |
| R-70 mitigation | 11 table | Yes | Risks | (covered) | qualifier + disposition + ledger | Yes |
| Glossary terms | 12 glossary | Yes | (named in design) | (covered) | two glossary rows | Yes |
| `ADR action: NONE` route | 09 (unchanged) | Yes | (Architecture Impact) | (covered) | arc42.md block | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Open-question qualifier` and `Decision ledger` plus the
existing `Use case` and `Question record` terms. Specs, design, and assets
reuse those terms: the delta scenarios say "(open: Q<N>)", "disposition",
"decisions ledger", and "Decides D"; the guidance uses the same vocabulary.
No glossary term is reused inconsistently and no undefined term appears in the
spec. Clean.

## Visual review

The change declares no visual interface: it adds guidance and template text,
not a rendered screen. Opposite-classification test: the guidance is plain
Markdown read in a terminal; nothing renders HTML, CSS, or JavaScript in a
browser, so no browser driver applies. No `@e2e` scenario in this delta
requires a screenshot or transcript. The visual block is N/A.

## Overlap dispositions

No non-contradicting shape match involves this change, so the table is empty.
The deterministic scan reports 10 assertion-subset warnings between delta and
permanent scenarios; all are structural overlaps on generic shared
assertions.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`The proposal design and design-review templates carry the ledger shapes` at
8 steps). No split-or-keep decisions are required.

## Removed+added pairs

None. The delta adds 5 scenarios and modifies none; no scenario is removed.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- The qualifier rule: asserted by the ideation scenario.
- The disposition rule and question IDs: asserted by the proposal scenario.
- The ledger contract: asserted by the design/design-review scenario.
- The review criterion: asserted by the review scenario.
- The template shapes: asserted by the eject scenario.

No dead code introduced; no missing coverage; no hard-to-test gap. The
disposition "block" is prose because the proposal declares runtime
enforcement out of scope; the instruction-integrity scenario asserts the
rule text.

## Use-case and Persona conformance

- The qualifier and ledger stay authoring contracts; no runtime gate is added.
- Support-fragment capabilities keep their fragment ownership.
- Confirmed Personas: `agent--technical-role` (guidance, templates) and
  `givn-maintainer--owner` (rule corpus). No Persona was invented or
  promoted; Gherkin actors remain the author, maintainer, and reviewer.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 528 passed, e2e
      88 passed; delta 5 passed).
- [x] Coverage classified from inspection; no gap remains.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No redundant unit tests added.
- [x] No `@wip` tags remain in the delta; no implementation-layer detail in
      the spec.
- [x] Canonical E2E policy read; the new scenarios are regular variants.
- [x] Every E2E scenario keeps its real-interface primary assertion.
- [x] Local run command starts the full stack: `./run-tests.sh`; no external
      dependency or digital twin exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; e2e scope is
      strictly smaller (88 vs 528).
- [x] Implementation matches design.md.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
