# Design: ci-debuginfo-optimization

## Technical Approach

Shrink the CI debug-build disk footprint without changing any observable
givn behaviour. Three independent changes:

1. **`Cargo.toml` dev profile** — stop emitting full debug information:
   ```toml
   [profile.dev]
   debug = "line-tables-only"
   ```
   The dev profile is what `cargo test` (lib + `features_runner`) and the CI
   acceptance step build. Full `debug = 2` (default) is what inflates every
   test binary (~220 MB each with `debuginfo`), every `libgivn` rlib
   (~84 MB), and the incremental `.bin` blobs. `"line-tables-only"` keeps
   usable backtraces while dropping the variable- and type-level records
   that dominate artifact size.

   Note on coverage: source-based coverage (`cargo llvm-cov`) does **not**
   require debug info at all — line counts come from the
   `-C instrument-coverage` mapping, and the same line counts are produced
   with `debug = "line-tables-only"` or `debug = 0` (verified with a test
   crate in `/tmp`). `"line-tables-only"` is chosen for usable backtraces,
   not as a coverage prerequisite. The archive coverage gate
   (`measure-coverage.sh`) is therefore unconditionally safe under this
   profile.

2. **CI workflow, job-wide `CARGO_INCREMENTAL=0`** — the `checks` job in
   `.github/workflows/ci.yml` sets `env: CARGO_INCREMENTAL: "0"`. Every
   cargo step in the job (`cargo check`, `clippy`, `test --lib`, and both
   `./run-tests.sh` acceptance invocations) then skips incremental
   compilation. Incremental `.bin` blobs (210 MB each locally, 16 GB in
   total) are pure overhead on a runner whose `target/` is never persisted;
   scoping it only to the acceptance step would still leave the earlier
   steps' blobs on disk at the acceptance-step peak.

3. **CI cache-key bump (one-time)** — the cache key is
   `givn-cargo-${{ runner.os }}-${{ runner.arch }}-${{ hashFiles('Cargo.lock') }}`
   (`.github/workflows/ci.yml:33`). This change touches `Cargo.toml
   [profile.dev]`, not `Cargo.lock`, so without a key change the next push
   would restore the stale full-debug `target/` *and* build the new light
   artifacts on top (peak ≈ old + new). Appending a one-time marker
   (e.g. `-lt1`) to the key makes the first post-change push build from
   scratch and cache the lean `target/` from then on.

### Why the debug/installed gating survives

The development-vs-installed distinction is
`ALLOW_SPEC_OVERRIDES: bool = cfg!(debug_assertions)`
(`src/spec/retrieval.rs:106`). `debug_assertions` is controlled by the
profile's `debug-assertions` option (default true in dev/test, false in
release), **not** by `debug` (debuginfo). Changing `[profile.dev] debug`
therefore leaves the gating bit-compiler-identical; the release profile
(`[profile.release]`) is untouched.

### Why the acceptance suite fits after the change

On the failing run (job 99085006761) the acceptance step restored a ~2.5 GB
`target/` cache, then linked `features_runner` and the `givn` bin for every
feature combination in `run-tests.sh` (16 link groups across the e2e path)
with full debuginfo, exhausting the runner's disk (`No space left on
device`, 56 MB free). Dropping to line-tables-only shrinks each linked
artifact by a factor of 4–10 and disables the incremental output entirely,
bringing the same work well inside the runner budget.

Cold/PR path: PRs restore no cache (`.github/workflows/ci.yml:25`), so the
cold build is a dependency compile plus 16 lean link sets — smaller than
the current cache-restored push path was. If the change is ever still
insufficient, the fallback is ordered: job-wide `CARGO_INCREMENTAL=0`
(already included) and a larger `--features` deduplication in `run-tests.sh`
(that would be a separate change).

## Architecture Impact

- `Cargo.toml`: add `[profile.dev] debug = "line-tables-only"`.
- `.github/workflows/ci.yml`: add job-level `CARGO_INCREMENTAL=0`; append
  a one-time marker to the cache key.
- `src/spec/retrieval.rs`: untouched. No code change.
- No step definitions: this change adds no scenarios (see below).

## Verifiability (zero-scenario decision)

No new scenarios by decision: the change has no observable command surface.

The genuine regression guards, in order of strength:

1. **Full permanent suite green on the dev profile.** CI runs
   `./run-tests.sh` and `./run-tests.sh --e2e` with `GIVN_FEATURES` unset,
   so `FEATURES_PATH=givn/specs` and the **entire permanent suite runs via
   cucumber** — including building and running the dev-profile binaries.
   If `debug = "line-tables-only"` broke compilation, `debug_assertions`,
   or the development/installed distinction (`src/spec/retrieval.rs:106`),
   a permanent scenario fails. The release `@release-proof` path also runs
   in `--e2e`, pinning the release profile's distinct behaviour.
   (`run-tests.sh`'s zero-scenario guard only fires for a scoped
   `GIVN_FEATURES` tree with no scenarios — that is the code-only change
   scope under `givn check review/tasks`, not the CI invocation.)

2. **The development-build permanent scenario**
   (`givn/specs/release-strict-model-overrides/`) runs the built dev binary
   via `CARGO_BIN_EXE_givn`, so it is compiled with the new profile. It is
   behavior-inert at the debuginfo level (debuginfo has no runtime-observable
   effect on stdout/stderr/exit), but it does fail RED if the profile change
   ever collapsed the development build to release semantics.

3. **Archive-time coverage measurement.** The archive gate runs
   `measure-coverage.sh` (`cargo llvm-cov`), which is the sole genuine
   check of coverage under the new profile; it remains line-accurate as
   verified above. CI does not run coverage measurement; that is out of
   scope and unchanged.

This mirrors the zero-scenario precedent of `semantic-scoring-throughput`.

Local verification before archive:

- `cargo check` / `cargo clippy --workspace --all-targets -- -D warnings`
  clean;
- `./run-tests.sh` and `./run-tests.sh --e2e` exit 0 (full suite);
- `measure-coverage.sh` still produces a Cobertura report with line data.

## Interaction Coverage Matrix

None. No observable user interaction is asserted; the retained behaviour is
verified by the existing permanent scenarios and the local suite.

## Decisions

- `"line-tables-only"` instead of `debug = 0`: keeps usable backtraces and
  gives the largest size reduction versus full debug; coverage is
  unaffected either way (verified).
- `CARGO_INCREMENTAL=0` job-wide in CI (not in `Cargo.toml`), keeping local
  dev experience unchanged.
- One-time cache-key bump (`-lt1`) to avoid restoring stale full-debug
  artifacts on the first post-change push.
- Zero scenario coverage (user decision), matching
  `semantic-scoring-throughput`.
- ADR disposition: `NONE`. All three choices are lower-level
  build/CI-infra settings — reversible, no structural boundary, no external
  contract; no existing ADR covers debuginfo or CI profiles (checked
  `docs/arc42/adr/`); ADR-004 (single static binary) governs the release
  binary shape and is untouched. Canonical destinations: `Cargo.toml
  [profile.dev]`, `.github/workflows/ci.yml`, and this design together with
  the spec's explanatory comment.
