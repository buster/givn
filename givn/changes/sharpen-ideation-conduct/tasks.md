# Tasks: sharpen-ideation-conduct

Guidance-only change. Owning contract: fragment `support`, capability
`generate-ideation-guidance` (see `givn/specs/fragments/fragment.md`).
Confirmed Personas: `developer--primary-user` (answers questions),
`agent--technical-role` (asks and cuts seeds), `givn-maintainer--owner`
(guidance corpus). Domain constraints copied into every scenario task: the
generated guidance is the enforcement surface; the template is the canonical
source and the command guardrails repeat the rules in one line each; no
runtime behavior changes.

## Setup

- [x] Change scaffolded; proposal, specs, design, design-review, and arc42
      marker complete.
- [x] Prior ideation handoff committed first so this change's commits stay
      scoped. COMMIT: `6b4541dd` -- `docs(ideation): hand off local-only-integrations`
- [ ] Runner strictness: `tests/features_runner.rs` carries
      `.fail_on_skipped()`; the first scenario's targeted RED run exits
      non-zero before implementation (evidence in that task).
- [x] Planning artifacts committed. COMMIT: `fd7c1669` -- `docs(sharpen-ideation-conduct): plan the change`

## Implementation

### Scenario 1 — The ideation guidance carries the live-question standard

Domain constraints: the guidance is generated from
`assets/skills/givn-ideation/SKILL.md.tmpl` for every agent target; the
capability is owned by fragment `support`.

- [x] RED: run
      `GIVN_FEATURES=givn/changes/sharpen-ideation-conduct/specs cargo test --test features_runner -- --exact "support/generate-ideation-guidance.feature::The ideation guidance carries the live-question standard"`;
      exits non-zero (missing phrases). Evidence:
      `1 scenario (1 failed)` / `3 steps (2 passed, 1 failed)` / exit 101.
- [x] GREEN: rewrite the first paragraph of the "One question at a time"
      section in `assets/skills/givn-ideation/SKILL.md.tmpl` to the design's
      wording (dialog-policy pointer; bare option list is a defect; decision
      it blocks; context that forces it). Files:
      `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run exits 0.
      Evidence: `1 scenario (1 passed)` / `5 steps (5 passed)`.
- [x] REFACTOR: re-read the section for wording consistency; targeted run
      still exits 0.
- [x] COMMIT: `d42117ad` -- `docs(generate-ideation-guidance): The ideation guidance carries the live-question standard`

### Scenario 2 — The ideation guidance stops questions at decided directions

Domain constraints: decided directions are never re-asked; mechanical review
findings are applied; later phase boundaries report.

- [x] RED: targeted run of
      "The ideation guidance stops questions at decided directions" exits
      non-zero. Evidence: `1 scenario (1 failed)` / exit 101.
- [x] GREEN: rewrite the second paragraph of the same section with the
      no-re-asking, applied-and-reported, phase-report, and shared-direction
      rules. Files: `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run
      exits 0. Evidence: `1 scenario (1 passed)` / `7 steps (7 passed)`.
- [x] REFACTOR: keep the paragraph under the template's wrap width; targeted
      run still exits 0.
- [x] COMMIT: `25e11a7a` -- `docs(generate-ideation-guidance): The ideation guidance stops questions at decided directions`

### Scenario 3 — The frame is confirmed once and later phase boundaries report

Domain constraints: the Frame defines goal, scope, and stakeholders and is
confirmed once; later boundaries report and proceed.

- [x] RED: targeted run of
      "The frame is confirmed once and later phase boundaries report" exits
      non-zero. Evidence: `1 scenario (1 failed)` / exit 101.
- [x] GREEN: replace the Frame paragraph ("Report it to the user and let them
      confirm it before any phase starts.") with the confirmed-once wording.
      Files: `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run exits
      0. Evidence: `1 scenario (1 passed)` / `4 steps (4 passed)`.
- [x] REFACTOR: verify the Personas section's "confirms every persona" rule is
      untouched; targeted run still exits 0.
- [x] COMMIT: `e28b7574` -- `docs(generate-ideation-guidance): The frame is confirmed once and later phase boundaries report`

### Scenario 4 — Handoff cuts change seeds by deliverable guarantee

Domain constraints: a seed is one change that can ship alone; no intermediate
state may make a confirmed rule untrue; the seed file is named by change
identity; arc42 8.19 and the glossary row describe the durable meaning.

- [x] RED: targeted run of
      "Handoff cuts change seeds by deliverable guarantee" exits non-zero.
      Evidence: `1 scenario (1 failed)` / exit 101.
- [x] GREEN: replace the Handoff seed bullet with the deliverable cut; rename
      the topic-state entry to `changes/<change-id>.md`; generalize the seed
      schema heading and use-case IDs to a change identity with plural IDs.
      Files: `assets/skills/givn-ideation/SKILL.md.tmpl`,
      `docs/arc42/08-crosscutting-concepts.md` (8.19, prepared during
      design), `docs/arc42/12-glossary.md` ("Change seed"). Targeted run
      exits 0. Evidence: `1 scenario (1 passed)` / `10 steps (10 passed)`.
- [x] REFACTOR: re-read 8.19 and the glossary row for consistent wording
      ("per-change, cut by deliverable guarantee"); targeted run still exits
      0.
- [x] COMMIT: `a1140501` -- `docs(generate-ideation-guidance): Handoff cuts change seeds by deliverable guarantee`

### Scenario 5 — The ideation command guardrails carry the same conduct rules

Domain constraints: the command repeats the rules in one line each; both
artifacts stay in sync.

- [x] RED: targeted run of
      "The ideation command guardrails carry the same conduct rules" exits
      non-zero. Evidence: `1 scenario (1 failed)` / exit 101.
- [x] GREEN: update `assets/commands/givn-ideation.md`: the question line
      gains the full-treatment and no-re-asking rules; the final batch line
      becomes the seed-cut guardrail; one line records the Frame-once rule.
      Files: `assets/commands/givn-ideation.md`. Targeted run exits 0.
      Evidence: `1 scenario (1 passed)` / `4 steps (4 passed)`.
- [x] REFACTOR: confirm no duplicated paragraph between Stance and
      Guardrails; targeted run still exits 0.
- [ ] COMMIT: <hash> -- `docs(generate-ideation-guidance): The ideation command guardrails carry the same conduct rules`

## Verification

- [ ] `givn lint --change sharpen-ideation-conduct` — clean (advisory overlap
      findings only).
- [ ] `./run-tests.sh` — green. Paste the summary.
- [ ] `./run-tests.sh --e2e` — green. Paste the summary.
- [ ] Review artifact complete and signed off.
- [ ] Archive the change.
