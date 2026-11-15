# Tasks: split-stack-instructions

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and `./run-tests.sh --e2e`
(`verify.e2e_command`), both already set in `givn/commands.yaml`. Strict mode:
`.fail_on_skipped()` at both registration points in `tests/features_runner.rs`
(lines 55 and 155); an undefined or skipped step fails the run.

Empty-body ban: no step body is `{}`, a bare `pass`, a bare `return`, or
whitespace. Unimplemented Rust steps use `unimplemented!()` or `todo!()`. This
change has no runner, version, or invocation change, so strictness is proven
once in `## Setup` and not re-proven per batch.

Outline parameters: a phrase carrying a quoted placeholder (`<id>`,
`<runner>`, `<binding>`, `<stub>`, `<strict>`, `<command>`) is an `Examples`
parameter, substituted before matching. Bind it with
`regex = r#"..."#`, never as a literal `<id>` binding (which never matches).
Use a greedy capture between the framing quotes (`^… carries "(.+)"$`): the
`steps-python` stub value itself contains quotes
(`raise NotImplementedError("TODO")`), so a `([^"]+)` capture truncates it.
The same phrase in two Outlines shares one binding.

Single-scenario run command (design.md "Step Definitions"). `<feature>` is the
path under this change's `specs/`; for an Outline use the Outline title and the
runner executes every `Examples` row.

```sh
GIVN_FEATURES=givn/changes/split-stack-instructions/specs \
  cargo test --locked --test features_runner -- --exact \
  "<capability>/<feature>.feature::<title>"
```

Release note (every batch commit subject, verbatim from `proposal.md`, with
its trailing period dropped; `02126585` and `e52b4667` already carry it):

```text
Every core step instruction now names the stack instruction for the project's runner and carries only the universal rules, while the per-stack mechanics are served on request by id
```

Batch commit rule: one atomic
`givn commit -m "<release note>"$'\n\n'"Batch: <batch-name>" -- <paths...>`
per batch with explicit paths. `givn commit` takes one `--message`, so subject
and body travel in one argument. Every scenario of the batch records the same
revision as `COMMIT: <hash> -- Batch: <batch-name>`; the gate reads the first
`COMMIT:` line under the scenario line, so a scenario section carries exactly
one. `givn check review` passes a scenario commit only when the commit message
contains the batch name.

**Evidence fields.** Each scenario carries the RED slot, the GREEN slot, and
the commit line. A scenario whose behavior already landed in `02126585` or
`e52b4667` is recorded as **existing behavior protected** with a discrimination
probe; no production edit is manufactured for it. A scenario whose behavior
does not exist yet records a real behavioral RED.

**`@wip`.** Each batch removes `@wip` from the scenarios it starts, never in
bulk. `givn check tasks` refuses the change while any selected delta scenario
still carries `@wip`.

**No `@e2e` scenario.** The delta adds none: requesting a stack instruction is
the same consumer action as requesting any other instruction id, so the
permanent `@e2e` rows keep their drivers (design.md "Interaction Coverage
Matrix"). No e2e setup task, no transcript, no `@e2e` tag is added or removed.

## Batch map

| Batch | Scenario entries | Executed rows | Owning capability |
|---|---|---|---|
| `setup` | 0 | 0 | none |
| `design-corrigenda` | 0 | 0 | none |
| `reference-check` | 3 | 3 | `instruction-integrity` |
| `stack-load-conditions` | 2 (two 6-row Outlines) | 12 | `serve-extended-rules` |
| `stack-bodies` | 1 (6-row Outline) | 6 | `serve-extended-rules` |
| `core-absence` | 2 | 2 | `serve-extended-rules` |
| `tasks-guidance` | 1 | 1 | `serve-extended-rules` |
| `stack-id-registration` | 2 | 2 | `serve-extended-rules` |
| `ise-mapping` | 1 | 1 | `instruction-integrity` |
| `review-obligations` | 1 | 1 | `instruction-integrity` |
| `adr-register-count` | 1 (`@givn.modified`) | 1 | `adr-qualification-gate` |
| **Total** | **14** | **29** | 10 `serve-extended-rules` + 3 `instruction-integrity` + 1 `adr-qualification-gate` |

The delta holds 14 scenario declarations: 13 `@givn.added` and one
`@givn.modified`,
`specs/obtain-effective-guidance/serve-extended-rules.feature` (8),
`specs/fragments/instruction-integrity.feature` (5), and
`specs/fragments/adr-qualification-gate.feature` (1). Nothing is
`@givn.removed`. Expanding the three Outlines' `Examples` rows gives 29
executed scenarios. Every declaration appears in exactly one batch by exact
title below.

---

## Interaction coverage pre-flight

The delta adds no inventory entry. Requesting a stack instruction is the same
consumer action as requesting any other instruction id with another id, so the
owning `## Interactions` rows and their `@e2e` scenarios are unchanged
(design.md "Interaction Coverage Matrix").

| Owning document | Inventory entries | Matrix rows | `@e2e` scenario exists |
|---|---|---|---|
| `specs/obtain-effective-guidance/usecase.md` | 1 | 1 | yes |
| `specs/fragments/fragment.md` (`instruction-integrity`) | 2 | 2 | yes |

No gap: every matrix target is a permanent `@e2e` scenario that already exists,
and the delta adds no `@e2e` scenario for this file to plan.

---

## Domain Constraints

Copied from `specs/obtain-effective-guidance/usecase.md` (goal, main flow,
extensions, rules, success guarantee, Personas), the `support` fragment, and
design.md's Decisions ledger. Each scenario's Design constraints repeat the
rules its behavior exercises.

| Capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| `serve-extended-rules` | The served text of `steps` and `design` names each stack instruction id with the condition for loading it and carries no stack body; `specs` and `review` carry no stack body and name no stack instruction; `tasks` and `tasks-reverse` point at `steps` for the mapping and carry no stack body. | `agent--technical-role`, `developer--primary-user` |
| `serve-extended-rules` | A stack instruction's body is served only on a request for its own id and never inside a core instruction's served text; a core instruction's served text is complete for a change that touches no named stack, and the split removes no universal rule. | `agent--technical-role`, `givn-maintainer--owner` |
| `serve-extended-rules` | A stack instruction id is a policy instruction id: it resolves with no change context. The text path and the machine-readable path carry the same effective content. | `agent--technical-role` |
| `serve-extended-rules` | The unknown-id refusal keeps its wording and lists the known ids, including the stack instruction ids; a request never mutates project state. | `developer--primary-user` |
| `serve-extended-rules` | The `ise` instruction names `steps` as the mapping for the change's stack mechanics, one stack instruction id as a worked example, and the two artifact instructions that own the records; an addon-bound id is served only while its addon is effective. | `agent--technical-role` |
| `instruction-integrity` | Every `givn instructions <id>` direction a served instruction or an effective addon instruction names resolves in the context that instruction needs: an artifact id with a change context, a policy id without one. An unresolved or mis-contexted id is a finding naming the id, its source line, and the line text. | `givn-maintainer--owner` |
| `instruction-integrity` | Each rule has one owner: a moved row leaves the core instruction instead of being copied, and the review guidance keeps every surviving obligation with one stable marker per obligation. | `givn-maintainer--owner`, `developer--primary-user` |
| `adr-qualification-gate` | `ADR-077` is accepted, the active register counts 29 accepted decisions and no proposed decision, and the change records the qualification once in `arc42.md`. | `givn-maintainer--owner` |

Carried by every batch: no Givn implementation symbol appears in served
guidance; instruction length carries no numeric cap, so no rule is removed or
split to satisfy a line count and a scenario never asserts a line count; a
request never mutates project state; the text and machine-readable paths carry
identical content.

---

## Contradictions

Each entry was observed while planning. Both are resolved by the operator
correction of 2026-11-15; the landing artifact is named in each entry.

1. **The reference check's context rule and six pre-existing corpus
   directions (resolved: the rule was narrowed).** The drafted rule made an
   artifact-id direction without `--change` a finding, and six shipped
   directions name an artifact instruction without one:
   `assets/instructions/characterize.md:69` and
   `assets/instructions/review-reverse.md:24` (`givn instructions review`),
   `assets/instructions/design-review.md:18,65` and
   `assets/instructions/proposal.md:111` (`givn instructions review`), and
   `assets/instructions/specs.md:326` (`givn instructions design`). Each
   resolves today because `run_artifact` auto-selects the single active change
   and refuses by name only when there is none or more than one
   (`src/cli/instructions.rs:119-130`); requiring the flag would edit five
   assets and force an agent to know the change id before reading guidance.
   The operator narrowed the rule to the half that is wrong: a policy id
   carrying `--change` is a finding, because `run_artifact` prints a
   non-artifact id before it reads `change`
   (`src/cli/instructions.rs:82-117`). The corpus assets stay unchanged, the
   two wrong-context scenarios collapse into one, and the delta drops to 14
   declarations and 29 executed rows. Landing: `design.md` (contract, boundary
   rows, D5), `specs/fragments/instruction-integrity.feature`,
   `docs/arc42/08-crosscutting-concepts.md`,
   `docs/arc42/05-building-block-view.md:65`, and `design-review.md`
   `## Operator correction`.

2. **The permanent ADR register scenario was red before archive (resolved: the
   reconciliation is applied).** `e52b4667` accepted `ADR-077`, so
   `docs/arc42/adr/README.md` counts 29 accepted rows while
   `givn/specs/fragments/adr-qualification-gate.feature:123` asserted 28; the
   observed failure was `assertion left == right failed: unexpected accepted
   ADR count` with `left: 29 right: 28`. The permanent scenario now asserts 29
   and `tests/steps/adr_qualification_gate_steps.rs:881` binds 29, so both the
   permanent and the change-scoped runs pass before archive, the same
   reconciliation `d5fa08ec` made at the 28 boundary. The `adr-register-count`
   batch records it as existing behavior protected.

3. **The machine-readable comparison excludes the skill route.** The delta
   Then reads "the machine-readable instruction field equals the served text",
   but the text path appends a trailing `Skill to load now:` line that the JSON
   `instruction` field does not carry. The comparison is the existing
   `without_skill_route` contract (`tests/steps/serve_extended_rules_steps.rs:261`);
   the batch reuses it and records why. No artifact changes.

---

## Durable documentation placement

The arc42 chapters, the glossary row, the ADR, and `docs/reference.md` already
carry this change from `02126585` and `e52b4667`. They describe delivered
behavior; each statement is checked in the batch whose code makes it true, and
a statement reality contradicts is corrected in that batch.

| Statement | Location | Batch | What makes it true |
|---|---|---|---|
| `TC-48`, the composition rule, the no-cap rule | `docs/arc42/02-architecture-constraints.md` | `stack-load-conditions` (presence), `core-absence` (absence) | the served presence and absence assertions |
| Guidance strategy row | `docs/arc42/04-solution-strategy.md` | `stack-load-conditions` | the id mapping in `steps` and `design` |
| `cli::instructions` responsibility, the test-support row | `docs/arc42/05-building-block-view.md:65` | `stack-id-registration` (id set), `reference-check` (extractor) | `POLICY_INSTRUCTION_IDS` and `tests/common/instruction_references.rs` |
| Split rule, reference check, addon sources | `docs/arc42/08-crosscutting-concepts.md:1460-1473` | `reference-check` | the extractor, findings, and context rules |
| `ADR-077` register row and the accepted count | `docs/arc42/adr/ADR-077-stack-instructions.md`, `docs/arc42/adr/README.md:64` | `adr-register-count` | the register's 29 accepted rows |
| `Q-126` | `docs/arc42/10-quality-requirements.md` | `stack-load-conditions`, `core-absence` | the served-text assertions |
| `R-148` and `D-29` named as retired | `docs/arc42/11-risks-and-technical-debt.md` | `core-absence` | no instruction carries a size ceiling |
| `Stack instruction` glossary row | `docs/glossary.md:93` | `stack-id-registration` | the resolved policy id |
| Policy-id list | `docs/reference.md` | `stack-id-registration` | the six registered ids |
| `review.md` keeps every obligation | `assets/instructions/review.md` | `review-obligations` | the nine markers |
| `docs/domain-model.md` unchanged | none | `ise-mapping` (recorded, not edited) | the model scopes domain units, not instruction composition |

---

## Setup

- [ ] Read the owning `specs/obtain-effective-guidance/usecase.md` (goal, main
      flow, extensions, rules, success guarantee, Personas, Interactions), the
      `support` fragment, and the seed
      `givn/ideation/ise-rebuildability/changes/split-stack-instructions.md`
      before binding any step. Domain constraints are in `## Domain
      Constraints` above.
      ```
      paste: the use-case sections read, and the constraint each batch carries
      ```

- [ ] Confirm the runner and both commands; change neither. No runner is
      installed and no dependency is added: cucumber-rs 0.23.0 as `Cargo.lock`
      pins it, already wired through `tests/features_runner.rs` with
      `.fail_on_skipped()` at both registration points.
      ```
      verify.command:     ./run-tests.sh --non-e2e
      verify.e2e_command: ./run-tests.sh --e2e
      regular scope: not @e2e and not @givn.removed
      e2e scope:     @e2e and not @givn.removed
      paste: grep -n 'fail_on_skipped' tests/features_runner.rs  -> lines 55 and 155
      ```

- [ ] Proof-of-strictness. Write one scratch feature outside the delta scope
      with one unbound step and run it through both the change-scoped runner
      and the `--exact` path this file uses; confirm non-zero exit on each.
      Then delete the scratch feature and confirm the permanent suite still
      exits 0 apart from Contradiction 2.
      ```
      paste: both commands, the non-zero exits, the skipped/failed counts,
      and the two deltas' current scenario counts
      ```

- [ ] Confirm the binding inventory before writing steps. Reuse these existing
      bindings and never rebind a phrase (a duplicate registration panics the
      runner):
      - `a Givn project with an active change`:
        `tests/steps/instruction_integrity_e2e_steps.rs:21` (`make_init_root`
        plus `givn new instruction-integrity`);
      - `the command exits 0` and `the command exits non-zero`:
        `tests/steps/mod.rs:630,669`;
      - `stderr names "{}"`: `tests/steps/archive_proof_steps.rs:742`.
      Helpers to reuse rather than duplicate:
      `served_instruction(env, id)` (`serve_extended_rules_steps.rs:60`, adds
      `--change` for manifest artifact ids), `request_change_guidance(env, id)`
      (`:80`), `without_skill_route(text)` (`:261`). Served text is
      line-wrapped: assert a fragment that fits one line, or normalize
      whitespace before comparing.
      ```
      paste: the phrase-to-binding map for the 60 unbound delta phrases, and
      the duplicate check (each phrase bound exactly once)
      ```

- [ ] Record the shared absence token sets in
      `tests/steps/serve_extended_rules_steps.rs` as `pub(crate)` constants,
      exactly these members; batches `stack-load-conditions`, `stack-bodies`,
      `core-absence`, and `tasks-guidance` reuse them:
      - `STACK_IDS`: `steps-rust`, `steps-python`, `steps-node`, `steps-java`,
        `steps-go`, `steps-ruby`;
      - `RUNNERS`: `cucumber-rs`, `behave`, `cucumber-js`, `cucumber-jvm`,
        `godog`, `cucumber-ruby`;
      - `STEP_PATHS`: `tests/steps/`, `features/steps/`, `src/test/java`;
      - `STUB_FORMS`: `unimplemented!()`, `todo!()`, `NotImplementedError`,
        `return "pending";`, `godog.ErrPending`, `pending("TODO")`,
        `PendingException`;
      - `STRICT_MECHANISMS`: `.fail_on_skipped()`, `--strict`,
        `godog.ErrPending`, `fail on pending`, `PendingException`;
      - `RUN_COMMANDS`: `cargo test <name> -- --exact`, `behave -n`,
        `cucumber-js --name`, `mvn test -Dcucumber.filter.name`,
        `go test ./... -run TestFeatures`, `cucumber --name`;
      - `BINDING_BULLETS`: `Step definition file:`, `Binding form:`,
        `Unimplemented body:`.
      The archive binding-token paragraph (`#[given(`, `@given(`, `.Given(`,
      `Given(`) is universal rule text kept in `steps.md`; an absence assertion
      never fires on it. `bare pass`, `{}`, `return`, and "strict-mode" as
      prose are universal wording, not per-runner mechanisms.
      ```
      paste: the constant block and the one place each set is asserted
      ```

- [ ] COMMIT: `<hash>` -- Batch: `setup`

---

## Behavior batches

Each batch names the complete file set for its scenarios; its scenarios share
one test phase, one implementation phase, one refactor phase, and one commit.
There is no `@e2e` scenario, so every batch is a non-`@e2e` batch and the
`@e2e` section of the tasks format is vacuous for this change.

### Batch 1: `design-corrigenda`

Two factual corrections the re-audit found. Neither adds a scenario, changes a
decision, changes a command or file path, or changes a delta feature. The first
batch carries them so every later batch reads a design whose statements are
true.

**Files modified:** `givn/changes/split-stack-instructions/design.md`,
`givn/changes/split-stack-instructions/design-review.md`.

- [ ] Correction 1, `design.md:14-17`. The overview claims registration is what
      makes `givn instructions steps-rust` resolve. It is not:
      `crate::config::cascade::resolve_instruction` tries
      `givn/artifacts/<id>/instructions.md`, then the embedded
      `instructions/<id>.md`, then the addon instruction, and only the caller
      `run_artifact` (`src/cli/instructions.rs:78-95`) turns a resolution error
      into the unknown-id refusal. The observed `givn instructions steps-rust`
      exits 0 without registration. Rewrite item 2 to: the six ids join
      `POLICY_INSTRUCTION_IDS`, so a stack id is a policy instruction id with no
      change context in the known-id list (and is not misread as an artifact),
      while resolution itself already comes from the embedded
      `instructions/<id>.md` fallback.
      ```
      paste: the corrected paragraph, the before/after command evidence
      (`givn instructions steps-rust` exit 0; `givn instructions steps-rustt`
      known list without `steps-rust`), and the source lines cited
      ```

- [ ] Correction 2, `design.md` `## Operator decision: no corpus line cap` and
      `design-review.md` `## Operator decision`. Both enumerate the restored
      `review.md` items as "Step 0 items 0, 8, 9, 11, and 12". The restored
      text carries six items:
      `assets/instructions/review.md:92` (0), `:161` (8), `:166` (9), `:174`
      (10), `:191` (11), `:215` (12). Change both enumerations to
      "Step 0 items 0, 8, 9, 10, 11, and 12".
      ```
      paste: both corrected sentences and the six `review.md` line numbers
      ```

- [ ] Record the corrigenda in `design-review.md` under the operator-decision
      note: two factual corrections, no decision, scenario, command, file path,
      or arc42 statement changed; the delta features and the arc42 chapter
      table are unaffected. If the implementer judges correction 1
      decision-bearing, stop and have `design-review` re-run before batch 2.
      It is not: it restates how an existing code path resolves.
      ```
      paste: the added corrigendum paragraph
      ```

- [ ] Verify: `givn status --change split-stack-instructions` still reports
      `design` and `design-review` present, and
      `givn lint --change split-stack-instructions` still reports the 14 `@wip`
      findings with no parse or shape-blocking finding.
      ```
      paste: both command outputs
      ```

- [ ] COMMIT: `<hash>` -- Batch: `design-corrigenda`

### Batch 2: `reference-check`

The extractor, the one context finding, and the resolution rule. This batch
creates the test-support module the `ise-mapping` batch also consumes; it
changes no production code.

**Files created:** `tests/common/instruction_references.rs`.
**Files modified:** `tests/common/mod.rs` (register the module; it carries no
`#[given]`/`#[when]`/`#[then]`),
`tests/steps/instruction_integrity_steps.rs` (the three scenarios' bindings; the
`named_instruction_ids` helper at `:509` moves into the new module).

**Design constraints (batch-wide, from design.md "Instruction Reference
Check"):**

- `named_instruction_ids(text) -> Vec<(usize, String, String)>`: every direction
  as its 1-based line, that line's text, and the id. A direction is a
  `givn instructions <id>` occurrence inside a code span, or followed by
  `--change`/`--json`, so "givn instructions are resolved" is not a direction.
  One implementation: moved from `instruction_integrity_steps.rs`, not copied.
- `IdFinding { id, source, line, line_text, expected }`.
- `unresolved_instruction_ids(project_root, sources, artifact_ids, change) ->
  Vec<IdFinding>`: resolve each id by running the built binary
  (`env!("CARGO_BIN_EXE_givn")`) in the fixture, never by inferring from the
  asset set: not an override, an addon instruction, and an embedded asset all
  count as resolving.
- Context rule (one-sided): a policy id resolves with no change context, and a
  policy direction that carries `--change` is a finding, because the flag is
  ignored for policy ids (`src/cli/instructions.rs:82-117`). An artifact id
  without `--change` is **not** a finding: it resolves through the
  auto-selected single active change (`src/cli/instructions.rs:119-130`), so
  the six shipped directions (`characterize.md:69`, `design-review.md:18,65`,
  `proposal.md:111`, `review-reverse.md:24`, `specs.md:326`) stay unchanged.
  The finding keeps the direction's line text so a mismatch names the sentence
  it read. A placeholder change token (`<id>`, `<change-id>`, `<CHANGE_ID>`)
  counts as a named context and is substituted with the fixture's active
  change.
- Source set: every manifest artifact id from the fixture's `givn graph --json`
  (with the active change), every policy id = the `assets/instructions/*.md`
  stems of this checkout minus the manifest artifact ids (no context), and
  every `assets/addons/<id>/instructions.md` for an addon the fixture has
  effective. `ise` is addon-bound: skip it when the fixture has `ise` off
  instead of reporting a false finding.
- Read-only: no file is written, staged, or rewritten. `describe(findings)`
  renders source line, line text, id, and expected context.

- [ ] Scenario: `Every instruction id a served instruction names resolves`
  - Design constraints:
    - Given: the shared `a Givn project with an active change` binding, plus a
      new `a Givn project with an active change and the guidance reference
      check available` in `instruction_integrity_steps.rs`: same fixture, and
      the check is the in-process module above (no CLI subcommand exists and
      none is added).
    - When: run `unresolved_instruction_ids` over the design's source set.
    - Then: `every instruction id a served instruction names resolves in the
      context that instruction needs`: no finding has `expected` unmet;
      `every instruction id an effective addon instruction names resolves`:
      the fixture's effective addons are `arc42-docs` and `coverage`, whose
      `instructions.md` files name no direction today, so the leg is asserted
      by running it, not by assuming it; `the check names no unresolved
      instruction id`: `describe(&findings)` is empty.
    - Contract this test uses: `unresolved_instruction_ids` above; the test
      never imports production internals.
    - Expected: the six artifact directions without `--change`
      (`characterize.md:69`, `design-review.md:18,65`, `proposal.md:111`,
      `review-reverse.md:24`, `specs.md:326`) produce **no** finding: the
      operator narrowed the rule on 2026-11-15, so the check resolves them
      through the fixture's single active change. A finding on any of them is
      the plausible-wrong implementation this leg detects; report it instead
      of editing the assets.
    - Step file: `tests/steps/instruction_integrity_steps.rs`.
  - [ ] TESTS: bind the three Thens and the new Given; record the observed
        result.
        ```
        RED: <single-scenario command>
        → paste: the unbound-step failure (the module does not exist yet), and
          after the rule lands, the finding list showing no finding for the six
          artifact directions
        ```
  - [ ] IMPLEMENTATION + GREEN: files created/modified as listed above.
        ```
        paste: the same command; state plainly whether it passed, and which
        findings remain
        ```
  - [ ] REFACTOR: clean up, no behavior change; re-run.
        ```
        paste: fmt/clippy and the re-run
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `reference-check`

- [ ] Scenario: `A dangling stack instruction id fails the reference check by name`
  - Design constraints:
    - Fixture: `write_override(env, "steps", "instructions.md", …)` writes
      `givn/artifacts/steps/instructions.md` (project override, resolution step
      1 in `src/config/cascade.rs`), whose served text carries one direction:
      ``Request `givn instructions steps-rustt` for the Rust mechanics.``
    - When: run the check over the served `steps` guidance only.
    - Then: `the check fails`: at least one finding; `it names "steps-rustt"`:
      the finding's id and line text.
    - Contract: a project override is graded as served, not as the asset.
    - Expected: new component; the plausible-wrong resolver (asset-stem
      inference) reports no finding, so the first bound run fails on this
      assertion.
    - Step file: `tests/steps/instruction_integrity_steps.rs`.
  - [ ] TESTS: bind the Given and the two Thens; record the observed failure.
        ```
        RED: <single-scenario command>
        → paste: the assertion message of the plausible-wrong resolver (no
          finding for `steps-rustt`), labelled as the first run of a new
          component, not as a pre-change historical RED
        ```
  - [ ] IMPLEMENTATION + GREEN: resolve by running the binary, so an unknown id
        fails by name.
        ```
        paste: green run, 1 scenario passed, and the finding text naming
        `steps-rustt` with its source line
        ```
  - [ ] REFACTOR: clean up, no behavior change; re-run.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `reference-check`

- [ ] Scenario: `A policy instruction named with a change context is a finding`
  - Design constraints:
    - Fixture: an override of a policy instruction
      (`write_override(env, "steps", "instructions.md", …)`) whose served text
      carries two directions: `` `givn instructions proposal` `` with no
      `--change`, and `` `givn instructions steps --change demo` ``; `steps` is
      a policy id (it is an `assets/instructions` stem and not a manifest
      artifact).
    - When: the check over the served guidance.
    - Then: `the check fails`; `it names "steps" and the extra change
      context`: the finding's `expected` states that a policy id takes no
      context, because `run_artifact` ignores `--change` for policy ids;
      `it reports no finding for "proposal"`: the artifact direction without
      `--change` resolves through the auto-selected single active change, as
      the CLI does (`src/cli/instructions.rs:119-130`).
    - Contract: the artifact/policy split comes from the fixture's
      `givn graph --json`. A stem-only or context-blind classification passes
      the `steps --change` direction wrongly; a check that still flags an
      artifact id without `--change` fails the `proposal` assertion, so one
      scenario discriminates both plausible-wrong implementations.
    - Step file: `tests/steps/instruction_integrity_steps.rs`.
  - [ ] TESTS: bind the phrase and the two Thens; record the failure against the
        context rule absent.
        ```
        RED: <single-scenario command>
        → paste
        ```
  - [ ] IMPLEMENTATION + GREEN: the one-sided context rule in
        `unresolved_instruction_ids`.
        ```
        paste
        ```
  - [ ] REFACTOR: clean up; re-run.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `reference-check`

- [ ] Batch verification: run all three `--exact` commands plus
      `cargo fmt --all -- --check` and
      `cargo clippy --locked --workspace --all-targets -- -D warnings`.
      ```
      paste: each command and its exit
      ```

The batch commit waits on the Contradiction 1 disposition. Implement the other
three scenarios and record their evidence, but do not commit the batch and do
not check any box until the six corpus directions are either repaired under an
updated `design.md` with a re-passed `design-review`, or the operator narrows
the rule. Both outcomes are reported, neither is applied here.

### Batch 3: `stack-load-conditions`

The two load-condition Outlines: `steps` and `design` each name the six ids with
their runner condition and carry no runner table. Both asset paragraphs landed
in `02126585`, so both scenarios are **existing behavior protected**; the batch
adds the bindings and the assertions only.

**Files modified:** `tests/steps/serve_extended_rules_steps.rs` (bindings plus
the token constants from `## Setup`). **Production:** none.

- [ ] Scenario: `Core step guidance names each stack instruction with its load condition`
  - Design constraints:
    - Outline, 6 rows: `steps-rust`/`cucumber-rs`, `steps-python`/`behave`,
      `steps-node`/`cucumber-js`, `steps-java`/`cucumber-jvm`,
      `steps-go`/`godog`, `steps-ruby`/`cucumber-ruby`.
    - Fixture: the shared `a Givn project with an active change` binding.
    - When: new `the author requests the steps instructions through the Givn
      CLI`: `served_instruction(env, "steps")`, which omits `--change` because
      `steps` is a policy id, into `world.last_output`.
    - Then: `the served text names "<id>" when the change's runner is
      "<runner>"`: bind with a regex capturing both quoted values; the served
      text carries the line ``<id>` when the change's runner is `<runner>``
      (asset `assets/instructions/steps.md:30-35`); `the served text carries no
      bare "<id>" direction without that condition`: no line carries the id
      without that row's runner token on the same line.
    - Presence rule this exercises: one owner per rule with the load condition
      stated (`obtain-effective-guidance` rule 6 and the `support` fragment's
      stack-instruction rule).
    - Evidence: existing behavior protected; no production edit.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS: bind the When and both Thens.
        ```
        RED: not available, existing behavior protected (`02126585`). Probe
        instead: delete one id line from assets/instructions/steps.md, run the
        row, observe the failure naming that id, restore the file.
        paste: probe failure and the restored green run
        ```
  - [ ] IMPLEMENTATION + GREEN: no production change; the assertions above.
        ```
        paste: 6 scenarios (6 passed) for the Outline
        ```
  - [ ] REFACTOR: whitespace-normalization helper only if a fragment straddles
        a line break; no behaviour change.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `stack-load-conditions`

- [ ] Scenario: `Design guidance names each stack instruction with its load condition and carries no runner table`
  - Design constraints:
    - Outline, 6 rows, same id/runner pairs as above.
    - When: new `the author requests the design instructions through the Givn
      CLI`: `givn instructions design --change <active change>`.
    - Then: the served text names the id with the runner condition (asset
      `assets/instructions/design.md:149-154`); `carries no per-runner
      strict-mode mechanism`: no `STRICT_MECHANISMS` member; `carries no
      per-stack single-scenario run command`: no `RUN_COMMANDS` member;
      `still requires the runner's strict-mode proof`: the served text carries
      `strict-mode config must be proven` (`design.md:378`).
    - Universal rule retained: the strict-mode requirement and the
      single-scenario requirement stay as rules pointing at the stack
      instruction (`design.md:25-28,144-155`).
    - Evidence: existing behavior protected; no production edit.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS: bind the When and the four Thens.
        ```
        probe: add `.fail_on_skipped()` to assets/instructions/design.md, run
        the row, observe the absence failure, restore.
        paste: probe failure and the restored green run
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste: 6 scenarios (6 passed)
        ```
  - [ ] REFACTOR.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `stack-load-conditions`

- [ ] Batch verification: both `--exact` commands; fmt; clippy.
      ```
      paste
      ```

### Batch 4: `stack-bodies`

The body Outline: each stack instruction is served only on its own request and
carries its runner's binding, stub, strict mechanism, and command, while the
`steps` text carries none of them. The six asset files landed in `02126585`.

**Files modified:** `tests/steps/serve_extended_rules_steps.rs`.
**Production:** none.

- [ ] Scenario: `Every stack instruction is served only on its own request with its runner body`
  - Design constraints:
    - Outline, 6 rows with `(id, binding, stub, strict, command)`:
      `steps-rust` (`#[given(`, `unimplemented!()`, `.fail_on_skipped()`,
      `cargo test <name> -- --exact`); `steps-python` (`@given(`,
      `raise NotImplementedError("TODO")`, `raise`, `behave -n "<title>"`);
      `steps-node` (`.Given(`, `return "pending";`, `--strict`,
      `cucumber-js --name "<title>"`); `steps-java` (`@Given(`,
      `PendingException("TODO")`, `--strict`,
      `mvn test -Dcucumber.filter.name="<title>"`); `steps-go` (`ctx.Step(`,
      `godog.ErrPending`, `fails by default`, `go test ./... -run TestFeatures`);
      `steps-ruby` (`Given(`, `pending("TODO")`, `fail on pending`,
      `cucumber --name "<title>"`).
    - Given: the active-change fixture plus `the author has requested the steps
      instructions`, which records the `steps` text in the world before the
      stack request.
    - When: new `the author requests "<id>"`: bind with a regex capturing the
      quoted id; `givn instructions <id>` with no change context (a stack id is
      a policy id).
    - Then: `the command exits 0` (shared binding); the served text carries the
      row's binding, stub, strict mechanism, and command (four Thens, each a
      greedy quoted capture as the binding rule above requires); the previously
      served `steps` text carries none of that row's stub, strict mechanism, or
      run command; and carries no per-runner binding instruction
      (`BINDING_BULLETS` absent).
    - Contract: resolution is the embedded `instructions/<id>.md` fallback in
      `src/config/cascade.rs`; the assertion never reads the asset file.
    - Evidence: existing behavior protected; no production edit.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS: bind the Given, the When, and the Thens.
        ```
        probe: delete the `- Strict mode:` bullet from
        assets/instructions/steps-go.md, run the `steps-go` row, observe the
        failure naming the missing mechanism, restore.
        paste: probe failure and the restored green run
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste: 6 scenarios (6 passed)
        ```
  - [ ] REFACTOR: keep the row table in one place per row; no duplication of the
        absence predicates (reuse `STUB_FORMS`, `STRICT_MECHANISMS`,
        `RUN_COMMANDS`, `BINDING_BULLETS`).
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `stack-bodies`

- [ ] Batch verification: the `--exact` command; fmt; clippy.
      ```
      paste
      ```

### Batch 5: `core-absence`

The absence assertions the composition rule depends on: the core instructions
carry no stack body, and `specs`/`review` name no stack instruction. Both hold
in the assets today.

**Files modified:** `tests/steps/serve_extended_rules_steps.rs`.
**Production:** none.

- [ ] Scenario: `Core step guidance carries no stack body`
  - Design constraints:
    - When: request `steps` for the active change.
    - Then: no `STEP_PATHS` member; no `STUB_FORMS` member; no
      `BINDING_BULLETS` member; `still requires one step definition file per
      capability`: the served text carries `One file per capability`
      (`assets/instructions/steps.md:24`); `keeps the archive binding-token
      list for every runner`: the served text carries `Archive reads these` and
      `binding tokens:` and every token of the list (`steps.md:58-62`).
    - Narrowed claim (design-review decision 1): the absence is "no per-runner
      binding instruction", so the archive token list must not trip it. Assert
      the bullet labels and the runner-specific forms, never the bare tokens.
    - Evidence: existing behavior protected.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS.
        ```
        probe: paste the `- Binding form: `#[given(` …` bullet from
        assets/instructions/steps-rust.md into assets/instructions/steps.md, run,
        observe the absence failure, restore.
        paste: probe failure and the restored green run
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste: the runner summary for the scenario (1 scenario passed) and its
        step count
        ```
  - [ ] REFACTOR.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `core-absence`

- [ ] Scenario: `Specs and review guidance carry no stack body`
  - Design constraints:
    - When: new `the author requests the specs and review instructions through
      the Givn CLI`: request both artifact ids with `--change`, keep both
      texts (`world.additional_output` for the second, `last_output` for the
      last run).
    - Then: `neither served text names a stack instruction`: no `STACK_IDS`
      member; `neither served text carries a per-runner mechanism`: no
      `RUNNERS`, `STUB_FORMS`, `STRICT_MECHANISMS`, `RUN_COMMANDS`, or
      `BINDING_BULLETS` member.
    - Interface-typed mentions are allowed: a browser driver name in `review`
      is interface policy, not a stack body (design-review decision 11). The
      predicate is the runner and per-runner mechanism tokens above, never a
      general "no technology word" test.
    - Evidence: existing behavior protected.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS.
        ```
        probe: append `steps-rust` to assets/instructions/review.md, run,
        observe the failure, restore.
        paste
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste
        ```
  - [ ] REFACTOR.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `core-absence`

- [ ] Batch verification: both `--exact` commands; fmt; clippy.
      ```
      paste
      ```

### Batch 6: `tasks-guidance`

The `tasks` and `tasks-reverse` consolidation: both point at `steps` for the
mapping and carry no stack body (`02126585`).

**Files modified:** `tests/steps/serve_extended_rules_steps.rs`.
**Production:** none.

- [ ] Scenario: `The tasks guidance points at the stack mapping and carries no stack body`
  - Design constraints:
    - When: new `the author requests the tasks and reverse-engineering tasks
      instructions through the Givn CLI`: both artifact ids with `--change`.
    - Then: `the tasks text names "steps" as the stack mapping`: carries
      ``givn instructions steps` maps the project's runner to the stack
      instruction`` (`assets/instructions/tasks.md:44`);
      `neither served text carries a per-runner step path, stub, or strict
      mechanism`: no `STEP_PATHS`, `STUB_FORMS`, `STRICT_MECHANISMS`,
      `RUN_COMMANDS`, or `BINDING_BULLETS` member; `both served texts still
      require the strict-mode proof`: each carries `proof-of-strictness`
      (`tasks.md:35`, `tasks-reverse.md:12,14`).
    - Universal wording kept: `{}`, `bare pass`, and `bare return` are the
      empty-body rule, not per-runner stubs; the predicate above excludes them.
    - Evidence: existing behavior protected.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS.
        ```
        probe: remove the `givn instructions steps` sentence from
        assets/instructions/tasks-reverse.md, run, observe the failure, restore.
        paste
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste
        ```
  - [ ] REFACTOR.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `tasks-guidance`

- [ ] Batch verification: the `--exact` command; fmt; clippy.
      ```
      paste
      ```

### Batch 7: `stack-id-registration`

The one production change: the six ids join `POLICY_INSTRUCTION_IDS`. It fixes
the known-id list and makes each stack id a first-class policy id.

**Files modified:** `src/cli/instructions.rs`
(`POLICY_INSTRUCTION_IDS` gains `steps-rust`, `steps-python`, `steps-node`,
`steps-java`, `steps-go`, `steps-ruby`, appended after `ise`; nothing else in
the resolver changes), `tests/steps/serve_extended_rules_steps.rs`.

- [ ] Scenario: `An unknown stack instruction id is refused by name with the known list`
  - Design constraints:
    - When: new `the author requests "steps-rustt"`:
      `givn instructions steps-rustt`.
    - Then: `the command exits non-zero` (shared binding);
      `stderr names "steps-rustt"` (shared `stderr names "{}"` binding);
      `stderr lists the known instruction ids including "steps-rust"`.
    - Contract: `run_artifact` builds the known list from the manifest artifact
      ids plus `POLICY_INSTRUCTION_IDS` (`src/cli/instructions.rs:88-95`); the
      refusal keeps its wording and gains the six ids.
    - Evidence: real behavioral RED. The observed pre-change refusal lists
      `proposal, specs, design, design-review, tasks, review, arc42-docs, start,
      dialog, dialog-examples, challenge, dev-principles, domain-modeling,
      architecture-principles, review-reverse, specs-reverse, tasks-reverse,
      ise` and no stack id.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS: bind the When and the last Then; reuse the two shared Thens.
        ```
        RED: <single-scenario command>
        → paste: the stale known list showing `steps-rust` absent
        ```
  - [ ] IMPLEMENTATION + GREEN: the six ids in `POLICY_INSTRUCTION_IDS`.
        ```
        paste: exit 0, and stderr listing `steps-rust`
        ```
  - [ ] REFACTOR: none expected; the id order is the list order.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `stack-id-registration`

- [ ] Scenario: `The machine-readable stack path carries the same content as the text path`
  - Design constraints:
    - When: new `the author requests "steps-rust" as machine-readable output`:
      `givn instructions steps-rust --json`.
    - Then: `the command exits 0`; `the machine-readable instruction field
      equals the served text`: compare the `instruction` field with
      `without_skill_route(served_instruction(env, "steps-rust"))`, the
      existing contract at `tests/steps/serve_extended_rules_steps.rs:261`;
      also assert `id == "steps-rust"`, `generates` and `requires` are empty
      arrays (D4: a policy id with no lifecycle fields).
    - Contradiction 3 records why the comparison strips the skill route.
    - Evidence: existing behavior protected (the cascade serves the id before
      registration); probe by requesting `steps` in the JSON call instead of
      `steps-rust` and confirm the equality assertion fails.
    - Step file: `tests/steps/serve_extended_rules_steps.rs`.
  - [ ] TESTS.
        ```
        RED: not available, protected. Probe as above; paste the failure and
        the restored green run.
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste: 1 scenario (1 passed), and the parity comparison's byte counts
        ```
  - [ ] REFACTOR.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `stack-id-registration`

- [ ] Batch verification: both `--exact` commands; fmt; clippy; and
      `./target/debug/givn instructions steps-rust` exits 0 in this checkout.
      ```
      paste
      ```

### Batch 8: `ise-mapping`

The `ise` instruction's directions resolve while the addon is effective. The
served text landed in `02126585`; this batch proves it with the new check.

**Files modified:** `tests/steps/instruction_integrity_steps.rs`.
**Production:** none.

- [ ] Scenario: `The ise instruction names only instruction ids that resolve`
  - Design constraints:
    - Fixture: new `a Givn project with the ise addon enabled and the guidance
      reference check available`: `make_init_root`, `givn addons enable ise`
      (its prerequisite `architecture_principles` is on), plus one active
      change (`givn new ise-guidance`) so the two artifact directions resolve.
    - When: new `the maintainer requests the ise instruction and runs the
      reference check over it`: `givn instructions ise` (no change context),
      then resolve every direction in the served text with the check, the
      active change substituted for a placeholder token.
    - Then: the served text names `steps` as the mapping for the change's stack
      mechanics; names `steps-rust` as one stack instruction example; names the
      artifact instruction that owns the change-strategy record (`design`) and
      the one that owns the spec-gap record (`proposal`); every named
      instruction id resolves.
    - Contract: `ise` is addon-bound (`ADDON_BOUND_INSTRUCTION_IDS`); with the
      addon off the request refuses by name and the check skips the id (batch
      2's rule). The asset text is `assets/instructions/ise.md:34-52`.
    - Evidence: existing behavior protected.
    - Step file: `tests/steps/instruction_integrity_steps.rs`.
  - [ ] TESTS.
        ```
        probe: change `steps-rust` to `steps-rustt` in
        assets/instructions/ise.md, run, observe the failure naming it, restore.
        paste
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste: 1 scenario (1 passed), and the resolved direction list
        ```
  - [ ] REFACTOR.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `ise-mapping`

- [ ] Batch verification: the `--exact` command; fmt; clippy.
      ```
      paste
      ```

### Batch 9: `review-obligations`

The review guidance keeps every surviving obligation, one stable marker each.
`review.md` was restored in `e52b4667` (611 lines, obligation set from
`aa50b100`).

**Files modified:** `tests/steps/instruction_integrity_steps.rs`.
**Production:** none.

- [ ] Scenario: `The review guidance keeps every surviving obligation`
  - Design constraints:
    - When: new `the maintainer requests the review instructions through the
      Givn CLI`: `givn instructions review --change <active change>`.
    - Then: nine `the served text carries "<marker>"` assertions with exactly:
      `` `@e2e` tag integrity `` (Step 0 item 0, `review.md:92`), `explicit E2E
      scope selection` and `receipt reading` (item 9, `:166`), `opposite
      classification` (`:383`), `local run command` (`:508`), `` second `@e2e`
      file `` (`:511`), `Design match:` (item 10, `:516`), `Interaction
      coverage maps` (item 11, `:520`), `no fourth bucket` (`:349`).
    - Every marker sits outside an addon conditional block, so the default
      fixture serves all nine; assert on the served text, never the asset.
    - Contract: one owner per rule, and the earlier line-reduction
      consolidation stays withdrawn; no marker is replaced by a count.
    - Evidence: existing behavior protected.
    - Step file: `tests/steps/instruction_integrity_steps.rs`.
  - [ ] TESTS.
        ```
        probe: delete the `local run command` sentence from
        assets/instructions/review.md, run, observe the failure naming that
        marker, restore.
        paste
        ```
  - [ ] IMPLEMENTATION + GREEN.
        ```
        paste: the runner summary for the scenario (1 scenario passed) and its
        step count
        ```
  - [ ] REFACTOR: parameterize the nine markers as one table with the obligation
        each names; no duplicate assertion bodies.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `review-obligations`

- [ ] Batch verification: the `--exact` command; fmt; clippy.
      ```
      paste
      ```

### Batch 10: `adr-register-count`

The `@givn.modified` register scenario: the accepted count moves from 28 to 29.
`ADR-077` is already accepted, so this is existing behavior protected; the
batch adds the `29` binding, deletes the dead `28` binding, and reconciles the
permanent scenario so the whole suite is green before archive (Contradiction 2).

**Files modified (landed in the correction's `chore(adr):` commit):**
`tests/steps/adr_qualification_gate_steps.rs` (`the final register has 29
accepted decisions and no proposed decisions` replaces the dead `the final
register has 28 accepted decisions and no proposed decisions` binding at
`:881`), `givn/specs/fragments/adr-qualification-gate.feature` (line 123: 28 →
29, so the permanent scenario matches the delta exactly and the permanent suite
stays green).

- [ ] Scenario: `The migration preserves independent architecture history across three proposed decisions`
  - Design constraints:
    - Given: the existing `the remediation case carries the distinguishing
      marker "harness jib mandrel nozzle oboe yacht yarn zither"` and
      `the consolidated ADR register and canonical sources are available`
      bindings; the fixture reads `docs/arc42/adr/README.md` and the
      `adr-qualification-gate` migration manifest from the repository, not a
      temp project.
    - When: `a maintainer compares the register with the canonical sources`.
    - Then: `the final register has 29 accepted decisions and no proposed
      decisions`: count lines containing `| accepted |` and `| proposed |` in
      the register; the register carries 29 accepted rows, including
      `ADR-077` at `docs/arc42/adr/README.md:64`, and zero proposed.
    - Rest of the scenario unchanged: the manifest dispositions, the reserved
      ids, the absorbed and superseded records, and the archived replacement
      links re-run their existing bindings.
    - Contract: the change-scoped run executes only this delta scenario, so the
      binding swap and the permanent-feature reconciliation must land in the
      same commit.
    - Evidence: existing behavior protected. **Landed early** in the operator
      correction of 2026-11-15 (the `chore(adr):` commit), which moved the
      permanent scenario to 29 and swapped the binding in the same commit; this
      batch records that commit as the scenario's `COMMIT:` line and runs the
      discrimination probe below. The pre-batch red state is the correction's
      observation: the permanent `--exact` run failed with `left: 29 right: 28`
      before the reconciliation.
    - Step file: `tests/steps/adr_qualification_gate_steps.rs`.
  - [ ] TESTS: add the 29 binding, delete the 28 binding, move the permanent
        feature to 29.
        ```
        RED: not available, protected. Probe: temporarily assert 30 in the new
        binding, run, observe `left: 29 right: 30`, restore 29.
        paste: the pre-batch permanent failure (`left: 29 right: 28`) and the
        probe failure
        ```
  - [ ] IMPLEMENTATION + GREEN: run the delta scenario and the permanent one.
        ```
        paste: both `--exact` runs, 1 scenario (1 passed) each; the delta
        scenario runs against the new binding
        ```
  - [ ] REFACTOR: none; the assertion bodies stay per count because each names a
        different register state.
        ```
        paste
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `adr-register-count`

- [ ] Batch verification: the delta `--exact` command, the permanent `--exact`
      command with `GIVN_FEATURES=givn/specs`; fmt; clippy.
      ```
      paste
      ```

---

## Closeout

- [ ] Every delta phrase is bound in a permanent per-capability step file and no
      delta step copy exists:
      `find givn/changes/split-stack-instructions/specs -name '*steps*'` prints
      nothing; the bindings live in
      `tests/steps/serve_extended_rules_steps.rs`,
      `tests/steps/instruction_integrity_steps.rs`, and
      `tests/steps/adr_qualification_gate_steps.rs`; the new
      `tests/common/instruction_references.rs` carries no
      `#[given]`/`#[when]`/`#[then]`.
      ```
      paste: the find output and the binding counts per file
      ```

- [ ] No empty, no-op, or stubbed step body remains in the files this change
      touched, and no phrase is bound twice.
      ```
      paste: grep for `unimplemented!()|todo!()` and for duplicate phrases
      across tests/steps/
      ```

- [ ] No delta scenario still carries `@wip`, and no `@e2e` tag was added or
      removed: `grep -rn '@wip' givn/changes/split-stack-instructions/specs/`
      prints nothing, and the delta declares no `@e2e` scenario.
      ```
      paste
      ```

- [ ] Contradictions 1 to 3 are dispositioned: Contradiction 1 is closed by the
      operator correction of 2026-11-15 (the context rule was narrowed; no
      shipped asset changes), Contradiction 2 by the permanent-feature
      reconciliation that landed early in the same correction, and
      Contradiction 3 by the `without_skill_route` comparison. None was fixed
      silently.
      ```
      paste: the disposition of each, and the reference-check finding list
      ```

- [ ] Whole-change verification, in this order, each exiting 0 with zero
      failures and zero in-scope skips (`cargo fmt --all -- --check` is the
      check form of `cargo fmt`; `run-tests.sh` runs both):
      ```
      cargo fmt --all -- --check
      cargo clippy --workspace --all-targets -- -D warnings
      cargo nextest run
      ./run-tests.sh --non-e2e
      ./run-tests.sh --e2e
      GIVN_FEATURES=givn/changes/split-stack-instructions/specs ./run-tests.sh --non-e2e
      givn lint --change split-stack-instructions
      ```
      ```
      paste: each command and its exit, the scenario counts per scope, and the
      delta's 30 executed scenarios (15 declarations, three 6-row Outlines)
      ```

- [ ] `givn check tasks --change split-stack-instructions` exits 0 and records
      the change verification receipt; `@wip` is gone, so the receipt covers the
      whole delta.
      ```
      paste: the summary, the scenario and step counts, and the receipt line
      ```

**Done when:** every box is checked with its batch revision, all 30 delta
scenario rows pass in the change and permanent scopes, every contradiction is
dispositioned, and the whole-change checks exit 0.
