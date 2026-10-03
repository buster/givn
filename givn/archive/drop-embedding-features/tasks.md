# Tasks: drop-embedding-features

Runner: `tests/features_runner.rs` (cucumber-rs, `.fail_on_skipped()` at
tests/features_runner.rs:20) via `./run-tests.sh` / `./run-tests.sh --e2e`
(`givn/commands.yaml` already wired). Gate scope is the change delta:
`GIVN_FEATURES=givn/changes/drop-embedding-features/specs`.

## Setup

- [x] Validate the three delta files parse and lint clean:
      `givn lint --change drop-embedding-features` exits 2 (WIP tags only;
      SUBST findings are dispositioned in review.md).
- [x] Commit the already-authored artifacts that carry no scenario work:
      the `@givn.retired` tombstone delta for `release-strict-model-overrides`,
      the arc42 chapter sweep + ADR-069 supersession + register update, the
      `discover-specifications/usecase.md` interaction-row removals.
  - [x] COMMIT: `e93087e`
- [x] Strictness proof is delivered by task 1's RED (undefined steps must exit
      non-zero); e2e subset proof is delivered by task 4 (e2e run count <
      full delta count).

## Scenario: Cargo installation enables every declared feature by default
(release-0-3-0, @givn.modified, non-e2e)

- [x] RED: remove this scenario's `@wip` only. Target it alone:
      `GIVN_FEATURES=givn/changes/drop-embedding-features/specs cargo test
      --test features_runner -- --name "Cargo installation enables every
      declared feature by default"`. The two new steps are undefined →
      must exit non-zero. Paste output.
- [x] GREEN (the engine deletion — compile-forced completeness):
  - [x] `Cargo.toml`: delete the `[features]` block and the optional deps
        (`fastembed`, `hf-hub`, `ndarray`, `ort`, `tokenizers`).
  - [x] Delete `src/spec/retrieval.rs`, `src/spec/remediation_log.rs`;
        reduce `src/spec/mod.rs` to `cluster`, `coverage`, `route`,
        `routing_table`, `tree`.
  - [x] `src/cli/spec.rs`: delete `run_index`, `run_search`, `run_explore`,
        `run_review`, `run_bge_parity` and the `#[cfg]` scaffolding.
  - [x] `src/main.rs`: delete `SpecCommands::{Index, Search, Explore, Review,
        BgeParity}` and their dispatch arms.
  - [x] `src/cli/check.rs`: delete `run_retrieval_gate` + call site.
  - [x] `src/cli/archive.rs`: delete `run_similarity_gate`,
        `refresh_similarity_index` (both cfg variants) + call sites.
  - [x] Collapse `run-tests.sh` (single pass, no feature groups, no stub
        envs, no `tag_present`, no `run_release_strict`; keep the
        scenario-presence guard, the `ready_fixture` pre-build, and the
        `GIVN_FEATURES` scope) and `measure-coverage.sh` (one non-e2e group +
        report, one e2e group + report).
  - [x] Delete `tests/steps/release_strict_overrides_steps.rs` and
        `tests/steps/release_strict_overrides_e2e_steps.rs` (+ `mod` entry),
        `common::run_release_givn`/`release_givn_binary`, the `GivnWorld`
        retrieval fields, the `env_remove("GIVN_SPEC_MAX_RERANK_PAIRS")`
        residue, and the orphaned Then-steps in
        `tests/steps/release_0_3_0_steps.rs`.
  - [x] Reduce the "semantic gate is offline" step in
        `tests/steps/readme_enforcement_steps.rs` to its deterministic
        assertion; strip `cfg!(feature = "spec-retrieval")` +
        `GIVN_SPEC_EMBEDDER` from
        `tests/steps/review_disposition_gates_steps.rs`.
  - [x] Implement the two new steps in
        `tests/steps/release_0_3_0_steps.rs` ("no optional model features
        are declared", reading the Cargo feature metadata like its
        neighbours).
  - [x] `cargo check --workspace --all-targets` and
        `cargo clippy --workspace --all-targets -- -D warnings` clean.
        Targeted run exits 0. Paste output. List every production file.
      Evidence: RED exit 101 (1 step failed: undefined
      "no optional model features are declared"); GREEN exit 0 after the
      engine deletion (4 steps passed); REFACTOR exit 0, fmt+clippy clean.
      Production files: Cargo.toml, src/spec/{mod.rs,retrieval.rs,remediation_log.rs},
      src/cli/{spec.rs,check.rs,archive.rs}, src/main.rs, run-tests.sh,
      measure-coverage.sh, tests/steps/{mod.rs,release_0_3_0_steps.rs,
      readme_enforcement_steps.rs,review_disposition_gates_steps.rs,
      run_execution_e2e_steps.rs}, tests/common/mod.rs.
- [x] REFACTOR: `cargo fmt`; re-run targeted. Paste output.
  - [x] COMMIT: `8296e97` Cargo installation enables every declared feature by default`

## Scenario: Retrieval guidance distinguishes build capabilities
(instruction-integrity, @givn.modified, non-e2e)

- [x] RED: remove this scenario's `@wip` only. Target:
      `GIVN_FEATURES=givn/changes/drop-embedding-features/specs cargo test
      --test features_runner -- --name "Retrieval guidance distinguishes
      build capabilities"`. New steps undefined → non-zero. Evidence: RED
      exit 101 (1 scenario failed, undefined steps).
- [x] GREEN: remove the engine/feature sections from
      `assets/skills/givn-spec/SKILL.md.tmpl` and
      `assets/commands/givn-spec.md`; implement the three absence-assertion
      steps in `tests/steps/instruction_integrity_steps.rs` (replacing the
      old retrieval-guidance bindings). Evidence: GREEN exit 0 (5 steps
      passed); also removed the dead `cli_distinguishes_retrieval_behavior`
      binding in instruction_integrity_e2e_steps.rs territory verified unused
      by any scenario.
- [x] REFACTOR: `cargo fmt`; re-run targeted. Paste output (exit 0).
  - [x] COMMIT: `6e21610`

## Scenario: A consumer invoking a retired engine command is told it does not exist
(release-0-3-0, @givn.added @e2e)

- [x] E2E setup: `./run-tests.sh --e2e` scoped to the delta runs 1 scenario
      vs 3 in the full delta scope — strictly fewer. No servers/containers;
      the real interface is the compiled `givn` binary
      (`CARGO_BIN_EXE_givn`) in a per-scenario TempDir.
- [x] RED: remove this scenario's `@wip` only. Evidence: RED exit 101
      (1 scenario failed, undefined steps).
- [x] GREEN: implemented the e2e steps in
      `tests/steps/release_0_3_0_steps.rs` (reusing the shared
      "the command exits non-zero" binding); assert non-zero exit,
      "unrecognized subcommand 'index'" error, and engine-wording-free
      error output. Interaction row added to
      `givn/specs/maintain-project/usecase.md`. Evidence: e2e run exit 0
      (5 steps passed); full delta scope exit 0 (3 scenarios).
- [x] REFACTOR: `cargo fmt`; re-run targeted (exit 0).
  - [x] COMMIT: `8c1a326`

## Scenario: The migration preserves independent architecture history across three proposed decisions
(adr-qualification-gate, @givn.modified, non-e2e — discovered when the archive
gate's post-merge verify failed on the pre-engine ADR register counts)

- [x] RED: post-merge verify exit 101 — "the final register has 36 accepted
      decisions and four proposed decisions" no longer holds (register now
      25 accepted / 0 proposed after superseding 16 engine records and
      adding ADR-069).
- [x] GREEN: delta scenario modified to the post-engine counts; new bindings
      in `tests/steps/adr_qualification_gate_steps.rs` (register counts,
      absorbed records with the serializer evidence read from the archived
      ADR-059, superseded semantic-engine records checked against
      ADR-069). Targeted run exit 0 (11 steps). Routing recorded as
      `NEW in fragments`; gate support added in commit `72dff8f`.
  - [x] COMMIT: (with archive artifacts)

## Teardown (before review)

- [x] Completion sweep: remaining grep hits are intentional — permanent
      step text pinned by live scenarios ("the semantic gate is offline"),
      absence-assertion bindings, historical wording in two permanent
      scenario titles and the 0.4.0-to-0.5.0 migration docs, and
      `givn/specs/.retired.yaml` tombstones. `cargo check
      --workspace --all-targets` and clippy `-D warnings` clean. Scoped
      delta run: 3 scenarios / 14 steps green (exit 0).
