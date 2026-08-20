# Tasks: semantic-compact-serialization

## Setup

- [x] Configure and prove the strict Gherkin runner before implementation.
  - Runner: `tests/features_runner.rs` with `.fail_on_skipped()`.
  - Verify command: `./run-tests.sh`.
  - E2E command: `./run-tests.sh --e2e`.
  - Create and register `tests/steps/semantic_compact_serialization_steps.rs`.
  - Create and register `tests/steps/semantic_compact_serialization_e2e_steps.rs`.
  - Strictness proof command: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Empty optional sections do not add retrieval content"` while the scenario is still `@wip` or has an unimplemented step.
  - Strictness proof output and non-zero exit: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Empty optional sections do not add retrieval content"` exited non-zero. The runner compiled with `.fail_on_skipped()` and failed on the undefined Given step: `Step doesn't match any function`; summary: `1 feature, 1 scenario (1 failed), 1 step (1 failed)`.
  - Setup files: `tests/steps/semantic_compact_serialization_steps.rs`, `tests/steps/semantic_compact_serialization_e2e_steps.rs`, and `tests/steps/mod.rs`.
  - Setup commit: `125b916` (included in the first implementation commit).

## Scenario: Compact serialization preserves empty and non-empty retrieval boundaries

- [x] RED
  - Remove `@wip` from this scenario only.
  - Bind the consolidated fixture/hash assertion in `tests/steps/semantic_compact_serialization_steps.rs`; the new assertion initially used `unimplemented!()`.
  - Run: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "Compact serialization preserves empty and non-empty retrieval boundaries"`.
  - Captured output: the setup and index steps passed, then the hash step matched and panicked with `not implemented`; summary: `1 feature, 1 scenario (1 failed), 3 steps (2 passed, 1 failed)`.
- [x] GREEN
  - Consolidate the minimal and rich fixtures into one regular scenario and assert both exact hashes and non-empty vectors.
  - Reuse the compact production serializer and direct source tests; no second serializer or production behavior change is added.
  - Compile: `cargo check --features spec-retrieval`.
  - Targeted output: `1 feature, 1 scenario (1 passed), 4 steps (4 passed)`.
  - Production files changed: none; acceptance files changed: `semantic-compact-serialization.feature` and `tests/steps/semantic_compact_serialization_steps.rs`.
- [x] REFACTOR
  - Keep one regular fixture scenario for both empty and rich boundaries while retaining the separate real CLI E2E smoke test.
  - Re-run the targeted command; output remained `1 feature, 1 scenario (1 passed), 4 steps (4 passed)`.
- [ ] COMMIT
  - Commit RED, GREEN, and REFACTOR together with a message referencing `Compact serialization preserves empty and non-empty retrieval boundaries`.
  - Commit hash: pending.

## E2E Setup

- [x] Prove the configured E2E runner is a strict subset of the full runner before the E2E scenario.
  - Local run command: `./run-tests.sh`.
  - E2E run command: `./run-tests.sh --e2e`.
  - No external services or digital twins are required; the real interface is the compiled CLI subprocess.
  - Full-suite scenario count: `334 scenarios (334 passed)` in the E5 run of `./run-tests.sh`.
  - E2E-suite scenario count: `47 scenarios (47 passed)` in the E5 E2E run of `./run-tests.sh --e2e`; the filtered count is strictly lower.
  - Strict E2E proof output: `.fail_on_skipped()` is shared by the runner; the configured E2E command completed all 47 scenarios with zero failures.

## Scenario: A maintainer can index scenarios with complete compact passages

- [x] RED
  - Remove `@wip` from this scenario only.
  - Bind the E2E steps in `tests/steps/semantic_compact_serialization_e2e_steps.rs`; unimplemented steps use `unimplemented!()`.
  - Run: `GIVN_FEATURES=givn/changes/semantic-compact-serialization/specs cargo test --features spec-retrieval --test features_runner -- --name "A maintainer can index scenarios with complete compact passages"`.
  - Expected result: non-zero because the E2E steps are not implemented.
  - Captured output: targeted run matched `tests/steps/semantic_compact_serialization_e2e_steps.rs:8:1`, panicked with `not implemented`, and exited non-zero; summary: `1 feature, 1 scenario (1 failed), 1 step (1 failed)`.
- [x] GREEN
  - Drive the real `givn spec index` subprocess in a temporary project.
  - Assert exit status and stdout as primary CLI evidence; assert the indexed scenario identity as secondary evidence.
  - Run `./run-tests.sh --e2e`; expected result is zero for this scenario.
  - Production files changed: none; the shared serializer was reused. Acceptance file changed: `tests/steps/semantic_compact_serialization_e2e_steps.rs`.
  - `cargo check --features spec-retrieval` succeeded. Targeted output: `1 feature, 1 scenario (1 passed), 5 steps (5 passed)`. The active-change E2E command `GIVN_FEATURES=... ./run-tests.sh --e2e` also passed the scenario; BGE/NLI subsets correctly reported zero in-scope scenarios.
  - Coverage: the configured runner emits no coverage artifact; coverage is unmeasured for this E2E run.
- [x] REFACTOR
  - Keep E2E setup isolated and avoid duplicating regular serializer assertions.
  - Re-run `./run-tests.sh --e2e`; expected result is zero.
  - Captured output: after `cargo fmt --all`, `GIVN_FEATURES=... ./run-tests.sh --e2e` remained green with `1 feature, 1 scenario (1 passed), 5 steps (5 passed)` for the E5 subset and `0 features, 0 scenarios, 0 steps` for the empty BGE/NLI subsets.
- [x] COMMIT: `facf069` — commit RED, GREEN, and REFACTOR together with a message referencing `A maintainer can index scenarios with complete compact passages`.

## Final Verification

- [x] Run `cargo fmt --all -- --check`.
  - Result: passed.
- [x] Run `cargo clippy --workspace --all-targets -- -D warnings`.
  - Result: passed.
- [x] Run `./run-tests.sh`.
  - Result: passed; E5 suite summary `33 features, 334 scenarios (334 passed), 2075 steps (2075 passed)`. All configured BGE/NLI subsets also passed.
- [x] Run `./run-tests.sh --e2e`.
  - Result: passed; E5 E2E suite summary `25 features, 47 scenarios (47 passed), 295 steps (295 passed)`. All configured BGE/NLI subsets also passed.
- [x] Run `./measure-coverage.sh && ./merge-coverages.sh`.
  - Result: passed; merged report `coverage/cobertura-coverage.xml` records `13014/18956` covered lines (`68.65%`) and `0/0` branches (`n/a`). The changed production file `src/spec/retrieval.rs` records `4273/5272` covered lines (`81.05%`). Non-E2E report: `337` scenarios / `2121` steps; E2E report: `60` scenarios / `403` steps.
- [x] Run `givn lint --change semantic-compact-serialization`.
  - Result: `givn lint: 1 file(s) checked -- clean`.
- [x] Record final status and review evidence before archive.
  - Coverage is measured by the configured project scripts; no unclassified coverage gap was introduced by this change.
  - Real E5 index result: `393` indexable permanent scenarios, `0` reused vectors, and `2` remaining token-cap endpoints; real duplicate audit: `395` scanned scenarios and `5560` total candidates.
  - The production review gate is intentionally non-zero: the compact index leaves 2 permanent `E5_TOKEN_CAP` findings and the three new test scenarios produce 3 unresolved delta-to-delta candidates. Per the user decision, this change remains unarchived and does not alter those unrelated/remediation-scope scenarios.
