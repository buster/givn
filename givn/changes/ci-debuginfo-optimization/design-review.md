# Design Review: ci-debuginfo-optimization

## Phase 1: Grilling

A fresh-context grilling subagent walked all required branches against the
proposal, delta spec, design, Cargo.toml, ci.yml, run-tests.sh,
measure-coverage.sh, `src/spec/retrieval.rs`, the ADR register, and the
glossary. Outcomes:

- **Scope (A)**: proposal/spec/design boundary holds; no observable givn
  behaviour lacks a permanent-spec guard. New observables would only
  duplicate existing permanent scenarios.
- **Tech choices (B)**: `debug = "line-tables-only"` correct. Verified
  empirically (test crate in `/tmp`): `cargo llvm-cov` line counts do NOT
  require debuginfo — identical line counts with `line-tables-only` or
  `debug = 0`. `cfg!(debug_assertions)` is controlled by the
  `debug-assertions` option, independent of `debug`; `[profile.test]`
  inherits `[profile.dev]`, so `--test features_runner` gets the new
  profile. `CARGO_INCREMENTAL=0` job-wide chosen over acceptance-scoped
  (earlier steps' blobs would otherwise stay in the peak).
- **Missing scenarios (C)**: none at the command surface. Gap is a missing
  insufficiency plan, not scenarios — captured the cold/PR path sizing and
  an ordered fallback in design.
- **Testability (D)**: the dev-build permanent scenario runs the actual
  dev binary (`CARGO_BIN_EXE_givn`) but is behavior-inert at the
  debuginfo level; the genuine RED-able guard is the full permanent suite's
  exit 0.
- **E2E fidelity (E)**: n/a — zero-scenario change; no inventory entries.
- **Interaction coverage (F)**: consistent — zero inventory, zero matrix
  rows, matching the `semantic-scoring-throughput` precedent.
- **Risk (G)**: cache-transition run can still blow disk (old full-debug
  `target/` restored + new build on top) — mitigated by one-time cache-key
  bump; and `CARGO_INCREMENTAL=0` must be job-wide. Both applied.
- **ADR qualification (H)**: `NONE` — all three choices are lower-level
  build/CI-infra settings; no existing ADR covers them; ADR-004 (single
  static binary) untouched.
- **Arc42 (I)**: independent 12-row re-derivation — all rows `No` (pure
  build/CI-infra change); recorded via the arc42-docs artifact and the ADR
  disposition marker.
- **Ubiquitous language (J)**: consistent — `Development build`,
  `Installed build`, `Fixture request`, `Model-location request` already in
  `docs/arc42/12-glossary.md:113-116`; design introduces no new term.

## User decisions

1. Cache-key transition: **one-time bump** (append `-lt1` to the CI cache
   key) so the first post-change push does not restore stale full-debug
   artifacts on top of the new lean build.
2. `CARGO_INCREMENTAL=0` scope: **job-wide** (`env:` on the `checks` job),
   not acceptance-step-only.

## Phase 2: Hardening

Applied to design.md:

- Corrected the llvm-cov claim: line tables are NOT a coverage
  prerequisite; `measure-coverage.sh` is unconditionally safe.
- Corrected the run-tests.sh zero-scenario guard description: CI runs the
  full permanent suite (no `GIVN_FEATURES`); the guard only fires for a
  scoped code-only tree via `givn check review/tasks`.
- `CARGO_INCREMENTAL=0` job-wide.
- One-time cache-key bump in the Architecture Impact + Decisions.
- Added the cold/PR-path sizing and an ordered fallback
  (insufficiency plan, branch C).
- Added ADR disposition `NONE` with canonical destinations.
- Noted the dev-scenario inertness and named the full-suite exit 0 as the
  primary guard, plus archive-time `measure-coverage.sh` as the sole
  genuine coverage check.

`givn lint --change ci-debuginfo-optimization` exits `0` (clean).

## Notes

- Zero-scenario change; no `@e2e` scenarios exist and none were added or
  removed.

DESIGN-REVIEW: PASS
