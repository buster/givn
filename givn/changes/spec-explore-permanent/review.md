# Review: spec-explore-permanent

## Scope

This review covers only PR 1: `givn spec explore` against permanent specs. The
change contains no `--change` input, semantic-review report, classification,
remediation, duplicate audit, BGE/NLI path, coverage function, or automatic
feature edit.

## Fabrication Audit

### Step definitions and strictness

- `tests/features_runner.rs` uses `.fail_on_skipped()`.
- The strict RED proof ran against the real feature runner and produced
  `1 scenario (1 failed)` with a matched `unimplemented!()` step.
- `tests/steps/spec_explore_permanent_steps.rs` and
  `tests/steps/spec_explore_permanent_e2e_steps.rs` contain no
  `unimplemented!()`, `todo!()`, empty bodies, or no-op assertions after GREEN.
- The e2e steps invoke the compiled `givn` process through the shared process
  seam, not retrieval functions in-process.

### Promised files

All files named in `design.md` exist: CLI/retrieval production modules, the
stdin process helper, the world/module registration, regular and e2e step files,
the delta feature, and all planning artifacts.

### E2E integrity and interaction coverage

The three `@e2e` tags remain present and map one-to-one to the inventory:

| Inventory entry | Scenario | Driver | Primary assertion |
|---|---|---|---|
| `givn spec explore --file <draft.feature>` with human output | File exploration finds a semantically similar permanent scenario | Real `givn` child process with `--file` | Terminal exit, candidate title, embedding score |
| `givn spec explore --stdin` with human output | Stdin exploration returns the same candidates as file exploration | Real `givn` child process with piped stdin | Candidate result lines equal the file invocation |
| `givn spec explore --format json` | JSON exploration reports query index and candidate fields | Real `givn` child process with JSON flag | One parsed JSON document with query/index/result fields |

The full change-only e2e filter reported `3 scenarios (3 passed)` and `20
steps (20 passed)`. The repository e2e command reported `31 scenarios (31
passed)`, strictly below the full repository scope of 208 scenarios.

### Design and Arc42 conformance

The implementation follows the reviewed design:

- `src/main.rs` owns the Clap surface and format enum.
- `src/cli/spec.rs` validates exactly one input source.
- `src/spec/retrieval.rs` parses one complete scenario, refreshes the existing
  content-hash index, ranks all permanent entries, filters only exact file/title
  identity, and renders text or JSON.
- Existing index/search/review code paths retain their output contracts.
- The no-feature process uses a separate Cargo target directory so its binary
  cannot replace the feature-enabled test binary.

Arc42 is enabled. The change marker reports the full twelve-row impact table,
and the durable chapters, ADR-043, risks R-30/R-31, quality scenarios Q-42 to
Q-45, and glossary entries agree with `design.md`. No deployment artifact is
introduced. `ARC42 CONFORMANCE: CLEAN` is blocked only by the repository-wide
baseline test failure recorded below, not by an Arc42 mismatch.

## Verification

| Command | Result |
|---|---|
| `GIVN_SPEC_EMBEDDER=stub GIVN_FEATURES=givn/changes/spec-explore-permanent/specs cargo test --features spec-retrieval --test features_runner` | PASS: 15 scenarios, 81 steps |
| `GIVN_SPEC_EMBEDDER=stub GIVN_FEATURES=givn/changes/spec-explore-permanent/specs cargo test --features spec-retrieval --test features_runner -- --tags @e2e` | PASS: 3 scenarios, 20 steps |
| `cargo fmt --all -- --check` | PASS |
| `cargo check --features spec-retrieval` | PASS |
| `cargo check --no-default-features --locked` | PASS |
| `cargo test --features spec-retrieval --lib` | PASS: 144 tests |
| `cargo test --no-default-features --lib` | PASS: 142 tests |
| `cargo clippy --workspace --all-targets --features spec-retrieval -- -D warnings` | PASS |
| `givn lint --change spec-explore-permanent` | PASS: clean |
| `./run-tests.sh --e2e` | PASS: 31 scenarios |
| `./run-tests.sh` | BASELINE FAILURE: 207/208 scenarios passed |

## Baseline Failure

The full repository runner fails at the pre-existing scenario
`Review passes when a shape match exists only between permanent scenarios` in
`givn/specs/review-disposition-gates/review-disposition-gates.feature`. It is
reproducible in isolation with the base-stack retrieval environment:

```text
error: similarity gate failed: 1 reranker candidate(s) are at or above the fixed raw-logit boundary 3.0000
```

The PR diff does not change `similarity_candidate_pool`, reranking, thresholds,
or review gating. The direct compatibility scenario
`Existing spec review keeps its current behavior` passes through the real CLI,
and the full e2e repository scope passes. Fixing the unrelated pre-existing
review experiment would violate the PR 1 boundary, so it remains a residual
base-stack risk and blocks a clean repository-wide review gate.

## Coverage Measurement

No coverage function, Cobertura branch-coverage policy, or mutation test is
introduced in this slice. Existing coverage scripts were not changed or used as
an acceptance shortcut. Coverage remains a later planned concern as required by
the PR boundary.

## Review Outcome

`REVIEW: FAIL`

Reason: the PR-specific executable suite and all requested compile/lint checks
are green, but the configured repository-wide `./run-tests.sh` remains red on a
pre-existing semantic-review experiment. No PR-1 code was changed to hide or
reinterpret that failure.
