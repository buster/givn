# Review: coverage-script-workflow

## Fabrication Audit

**Files scanned:**

- `givn/changes/coverage-script-workflow/specs/coverage-script-workflow/coverage-script-workflow.feature`
- `tests/steps/coverage_script_workflow_steps.rs`
- `tests/steps/coverage_script_workflow_e2e_steps.rs`
- `src/cli/coverage_gate.rs`
- `src/cobertura.rs`
- `src/cli/agents_block.rs`
- `src/config/manifest.rs`
- `assets/commands.yaml.tmpl`
- `assets/addons/readme/README.md.tmpl`
- `assets/skills/givn-archive/SKILL.md.tmpl`

**@e2e integrity:** CLEAN. The delta retains exactly two `@e2e` scenarios:
initialization guidance and archive publication. The merge-failure scenario is
non-E2E by design because it tests a deterministic command failure.

**Empty step-body scan:** CLEAN. `0` empty or no-op step bodies found across
the two capability step modules. Setup steps create isolated files or invoke
the real CLI; assertion steps inspect command results, reports, or README
content.

**Checked-task commit cross-check:**

| Task area | Commit | Production source touched? |
|---|---|---|
| Script command contract and schema | `d256576` | Yes — `src/config/manifest.rs`, `src/integrity_hook.rs` |
| Failed merge blocks archive | `ab0862a` | Yes — `src/cli/coverage_gate.rs` |
| README merged line/branch publication | `c21ae9c` | Yes — `src/cobertura.rs`, `src/cli/agents_block.rs` |
| Legacy command migration compatibility | `eff3d28` | Yes — `src/config/manifest.rs` |
| Merged report counter/rate validation | `8727d02` | Yes — `src/cobertura.rs` |

**Promised components:** CLEAN. The configured scripts, command schema,
coverage gate, Cobertura renderer, title-aware managed-block helper, separate
step modules, README template, archive guidance, and durable Arc42 updates all
exist at the paths named in `design.md`.

**Strict-mode proof:** CLEAN. `tests/features_runner.rs` uses
`.fail_on_skipped()`. A temporary undefined-step feature was run through both
commands:

```text
GIVN_FEATURES=/tmp/givn-coverage-strict cargo test --test features_runner
2 scenarios (2 failed), 2 steps (2 failed), Step doesn't match any function

GIVN_FEATURES=/tmp/givn-coverage-strict cargo test --test features_runner -- --tags @e2e
1 scenario (1 failed), 1 step (1 failed), Step doesn't match any function
```

Both exits were non-zero.

**E2E interface audit:** CLEAN. Both E2E scenarios invoke the compiled `givn`
subprocess in an isolated temporary project. Their primary result is the CLI
operation and its generated README/report output, not a direct Rust function
call. No browser or HTTP shortcut exists.

**E2E command audit:** The effective values are read from `givn/commands.yaml`:

```yaml
verify:
  command: "./run-tests.sh"
  e2e_command: "./run-tests.sh --e2e"
```

`./run-tests.sh` reports `134 scenarios (134 passed)` and
`./run-tests.sh --e2e` reports `17 scenarios (17 passed)`. The E2E command is a
strict tag-filtered subset and is not identical to the normal command. No other
step implementation for this capability exists in the tree.

**Local runnability:** CLEAN. The complete capability runs with
`cargo test --test features_runner`; it uses only temporary directories,
subprocesses, and local shell fixtures. No external service or network is
required.

**Interaction coverage cross-reference:**

| User interaction inventory entry | E2E scenario | Step module | Driver/result |
|---|---|---|---|
| initialize a project and inspect its test and coverage command guidance | An initialized project documents reusable test and coverage scripts | `coverage_script_workflow_e2e_steps.rs` | Real `givn init` subprocess; asserts generated `commands.yaml` and README |
| archive a completed change and inspect the published merged coverage result | Archiving publishes line and branch badges from the merged report | `coverage_script_workflow_e2e_steps.rs` | Real `givn archive` subprocess; asserts exit status, merged XML, top badges, and README summary |

The matrix matches the feature inventory exactly. There is one E2E scenario per
distinct CLI action and no excess variant coverage.

## Arc42 Implementation Conformance

| Arc42 chapter/fact | Durable source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Project-owned coverage commands | `docs/arc42/08-crosscutting-concepts.md` | Chapter 8 Yes | `measure_command` and `merge_command` | Setup contract | `givn/commands.yaml`, scripts | Yes |
| Archive coverage building block | `docs/arc42/05-building-block-view.md` | Chapter 5 Yes | `coverage_gate` runs/validates/rolls back | Scenario commits | `src/cli/coverage_gate.rs` | Yes |
| Measure/merge runtime order | `docs/arc42/06-runtime-view.md` | Chapter 6 Yes | fresh output and rollback | Archive scenario | `run_coverage_gate_inner` | Yes |
| Merge authority | `docs/arc42/09-architecture-decisions.md` ADR-029 | Chapter 9 Yes | script-produced report authoritative | Archive scenario | no Rust merge on new path | Yes |
| README ownership | `docs/arc42/09-architecture-decisions.md` ADR-030 | Chapter 9 Yes | two managed blocks | README scenario | `agents_block`, README | Yes |
| Stale output and rollback risks | `docs/arc42/11-risks-and-technical-debt.md` R-13-R-16 | Chapter 11 Yes | fresh output and transaction | Merge-failure scenario | coverage state backup/restore | Yes |
| Coverage vocabulary | `docs/arc42/12-glossary.md` | Chapter 12 Yes | terms defined | Review evidence | glossary entries | Yes |

**ARC42 CONFORMANCE: CLEAN**

## Coverage Measurement

Measurement was run through the configured scripts:

```sh
./measure-coverage.sh
./merge-coverages.sh
```

The normal and E2E reports were generated separately. The scripts use
collision-safe `coverage/profraw/%p-%m.profraw` output, instrument the
`givn` production binary and the Gherkin runner, and the merged report includes
non-zero production coverage. For example, the merged report shows

**COVERAGE MEASUREMENT: VALID**

Merged Cobertura output used for README publication:

```text
Line coverage: 60% (4302/7116)
Branch coverage: n/a (0/0)
```

The branch denominator is zero in the current `cargo llvm-cov` output. The
README deliberately renders `n/a`, not a false `0%` claim.

## Coverage Gap Classification

The changed production modules have no unclassified behavior gap:

| Area | Result | Classification/evidence |
|---|---|---|
| New command schema and legacy parsing | Exercised by archive scenarios and manifest unit test | Missing-test gaps resolved; no dead code introduced |
| Measurement/merge success path | Exact `95/100` line and `19/20` branch fixture | Missing-test bucket resolved by the archive E2E scenario |
| Merge failure rollback | Failing merge fixture preserves active change and transaction state | Missing-test bucket resolved by the merge-failure scenario |
| Badge placement and rendering | Pure helper tests plus archive README assertions | Missing-test bucket resolved |
| OS spawn, malformed XML, and filesystem failure branches | No deterministic production path can force every OS error without mocking; the parser and command status boundaries are exercised | Bucket 3: legitimately hard to test, with concrete boundary justification |

No dead code was added. No gap is excused by a percentage threshold or scope
claim. Branch percentages are descriptive only; completeness remains the gate.

## Test Suite

```text
cargo test --test features_runner
151 scenarios (151 passed)
819 steps (819 passed)

cargo clippy --workspace --all-targets -- -D warnings
passed
```

Both permanent-spec and change-scoped lint are clean after implementation;
there are no remaining `@wip` tags.

## Sign-Off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified production-touching commit.
- [x] Every promised component exists.
- [x] Strict mode proven with non-zero normal and E2E runs.
- [x] Normal and E2E commands both pass and E2E is isolated.
- [x] Coverage measured across the Gherkin runner and production binary.
- [x] Coverage gaps classified into the three permitted buckets.
- [x] No `@wip` tags remain in permanent or active change specs.
- [x] Interaction inventory, design matrix, feature scenarios, and step modules match.
- [x] Arc42 implementation conformance is clean.

REVIEW: PASS
