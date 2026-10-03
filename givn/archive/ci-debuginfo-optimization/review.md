# Review: ci-debuginfo-optimization

Zero-scenario, code-only change (build profile + CI workflow). The review
checklist below is run in full; the fabrication-audit branches that target
step definitions and `@e2e` scenarios are n/a because the change carries no
scenarios.

## Step 0: Fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity in delta | Clean — delta has no scenarios and never had `@e2e` tags (`rg "@e2e"` → none; feature file is a rationale comment + heading). |
| 1 | Empty/no-op step bodies | n/a — no step files exist (no scenarios). |
| 2 | Checked `[x]` tasks have commits touching production source | Clean — commit `be4811f` modifies `Cargo.toml` and `.github/workflows/ci.yml` (production build/CI surface), not spec-only. No task is checked off without a real commit. |
| 3 | Components design promised exist | Clean — design promises only: `Cargo.toml [profile.dev]`, `ci.yml` job env + cache key. All three present in the diff. |
| 4 | Strict-mode proof-of-strictness | n/a — zero scenarios; no cucumber step-definition surface to prove. The runner (`./run-tests.sh`) is already strict (`.fail_on_skipped()`, `features_runner.rs`). |
| 5 | Downgraded `@e2e` (repo-only Then) | n/a — no `@e2e` scenarios. |
| 6 | Browser-vs-HTTP driver | n/a — no UI capability. |
| 7 | Weaker impl shadowing stronger | n/a — no step implementation at all. |
| 8 | Excess `@e2e` on same action | n/a — no `@e2e`. |
| 9 | Local run command starts full stack | Clean — `verify.command` is `./run-tests.sh`; no external services or digital twins for this change. |
| 10 | `verify.command` vs `verify.e2e_command` isolation | Clean — they are distinct strings (`./run-tests.sh` vs `./run-tests.sh --e2e`); not applicable to this change's absence of `@e2e` scenarios. |
| 11 | Implementation vs design.md deviation | Clean — matches design exactly: `[profile.dev] debug = "line-tables-only"`, job-wide `CARGO_INCREMENTAL=0`, one-time `-lt1` cache-key **and** restore-prefix bump. No silent deviation. |
| 12 | Reopened tasks | n/a — no findings. |
| 13 | Interaction coverage cross-reference | n/a — zero inventory entries (delta has no `# User Interaction Inventory:` block), zero matrix rows, consistent with the `semantic-scoring-throughput` precedent. |
| 14 | Coverage measurement validity | Valid — `measure-coverage.sh` ran under the new profile: produced non-e2e and e2e Cobertura reports with line rates 0.828 / 0.711, non-zero production coverage, distinct reports (archive gate satisfied). Instrumentation is `cargo llvm-cov`; merge is the configured command. |

**Fabrication audit: CLEAN.**

## Arc42 implementation conformance

| Durable/claim fact | Design | Task | Implementation evidence | Match |
|---|---|---|---|---|
| `[profile.dev] debug = "line-tables-only"` | design.md "Technical Approach" (1) | tasks.md impl bullet 1 | `Cargo.toml` profile block | ✓ |
| Job-wide `CARGO_INCREMENTAL=0` | design.md (2) | tasks.md impl bullet 2 | `ci.yml` `env:` on `checks` job | ✓ |
| One-time `-lt1` cache-key bump (+ restore-prefix) | design.md (3) + Decisions | tasks.md impl bullet 3 | `ci.yml` key + restore-keys | ✓ |
| `src/spec/retrieval.rs` untouched | design.md "Why gating survives" | tasks.md scope | `git diff` shows no `src/` change | ✓ |
| ADR disposition `NONE` | arc42.md ch. 09 "No" | design.md Decisions | no ADR file created; durable `docs/arc42/` unchanged | ✓ |

**ARC42 CONFORMANCE: CLEAN**

## Ubiquitous language conformance

The change reuses existing glossary terms (`Development build`, `Installed
build` — `docs/arc42/12-glossary.md:113-114`). No new domain terms
introduced; no term contradicts the glossary.

**UBIQUITOUS LANGUAGE: CLEAN**

## Deterministic gate dispositions

The change adds no scenarios (zero delta ops: 0 added, 0 modified, 0
removed), so there are no shape matches, subsets, or long scenarios to
dispose.

## Coverage gate (completeness)

The change touches no production source (`src/` untouched; only build
profile + CI workflow). There is therefore no new uncovered production line
to classify. The retained build behaviour is covered by the existing
permanent suite (exit 0, both runners) and the archive-time
`measure-coverage.sh` (line rates 0.828 / 0.711).

## Sign-off checklist

- [x] `./run-tests.sh` passes (exit 0, full suite on dev profile).
- [x] `./run-tests.sh --e2e` passes (exit 0, incl. `@release-proof`).
- [x] Coverage measurement valid and freshly produced by `measure-coverage.sh`.
- [x] Fabrication audit clean.
- [x] Arc42 conformance clean.
- [x] Ubiquitous language clean.
- [x] No overlap dispositions required (zero scenarios).
- [x] Commit evidence recorded in tasks.md (`be4811f`).

REVIEW: PASS
