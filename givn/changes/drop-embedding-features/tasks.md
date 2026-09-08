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
  - [x] COMMIT: see setup commit below
- [ ] Strictness proof is delivered by task 1's RED (undefined steps must exit
      non-zero); e2e subset proof is delivered by task 4 (e2e run count <
      full delta count).

## Scenario: Cargo installation enables every declared feature by default
(release-0-3-0, @givn.modified, non-e2e)

- [ ] RED: remove this scenario's `@wip` only. Target it alone:
      `GIVN_FEATURES=givn/changes/drop-embedding-features/specs cargo test
      --test features_runner -- --name "Cargo installation enables every
      declared feature by default"`. The two new steps are undefined →
      must exit non-zero. Paste output.
- [ ] GREEN (the engine deletion — compile-forced completeness):
  - [ ] `Cargo.toml`: delete the `[features]` block and the optional deps
        (`fastembed`, `hf-hub`, `ndarray`, `ort`, `tokenizers`).
  - [ ] Delete `src/spec/retrieval.rs`, `src/spec/remediation_log.rs`;
        reduce `src/spec/mod.rs` to `cluster`, `coverage`, `route`,
        `routing_table`, `tree`.
  - [ ] `src/cli/spec.rs`: delete `run_index`, `run_search`, `run_explore`,
        `run_review`, `run_bge_parity` and the `#[cfg]` scaffolding.
  - [ ] `src/main.rs`: delete `SpecCommands::{Index, Search, Explore, Review,
        BgeParity}` and their dispatch arms.
  - [ ] `src/cli/check.rs`: delete `run_retrieval_gate` + call site.
  - [ ] `src/cli/archive.rs`: delete `run_similarity_gate`,
        `refresh_similarity_index` (both cfg variants) + call sites.
  - [ ] Collapse `run-tests.sh` (single pass, no feature groups, no stub
        envs, no `tag_present`, no `run_release_strict`; keep the
        scenario-presence guard, the `ready_fixture` pre-build, and the
        `GIVN_FEATURES` scope) and `measure-coverage.sh` (one non-e2e group +
        report, one e2e group + report).
  - [ ] Delete `tests/steps/release_strict_overrides_steps.rs` and
        `tests/steps/release_strict_overrides_e2e_steps.rs` (+ `mod` entry),
        `common::run_release_givn`/`release_givn_binary`, the `GivnWorld`
        retrieval fields, the `env_remove("GIVN_SPEC_MAX_RERANK_PAIRS")`
        residue, and the orphaned Then-steps in
        `tests/steps/release_0_3_0_steps.rs`.
  - [ ] Reduce the "semantic gate is offline" step in
        `tests/steps/readme_enforcement_steps.rs` to its deterministic
        assertion; strip `cfg!(feature = "spec-retrieval")` +
        `GIVN_SPEC_EMBEDDER` from
        `tests/steps/review_disposition_gates_steps.rs`.
  - [ ] Implement the two new steps in
        `tests/steps/release_0_3_0_steps.rs` ("no optional model features
        are declared", reading the Cargo feature metadata like its
        neighbours).
  - [ ] `cargo check --workspace --all-targets` and
        `cargo clippy --workspace --all-targets -- -D warnings` clean.
        Targeted run exits 0. Paste output. List every production file.
- [ ] REFACTOR: `cargo fmt`; re-run targeted. Paste output.
  - [ ] COMMIT: `feat(release-0-3-0): Cargo installation enables every declared feature by default`

## Scenario: Retrieval guidance distinguishes build capabilities
(instruction-integrity, @givn.modified, non-e2e)

- [ ] RED: remove this scenario's `@wip` only. Target:
      `GIVN_FEATURES=givn/changes/drop-embedding-features/specs cargo test
      --test features_runner -- --name "Retrieval guidance distinguishes
      build capabilities"`. New steps undefined → non-zero. Paste output.
- [ ] GREEN: remove the engine/feature sections from
      `assets/skills/givn-spec/SKILL.md.tmpl` and
      `assets/commands/givn-spec.md`; implement the three absence-assertion
      steps in `tests/steps/instruction_integrity_steps.rs` (replacing the
      old retrieval-guidance bindings). Targeted run exits 0. Paste output.
- [ ] REFACTOR: `cargo fmt`; re-run targeted. Paste output.
  - [ ] COMMIT: `feat(instruction-integrity): Retrieval guidance distinguishes build capabilities`

## Scenario: A consumer invoking a retired engine command is told it does not exist
(release-0-3-0, @givn.added @e2e)

- [ ] E2E setup: `./run-tests.sh --e2e` scoped to the delta runs strictly
      fewer scenarios than `./run-tests.sh` scoped to the delta (1 vs 2) —
      record both counts. No servers/containers; the real interface is the
      compiled `givn` binary (`CARGO_BIN_EXE_givn`) in a per-scenario
      TempDir.
- [ ] RED: remove this scenario's `@wip` only. Target:
      `GIVN_FEATURES=givn/changes/drop-embedding-features/specs
      ./run-tests.sh --e2e`. Undefined steps → non-zero. Paste output.
- [ ] GREEN: implement the e2e steps in
      `tests/steps/release_0_3_0_steps.rs`: run `givn spec index` through
      the real binary, assert non-zero exit, unavailable-command error, and
      stdout free of "embedding"/"download" wording. Add the interaction row
      to `givn/specs/maintain-project/usecase.md`. Targeted run exits 0.
      Paste output.
- [ ] REFACTOR: `cargo fmt`; re-run targeted. Paste output.
  - [ ] COMMIT: `test(e2e): A consumer invoking a retired engine command is told it does not exist`

## Teardown (before review)

- [ ] Completion sweep:
      `grep -rl 'retrieval\|GIVN_SPEC\|spec-retrieval' tests/ assets/
      givn/specs/` returns only intentional tombstone wording;
      `cargo check --workspace --all-targets` and clippy `-D warnings`
      clean; unscoped `./run-tests.sh` may be red on the not-yet-merged
      permanent scenarios (retired capability, old guidance scenario) —
      that is the pre-archive interim state the merge resolves; the scoped
      delta run and the full delta scope must be green.
