# Review — fix-lint-removed-suppression

Coverage review for the `@givn.removed` suppression fix.

- Production code changed: `src/cli/lint.rs` (line 61: `<=` → `==`).
- Test code changed: `tests/steps/mod.rs` (2 new fixture-builder Given steps).
- Delta spec: 2 `@givn.added` scenarios (no docstrings — avoids merge-engine
  docstring-dropping bug).

## 1. Coverage

Coverage tool: `cargo llvm-cov`. The `features_runner` binary spawns the
compiled `givn` binary as a subprocess; `cargo llvm-cov` instruments the
`givn` bin crate and merges the subprocess profraw.

```
src/cli/lint.rs    79 lines  1 missed   98.73%
```

The 1 missed line is the closing `}` of the `if let` block (llvm-cov
tooling artifact — same as the characterization review).

## 2. Coverage gap table

| Bucket | Region | Action |
|---|---|---|
| Hard to test (tooling artifact) | `src/cli/lint.rs:65` — closing `}` of `if let Some(ref feature)` block. | Same llvm-cov artifact as characterization. Justified. |

No dead code. No missing tests — both the self-suppression and
unrelated-scenarios cases are covered by the two new scenarios.

## 3. Test suite output and result

```
$ cargo test --test features_runner
[Summary]
2 features
30 scenarios (30 passed)
163 steps (163 passed)
```

All GREEN.

## 4. Sign-off checklist

- [x] All scenarios pass (test runner exits 0).
- [x] Coverage measured across all test binaries including the Gherkin runner.
- [x] Every coverage gap classified and resolved (or justified).
- [x] No `@wip` tags remain in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Redundant unit tests removed (none redundant — no unit tests exist
      for lint after the characterization cleanup).

REVIEW: PASS
