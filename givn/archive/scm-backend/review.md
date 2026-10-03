# Review: scm-backend

## Step 0: Fabrication audit

| # | Check | Result |
|---|---|---|
| 0 | `@e2e` tag integrity in the delta | Clean: `scm-backend-selection.feature` has exactly one `@e2e` scenario, tagged from the start; no tag was removed. |
| 1 | Empty or no-op step bodies | Clean: no `unimplemented!()`, `todo!()`, or empty body remains in `tests/steps/scm_backend_selection_steps.rs` or `tests/steps/scm_backend_selection_e2e_steps.rs`. |
| 2 | Every checked task has a commit touching production source | Clean: `b61e58f5`, `823f9dbf`, `070612d0`, `e72119d4`, `b1d1a857`, `037fdaf5`, `9d3770f2`, `d96ffff4`; each touches `src/scm.rs`, `src/git.rs`, `src/cli/migration.rs`, or the bound steps. |
| 3 | Components promised by design exist | Clean: `src/scm.rs` (`Backend`, `select`, probes, refusals), `Worktree::at/stage_paths/has_staged_changes/commit_staged` in `src/git.rs`, migration routed through the seam. |
| 4 | Strict-mode proof present | Clean: tasks.md Setup shows the first targeted run exiting 1 with `Step panicked. Captured output: not implemented`; `.fail_on_skipped()` is configured in `tests/features_runner.rs`. |
| 5 | `@e2e` Then steps assert through the real interface | Clean: the E2E Then asserts CLI exit 0, stdout reporting `Refreshed` and the new revision, and the changed revision plus restored managed file. |
| 6 | Browser-UI driver fidelity | Not applicable: the interface is a CLI subprocess; no browser, HTTP client, or repository-only assertion replaces it. |
| 7 | Single `@e2e` implementation for the capability | Clean: only `scm_backend_selection_e2e_steps.rs` binds the E2E Then; no duplicate exists in the tree. |
| 8 | E2E scope matches the canonical policy | Clean: one distinct real action (`givn upgrade`); the five refusals are error preconditions of the same action and are regular scenarios. |
| 9 | Local run command starts the full stack | Clean: `./run-tests.sh`; no service, database, or digital twin exists. |
| 10 | `verify.command` differs from `verify.e2e_command` and isolates `@e2e` | Clean: `./run-tests.sh` runs 467 scenarios; `./run-tests.sh --e2e` runs 83 (strictly fewer). Change scope: 7 vs 1. |
| 11 | Implementation matches design-named files/commands | One deviation found and reconciled: `commit_staged` is commit-only and `Backend::revision()` reads the revision; design.md was updated and a fresh-context re-review confirmed VALID (addendum in `design-review.md`). |
| 12 | Findings are reopened and redone | No unfinished implementation found. |
| 13 | Interaction coverage cross-reference | See the table below. |
| 14 | Coverage measurement validity | See "Coverage measurement" below. |

## Interaction coverage cross-reference

The fragment declares `Interactions: none`; the routing and lint contracts
exempt fragments from the inventory check, so the design's matrix is an
evidence map, not a normalized inventory.

| Design matrix row | `.feature` scenario | Driving mechanism in the step file | Match |
|---|---|---|---|
| Primary E2E action: run `givn upgrade` in a Git worktree | "A Git worktree at the project root selects the Git backend" (`@e2e`) | `consumer_runs` invokes the compiled binary via `common::run_givn`; the E2E Then asserts stdout and the revision | Yes |
| Refusal invariants | The five refusal scenarios (regular) | `user_runs` / capability-specific When invoke the compiled binary; Thens assert the named refusal and no file change | Yes |

## Coverage measurement

- Runner and processes: `./measure-coverage.sh` runs the feature suite through
  `cargo llvm-cov` (non-E2E and E2E passes with separate profraw sets) and
  `./merge-coverages.sh` merges them into `coverage/cobertura-coverage.xml`.
  The production process is the instrumented `givn` binary; no service process
  exists.
- Fresh output: the merge produced the report in this review run; line-rate
  0.783, branch-rate 0.0 (branch coverage is not instrumented in this repo).
- Non-zero production probe: all 54 production classes report a non-zero
  line-rate.
- Gap classification for the new code:
  - `src/scm.rs` error branches (`git` unavailable without a marker,
    unexpected `io` error, `rev-parse --show-toplevel` failure, canonicalize
    fallback): **legitimately hard to test** — each requires fault injection
    into the `git` executable (a fake binary that reports inside-worktree and
    then fails), and each produces the same named refusal already covered by
    the observable scenarios.
  - `src/scm.rs` `_FOSSIL_` alias: **legitimately hard to test** by explicit
    exclusion — the design records that the Windows alias shares the same
    `exists()` path as `.fslckout`, which is covered.
  - `src/git.rs` new-method error branches and `src/cli/migration.rs`
    pre-existing migration/error paths: **legitimately hard to test** —
    defensive `bail!` branches and legacy bundle paths that predate this
    change; no new behavior is uncovered.
  - No dead code and no missing test coverage was found: every behavior the
    change adds has an executable scenario.

## Arc42 implementation conformance

| Chapter or fact | Durable claim | Implementation evidence | Match |
|---|---|---|---|
| 02 TC-21 | Managed upgrade requires a selected version-controlled project and a clean checkout | `src/cli/migration.rs` uses `scm::select` + `dirty_entries` | Yes |
| 02 TC-33 | One seam selects Git or refuses absence, ambiguity, nested root, missing executable by name | `src/scm.rs` `select` | Yes |
| 03 external entities | Git CLI answers backend selection; Fossil marker identifies a checkout | `git_worktree`, `fossil_marker` | Yes |
| 04 solution strategy | SCM backend row | Seam exists; archive stays on its own Git boundary | Yes |
| 05 building blocks | `scm` block and updated `git`/`migration` responsibilities | `src/scm.rs`, `src/git.rs`, `src/cli/migration.rs` | Yes |
| 06.13 / 6.16 | Upgrade preflight via the seam; detection flow and refusals | Implementation and scenarios match the documented flow | Yes |
| 08.18 | Selection order, refusals, backend operations | `src/scm.rs` | Yes |
| 09 / ADR-071 | Accepted record, 27 accepted in the register | `adr/README.md`, `adr-qualification-gate` scenario | Yes |
| 10 Q-50 | No usable backend is refused with the root named | `scm::select` message + `release-0-3-0` scenario | Yes |
| 11 R-63/R-64/D-20 | Mixed-root, missing-executable, and Fossil-increment limitations | Refusals implemented; Fossil backend deferred | Yes |
| 12 glossary | Backend, revision, checkout, clean state, marker, named refusal | Terms used in code and specs | Yes |

ARC42 CONFORMANCE: CLEAN

## Ubiquitous language conformance

Specs, design, and code reuse the glossary terms exactly: `SCM backend`
(`Backend`), `revision` (`revision`, `head`), `checkout` (`dirty_entries`),
`clean state`, `marker` (`fossil_marker`, `.git`), and `named refusal`. No
term appears in the change that the glossary does not hold.

## Persona disposition

| Persona | Relevance | Evidence |
|---|---|---|
| `developer--primary-user` | Named refusals explain what to fix | The five refusal scenarios assert the root and the missing prerequisite |
| `agent--technical-role` | Deterministic command surface | `givn upgrade` behavior and exit codes unchanged on Git; one seam for backend questions |
| `givn-maintainer--owner` | One seam, truthful docs | `src/scm.rs` is the only selection point; ADR-071 and arc42 chapters updated |

## Visual review

Not applicable: the change's design declares no visual interface (`Interface:
none`); the surface is CLI text with no rendered screen. No `VISUAL-REVIEW`
line is required.

## Deterministic gate dispositions

`givn lint --change scm-backend` reports 3 files checked and clean. The only
shape matches are `[SUBST]` subset warnings between the E2E scenario and
unrelated visual-review/overlap scenarios; they share generic step shapes
("Given a project root ...", "When the user runs ...") and assert different
invariants. Disposition: **keep** — no duplication; the overlap disposition
gate passes.

## README impact

README-IMPACT: none

## Verification evidence

```text
./run-tests.sh           467 tests run: 467 passed, 0 skipped
./run-tests.sh --e2e      83 tests run: 83 passed, 384 skipped
GIVN_FEATURES=givn/changes/scm-backend/specs ./run-tests.sh
                           7 tests run: 7 passed, 0 skipped
givn lint --change scm-backend   3 file(s) checked — clean
givn check review --change scm-backend   integrity, run declaration,
                                         overlap dispositions passed
cargo check --all-targets               warning-free
cargo clippy --workspace --all-targets -- -D warnings   passes
coverage merge            line-rate 0.783; 54/54 production classes non-zero
```

REVIEW: PASS
