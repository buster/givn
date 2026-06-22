# Review — fix-lint-missing-change-error

Coverage review for the missing-change-id error fix.

- Production code changed: `src/cli/lint.rs` — added change-dir existence
  check in the `--change` branch; returns `anyhow::bail!` if the change
  directory doesn't exist.
- Delta spec: 1 `@givn.modified` scenario (asserts exit 1 + stderr error
  instead of exit 0 + "no .feature files found").

## 1. Coverage

```
src/cli/lint.rs    81 lines  1 missed   98.77%
```

The 1 missed line is the same llvm-cov closing-brace artifact.

## 2. Coverage gap table

| Bucket | Region | Action |
|---|---|---|
| Hard to test (tooling artifact) | `src/cli/lint.rs:65` — closing `}` of `if let` block. | llvm-cov artifact. Justified. |

No dead code. No missing tests.

## 3. Test suite output and result

```
$ cargo test --test features_runner
[Summary]
2 features
31 scenarios (31 passed)
167 steps (167 passed)
```

## 4. Sign-off checklist

- [x] All scenarios pass (test runner exits 0).
- [x] Coverage measured across all test binaries including the Gherkin runner.
- [x] Every coverage gap classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Redundant unit tests removed (none redundant).

REVIEW: PASS
