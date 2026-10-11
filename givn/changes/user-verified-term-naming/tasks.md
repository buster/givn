# Tasks: user-verified-term-naming

Owning contract: fragment `support`, capabilities `generate-ideation-guidance`
and `guide-domain-modeling` (see `givn/specs/fragments/fragment.md`). Confirmed
Personas: `developer--primary-user` (answers the naming question and the term
check), `agent--technical-role` (asks the naming question and runs the term
check), `givn-maintainer--owner` (owns the guidance corpus and the durable
records). Domain constraints copied into every scenario task: the generated
guidance is the enforcement surface; the templates are the canonical source
and the command repeats the rule in one line; no runtime behavior changes; the
naming-dialog shape is single-sourced in `givn-domain-modeling`; the ideation
term-record gate is unconditional; the durable rows (`docs/workflow.md`, the
arc42 chapters) are review-verified against the exact texts in `design.md`,
not spec-asserted.

## Setup

- [x] Change scaffolded; proposal, specs, design, design-review, and arc42
      marker complete.
- [x] Runner contract confirmed: `./run-tests.sh` executes the repository
      feature suite via `tests/features_runner.rs`;
      `GIVN_FEATURES=givn/changes/user-verified-term-naming/specs ./run-tests.sh`
      executes the change delta; `./run-tests.sh --e2e` and `--non-e2e` select
      the tagged subsets. Strict mode:
      `GivnWorld::cucumber().fail_on_skipped()`; the runner exits non-zero
      when `failed > 0` or `skipped > 0`.
- [x] Single-scenario command (exact, nextest is the default runner):
      `GIVN_FEATURES=givn/changes/user-verified-term-naming/specs cargo nextest run --locked --test features_runner -- "fragments/<feature>.feature::<scenario title>"`
- [x] Proof of strictness (before scenario work): the two `@wip`
      domain-modeling scenarios currently fail with undefined steps under the
      strict runner; the change delta run reports 0 passed. Evidence: change
      delta run at setup time — `7 tests run: 0 passed, 7 failed,
      0 skipped` (exit non-zero); the two `@wip` scenarios fail with
      undefined steps under `fail_on_skipped`.
- [x] Step-definition locations per `design.md`: the naming-dialog steps
      extend `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`
      (capability `guide-domain-modeling`); the ideation delta reuses the
      shared guidance/command content steps in
      `tests/steps/prose_consolidation_steps.rs` (capability
      `generate-ideation-guidance`); no new runner, test target, or service.
- [x] Interfaces: CLI text output only (`design.md` Visual Design Contract:
      `Interface: terminal`, Viewports not applicable). No local service,
      isolated network, credential, or digital twin is required; the real
      interface is the `givn skills sync` CLI writing the rendered Markdown.
- [x] E2E scope: this change adds no `@e2e` scenario (no new consumer
      action; the existing synchronize-skills `@e2e` scenarios remain the
      evidence). E2E setup task not applicable. Count proof pasted in
      Verification (e2e strictly smaller than the full suite).
- [x] Planning artifacts committed. COMMIT: `2bc72212` —
      `docs(user-verified-term-naming): plan the change with the reviewed design and the scenario task breakdown`

## Implementation

### Scenario 1 — The ideation guidance binds every confirmed term to the user's verified name

Domain constraints: the `confirmed` clause in
`assets/skills/givn-ideation/SKILL.md.tmpl` gains the user's verified name
alongside the term introduction test, the contrast pair, the concrete
sentence, and the collision check; the clause is unconditional; the existing
`{{#if addons.domain_modeling}}` sentence after it stays wrapped.

- [x] RED: targeted run of "The ideation guidance binds every confirmed term
      to the user's verified name" exits non-zero (string absent). Evidence:
      `1 scenario (1 failed)` / `3 steps (2 passed, 1 failed)`; nextest
      `1 test run: 0 passed, 1 failed, 6 skipped`; exit non-zero.
- [x] GREEN: the `confirmed` status bullet carries the design's wording.
      Files: `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run exits
      0. Evidence: `1 test run: 1 passed, 6 skipped`; the bullet now reads
      "one concrete domain sentence, the user's verified name, and a
      recorded collision check."
- [x] REFACTOR: re-read the section for wording consistency and wrap width;
      every permanent assertion (`definition and first use`, `recorded
      collision check`, `never deleted`) stays intact; targeted run still
      exits 0. Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] COMMIT: `3b4bc796` — `feat(generate-ideation-guidance): Givn asks the
      user to verify every newly coined domain term — failed names
      immediately, with the concept's description and concrete candidate
      names, the rest at the phase's term check — before any canvas, use
      case, or change seed relies on it.` (subject = release note; body =
      `Scenario: The ideation guidance binds every confirmed term to the
      user's verified name`)

### Scenario 2 — The ideation guidance asks the naming question as soon as a name fails

Domain constraints: the failed-name triggers (generic, overloaded, no genus,
competing candidates) and the collision-checked user-supplied name are stated
where the term record is defined; the dialog shape itself stays single-sourced
in the domain-modeling challenge bullet (this text names the "naming dialog"
only); the section is unconditional.

- [x] RED: targeted run of "The ideation guidance asks the naming question as
      soon as a name fails" exits non-zero. Evidence: `1 scenario (1 failed)`
      / `3 steps (2 passed, 1 failed)`; nextest `1 test run: 0 passed,
      1 failed, 6 skipped`; exit 100.
- [x] GREEN: the term-check section header and its first bullet, plus the
      clause "A name the user supplies is collision-checked like any other."
      Files: `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run exits
      0. Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] REFACTOR: wording consistency with the domain bullet (no duplicated
      dialog shape); targeted run still exits 0. Evidence: `1 test run:
      1 passed, 6 skipped`.
- [x] COMMIT: `74d11d21` — `feat(generate-ideation-guidance): Givn asks the
      user to verify every newly coined domain term — failed names
      immediately, with the concept's description and concrete candidate
      names, the rest at the phase's term check — before any canvas, use
      case, or change seed relies on it.` (body = `Scenario: The ideation
      guidance asks the naming question as soon as a name fails`)

### Scenario 3 — The ideation guidance asks the term check before a phase closes

Domain constraints: the term check is the phase's single closing question;
`Confirm all` makes the terms `confirmed`; a revisited term gets its naming
dialog one at a time before confirmation, the other terms stand, and nothing
else re-opens; a replaced name becomes an anti-term; the rows carry the
one-sentence description and the recorded collision check; the section is
unconditional.

- [x] RED: targeted run of "The ideation guidance asks the term check before
      a phase closes" exits non-zero. Evidence: `1 scenario (1 failed)` /
      `4 steps (3 passed, 1 failed)`; nextest `1 test run: 0 passed,
      1 failed, 6 skipped`; exit 100.
- [x] GREEN: the section's remaining wording: "Every other new core term —
      ... rides the phase's closing checkpoint as one question. It reads:
      'Term check — <phase>...'" with the confirm/revisit outcomes. Files:
      `assets/skills/givn-ideation/SKILL.md.tmpl`. Targeted run exits 0.
      Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] REFACTOR: cross-check the wording against the design's block (em dash
      and backtick placement); targeted run still exits 0. Evidence:
      `1 test run: 1 passed, 6 skipped`.
- [x] COMMIT: `c8300a22` — `feat(generate-ideation-guidance): Givn asks the
      user to verify every newly coined domain term — failed names
      immediately, with the concept's description and concrete candidate
      names, the rest at the phase's term check — before any canvas, use
      case, or change seed relies on it.` (body = `Scenario: The ideation
      guidance asks the term check before a phase closes`)

### Scenario 4 — The verified-name gate holds without the addon

Domain constraints: this scenario pins the placement decision — the verified
name clause and the term check render with no addons; only the "full term
introduction test lives in `givn-domain-modeling`" sentence stays wrapped in
`{{#if addons.domain_modeling}}`; the no-addon render must not name
addon-owned words outside their branch.

- [x] RED: setup-time placement proof — at the planning commit (`2bc72212`
      state), the change delta run `7 tests run: 0 passed, 7 failed` includes
      this scenario failing with both strings absent (no-addon render). Evidence:
      scoped setup run recorded in `## Setup`.
- [x] GREEN: no production change — placement pin; the two strings are
      unconditional after scenarios 1 and 3; verified against the no-addon
      fixture. Evidence: `1 test run: 1 passed, 6 skipped` (exit 0).
- [x] REFACTOR: none required (text-only pin); targeted run still exits 0.
      Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] COMMIT: none — placement pin. No production change, so no scenario
      commit; the unconditional placement was delivered by scenarios 1 and 3
      and is verified here, not implemented again.

### Scenario 5 — The ideation command guardrails carry the verified-name rule

Domain constraints: the command repeats the rule in one line; command assets
carry no template conditionals; the rule must match the skill text in
substance (phase term check, naming dialog, the before-use gate).

- [x] RED: targeted run of "The ideation command guardrails carry the
      verified-name rule" exits non-zero. Evidence: cucumber `1 step failed`;
      nextest `1 test run: 0 passed, 1 failed, 6 skipped`.
- [x] GREEN: the guardrail bullet appended after the persona bullet. Files:
      `assets/commands/givn-ideation.md`. Targeted run exits 0. Evidence:
      `1 test run: 1 passed, 6 skipped`.
- [x] REFACTOR: one line, no paragraph duplicated with the skill text;
      targeted run still exits 0. Evidence: `1 test run: 1 passed,
      6 skipped`.
- [x] COMMIT: `d066cea9` — `feat(generate-ideation-guidance): Givn asks the
      user to verify every newly coined domain term — failed names
      immediately, with the concept's description and concrete candidate
      names, the rest at the phase's term check — before any canvas, use
      case, or change seed relies on it.` (body = `Scenario: The ideation
      command guardrails carry the verified-name rule`)

### Scenario 6 — The challenge bullet hands the name choice to the user

Domain constraints: the challenge bullet drops "Propose it and justify it"
and keeps the "Challenge generic or overloaded terms" heading (a permanent
assertion); the dialog shape is stated once, here: concept sentence, up to
three candidate names with what each evokes and any collision it carries, the
user's own name accepted.

- [x] RED: remove `@wip` from this scenario only; targeted run of "The
      challenge bullet hands the name choice to the user" exits non-zero
      (undefined steps under strict mode). Evidence: `1 scenario (1 failed)` /
      `3 steps (2 passed, 1 failed)`; nextest `1 test run: 0 passed,
      1 failed, 6 skipped`.
- [x] GREEN: the challenge bullet replacement (full block) and the five
      naming-dialog steps (assertions per `design.md`). Files:
      `assets/skills/givn-domain-modeling/SKILL.md.tmpl`,
      `tests/steps/fix_domain_modeling_skill_trigger_steps.rs`. Targeted run
      exits 0. Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] REFACTOR: term introduction test section untouched; targeted run still
      exits 0. Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] COMMIT: `57375275` — `feat(guide-domain-modeling): Givn asks the user
      to verify every newly coined domain term — failed names immediately,
      with the concept's description and concrete candidate names, the rest
      at the phase's term check — before any canvas, use case, or change
      seed relies on it.` (body = `Scenario: The challenge bullet hands the
      name choice to the user`; the `@wip` removal from the delta scenario
      rode this commit)

### Scenario 7 — The challenge bullet shows a naming question with candidates and a collision

Domain constraints: the worked example carries the naming question with the
concept sentence, the candidate names, the collision note, and the free-form
invitation; the asserted substrings stay contiguous on one line; the permanent
assertion `Genus + differentia` etc. remain intact.

- [x] RED: `@wip` removed from this scenario only; targeted run exits
      non-zero (undefined steps under strict mode). Evidence: `1 scenario
      (1 failed)` / `3 steps (2 passed, 1 failed)`; nextest `1 test run:
      0 passed, 1 failed, 6 skipped`; failure `Step doesn't match any
      function` for the first naming-question step.
- [x] GREEN: the three example steps — `body_shows_naming_question` (regex,
      asserts the term and `Candidates:`), `body_shows_colliding_candidate`
      (regex, asserts the candidate and `collides`), `body_asks_which_name`
      (asserts `should the record use, or what is yours`). Files:
      `tests/steps/fix_domain_modeling_skill_trigger_steps.rs` (the example
      wording itself rode scenario 6's commit `57375275`). Targeted run
      exits 0. Evidence: `1 test run: 1 passed, 6 skipped`.
- [x] REFACTOR: wrap check — `Candidates:`, `collides`, and `should the
      record use, or what is yours` each on one line; no template edit
      needed; targeted run still exits 0. Evidence: `1 test run: 1 passed,
      6 skipped`.
- [x] COMMIT: `486ede9b` — `feat(guide-domain-modeling): Givn asks the user
      to verify every newly coined domain term — failed names immediately,
      with the concept's description and concrete candidate names, the rest
      at the phase's term check — before any canvas, use case, or change
      seed relies on it.` (body = `Scenario: The challenge bullet shows a
      naming question with candidates and a collision`; the `@wip` removal
      from the delta scenario rode this commit)

## Verification

- [x] `givn lint --change user-verified-term-naming` — exits 0 (no `@wip`
      findings remain once scenarios 6 and 7 are GREEN); the advisory
      `[SHAPE]`/`[SUBST]` overlap findings are dispositioned at review.
      Evidence: `givn lint: 2 file(s) checked — clean` (exit 0); advisories
      (6 shape overlaps, subset advisories) dispositioned in the review
      artifact as expected: delta scenarios pin distinct strings over the
      same guidance files and merging would violate one-scenario-one-behaviour.
- [x] Change delta suite —
      `GIVN_FEATURES=givn/changes/user-verified-term-naming/specs ./run-tests.sh`
      — green. Evidence: `7 tests run: 7 passed, 0 skipped` (exit 0).
- [x] `./run-tests.sh` — green. Evidence: `591 tests run: 591 passed,
      0 skipped` (44.451s, exit 0).
- [x] `./run-tests.sh --e2e` — green; e2e count strictly smaller than the
      full-suite count (paste both). Evidence: `90 tests run: 90 passed,
      501 skipped` (7.453s, exit 0) — 90 < 591.
- [x] Review artifact complete and signed off.
- [x] Archive the change.
