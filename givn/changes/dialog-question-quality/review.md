# Review: dialog-question-quality

## Fabrication audit

| Check | Result |
|---|---|
| @e2e tag integrity | Clean — no delta scenario carries or ever carried `@e2e`; no tag was removed. The change adds no `@e2e` scenario because every new scenario is a variant of an action already proven end-to-end. |
| Empty/no-op step bodies | 0 found. `grep -n "unimplemented!\|todo!()" tests/steps/instruction_integrity_steps.rs tests/steps/prose_consolidation_steps.rs` returns no matches. |
| Checked tasks have commits touching production | Clean — setup commit `b24db59a` plus eight scenario commits (`f6c1db11`, `17acb19e`, `2001fb6b`, `60e8cdc5`, `d2749d63`, `da258888`, `22a81c6c`, `fb13bf15`), each touching assets, `src/cli/instructions.rs`, or real step assertions. |
| Promised components exist | Clean — `assets/instructions/dialog.md`, the `src/cli/instructions.rs` fallback, the reshaped `assets/templates/design-review.md`, the eight pointer assets, `assets/addons/absolute-mode/policy.md`, the refreshed `AGENTS.md` block, and the new step definitions all exist. |
| Strict-mode proof | Present in `tasks.md` Setup with non-zero-exit evidence. |
| Downgraded `@e2e` scenarios | None — the delta has no `@e2e` scenario; existing `@e2e` scenarios are untouched. |
| `verify.e2e_command` isolation | `verify.command` is `./run-tests.sh`; `verify.e2e_command` is `./run-tests.sh --e2e` (different strings). Counts: 491 regular vs 88 e2e — strictly fewer. |
| Implementation vs design.md | Matches. The fallback order was changed during design-review hardening and design.md was updated before implementation; during review the Step Definitions table was reformatted so its file column is a clean path (integrity-hook requirement) with no decision change. |
| Parallel e2e implementations | None found in the tree (tracked or untracked). |

## Interaction coverage

| Inventory entry | Design matrix row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| prose-consolidation: request canonical E2E guidance (`givn/specs/author-change/usecase.md`) | yes | `Canonical E2E policy is emitted by specs instructions` | Real `givn instructions specs` subprocess |
| prose-consolidation: inspect generated E2E guidance | yes | `Generated guidance points to the canonical E2E policy` | Real `givn skills sync` subprocess plus generated files |
| instruction-integrity: request artifact instructions (fragment; no inventory) | yes | `An agent requests machine-readable artifact guidance` (existing) | Real `givn instructions specs --json` subprocess |
| ideation-skill: synchronize skills (fragment; no inventory) | yes | `A consumer synchronizes skills and every agent target receives the ideation facilitation skill` (existing) | Real `givn skills sync` subprocess |
| install-extras: enable an addon (no inventory row; regular scenarios only) | n/a | n/a — no `@e2e` scenario for this capability | Real `givn addons enable` subprocess in regular scenarios |

The new dialog-retrieval and dialog-pointer scenarios are variants of the
already-proven actions (request artifact instructions, inspect generated
guidance), so they are regular scenarios and the one-`@e2e`-per-action rule
holds. No browser UI exists; a browser driver would be the wrong mechanism.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| `givn instructions dialog` is an externally visible output | 03 §3.2 row | Yes | CLI Reachability | S1, S3, S4 | `src/cli/instructions.rs` fallback + `assets/instructions/dialog.md` | Yes |
| `cli::instructions` resolves non-artifact policy instructions | 05 §5.3 row | Yes | CLI Reachability | S1 | fallback branch before change resolution | Yes |
| Non-artifact fallback in the instruction flow | 06 §6.3 | Yes | CLI Reachability | S1, S3, S4 | same branch, error mapping keeps known-artifact error | Yes |
| Dialog policy is a normative authoring source; surfaces reference it | 08 §8.3a | Yes | The Canonical Policy Asset, Asset Responsibilities | S6, S7 | `assets/instructions/dialog.md` + eight pointer assets + `tests/steps/prose_consolidation_steps.rs` | Yes |
| Policy instruction JSON shape and lookup boundary | 08 §8.3b | Yes | CLI Reachability | S1, S3, S4 | empty `generates`/`requires` arrays; `unknown artifact` error | Yes |
| R-67 dialog-rule drift risk and mitigation | 11 table | Yes | Risks | S7 | pointer scenario + review asset scan | Yes |
| `Dialog policy` term | 12 glossary | Yes | Use-Case Traceability | S1, S2 | glossary row + policy asset | Yes |
| `Question record` term | 12 glossary | Yes | The Canonical Policy Asset | S2, S5 | glossary row + policy record shape + template block | Yes |
| Chapter 10 quality requirements unaffected | 10 (unchanged) | No | (not named) | (no task) | no quality attribute changes; functional scenarios only | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

The glossary holds `Dialog policy` and `Question record`. Specs, design, and
code reuse those terms: the delta scenarios say "dialog policy" and "record
shape"; `assets/instructions/dialog.md` defines the option treatment, the
cross-reference gloss, standalone answerability, and the rejected-alternative
rule with exactly the wording the specs assert. No glossary term is reused
inconsistently and no undefined term appears in the spec. Clean.

## Visual review

The change declares no visual interface: it adds CLI text output and guidance
text, not a rendered screen. Opposite-classification test: the instruction
output is plain terminal text; it renders no HTML, CSS, or JavaScript in a
browser, so it is not a Web UI and no browser driver applies. No `@e2e`
scenario in this delta requires a screenshot or transcript. The visual block
is N/A.

## Overlap dispositions

Every shape match involving this change (kind `shape_match`), with its
decision. The pair shares an error-message assertion shape; they sit on
different command boundaries (`givn check` vs `givn instructions`) and both
assert their own known-artifact error, so the pair is a variant, not a
duplicate.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `Consumer invokes givn check with an unknown artifact and gets a helpful error` | `An unresolvable instruction id keeps the known-artifact error` | variant |

The deterministic scan additionally reports 52 assertion-subset warnings
between delta and permanent scenarios. Every one is a structural overlap on a
generic shared assertion (`the command exits 0`, `stderr contains …`): the
permanent scenario's assertion sequence is a subsequence of the delta
scenario's, not a behavioral duplicate. The overlap gate does not require
disposition rows for subset findings; no scenario pair needs a duplicate,
split, or removal decision.

## Split-or-keep

No scenario in this delta exceeds the long-scenario threshold (longest:
`Generated interactive guidance points to the canonical dialog policy` at
11 steps). No split-or-keep decisions are required.

## Removed+added pairs

None. The delta has 7 added and 1 modified scenario; no scenario is removed,
so there is no removed+added pair to explain.

## Coverage classification

Review does not run an instrumented pass; the archive receipt carries the
measured coverage. Classified from inspection:

- `src/cli/instructions.rs` non-artifact fallback: covered by three scenarios
  (retrieval, no-change retrieval, unknown-id error). No uncovered region.
- `assets/instructions/dialog.md`: every rule is asserted by the retrieval
  and record-shape scenarios. No uncovered region.
- The eight pointer assets and `assets/templates/design-review.md`: asserted
  by the pointer and eject scenarios. No uncovered region.
- `assets/addons/absolute-mode/policy.md`: asserted by the install-extras
  scenario. No uncovered region.

No dead code introduced; no missing coverage; no hard-to-test gap. The only
new Rust code is the fallback branch, exercised by three scenarios.

## Use-case and Persona conformance

- `author-change` rule "generated guidance is synchronized from canonical
  source assets": the dialog policy is one asset, and every listed surface
  references it; the pointer scenario asserts the generated result.
- `author-change` rule "generated guidance may name only commands that work":
  `givn instructions dialog` is proven by the retrieval and no-change
  scenarios.
- Support-fragment capabilities keep their fragment ownership and carry no
  independent user goal or inventory.
- Confirmed Personas: `agent--technical-role` (consumes the CLI output and
  generated guidance) and `givn-maintainer--owner` (owns the rule corpus).
  No Persona was invented or promoted; Gherkin actors remain the Developer
  and the Authoring agent.
- No deferral of a mandatory check occurred.

## Sign-off checklist

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production code.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope (regular 491 passed,
      e2e 88 passed; delta 8 passed); the archive receipt will carry the
      measured coverage.
- [x] Coverage classified from inspection; no gap remains.
- [x] No dead code; no missing tests; no hard-to-test gap.
- [x] No redundant unit tests added.
- [x] No `@wip` tags remain in the delta; no implementation-layer detail in
      the spec.
- [x] Canonical E2E policy read from the resolved specs instruction; every
      normalized inventory action has one `@e2e` scenario; new scenarios are
      regular variants.
- [x] Every E2E scenario keeps its real-interface primary assertion (existing
      scenarios unchanged).
- [x] Local run command starts the full stack: `./run-tests.sh` and
      `./run-tests.sh --e2e`; no external dependency or digital twin exists.
- [x] `verify.e2e_command` is not identical to `verify.command`; e2e scope is
      strictly smaller (88 vs 491).
- [x] Implementation matches design.md; the review-time table reformat was
      formatting only, recorded above.
- [x] Interaction coverage verified with no mismatch.
- [x] No finding excused outside the three coverage buckets.

README-IMPACT: none

REVIEW: PASS
