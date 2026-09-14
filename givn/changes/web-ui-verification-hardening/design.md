# Design: web-ui-verification-hardening

## Overview

The change hardens three existing capabilities by making the browser/HTTP
escape hatch fail-closed. Two layers:

1. **Canonical guidance** (`assets/instructions/*.md`, the matching
   `assets/skills/*/SKILL.md.tmpl`, and `assets/templates/*.md`): the emitted
   instructions and synchronized skills state the Web UI classification, the
   default browser driver, the rendered-state assertion, the per-state
   evidence convention, the design-system breakpoints and accessibility
   checks, the persona disposition, and the operator question that precedes
   any deferral of a mandatory check.
2. **Decidable gate checks** (`src/integrity_hook.rs`): the visual gate gains
   state-evidence and persona-disposition checks for a declared visual
   interface, and the integrity hook gains a deferral check that requires an
   `OPERATOR-DECISION:` record next to any `DEFERRAL:` in `review.md`.

No new dependency, runtime, or browser framework is added to givn. The
browser driver named by the guidance belongs to the consuming project.

## Use-Case Traceability

> Owning use-case IDs: `author-change`, `verify-change`
> Ideation source: none
> Confirmed Persona IDs: none — both use cases declare `Personas: none`; the
> operator confirmed no personas for this repository.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| author-change rule: generated guidance is synchronized from canonical source assets | All guidance edits live in `assets/`; `givn skills sync` regenerates the agent trees | visual-design-contract: "Synchronized authoring guidance carries the classification, driver, and persona rules" |
| author-change goal: create a valid change and the guidance to carry it | The design, specs, proposal, and design-review guidance gain the fail-closed rules | prose-consolidation: "Canonical policy classifies a browser-rendered page as a Web UI"; visual-design-contract scenarios |
| verify-change goal: a change is complete and free of unresolved findings | The visual gate checks state evidence and persona disposition; the integrity hook checks deferrals | visual-review-gate: "A web contract with missing state evidence fails review", "A visual change without a persona disposition fails review", "A deferral without an operator decision fails review" |
| verify-change rule: an invalid change cannot pass the verification gate | Every new check bails non-zero with a named finding | the same three failure scenarios |
| Persona impact: none confirmed | Persona vision review remains an opt-in lens; the new persona-disposition check requires a confirmed persona marker for a declared visual interface | visual-review-gate: "A visual change with a confirmed persona disposition passes review" |

## Technology Decisions

No new language, framework, library, or runtime. Version freshness does not
apply because nothing version-bearing is introduced.

| Technology | Version | Resolution |
|---|---|---|
| Rust | existing workspace toolchain | unchanged; `cargo check` / `cargo clippy` as configured |
| `gherkin` crate | existing workspace dependency | reused to parse delta features for `@e2e` scenario titles |
| cucumber-rs (`tests/features_runner.rs`) | existing | reused for the new scenarios; no new runner |

## Step Definitions

| Capability | Step definition file | Notes |
|---|---|---|
| prose-consolidation | `tests/steps/prose_consolidation_steps.rs` | no new definitions; the new scenarios reuse the existing `givn instructions` and `stdout contains` steps |
| visual-design-contract | `tests/steps/visual_design_contract_steps.rs` | new then-steps for the ejected proposal and design-review templates; the ejected review template reuses the existing step in `tests/steps/arc42_review_alignment_steps.rs` |
| visual-design-contract | `tests/steps/prose_consolidation_steps.rs` | reused synchronized-guidance steps |
| visual-review-gate | `tests/steps/visual_review_gate_steps.rs` | new fixture steps: declared UI states, final-state screenshots for both viewports, state screenshots, empty state screenshots, persona disposition (including an empty value), and deferral records. The shared `review_md_contains` base gains `PERSONAS: fixture-persona` so the existing positive scenarios stay green under the new persona check |

Test runner command (from `verify.command` in `givn/commands.yaml`):

```
./run-tests.sh
```

Single-scenario run command:

```
GIVN_FEATURES=givn/changes/web-ui-verification-hardening/specs cargo test --test features_runner -- --name "<scenario title>"
```

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| `VisualContract` | `struct` | Existing parsed contract; gains `states: Vec<String>` |
| `parse_visual_contract` | `fn` | Existing parser; now reads the optional `- States:` line |
| `check_persona_disposition` | `fn` | New: a declared visual interface requires a `PERSONAS:` line in `review.md`; `none` is rejected |
| `check_deferrals` | `fn` | New: a `DEFERRAL:` line in `review.md` requires an `OPERATOR-DECISION:` line |
| `check_visual_evidence` | `fn` | Existing; now also requires one non-empty `<viewport>--<state>.png` per declared state and viewport |
| `persona_disposition` | `fn` | New: extracts the `PERSONAS:` value from `review.md` |

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

**E2E step location:** the existing `tests/steps/` modules; this change adds
no new `@e2e` scenario, so no new e2e step file is introduced. The fixture
steps live in `tests/steps/visual_review_gate_steps.rs`.

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

**E2E scope:** the delta adds no new consumer action, so the existing
`@e2e` scenario set is unchanged; the new behaviour is asserted by regular
scenarios that drive the same real CLI interface.

## Guidance content decisions

The exact wording is owned by `assets/instructions/*.md` and mirrored in
`assets/skills/*/SKILL.md.tmpl`. The load-bearing phrases the delta scenarios
assert:

- `renders HTML, CSS, or JavaScript in a browser`; `even when it needs only a
  page request`; `no controls`; `an HTTP client is not a valid driver`.
- `rendered state`; `changes between two observations`; `static heading`.
- `declares UI states`; `<viewport>--<state>.png`; `empty, loading, or error`.
- `never deferred`; `operator question`; `recorded decision`.
- `default browser driver`; `Playwright`; `does not pin`.
- `Breakpoints`; `contrast`; `keyboard focus`; `how each is verified`.
- `is a Web UI row`; `needs a browser driver`.
- `opposite classification`; `rendered in a browser`; `must say why`;
  `rendered page is a Web UI`; `restates`.
- `per persona`; `relevance and evidence`; `operator confirmation`;
  `disposition for every confirmed Persona`; `at least one confirmed Persona`;
  `record the reason`.
- `deferral`; `operator decision`; `fails the review`.
- `state screenshots the review saw`; `each declared UI state`.
- `user-visible`; `CLI text`.
- Template markers: `States:`; `PERSONAS:`; `DEFERRAL:`;
  `OPERATOR-DECISION:`.

## Gate semantics (fail-closed)

Order inside `check_visual_review`:

1. No section, or `Interface: none` → exempt.
2. `Interface: web` without viewports → fail (existing).
3. `review.md` must exist and carry exactly one verdict (existing).
4. A declared visual interface (web or terminal) requires a `PERSONAS:` line
   in `review.md`; an empty value or `none` fails with a message naming the
   user-visible surface.
5. Every delta `@e2e` scenario needs its final-state evidence (existing).
6. A web contract that declares `- States:` needs, per declared state and
   viewport, one non-empty `<viewport>--<state>.png` under a known
   `@e2e` scenario slug directory. Missing state evidence fails with the
   state and viewport named.

`check_deferrals` runs for every change in `run_integrity_hook`: if
`review.md` carries more `DEFERRAL:` lines than `OPERATOR-DECISION:` lines,
the check fails with the expected marker. Both marker checks strip fenced
code blocks first, so the scaffolded examples in the review template never
satisfy or trigger a check by themselves; a compliant change omits both
lines when nothing is deferred.

The state-evidence check is per declared state and viewport across the
change, not per scenario: which scenario traverses which state is a design
decision the guidance states, while the gate proves the state was captured
at all. The per-scenario intent lives in the guidance and the review; the
gate boundary is recorded in `assumptions.md` (A5).

The existing fenced-example unit test in `src/integrity_hook.rs` points at
the archived `visual-ui-review` design and is a no-op; it is retargeted to an
inline fixture so it exercises the parser again.

## Compatibility constraints

- The rewritten instructions must preserve every phrase asserted by the
  permanent corpus, including `style-level`, `not pixel`, `docs/design/`,
  `design tokens`, `component states`, `rendered reference`, `Samsung S24`,
  `360x780`, `Full HD`, `1920x1080`, `improvement`, `updates the contract`,
  `evidence/visual`, `final-state`, `animations are frozen`, `reports/`,
  `VISUAL-REVIEW: PASS`, `VISUAL-REVIEW: UNVERIFIED`, `human sign-off`,
  `transcript`, `API-only capabilities are exempt`, `no pixel-level`, and the
  existing E2E-policy phrases asserted in `prose-consolidation.feature`.
- The shared `review_md_contains` fixture gains `PERSONAS: fixture-persona`
  in the same task as the persona check, so the four existing visual
  scenarios that expect exit 0 or a specific evidence error keep their
  outcome.
- The positive gate scenarios (complete state evidence, confirmed persona,
  operator decision) are paired with their negative counterparts in one
  task; the negative scenario is the RED driver, the positive proves the
  complete artifact passes.

## Architecture Impact

| Module / asset | Change |
|---|---|
| `assets/instructions/specs.md` | Web UI classification, rendered-state assertion, per-state evidence, no-silent-deferral rule |
| `assets/instructions/design.md` | Fail-closed interface classification, `States:` contract line, Playwright default, breakpoint and accessibility bullets, persona boundary, operator-question rule |
| `assets/instructions/design-review.md` | Opposite-classification branch, persona disposition, deferral branch |
| `assets/instructions/review.md` | Opposite-classification check, deferral failure, persona disposition, state-screenshot naming |
| `assets/instructions/proposal.md` | Persona confirmation rule |
| `assets/templates/proposal.md` | `Confirmed Personas` line with reason and operator confirmation |
| `assets/templates/design.md` | `States:` line, breakpoint and accessibility bullets |
| `assets/templates/design-review.md` | Persona disposition columns (relevance, evidence) |
| `assets/templates/review.md` | `PERSONAS:`, `DEFERRAL:`, `OPERATOR-DECISION:` scaffold, state-screenshot mention |
| `assets/skills/givn-design/SKILL.md.tmpl` | Mirrors the design instruction changes |
| `assets/skills/givn-design-review/SKILL.md.tmpl` | Mirrors the design-review instruction changes |
| `assets/skills/givn-review/SKILL.md.tmpl` | Mirrors the review instruction changes |
| `assets/skills/givn-propose/SKILL.md.tmpl` | Mirrors the proposal persona rule |
| `assets/skills/givn-steps/SKILL.md.tmpl` | State capture convention `<viewport>--<state>.png` |
| `assets/skills/givn-tasks/SKILL.md.tmpl` | Matrix row check names the classification and state evidence |
| `src/integrity_hook.rs` | `states` parsing, persona disposition, deferral, state evidence |
| `tests/steps/visual_review_gate_steps.rs` | Fixture steps for states, persona, deferral |
| `tests/steps/visual_design_contract_steps.rs` | Ejected-template step for proposal, review, design-review |
| `docs/arc42/05-building-block-view.md`, `06-runtime-view.md`, `08-crosscutting-concepts.md` | Integrity hook and visual review discipline |
| `docs/arc42/09-architecture-decisions.md` | ADR-028 amendment: persona and deferral checks join the mandatory-check boundary |
| `docs/arc42/10-quality-requirements.md`, `11-risks-and-technical-debt.md` | Quality scenarios and risks for the new checks |
| `docs/arc42/12-glossary.md` | Terms: interface classification, UI state evidence, persona disposition, deferral decision |

## Data Model Changes

None. The parsed contract struct gains a field; no product data model changes.

## Open Technical Questions

None. The remaining interpretation choices (state evidence scope, persona
marker format, deferral marker format) are recorded in `assumptions.md` for
post-change revision.
