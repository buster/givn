# Review — fix-lint-exit-handling

Coverage review for the exit-handling refactor.

- Production code changed:
  - `src/cli/lint.rs` — `run()` returns `Result<i32>` (exit code) instead of
    calling `std::process::exit` directly. Early returns use `Ok(0)`.
  - `src/main.rs` — dispatch maps non-zero code to `std::process::exit`.
- Delta spec: 1 `@givn.added` scenario (regression guard for exit-code
  contract).

## 1. Coverage

```
src/cli/lint.rs    82 lines  0 missed   100.00%
src/main.rs        184 lines  2 missed   98.91%
```

The refactor eliminated the `std::process::exit` calls inside `run()`,
which were previously unreachable-after-exit lines. `src/cli/lint.rs` is
now at 100% line coverage.

## 2. Coverage gap table

| Bucket | Region | Action |
|---|---|---|
| Hard to test (tooling artifact) | `src/main.rs` — `std::process::exit` calls in `main()` itself. | Cannot test process exit from within the process. Justified. |

No dead code. No missing tests.

## 3. Test suite output and result

```
$ cargo test --test features_runner
[Summary]
2 features
31 scenarios (31 passed)
168 steps (168 passed)
```

All GREEN — behaviour-preservation invariant confirmed. All exit codes
(0, 1, 2) are unchanged after the refactor.

## 4. Sign-off checklist

- [x] All scenarios pass (test runner exits 0).
- [x] Coverage measured across all test binaries including the Gherkin runner.
- [x] Every coverage gap classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Redundant unit tests removed (none redundant).
- [x] Behaviour-preservation invariant confirmed (all exit codes unchanged).

REVIEW: PASS
