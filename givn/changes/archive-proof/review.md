# Review: archive-proof

## Fabrication Audit (do this first)

**Step definition scan** — the new and modified step files were scanned for
empty/no-op bodies: `tests/steps/archive_proof_steps.rs`,
`tests/steps/archive_proof_e2e_steps.rs`,
`tests/steps/archive_proof_support.rs`, plus the touched fixtures in
`tests/steps/extract_coverage_gate_steps.rs`,
`tests/steps/enforce_post_merge_e2e_gate_steps.rs`,
`tests/steps/workflow_enforcement_steps.rs`,
`tests/steps/repository_ownership_fixtures_*.rs`,
`tests/steps/readme_enforcement*.rs`,
`tests/steps/run_execution_e2e_steps.rs`,
`tests/steps/release_0_3_0_e2e_steps.rs`,
`tests/steps/coverage_script_workflow*.rs`,
`tests/steps/fix_uppercase_checkbox_parsing_steps.rs`. Every body drives the
real `givn` binary and asserts exit codes, stdout/stderr, receipt JSON, git
history, or filesystem state. The only no-op is the shared
`a placeholder step` binding that exists for `@givn.removed` placeholders.

**Commit cross-check** — the implementation was delivered in four real
commits, each touching production code or the executable test surface:

| Scope | Commit hash | Touches production/test code? |
|---|---|---|
| Forward-only pipeline, Git wrapper, proof module, runner result, check/coverage/status/instructions | `9f136bc0` | yes (`src/`, `run-tests.sh`, `tests/features_runner.rs`) |
| Step definitions and fixture migration for Git + result contracts | `56441e67` | yes (`tests/steps/`) |
| ADR-070 acceptance, scope deltas, design/review guidance | `c3c35455` | docs + specs |
| Implementation evidence, README/reference/workflow/CHANGELOG | `5b311e51` | docs |

Deviation recorded: the planned one-commit-per-scenario rhythm was
consolidated into capability-level commits because the archive pipeline change
is a single intertwined subsystem; the capability-level mapping is recorded
in `tasks.md`'s `## Commits` section instead of per-scenario hashes. The
review treats this as a process deviation, not a coverage gap; the executable
evidence for every scenario is the change-scoped runs below.

**Promised-architecture cross-check** — every component named in design.md
exists:

| Component (from design.md) | Exists in tree? | Path |
|---|---|---|
| Git worktree wrapper | yes | `src/git.rs` |
| Proof module (receipt, runner result, evidence paths) | yes | `src/proof.rs` |
| Forward-only archive pipeline | yes | `src/cli/archive.rs` |
| Coverage gate without rollback | yes | `src/cli/coverage_gate.rs` |
| Review without suite execution | yes | `src/cli/check.rs` |
| Status/instructions pending projection | yes | `src/cli/status.rs`, `src/cli/instructions.rs` |
| Result runner support | yes | `tests/features_runner.rs`, `run-tests.sh` |

**Strict-mode proof** — `GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`; the change-scoped suite was red before the
implementation (18 failing scenarios in the first run) and green after:

```text
GIVN_FEATURES=givn/changes/archive-proof/specs GIVN_TEST_RUNNER=cargo ./run-tests.sh
8 features / 44 scenarios (44 passed) / 270 steps
exit 0
```

**Downgraded e2e scenario scan** — the three new `@e2e` scenarios drive the
compiled CLI as a subprocess and assert the proven receipt, the rerun proof,
and `givn status` output; the modified enforce e2e asserts the pre-candidate
block. No Then step asserts only repository state where the CLI exposes the
real result.

| @e2e Scenario | Real-interface assertion present? | Driven via real interface? | Downgraded? |
|---|---|---|---|
| Consumer archives a completed change and receives a proven archive | yes (receipt through CLI + archive state) | yes (compiled CLI subprocess) | no |
| Consumer repairs a blocked archive and the rerun proves it | yes | yes | no |
| Consumer inspects an unproven archive candidate | yes (`givn status` stdout) | yes | no |
| Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner | yes (stderr block, change active) | yes | no |

**E2e scope check** — one `@e2e` scenario per normalized archive-proof
inventory action; variants (missing/malformed/failed/skipped/zero/
non-reconciling/wrong-scope results) are regular outline examples.

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---|---|---|
| archive-proof | 3 | yes (archive, repair, inspect) | no |
| modified capabilities | 1 (enforce delta) | yes | no |

**Duplicate-implementation check** — `verify.e2e_command` is
`./run-tests.sh --e2e`; the only e2e step implementation for archive-proof is
`tests/steps/archive_proof_e2e_steps.rs`. No parallel/untracked implementation
exists.

**Local environment check** — the product is the CLI; no service, database, or
digital twin exists. `cargo run -- archive --change <id>` runs locally, and
test scenarios run against temporary projects with the compiled binary.

**E2e isolation check** — `verify.command` (`./run-tests.sh`) and
`verify.e2e_command` (`./run-tests.sh --e2e`) are distinct; the archive
receipt's per-scope counts prove the e2e scope is strictly smaller (44
regular vs 4 e2e in the change scope).

## Use-Case And Persona Conformance

| Contract | Evidence | Pass? |
|---|---|---|
| Owning use-case guarantees and rules preserved | `archive-change` minimal guarantee rewritten to forward repair in the delta `usecase.md`; success guarantee binds the receipt to the tested commit | yes |
| Capability ownership and typed relationships preserved | deliverables routed under `archive-change`, `verify-change`, and the `support` fragment; fragment capabilities declared `NEW in fragments` | yes |
| Confirmed Persona-relevant outcomes preserved | no Personas confirmed; Actors remain Developer and Reviewer | n/a |
| Actors remain distinct from Persona biographies | no Persona appears in Gherkin | yes |
| Migration ledger and coverage evidence complete, if applicable | not a migration change | n/a |

**Design.md conformance check**

| Decision (from design.md) | design.md says | Actually built | Matches? | If not: updated + design-review re-run? |
|---|---|---|---|---|
| Git access | shell out to `git` CLI, no new crate | `src/git.rs` shells out | yes | |
| Candidate before verification | pending receipt + candidate commit, null commit ids | `Receipt::write_pending`, archive commits before scopes | yes | |
| Fail-closed result contract | scope mismatch, zero, failed, skipped, non-reconciling, malformed, missing all blocked | `proof::RunnerResult::read` unit tests + outline scenarios | yes | |
| Finalization guards before receipt | HEAD equality and evidence-only dirty set before proven write | `cli::archive::run` finalization section | yes | |
| Evidence predicate shared | one `is_evidence_path` for witness and guard | `src/proof.rs` | yes | |
| Test fixtures | git init + result runner migration | `tests/common/mod.rs`, `archive_proof_support.rs`, fixtures updated | yes | |
| Scope additions | three permanent-scenario deltas + ADR-070 accepted before archive | deltas + chapter 09 amendment | yes, re-reviewed | |

**Interaction coverage verification**

| Inventory entry | Matrix row exists? | @e2e scenario exists? (title) | Promised driving mechanism | Actual driving mechanism | Match? |
|---|---|---|---|---|---|
| archive a completed change | yes | Consumer archives a completed change and receives a proven archive | CLI subprocess | `common::run_givn(env, ["archive", ...])` | yes |
| repair a blocked archive | yes | Consumer repairs a blocked archive and the rerun proves it | CLI subprocess | `common::run_givn` after fixture repair commit | yes |
| inspect an unproven archive candidate | yes | Consumer inspects an unproven archive candidate | CLI subprocess | `common::run_givn(env, ["status"])` | yes |
| archive a new capability / without routing / multi-file | yes (carried-over rows) | existing grouped-merge @e2e scenarios | CLI subprocess | `common::run_givn` archive | yes |
| archive a modified scenario | yes | existing preserve-docstrings @e2e | CLI subprocess | `common::run_givn` archive | yes |
| archive with coverage | yes | existing extract-coverage @e2e | CLI subprocess | `common::run_givn` archive | yes |
| archive an E2E delta | yes | modified enforce @e2e | CLI subprocess | `common::run_givn` archive | yes |

## Retirements

RETIRE: none — the change removes no capability; it retires individual
rollback scenarios via `@givn.removed` deltas.

## Arc42 implementation conformance

The independent 12-row derivation from proposal/spec/design matches
`arc42.md`: chapters 2, 3, 4, 5, 6, 8, 9, 10, 11, and 12 affected; 1 and 7
not. Content checks:

- Ch 2: TC-31/TC-32 present; gate bullets match `src/cli/check.rs` and
  `src/cli/archive.rs`.
- Ch 3: Git entity, `GIVN_RESULT_FILE`, and archive output row present.
- Ch 5: `git` and `proof` rows present; archive/check/status rows match code.
- Ch 6: 6.7 forward-only sequence matches the archive implementation; 6.6 and
  6.14 updated.
- Ch 8: 8.6 provenance, 8.9 no-rollback, 8.9a finalization ordering, 8.12
  review checks all match code.
- Ch 9: ADR-070 accepted and registered; the deliberate early acceptance is
  recorded.
- Ch 10: Q-80..Q-84 present.
- Ch 11: R-5 rewritten; R-53..R-55 added.
- Ch 12: archive-proof vocabulary and invariants present.
- No ASCII-art diagrams were found in the 12 chapters; diagrams remain Mermaid.

| Arc42 chapter or architectural fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Completed implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Forward-only Git-backed publication | `02`, `06.7`, `09` | Affected | Yes | Yes | `src/cli/archive.rs`, `src/git.rs`, ADR-070 | yes |
| Receipt and result contract | `05`, `08.6` | Affected | Yes | Yes | `src/proof.rs` | yes |
| Review drops suite execution | `02.2`, `06.6`, `08.12` | Affected | Yes | Yes | `src/cli/check.rs` | yes |
| Coverage without rollback | `08.9` | Affected | Yes | Yes | `src/cli/coverage_gate.rs` | yes |
| Git prerequisite and repo-root rule | `02`, `03` | Affected | Yes | Yes | `src/git.rs` | yes |

ARC42 CONFORMANCE: CLEAN

## Coverage

Coverage is measured and published by the archive receipt. The last measured
run before this archive showed line coverage 35% (6461/18713) in the merged
report; the archive re-measures against the candidate and records the result
in `givn/archive/archive-proof/verification.json`. Review classified uncovered
regions from the change's new unit tests (`src/git.rs`, `src/proof.rs`),
the executable specs, and static inspection.

## Coverage gap classification

| Location | Lines | Classification | Action taken |
|---|---|---|---|
| `src/git.rs` error paths | error text | covered by archive-proof scenarios (non-repo, nested repo, rejected commit) | scenario + unit coverage |
| `src/proof.rs` validation branches | all | covered by unit tests and the result-contract outline | unit tests added |
| `src/cli/status.rs` / `instructions.rs` pending projection | branch | covered by two scenarios | scenarios added |
| `src/cli/coverage_gate.rs` failure path | branch | covered by the coverage-failure and rerun scenarios | scenarios added |
| Legacy `coverage/` directory entries | n/a | covered by `-uall` status listing | unit test updated |

No fourth-bucket excuses are used.

## Scenario execution evidence

Review does not re-run the suite. Change-scoped verification runs:

```text
GIVN_FEATURES=givn/changes/archive-proof/specs GIVN_TEST_RUNNER=cargo ./run-tests.sh
8 features / 44 scenarios (44 passed) / 270 steps

GIVN_FEATURES=givn/changes/archive-proof/specs GIVN_TEST_RUNNER=cargo ./run-tests.sh --e2e
2 features / 4 scenarios (4 passed) / 23 steps
```

The permanent corpus still shows the eight pre-merge scenarios this change's
deltas replace; the archive verifies the merged corpus, which is the real
full-suite authority.

**Receipt status:** unproven candidate — the archive writes the receipt.

## Overlap dispositions

The overlap gate passed with net delta 27 added, 7 modified, 4 removed. The
modified/removed pairs are this change's own delta operations (superseding
their pre-forward-only versions); no unresolved shape match remains.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A failed archive preserves the merged candidate unproven` | `Pre-merge archive failure preserves the permanent specification tree` | duplicate (superseding removed scenario) |
| `A failed coverage merge leaves the archive candidate unproven` | `A failed merge command blocks archive and preserves the project` | duplicate (superseding removed scenario) |
| `A failed new-capability candidate tells the developer how to repair it` | `New-capability merge failure produces clear recovery instructions` | duplicate (superseding removed scenario) |
| `Archive with coverage addon enabled still runs the coverage gate` | its pre-forward-only version | duplicate |
| `Archive runs e2e_command ... configured` | its pre-forward-only version | duplicate |
| `Archive blocked when merged permanent specs fail verify` | its rollback version | duplicate |
| `Archive re-runs verify ... passes` | its backup version | duplicate |
| `Integrity hook resolves the change directory ...` | its suite-expectation version | duplicate |

## Split-or-keep

| Scenario | Decision |
|---|---|
| `A scope result that does not prove the suite blocks proof` (outline) | keep |
| `Repairing a failed candidate forward proves it without repeating the merge` | keep |

## Sign-off

- [x] Fabrication audit: CLEAN (no empty step bodies; commits touch production
      code; promised components exist; no downgraded e2e scenarios; no
      over-produced e2e scope). Capability-level commit consolidation is
      recorded above as a process deviation.
- [x] Scenario execution evidence recorded per scope in the archive
      verification receipt (review does not run the suite).
- [x] Every coverage gap is classified and resolved (or justified) from the
      most recent measured evidence plus static inspection.
- [x] No `@wip` tags remain in the delta spec (`givn lint` clean).
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command starts the single CLI cleanly; no digital twins exist.
- [x] The exact file(s) `verify.e2e_command` invokes were identified
      (`tests/steps/archive_proof_e2e_steps.rs`); no parallel implementation.
- [x] `verify.e2e_command` is not identical to `verify.command`; per-scope
      receipt counts prove isolation.
- [x] Implementation matches design.md as reviewed (including the re-reviewed
      scope additions).
- [x] Interaction coverage verified: every inventory entry maps to a matrix
      row, an `@e2e` scenario, and CLI step definitions.
- [x] Design.md's `## Internal Primitives` section lists the new
      `git::Worktree`, `proof::*` types, and the evidence predicate.
- [x] arc42 slug-mapping verified; `ARC42 CONFORMANCE: CLEAN`.
- [x] No finding is excused with a classification outside the three buckets.

**REVIEW: PASS**
