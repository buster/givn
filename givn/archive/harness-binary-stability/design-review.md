# Design Review: harness-binary-stability

## Phase 1: Grilling

Fresh-context grilling subagent (task ses_f8d41a1c3ffeQ3qT28tjgyv3PJ) read the
proposal, delta spec, and design, and verified every claim against the code
and by live execution of the two proposed prebuild commands.

### Verified claims (evidence)

- Swap mechanism confirmed: `cargo build --test ready_fixture` rebuilds the
  givn lib and bin (default features) before linking the fixture;
  `CARGO_BIN_EXE_givn` subprocesses (tests/common/mod.rs:414, :446) spawn
  exactly that path, so a mid-suite on-demand build
  (tests/steps/run_execution_support.rs:55-61) swaps it.
- Landing zones correct for both runners: plain prebuild lands in
  `target/debug/deps/` (what plain runners scan); `cargo llvm-cov --no-report
  --test ready_fixture` accepted the flags, ran the fixture (exit 0), and
  landed it in `target/llvm-cov-target/debug/deps/` — where the replay
  runners scan.
- Fixture is inert and lib-free: no args exits 0 (tests/ready_fixture.rs:26-29);
  std + libc imports only; `harness = false` (Cargo.toml:57-59).
- Both permanent guards genuinely fail under the swap: `givn spec index` and
  the `check review` retrieval gate bail "spec retrieval is unavailable"
  without the feature (src/cli/spec.rs:45-51, src/spec/retrieval.rs:1857-1863).
- Release on-demand build cannot swap under the suite (run-once latch,
  first scenario blocks, targets only `target/release`;
  tests/common/mod.rs:489-512).
- CI cache claim holds: exact-key hit skips the save while `Cargo.lock` is
  unchanged (ci.yml:30-37); PR runs restore no cache at all (ci.yml:27).
- Insertion point correct: zero-scenario guard closes at run-tests.sh:74,
  dispatch begins at :76.

### Findings and resolutions

1. **measure-coverage.sh insertion point unspecified** — resolved: prebuild
   goes before `cargo llvm-cov clean --profraw-only` so the fixture run's
   profraw is discarded before the non-e2e groups.
2. **Cold-tree reproduction ambiguous** — resolved: exact removal command
   (`rm -f target/debug/deps/ready_fixture-*
   target/llvm-cov-target/debug/deps/ready_fixture-*`) plus the coverage
   replay reproduction added to design.md.
3. **CI account incomplete** — resolved: design now states PR runs are fully
   cold (no cache restore at all) and corrects the ~8 s figure to
   warm-incremental only; PR-cold pays a full default-features compile.
4. **`--locked` missing from prebuild snippets** — resolved: both snippets
   use `--locked`, matching every CI cargo step.
5. **Zero-scenario delta conflicts with the lint contract** — resolved by
   dropping the delta spec entirely: the change carries no specs directory
   (no-delta precedent: migrate-0-4-0-to-0-5-0, group-specs-by-use-case).
   `givn lint --change` exits 0 ("no .feature files found"); the routing
   gate accepts an empty delta (check_routing_consistency returns Ok before
   consulting the table); the archive flow tolerates a change without delta
   specs. No capability is created, so no retirement sweep is needed and no
   zero-scenario feature ever reaches the permanent tree.
6. **Wording nits** — resolved: ten of eleven dispatch invocations scan for
   the fixture (`cargo test --lib -- release_strict` does not run cucumber);
   more than two scenarios fail under the swap (every retrieval-expecting
   scenario, not just the first two observed).
7. **Alternative analysis** — resolved: design.md now records why in-process
   caching, a dedicated target dir, and a feature-preserving on-demand build
   were rejected; the failed-prebuild error path is `set -eu` abort at script
   start, which is the correct and only writable error path.
8. **Residual risk examined** — the stale-variant `scan_for_fixture` match is
   pre-existing, harmless, and unchanged; documented under Residual risks.

## Phase 2: Hardening

- design.md, proposal.md, and arc42.md updated with all resolutions above;
  the zero-scenario delta spec was removed in favour of the no-delta shape.
- No scenarios added (no delta; no `@e2e` tags exist to preserve).
- `givn lint --change harness-binary-stability` exits 0.
- The design-review contract also re-derived the arc42 12-row assessment:
  all rows No, ADR disposition NONE (canonical destination: design.md) —
  recorded in arc42.md; glossary unchanged (harness-internal vocabulary, not
  product domain language).

No human decisions were required; all findings were refinements within the
chosen approach, verified against code and live execution.

DESIGN-REVIEW: PASS
