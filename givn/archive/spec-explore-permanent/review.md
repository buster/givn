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
baseline alignment that was fixed below, not by an Arc42 mismatch.

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
| `./run-tests.sh` | PASS: 208 scenarios, 1174 steps |

## Baseline Alignment

The existing permanent scenario `Review passes when a shape match exists only
between permanent scenarios` still exercised the current cross-encoder review
path but asserted the pre-reranker behavior (`exit 0`). The active
cross-encoder delta already defined the intended behavior: the permanent pair
is reported as a similarity candidate and the review exits non-zero. The
permanent executable spec was aligned with that existing decision by asserting:

```text
exit non-zero
stderr contains "similarity gate failed"
```

No reranker implementation, threshold, or PR-1 exploration behavior changed.
The targeted scenario now passes, and the full repository runner is green.

## Coverage Measurement

No coverage function, Cobertura branch-coverage policy, or mutation test is
introduced in this slice. Existing coverage scripts were not changed or used as
an acceptance shortcut. Coverage remains a later planned concern as required by
the PR boundary.

## Review Outcome

`REVIEW: PASS`

The PR-specific executable suite, repository-wide runner, compile/lint checks,
and e2e runner are green. The permanent review spec now matches the existing
cross-encoder behavior without expanding PR-1's implementation scope.
