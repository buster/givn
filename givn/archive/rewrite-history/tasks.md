# Tasks: rewrite-history

Owning contract: use case `rewrite-history`, capability `history-rewrite`.
Confirmed Personas: `developer--primary-user`, `givn-maintainer--owner`.

## Setup

- [x] Change scaffolded; proposal, specs, use-case document, design,
      design-review, arc42 complete.
- [x] Runner strict mode: `.fail_on_skipped()` (existing); the delta suite
      failed RED before the command existed.
- [x] Planning artifacts committed. COMMIT: `7946838b` -- `docs(rewrite-history): plan the change`

## Implementation

- [x] RED/GREEN: the command refuses under `versioned`, on Fossil, on a
      dirty checkout, and without confirmation; it purges reachable history
      through `git-filter-repo`, keeps the files, reports the new revision,
      and names the force-push boundary. Files:
      `src/cli/rewrite_history.rs`, `src/cli/mod.rs`, `src/main.rs`,
      `tests/steps/local_only_rules_steps.rs`, arc42 5/8.
      Targeted runs: 5 scenarios pass.
- [x] COMMIT: `60814310` -- `feat(history-rewrite): purge generated rules from reachable history`

## Verification

- [x] Delta suite — `GIVN_FEATURES=givn/changes/rewrite-history/specs ./run-tests.sh`
      — green. Evidence: `Summary [0.783s] 5 tests run: 5 passed, 0 skipped`.
- [x] `cargo fmt` and `cargo clippy --workspace --all-targets -- -D warnings`
      — clean.
- [x] `givn lint --change rewrite-history` — clean.
- [x] Review artifact complete and signed off.
- [x] Archive the change; the archive verifies both scopes against the
      merged candidate and records the receipt.
