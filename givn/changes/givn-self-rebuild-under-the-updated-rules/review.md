# Review: givn-self-rebuild-under-the-updated-rules

Review kind: independent
Author: givn-self-rebuild-under-the-updated-rules author (omp)
Reviewer context: independent-design-review-2026-11-17

Reviewed revision: `1cda02c5b81729a4f625431f7448b85fdd6e2c93`. The change's
code and test content has not moved since `a524d143`; the later commits hold
evidence and task ticks only. `design.md` is unchanged since the design review
(`sha256` prefix `72dc2b2c7e33aca0`, the value `design-review.md` records).

## Verdict

PASS. The declaration resolves the five former `specification defect` rows,
the trial is an unpromoted `Dry-Run Ise`, the neighbour record and the
clean-room scan are complete, both delta scenarios discriminate, and every new
step phrase is bound in the permanent `tests/steps/` tree. No blocking finding
remains.

## Findings

| # | Severity | Finding | State |
|---|---|---|---|
| F1 | Minor | The committed non-e2e log (`1023349d`) carried a stray `ry` line at line 915, an artifact of committing the file while the suite was still writing. The test sequence around it is otherwise complete. | Closed by the review's own `./run-tests.sh --non-e2e` rerun; the clean 1124-line log is committed with this review at the same path. |
| F2 | Minor | The suite task cited `78dfde5a` (the trial attempt-1 commit) instead of the commit holding the suite evidence. | Closed by the author at `1cda02c5`: the task cites `1023349` and records the suite counts. |
| F3 | Minor | `attempt-1/spec-gaps.md` still says "it is the only attempt". The file is the superseded attempt's record. | Not blocking. The final `attempt-2/spec-gaps.md` names the classified attempt and lists both attempts' counts, which is the design's requirement. The stale sentence stays as the superseded attempt's record; the next change carries no edit. |
| F4 | Recorded | The review task anticipated `variant` overlap rows for both added scenarios. The overlap scanner reports no `[SHAPE]` pair involving either scenario; both produce `[SUBST]` subset advisories only. | The table holds no rows because a row that matches no current finding is a stale disposition. The distinct invariants are recorded under `## Overlap dispositions`. |

## Implementation Evidence Check

**Step definition scan**: no empty or trivial bodies in the changed step files.
The five new bindings (`tests/steps/specification_declaration_steps.rs:121`,
`:229`, `:241`; `tests/steps/clean_room_preparation_steps.rs:322`, `:329`) each
drive `givn lint` or `givn ise rebuild-all` and assert on `stderr`, the copy, or
the command result. The shared fixture functions in
`tests/steps/ise_clean_room_support.rs:269-323` write and commit files and read
no assertion-free path. No scan hole: the files contain no `{}` or bare
`pass`/`return` body.

**Commit cross-check**: both added scenarios cite `a524d14`, which holds the
declaration, the pin, both features untagged, and all three step files.

| Scenario | Commit | Contains the implementation? |
|---|---|---|
| A declaration that names build inputs under the four content roles is accepted | `a524d143` | Yes: `givn/specification.yaml`, `rust-toolchain.toml`, both step bindings, the feature |
| Preparation carries every declared build input into the spec copy | `a524d143` | Yes: the shared fixture, both `Given` bindings, the feature |

**Promised-architecture cross-check**: the design promises no new component,
contract, or production module. None exists in the diff.

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| none promised | n/a | The change-scope diff is `givn/specification.yaml`, `rust-toolchain.toml`, three `tests/steps/` files, three arc42 chapters |

**Primitive inventory**: the change adds no production primitive.

| Primitive | Type | Purpose |
|---|---|---|
| `BUILD_INPUT_FILES` | test const | The five declared build-input paths and one committed content each |
| `build_input_declaration` | test fn | The declaration over the four content roles plus `e2e` and `source` |
| `build_input_files` | test fn | The declared files plus the behavior feature and one source file |
| `write_build_inputs` | test fn | Writes every declared file outside the `e2e` role |
| `build_input_project` | test fn | Initialized Git project with the declaration committed |
| `commit_build_inputs` | test fn | Commits the declared files, source file, and `e2e/Cargo.toml` |
| `ise_project_with_build_input_declaration` | step fn | The declaration scenario's `Given` |
| `stderr_contains_no_declaration_schema_error` | step fn | The `[DECL ]` negative assertion |
| `stderr_contains_no_declared_path_advisory_for_the_build_inputs` | step fn | The `[SPEC ] advisory` negative assertion over five subjects |
| `git_project_with_build_input_declaration` | step fn | The carrier scenario's first `Given` |
| `committed_revision_with_the_build_inputs` | step fn | The carrier scenario's second `Given` |

`design.md` has no `## Internal Primitives` section. I do not judge that a
defect here: the change adds no production primitive, and `## Step Definitions`
names the two capability files while `## Delta Step Bindings` names every new
phrase and its asserted behavior. The table above is the inventory against
those sections. No `design.md` edit is made, so `design-review.md` remains the
recorded review of the current design hash.

**Strict-mode proof**: `.fail_on_skipped()` is present on both builders
(`tests/features_runner.rs:55`, `:155`; `e2e/tests/e2e_runner.rs:46`). The
delta scope ran with 0 skipped; the frozen e2e run recorded 120 total,
0 skipped.

**Downgraded e2e scenario scan**: the delta adds no `@e2e` scenario. The
frozen package was not changed by this change.

| @e2e scenario | Real-interface assertion | Real browser (if UI) | Downgraded? |
|---|---|---|---|
| none added by this change | n/a | n/a | No |

**E2E scope check**: no new user action and no new `@e2e` scenario. The frozen
inventory stays at 115 definitions over 56 feature paths plus 5 outline
examples (120 executions), matching `e2e-inventory.md` and
`attempt-<N>/e2e-result.json`.

**Duplicate-implementation check**: `verify.e2e_command` is
`./run-tests.sh --e2e --package e2e`, which builds the checkout binary and runs
`e2e/tests/e2e_runner.rs`. No second e2e runner or step implementation exists
in the tree.

| Capability | File verify.e2e_command invokes | Other implementation found? | Stronger one used? |
|---|---|---|---|
| all frozen `@e2e` capabilities | `e2e/tests/e2e_runner.rs` | No | n/a |

**Local environment check**: the product is a CLI. No server, database, queue,
or digital twin is started outside the scenarios. The trial's commands ran
locally and the frozen package drove the rebuilt binary through `GIVN_BIN`.

**E2E scope selection and receipt**: the two scopes are selected explicitly by
the runner arguments and the tag expression. No archive receipt exists yet;
this change is an unproven candidate.

| `verify.command` value | `verify.e2e_command` value | Explicit scope selection | Regular scope evidence | E2e scope evidence | Zero failed or skipped? |
|---|---|---|---|---|---|
| `./run-tests.sh --non-e2e` | `./run-tests.sh --e2e --package e2e` | `regular_tags='not @e2e and not @givn.removed'`; e2e package invocation | 1115 tests, 1115 passed, 115 `@e2e` tests excluded | 120 scenarios, 120 passed, 784 steps passed | Yes in both scopes |

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | The declaration keeps the five roles and one `source:` list; the delta scenarios and the trial evidence cover the declaration and copy guarantees. | Yes |
| Capability ownership and typed relationships preserved | Two `@givn.added` capabilities owned by `declare-the-specification` and `prepare-an-ise-test`; no new capability, no retagging. | Yes |
| Confirmed Persona-relevant outcomes preserved | `developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner` each have impact and evidence rows in `design.md`. | Yes |
| Actors remain distinct from Persona biographies | The changes touch no use-case actor row. | Yes |
| Migration ledger and coverage evidence complete, if applicable | No migration; coverage is measured by the archive. | Yes |

PERSONAS: developer--primary-user

## Design.md conformance

| Decision (from design.md) | design.md says | Actually built | Matches? | Design review re-run? |
|---|---|---|---|---|
| Declaration entries | The exact 29 entries across `behavior`, `structure`, `contracts`, `limits`, and `e2e` | `givn/specification.yaml` matches the exact block (`declaration.diff`) | Yes | Not needed; unchanged |
| Toolchain pin | `rust-toolchain.toml` with `1.97.1`, `clippy`, `rustfmt` | The file matches; the trial built on it | Yes | Not needed |
| Delta files | `specs/declare-the-specification/specification-declaration.feature` and `specs/prepare-an-ise-test/clean-room-preparation.feature` | Both files, `@givn.added`, no `@e2e`, no `@wip` | Yes | Not needed |
| Step files | `tests/steps/specification_declaration_steps.rs`, `tests/steps/clean_room_preparation_steps.rs`, fixture in `tests/steps/ise_clean_room_support.rs` | The three files, both bindings and the fixture | Yes | Not needed |
| Verify commands | `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e --package e2e` | `givn/commands.yaml` holds both literally | Yes | Not needed |
| Rebuild mode | `givn ise rebuild-all --at <revision>`, no `--dry-run`, nothing promoted | `rebuild-report.txt` names the trial revision; the copy, manifest, and binary stay outside the checkout | Yes | Not needed |

## Interaction coverage verification

The five inventory entries from `givn/specs/declare-the-specification/usecase.md`,
`givn/specs/prepare-an-ise-test/usecase.md`, and
`givn/specs/create-change/usecase.md` each have a `design.md`
`## Interaction Coverage Matrix` row and an existing `@e2e` scenario.

| Inventory entry | Matrix row | @e2e scenario | Promised mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| `specification-declaration` / declare the roles | Yes | `The declared roles are read by the project-health pass` | CLI | `givn lint` through the built binary | Yes |
| `clean-room-preparation` / write the spec copy | Yes | `Developer rebuilds the whole product from its Specification` | CLI | `givn ise rebuild-all --at HEAD` | Yes |
| `trial-input-verification` / compare copy and checkout | Yes | `Verifier compares the spec copy with the manifest` | CLI | `givn ise compare` | Yes |
| `change-strategy` / read the design guidance | Yes | `The design guidance requires one recorded change strategy` | CLI | `givn instructions design --change <id>` | Yes |
| `spec-gap` / read the review guidance | Yes | `The review guidance requires one spec gap outcome before any fix` | CLI | `givn instructions review --change <id>` | Yes |

No row is added, removed, or retagged; the change adds no user action.

## Visual Review

The design declares `Interface: none`. The change renders no screen; its
user-visible surface is terminal text. The delta adds no `@e2e` scenario, so no
visual transcript or screenshot is required.

| @e2e scenario | Viewport | Evidence file | Present and non-empty? |
|---|---|---|---|
| none added | not applicable | not applicable | n/a |

VISUAL-REVIEW: PASS

## Retirements

N/A. This change retires no capability.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Declared build inputs as a technical constraint | `docs/arc42/02-architecture-constraints.md` §2.3 | Affected row 2 | `## Declaration` | Batch `declaration-and-copy` | The chapter diff names the roles; the declaration matches | Yes |
| Self-rebuild strategy and the resolved defects | `docs/arc42/04-solution-strategy.md` `Self-rebuild` row | Affected row 4 | `## Trial protocol` | Batch `trial` | The row records the copy, the five resolved rows, the residual implementation defects, and the no-promotion rule | Yes |
| Undeclared-surface risk and toolchain risk | `docs/arc42/11-risks-and-technical-debt.md` `R-163`, `R-164` | Affected row 11 | `## Risks and tradeoffs`, `## Version freshness` | Batch `setup` | The chapter diff matches the design's risk text | Yes |
| No other chapter affected | `arc42.md` rows 1, 3, 5-10, 12 | No | No | No | No component, runtime, deployment, quality, or glossary change | Yes |

ARC42 CONFORMANCE: CLEAN

## Declaration and trial evidence

### The five former Specification defects

| Former defect | Evidence | Resolution | Residual |
|---|---|---|---|
| OpenCode integration text | The manifest lists `assets/agent-integrations/opencode/givn-attribution.js` and `givn-timeline.js`; `compare-before.txt` and `compare-after.txt` say `match`; the frozen OpenCode scenario passes | Resolved by the `behavior` entry | None |
| Claude integration text | The manifest lists `assets/agent-integrations/claude/**`; the copy carries it; the residual failure is the rebuilt binary's missing subcommand | Resolved by the `behavior` entry | `implementation defect` row 19 |
| omp integration text | The manifest lists `assets/agent-integrations/omp/**`; the residual failure is the rebuilt trailer | Resolved by the `behavior` entry | `implementation defect` row 20 |
| Maki integration text | The manifest lists `assets/agent-integrations/maki/**`; the residual failure is the rebuilt integration's revision command | Resolved by the `behavior` entry | `implementation defect` row 21 |
| Product version `0.9.10` | The declared `Cargo.toml` at `746e660b` reads `0.9.20`; `rebuilt-version.txt` reports `givn 0.9.20`; the clean-room scenario passes every step | Resolved by the `contracts` entry | None |

`attempt-2/spec-gaps.md` records 0 `specification defect` rows and 79
`implementation defect` rows. The final table names no `specification defect`
on those five surfaces, which is the acceptance the task states. The three
residual failures are implementation defects, not gaps in the Specification:
the retained `givn/specs/fragments/attribute-commits.feature` scenarios state
the observables the rebuilt integration fails.

### Unpromoted dry run

`attempt-<N>/neighbor-record.md` records that the builder authored only the
replaced unit (24 files: 22 under `src/**`, 2 under `tests/**`) and that no
rebuilt file was promoted. The checkout's `src/**`, `e2e/**`, `assets/**`, and
`docs/reference.md` are unchanged by this change's commits
(`git diff --stat ee16f1fa^..HEAD -- src e2e assets docs/reference.md` is
empty). The copy, the manifest, and the rebuilt binary remain under `/tmp`.

### Neighbor record

Both attempts record an empty neighbour table, an empty consolidation table,
the three scanned stores, no quarantine, the test-infrastructure inputs
(`GIVN_BIN`, `GIVN_READY_FIXTURE`, `GIVN_ROOT`), and the no-promotion
statement. Attempt 2 also records that the repair rebuilt the binary in place
and that attempt 1's hash is no longer in the copy. Complete.

### Clean-room scan

The scan reads `~/.cargo/registry/src`, `~/.cargo/registry/cache`, and
`~/.cargo/git` at maxdepth 4. `clean-room-scan.txt` is empty, which is the
documented result when all three stores exist and hold no `givn` or `givn-*`
path. I reproduced the exact loop on this host: all three stores exist and the
loop prints nothing, exit 0. The sandbox binds `~/.cargo/registry` and
`~/.cargo/git`, so the scan covers every Cargo path the builder can read.

### Delta scenario discrimination

Both probes mutate the shared fixture, run the scenario, and restore the file
with `git checkout --`.

| Probe | Mutation | Observed |
|---|---|---|
| Declaration 1 | `Cargo.lock` removed from `BUILD_INPUT_FILES` | `And stderr contains no declared-path advisory for the build inputs` fails, naming `Cargo.lock`: `[SPEC ] advisory givn/specification.yaml & Cargo.lock the 'contracts' role declares 'Cargo.lock', which matches no path in the project` |
| Declaration 2 | `rust-toolchain.toml` removed from `BUILD_INPUT_FILES` | The same `Then` fails, naming `rust-toolchain.toml`: `[SPEC ] advisory givn/specification.yaml & rust-toolchain.toml the 'limits' role declares 'rust-toolchain.toml', which matches no path in the project` |
| Carrier 1 | `assets/templates/glossary.md` declared but not committed | `Then the command succeeds and prints "Specification files: 7"` fails: `ise rebuild-all: the declared Specification entry 'assets/templates/glossary.md' in the structure role matches nothing at revision 'bff8c9ab...'` |
| Carrier 2 | `assets/instructions/ise.md` declared but not committed | The same `Then` fails: `ise rebuild-all: the declared Specification entry 'assets/instructions/ise.md' in the behavior role matches nothing at revision '96aa0526...'` |

Baseline and post-restore runs are green: the single scenarios pass, and the
delta scope reports `2 tests run: 2 passed, 0 skipped`. The scenarios
discriminate on the declaration content, not on the fixture's exit code alone.

### Merged-step closeout

Both added scenarios' phrases are bound in the permanent `tests/steps/` tree;
the change directory holds no step file. `givn check tasks` runs the delta
scope green, and `givn archive` will merge the two scenarios into the
permanent features with no new step file.

## Coverage

No measured evidence exists yet; the archive runs `./measure-coverage.sh` and
`./merge-coverages.sh` over the regular suite and records the result in
`verification.json`. The change adds no production code and no production
process, so it adds no uncovered production region. The two added scenarios
are the coverage of the new bindings.

**Coverage result:** not measured yet; the change is an unproven candidate.

## Scenario execution evidence

```
givn check tasks --change givn-self-rebuild-under-the-updated-rules
  ✓ scenario commits passed (2 scenario(s))
  [Summary] 2 features, 2 scenarios (2 passed), 13 steps (13 passed)
  ✓ change verification receipt: 2 scenario(s), 0 failed, 0 skipped at 1cda02c5b81729a4f625431f7448b85fdd6e2c93
  ✓ E2E verification: not_applicable
  ✓ verify passed

./run-tests.sh --non-e2e
  Summary [ 281.611s] 1115 tests run: 1115 passed, 115 skipped

./run-tests.sh --e2e --package e2e
  [Summary] 56 features, 120 scenarios (120 passed), 784 steps (784 passed)
```

**Receipt status:** unproven candidate. The archive runs both scopes against
the merged candidate and publishes the receipt.

## Overlap dispositions

| Scenario A | Scenario B | Disposition |
|---|---|---|

The overlap scanner reports no `[SHAPE]` finding involving either added
scenario; both produce `[SUBST]` subset advisories against permanent
scenarios. A `variant` row would match no current finding and would be a stale
disposition, so the table holds no rows. The distinct invariants the task asks
for are:

- `A declaration that names build inputs under the four content roles is
  accepted` proves `givn lint` accepts a declaration that places the build
  inputs under the four content roles and prints no declared-path advisory for
  them. No permanent scenario asserts that declaration content.
- `Preparation carries every declared build input into the spec copy` proves
  the spec copy carries those inputs while the End-to-end manifest stays out.
  The permanent copy scenarios cover other copy rules, not this carrier
  content.

## Split-or-keep

| Scenario | Decision |
|---|---|

No added scenario exceeds the lint's long-scenario threshold: they hold 5 and
10 steps. `givn lint` reports no `long_scenario` finding for either, so a
`keep` row would be a stale split-or-keep row. Both stay in their current
capability files; no scenario is split.

## Sign-off

- [x] Fabrication audit: CLEAN (0 empty step bodies in the changed files,
      every checked scenario cites the commit that holds its implementation,
      no promised component is missing, no downgraded e2e scenario, no e2e
      scenario driven by HTTP in place of the real interface, no
      over-produced e2e scope).
- [x] Scenario execution evidence: the change receipt at `1cda02c5` records
      2 scenarios, 0 failed, 0 skipped; both verification scopes are green.
- [x] Every coverage gap is classified from static inspection: the change adds
      no production code; the archive measures the merged candidate.
- [x] No `@wip` tag remains in the delta spec.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the product cleanly; the product is a CLI.
- [x] The exact file `verify.e2e_command` invokes was identified, and the tree
      holds no parallel e2e implementation.
- [x] The E2E execution selects its scope explicitly; both scopes report zero
      failures and skips.
- [x] Implementation matches `design.md`; no design deviation.
- [x] The reviewer is a fresh context, not the author; the review context
      matches `design-review.md` byte for byte.

README-IMPACT: none

REVIEW: PASS
