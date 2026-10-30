# Tasks: strengthen-outline-guidance

Owning use case: `obtain-effective-guidance`
(`givn/specs/obtain-effective-guidance/usecase.md`). Supporting owners:
`assess-code-structure`, `verify-change`, and the reusable `fragments`
document. Personas: `agent--technical-role`, `developer--primary-user`.

Release note: Outline guidance now appears only in projects that enable its
addon, routes implementation to the shared step mechanics, and treats changed
code that cannot explain itself from its outline as a review finding.

The change is asset and test work. No `src/` module changes: the renderer
already resolves `{{#if addons.outline}}` from project configuration.

## Setup

- [x] Confirm the existing runner and strict mode; no new runner is installed.
      Evidence:
      ```text
      tests/features_runner.rs: GivnWorld::cucumber().fail_on_skipped()
      givn/commands.yaml: verify.command = ./run-tests.sh --non-e2e
      givn/commands.yaml: verify.e2e_command = ./run-tests.sh --e2e
      ```
      Strict-mode probe (undefined step):
      ```text
      $ GIVN_FEATURES=/tmp/strict-probe cargo test --locked --test features_runner -- --name '^undefined step$'
      Scenario: undefined step
        ✘  Given an undefined strict probe step
      Step doesn't match any function
      1 scenario (1 failed), 1 step (1 failed)
      exit status: 1
      ```
      Probe removed; the suite returned to its passing baseline.
      Probe removed; the suite returned to its passing baseline.
- [x] Confirm the step definition files named by design.md exist and stay the
      homes for this change:
      `tests/steps/serve_extended_rules_steps.rs`,
      `tests/steps/template_conditional_rendering_steps.rs` (+
      `template_conditional_rendering_support.rs`),
      `tests/steps/arc42_review_alignment_steps.rs`,
      `tests/steps/skill_rewrites_steps.rs`.
- [x] No new E2E action is added. The changed `@e2e` scenario keeps the
      existing `givn eject review:template` action, its real CLI driver, and
      its terminal transcript evidence.

## Batch A — outline guidance follows the addon

Design constraints for every scenario in this batch:

- `assets/instructions/steps.md`: wrap the entire REFACTOR outline paragraph
  from `When the `outline` addon is enabled` through
  `no blocking verdict, metric waiver, or forced reviewer context` in
  `{{#if addons.outline}}` / `{{/if}}`. Leave the following `Concrete cleanup`
  paragraph outside.
- `assets/instructions/review.md`: wrap the paragraph at lines 50-53
  (`When the outline addon is enabled, its report is advisory evidence...`)
  in `{{#if addons.outline}}` / `{{/if}}`. Keep the existing advisory block at
  line 529 for Batch B.
- `assets/templates/review.md`: change `records evidence and material
  mismatches without an outline verdict or metric waiver` to
  `records evidence and material mismatches`. The template is ejected raw, so
  it must contain no outline wording.
- Renderer: `outline` is already a known key in
  `src/skills/render.rs::KNOWN_ADDON_KEYS` and
  `src/cli/instructions.rs::addon_flags`. No code change.
- Fixtures: `common::make_init_root()` enables every addon;
  `support::set_addon` writes `givn/config.yaml`; `support::sync` runs
  `givn skills sync`.
- New bindings live in `tests/steps/template_conditional_rendering_steps.rs`;
  the template assertion binding lives in
  `tests/steps/arc42_review_alignment_steps.rs`.
- Assert addon-off by the absence of `outline` in both served instructions
  and in every generated skill; assert addon-on by distinctive phrases
  (`inspect its advisory report once per batch` for steps,
  `## Advisory outline evidence` for review).
- `docs/arc42/08-crosscutting-concepts.md` 8.26a already records the serving
  path from design-review hardening; commit it with this batch.

### Scenario: Outline guidance follows the opt-in addon

- [x] RED: both example steps were undefined; `Step doesn't match any function` for the enabled and disabled rows.
- [x] GREEN: `2 scenarios (2 passed)`, `8 steps (8 passed)` with the conditional wrappers and step bindings.
- [x] REFACTOR: Re-ran the same name filter; `2 scenarios (2 passed)`.
- [x] COMMIT: `8c1e999b` -- `fix(serve-extended-rules): outline guidance follows the addon` (Batch: addon-conditional-outline)

### Scenario: The review template includes the Arc42 conformance block (@e2e)

- [x] RED: `the template records evidence and material mismatches` failed on the retired-gate phrase.
- [x] GREEN: `1 scenario (1 passed)`, `7 steps (7 passed)` after the template sentence changed and the delta assertion bound.
- [x] REFACTOR: Re-ran the same name filter; `1 scenario (1 passed)`. Transcript written under `givn/changes/strengthen-outline-guidance/evidence/visual/the-review-template-includes-the-arc42-conformance-block/`.
- [x] COMMIT: `8c1e999b` -- `fix(serve-extended-rules): outline guidance follows the addon` (Batch: addon-conditional-outline)

## Batch B — the readability contract

Design constraints for every scenario in this batch:

- `assets/instructions/steps.md` inside the conditional block must preserve
  these phrases exactly:
  `inspect its advisory report once per batch for supported changed components and relevant neighbors`,
  `Disclose which languages and components the outline covers and which it omits`,
  `Unsupported components receive direct source assessment using the same responsibility, interface, dependency-direction, and decomposition questions`,
  `Unavailable outline support is not a failure and is not a reason for a waiver or an outline verdict`,
  `Ask whether responsibilities are cohesive, public interfaces narrow, dependencies correctly directed, and decomposition understandable`,
  `Confirm suspected problems against source before conclusions or edits`,
  `Structural call order is not a complete runtime trace`,
  `uncertain or omitted edges limit the conclusions`,
  `a later structural edit is the reason to regenerate it, not every small edit`,
  `Reuse the report until structural changes make its relevant evidence stale`,
  `nonstructural edits alone do not require rerunning the outline`,
  `no blocking verdict`, `metric waiver`, `forced reviewer context`.
- Add before the architecture questions:
  `Read the outline before the metrics: declarations, signatures, and call
  order, without bodies or comments. State the unit's responsibility in one
  sentence and its ordered call flow. If the outline cannot carry that story
  without the body or a comment, the unit is unreadable: fix it in REFACTOR; if
  it cannot be fixed within the batch, record a substantive review finding. Do
  not waive it.`
- Add the names-as-contract paragraph naming
  `unit and type names, method names, parameter names and types, return types,
  and called type names`, the project vocabulary, and the generic examples
  (`handle`, `process`, `data`, `item`, `manager`, `util`, `helper`).
- Add beside the metrics sentence:
  `Metrics and classifications are hints: a clean metric never excuses an
  unreadable outline, and no metric waives it. Source confirms a suspected
  problem; it never substitutes for the outline story.`
- `assets/instructions/review.md` advisory block must preserve
  `supports substantive, source-confirmed architecture review`,
  `no verdict`, `no metric waiver`, `no forced reviewer context`,
  `no authority to block review or archive`,
  `Disclose which languages and components it covers and which it omits`,
  `Unsupported components receive direct source assessment`,
  `same responsibility, interface, dependency-direction, and decomposition`,
  `not a failure and needs no waiver`,
  `The outline alone cannot block review or archive`.
  It adds the comprehension/naming paragraph and closes with the distinction
  that the outline supplies no verdict while a substantive readability finding
  can reject PASS.
- Served guidance says `supported`/`unsupported`; no concrete language set.
  `docs/outline.md` keeps Rust/Python only in its factual analyzer capability
  section and updates the workflow, metrics table, serving rule, and data
  limit. `README.md` names readability/comprehension and advisory reports.
  `docs/reference.md` states the guidance renders when enabled.
- New step bindings live in `tests/steps/serve_extended_rules_steps.rs` and
  `tests/steps/arc42_review_alignment_steps.rs`; keep the old
  language-specific bindings until the post-archive cleanup.

### Scenario: Refactor guidance uses the outline once for supported changed components and their neighbors

- [x] RED: passed immediately after Batch A's committed supported/unsupported wording; kept as regression protection for the cadence contract. No production diff was forced.
- [x] GREEN: `1 scenario (1 passed)`, `12 steps (12 passed)`.
- [x] REFACTOR: Re-ran the same name filter; `1 scenario (1 passed)`.
- [x] COMMIT: `48e458a1` -- `docs(outline): readability contract` (Batch: outline-readability-contract)

### Scenario: Unsupported outline languages receive source assessment without a waiver

- [x] RED: passed immediately after Batch A's committed supported/unsupported wording; kept as regression protection for the unsupported-only path.
- [x] GREEN: `1 scenario (1 passed)`, `5 steps (5 passed)`.
- [x] REFACTOR: Re-ran the same filter; `1 scenario (1 passed)`.
- [x] COMMIT: `48e458a1` -- `docs(outline): readability contract` (Batch: outline-readability-contract)

### Scenario: Refactor guidance treats an unreadable changed unit as a finding

- [x] RED (pre-change steps asset): `REFACTOR guidance substitutes source reading for the outline story` on the source-confirmation Then.
- [x] GREEN: `1 scenario (1 passed)`, `8 steps (8 passed)` after the comprehension, naming, metrics, source, and fix-or-finding paragraphs.
- [x] REFACTOR: Re-ran; `1 scenario (1 passed)`.
- [x] COMMIT: `48e458a1` -- `docs(outline): readability contract` (Batch: outline-readability-contract)

### Scenario: Final architecture review uses the advisory outline without a second verdict

- [x] RED (pre-change review asset): `mixed-language coverage is disclosed without forcing outline analysis of unsupported components` failed on the old Go/Java sentence.
- [x] GREEN: `1 scenario (1 passed)`, `9 steps (9 passed)`.
- [x] REFACTOR: Re-ran; `1 scenario (1 passed)`.
- [x] COMMIT: `48e458a1` -- `docs(outline): readability contract` (Batch: outline-readability-contract)

### Scenario: A changed unit unreadable from its outline rejects PASS

- [x] RED (pre-change review asset): `the reviewer must reconstruct responsibility and ordered call flow from the outline alone` failed; the comprehension and naming paragraphs were absent.
- [x] GREEN: `1 scenario (1 passed)`, `7 steps (7 passed)`.
- [x] REFACTOR: Re-ran; `1 scenario (1 passed)`.
- [x] COMMIT: `48e458a1` -- `docs(outline): readability contract` (Batch: outline-readability-contract)

## Batch C — implementation routing

Design constraints for every scenario in this batch:

- `assets/instructions/tasks.md`: immediately after the
  `**Audience: the agent running the implement step.**` paragraph in
  `## Step orchestration (implement)`, add:
  `**Step mechanics: `givn instructions steps`.** Read that instruction before
  writing or running any step. It owns the RED/GREEN/REFACTOR loop, step reuse,
  and the REFACTOR outline obligation. This section owns batch sequencing,
  verification, and commit evidence.`
- `assets/skills/givn-implement/SKILL.md.tmpl`: after the existing tasks
  pointer, add `Before writing or running any step, load the RED/GREEN/REFACTOR
  mechanics with `givn instructions steps`.` No `givn-steps` template change.
- Regenerate the tracked generated skills with `givn skills sync` so the
  generated `givn-implement` copies match the template.
- Bindings live in `tests/steps/serve_extended_rules_steps.rs` (tasks
  instruction) and `tests/steps/skill_rewrites_steps.rs` (generated skill and
  served tasks instruction).

### Scenario: Implementation guidance routes to the shared step mechanics

- [x] RED (pre-change assets): `the served guidance names givn instructions steps as the step mechanics` failed; the tasks instruction did not route to the steps mechanics.
- [x] GREEN: `1 scenario (1 passed)`, `3 steps (3 passed)`.
- [x] REFACTOR: Re-ran; `1 scenario (1 passed)`.
- [x] COMMIT: `ebcd2fd9` -- `docs(tasks): route to step mechanics` (Batch: implementation-routing)

### Scenario: Generated implementation guidance routes to the task and step instructions

- [x] RED (pre-change assets): `the generated givn-implement skill names both givn instructions tasks and givn instructions steps` failed; the generated skill named only the tasks instruction.
- [x] GREEN: `1 scenario (1 passed)`, `4 steps (4 passed)` after the skill template change and `./target/debug/givn skills sync`.
- [x] REFACTOR: Re-ran; `1 scenario (1 passed)`.
- [x] COMMIT: `ebcd2fd9` -- `docs(tasks): route to step mechanics` (Batch: implementation-routing)

## Closeout

- [x] Every delta phrase has a permanent binding in the files named above.
- [x] Post-archive cleanup: removed the obsolete language-specific bindings
      from `tests/steps/serve_extended_rules_steps.rs` and
      `tests/steps/arc42_review_alignment_steps.rs` at `4d884e1f`; the archive
      merge and its full-suite run cover the merged candidate.
- [x] Confirm no served guidance or durable doc carries a hardcoded
      supported-language list outside the factual analyzer capability section.
