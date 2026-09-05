# Design: harness-binary-stability

## Problem mechanics (why the harness needs the guarantee)

The run-execution scenarios locate the ready-fixture process by scanning the
directory the current runner executable lives in; when the scan finds nothing,
they build it on demand. That build compiles the whole workspace with the
default (feature-free) feature set into the shared target directory, replacing
the givn binary the suite's own invocation just compiled with its scoped
feature set. Cucumber workers keep spawning that binary by absolute path for
the rest of the run, so every semantic scenario scheduled after the swap sees
a feature-free build — not only the two first observed, but every scenario
that expects the retrieval gate to exist.

Three properties make the failure deterministic on CI and invisible locally:

- Main-branch runs restore a cargo cache keyed on `Cargo.lock`
  (.github/workflows/ci.yml:30-37); the cached `target/` predates the fixture
  binary, and on an exact-key hit actions/cache skips the save, so the cache
  never gains the fixture while the lockfile is unchanged.
- Pull-request runs restore no cache at all (ci.yml:27) — every PR run is
  fully cold, which is the determinism source there.
- A warm local tree already contains a fixture binary, so the scan succeeds
  and no build runs.

## Solution

Prepare the fixture binary before the first scenario of any suite invocation,
so the on-demand path never fires in a prepared environment. The on-demand
build stays as a fallback (unscoped, ad-hoc runner invocations still work).

### run-tests.sh

One command inserted after the zero-scenario early-exit guard (closes at
run-tests.sh:74) and before the suite dispatch (run-tests.sh:76), so a
code-only scoped run still skips it:

```sh
cargo build --locked --test ready_fixture
```

- Default features are correct here: nothing executes the fixture during
  preparation, and each suite invocation immediately after rebuilds the givn
  binary with its own feature set before its runner starts (this rebuild
  already happens today between suites). The preparation adds one extra
  default-features lib+bin compile per script invocation: warm-incremental
  ~8 s, but a fully cold tree (every PR run) pays a full compile — accepted,
  because the alternative is a mid-suite binary swap.
- `--locked` matches every CI cargo step: a lockfile drift fails fast at
  script start instead of rewriting `Cargo.lock` mid-suite.
- Ten of the eleven cargo invocations in the dispatch scan for the fixture
  (the `cargo test --lib -- release_strict` tail does not run cucumber at
  all); the built fixture lands in the same `deps/` directory those runners
  scan, so every arm finds it.

### measure-coverage.sh

The coverage replay has the identical race against its own target directory
(cargo-llvm-cov keeps a separate one): a cold replay target makes the first
run-execution scenario build the fixture with plain `cargo build`, swapping
the instrumented, feature-scoped givn binary for a plain feature-free one.
One command at the top of the script, **before** `cargo llvm-cov clean
--profraw-only`, so the fixture run's own profraw is discarded before the
non-e2e groups and never lands in the must-differ report pair:

```sh
cargo llvm-cov --no-report --locked --test ready_fixture
```

- `--no-report` builds the fixture with the same instrumentation as the
  replay groups and runs it once with no arguments (the fixture exits 0
  immediately by contract, so this terminates deterministically).
- It links none of the production library (std-only harness, Cargo.toml
  `harness = false`), so the extra profraw contributes no production
  coverage noise even if it survived — discarding it first is hygiene.
- The replay target dir is cargo-llvm-cov's choice, not the script's; using
  the same wrapper keeps the fixture in whatever directory the replay uses
  (verified: the fixture lands in `target/llvm-cov-target/debug/deps/`,
  exactly where the replay runners scan).

## Alternatives considered and rejected

- **In-process caching of the fixture path** — does not fix the swap: the
  first cold scan still triggers the build that replaces the binary; a cache
  only prevents repeats.
- **Dedicated fixture target directory** — prevents the shared-dir swap by
  construction, but breaks the exe-dir-relative scan (needs a new lookup
  contract), adds a second unit-graph build, and leaves the on-demand
  fallback dangerous for ad-hoc runs. More churn than two one-line
  prebuilds.
- **Feature-preserving on-demand build** — makes the swap semantically
  harmless, but requires plumbing each arm's feature set into the runner
  process (eleven duplicated strings), does not fix the llvm-cov replay, and
  keeps a cargo invocation mid-suite (slow, and it panics inside a scenario
  on failure per tests/steps/run_execution_support.rs:61). The prebuild moves
  the failure to script start under `set -eu` — which is also the correct
  error-path behaviour: a failed preparation aborts the script before any
  scenario runs.

## Verifiability (no-delta decision)

The change produces no delta spec and creates or extends no capability: the
suite cannot execute its own harness — a scenario that ran run-tests.sh from
inside the suite would recurse — and no givn command observes the
preparation. This mirrors the no-delta precedent of migrate-0-4-0-to-0-5-0
and group-specs-by-use-case. The permanent regression guards are the
scenarios that exposed the defect: the development/installed build
distinction (givn/specs/spec-discovery/release-strict-model-overrides.feature)
and the review disposition gates
(givn/specs/verification/review-disposition-gates.feature).

Cold-tree reproduction (manual, deliberate, repeatable):

```sh
rm -f target/debug/deps/ready_fixture-* target/llvm-cov-target/debug/deps/ready_fixture-*
./run-tests.sh            # before the fix: semantic scenarios fail after the mid-suite swap
./run-tests.sh            # after the fix: all pass, no mid-suite build
./measure-coverage.sh     # after the fix: replay completes with no mid-replay build
```

## Interaction Coverage Matrix

None — the change carries no delta spec, hence zero scenarios and zero
interaction-inventory entries (zero/zero parity).

## Data model changes

None.

## Step definitions

None — no delta capability exists.

## Test runner

Permanent regression guards (no new scenarios):

```sh
./run-tests.sh
./run-tests.sh --e2e
```

These execute the `.feature` files under `givn/specs/` and the change's
`specs/` (scoped runs via `GIVN_FEATURES`).

## Single-scenario run command

Not applicable — no delta scenarios. The regression guards run as whole
suites.

## Strict mode

`cucumber-rs` with `.fail_on_skipped()` on the builder
(tests/features_runner.rs) — unchanged; the change adds no steps.

## Version freshness

No new runtime, framework, or library. `cargo` and `cargo-llvm-cov` are the
versions already pinned by the repository lockfile and CI image; no version
decision is made here.

## E2E smoke test infrastructure

Not applicable — no `@e2e` scenarios; no new runner command, step file, or
infrastructure.

## Local runnability

The system under test is the Rust workspace plus its own repo-owned scripts.
`./run-tests.sh` remains the one-command local run (already documented in the
README and `givn/commands.yaml`). No containers, services, or third-party
dependencies are involved; nothing talks to a network outside the machine.

## Residual risks

- `scan_for_fixture` matches any executable named `ready_fixture*` in the
  scan directory, so an ad-hoc run can pick a stale-variant binary by
  directory order. All variants are functionally identical (std-only
  fixture), the accumulation is pre-existing, and this fix neither worsens
  nor relies on it.
- The release-build fixtures' on-demand release build remains a mid-suite
  cargo invocation, but it is guarded by a run-once latch and the first
  scenario blocks until it completes (tests/common/mod.rs:489-512); it only
  ever touches `target/release`, never the suite's debug binary. Out of scope.
