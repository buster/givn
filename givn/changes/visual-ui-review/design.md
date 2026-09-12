# Design: visual-ui-review

## Overview

Givn gains a visual review discipline for consumer interfaces, in three
layers:

1. **Canonical policy** (`assets/instructions/specs.md`): for a visual
   interface, every `@e2e` scenario commits a final-state screenshot per
   viewport (a transcript for a terminal), and the mechanical visual baseline
   is asserted at every interaction.
2. **Authoring contract** (`assets/instructions/design.md`, the design
   template, `givn-design-review`, `givn-steps`): a change that introduces or
   visibly changes a screen records a style-level `## Visual Design Contract`
   with viewports, a design-system home, and capture mechanics. Design-review
   cannot pass without an approved contract.
3. **Review gate** (`assets/instructions/review.md`, `givn-review`,
   `src/integrity_hook.rs`): a change whose design declares a visual
   interface must carry a `VISUAL-REVIEW: PASS` verdict and complete evidence,
   or `givn check review` fails.

The contract is style-level, never pixel-level. Improvements that stay inside
the contract update the contract; they are not review failures.

## Use-Case Traceability

> Owning use-case IDs: `author-change`, `verify-change`
> Ideation source: none
> Confirmed Persona IDs: none

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| author-change rule: generated guidance is synchronized from canonical source assets | All policy changes are made in `assets/`; generated skills are produced by `givn skills sync` | visual-design-contract: "Synchronized guidance defines the contract approval and capture mechanics" |
| author-change goal: create a valid change and the guidance to carry it | The design contract is authored guidance plus a machine-readable section | visual-design-contract: "Author requests the visual design contract guidance" |
| verify-change goal: a change is complete and free of unresolved findings | The visual verdict and evidence completeness join the review gate | visual-review-gate: "A visual change with complete evidence passes review" |
| verify-change rule: an invalid change cannot pass the verification gate | `check_visual_review` blocks a declared visual interface without a verdict or evidence | visual-review-gate: "A visual change with complete evidence passes review" |
| Persona impact: none confirmed | Persona vision review is an opt-in lens driven by confirmed Personas; with none, the fixed technical lenses still run | review guidance scenario |

## Technology Decisions

No new language, framework, library, or runtime is introduced.

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | unchanged; `cargo check` / `cargo clippy` as configured |
| `gherkin` crate | existing workspace dependency | reused to parse delta features for `@e2e` scenario titles |
| cucumber-rs (`tests/features_runner.rs`) | existing | reused for the new scenarios; no new runner |

The visual gate is implemented with the standard library and existing
workspace crates only. No image processing, no browser automation, and no new
dependency is added to givn itself: the screenshots are produced by each
project's own e2e driver, and givn only checks that the declared evidence
exists.

## Step Definitions

| Capability | Step definition file | Notes |
|---|---|---|
| prose-consolidation | `tests/steps/prose_consolidation_steps.rs` | modified scenario reuses the existing `givn instructions` and `stdout contains` steps; no new definitions |
| visual-design-contract | `tests/steps/visual_design_contract_steps.rs` | new step for the ejected design template content |
| visual-design-contract | `tests/steps/prose_consolidation_steps.rs` | reused generated-guidance and instruction-output steps |
| visual-review-gate | `tests/steps/visual_review_gate_steps.rs` | new fixture steps for the contract section, delta `@e2e` scenarios, evidence files, transcripts, and review verdicts |

Test runner command (from `verify.command` in `givn/config.yaml`):

```
./run-tests.sh
```

Single-scenario run command:

```
GIVN_FEATURES=givn/changes/visual-ui-review/specs cargo test --test features_runner -- --name "<scenario title>"
```

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| `VisualContract` | `struct` | Parsed `## Visual Design Contract` section: interface kind (`web` / `terminal` / `none`) and the declared viewport names |
| `parse_visual_contract` | `fn` | Extracts the contract from `design.md` after removing fenced code blocks; returns `None` when the section is absent or declares `none` (exempt) |
| `scenario_slug` | `fn` | Deterministic scenario-title slug used by both the capture helper guidance and the evidence check |
| `collect_e2e_scenario_titles` | `fn` | Parses the change's delta features and returns every `@e2e` scenario title that is not `@givn.removed` |
| `check_visual_review` | `fn` | The gate: verdict marker plus evidence completeness; called from `run_integrity_hook` |
| `missing_visual_evidence` | `fn` | Builds the finding message naming scenario title, viewport, and expected path |

## Strict Mode (mandatory)

Unchanged from the existing suite:

```
tests/features_runner.rs uses cucumber-rs `.fail_on_skipped()`; undefined
steps fail the runner.
```

Not-implemented stub for every new step:

```rust
unimplemented!() // or todo!()
```

No new step body may be empty; `givn lint` and the review fabrication audit
both scan for empty bodies.

## Local Runnability & Digital Twins (mandatory)

**Local run command** — the single command that runs the whole verification
surface:

```
./run-tests.sh
```

**Isolated network:** not applicable. Givn is a CLI; scenarios spawn the real
`givn` binary inside temporary project roots and assert on exit status and
stdout/stderr. No server, database, queue, or network service is involved.

**Digital twin per external/third-party dependency:** none. This system talks
to no external service.

**Anticipated interface obstacles and their fix:** none. The real interface
is the `givn` subprocess, exercised directly; no browser or network boundary
is involved.

## E2E Smoke Test Infrastructure

**Interface type:** CLI. The `givn` binary is the real consumer interface.

**E2E runner command** (`verify.e2e_command`):

```
./run-tests.sh --e2e
```

**E2E step location:** `tests/steps/visual_review_gate_steps.rs`, separate
from unit/integration step files; one file for the new capability.

**Local test infrastructure:** a temporary project root plus the built
`givn` binary. No browser, server, port, database, or container.

**E2E framework choice and justification:** cucumber-rs already drives the
real binary for every existing `@e2e` scenario; it matches the CLI interface
and needs no new infrastructure.

**Interaction Coverage Matrix:**

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| request canonical E2E guidance (prose-consolidation) | Canonical E2E policy is emitted by specs instructions | CLI | `givn` subprocess: `givn instructions specs --change <id>`, assert stdout |
| request the design guidance for a visual interface | Author requests the visual design contract guidance | CLI | `givn` subprocess: `givn instructions design --change <id>`, assert stdout |
| read the synchronized design and step guidance | Synchronized guidance defines the contract approval and capture mechanics | CLI | `givn` subprocess: `givn skills sync`, read generated skill files |
| run the review check on a visual change | A visual change with complete evidence passes review | CLI | `givn` subprocess: `givn check review --change <id>`, assert exit status |
| read the visual review guidance | Review guidance requires the visual verdict and persona lenses | CLI | `givn` subprocess: `givn instructions review --change <id>`, assert stdout |

**Real-interface assertion rule:** every `@e2e` scenario asserts on the
`givn` subprocess's exit status, stdout, stderr, or generated files. No
scenario calls library functions directly.

**Real-interface driving rule:** the step definitions invoke the real binary
through the existing harness. No in-process substitute is used for an
`@e2e` scenario.

**E2E scope:** one `@e2e` scenario per distinct consumer action, per the
canonical policy in `givn instructions specs`. Variants (missing verdict,
incomplete evidence, exemption, unverified verdict, terminal transcript) are
regular scenarios.

## Architecture Impact

| Module / asset | Change |
|---|---|
| `assets/instructions/specs.md` | New visual evidence and baseline subsection in the canonical E2E policy; owns the viewport classes (Samsung S24 class, Full HD) and the per-viewport evidence rule |
| `assets/instructions/design.md` | New `## Visual Design Contract` requirements, the exact default viewport dimensions (360x780, 1920x1080), design-system home, improvement rule, capture mechanism |
| `assets/instructions/design-review.md` | New mandatory visual contract branch: propose rendered options when none exists, require user approval before sign-off |
| `assets/instructions/review.md` | New mandatory `## Visual review` block for a declared visual interface |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | Grilling branch and hardening step for the contract |
| `assets/skills/givn-steps/SKILL.md.tmpl` | Capture mechanics: freeze animations, wait for fonts/network, deterministic paths, evidence split |
| `assets/skills/givn-review/SKILL.md.tmpl` | Visual review block with evidence completeness and persona lenses |
| `assets/templates/design.md` | `## Visual Design Contract` scaffold section |
| `assets/templates/review.md` | `## Visual Review` scaffold section |
| `src/integrity_hook.rs` | `check_visual_review` and helpers, wired into `run_integrity_hook` |
| `tests/steps/visual_review_gate_steps.rs` | New fixture steps |
| `docs/arc42/12-glossary.md` | Visual interface, contract, design system, evidence, baseline, verdict, style tile terms and invariants |
| `docs/arc42/05-building-block-view.md`, `06-runtime-view.md`, `08-crosscutting-concepts.md` | The integrity hook, the review flow, and the visual review discipline |
| `docs/arc42/09-architecture-decisions.md` | ADR-028 amendment: the visual review gate joins the non-deactivatable mandatory-check boundary |
| `docs/arc42/10-quality-requirements.md`, `11-risks-and-technical-debt.md` | Q-85 to Q-87, R-56 to R-59, D-18 |

The contract section format the gate parses (inside the section, before the
prose):

```
## Visual Design Contract

- Interface: web
- Viewports: mobile (360x780), desktop (1920x1080)
- Design system: docs/design/design-system.md
```

For a terminal interface the section declares `- Interface: terminal`.
`- Interface: none` or an absent section means the change has no visual
contract and the gate is exempt. The section's remaining prose carries
tokens, component primitives, states, and motion rules.

Evidence path convention (fixed, not parsed from the contract):

- Web UI: `givn/changes/<id>/evidence/visual/<scenario-slug>/<viewport>.png`
- Terminal: `givn/changes/<id>/evidence/visual/<scenario-slug>/transcript.txt`
- Intermediate captures (not review evidence): `reports/visual/...`

`scenario_slug` rule: lowercase ASCII; every run of non-alphanumeric
characters becomes one `-`; leading/trailing `-` removed. Example:
"Consumer opens the dashboard" → `consumer-opens-the-dashboard`.

### Gate semantics (fail-closed)

`parse_visual_contract` scans `design.md` line-based but first removes fenced
code blocks, so an example contract inside a fence never triggers the gate.
This change's own `design.md` contains such an example and MUST stay exempt;
a unit test asserts that.

The check order is fixed:

1. No section, or `Interface: none` → exempt.
2. `Interface: web` requires a non-empty `Viewports:` list; a web contract
   without viewports fails. `Interface: terminal` takes no viewports.
3. `review.md` must exist and carry exactly one verdict:
   `VISUAL-REVIEW: PASS`; `VISUAL-REVIEW: UNVERIFIED` fails with a message
   naming the human sign-off; both markers present fails as ambiguous;
   neither fails as missing.
4. Every delta `@e2e` scenario that is not `@givn.removed` must have one
   non-empty evidence file per viewport (web) or one non-empty transcript
   (terminal). A zero-byte file counts as missing.

Zero `@e2e` scenarios is not a visual-gate failure: the canonical E2E policy
already requires one per capability, and the fabrication audit owns that
finding.

The test fixture for a passing visual review writes `README-IMPACT: none`
alongside the verdict, because the readme addon's integrity check requires
it; the fixture's delta scenario uses steps that shape-match no permanent
scenario, so no overlap disposition is needed.

"Committed" evidence is a process obligation: the machine check verifies the
files are present in the change tree at review time, not their Git tracking
(non-Git fixtures must remain testable).

## Data Model Changes

None. No schema or struct in the product data model changes. The new file
conventions (contract lines, evidence paths, verdict markers) are described
under Architecture Impact.

## Open Technical Questions

None. The contract format, evidence path, slug rule, and gate ordering are
fixed above; the design-review grilling may still challenge them.
