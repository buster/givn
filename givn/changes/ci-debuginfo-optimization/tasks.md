# Tasks: ci-debuginfo-optimization

Runner: `./run-tests.sh`; E2E smoke: `./run-tests.sh --e2e`
(`givn/commands.yaml`).

## Scope decision (zero scenarios, user-directed)

Zero-scenario change by decision: the change has no observable command
surface (it is build- and CI-infrastructure only). The regression guards are
the existing permanent scenarios that run in the full suite
(`release-strict-model-overrides`, `coverage-script-workflow`), plus the
archive-time `measure-coverage.sh` run. This mirrors the
`semantic-scoring-throughput` precedent. `run-tests.sh`'s zero-scenario
guard already exists, so the code-only change scope exits 0 without a
cucumber zero-match run.

Interaction-coverage check: no User Interaction Inventory entries remain and
no Interaction Coverage Matrix rows exist — consistent (verified against
`givn/specs/` and `design.md`).

Production surface (design.md):

- `Cargo.toml`: add `[profile.dev] debug = "line-tables-only"`.
- `.github/workflows/ci.yml`: add job-level `CARGO_INCREMENTAL=0` (env on
  the `checks` job); append a one-time `-lt1` marker to the cache key.
- `src/spec/retrieval.rs`: untouched — the
  `ALLOW_SPEC_OVERRIDES = cfg!(debug_assertions)` gating is driven by the
  `debug-assertions` option, not by `debug` (debuginfo); verified in design
  review.

---

## Implementation (code-only, all done)

- [ ] `Cargo.toml`: `[profile.dev] debug = "line-tables-only"` added below
      the existing `[profile.release]` block.
- [ ] `.github/workflows/ci.yml`: `CARGO_INCREMENTAL=0` set at job level
      (`env:` on the `checks` job) so every cargo step skips incremental
      blobs.
- [ ] `.github/workflows/ci.yml`: cache key appended with `-lt1`
      (`givn-cargo-${{ runner.os }}-${{ runner.arch }}-lt1-${{
      hashFiles('Cargo.lock') }}`), a one-time transition marker so the
      first post-change push does not restore stale full-debug artifacts.
- [ ] `cargo check --locked --all-targets` compiles.
- [ ] `cargo clippy --locked --all-targets -- -D warnings` clean.
- [ ] `cargo fmt --all -- --check` clean.
- [ ] `git diff` reviewed: no other file touched; `src/` unchanged.

## Gates

- [ ] `./run-tests.sh` (full suite, dev profile) exits 0.
- [ ] `./run-tests.sh --e2e` exits 0 (includes the release `@release-proof`
      path).
- [ ] `measure-coverage.sh` produces line coverage (Cobertura report with
      line data) under the new profile.
- [ ] `givn lint` (all specs incl. permanent) passes.
- [ ] `givn check tasks --change ci-debuginfo-optimization` passes.
- [ ] `givn check review --change ci-debuginfo-optimization` passes.
- [ ] `givn status --change ci-debuginfo-optimization` shows every
      apply.requires artifact complete.

## Commits

Code-only change; one atomic implementation commit:

- [ ] COMMIT: <HASH> -- implementation (dev profile line-tables-only,
  job-wide CARGO_INCREMENTAL=0, one-time cache-key bump in ci.yml). No
  per-scenario commits exist because the change carries zero scenarios by
  decision.

---

## Archive

- [ ] `givn archive --change ci-debuginfo-optimization` succeeds with no
      remaining `@wip` and no unchecked boxes.
