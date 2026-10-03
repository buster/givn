# Tasks: harden-event-storming

Guidance-only change. Owning contract: fragment `support`, capability
`generate-ideation-guidance` (see `givn/specs/fragments/fragment.md`).
Confirmed Personas: `developer--primary-user` (reads the guidance and reviews
the canvas), `agent--technical-role` (runs the phase and draws the canvas),
`givn-maintainer--owner` (guidance corpus). Domain constraints copied into
every scenario task: the generated guidance is the enforcement surface; the
template is the canonical source and the command guardrails repeat the rule in
one line; no runtime behavior changes; the aggregate bullet's glossary
reference is addon-conditional; the `docs/workflow.md` row and the glossary
row are review-verified against the exact texts in `design.md`, not
spec-asserted.

## Setup

- [x] Change scaffolded; proposal, specs, design, design-review, and arc42
      marker complete.
- [x] Runner strictness: `tests/features_runner.rs` carries
      `.fail_on_skipped()`; each scenario's targeted RED run exits non-zero
      (evidence in that task).
- [x] Planning artifacts committed. COMMIT: `cde2c5d8` -- `docs(harden-event-storming): plan the change`

## Implementation

### Scenario 1 — The ideation guidance requires a canvas with distinguishable element kinds

Domain constraints: the guidance renders from
`assets/skills/givn-ideation/SKILL.md.tmpl` for every agent target; the
canvas is one or more mermaid diagrams; kinds are distinguishable; the
canvas must render; the topic-state line names the rendered canvas.

- [x] RED: targeted run of
      "The ideation guidance requires a canvas with distinguishable element
      kinds" exits non-zero. Evidence: `1 scenario (1 failed)` / exit 101.
- [x] GREEN: canvas bullet (canvas, causal reading), the completeness bullet,
      the per-event aggregate clause, and the topic-state line. Files:
      `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run exits 0.
      Evidence: `1 scenario (1 passed)` / `10 steps (10 passed)`.
- [x] REFACTOR: re-read the section for wording consistency and wrap width;
      targeted run still exits 0.
- [x] COMMIT: `e3c103c3` -- `docs(generate-ideation-guidance): The event storming guidance requires a rendered canvas with distinguishable element kinds`

### Scenario 2 — The ideation guidance keeps aggregates to declared consistency boundaries

Domain constraints: aggregates are given, not minted; the declared
aggregates are reused by name; a new aggregate is a confirmed domain term;
non-aggregates are named as their kind; the durable rows carry the same rule;
the glossary reference is addon-conditional.

- [x] RED: targeted run of
      "The ideation guidance keeps aggregates to declared consistency
      boundaries" exits non-zero. Evidence: `1 scenario (1 failed)` /
      exit 101.
- [x] GREEN: the aggregate-legality bullet (with the
      `{{#if addons.arc42}}` glossary reference) and the non-aggregate-kinds
      bullet; the `docs/workflow.md` row and the
      `docs/arc42/12-glossary.md` row replaced with the design's exact texts.
      Files: `assets/skills/givn-ideation/SKILL.md.tmpl`, `docs/workflow.md`,
      `docs/arc42/12-glossary.md`. Targeted run exits 0. Evidence:
      `1 scenario (1 passed)` / `8 steps (8 passed)`.
- [x] REFACTOR: both conditional branches rendered in scratch projects
      (`givn init --agents opencode` with and without `--no-addons`); the
      arc42 branch renders "(the arc42 glossary's "Aggregates / boundaries")"
      and the no-addon branch renders "(the project's durable
      documentation)", both keeping "Reuse the aggregates the project
      already declares" contiguous. Targeted run still exits 0.
- [x] COMMIT: `3a2ea1fe` -- `docs(generate-ideation-guidance): The event storming guidance keeps aggregates to declared consistency boundaries`

### Scenario 3 — The event storming review checks canvas coverage, legality, agreement, and rendering

Domain constraints: the phase's review lens checks the canvas, not only the
text; the always-on guidance must not name addon-owned words.

- [x] RED: targeted run of
      "The event storming review checks canvas coverage, legality, agreement,
      and rendering" exits non-zero. Evidence: `1 scenario (1 failed)` /
      exit 101.
- [x] GREEN: the Quality reviews Event Storming bullet replaced with the
      design's lens. Files: `assets/skills/givn-ideation/SKILL.md.tmpl`.
      Targeted run exits 0. Evidence: `1 scenario (1 passed)` /
      `7 steps (7 passed)`.
- [x] REFACTOR (finding fixed during verification): the first lens wording
      carried the proposal's "declared glossary aggregate"; the full suite's
      `match-guidance-to-enabled-addons` scenario failed because a project
      with the addons disabled may not mention "glossary" (case-insensitive).
      The lens now says "a declared consistency boundary", which the
      aggregate bullet defines; the spec assertion and `design.md` were
      updated, RED was re-proven for the corrected assertion set
      (`1 scenario (1 failed)` / exit 101), and GREEN re-proven
      (`1 scenario (1 passed)` / `7 steps (7 passed)`). The no-addon
      render of the ideation skill and command contains zero occurrences of
      `arc42`, `glossary`, `ubiquitous`, and `domain-modeling`.
- [x] COMMIT: `7fb1d8c6` -- `docs(generate-ideation-guidance): The event storming review checks canvas coverage, legality, agreement, and rendering`

### Scenario 4 — The ideation command guardrails require the canvas and legal aggregates

Domain constraints: the command repeats the rule in one line; both artifacts
stay in sync.

- [x] RED: targeted run of
      "The ideation command guardrails require the canvas and legal
      aggregates" exits non-zero. Evidence: `1 scenario (1 failed)` /
      exit 101.
- [x] GREEN: the guardrail line appended to
      `assets/commands/givn-ideation.md`. Targeted run exits 0. Evidence:
      `1 scenario (1 passed)` / `5 steps (5 passed)`.
- [x] REFACTOR: the guardrail restates the rule in one line, no paragraph
      duplicated with the skill text; targeted run still exits 0.
- [x] COMMIT: `d7852123` -- `docs(generate-ideation-guidance): The ideation command guardrails require the canvas and legal aggregates`

## Verification

- [x] `givn lint --change harden-event-storming` — exits 0 (advisory
      findings only).
- [x] Change delta suite —
      `GIVN_FEATURES=givn/changes/harden-event-storming/specs ./run-tests.sh`
      — green. Evidence: `Summary [0.415s] 4 tests run: 4 passed, 0 skipped`.
- [x] `./run-tests.sh` — green. Evidence: `Summary [36.960s] 569 tests run:
      569 passed, 0 skipped`.
- [x] `./run-tests.sh --e2e` — green. Evidence: `Summary [7.069s] 91 tests
      run: 91 passed, 478 skipped`.
- [ ] Review artifact complete and signed off.
- [ ] Archive the change.
