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

- [ ] Change scaffolded; proposal, specs, design, design-review, and arc42
      marker complete.
- [ ] Runner strictness: `tests/features_runner.rs` carries
      `.fail_on_skipped()`; each scenario's targeted RED run exits non-zero
      (evidence in that task).
- [ ] Planning artifacts committed. COMMIT: `PENDING` -- `docs(harden-event-storming): plan the change`

## Implementation

### Scenario 1 — The ideation guidance requires a canvas with distinguishable element kinds

Domain constraints: the guidance renders from
`assets/skills/givn-ideation/SKILL.md.tmpl` for every agent target; the
canvas is one or more mermaid diagrams; kinds are distinguishable; the
canvas must render; the topic-state line names the rendered canvas.

- [ ] RED: run
      `GIVN_FEATURES=givn/changes/harden-event-storming/specs cargo test --locked --test features_runner -- --exact "fragments/generate-ideation-guidance.feature::The ideation guidance requires a canvas with distinguishable element kinds"`;
      exits non-zero (missing phrases).
- [ ] GREEN: apply the design's canvas bullets (canvas, causal reading,
      incomplete-without-canvas), the per-event aggregate clause, and the
      topic-state line. Files: `assets/skills/givn-ideation/SKILL.md.tmpl`.
      Targeted run exits 0.
- [ ] REFACTOR: re-read the section for wording consistency and wrap width;
      targeted run still exits 0.
- [ ] COMMIT: `PENDING` -- `docs(generate-ideation-guidance): The event storming guidance requires a rendered canvas with distinguishable element kinds`

### Scenario 2 — The ideation guidance keeps aggregates to declared consistency boundaries

Domain constraints: aggregates are given, not minted; the declared
aggregates are reused by name; a new aggregate is a confirmed domain term;
non-aggregates are named as their kind; the durable rows carry the same rule;
the glossary reference is addon-conditional.

- [ ] RED: targeted run of
      "The ideation guidance keeps aggregates to declared consistency
      boundaries" exits non-zero.
- [ ] GREEN: apply the aggregate-legality bullet (with the
      `{{#if addons.arc42}}` glossary reference) and the non-aggregate-kinds
      bullet; replace the `docs/workflow.md` row and the
      `docs/arc42/12-glossary.md` row with the design's exact texts. Files:
      `assets/skills/givn-ideation/SKILL.md.tmpl`, `docs/workflow.md`,
      `docs/arc42/12-glossary.md`. Targeted run exits 0.
- [ ] REFACTOR: verify the conditional renders both branches; targeted run
      still exits 0.
- [ ] COMMIT: `PENDING` -- `docs(generate-ideation-guidance): The event storming guidance keeps aggregates to declared consistency boundaries`

### Scenario 3 — The event storming review checks canvas coverage, legality, agreement, and rendering

Domain constraints: the phase's review lens checks the canvas, not only the
text.

- [ ] RED: targeted run of
      "The event storming review checks canvas coverage, legality, agreement,
      and rendering" exits non-zero.
- [ ] GREEN: replace the Quality reviews Event Storming bullet with the
      design's lens. Files: `assets/skills/givn-ideation/SKILL.md.tmpl`.
      Targeted run exits 0.
- [ ] REFACTOR: re-read the Quality reviews list for consistency with the
      other phase bullets; targeted run still exits 0.
- [ ] COMMIT: `PENDING` -- `docs(generate-ideation-guidance): The event storming review checks canvas coverage, legality, agreement, and rendering`

### Scenario 4 — The ideation command guardrails require the canvas and legal aggregates

Domain constraints: the command repeats the rule in one line; both artifacts
stay in sync.

- [ ] RED: targeted run of
      "The ideation command guardrails require the canvas and legal
      aggregates" exits non-zero.
- [ ] GREEN: append the guardrail line to
      `assets/commands/givn-ideation.md`. Targeted run exits 0.
- [ ] REFACTOR: confirm no paragraph is duplicated with the skill text;
      targeted run still exits 0.
- [ ] COMMIT: `PENDING` -- `docs(generate-ideation-guidance): The ideation command guardrails require the canvas and legal aggregates`

## Verification

- [ ] `givn lint --change harden-event-storming` — exits 0 or 2.
- [ ] Change delta suite — `GIVN_FEATURES=givn/changes/harden-event-storming/specs ./run-tests.sh`
      — green.
- [ ] `./run-tests.sh` — green.
- [ ] `./run-tests.sh --e2e` — green.
- [ ] Review artifact complete and signed off.
- [ ] Archive the change.
