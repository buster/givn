# Review: drop-legacy-residue

Completeness-based coverage review against the committed candidate at
`fa900835` (branch `list-givn-features`). All 111 tasks are checked;
`givn check tasks` passes; both change-scoped gates were re-run by this
review and pass. The first review pass at `de8adb86` returned
`REVIEW: FAIL` with four findings; the remediation commits `8cb752a4`,
`982c0ed3`, `33cbe10f`, `4ea92522`, `fa900835` repair them, and this
re-audit verifies each repair independently (see Resolved findings).

## Fabrication audit (performed first)

**Step-definition scan.** 0 empty step bodies found across all 67
step-definition files under `tests/steps/` (re-scanned by this re-audit:
the only empty body is the `#[cfg(not(unix))]` Windows no-op of the
`make_executable` fixture helper in `llm_attribution_e2e_steps.rs` — a
platform-conditional stub whose Unix branch does the real work,
pre-existing and untouched by this change). The touched files
(`release_0_3_0_steps.rs`, `release_0_3_0_e2e_steps.rs`,
`enforce_post_merge_e2e_gate_steps.rs`, `grouped_merge_steps.rs`,
`local_only_rules_steps.rs`, `extract_coverage_gate_steps.rs`,
`deduplicate_change_dir_detection_steps.rs`, `mod.rs`) were read line by
line: no `{}` bodies, no bare `pass`/`return`, no whitespace-only bodies,
no assertion-free bodies. No `unimplemented!()` or `todo!()` remains
anywhere in `src/` or `tests/` (the strictness probes were removed after
use, as the tasks record). The `Given a placeholder step` occurrences are
fixture-file string literals, not executed step bodies; the runner filters
`@givn.removed` placeholders out (`tests/features_runner.rs`,
`is_removed_placeholder`), so the four removed scenarios never execute.

**@e2e tag integrity.** No delta scenario's tags were changed from
including `@e2e` to excluding it. The delta carries six `@e2e` units: two
added (`A clean 0.7.0 project receives a reviewable aggregate migration
plan` — a Scenario Outline counting as one evidence unit per the canonical
policy; `A consumer invoking the retired history rewrite command is told it
does not exist`) and four re-pointed `@givn.modified` scenarios that keep
their tags. The removed 0.2.0 outline was itself `@e2e` in the permanent
corpus and is replaced by the added 0.7.0 `@e2e` outline; no tag was
stripped to bypass the verify-e2e gate. `givn lint --change
drop-legacy-residue` exits 0 ("8 file(s) checked — clean").

**Commit cross-check.** Every recorded hash resolves; every scenario
section's commit is present and its diff is consistent with the section's
GREEN claim. Reuse/setup/bookkeeping commits that touch only the spec or
tasks.md are declared as such by the design and are listed with their
enabling commit:

| Task / section | Commit | Production/fixture delta (beyond spec/tasks) |
|---|---|---|
| Setup: strict mode proof | `743028ff` | none — probe added then removed; tasks.md evidence (by design) |
| Archive is blocked… (regular) | `8bdcccac` | `tests/steps/enforce_post_merge_e2e_gate_steps.rs` (Given rename) |
| Archive succeeds… (reuse) | `e4a9ce4a` | none — pure reuse; enabling rename in `8bdcccac` |
| Retiring a use case's last capability… | `6047f6e6` | `src/cli/archive.rs` (retirement extension + unit tests), `tests/steps/grouped_merge_steps.rs` |
| Verify-e2e hook resolves… (reuse) | `dacdfd43` | none — pure reuse; enabling rename in `8bdcccac` |
| The migration guide is an executable prompt sequence | `e4235ed2` | `tests/steps/release_0_3_0_steps.rs` (re-point + assertions) |
| A project below the oldest migration route… | `ed00315e` | `assets/migrations/catalog.yaml`, five bundle dirs deleted, `src/cli/migration.rs` (unit tests), `tests/steps/release_0_3_0_steps.rs`, `tests/steps/release_0_3_0_e2e_steps.rs` |
| An unclean project is protected… | `efbe9164` | `tests/steps/release_0_3_0_e2e_steps.rs` (0.7.0 fixture) |
| A project outside Git is protected… | `c97bc27f` | `tests/steps/release_0_3_0_e2e_steps.rs` (0.7.0 fixture) |
| An existing migration plan is never overwritten | `320e09c1` | `tests/steps/release_0_3_0_steps.rs` (0.7.0 fixture) |
| A Fossil project with uncommitted changes… | `111bac30` | `tests/steps/release_0_3_0_e2e_steps.rs` (0.7.0 Fossil fixture) |
| E2E setup and subset proof | `3d15bda9` | none — probe added then removed; tasks.md evidence (by design) |
| Consumer archives a delta that adds e2e scenarios… (@e2e) | `0dbf6991` | `tests/steps/enforce_post_merge_e2e_gate_steps.rs` (Given rename) + transcript |
| Consumer archives a new capability… (@e2e) | `77de1c24` | `tests/steps/enforce_post_merge_e2e_gate_steps.rs` (shared Given rename) + transcript |
| Archive is blocked when a new capability… (@e2e, reuse) | `a31ec468` | none — pure reuse; enabling rename in `77de1c24`; transcript |
| Nothing is merged when one delta file… (@e2e, reuse) | `591338fb` | none — pure reuse; enabling rename in `77de1c24`; transcript |
| A clean 0.7.0 project receives a reviewable aggregate migration plan (@e2e) | `bf8262fb` | `tests/steps/release_0_3_0_e2e_steps.rs` (0.7.0 Git/Fossil fixtures, assertion updates) + transcript |
| A consumer invoking the retired history rewrite command… (@e2e) | `8666d3c3` | `src/main.rs`, `src/cli/mod.rs`, `src/cli/rewrite_history.rs` (deleted), `tests/steps/release_0_3_0_steps.rs`, `tests/steps/local_only_rules_steps.rs` + transcript |
| Residue sweep: docs and dead guides | `89029a6d` | `README.md`, `CHANGELOG.md`, `docs/INDEX.md`, `docs/reference.md`, `docs/workflow.md`, `docs/migrations/README.md`, `src/cli/migration.rs`, two guide deletions |
| Residue sweep: served guidance | `30387ed9` | `assets/instructions/grill-with-docs.md`, `assets/instructions/review.md`, `assets/instructions/specs.md`, `assets/instructions/tasks.md`, `assets/templates/design.md`, `assets/templates/review.md` |
| Residue sweep: configuration scaffold | `b2eeae29` | `assets/config.yaml.tmpl`, `assets/commands.yaml.tmpl` |
| Residue sweep: AGENTS.md and CI prerequisites | `92fa89c9` | `AGENTS.md`, `.github/workflows/ci.yml`, `.github/workflows/release.yml` |
| Arc42 documentation update | `802de3f6` | 13 files under `docs/arc42/` |
| Remove the dead extras flag | `80927541` | `src/config/manifest.rs`, `src/config/overlay.rs`, `src/cli/graph.rs`, `src/hooks.rs`, `tests/steps/mod.rs` (also carries the planning artifacts) |
| Remove the legacy coverage schema and in-tool merge | `d3c74f7f` | `src/config/manifest.rs`, `src/cli/coverage_gate.rs`, `src/cobertura.rs`, `src/integrity_hook.rs`, `tests/steps/extract_coverage_gate_steps.rs` |
| Remove the legacy attribution-trailer prefixes | `00fac4fc` | `src/cli/attribution.rs` |
| Bookkeeping: merge records and tombstone | `afdcdef3` | none — merge-record verification; tasks.md evidence (by design) |

Post-bookkeeping, two commits are not recorded in tasks.md: `5198293a`
(fills in the `afdcdef3` hash reference in tasks.md — the standard
record-after-commit pattern) and `de8adb86` (`style(release-steps)`:
rustfmt-only reformat of `tests/steps/release_0_3_0_steps.rs`, 10
insertions, 6 deletions, no behavior change). Neither invalidates any
recorded evidence; both are cosmetic and noted here for completeness.

The five remediation commits after the first review pass are recorded in
the Review remediation section of tasks.md: `8cb752a4` (finding-1 error
strings), `982c0ed3` (finding-2 help text), `33cbe10f` (finding-1 pin),
`4ea92522` (findings 3-4 unit tests), `fa900835` (bookkeeping). Their
diffs are confined to exactly the files the remediation claims:
`src/cli/archive.rs`, `src/hooks.rs`, `src/main.rs`,
`src/config/overlay.rs`, the blocked-runner delta feature file, the
blocked-runner step file, the re-captured transcript, and tasks.md
(verified by `git diff de8adb86 HEAD`). No tag line, no other scenario,
and no other production file changed.

**Promised-architecture cross-check.** Every component the design promised
exists, and every deletion is real: `src/cli/rewrite_history.rs` is gone
and `RewriteHistory` has no remaining references in `src/main.rs` /
`src/cli/mod.rs`; `assets/migrations/` holds only `0.7.0-to-0.8.0/`,
`0.8.0-to-0.9.0/`, `catalog.yaml` and the catalog's first entry is
`migrate-0-7-0-to-0-8-0`; `docs/migrations/` holds only the two retained
guides plus the README; `MIGRATION_GUIDE` is gone from
`src/cli/migration.rs`; `Manifest.extras` / `OverlayDoc.extras` /
`Coverage.non_e2e_command` / `Coverage.e2e_command` are gone;
`merge_legacy_reports`, `CoberturaReport.line_hits`, `CoberturaLine.number`
are gone from `src/cobertura.rs`; the legacy probe/branch and
`remove_legacy_coverage_badges` are gone from `src/cli/coverage_gate.rs`;
`LEGACY_TRAILER_PREFIXES` is gone from `src/cli/attribution.rs`;
`legacy_configured` and the legacy acceptance sentence are gone from
`src/integrity_hook.rs`; `src/cli/readme.rs` keeps `LEGACY_TEXTS` unchanged
as designed; the retirement extension in `src/cli/archive.rs` (the
`Some(usecase_dir)` arm: `fragments` exemption, `holds_feature_files`
check, `usecase.md` removal and directory removal with `.with_context()`)
is present with its three unit tests (`retirement_deletes_capability_and_writes_tombstone`,
`retirement_of_shared_usecase_keeps_directory_and_narrative`,
`retirement_targets_grouped_capability_directory`); the orphaned steps are
deleted (`manifest_extras_enabled` and the section label rename in
`tests/steps/mod.rs`; the two reachable-history steps in
`tests/steps/local_only_rules_steps.rs`; the six legacy fixture helpers in
`tests/steps/extract_coverage_gate_steps.rs`).

**Strict-mode proof.** Present in the setup task of tasks.md with pasted
outputs: the temporary probe scenario failed both `./run-tests.sh` and
`./run-tests.sh --e2e` invocations with non-zero exits, and the targeted
libtest run shows the panic. The e2e-setup task records the same probe
against the `@e2e` invocation. This review re-ran both scoped gates after
the remediation:

```text
$ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh
    Summary [   1.567s] 16 tests run: 16 passed, 0 skipped   (exit 0)

$ GIVN_FEATURES=givn/changes/drop-legacy-residue/specs ./run-tests.sh --e2e
    Summary [   1.270s] 6 tests run: 6 passed, 10 skipped    (exit 0)
```

**Downgraded-e2e scan.** Every delta `@e2e` scenario's Then steps assert
the real CLI boundary (exit code, stdout/stderr content, on-disk effects);
none asserts repository state as its only assertion:

| @e2e scenario | Primary assertion | Real-interface driver | Downgraded? |
|---|---|---|---|
| Consumer archives a delta that adds e2e scenarios and is blocked without a configured runner | command fails; stderr says the e2e runner is not configured and names givn/commands.yaml as the runner's configuration file; change dir survives; no candidate | `CARGO_BIN_EXE_givn` subprocess (`givn archive`) | no |
| Consumer archives a new capability and the merged spec lands in its declared use case | exit 0; merged file exists; no flat dir; dir moved to archive | `CARGO_BIN_EXE_givn` subprocess | no |
| Archive is blocked when a new capability has no declared routing decision | exit non-zero; stderr names the refusal; nothing merged | `CARGO_BIN_EXE_givn` subprocess | no |
| Nothing is merged when one delta file of a change has no routing decision | exit non-zero; routed file absent; change dir survives | `CARGO_BIN_EXE_givn` subprocess | no |
| A clean 0.7.0 project receives a reviewable aggregate migration plan | upgrade exit 0; config version; commit shape; migration change contents (Git and Fossil Examples) | `CARGO_BIN_EXE_givn` subprocess via `common::run_givn` | no |
| A consumer invoking the retired history rewrite command is told it does not exist | exit non-zero; stderr reports "rewrite-history" is not an available command | `CARGO_BIN_EXE_givn` subprocess (`.args(["rewrite-history"])`) | no |

This is a CLI capability, not a browser UI: nothing renders HTML, CSS, or
JavaScript, so the browser-driver rule does not apply, and the CLI
subprocess is the real interface.

**E2E scope check (one @e2e per normalized action).** The delta's six
`@e2e` units map to distinct normalized consumer actions: plan a release
migration (0.7.0 outline — the removed 0.2.0 outline is its superseded
predecessor, same action), invoke the retired history rewrite command (new
action), and the four re-pointed archive actions (blocked-runner archive,
grouped-merge success, unrouted-capability refusal, multi-delta
atomicity). Variants and error cases of the same action are regular
scenarios. No capability over-produces or under-produces.

**Duplicate-implementation check.** `verify.e2e_command` is
`./run-tests.sh --e2e` (`givn/commands.yaml`); `run-tests.sh --e2e` runs
`cargo nextest run --test features_runner` with `GIVN_CUCUMBER_TAGS='@e2e'`
against `GIVN_FEATURES`. The runner is `tests/features_runner.rs`
(cucumber-rs, `harness = false`, `.fail_on_skipped()`). The e2e step
definitions for this change's capabilities live in
`tests/steps/release_0_3_0_e2e_steps.rs` and the shared Given in
`tests/steps/enforce_post_merge_e2e_gate_steps.rs` — exactly the files the
design names; no new step-definition file exists. The tree (including
untracked files — `git status` is clean) holds no second or parallel e2e
implementation for these capabilities. The tree was searched including
untracked files: the only untracked file at review time was this
`review.md`; `git status` is otherwise clean.

**Local environment check.** Both scoped gates pass from the local
checkout (outputs above); the e2e scenarios spawn the compiled
`CARGO_BIN_EXE_givn` plus `git`/`fossil`/`script` on `PATH`; no server,
container, network, or digital twin is required, matching the design's
Local Runnability section.

**E2E isolation check.** `verify.command` is `./run-tests.sh`;
`verify.e2e_command` is `./run-tests.sh --e2e` — different strings, and the
`--e2e` flag selects the `@e2e` tag expression inside `run-tests.sh`. This
change is not yet archived, so no verification receipt exists; the design's
subset proof is re-verified by this review's own runs: regular scope 16
scenarios, e2e scope 6 scenarios, 6 < 16 — the e2e scope is a strict
subset. The archive receipt will carry the per-scope counts.

**Design.md conformance.** The as-built commands, file layout, and
framework match what design.md explicitly named: `verify.e2e_command:
./run-tests.sh --e2e` (with the `@e2e` tag filter, proven by the subset
counts); per-capability step files with no change-wide file; cucumber-rs
through `features_runner`; strict mode `.fail_on_skipped()`. One recorded
attribution nuance is documented in tasks.md (both
`current_version_migration_change_exists` and `migration_contains_plan`
live in `release_0_3_0_e2e_steps.rs`, where they already existed — the
design's fixture table names both files for the capability without
per-function attribution), which is a location note, not a design
deviation. No silent shortcut exists.

**Interaction coverage verification.** The owning `## Interactions`
inventory (delta `specs/upgrade-a-project/usecase.md`, six rows) maps to
the design's Interaction Coverage Matrix and to real `@e2e` scenarios with
the promised driving mechanism:

| Inventory entry | Matrix row | @e2e scenario | Driving mechanism in step file | Match? |
|---|---|---|---|---|
| apply-a-managed-upgrade: plan a release migration | yes | A clean 0.7.0 project receives a reviewable aggregate migration plan (outline, Git+Fossil) | real `givn upgrade` subprocess (`common::run_givn` → `CARGO_BIN_EXE_givn`) | yes |
| apply-a-managed-upgrade: run a managed upgrade | yes | A clean project without migration requirements receives one managed upgrade commit (permanent, @e2e) | real `givn upgrade` subprocess | yes |
| apply-a-managed-upgrade: upgrade a Fossil project | yes | A clean Fossil project receives the same managed upgrade revision shape (permanent, @e2e) | real `givn upgrade` subprocess | yes |
| apply-a-managed-upgrade: invoke a retired engine command | yes | A consumer invoking a retired engine command is told it does not exist (permanent, @e2e) | compiled binary invoked as `givn spec index` | yes |
| apply-a-managed-upgrade: invoke the retired history rewrite command | yes | A consumer invoking the retired history rewrite command is told it does not exist (added, @e2e) | compiled binary invoked as `givn rewrite-history` (`tests/steps/release_0_3_0_steps.rs:388-405`) | yes |
| apply-a-managed-upgrade: archive a maintenance migration | yes | A completed maintenance migration can be archived without a product specification (permanent, @e2e) | real `givn archive` subprocess | yes |
| extract-coverage-gate: archive with coverage | yes | Consumer archives a change with coverage enabled and the coverage summary appears in README (permanent, @e2e) | real `givn archive` subprocess with script-based coverage | yes |
| describe-the-release: generate the release section | yes | Repeated revisions of one change yield one changelog entry (permanent, @e2e) | changelog generation over a real revision range (git-cliff fixture) | yes |
| fragment customize-the-lifecycle-graph | matrix row: no inventory row (fragments carry no interactions) | — | in-process overlay application via the shared overlay steps | yes |

The added retirement scenario deliberately creates no inventory entry and
is not `@e2e` (recorded in the design's retirement mechanics and the
design-review), so no matrix row is missing or extra. No driving-mechanism
mismatch: every promised mechanism is the one implemented.

## Fabrication-audit findings — resolved

The first review pass found four defects; none was a fabricated step, a
fake run, or a spec-only bluff — they were contract and coverage gaps the
implementation left open. The remediation pass repaired all four, and this
re-audit verifies each repair independently, not from the remediation's
own claims:

1. **R1 — production error strings misattributed `verify.e2e_command` to
   `givn/config.yaml` (finding 1). Resolved.** `src/cli/archive.rs`
   (`ensure_e2e_configured`), `src/hooks.rs` (missing-runner bail), and
   `src/hooks.rs` (tag-removal bail) now all say
   "configured in givn/commands.yaml" (commit `8cb752a4`). Independent
   verification: a sweep of every remaining `givn/config.yaml` string in
   `src/` (55 sites) found only legitimate references — `addons`,
   `skills`, `extensions`, the version marker, the init/upgrade scaffold
   writers, the migration `git add`, and the eject manifest note; no
   remaining site attributes `verify.*`, `coverage.*`, or `run:` keys to
   `givn/config.yaml` (the coverage gate, the run-declaration gate, the
   missing-testrunner sentinel, and the missing-run-command sentinel all
   name `givn/commands.yaml`). The attribution is now pinned by scenario
   (see R3).
2. **R2 — `givn spec --help` advertised the removed retrieval index
   (finding 2). Resolved.** The `Spec` variant's doc comment in
   `src/main.rs` now reads "Work with the permanent spec corpus: coverage
   inventory, duplicate report, use-case proposals, tree overview, lexical
   scenario search, and proposal routing." (commit `982c0ed3`). Independent
   verification: rebuilt the binary (`cargo build --locked`, exit 0) and
   ran `./target/debug/givn spec --help` — it prints exactly that text
   followed by the six real subcommands (`coverage`, `duplicates`,
   `propose-usecases`, `tree`, `find`, `route`) with no retrieval/index
   vocabulary.
3. **R3 — the corrected attribution is now pinned by the blocked-runner
   e2e scenario (finding 1 pin). Resolved.** The delta scenario "Consumer
   archives a delta that adds e2e scenarios and is blocked without a
   configured runner" (`@givn.modified @e2e` — tag untouched) gains the
   Then step "stderr names givn/commands.yaml as the e2e runner's
   configuration file", implemented in
   `tests/steps/enforce_post_merge_e2e_gate_steps.rs`
   (`e2e_stderr_names_commands_yaml`, `s.contains("givn/commands.yaml")`)
   (commit `33cbe10f`). Independent verification:
   - Re-ran the scenario targeted
     (`GIVN_FEATURES=givn/changes/drop-legacy-residue/specs cargo test
     --test features_runner -- --name '^Consumer archives a delta that
     adds e2e scenarios and is blocked without a configured runner$'`):
     1 scenario, 9 steps, all passed, exit 0.
   - The re-captured transcript diff versus the pre-repair commit
     `0dbf6991` is exactly timestamps, the temp-dir suffix, and
     `givn/config.yaml` → `givn/commands.yaml` in the stderr line — no
     other content change; the committed pre-repair stderr line does not
     contain the substring `givn/commands.yaml`, so the new assertion
     would have failed against the old string (the pin is genuine).
   - The scenario passes under the e2e gate protocol (6/6 green in the
     e2e gate run below).
4. **R4 — the D2 config-refusal and D1 extras-tolerance consequences now
   have executable coverage (findings 3 and 4). Resolved.** Two unit
   tests in the existing `src/config/overlay.rs` test module (commit
   `4ea92522`):
   - `coverage_refuses_legacy_two_command_keys_and_accepts_the_surviving_schema`
     asserts that an overlay carrying `coverage: {non_e2e_command: "x"}`
     (and likewise `e2e_command`) fails through the strict `Coverage`
     deserializer with the error naming the offending key, while
     `measure_command` / `merge_command` still parse and land on the
     effective manifest.
   - `stray_extras_key_parses_and_has_no_effect` asserts that an overlay
     carrying a stray top-level `extras: true` parses and the effective
     manifest serializes byte-identical to the base.
   Independent verification of the pin strength (by inspection, not by
   reverting): `Coverage` carries `#[serde(deny_unknown_fields)]` and
   holds only the surviving keys, so the refusal test genuinely fails if
   the strict schema were relaxed (dropping `deny_unknown_fields` or
   re-adding a key makes `apply` succeed and both `unwrap_err()` calls
   panic). `OverlayDoc` is the lenient document (no `deny_unknown_fields`,
   no `extras` field), so the tolerance test's byte-identical assertion
   genuinely pins "parses and carries no effect". Both tests run and pass:
   `cargo test --lib config::overlay` → 22 passed, 0 failed (exit 0).

**Re-audit regression battery.** After the four repairs:
`cargo fmt --check` (exit 0); `cargo clippy --workspace --all-targets --
-D warnings` (exit 0, no warnings); the regular scoped gate (16/16 green,
exit 0); the e2e scoped gate (6/6 green, 10 skipped, exit 0). The working
tree is clean apart from this `review.md` until it is committed. The
`@e2e` tag set at HEAD is byte-identical to the set at the first-pass
candidate `de8adb86` — no tag was removed.

## Arc42 implementation conformance

The change-level `arc42.md` marks chapters 1, 2, 3, 4, 5, 6, 7, 8, 9
(register row only), 10, 11, 12 plus the arc42 README and the ADR index.
Every marked chapter was opened and cross-referenced against `design.md`,
`tasks.md`, and the committed implementation:

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| FR-33 and the migrating-maintainer row | 01 | FR-33 requires the changelog and release-owned migration routes | D6 re-points the guide scenario; the 0.2.0 guide leaves | docs sweep task | chapter 01:55-56 names release-owned routes without the 0.3.0 pin | yes |
| Section 2.7 migration-ownership constraint | 02 | version pin removed | D4: oldest source 0.7.0 | catalog deletion task | chapter 02:99-102 names the automatic migration route unpinned | yes |
| Upgrade interface row and in/out scope | 03 | declared route + below-route managed refresh | D4 | no-route scenario section | chapter 03:56,99 names the catalog-declared floor (0.7.0) and the managed refresh | yes |
| Strategy rows | 04 | declared route (currently 0.7.0) + below-route refresh | D4 | same | chapter 04:27,73 | yes |
| `cli::rewrite_history` module row | 05 | row deleted; stamper keeps only the managed co-author line | production table | retired-command + attribution tasks | no `rewrite` mention remains in chapter 05 | yes |
| Init sequence addon registry | 06 | step 6 names the current addon registry | scaffold task | config scaffold sweep | chapter 06 step 6 lists Arc42, Absolute Mode, README, coverage, the four project-level skills/commands | yes |
| Section 7.6 route and revisions | 07 | declared route, two revisions, below-route refresh | D4 | same | chapter 07:94 | yes |
| Rules policy 8.7, coverage merge 8.9, attribution 8.12a | 08 | report-only cleanliness; project-owned merge arithmetic; stamper replaces only the managed line | decisions + production table | rewrite-history, coverage, attribution tasks | chapter 08:314 ("Givn itself never rewrites history"), 08.9 ("the merge arithmetic belongs to the project's merge command"), 8.12a ("stamps only the managed co-author attribution") | yes |
| Chapter 9 register evidence row | 09 | evidence row links `docs/migrations/README.md`; no new MADR | D6 | docs sweep | chapter 09:94 no longer links the deleted guide | yes |
| Q-48, Q-55, Q-77 | 10 | scenarios pin 0.7.0, the 0.8.0-to-0.9.0 skip route, the current bundle | D4, D6 | fixture tasks | chapter 10:60 | yes |
| R-29, R-81, R-82, 11.4, D-3 repaid, R-7 refreshed | 11 | restated marker; refusal risk; report-only boundary; retirement + floor; scaffold debt repaid | D1, D2, decisions | coverage, scaffold tasks | chapter 11:16 (R-7), 11:68 (R-81), 11:69 (R-82), 11.4 records the retirement and 0.7.0 floor, line 111 records D-3 repaid and removed | yes |
| Glossary rows | 12 | `History rewrite` row deleted; `Clean history` states Givn never rewrites history | D5, decisions | bookkeeping task | no `History rewrite` row remains; `Clean history` (12:57) carries the report-only statement; `Migration catalog` (12:114) holds the floor's declared-edge concept | yes |
| Config-file attribution of runner values | 2, 4, 6, 8, glossary | chapters already attribute them to `givn/commands.yaml`; no chapter edited | spec-text correction | Given renames | chapters 2/4/6/8 and the glossary's **Manifest overlay** name the commands file as the later overlay home | yes |

One pre-existing observation outside the affected set: ADR-037
(`docs/arc42/adr/ADR-037-deterministic-overlap-fingerprint.md`) still
cross-references "chapter 6 §6.8 retrieval-primitives", a section the
earlier `drop-embedding-features` change removed. This change did not
touch ADR-037, did not remove §6.8, and does not mark the ADR file
affected; the stale cross-reference belongs to the earlier change's
cleanup and does not contradict any row of this change's assessment.

ARC42 CONFORMANCE: CLEAN (for the affected set; the ADR-037 cross-reference
is recorded above as pre-existing and outside this change's affected
chapters).

## Ubiquitous language conformance

The delta specs, design, tasks, and code reuse the durable glossary terms:
**Migration catalog** (the floor is phrased as "the oldest source the
migration catalog declares (0.7.0)", holding the design-review Q3 decision
— no new glossary row), **Managed upgrade**, **Managed upgrade path**,
**Maintenance change**, **Migration bundle**, **Aggregate migration
change**, **Clean history** (updated to "Givn itself never rewrites
history"), **Manifest overlay** ("givn/config.yaml is applied first;
givn/commands.yaml is applied later"), **Verification scope** /
**Verification receipt**, **Delta**, **Tombstone**-adjacent vocabulary
(the `@givn.retired` tombstone and `.retired.yaml` are established
retirement vocabulary from `retire-semantic-capabilities` and
`drop-embedding-features`, not a new term). The scenario steps stay in
domain language ("the consumer invokes `givn upgrade`", "the tombstone
records capability") with no implementation-layer detail. No spec or
design term is missing from the glossary and no reuse is inconsistent.

## Coverage

No measured coverage evidence exists yet for this change (it is not
archived; the archive receipt will carry the instrumented result). The
classification below is from static inspection against the committed
candidate.

**Bucket 1 — dead code, deleted by this change.** The rewrite-history
module and clap variant; the five pre-0.8 bundles and their catalog
entries; the two dead guides; `MIGRATION_GUIDE` and its substitution;
`Manifest.extras` / `OverlayDoc.extras`; `Coverage.non_e2e_command` /
`Coverage.e2e_command`; `merge_legacy_reports`, `CoberturaReport.line_hits`,
`CoberturaLine.number`, the legacy coverage gate probe/branch/
`write_legacy_merged_report`/`remove_legacy_coverage_badges`;
`LEGACY_TRAILER_PREFIXES`; the orphaned step definitions
(`manifest_extras_enabled`, the two reachable-history steps, the six
legacy coverage fixture helpers). Their scenarios go with them as
`@givn.removed` placeholders — the `.feature` file remains the source of
truth and no redundant unit test survived (the removed legacy unit tests
had no surviving subject).

**Bucket 2 — missing coverage, four findings (resolved by the remediation
and re-verified here).**
1. The three production error strings misattributing `verify.e2e_command`
   to `givn/config.yaml` (`src/cli/archive.rs`, `src/hooks.rs` ×2) —
   corrected to `givn/commands.yaml` (commit `8cb752a4`) and pinned by
   the blocked-runner scenario's new Then step (commit `33cbe10f`),
   verified by the targeted scenario run (exit 0) and by the source
   sweep above.
2. The `givn spec` help text advertising the removed retrieval index
   (`src/main.rs`) — corrected (commit `982c0ed3`), verified by building
   and running `givn spec --help` (six real subcommands, no retrieval
   vocabulary).
3. The D2 config-refusal contract (legacy coverage keys refused on load,
   risk R-81) — pinned by the strict-deserializer unit test (commit
   `4ea92522`); the test genuinely fails if the strict schema is relaxed
   (`Coverage` carries `deny_unknown_fields`), verified by inspection.
4. The D1 tolerance contract (stale `extras:` accepted and ignored) —
   pinned by the lenient-overlay unit test asserting a byte-identical
   effective manifest (commit `4ea92522`).

None of the four is excusable as "acceptable for now", "future scope", or
"sufficient for current scope", and none is legitimately hard to test
(bucket 3 does not apply — each is a one-assertion pin). All four are
resolved; no bucket-2 finding remains open.

**Bucket 3 — legitimately hard to test.** None.

The documentation, served-guidance, and scaffold prose corrections are
not behavior: the behaviors they describe are pinned by the existing
scenarios (reference resolution, addon-conditional rendering, manifest
schema) and the design's served-guidance spec coverage audit; pinning
incidental prose wording would over-fit, so no coverage gap attaches to
them.

## E2E coverage review

The canonical E2E policy (`givn instructions specs --change
drop-legacy-residue`) was applied. The opposite classification is a Web
UI: a rendered page in a browser. It does not hold — this change renders
nothing in HTML, CSS, or JavaScript; its consumer surface is CLI text
output (a retired command's refusal, the upgrade's migration-plan output,
archive gate refusals) and repository text. The real interface is the
compiled `givn` binary, and every delta `@e2e` scenario spawns it
(`CARGO_BIN_EXE_givn`) in a fixture project and asserts exit code, raw
stdout/stderr, and on-disk effects — the primary assertions are on what
the CLI printed or did, never on repository state alone (table in the
fabrication audit). The driver is the one the design reviewed; no HTTP
client or in-process fallback is used. One `@e2e` scenario per normalized
action; variants are regular scenarios. The six transcripts are the
terminal evidence (next section).

## Deferrals

None requested, none recorded. Every mandatory check was performed.

## Use-case and Persona conformance

**Scope rejection criteria** (run against the use cases the change
touches — `upgrade-a-project`, `publish-a-release`, `archive-change`,
`verify-change`, and the `support` fragment):
- Goal conjunction: `upgrade-a-project`'s goal is one verb phrase ("Cross
  a release gap with reviewable upgrade and migration revisions"); its
  success guarantee is a single compound contract, unchanged from the
  accepted permanent narrative. No two-outcome guarantee was introduced.
- Missing primary actor: `## Actors` names the Project maintainer as
  primary, and `## Trigger` names them ("The maintainer upgrades Givn").
- Placement failure: `apply-a-managed-upgrade` advances the upgrade goal;
  `describe-the-release` owns the release guide rule; the archive
  capabilities own the archive merge/gate rules. The fragment's removed
  extras scenario is a lifecycle-graph concern, owned by the fragment.
  No capability fails the placement test.
- Mutation under an observing goal: none of the touched capabilities has
  an observing goal; the re-pointed scenarios assert the same observable
  outcomes as their permanent titles.
- Extension without a step reference: the delta narrative's extensions all
  reference main-flow steps (1, 2, 3, 5, 6) and add no new success
  outcome; main flow 6 / extension 6 ("the system names any command a
  release removed") is preserved and newly pinned by the retired-command
  scenario.
- Inconsistent preconditions: the narrative's preconditions are
  pre-flow state; the trigger is an event, not a state or desire; no open
  `(open: Q<N>)` prerequisite exists.
- Mechanism vocabulary: goal, trigger, and guarantees use domain language
  (upgrade revision, plan revision, migration change, guidance direction)
  with no mechanism named.
- Component- or channel-shaped name: none of the touched names is
  component- or channel-shaped.
- Oversized leaf: the main flow's six steps each carry one actor goal.
- Relationship misuse: `Extends: none`, `Includes: fragment: support`
  (the fragment's id is `support`; the retired use case's dangling
  `Includes: fragment: rules-policy` baseline finding leaves the corpus
  with the retirement — see the baseline note).

The delta narrative (`specs/upgrade-a-project/usecase.md`) is a valid
replacement: its declared base SHA-256
(`52b61b2d64f47251cd0827f3a6df8d19156cae1d0c4f3e878bc1eec2ec9cbd40`)
equals the SHA-256 of the current permanent file (verified), and the diff
is exactly the two designed rows (the 0.2.0 plan row becomes the 0.7.0
title; the retired-history-rewrite interaction row is added). The four
`@givn.removed` titles resolve at the lines the bookkeeping task names.
The tombstone is two lines, zero scenarios, capability tag
`@history-rewrite`.

**Confirmed Personas.** `developer--primary-user` (runs the retired
command, crosses the migration gap), `agent--technical-role` (reads exit
codes and stderr), `givn-maintainer--owner` (owns the catalog and release
routes). All three resolve to files under `givn/personas/`. Actors in the
scenarios ("the consumer") stay interaction roles, distinct from Persona
biographies; no Persona is invented or silently promoted.

PERSONAS: developer--primary-user

## Visual review

The design's Visual Design Contract declares `Interface: terminal`, with
six expected transcripts (design-review Q1). All six exist, are committed
(one per `@e2e` section commit), are non-empty, and were produced by real
runs: this review re-executed the same CLI interactions through the e2e
gate (`./run-tests.sh --e2e`, 6/6 green), and each transcript records the
scenario title, the exact argv, the cwd disposition, the exit code, and
raw stdout/stderr with the on-disk tree assertions:

1. `a-clean-0-7-0-project-receives-a-reviewable-aggregate-migration-plan/transcript.txt`
   — the Git example's `givn upgrade` run: exit 0, managed upgrade commit
   plus aggregate migration commit, `givn/changes/migrate-0-7-0-to-0-9-0`
   with the two ordered phases, config version bumped to 0.9.0.
2. `a-consumer-invoking-the-retired-history-rewrite-command-is-told-it-does-not-exist/transcript.txt`
   — `givn rewrite-history`: exit 2, stderr "unrecognized subcommand
   'rewrite-history'".
3. `consumer-archives-a-delta-that-adds-e2e-scenarios-and-is-blocked-without-a-configured-runner/transcript.txt`
   — `givn archive --change real-e2e-merge`: exit 1, stderr names the
   unconfigured e2e runner and its configuration file
   (`givn/commands.yaml`, the corrected attribution — re-captured in the
   remediation), change directory survives.
4. `consumer-archives-a-new-capability-and-the-merged-spec-lands-in-its-declared-use-case/transcript.txt`
   — `givn archive --change grouped-new`: exit 0, merged file lands in
   `givn/specs/some-group/`, no flat directory, change moved to archive.
5. `archive-is-blocked-when-a-new-capability-has-no-declared-routing-decision/transcript.txt`
   — `givn archive --change flat-new`: exit 1, routing refusal, nothing
   merged.
6. `nothing-is-merged-when-one-delta-file-of-a-change-has-no-routing-decision/transcript.txt`
   — `givn archive --change mixed-routing`: exit 1, even the routed delta
   stays unmerged, change directory survives.

Per-Persona judgment against the committed transcripts: the developer
primary user can cross the gap (transcript 1 shows the reviewable
aggregate plan and the next step) and sees the retired command named
instead of a silent gap (transcript 2); the agent technical role gets the
exit codes and stderr it needs to branch on (transcripts 2, 3, 5, 6); the
givn maintainer sees the archive merge, the routing gate, and the
atomicity guarantee working as the catalog owner requires (transcripts
3-6). Every finding names a contract rule, and none remains
undispositioned.

VISUAL-REVIEW: PASS

## Overlap dispositions

`givn lint --change drop-legacy-residue` reports 27 SHAPE findings
(26 distinct pairs) involving the delta plus 436 SUBST advisories. Every
non-contradiction `shape_match` pair involving a delta file is disposed
below with a valid decision word; subset advisories are advisory and never
block. The modified deltas share their permanent titles, so the pairs
against their own merge targets are the replacement pairs the merge
applies atomically; the remaining pairs are cross-capability shape
coincidence on common assertion shapes (the re-pointed
`resolve-change-directories` scenario is the broadest, matching the
readme, visual-gate, review-disposition, and workflow-enforcement
families; the two archive capabilities match the gate-shaped scenarios) —
each operation changes only a Given step's file name, never the asserted
behaviour.

| Scenario A | Scenario B | Decision | Distinct invariant |
|---|---|---|---|
| A clean 0.2.0 project receives a reviewable aggregate migration plan | A clean 0.7.0 project receives a reviewable aggregate migration plan | variant | remove+add pair: the 0.7.0 outline supersedes the removed 0.2.0 outline (the title carries the fixture version; modified replaces by exact title, so the merge removes one and adds the other atomically); the retained route exists at 0.7.0 |
| A clean project without migration requirements receives one managed upgrade commit | A project below the oldest migration route receives no migration change | variant | same observable end state (managed-only commit, no migration change), distinct precondition: current version with a stale integration vs recorded version below the oldest declared route — the no-route case is the boundary this change exists to pin |
| Archive allowed when @wip only on a @givn.removed placeholder | Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured | boundary | different gates: wip-placeholder allowance vs e2e-runner absence; shared exit-0 shape only |
| Uppercase checkboxes satisfy the archive gate | Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured | boundary | checkbox-casing tolerance vs e2e gate; shared archive exit-0 shape only |
| A change that declares no visual interface is exempt | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | visual-contract exemption vs change-directory resolution; shared "gate passes" shape only |
| A change without a visual contract is exempt from the visual gate | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | visual-contract exemption vs change-directory resolution |
| A change without a visual interface passes review with a reasoned none | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | persona-disposition path vs change-directory resolution |
| A deferral with an operator decision passes review | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | deferral bookkeeping vs change-directory resolution |
| A disabled addon needs no impact decision | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | README-impact addon gate vs change-directory resolution |
| A modified delta matching its merge target in the same capability is allowed | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | overlap-gate rule vs change-directory resolution |
| A recorded impact decision passes the review | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | README-impact record vs change-directory resolution |
| A removed delta referencing its target by title is allowed | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | overlap-gate rule vs change-directory resolution |
| A terminal change passes review with a transcript | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | visual evidence path vs change-directory resolution |
| A visual change with a confirmed persona disposition passes review | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | persona-disposition path vs change-directory resolution |
| A visual change with complete evidence passes review | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | visual evidence path vs change-directory resolution |
| A web contract with complete state evidence passes review | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | visual state evidence vs change-directory resolution |
| givn addons disable when already disabled is idempotent | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | addon idempotence vs change-directory resolution |
| givn check with known artifact proceeds to run hooks | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | artifact-id validation vs change-directory resolution |
| Integrity hook passes on edited arc42 chapter | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | arc42 integrity vs change-directory resolution |
| Integrity hook passes when arc42 is disabled | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | arc42 addon gate vs change-directory resolution |
| Review passes when a long scenario has a split-or-keep entry | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | split-or-keep gate vs change-directory resolution |
| Review passes when a shape match exists only between permanent scenarios | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | overlap gate vs change-directory resolution |
| The check passes on a complete README | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | README completeness vs change-directory resolution |
| The default scope excludes the archive tree | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | scope exclusion vs change-directory resolution |
| Updated impact decision passes the review | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | README-impact record vs change-directory resolution |
| Valid kebab-case change id is accepted by every command that accepts --change | Verify-e2e hook resolves the change directory from a relative features path inside givn/changes | boundary | change-id validation vs change-directory resolution |

Removed+added pair record: the added "A clean 0.7.0 project receives a
reviewable aggregate migration plan" outline supersedes the removed "A
clean 0.2.0 project…" outline (same normalized action — plan a release
migration — re-pinned to the oldest retained route). The three other
removed scenarios ("A line measured by both reports…", "A line only one
report measures…", "Extras flag is propagated from overlay") have no
added successor: their branches were deleted, so no removed+added pairing
applies.

## README impact

The change edits `README.md` (commit `89029a6d`): the blocking-gates
paragraph in "What it is" loses the semantic-duplicate-review clause, the
"Runner-agnostic" bullet gains the `agents`/`claude`/`opencode`/`omp`
formats, the "Configurable without forking" bullet names the single
feature-free binary, the Install block's comments lose the
`--no-default-features` variant, the Documentation bullets lose the
semantic-search and extras wording, and the "Enforcement note for agents"
section is deleted. The named sections were actually edited, not merely
declared.

README-IMPACT: updated - What it is, Install, Documentation (the Enforcement note for agents section was deleted)

## Retirement disclosure

This change retires the `history-rewrite` capability: the `@givn.retired`
tombstone (`specs/rewrite-history/history-rewrite.feature`, two lines,
zero scenarios) removes the capability at the archive merge and appends
`history-rewrite / retired_by_change: drop-legacy-residue` to
`givn/specs/.retired.yaml`; the extended archive retirement then removes
the use-case directory and its narrative.

RETIRE: history-rewrite — the in-tool history rewrite left the product: rule cleanliness is report-only, a required purge is the project's own SCM work, and the retired use case leaves the corpus with its only capability (tombstone plus the extended archive retirement).

## Baseline note

The permanent corpus still carries the pre-merge state: the
`@givn.modified` Given texts and the tombstone merge at archive, and the
permanent `history-rewrite` feature still exists until then. Default-scope
`givn lint` exits 3 while the `rewrite-history` directory exists (the
dangling `Includes: fragment: rules-policy` reference and the
interaction-inventory `@e2e` mismatch), and returns to 0 exactly when the
archive retirement removes the directory — the design documents this
interval and it clears at archive. The same interval explains the stale
config-file wording still served by the pre-change installed binary; the
corrected assets refresh through the post-archive dogfood
(`givn skills sync`). These baseline states are expected and are not
findings.

## Result

The first review pass returned `REVIEW: FAIL` with four bucket-2 findings
(unpinned user-visible contract and coverage pins). The remediation pass
repaired all four with one atomic commit per repair, and this re-audit
verified each repair independently rather than trusting the remediation's
claims:

1. The three production error strings now attribute `verify.e2e_command`
   to `givn/commands.yaml` (`8cb752a4`); a full `src/` sweep confirms no
   remaining `verify.*` / `coverage.*` / `run:` misattribution, and the
   blocked-runner `@e2e` scenario pins the corrected string via a new
   Then step whose assertion demonstrably fails against the committed
   pre-repair transcript (`33cbe10f`); the targeted scenario re-run
   exits 0.
2. `givn spec --help` now describes the six real subcommands with no
   retrieval vocabulary (`982c0ed3`); verified by building the binary and
   reading its actual help output.
3. The D2 config-refusal contract is pinned by a unit test that genuinely
   fails if the strict `Coverage` schema is relaxed (`4ea92522`),
   verified by inspecting the `deny_unknown_fields` attribute and the
   assertion path.
4. The D1 extras-tolerance contract is pinned by a unit test asserting a
   byte-identical effective manifest for a stray `extras:` key
   (`4ea92522`).

Regression battery after the repairs, all re-run by this review:
`cargo fmt --check` (exit 0), `cargo clippy --workspace --all-targets --
-D warnings` (exit 0), the regular scoped gate (16/16, exit 0), the e2e
scoped gate (6/6, exit 0). `givn check tasks` and `givn lint --change
drop-legacy-residue` pass; the `@e2e` tag set is unchanged from the
first-pass candidate. The remediation commits touch exactly the files
they claim, and tasks.md's Review remediation section records real
revision identifiers with pasted verification outputs.

No finding remains open, no finding was excused with a classification
outside the three buckets, and the fabrication audit (step bodies,
tag integrity, commit cross-check, strict-mode proof, downgraded-e2e
scan, e2e scope and isolation, design conformance, interaction coverage)
remains clean for the current tree.

REVIEW: PASS
