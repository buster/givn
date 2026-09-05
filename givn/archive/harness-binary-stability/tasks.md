# Tasks: harness-binary-stability

No-delta change (precedent: migrate-0-4-0-to-0-5-0): the runner, strict-mode
configuration, and step-definition skeleton already exist
(tests/features_runner.rs, `.fail_on_skipped()`); no scenarios and no step
files are added. Tasks are the two harness preparations plus cold-tree
verification.

- [x] Task 1: run-tests.sh prepares the fixture process before suite dispatch (after the zero-scenario guard, before the dispatch case): `cargo build --locked --test ready_fixture`. A code-only scoped run still exits before it.
  - RED/GREEN: not applicable (no delta scenarios). Verification is the cold-tree reproduction in Task 3.
  - [x] COMMIT: `ae59ad5`
- [x] Task 2: measure-coverage.sh prepares the instrumented fixture process before the profraw clean and the replay groups: `cargo llvm-cov --no-report --locked --test ready_fixture`.
  - RED/GREEN: not applicable (no delta scenarios). The command form, flag acceptance, exit 0, and landing zone (target/llvm-cov-target/debug/deps/) were verified live during design review.
  - [x] COMMIT: `d4dfa5f`
- [x] Task 3: cold-tree verification — remove the fixture binaries, run the default suite and the e2e suites, confirm every scenario passes with no mid-suite build; then confirm a second run stays warm and green.
  - [x] `rm -f target/debug/deps/ready_fixture-* target/llvm-cov-target/debug/deps/ready_fixture-*` (only 0-byte rmeta metadata remained; scan requires exec bits)
  - [x] `./run-tests.sh` → 363 scenarios (363 passed), exit 0
  - [x] `./run-tests.sh --e2e` → 69 + 5 scenarios passed, exit 0
  - [x] second `./run-tests.sh` → 363 scenarios (363 passed), exit 0
  - [x] COMMIT: `d4dfa5f` (Task 3 verification evidence rides on the Task 2 commit; no source change)

DESIGN-REVIEW: PASS marker lives in design-review.md.
