# Review: release-0-3-0

## Fabrication Audit

This change adds release metadata, public migration guidance, a real CLI
upgrade path, and an explicit all-feature Cargo installation contract. The
release-specific production behavior is exercised through the compiled binary
in isolated temporary Git repositories.

**@e2e tag integrity:** The feature contains three intended interaction entries
and three matching `@e2e` scenarios. No planned `@e2e` tag was removed.

**Step definition scan** — both capability-specific files were checked for
empty bodies, `unimplemented!()`, `todo!()`, bare returns, and no-op bodies:

```text
tests/steps/release_0_3_0_steps.rs: 0 matches
tests/steps/release_0_3_0_e2e_steps.rs: 0 matches
0 empty or unimplemented step bodies found across 2 files.
```

All regular steps assert release documentation or use real fixture state. The
E2E steps invoke the compiled CLI and local Git executable and assert captured
exit status, output, files, and commit history.

**Commit cross-check**

| Scenario | Commit hash | Authoritative release/runtime source touched? |
|---|---|---|
| The release identifies itself as its package version | `3f02170` | Yes — package and lockfile metadata plus real CLI binding |
| The changelog describes the complete 0.3.0 release | `65d0b62` | Yes — public release artifact and executable documentation binding |
| The migration guide is an executable prompt sequence | `2eb8248` | Yes — public migration artifact and executable guide binding |
| The migration catalog defines complete release-owned bundles | `3989ebe` | Yes — embedded catalog, bundle assets, resolver, and executable catalog binding |
| Cargo installation enables every declared feature by default | `65ece69`, `426f2a1`, `082d3db` | Yes — Cargo default list, minimal-build documentation, executable release assertion, all-default lint cleanup, and the corrected blocking-pair scope |
| A clean 0.2.0 project receives a reviewable 0.3.0 migration plan | `6157355` | Yes — migration runtime, allowlist, Git commits, and CLI binding |
| A clean project without migration requirements receives one managed upgrade commit | `c678344` | Yes — managed-upgrade runtime, no-migration dispatch, CLI binding, and no-empty-commit unit boundary |
| An unclean project is protected from automatic migration commits | `86be977` | Yes — dirty-worktree runtime guard and CLI binding |
| Repeating the upgrade does not duplicate the migration | `8fa1b29` | Yes — idempotent runtime guard and CLI binding |
| A project outside Git is protected from a partial automatic migration | `56f8b09` | Yes — Git preflight diagnostic and CLI binding |
| An existing migration plan is never overwritten | `24bd7fa` | Yes — migration-path preservation and CLI binding |
| The migration change remains honest until the project review passes | `9193317` | Yes — generated plan contract and CLI/documentation binding |
| A completed maintenance migration can be archived without a product specification | `062eb7f` | Yes — archive/check runtime path and CLI binding |

Documentation scenarios do not touch Rust production code because their
authoritative product surface is the shipped changelog or migration guide;
their commits are not spec-only commits and their content is asserted by real
Gherkin steps.

**Promised-architecture cross-check**

| Component or source from `design.md` | Exists in tree? | Path |
|---|---|---|
| Legacy-version detection | Yes | `src/cli/upgrade.rs:active_config_version` |
| Migration orchestration | Yes | `src/cli/migration.rs` |
| Managed generated-path allowlist | Yes | `src/skills/targets.rs:managed_generated_paths` |
| Clean Git preflight and managed/aggregate commits | Yes | `src/cli/migration.rs` |
| Migration catalog and path resolver | Yes | `src/cli/migration.rs`, `assets/migrations/catalog.yaml` |
| Maintenance proposal/design/metadata generation | Yes | `src/cli/migration.rs` |
| Canonical LLM prompt pack | Yes | `docs/migrations/0.2.0-to-0.3.0.md` |
| Regular release step definitions | Yes | `tests/steps/release_0_3_0_steps.rs` |
| Separate CLI E2E step definitions | Yes | `tests/steps/release_0_3_0_e2e_steps.rs` |

**Strict-mode proof:** The setup proof in `tasks.md` ran the Cucumber runner
against an `unimplemented!()` step and recorded a non-zero result. The runner
uses `GivnWorld::cucumber().fail_on_skipped()`.

**Downgraded E2E scenario scan**

| @e2e Scenario | Real-interface assertion present? | Browser driver required? | Downgraded? |
|---|---|---|---|
| A clean 0.2.0 project receives a reviewable 0.3.0 migration plan | Yes — CLI exit status/output, config, generated plan, and local Git commits | No — CLI | No |
| A clean project without migration requirements receives one managed upgrade commit | Yes — CLI exit status/output, config, and local Git commit | No — CLI | No |
| A completed maintenance migration can be archived without a product specification | Yes — CLI archive result and permanent tree | No — CLI | No |

**E2E scope check**

| Capability | @e2e scenario count | Distinct actions? | Over-produced? |
|---|---:|---|---|
| 0.2.0-to-0.3.0 migration | 3 | Yes — prepare upgrade, managed-only refresh, and archive maintenance plan are distinct CLI actions | No |

**Duplicate-implementation check**

| Capability | File(s) `verify.e2e_command` invokes | Other implementation found? | Stronger one used instead? |
|---|---|---|---|
| Release migration | `./run-tests.sh --e2e` invokes `tests/features_runner.rs` with `tests/steps/release_0_3_0_e2e_steps.rs` registered | No parallel release-migration E2E implementation; existing E2E modules cover other capabilities | No alternative existed |

**Local environment check:** `./run-tests.sh` is the project-local command.
This capability starts no application server, database, queue, browser, or
third-party service. The temporary Git repository is created per scenario and
has no remote or shared state.

**E2E isolation check**

| `verify.command` value | `verify.e2e_command` value | Identical? | Full change-suite count | E2e-only count | Strictly smaller? |
|---|---|---|---:|---:|---|
| `./run-tests.sh` | `./run-tests.sh --e2e` | No | 13 scenarios, 13 passed | 3 scenarios, 3 passed | Yes |

The direct release runs proved 10 non-E2E scenarios and 3 E2E scenarios; the
review-gate run exercised the same 13-scenario feature scope.

**Design.md conformance check**

| Decision | `design.md` says | Actually built | Matches? |
|---|---|---|---|
| Version route | Catalog-selected route, including compatible skipped patches | `upgrade.rs` dispatches through `migration_path` | Yes |
| Git preflight | Repository and fully clean worktree before writes | `ensure_clean_git` checks repository and porcelain status | Yes |
| Generated scope | Explicit config/skill/command/owned-block allowlist | `managed_generated_paths` plus `givn/config.yaml` | Yes |
| Commit protocol | One managed commit without migration, or Commit A plus aggregate plan Commit B | `prepare_managed_upgrade`, `prepare_legacy_upgrade`, and `commit_migration_plan` | Yes |
| Plan shape | Proposal, design, `migration.yaml`, `.givn-skip: specs`; no fake evidence | `write_aggregate_migration_plan` and generated content | Yes |
| Blocking semantic scope | Active Delta relationships only; permanent-permanent pairs excluded | `embedding_similarity_candidates` filters projected pairs by active delta identity | Yes |
| Maintenance archive | Permanent verify path; no delta semantic gate or synthetic feature | `check.rs` and `archive.rs` inspect the skip marker | Yes |
| E2E bindings | Separate compiled-CLI capability file | `tests/steps/release_0_3_0_e2e_steps.rs` | Yes |
| Runner strictness | Cucumber `fail_on_skipped()` and real step bodies | `tests/features_runner.rs` and release bindings | Yes |

**Interaction coverage verification**

| Inventory entry | Matrix row? | Matching @e2e scenario? | Promised driving mechanism | Actual mechanism | Match? |
|---|---|---|---|---|---|
| run `givn upgrade` in a clean project created with Givn 0.2.0 | Yes | A clean 0.2.0 project receives a reviewable 0.3.0 migration plan | Compiled CLI subprocess plus local Git inspection | `run_givn` and fixture `git` subprocess | Yes |
| run `givn upgrade` in a clean project that needs managed guidance refresh but no project migration | Yes | A clean project without migration requirements receives one managed upgrade commit | Compiled CLI subprocess plus local Git inspection | `run_givn` and fixture `git` subprocess | Yes |
| archive a completed 0.2.0-to-0.3.0 maintenance migration without a product specification | Yes | A completed maintenance migration can be archived without a product specification | Compiled `givn archive` plus permanent-tree inspection | `run_givn` and recursive feature-file inspection | Yes |

## Arc42 Implementation Conformance

The Arc42 addon is enabled. All twelve rows in the change assessment are
affected and were independently checked against the durable chapters and
implementation:

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` treatment | `tasks.md` treatment | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Release/migration/install goal | `01-introduction-and-goals.md` | Yes | Overview, public contract, and default feature set | Release, guide, and Cargo scenarios | `Cargo.toml`, `CHANGELOG.md`, guide | Yes |
| Git and ownership constraint | `02-architecture-constraints.md` | Yes | Allowlist and clean preflight | Dirty/non-Git scenarios | `ensure_clean_git`, `stage_paths` | Yes |
| Upgrade interface | `03-context-and-scope.md` | Yes | CLI E2E surface | Clean upgrade scenario | `givn upgrade` subprocess | Yes |
| Separation strategy | `04-solution-strategy.md` | Yes | Managed-only versus catalog-selected aggregate phases | Upgrade and honesty scenarios | Managed commit, aggregate metadata, and plan content | Yes |
| Migration building block | `05-building-block-view.md` | Yes | `cli::migration` and embedded catalog responsibility | Catalog and clean upgrade scenarios | `src/cli/migration.rs`, `assets/migrations/` | Yes |
| Managed, aggregate, and archive flow | `06-runtime-view.md` | Yes | Route selection, managed commit, aggregate plan, and no-spec archive behavior | Upgrade/archive scenarios and catalog tests | `upgrade.rs`, `migration.rs`, `archive.rs`, `check.rs` | Yes |
| Binary/Git deployment boundary | `07-deployment-view.md` | Yes | Git requirement and generated footprint | Non-Git scenario | Git preflight | Yes |
| Cross-cutting ownership/idempotency | `08-crosscutting-concepts.md` | Yes | Config, allowlist, catalog, pending plan, and gates | Catalog, repeat, existing-plan, and managed-only scenarios | `migration_path`, `pending_migration_change`, snapshots | Yes |
| ADR-023/ADR-055 refinement | `09-architecture-decisions.md`, ADR-023, ADR-055 | Yes | `AMEND_ADR` to both existing decisions | ADR documentation task | ADR-023, ADR-055, and both indexes | Yes |
| Release quality scenarios | `10-quality-requirements.md` | Yes | E2E matrix, managed-only path, safety boundaries, and installation defaults | Q-48 through Q-56 | 13 passing scenarios | Yes |
| Migration risks | `11-risks-and-technical-debt.md` | Yes | Failure, marker, catalog, aggregate-order, and default-build-cost semantics | Review and archive tasks | R-27 through R-32 | Yes |
| Migration vocabulary | `12-glossary.md` | Yes | Managed upgrade, bundle/catalog, aggregate plan terminology | Catalog, honesty, and archive scenarios | glossary entries | Yes |

- [x] Every affected chapter was opened and checked for stale or placeholder
      content.
- [x] Every relevant architecture fact and ADR refinement is represented in
      `design.md` and mapped to tasks.
- [x] The implementation follows the documented ownership, Git, catalog,
      aggregate, managed-only, and no-product-spec archive decisions.
- [x] The durable chapters and change-level assessment agree.
- [x] No Arc42 mismatch remains.

ARC42 CONFORMANCE: CLEAN

## Coverage

**Full repository coverage:**

```text
non-E2E: 335 scenarios passed
E2E:      61 scenarios passed
merged:   13168 / 19656 lines covered = 66.9923%
branches: 0 / 0 (Cobertura emitted no branch nodes)
```

The global report is `coverage/cobertura-coverage.xml`, produced by
`CARGO_INCREMENTAL=0 ./measure-coverage.sh` and `./merge-coverages.sh`.

**Release-specific coverage:**

```text
release scope: 13 scenarios passed
merged release scope: 2610 / 19656 lines covered = 13.2784%
src/cli/migration.rs: 450 / 548 lines covered = 82.1168%
```

The release-specific report is `coverage/release-cobertura.xml`, produced by
the same compiled Cucumber runner with the release feature path. The global
report intentionally uses permanent `givn/specs/` and therefore does not
execute the still-active release delta; the separate report prevents the new
migration path from being misclassified as absent coverage.

## Coverage Gap Classification

| Location | Lines | Classification | Action taken |
|---|---:|---|---|
| Existing production modules outside the release delta | 6488 uncovered lines in the full repository report | Legitimately hard to test in this change | The permanent suite and prior change evidence own those paths; adding unrelated release scenarios would violate the release boundary and the black-box-first rule. |
| Release-specific existing runtime code not reached by the thirteen migration scenarios | 17046 lines in the release-only report | Legitimately hard to test in this change | The report is a change-scope diagnostic; existing code is covered by the full permanent suite, while the release scenarios cover every new externally observable route. |
| `src/cli/migration.rs` failure-only Git commit, snapshot-restore, and temporary-I/O branches | 98 uncovered lines in the release-specific report | Legitimately hard to test | These branches require injected filesystem failure, failing Git hooks, or missing commit identity while preserving the pre-write fixture. The success, managed-only, dirty, non-Git, existing-plan, idempotent, catalog, and archive boundaries are covered through the real CLI or focused catalog tests; the remaining fault-injection paths are explicitly isolated and do not alter user history. |
| Branch metric | 0 valid branch nodes emitted | Legitimately hard to test in this Cobertura format | The producer emitted `0/0`; no branch denominator exists to classify. |

No dead code was introduced and no redundant unit-test copy of a release
behavior was added. The two `active_config_version` unit tests cover parser
boundaries that are not distinct user actions.

## Semantic Review and Remediation

The blocking review gate generated and retained:

```text
Status: PASS
Candidate count: 0
```

The gate was run with the installed all-feature `givn 0.3.0` binary and
deterministic stub embedding with BGE/NLI disabled. The blocking path reported
zero candidates after permanent-permanent projected pairs were excluded. No
semantic report was silently omitted.

## Test Suite Result

```text
GIVN_FEATURES=givn/changes/release-0-3-0/specs cargo test --test features_runner -- --tags 'not @e2e'
10 scenarios passed; 44 steps passed

GIVN_FEATURES=givn/changes/release-0-3-0/specs cargo test --test features_runner -- --tags @e2e
3 scenarios passed; 19 steps passed

cargo test --lib
156 tests passed

cargo test --no-default-features --lib
151 tests passed

cargo fmt --all -- --check
passed

cargo clippy --workspace --all-targets -- -D warnings
passed

givn lint --change release-0-3-0
clean

GIVN_SPEC_EMBEDDER=stub GIVN_SPEC_BGE=disabled GIVN_SPEC_RERANKER=disabled givn check review --change release-0-3-0
verify, verify-e2e, integrity, overlap, and semantic review passed
```

**Result:** PASS

**Coverage measured:** YES

## Sign-off

- [x] Fabrication audit clean: no empty step bodies, all task commits and
      authoritative release/runtime paths checked, all promised components
      exist, and no downgraded or over-produced E2E scenarios remain.
- [x] All release scenarios pass through the configured Cucumber runner.
- [x] Every coverage region relevant to this change is measured or classified
      in one of the three permitted buckets.
- [x] No `@wip` tags remain in the release delta.
- [x] The release delta contains observable behavior only; implementation
      detail remains in `design.md` and the task plan.
- [x] Local runnability requires no external service and uses isolated local
      Git fixtures.
- [x] The exact E2E command and capability-specific implementation were read;
      no parallel implementation exists.
- [x] `verify.e2e_command` is distinct from `verify.command` and its count is
      strictly smaller.
- [x] Implementation commands, file layout, framework, and interface match
      `design.md` after the follow-up archive hardening review.
- [x] Every interaction inventory entry maps to the design matrix, a matching
      E2E scenario, and the promised real CLI mechanism.
- [x] `design.md` contains `## Internal Primitives` and those primitives exist
      in the implementation.
- [x] Every Arc42 slug maps to an existing durable chapter file.
- [x] No finding is excused with a fourth classification bucket.

REVIEW: PASS
