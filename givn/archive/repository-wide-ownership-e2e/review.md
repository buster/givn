# Review: repository-wide-ownership-e2e

## Summary

- Three repository-owned CLI acceptance scenarios are complete.
- Each scenario invokes the compiled `givn` binary in a fresh `TempDir`.
- Review, successful archive, and pre-merge failure preservation all pass.
- No Watn-specific fixture, provider, network, server, database, or digital twin
  remains in the change.
- The scenario commits are recorded in `tasks.md`:
  `4369d77`, `5bbeadb`, and `37d244e`.

## Fabrication Audit

### Tag integrity

The active feature file contains three `@e2e` scenarios and no `@wip` tags.
Git history shows that the review and archive scenarios retained their
`@e2e` tags while `@wip` was removed, and the failure scenario gained its
`@e2e` tag before implementation. No `@e2e` tag was removed to bypass a gate.

### Step implementation

- Scanned `tests/steps/repository_ownership_fixtures_steps.rs` and
  `tests/steps/repository_ownership_fixtures_support.rs`: zero empty step
  bodies, zero `unimplemented!()`/`todo!()` bodies, and zero no-op assertion
  steps remain.
- The `return` in `collect_files` is an intentional early exit when a
  snapshot root does not exist; it is not a step body.
- The only matching capability files are the CLI step file and its shared
  helper file. The shared file contains no Cucumber step declarations.
- The strictness proof is recorded in `tasks.md`: the stubbed active scope
  exited non-zero with three failed scenarios and captured `not implemented`.

### Scenario commit audit

| Scenario | Commit | Implementation touched | Result |
|---|---|---|---|
| Repository-wide review accepts explicit overlap dispositions | `4369d77` | CLI step bindings, fixture subprocess/layout helpers, runner world state | `1 scenario (1 passed)`, `5 steps (5 passed)` |
| Archive publishes consolidated permanent specifications | `5bbeadb` | Archive invocation helper reused by the CLI step bindings and shared tree assertions | `1 scenario (1 passed)`, `7 steps (7 passed)` |
| Pre-merge archive failure preserves the permanent specification tree | `37d244e` | Shared byte-for-byte tree assertion helper and failure-path CLI binding | `1 scenario (1 passed)`, `4 steps (4 passed)` |

The implementation is test-system code because the production CLI contract
already exists in the stacked `spec-overlap-gates` implementation. No new
production runtime branch is required by this change. Each scenario commit
touches executable step/support code, not only a feature file or a stub.

### Real-interface audit

All three scenarios use `tests/common::run_givn`, which resolves
`CARGO_BIN_EXE_givn`, starts a child process, sets the fixture root as its
working directory, and captures stdout, stderr, and exit status.

- Review assertions check CLI exit status and overlap/net-delta stdout before
  any filesystem assertion.
- Successful archive assertions check CLI exit status and archive stdout
  before checking the permanent tree secondarily.
- Failure assertions check the non-zero CLI status before comparing the
  permanent tree snapshot byte-for-byte.
- This is a CLI capability, not a browser capability; no HTTP or in-page
  shortcut exists.

### Interaction coverage

| User Interaction Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism | Result |
|---|---|---|---|---|
| invoke repository-wide review for a project with explicit overlap dispositions | review CLI row | Repository-wide review accepts explicit overlap dispositions | `check review` subprocess in `TempDir` | Covered |
| archive a consolidated project and publish its permanent specifications | successful archive CLI row | Archive publishes consolidated permanent specifications | `archive` subprocess in `TempDir` | Covered |
| invoke archive for a project whose pre-merge hook fails | failure archive CLI row | Pre-merge archive failure preserves the permanent specification tree | failing `archive` subprocess in `TempDir` | Covered |

There is exactly one E2E scenario for each distinct interaction contract.
The failure-preservation contract is a distinct recovery interaction, not an
input variant of the successful archive contract.

### Runner and scope audit

`givn/commands.yaml` defines distinct commands:

```text
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
```

The permanent projections passed with `163` non-E2E scenarios and `185`
E2E scenarios. For the active delta, the unfiltered command and the `@e2e`
projection each passed with `3` scenarios and `16` steps; the `not @e2e`
projection contained `0` scenarios. This equal-count case is intentional:
all three active scenarios are E2E, and the exception is recorded rather than
presented as a strict subset.

The exact E2E command invokes `run-tests.sh`, whose `--e2e` branch runs
`cargo test --test features_runner -- --tags @e2e`. A repository-wide search
found no second implementation of this capability.

The implementation matches `design.md`: Rust cucumber-rs, one CLI E2E step
file for the capability, one shared helper file, `TempDir`, local commands,
`GIVN_FEATURES`, and the compiled CLI boundary. No local service stack is
required; the TempDir fixture starts and completes without external
infrastructure.

## Arc42 Implementation Conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Repository-owned acceptance goal | Ch. 01, section 1.5 | Ch. 1 affected; Givn owns its acceptance contract | Scope moves generic review/archive evidence into Givn | Three CLI scenarios completed | Delta feature and CLI fixture steps | Yes |
| Isolated CLI boundary | Ch. 02, section 2.6 | Ch. 2 affected; no consumer, network, or shared filesystem | Compiled binary plus TempDir and local commands | Setup fixture has no external infrastructure | `common::run_givn` and `make_bare_root` | Yes |
| Black-box strategy | Ch. 04, sections 4.1 and 4.3 | Ch. 4 affected; public CLI is the acceptance boundary | Exit status/stdout are primary assertions | GREEN entries require real subprocess output | Three scenarios invoke `CARGO_BIN_EXE_givn` | Yes |
| Test building blocks | Ch. 05, section 5.3 | Ch. 5 affected; CLI bindings and shared helpers | Exact step/support paths are named | Setup registers both modules | `repository_ownership_fixtures_steps.rs` and `_support.rs` exist | Yes |
| Runtime sequence | Ch. 06, sections 6.5b and 6.6 | Ch. 6 affected; review/archive/failure flows are executable | Fixture lifecycle and pre-merge failure phase are explicit | Targeted RED/GREEN/REFACTOR evidence | Review/archive subprocess calls and snapshot timing | Yes |
| Strictness, isolation, and coverage rules | Ch. 08, sections 8.3.1 and 8.5 | Ch. 8 affected; strict runner and CLI evidence | `fail_on_skipped`, `GIVN_FEATURES`, and coverage commands are named | Strictness proof and scope counts recorded | Runner rejects `unimplemented!()` and all scenarios pass | Yes |
| Repository-owned fixture decision | Ch. 09, ADR-038 | Ch. 9 affected; ADR-038 records the boundary decision | No consumer-specific fixture behavior | Scenario commits implement the decision | Generic fixture contents and compiled CLI invocation | Yes |
| Acceptance quality signals | Ch. 10, Q-27, Q-28, Q-29 | Ch. 10 affected; all three quality scenarios mapped | Review receipt, archive publication, and unchanged-tree evidence | One completed task section per scenario | Feature assertions match all three quality signals | Yes |
| Fixture drift and child-process risks | Ch. 11, R-17, R-18, D-10 | Ch. 11 affected; risks and mitigation retained | TempDir, stdout/status assertions, and child coverage are explicit | Coverage and real-interface evidence recorded | Fresh fixture roots and non-zero CLI production coverage | Yes |
| Ownership-fixture terminology | Ch. 12 glossary | Ch. 12 affected; terms are added durably | Design uses ownership fixture, disposition, and net delta consistently | Tasks use the same scenario language | Feature, support, and review use the glossary terms | Yes |

ARC42 CONFORMANCE: CLEAN

## Coverage Review

### Measurement validity

The configured commands were executed freshly in order:

```text
./measure-coverage.sh
./merge-coverages.sh
```

The measurement script runs both `features_runner` projections under
`cargo llvm-cov`; the E2E projection invokes the compiled child binary through
`CARGO_BIN_EXE_givn`. The active-delta measurement produced distinct reports,
and the full project measurement was then regenerated after the merge-script
fix. `merge-coverages.sh` now merges class-level and method-level line hits in
addition to the top-level counters, so the merged report does not silently
discard E2E coverage for a class.

### Fresh report counters

The pre-archive review measurement produced these project-level reports:

| Report | Lines covered | Lines valid | Line rate | Branches covered/valid |
|---|---:|---:|---:|---:|
| `coverage/non-e2e-cobertura.xml` | 2978 | 4093 | 72.758% | 0/0 |
| `coverage/e2e-cobertura.xml` | 2207 | 4093 | 53.921% | 0/0 |
| `coverage/cobertura-coverage.xml` | 5185 | 8186 | 63.340% | 0/0 |

The active-delta E2E report separately exercised `src/cli/check.rs` at
`41/98` lines and `src/cli/archive.rs` at `118/168` lines. In the final
project-level E2E report, the same files have non-zero class coverage, and the
fresh merged report contains:

| File | E2E line rate | Merged covered/valid | Merged line rate |
|---|---:|---:|---:|
| `src/cli/check.rs` | 47.959% | 57/98 | 58.163% |
| `src/cli/archive.rs` | 74.405% | 143/168 | 85.119% |

The known exercised production paths are therefore non-zero in both the E2E
source report and the merged report. The branch denominator is zero in all
three fresh Cobertura reports because the configured llvm-cov command does not
enable its unstable branch instrumentation. This is classified as bucket 3,
legitimately hard to test with the current stable project measurement
contract: there are no emitted branch regions to classify as uncovered, and
the existing Cobertura validator explicitly treats `branches-valid=0` as a
valid unmeasured denominator rendered as `n/a`. No line-coverage gap caused by
this change is unresolved: the change adds no production runtime branch, and
all new fixture behavior has direct CLI coverage.

Coverage gap classification:

- Dead code: none introduced by this change.
- Missing test coverage: none for the new fixture behavior; each new action
  has a real CLI scenario and the exercised production paths are non-zero.
- Legitimately hard to test: branch regions are not emitted by the configured
  stable measurement command (`0/0`); no branch claim is made.

## Final Gates

- `givn lint --change repository-wide-ownership-e2e`: clean.
- `./run-tests.sh`: `20 features`, `163 scenarios (163 passed)`, `926 steps`.
- `./run-tests.sh --e2e` passed before archive; the post-merge archive gate
  and rerun passed with `16 features`, `25 scenarios (25 passed)`, and `143
  steps`.
- Active delta: `3/3` scenarios and `16/16` steps passed in both full and
  `@e2e` projections; non-E2E projection was empty by design.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `givn check review --change repository-wide-ownership-e2e`: verify,
  verify-e2e, integrity, overlap, and net-delta gates passed; the receipt was
  `net delta: 3 added, 0 modified, 0 removed`.

## Post-merge Archive

- `cargo run --quiet -- archive --change repository-wide-ownership-e2e` exited
  0.
- The delta was moved to
  `givn/archive/repository-wide-ownership-e2e/` and merged into
  `givn/specs/repository-ownership-fixtures/`.
- The archive gate reran the permanent verify and E2E projections. The E2E
  projection passed with `16 features`, `25 scenarios`, and `143 steps`.
- The fresh post-merge reports contain non-E2E `2978/4093` lines, E2E
  `2238/4093` lines, and merged `5216/8186` lines (`64%`). The merged class
  rates are `57/98` for `src/cli/check.rs` and `144/168` for
  `src/cli/archive.rs`; branch coverage remains `n/a (0/0)`.
- The coverage gate updated the managed README summary and removed
  `.archive-backup-repository-wide-ownership-e2e`.

REVIEW: PASS
