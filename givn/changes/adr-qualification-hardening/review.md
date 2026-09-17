# Review: adr-qualification-hardening

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — the delta has no `@e2e` scenario and none was removed; the two existing `@e2e` CLI scenarios are untouched. |
| Empty/no-op step bodies | 0 found. `grep -c "unimplemented!\\|todo!()" tests/steps/adr_qualification_gate_steps.rs` returns 0. |
| Checked tasks have commits touching production | Clean — setup `97e419f0` plus six scenario commits (`4203f326`, `bd73138d`, `aa5a0e91`, `418f7c18`, `086d1ed6`, `2cf72220`), each touching assets or real step assertions. |
| Promised components exist | Clean — the falsification block, the design prompt, the verdict placement rule, the blank template, the cheaper-home challenge in all three surfaces, and the row-9 reword all exist. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario. |
| `verify.e2e_command` isolation | `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e`. Counts: 514 regular vs 88 e2e — strictly fewer. |
| Implementation vs design.md | Matches; three asserted phrases needed single-line rewrapping during GREEN (the two non-examples and the verdict placement phrase), which is wording layout, not a decision change. |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

Every new Then-step asserts a phrase that exists only after this change; the
row-9 scenario additionally asserts the absence of the removed normalizing
sentence, so it fails RED against the unedited asset.

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| adr-qualification-gate: request the Arc42 ADR guidance (fragment; no inventory) | yes | `An author requests the strict ADR guidance through the Givn CLI` | Real `givn instructions arc42-docs` subprocess |
| adr-qualification-gate: request the design-review guidance (fragment; no inventory) | yes | `A reviewer requests design-review guidance through the Givn CLI` | Real `givn instructions design-review` subprocess |

No new `@e2e` scenario is added: both actions are already proven end-to-end,
and the new scenarios are guidance-content variants. The fragment declares no
interactions of its own. No browser UI exists.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Falsification tests | 08 §8.15 | Yes | Falsification Tests | falsification task | `assets/addons/arc42-docs/instructions.md` | Yes |
| Verdict before drafting and blank template | 08 §8.15 | Yes | Verdict Before Drafting, Blank ADR Template | verdict and blank-template tasks | impact-assessment template + ADR template | Yes |
| Cheaper-home challenge | 08 §8.15 | Yes | Cheaper-Home Challenge | cheaper-home task | design-review instruction, command, and skill | Yes |
| Row-9 wording | 08 §8.15 | Yes | Row-9 Pressure Language | row-9 task | instructions.md paragraph | Yes |
| Design-template prompt | 08 §8.15 | Yes | Design Template Prompt | design-template task | `assets/templates/design.md` | Yes |
| R-23 mitigation | 11 table | Yes | Risks | falsification task | tests + challenge | Yes |
| `Falsification tests`, `Cheaper-home challenge` terms | 12 glossary | Yes | (named in design) | (covered) | glossary rows | Yes |
| Chapter 9 unaffected | 09 (unchanged) | No | (not named) | (no task) | the change's own choices fail single-change reversibility | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Falsification tests`, `Cheaper-home challenge`, and the
existing `ADR candidate`, `Mandatory qualification dimension`, and `ADR
routing` terms. Specs, design, and assets reuse those terms: the delta
scenarios say "falsification tests", "cheaper home", "verdict", and
"product-behavior exclusion"; the instruction and templates use the same
vocabulary. No glossary term is reused inconsistently and no undefined term
appears in the spec. Clean.

## Visual review

The change declares no visual interface: it adds guidance and template text,
not a rendered screen. Opposite-classification test: the Arc42 instruction and
templates are plain Markdown read in a terminal; nothing renders HTML, CSS, or
JavaScript in a browser, so no browser driver applies. No `@e2e` scenario in
this delta requires a screenshot or transcript. The visual block is N/A.

## Overlap dispositions

No shape match involves this change, so the table is empty. The deterministic
scan reports no subset warnings for the delta either.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`The guidance carries the three falsification tests` at 9 steps). No
split-or-keep decisions are required.

## Removed+added pairs

None. The delta adds 6 scenarios and modifies none; no scenario is removed.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- The falsification block: asserted by the falsification scenario.
- The design-template prompt: asserted by the design-template scenario.
- The verdict placement rule: asserted by the impact-assessment scenario.
- The blank ADR template: asserted by the blank-template scenario.
- The cheaper-home challenge in the instruction, command, and skill template:
  asserted per file by the cheaper-home scenario; the generated copies were
  refreshed with `givn skills sync` in the same commit.
- The row-9 reword: asserted by the chapter-selection scenario.

No dead code introduced; no missing coverage; no hard-to-test gap.

## Use-case and Persona conformance

- The qualification gate stays authoring guidance; the AND semantics, routing
  values, and verdict schema are unchanged.
- Support-fragment `adr-qualification-gate` keeps its fragment ownership.
- Confirmed Personas: `agent--technical-role` (guidance) and
  `givn-maintainer--owner` (rule corpus, ADR register). No Persona was
  invented or promoted; Gherkin actors remain the author, maintainer, and
  reviewer.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 514 passed, e2e
      88 passed; delta 6 passed); the archive receipt will carry the measured
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
      strictly smaller (88 vs 514).
- [x] Implementation matches design.md; the layout rewraps are recorded above.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
