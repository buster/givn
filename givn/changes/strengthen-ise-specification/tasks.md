# Tasks: strengthen-ise-specification

Runner: `./run-tests.sh --non-e2e` (`verify.command`) and
`./run-tests.sh --e2e --package e2e` (`verify.e2e_command`), both already set in
`givn/commands.yaml`. Strict mode is `.fail_on_skipped()` on both cucumber
builders (`tests/features_runner.rs`, `e2e/tests/e2e_runner.rs`). No runner,
framework, or fixture infrastructure is added by this change; the setup task
proves strictness once and returns to a green baseline.

Narrow run for one scenario (regular scope, design.md "Test Runner"):

```sh
GIVN_FEATURES=givn/changes/strengthen-ise-specification \
  cargo nextest run --locked --test features_runner \
  'specs/prepare-an-ise-test/clean-room-preparation.feature::<scenario title>'
```

Narrow run for one `@e2e` scenario:

```sh
cargo build --locked
GIVN_FEATURES=givn/changes/strengthen-ise-specification \
  GIVN_BIN=target/debug/givn \
  cargo nextest run --manifest-path e2e/Cargo.toml --test e2e_runner \
  'specs/prepare-an-ise-test/clean-room-preparation.feature::<scenario title>'
```

The `@e2e` runner refuses a `GIVN_BIN` that is not an executable file, so the
build precedes the run. `GIVN_FEATURES` also selects `specs/declare-the-specification/`
and `specs/prepare-an-ise-test/trial-input-verification.feature` through the
same filter suffix.

Release note (every behaviour batch commit subject, from `proposal.md`):

```text
The clean-room commands become three named actions (rebuild-all, rebuild-component, and compare), and every one writes a manifest whose checksum rules a second implementation can reproduce byte for byte.
```

Batch commit rule: one atomic `givn commit` with explicit paths per batch. The
subject is the release note; the body is `Batch: <batch-name>`, plus the
scenario title for a single-scenario batch. Every scenario of the batch records
the same revision as `COMMIT: <hash> -- Batch: <batch-name>`. The commit
contains the batch's implementation or its regression test; a comment-only,
task-only, or `@wip`-only change is not evidence. `@wip` is removed from a
scenario when its batch starts, never in bulk.

Evidence fields. Per the served `tasks` instruction this file records no
test-first order and no RED/GREEN checkbox. Each batch map entry names the
planned RED proof, and each batch section carries one evidence slot where the
implementer pastes the observed run. A scenario whose requirement is already
implemented is recorded as existing behaviour protected and manufactures no
production diff.

RED procedure (applies to every batch). A failure caused by an unknown
subcommand, an unknown flag, an unbound step, a panic, a missing fixture, or a
non-compiling tree is not accepted as RED. Each batch binds the parser surface
it needs first (a compile-only slice), runs the scenario, and records the
assertion failure under test. Where a scenario protects existing behaviour, the
batch says so and the real RED is carried by a named sibling scenario in the
same batch.

Ordering. `@e2e` scenarios sit last inside the batch that completes their
action's real-interface slice, never as a separate batch after the dependent
scenarios: the whole-product `@e2e` proves the report and manifest that the
later checksum and target batches extend, and the comparison `@e2e` proves the
two subject lines that the checkout and refusal batches extend. Every regular
scenario of a batch precedes its `@e2e` sibling.

## Batch map

Batch 1 `command-surface` — 9 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation refuses while the ise addon is off`, `Preparation refuses while architecture principles are off`, `Preparation refuses a dirty checkout and names every dirty path`, `Preparation refuses a project with no version-control backend`, `Preparation refuses a project root nested inside a Git worktree`, `Preparation refuses a checkout carrying both Git and Fossil markers`, `Preparation refuses an omitted target`, `A component rebuild refuses an empty path list`, `The retired command names and flags exit 2`.
- RED proof: `The retired command names and flags exit 2` runs `givn ise replace --into <dir>` against the pre-batch binary; it exits 0 and writes a copy where the scenario expects exit 2 and no copy. `A component rebuild refuses an empty path list` runs `givn ise rebuild-component --into <dir>`; the pre-batch parser refuses `--full | --path` and never prints the phrase `component path`. The six admission refusals and the omitted-target scenario assert the refusal prefix `ise rebuild-all:` (D5) and fail on the pre-batch `ise replace:` prefix; their admission logic is existing behaviour protected.
- Implementation slice: `IseCommands::RebuildAll` and `IseCommands::RebuildComponent` in `src/main.rs`; `RebuildRequest` with `RebuildScope::{All, Paths}` and the `manifest: Option<PathBuf>` field in `src/ise/mod.rs`; `--full`, `--path`, and `--receipt` removed; `--manifest <file>` and `--dry-run` parsed and carried, with the manifest writer landing in batch `report-and-manifest` so every refusal fixture can plan both targets; `rebuild-component` requires at least one positional path and refuses an empty list with exit 2 naming the component path; refusal prefixes follow the verb in `src/cli/ise.rs`. `IseCommands::Compare` is added by batch `compare-report`, so `givn ise verify` stays unknown in this batch.
- Verification: the narrow command for each scenario title above; then `givn check tasks --change strengthen-ise-specification`.

Batch 2 `revision-declaration` — 9 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation refuses when the declaration is missing`, `Preparation refuses a malformed declaration`, `Preparation refuses an absent revision`, `Preparation refuses when a revision blob cannot be read`, `Preparation refuses a revision that does not record the declaration`, `Preparation refuses a malformed declaration at the selected revision`, `Preparation refuses a declaration that is not UTF-8`, `Preparation refuses a declaration the revision records but cannot read`, `Preparation refuses a Specification path missing at the revision`.
- RED proof: `Preparation refuses a Specification path missing at the revision` is the batch's real assertion failure: the pre-batch refusal is produced by `require_declared_retained_entries` and names the entry by its role only, so the step's assertion on the phrase `declared Specification entry` fails. `Preparation refuses a declaration the revision records but cannot read` fails on the same run only if the step asserts the declaration read path rather than the prefix; the wording `returned no blob` already exists in `Backend::revision_blobs`, so this scenario is recorded as existing behaviour protected and asserts the exact path, revision, and phrase. The three declaration-content refusals are protected; each asserts the distinct named output D10 fixes.
- Implementation slice: `revision_declaration` distinguishes not-recorded, no-blob, non-UTF-8, and malformed; `require_declared_retained_entries` becomes the Specification-entry check and names the entry and the revision; `--at` keeps the explicit-revision and `HEAD` paths. The declaration stays the revision's blob, never the working tree.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 3 `component-paths` — 5 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation refuses a path outside every source root`, `Preparation refuses a path matched by a Specification role`, `Preparation refuses a directory path that contains a Specification path`, `Preparation refuses a directory that contains the declaration`, `Preparation refuses a component path that names the declaration`.
- RED proof: `Preparation refuses a path outside every source root` asserts stderr names both the path and `rebuild-all`; the pre-batch sentence names only the path, so the assertion fails. The three Specification-role refusals assert the role name (`behavior`) and the declaration path; the pre-batch wording says `retained`, so they fail on that token. The directory-expansion refusal is protected and asserts the expanded path, not the named directory.
- Implementation slice: the declaration check runs over the directory expansion; the out-of-root refusal names `rebuild-all`; role-match refusals name the role from `Role::as_str`.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 4 `copy-set` — 4 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation refuses a repository link in the copy set`, `Preparation refuses a nested repository marker in the copy set`, `Preparation refuses a symlink whose target resolves outside the spec copy`, `Preparation refuses an absolute symlink target`.
- RED proof: `Preparation refuses a symlink whose target resolves outside the spec copy` and `Preparation refuses an absolute symlink target` are the real failures: the pre-batch writer copies the symlink and exits 0, while the scenario expects exit 1, names the link and the target, and no copy. The repository-link and nested-marker scenarios are protected and assert the file name plus the new prefix.
- Implementation slice: `require_copy_set` keeps the gitlink and nested-marker refusals; the writer refuses an absolute symlink target and a relative target that resolves outside the copy before the first write (D14); a relative target inside the copy is written as recorded.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 5 `partition-and-copy` — 15 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation expands a directory path to the tracked files beneath it`, `Preparation does not expand a glob-shaped path`, `A named path absent from the revision removes nothing`, `Preparation counts the files left in a removed path's directory`, `A whole-product rebuild removes every tracked file under the source roots`, `A whole-product rebuild keeps a Specification file under a source root`, `A role that matches the declaration counts it once`, `Preparation partitions with the declaration at the selected revision`, `Preparation writes no version-control metadata into the spec copy`, `Preparation leaves an ignored file out of the spec copy`, `Preparation preserves the executable bit and the symlink target`, `Preparation reads an explicit revision and reports it`, `Preparation reads a Fossil revision and preserves an executable file and a symlink`, `The spec copy carries the declaration when no role names it`, `A whole-product rebuild keeps the declaration under a source root`.
- RED proof: `Preparation counts the files left in a removed path's directory` and `A whole-product rebuild keeps a Specification file under a source root` fail on the report nouns: the pre-batch renderer prints `Not replaced:` and `Retained:`, so `Files beside removed paths: 2` and `Specification files: 3` are absent. `A role that matches the declaration counts it once` fails on `Specification files: 2` for the same reason. The copy-content scenarios (executable bit, symlink, ignored file, version-control metadata, Fossil kinds, explicit revision) are existing behaviour protected and assert the copy tree and `Backend:`/`Revision:` lines.
- Implementation slice: `Partition` scope `All | Paths`, the three-layer placement (Specification file wins over a source root; an E2E match under a source root is neither copied nor removed; the declaration is an implicit Specification entry counted once), the five count definitions, directory expansion, repeated-path collapse, no glob expansion, absent path removes nothing, and the report count labels `Specification files`, `Source code kept`, `Source code removed`, `Outside the spec copy`, `Files beside removed paths`, plus `Specification checksum`/`End-to-end checksum` on the existing rollups.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 6 `report-and-manifest` — 12 scenarios (11 regular + 1 `@e2e`), `clean-room-preparation`.
- Scenarios: `Preparation writes no manifest when no manifest path is given`, `A dry run writes no spec copy and no manifest`, `A dry run without a manifest prints no manifest line`, `Preparation collapses a repeated path`, `The manifest lists the declaration among the Specification files`, `The manifest lists the End-to-end tests it does not copy`, `A declaration with no End-to-end role copies every Specification file`, `An End-to-end role that covers the declaration keeps it among the Specification files`, `A project-root End-to-end entry covers the whole tracked tree`, `An End-to-end entry that overlaps a source root wins over the removal`, `Developer rebuilds one component and leaves its neighbours`, `Developer rebuilds the whole product from its Specification`.
- RED proof: with `--manifest` parsed (batch `command-surface`) and no writer, `Preparation writes no manifest when no manifest path is given` fails because the pre-batch renderer has no `Manifest:`/`Copy directory:` lines and still emits the trial record; `Preparation collapses a repeated path` fails on the absent manifest and its `removed_paths` entry; `A project-root End-to-end entry covers the whole tracked tree` fails because the pre-batch E2E resolution treats `.` as a package path without a manifest and reports 0 E2E files while removing `src/kept.rs`; `An End-to-end entry that overlaps a source root wins over the removal` fails because the pre-batch copy removes the E2E match under `tests/**`. The `@e2e` scenario fails on `Specification files: 2`/`Source code removed: 3` and the absent `Manifest:` line, then asserts the real binary's stdout and written manifest.
- Implementation slice: `src/ise/record.rs` becomes `src/ise/manifest.rs` with the `ise-manifest/v1` shape (scope, revision, `removed_paths`, the three sorted path lists, five counts under `counts`, three checksum maps, two rollups, `copy_directory`, `givn_version`); `--manifest` writes it with create-new semantics and never prints it; `Manifest:` and `Copy directory:` lines plus the dry-run labels `Copy directory (would write):` and `Manifest (not written):`; the `e2e` entry `.` covers the whole tracked tree; an E2E match under a source root is neither copied nor removed.
- Verification: the narrow command for each title, the `@e2e` command form above; then `givn check tasks --change strengthen-ise-specification`.

Batch 7 `checksums` — 3 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation records the canonical checksums for a pinned fixture`, `Preparation records the canonical checksums for a richer feature`, `Preparation checksums a repository link in the End-to-end set`.
- RED proof: `Preparation records the canonical checksums for a pinned fixture` asserts the full 64-hex `specification_checksum`, `end_to_end_checksum`, the per-path map rows, and the scenario checksum; with the manifest writer from batch 6 the values are absent or differ, so the first map assertion fails. `Preparation checksums a repository link in the End-to-end set` fails because the pre-batch copy set refuses a gitlink globally instead of hashing it as kind `g` in the E2E set.
- Implementation slice: `path_checksum` over `sha256(path, NUL, kind, NUL, executable, NUL, content)` with kinds `f`/`x`/`l`/`g`; the two rollups with the scenario checksum winning over the content checksum for the same path; `e2e_scenario_digest` reused for the per-feature scenario checksum; a gitlink in the E2E set hashed as `g`, a gitlink in the copy set still refused. The two fixture helpers (`canonical_checksum_project`, `rich_checksum_project`) write the normative bytes and commit them.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 8 `write-targets` — 9 scenarios, `clean-room-preparation`.
- Scenarios: `Preparation refuses a target inside the checkout`, `Preparation refuses an existing spec copy directory`, `Preparation refuses an existing spec copy file`, `Preparation refuses a spec copy target symlinked into the checkout`, `Preparation refuses a manifest inside the spec copy`, `Preparation refuses a manifest inside the checkout`, `Preparation refuses a manifest that is an ancestor of the spec copy`, `Preparation refuses an existing manifest`, `Preparation refuses when the manifest cannot be written`.
- RED proof: `Preparation refuses when the manifest cannot be written` fails: the pre-batch path has no manifest write, so the run succeeds and leaves the spec copy, while the scenario expects exit 1, the manifest path named, and the created copy removed. The other eight are protected write-target checks renamed from receipt to manifest and assert the new prefix; `Preparation refuses an existing manifest` is not satisfiable while the flag is still `--receipt`.
- Implementation slice: `require_write_targets` names the spec copy and the manifest, resolves every existing ancestor, and refuses inside-checkout, existing, inside-copy, and ancestor-of-copy; a failed manifest write removes the copy the command created.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 9 `compare-report` — 9 scenarios (8 regular + 1 `@e2e`), `trial-input-verification`.
- Scenarios: `Comparison names a changed Specification file in the copy`, `Comparison names a missing Specification file in the copy`, `Comparison reports no difference after the agent adds Source code`, `Comparison reports no difference after a kept Source code file is edited`, `Comparison refuses a file added under a declared Specification path`, `Comparison refuses a file added under an End-to-end role`, `Comparison refuses a deleted kept Source code file`, `Comparison prints one line for a path drifting in the copy and the checkout`, `Verifier compares the spec copy with the manifest`.
- RED proof: with `IseCommands::Compare` parsed and delegated to the existing `verify` body (compile-only slice), `Comparison names a changed Specification file in the copy` fails because the old report prints `Retained drift: <path>` and `Verification failed: 1 drifting path(s)` where the scenario expects `Specification files: 1 changed` and `- <path>`; `Comparison refuses a file added under an End-to-end role` fails because the old check does not refuse an added path under the copy's declaration. The `@e2e` scenario fails on the same lines and asserts the real binary.
- Implementation slice: `src/ise/verify.rs` becomes `src/ise/compare.rs`; `CompareRequest { manifest, copy }`; the manifest reader for the fields the comparison consumes; two subject lines with `match` or the fixed order `changed`, `missing`, `added`; one `- <path>` per changed or missing path and one `+ <path>` per added path; the copy-side added-file refusal across the five roles and the deleted kept-path refusal; `Specification changed after the copy was written`; exit 0 only when both subjects match.
- Verification: the narrow command for each title, the `@e2e` command form; then `givn check tasks --change strengthen-ise-specification`.

Batch 10 `compare-checkout` — 13 scenarios, `trial-input-verification`.
- Scenarios: `Comparison names an End-to-end test changed in the checkout`, `Comparison names an @e2e scenario changed in the checkout`, `Comparison names a content-only change to an @e2e feature in the checkout`, `Comparison names an added End-to-end file in the checkout`, `Comparison names an added @e2e scenario file in the checkout`, `Comparison names an added End-to-end root in the checkout declaration`, `Comparison names an added Specification file in the checkout`, `Comparison names a missing Specification file in the checkout`, `Comparison names a missing End-to-end test in the checkout`, `Comparison names a missing @e2e feature in the checkout`, `Comparison names every drifting path in one run`, `Comparison ignores an ignored file under the End-to-end package`, `Comparison reports a Specification changed after the copy was written`.
- RED proof: `Comparison names an added @e2e scenario file in the checkout` fails: batch 9's comparison reads only the manifest's recorded paths, so the new feature file contributes no `End-to-end tests: 1 added` line. `Comparison reports a Specification changed after the copy was written` fails because the batch 9 reader compares recorded paths only and does not rescan the checkout declaration, so the moved `structure` entry produces neither `Specification files: 1 changed` nor `+ config/kept.toml`. `Comparison ignores an ignored file under the End-to-end package` is the discrimination check: an added untracked-but-ignored file must not become `added`.
- Implementation slice: checkout content paths (tracked files plus untracked non-ignored files, sorted and deduplicated) through `src/scm.rs` and both backends; the checkout declaration derives the added-path scan; a path in the checkout's declared E2E set or a feature path carrying a selected `@e2e` scenario is owned by `End-to-end tests`, even when another role also matches; one line per path when the copy and the checkout both differ; every difference in one run.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 11 `compare-refusals` — 9 scenarios, `trial-input-verification`.
- Scenarios: `Comparison refuses while the ise addon is off`, `Comparison refuses while architecture principles are off`, `Comparison refuses a manifest whose scope is not ise-manifest/v1`, `Comparison refuses a manifest with an unknown field`, `Comparison refuses a manifest with a missing field`, `Comparison refuses a manifest with a malformed checksum`, `Comparison refuses an unreadable manifest`, `Comparison refuses a manifest whose copy directory is not the copy`, `Comparison refuses a manifest inside the copy`.
- RED proof: `Comparison refuses a manifest whose scope is not ise-manifest/v1` fails: the batch 9 reader accepts `ise-trial/v1` and reports counts, while the scenario expects exit 1, the manifest and `ise-manifest/v1` named, and no `Specification files:` line. `Comparison refuses a manifest with an unknown field` and `Comparison refuses a manifest with a missing field` fail on the same permissive reader. The two add-on refusals are protected and assert the `ise compare:` prefix and that no subject line prints.
- Implementation slice: the strict `ise-manifest/v1` reader (`deny_unknown_fields`, every listed field required, no optional field in this scope), the scope refusal naming the manifest and the expected value, 64-hex validation for every checksum and rollup, the copy-directory check, the manifest-inside-copy refusal, and the add-on refusal ordering before any read.
- Verification: the narrow command for each title; then `givn check tasks --change strengthen-ise-specification`.

Batch 12 `declaration-boundary` — 1 scenario (`@givn.modified`), `specification-declaration`.
- Scenarios: `An unreadable declaration is refused by name`.
- RED proof: existing behaviour protected. `givn lint` already refuses a declaration path that is a directory and exits nonzero; the delta pins the code to exactly 1. The scenario passes on first run after the step asserts the exact code; no production edit is manufactured.
- Implementation slice: none in production; the step asserts exit 1 and stderr naming `givn/specification.yaml`, which `src/cli/lint/specification.rs` already produces.
- Verification: the narrow command for the title; then `givn check tasks --change strengthen-ise-specification`.

Durable-doc edits have no scenario batch. They are committed with the plan in the
setup batch and verified in the closeout against the served instruction, the
reference, the glossary, the ideation topic, the arc42 chapters, the ADR-076
amendment and its registers, `README.md`, and
`givn/specs/obtain-effective-guidance/usecase.md`.

## Interaction coverage pre-flight

Cross-checked the two owning `## Interactions` inventories against design.md's
Interaction Coverage Matrix. `specs/prepare-an-ise-test/usecase.md` holds two
entries (write the spec copy; compare the spec copy and the checkout) and both
have a matrix row and an existing `@e2e` scenario in the delta:
`Developer rebuilds the whole product from its Specification` and
`Verifier compares the spec copy with the manifest`.
`specs/declare-the-specification/usecase.md` holds one unchanged entry whose
`@e2e` scenario is permanent and unchanged. No gap; no task is written for an
`@e2e` scenario that does not exist.

## Domain constraints

Copied from `specs/prepare-an-ise-test/usecase.md` (guarantees, main flow,
extensions, rules) and `specs/declare-the-specification/usecase.md`, and from
the proposal's five strengthen statements.

| Capability | Guarantee or rule | Persona |
|---|---|---|
| `clean-room-preparation` | The rebuild reads the selected revision (HEAD when none is named) and never the working tree, so an ignored or untracked file cannot enter the copy; the checkout is never written, including the manifest. | `developer--primary-user`, `agent--technical-role` |
| `clean-room-preparation` | `rebuild-all` takes no path; `rebuild-component` takes at least one path and refuses an empty list with exit 2; a glob is never expanded; an absent path removes nothing and is not a refusal. | `developer--primary-user` |
| `clean-room-preparation` | A Specification-role match wins over a source root, the declaration is an implicit Specification entry counted once, and an E2E match is neither copied nor removed. | `givn-maintainer--owner` |
| `clean-room-preparation` | Every refusal is checked before the first write, names its condition, exits 1, and leaves the checkout and the targets unchanged; a usage error exits 2. | `agent--technical-role` |
| `clean-room-preparation` | The copy is a plain directory outside the checkout with no version-control metadata, the executable bit and a relative in-copy symlink preserved, and no E2E test. | `developer--primary-user`, `givn-maintainer--owner` |
| `clean-room-preparation` | The manifest is `ise-manifest/v1`, strict, ephemeral, outside the copy and the checkout, and written only when `--manifest` is given; a kept path carries no checksum so a kept edit is allowed and a deleted kept path is refused. | `givn-maintainer--owner` |
| `clean-room-preparation` | One path checksum is `sha256(path, NUL, kind, NUL, executable, NUL, content)` with kinds `f`/`x`/`l`/`g`; a rollup hashes each sorted `path, NUL, checksum, LF`; the E2E rollup uses the scenario checksum when a path is in both maps. | `agent--technical-role` |
| `trial-input-verification` | The comparison prints two subject lines and names every difference once in one run; exit 0 requires both lines to say `match`. | `developer--primary-user`, `agent--technical-role` |
| `trial-input-verification` | The copy's added file under any declared Specification path and a deleted kept path are refused before any count line. | `givn-maintainer--owner` |
| `trial-input-verification` | The checkout's added paths are differences; E2E ownership decides the subject for a path in the declared E2E set or carrying a selected `@e2e` scenario. | `agent--technical-role` |
| `trial-input-verification` | An unreadable, foreign-scope, unknown-field, missing-field, malformed-checksum, wrong-copy, or inside-copy manifest is refused naming the manifest and the reason. | `givn-maintainer--owner` |
| `specification-declaration` | The declaration reader parses text; `givn lint` owns the working-tree read and refuses a present but unreadable declaration by name with exit 1; the rebuild commands read the revision's blob and refuse the four cases distinctly. | `agent--technical-role` |

Carried by every batch: no Rust module, struct, or field name appears in
user-facing output; exit 0 is success or clean, 1 is a refusal or difference, 2
is a clap usage error; the manifest and the copy are evidence, never the
acceptance authority.

## Setup

- [x] Land the plan on the change branch: commit the change artifacts and the
      durable-doc edits that state the settled surface. Paths:
      `givn/changes/strengthen-ise-specification/`, `README.md`,
      `assets/instructions/archive.md`, `assets/instructions/ise.md`,
      `docs/reference.md`, `docs/glossary.md`, `docs/domain-model.md` (unchanged,
      verify only), `docs/ideation/ise-rebuildability.md`,
      `docs/arc42/02-architecture-constraints.md`,
      `docs/arc42/03-context-and-scope.md`, `docs/arc42/04-solution-strategy.md`,
      `docs/arc42/05-building-block-view.md`, `docs/arc42/06-runtime-view.md`,
      `docs/arc42/08-crosscutting-concepts.md`,
      `docs/arc42/09-architecture-decisions.md`,
      `docs/arc42/10-quality-requirements.md`,
      `docs/arc42/11-risks-and-technical-debt.md`, `docs/arc42/README.md`,
      `docs/arc42/adr/ADR-076-clean-room-trial-record.md`,
      `docs/arc42/adr/ADR-078-frozen-package-outside-the-trial.md`,
      `docs/arc42/adr/README.md`,
      `givn/specs/obtain-effective-guidance/usecase.md`.
      ```sh
      givn commit givn/changes/strengthen-ise-specification README.md \
        assets/instructions/archive.md assets/instructions/ise.md \
        docs/arc42 docs/glossary.md docs/ideation/ise-rebuildability.md \
        docs/reference.md givn/specs/obtain-effective-guidance/usecase.md \
        -m 'docs(strengthen-ise-specification): plan the change with the settled clean-room surface'
      ```

- [x] Confirm the runner configuration is the design's, with no change needed:
      `verify.command` is `./run-tests.sh --non-e2e`, `verify.e2e_command` is
      `./run-tests.sh --e2e --package e2e`, and `.fail_on_skipped()` sits on
      both cucumber builders. Record the two scopes and the delta discovery
      counts.
      ```sh
      GIVN_FEATURES=givn/changes/strengthen-ise-specification \
        cargo nextest list --locked --test features_runner | grep -c '::'
      GIVN_FEATURES=givn/changes/strengthen-ise-specification \
        GIVN_CUCUMBER_TAGS='not @e2e and not @givn.removed' \
        cargo nextest list --locked --test features_runner | grep -c '::'
      ```
      Expected: 98 scenarios in the delta, 96 regular, 2 excluded as `@e2e`.
      Observed: 98 delta scenarios, 96 regular (2 `@e2e` excluded); scopes are
      `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e --package e2e` in
      `givn/commands.yaml`.

- [x] Proof-of-strictness, once. Write a scratch feature outside the delta
      scope with one unbound step, run it through the regular runner, and
      confirm a non-zero exit with the runner's unbound-step message. Paste the
      command and the output here. Then delete the scratch directory and
      re-run the baseline below; do not proceed while the probe exits 0.
      ```sh
      mkdir -p /tmp/ise-strictness-probe
      printf 'Feature: Strictness probe\n  Scenario: An unbound step fails\n    Given this step is deliberately unbound\n' \
        > /tmp/ise-strictness-probe/probe.feature
      GIVN_FEATURES=/tmp/ise-strictness-probe \
        cargo nextest run --locked --test features_runner
      ```
      ```text
      FAIL probe.feature::An unbound step fails
        Given this step is deliberately unbound
          Step doesn't match any function
      Summary: 1 test run: 0 passed, 1 failed, 0 skipped; exit 100.
      Probe deleted: /tmp/ise-strictness-probe removed.
      ```

- [x] Confirm the step-definition layout is the design's, one file per
      capability, all permanent and none created by this change:
      `tests/steps/clean_room_preparation_steps.rs`,
      `tests/steps/trial_input_verification_steps.rs`,
      `tests/steps/specification_declaration_steps.rs`,
      `tests/steps/ise_clean_room_support.rs` (fixtures only),
      `e2e/tests/steps/clean_room_preparation_e2e_steps.rs`,
      `e2e/tests/steps/trial_input_verification_e2e_steps.rs`.
      No phrase is bound ahead of its batch: an unbound phrase must fail the
      strict runner rather than be stubbed with `todo!()`, because a stub panic
      is not an accepted RED. Each batch binds its own phrases with real
      assertion bodies. Reused phrases stay as they are.
      ```text
      tests/steps/clean_room_preparation_steps.rs
      tests/steps/trial_input_verification_steps.rs
      tests/steps/specification_declaration_steps.rs
      tests/steps/ise_clean_room_support.rs
      e2e/tests/steps/clean_room_preparation_e2e_steps.rs
      e2e/tests/steps/trial_input_verification_e2e_steps.rs
      All six exist and are registered; no new step file was added.
      ```

- [x] Restore and record the passing baseline with the delta scenarios still
      tagged `@wip`.
      ```sh
      ./run-tests.sh --non-e2e
      ```
      ```text
      ./run-tests.sh --non-e2e exited 0; no failed or skipped scenario.
      ```

- [x] COMMIT: `76159193af` -- Batch: `setup`
      The plan commit above. The setup tasks change no production file; the
      baseline is green before batch 1 starts.

## Behaviour batches

Each scenario task names its fixture, its observation, and the production
symbol it exercises. The batch section carries the evidence slot; the batch map
carries the RED proof, the slice, and the verification command.

### Batch 1: `command-surface`

Files modified: `src/main.rs`, `src/cli/ise.rs`, `src/ise/mod.rs`,
`src/ise/partition.rs`, `tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [x] Evidence for this batch: paste the narrow run and observed failure for
      `A component rebuild refuses an empty path list` and `The retired command
      names and flags exit 2`, then the GREEN run of all nine scenarios.
      ```text
      RED (narrow run per title, parser slice in place):
      A component rebuild refuses an empty path list
        assertion `left == right` failed: status 0, expected 2; stderr empty
        (the empty component list wrote a spec copy).
      The retired command names and flags exit 2
        stderr "error: the following required arguments were not provided:
        <--full|--path <PATH>> ..." does not name an unrecognized subcommand.
      The six admission refusals
        stderr "ise replace: ise is not enabled; ..." does not start with the
        "ise rebuild-all: " prefix; the same prefix failure for
        architecture_principles, the dirty checkout, the missing backend, the
        nested project root, and the ambiguous version control.

      GREEN (narrow run per title): 9 passed, 0 failed.
      ```

- [x] Scenario: `Preparation refuses while the ise addon is off`
  - Design constraints: fixture `ise_project()` with `addons.ise: false` and a
    committed declaration; When `givn ise rebuild-all --into <dir> --manifest
    <file>` is refused; Then exit 1, stderr starts with `ise rebuild-all:` and
    names `ise` and its enable command, the declaration is not read, and both
    targets stay absent. Contract: `require_addons` before the declaration read.
    Existing behaviour protected: the admission check; the prefix is new.
  - Files: `src/ise/mod.rs`, `src/cli/ise.rs`,
    `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `Preparation refuses while architecture principles are off`
  - Design constraints: fixture with `ise` on, `architecture_principles` off,
    and a malformed declaration committed; Then exit 1, stderr names
    `architecture_principles`, and the malformed declaration is never parsed.
    Contract: the add-on pair is checked before `revision_declaration`.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `Preparation refuses a dirty checkout and names every dirty path`
  - Design constraints: one modified tracked file and one untracked file; Then
    exit 1, stderr names both paths, and both targets stay absent. Contract:
    `Backend::dirty_entries` before the revision read.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `Preparation refuses a project with no version-control backend`
  - Design constraints: a directory with no Git or Fossil marker; Then exit 1
    and stderr names the missing backend. Contract: `scm::select`.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `Preparation refuses a project root nested inside a Git worktree`
  - Design constraints: the project root is a subdirectory of the worktree top
    level; Then exit 1 and stderr names the nested root. Contract: `scm::select`.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `Preparation refuses a checkout carrying both Git and Fossil markers`
  - Design constraints: both markers at the root; Then exit 1 and stderr names
    the ambiguous version control. Contract: `scm::select`.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `Preparation refuses an omitted target`
  - Design constraints: the When runs `givn ise rebuild-all` with no `--into`;
    Then exit 2 and stderr names `--into`, with no copy and no manifest.
    Contract: the clap requirement on `RebuildAll`. Existing behaviour
    protected (the `--into` requirement); the command name is the only change.
  - Files: `src/main.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `A component rebuild refuses an empty path list`
  - Design constraints: the When runs `givn ise rebuild-component --into <dir>`;
    Then exit 2 and stderr names the missing component path list, and no copy
    or manifest exists. Contract: the positional `paths` requirement on
    `RebuildComponent`; there is no `--full` and no `--path`.
  - Files: `src/main.rs`, `src/cli/ise.rs`,
    `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

- [x] Scenario: `The retired command names and flags exit 2`
  - Design constraints: three Whens in one scenario: `givn ise replace --into
    <dir>`, `givn ise verify --manifest <file>`, and `givn ise rebuild-all
    --full --into <dir>`; each exits 2, the first two name the unknown command,
    the third names the unknown flag, and no copy exists after the first and
    third. Contract: the removed variants do not exist in `IseCommands`.
  - Files: `src/main.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `5bcf0c9d0cb1fddeb1f754e75e7a3a0153563f97` -- Batch: `command-surface`

### Batch 2: `revision-declaration`

Files modified: `src/ise/mod.rs`, `src/ise/partition.rs`, `src/scm.rs`,
`src/git.rs`, `src/fossil.rs`,
`givn/changes/strengthen-ise-specification/specs/prepare-an-ise-test/clean-room-preparation.feature`,
`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [x] Evidence for this batch: paste the RED run for `Preparation refuses a
      Specification path missing at the revision` and the GREEN run of all
      nine scenarios.
      ```text
      RED (GIVN_FEATURES=givn/changes/strengthen-ise-specification/specs,
      before the production edit):
      cargo test --test features_runner -- \
        "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a Specification path missing at the revision" \
        --exact --nocapture
        Step panicked. Captured output: stderr "ise rebuild-all: the declared
        contracts entry 'docs/contract.md' matches nothing at revision
        '0461b6a92c1de450e4e76c0bc5b4a469757ad22e'" does not carry "declared
        Specification entry".

      The other eight passed on that run: the tree check, the blob refusal, the
      UTF-8 decode, and the loader are existing behaviour protected, and each
      asserts its own path, revision, and phrase.

      GREEN (narrow run per title, after the production edit):
        9 scenarios (9 passed), 0 failed.
      ```

- [x] Scenario: `Preparation refuses when the declaration is missing`
  - Design constraints: no declaration committed; Then exit 1, stderr names
    `givn/specification.yaml`, and both targets stay absent. Contract: the
    revision tree lacks the path, so `revision_declaration` names it.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses a malformed declaration`
  - Design constraints: one unknown top-level key in the committed declaration;
    Then exit 1, stderr names `givn/specification.yaml` and the loader's reason
    (`unknown key 'component'`), and both targets stay absent. Contract:
    `SpecificationDeclaration::from_content` on the revision blob.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses an absent revision`
  - Design constraints: `--at 0000000000000000000000000000000000000000`; Then
    exit 1, stderr names the revision, and both targets stay absent. Contract:
    `Backend::commit_exists` before the tree read.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses when a revision blob cannot be read`
  - Design constraints: a tracked blob is removed from the object store while
    the tree still lists it; Then exit 1, stderr names the path and the
    revision, and both targets stay absent. Contract:
    `Backend::revision_blobs` refusal during the copy read.
  - Files: `src/scm.rs`, `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses a revision that does not record the declaration`
  - Design constraints: the first revision predates the declaration; Then exit 1,
    stderr names `givn/specification.yaml`, the revision, and `does not record`;
    the blob-absent phrase must not satisfy it. Contract: the tree check in
    `revision_declaration`, before any blob read.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses a malformed declaration at the selected revision`
  - Design constraints: a malformed declaration in the first revision and a
    valid one at HEAD; Then exit 1, stderr names the path and the loader's
    reason for the selected revision, never HEAD's. Contract: the blob is read
    at the selected revision.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses a declaration that is not UTF-8`
  - Design constraints: a non-UTF-8 declaration in the first revision, valid at
    HEAD; Then exit 1, stderr names the path and the UTF-8 decoding error.
    Contract: `std::str::from_utf8` on the revision blob.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses a declaration the revision records but cannot read`
  - Design constraints: the loose object for `HEAD:givn/specification.yaml` is
    removed and the tree still lists it; Then exit 1, stderr names the path, the
    revision, and `returned no blob`, and the not-recorded phrase must not
    satisfy it. Contract: `Backend::revision_blobs` refusal surfaced by
    `revision_declaration`. Existing behaviour protected: the wording exists;
    the assertion is that the declaration read produces it rather than a panic.
  - Files: `src/scm.rs`, `src/ise/mod.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

- [x] Scenario: `Preparation refuses a Specification path missing at the revision`
  - Design constraints: the `contracts` role names `docs/contract.md` and the
    revision does not hold it; Then exit 1 and stderr names the path and the
    revision. Contract: the declared-entry check renamed from the role noun to
    `Specification entry`, naming the entry and the revision.
  - Files: `src/ise/partition.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `1b69e56fde6269a7c4ab4f4695fb54f0346eaa28` -- Batch: `revision-declaration`

### Batch 3: `component-paths`

Files modified: `src/ise/partition.rs`, `src/cli/ise.rs`,
`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [x] Evidence for this batch: paste the RED run for `Preparation refuses a path
      outside every source root` and the GREEN run of all five scenarios.
      ```text
      RED (before the production edit, GIVN_FEATURES=givn/changes/
      strengthen-ise-specification/specs):
      cargo test --test features_runner -- "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a path outside every source root" --exact --nocapture
        Step panicked. Captured output: stderr "ise rebuild-component:
        'docs/reference.md' is outside every declared source root at revision
        'c484f1f4'" does not carry "rebuild-all".
      The two role scenarios failed on the assertion "keeps the retired
      retained vocabulary": stderr "... 'givn/specs/demo.feature' is retained by
      the behavior role at revision '<sha>'; a retained path is never replaced".
      The two declaration scenarios failed on "does not carry \"Specification
      file\"": stderr "... 'givn/specification.yaml' is retained by the
      Specification declaration at revision '<sha>'; a retained path is never
      replaced".

      GREEN (narrow run per title, after the production edit):
        5 scenarios (5 passed), 0 failed; 4, 4, 5, 5, and 5 steps green.
      cargo fmt --all -- --check: exit 0.
      cargo clippy --workspace --all-targets --locked -- -D warnings: exit 0.
      ```

- [x] Scenario: `Preparation refuses a path outside every source root`
  - Design constraints: one source root `src/**`, component path
    `docs/reference.md`; Then exit 1 and stderr names both the path and
    `rebuild-all`. Contract: the refusal names the command that writes the copy
    removing every declared source root.
  - Files: `src/ise/partition.rs`, `src/cli/ise.rs`.
  - [x] COMMIT: `bf5cf1d29c8756020bcfd3fa3b892ed91e71e245` -- Batch: `component-paths`

- [x] Scenario: `Preparation refuses a path matched by a Specification role`
  - Design constraints: `behavior: ["givn/specs/**"]`, component path
    `givn/specs/demo.feature`; Then exit 1 and stderr names the path and the
    `behavior` role, and both targets stay absent. Contract: the declaration
    check runs over the named file.
  - Files: `src/ise/partition.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `bf5cf1d29c8756020bcfd3fa3b892ed91e71e245` -- Batch: `component-paths`

- [x] Scenario: `Preparation refuses a directory path that contains a Specification path`
  - Design constraints: `behavior: ["givn/specs/**"]` under source root
    `givn/**`, component path `givn/specs`; Then exit 1 and stderr names
    `givn/specs/demo.feature` and the `behavior` role. Contract: the
    Specification check runs over the tracked files the directory expands to.
  - Files: `src/ise/partition.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `bf5cf1d29c8756020bcfd3fa3b892ed91e71e245` -- Batch: `component-paths`

- [x] Scenario: `Preparation refuses a directory that contains the declaration`
  - Design constraints: source root `givn/**`, component path `givn`, tracked
    `givn/notes.md`; Then exit 1 and stderr names `givn/specification.yaml` as a
    Specification file. Contract: the declaration is an implicit Specification
    entry inside the expansion. Existing behaviour protected: the expansion
    check; the noun is new.
  - Files: `src/ise/partition.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `bf5cf1d29c8756020bcfd3fa3b892ed91e71e245` -- Batch: `component-paths`

- [x] Scenario: `Preparation refuses a component path that names the declaration`
  - Design constraints: source root `givn/**`, component path
    `givn/specification.yaml`, and an unrelated committed `src/kept.rs`; Then
    exit 1 and stderr names the declaration as a Specification file, and both
    targets stay absent. Contract: the declaration is tested before the source
    root.
  - Files: `src/ise/partition.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `bf5cf1d29c8756020bcfd3fa3b892ed91e71e245` -- Batch: `component-paths`

### Batch 4: `copy-set`

Files modified:
`givn/changes/strengthen-ise-specification/specs/prepare-an-ise-test/clean-room-preparation.feature`,
`src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [x] Evidence for this batch: paste the RED run for `Preparation refuses a
      symlink whose target resolves outside the spec copy` and the GREEN run of
      all four scenarios.
      ```text
      RED (GIVN_FEATURES=givn/changes/strengthen-ise-specification/specs,
      before the production edit):
      cargo test --test features_runner -- "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses a symlink whose target resolves outside the spec copy" --exact --nocapture
        Then the command fails and exits 1, naming "src/out" and "../../outside"
        Step panicked. Captured output: assertion `left == right` failed:
        stderr= left: Some(0) right: Some(1)
      cargo test --test features_runner -- "prepare-an-ise-test/clean-room-preparation.feature::Preparation refuses an absolute symlink target" --exact --nocapture
        Then the command fails and exits 1, naming "src/abs" and its absolute target
        Step panicked. Captured output: assertion `left == right` failed:
        stderr= left: Some(0) right: Some(1)
      Both failures show the pre-batch writer copying the symlink and exiting 0.
      The two protected copy-set scenarios passed on that run: each asserts
      exit 1, the refusal prefix, and the file name.

      GREEN (narrow run per title, after the production edit):
        4 scenarios (4 passed), 0 failed; 5 steps green per scenario.
      cargo fmt --all -- --check: exit 0.
      cargo clippy --workspace --all-targets --locked -- -D warnings: exit 0.
      ```

- [x] Scenario: `Preparation refuses a repository link in the copy set`
  - Design constraints: `structure: ["vendor/lib"]` and a gitlink at
    `vendor/lib`; Then exit 1, stderr names `vendor/lib`, and both targets stay
    absent. Contract: `require_copy_set` refuses a gitlink in the copy set.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `f0d326915f0c1f030cb78ce32e96488923401e86` -- Batch: `copy-set`

- [x] Scenario: `Preparation refuses a nested repository marker in the copy set`
  - Design constraints: `structure: ["nested/_FOSSIL_"]` and a committed
    `nested/_FOSSIL_`; Then exit 1 and stderr names the path. Contract:
    `require_copy_set` refuses a nested repository marker.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `f0d326915f0c1f030cb78ce32e96488923401e86` -- Batch: `copy-set`

- [x] Scenario: `Preparation refuses a symlink whose target resolves outside the spec copy`
  - Design constraints: committed symlink `src/out` declared through the
    `structure` role to `../../outside`, source root `src/**`; Then exit 1,
    stderr names `src/out` and `../../outside`, and both targets stay absent.
    Contract: the relative target is resolved against the link's parent inside
    the copy; leaving the copy refuses before any write.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `f0d326915f0c1f030cb78ce32e96488923401e86` -- Batch: `copy-set`

- [x] Scenario: `Preparation refuses an absolute symlink target`
  - Design constraints: committed `src/run.sh` and symlink `src/abs` declared
    through the `structure` role, whose absolute target points inside the
    planned copy; Then exit 1, stderr names `src/abs` and the absolute target
    text, and both targets stay absent. Contract: an absolute target always
    refuses, even when it resolves inside the copy.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `f0d326915f0c1f030cb78ce32e96488923401e86` -- Batch: `copy-set`

### Batch 5: `partition-and-copy`

Files modified: `src/ise/partition.rs`, `src/ise/mod.rs`, `src/cli/ise.rs`,
`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [x] Evidence for this batch: paste the RED run for `Preparation counts the
      files left in a removed path's directory` and `A whole-product rebuild
      keeps a Specification file under a source root`, then the GREEN run of all
      fifteen scenarios.
      ```text
      RED (GIVN_FEATURES=givn/changes/strengthen-ise-specification/specs,
      before the production edit):
      cargo test --locked --test features_runner -- "prepare-an-ise-test/clean-room-preparation.feature::Preparation counts the files left in a removed path's directory" --exact --nocapture
        Then the command succeeds and prints "Files beside removed paths: 2"
        Step panicked: stdout reports "Retained: 1 ... Not replaced: 2";
        "Files beside removed paths: 2" is absent.
      cargo test --locked --test features_runner -- "prepare-an-ise-test/clean-room-preparation.feature::A whole-product rebuild keeps a Specification file under a source root" --exact --nocapture
        Then the command succeeds and prints "Specification files: 3", ...
        Step panicked: stdout reports "Retained: 3"; "Specification files: 3"
        is absent.
      cargo test --locked --test features_runner -- "prepare-an-ise-test/clean-room-preparation.feature::A role that matches the declaration counts it once" --exact --nocapture
        Then the command succeeds and prints "Specification files: 2"
        Step panicked: stdout reports "Retained: 2"; "Specification files: 2"
        is absent.

      GREEN (narrow runs of the fifteen titles):
      cargo nextest run --locked --test features_runner -E 'test(/clean-room-preparation\.feature::(Preparation expands a directory path|Preparation does not expand a glob-shaped path|A named path absent from the revision|Preparation counts the files left|A whole-product rebuild removes every tracked file|A whole-product rebuild keeps a Specification file|A role that matches the declaration|Preparation partitions with the declaration|Preparation writes no version-control metadata|Preparation leaves an ignored file|Preparation preserves the executable bit|Preparation reads an explicit revision|Preparation reads a Fossil revision|The spec copy carries the declaration|A whole-product rebuild keeps the declaration)/)'
        15 tests run: 15 passed, 83 skipped.
      cargo fmt --all -- --check: exit 0.
      cargo clippy --workspace --all-targets --locked -- -D warnings: exit 0.
      ```

- [x] Scenario: `Preparation expands a directory path to the tracked files beneath it`
  - Design constraints: `src/keys/parser.rs`, `src/keys/mod.rs`, `src/other.rs`;
    component path `src/keys`; Then `Source code removed: 2` and `Files beside
    removed paths: 0`, the two key files absent, `src/other.rs` present.
    Contract: directory expansion over the revision's tracked files.
  - Files: `src/ise/partition.rs`, `src/cli/ise.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation does not expand a glob-shaped path`
  - Design constraints: component path `src/**` in a project with
    `src/one.rs` and `src/nested/two.rs`; Then `Source code removed: 0` and both
    tracked files stay in the copy. Contract: no glob expansion; the value
    resolves to no tracked file, so it removes nothing.
  - Files: `src/ise/partition.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `A named path absent from the revision removes nothing`
  - Design constraints: component path `src/keys/parsr.rs` (typo); Then
    `Source code removed: 0` and `src/keys/parser.rs` stays. Contract: an absent
    path is not a refusal.
  - Files: `src/ise/partition.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation counts the files left in a removed path's directory`
  - Design constraints: `src/keys/parser.rs` beside `src/keys/mod.rs` and
    `src/keys/tests.rs`; component path the parser file; Then `Files beside
    removed paths: 2` and both siblings stay in the copy. Contract: the
    containment count is the direct tracked siblings the command did not name
    and did not remove.
  - Files: `src/ise/partition.rs`, `src/cli/ise.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `A whole-product rebuild removes every tracked file under the source roots`
  - Design constraints: `src/one.rs` and `src/nested/two.rs`; Then
    `Specification files: 1`, `Source code kept: 0`, `Source code removed: 2`,
    the copy holds the declaration and omits both files. Contract: `All` scope
    removes every tracked file under every source root.
  - Files: `src/ise/partition.rs`, `src/cli/ise.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `A whole-product rebuild keeps a Specification file under a source root`
  - Design constraints: `structure: ["src/kept.rs", "src/nested/notes.md"]`, one
    source root `src/**`, tracked `src/other.rs`; Then `Specification files: 3`,
    `Source code kept: 0`, `Source code removed: 1`, `Files beside removed
    paths: 1`, both structure files held and `src/other.rs` omitted. Contract:
    a Specification-role match under a source root is a Specification file, not
    kept Source code.
  - Files: `src/ise/partition.rs`, `src/cli/ise.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `A role that matches the declaration counts it once`
  - Design constraints: `contracts: ["givn/**"]`, tracked `givn/specs/a.md`;
    Then `Specification files: 2` and the copy holds the declaration. Contract:
    the Specification set is a union keyed by path.
  - Files: `src/ise/partition.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation partitions with the declaration at the selected revision`
  - Design constraints: first revision holds `structure: ["docs/glossary.md"]`
    and both files, a later revision rewrites the declaration without the entry;
    Then the copy holds `docs/glossary.md` and its declaration is the first
    revision's bytes. Contract: the partition and the copy use one revision
    blob. Existing behaviour protected: the revision read.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation writes no version-control metadata into the spec copy`
  - Design constraints: one source root and a committed `src/kept.rs`; Then the
    command succeeds and the copy holds no `.git` and no `.fslckout` entry.
    Contract: the writer copies revision blobs, never metadata. Existing
    behaviour protected.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation leaves an ignored file out of the spec copy`
  - Design constraints: a committed `src/kept.rs` that the `structure` role keeps
    under `src/**`, plus an ignored working-tree file; Then the copy holds
    `src/kept.rs` and not the ignored file. Contract: the read is the revision,
    never the working tree. Existing behaviour protected.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation preserves the executable bit and the symlink target`
  - Design constraints: committed executable `src/run.sh` and symlink `src/link`
    to `run.sh`, both kept through the `structure` role; Then the copy's script
    is executable and the link is a symlink to `run.sh`. Contract: the writer
    sets the recorded mode and writes the relative target. Existing behaviour
    protected.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation reads an explicit revision and reports it`
  - Design constraints: first revision holds `src/kept.rs` (kept through
    `structure`), a later revision adds `src/later.rs`; `--at <first>`; Then
    `Backend: git` and `Revision:` carry the first revision and the copy omits
    `src/later.rs`. Contract: `--at` reads that revision's tree and declaration.
    Existing behaviour protected.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `Preparation reads a Fossil revision and preserves an executable file and a symlink`
  - Design constraints: Fossil checkout with `allow-symlinks`, one source root,
    and committed Specification, kept, removed, executable, and symlink entries;
    Then `Backend: fossil`, the copy holds the Specification and kept files,
    omits the removed one, and preserves the bit and target. Contract:
    `Backend::revision_files`/`revision_blobs` on Fossil. Existing behaviour
    protected.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `The spec copy carries the declaration when no role names it`
  - Design constraints: one source root and no role naming the declaration;
    Then the copy holds `givn/specification.yaml`. Contract: the declaration is
    an implicit Specification entry. Existing behaviour protected; the noun is
    new.
  - Files: `src/ise/partition.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

- [x] Scenario: `A whole-product rebuild keeps the declaration under a source root`
  - Design constraints: source root `givn/**` covering the declaration, tracked
    `givn/notes.md`; Then `Specification files: 1`, `Source code removed: 1`,
    the declaration held and `givn/notes.md` omitted. Contract: the declaration
    is never counted as removed.
  - Files: `src/ise/partition.rs`.
  - [x] COMMIT: `107dfb71513809194ae467e941984e370f5c973b` -- Batch: `partition-and-copy`

### Batch 6: `report-and-manifest`

Files modified: `src/ise/manifest.rs` (renamed from `src/ise/record.rs`),
`src/ise/mod.rs`, `src/ise/partition.rs`, `src/cli/ise.rs`, `src/main.rs`,
`src/e2e_boundary/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`,
`e2e/tests/steps/clean_room_preparation_e2e_steps.rs`.
Files created: `givn/changes/strengthen-ise-specification/evidence/visual/`
for the `@e2e` scenario slug below.

- [x] Evidence for this batch: paste the RED and GREEN runs for
      `A project-root End-to-end entry covers the whole tracked tree` (regular)
      and `Developer rebuilds the whole product from its Specification`
      (`@e2e`), then the GREEN run of the other ten scenarios.
      ```text
      RED against the pre-batch binary (b26f10ce7d):

      $ GIVN_FEATURES=givn/changes/strengthen-ise-specification cargo nextest run \
          --test features_runner \
          'specs/prepare-an-ise-test/clean-room-preparation.feature::Preparation writes no manifest when no manifest path is given'
      FAIL
        ✔ Given a Git project with ise enabled and a declaration
        ✔ When the developer rebuilds the whole product from "HEAD" into a spec copy without a manifest
        ✘ Then the command succeeds and prints no "Manifest:" line
          stdout "...\nWritten path: /tmp/.tmpCzSJjZ/trial\nschema: 1\ncontent_scope: ise-trial/v1\n..."
          the pre-batch build appends the trial record after the report

      $ ... '...::A project-root End-to-end entry covers the whole tracked tree'
      FAIL
        ✘ Then the command succeeds and prints "Specification files: 1", "Source code kept: 0",
          "Source code removed: 0", "Outside the spec copy: 0", and "End-to-end tests: not copied (2 files)"
          stderr=ise rebuild-all: the declared Specification entry '' in the e2e role matches
          nothing at revision '28cb28e51944db775c9c71ad962f61056c2cf640'
          the normalized "." entry is checked as a package path, so the rebuild refuses

      $ ... '...::An End-to-end entry that overlaps a source root wins over the removal'
      FAIL
        ✘ Then the command succeeds and prints "Specification files: 1", "Source code kept: 0",
          "Source code removed: 1", "Outside the spec copy: 0", and "End-to-end tests: not copied (1 file)"
          stdout prints "Frozen package: e2e, not copied (1 file)" and no "End-to-end tests:" line

      $ GIVN_FEATURES=$PWD/givn/changes/strengthen-ise-specification \
          GIVN_BIN=$PWD/target/debug/givn GIVN_CUCUMBER_TAGS='@e2e and @clean-room-preparation' \
          cargo test --manifest-path e2e/Cargo.toml --test e2e_runner
      FAIL
        ✘ And the report prints "End-to-end tests: not copied (2 files)"
          stdout prints "Frozen package: e2e, not copied (2 files)" and the trial record

      GREEN (48b71831bb08e9bd482eb871fc6828051f1944d0):

      $ cargo nextest run --test features_runner '<each regular batch title>'
      PASS Preparation writes no manifest when no manifest path is given
      PASS A dry run writes no spec copy and no manifest
      PASS A dry run without a manifest prints no manifest line
      PASS Preparation collapses a repeated path
      PASS The manifest lists the declaration among the Specification files
      PASS The manifest lists the End-to-end tests it does not copy
      PASS A declaration with no End-to-end role copies every Specification file
      PASS An End-to-end role that covers the declaration keeps it among the Specification files
      PASS A project-root End-to-end entry covers the whole tracked tree
      PASS An End-to-end entry that overlaps a source root wins over the removal
      PASS Developer rebuilds one component and leaves its neighbours

      $ GIVN_FEATURES=$PWD/givn/changes/strengthen-ise-specification \
          GIVN_BIN=$PWD/target/debug/givn GIVN_CUCUMBER_TAGS='@e2e and @clean-room-preparation' \
          cargo test --manifest-path e2e/Cargo.toml --test e2e_runner
      Developer rebuilds the whole product from its Specification
      1 scenario (1 passed), 11 steps (11 passed)

      Transcript:
      givn/changes/strengthen-ise-specification/evidence/visual/developer-rebuilds-the-whole-product-from-its-specification/transcript.txt

      Slice notes: `src/ise/record.rs` became `src/ise/manifest.rs`; the retired
      `ise-trial/v1` verifier body (`src/ise/verify.rs` and
      `E2eBoundary::contains`) was deleted with the record it read, and the
      comparison slice re-creates its reader. `Preparation collapses a repeated
      path` names no manifest in its When but asserts the manifest in its Then,
      so the step requests `--manifest`; the scenario text is unchanged.
      ```

- [x] Scenario: `Preparation writes no manifest when no manifest path is given`
  - Design constraints: `givn ise rebuild-all --into <dir>` with no
    `--manifest`; Then the command succeeds, prints no `Manifest:` line, writes
    no manifest file, and the copy holds the declaration. Contract: the report
    is the only output; no record on stdout.
  - Files: `src/main.rs`, `src/cli/ise.rs`, `src/ise/mod.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `A dry run writes no spec copy and no manifest`
  - Design constraints: `--dry-run --manifest <file>`; Then the report prints
    `Copy directory (would write):` and `Manifest (not written):`, and both
    paths stay absent. Contract: every read, refusal, count, and checksum runs;
    nothing is written.
  - Files: `src/ise/mod.rs`, `src/cli/ise.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `A dry run without a manifest prints no manifest line`
  - Design constraints: `--dry-run` without `--manifest`; Then the report prints
    `Copy directory (would write):` and no `Manifest (not written):` line, and
    the copy path stays absent. Contract: the conditional dry-run label.
  - Files: `src/cli/ise.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `Preparation collapses a repeated path`
  - Design constraints: `src/one.rs` and `src/two.rs`, with `src/one.rs` named
    twice; Then `Source code removed: 1` and the manifest's `removed_paths` holds
    one entry. Contract: the named set is a `BTreeSet`, and the manifest records
    the collapsed set.
  - Files: `src/ise/partition.rs`, `src/ise/manifest.rs`,
    `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `The manifest lists the declaration among the Specification files`
  - Design constraints: one source root, `--manifest` given; Then
    `specification_files` holds `givn/specification.yaml`. Contract: the list
    equals the Specification set, declaration included.
  - Files: `src/ise/manifest.rs`, `src/ise/partition.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `The manifest lists the End-to-end tests it does not copy`
  - Design constraints: `e2e: ["tests/e2e"]` and two committed files under it;
    Then `End-to-end tests: not copied (2 files)`, `end_to_end_files` holds both
    sorted paths, and the copy omits both. Contract: the E2E set is recorded and
    never copied.
  - Files: `src/ise/manifest.rs`, `src/ise/mod.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `A declaration with no End-to-end role copies every Specification file`
  - Design constraints: no `e2e` entries; Then `End-to-end tests: not copied (0
    files)` and `end_to_end_files` is empty. Contract: an empty E2E set is not
    an error.
  - Files: `src/ise/mod.rs`, `src/ise/manifest.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `An End-to-end role that covers the declaration keeps it among the Specification files`
  - Design constraints: `e2e: ["givn/**"]`, tracked `givn/specs/a.md`; Then
    `Specification files: 1` and `End-to-end tests: not copied (1 file)`, the
    declaration held and `givn/specs/a.md` omitted. Contract: the declaration is
    never frozen.
  - Files: `src/ise/partition.rs`, `src/ise/mod.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `A project-root End-to-end entry covers the whole tracked tree`
  - Design constraints: `e2e: ["."]` and `source: ["src/**"]`, tracked
    `src/kept.rs` and `notes.md`; Then `Specification files: 1`, `Source code
    kept: 0`, `Source code removed: 0`, `Outside the spec copy: 0`, `End-to-end
    tests: not copied (2 files)`, the copy holds only the declaration. Contract:
    an entry matching the project root resolves to the tracked tree. A reader
    that treats `.` as a package path without a manifest fails.
  - Files: `src/e2e_boundary/mod.rs`, `src/ise/partition.rs`, `src/ise/mod.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `An End-to-end entry that overlaps a source root wins over the removal`
  - Design constraints: `e2e: ["tests/e2e"]` and `source: ["tests/**"]`, tracked
    `tests/e2e/runner.rs` and `tests/unit/helper.rs`; Then `Specification files:
    1`, `Source code removed: 1`, `End-to-end tests: not copied (1 file)`, both
    files omitted. Contract: an E2E match under a source root is neither copied
    nor removed; the non-E2E sibling is removed.
  - Files: `src/ise/partition.rs`, `src/ise/mod.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `Developer rebuilds one component and leaves its neighbours`
  - Design constraints: `behavior: ["givn/specs/**"]`, `e2e: ["tests/e2e"]`,
    `source: ["src/**"]`, tracked `givn/specs/demo.feature`,
    `tests/e2e/Cargo.toml`, `src/keys/parser.rs`, `src/keys/mod.rs`,
    `src/other.rs`; component path the parser file with `--manifest`; Then
    `Backend: git`, `Specification files: 2`, `Source code kept: 2`, `Source code
    removed: 1`, `Outside the spec copy: 0`, `Files beside removed paths: 1`;
    the copy holds both neighbours and omits the parser and the E2E manifest;
    the manifest's removed paths are exactly `src/keys/parser.rs`. Contract:
    `rebuild-component` scope over the driver.
  - Files: `src/ise/partition.rs`, `src/ise/mod.rs`, `src/ise/manifest.rs`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

- [x] Scenario: `Developer rebuilds the whole product from its Specification`
  - Design constraints: the same declaration with a committed
    `givn/specs/demo.feature`, `tests/e2e/Cargo.toml`,
    `tests/e2e/steps/demo.rs`, three `src/**` files, and `notes.md`; the When
    runs `givn ise rebuild-all --into <copy> --manifest <file>` as a subprocess
    of the compiled binary; Then the eleven report lines hold `Backend: git`,
    `Specification files: 2`, `Source code kept: 0`, `Source code removed: 3`,
    `Outside the spec copy: 1`, `Files beside removed paths: 0`, a 40-hex
    `Revision:`, `End-to-end tests: not copied (2 files)`, 12-hex
    `Specification checksum:` and `End-to-end checksum:` lines, and the copy and
    manifest paths; the copy holds the feature and the declaration, omits every
    `src/**` file, `notes.md`, and both E2E files; the manifest records the
    scope, revision, removed paths, five counts, checksums, copy directory, and
    Givn version. Evidence: write
    `evidence/visual/developer-rebuilds-the-whole-product-from-its-specification/transcript.txt`
    from the real run.
  - Files: `src/cli/ise.rs`, `src/ise/mod.rs`, `src/ise/manifest.rs`,
    `src/gherkin/content.rs`, `tests/steps/clean_room_preparation_steps.rs`,
    `e2e/tests/steps/clean_room_preparation_e2e_steps.rs`,
    `givn/changes/strengthen-ise-specification/evidence/visual/developer-rebuilds-the-whole-product-from-its-specification/transcript.txt`.
  - [x] COMMIT: `48b71831bb08e9bd482eb871fc6828051f1944d0` -- Batch: `report-and-manifest`

### Batch 7: `checksums`

Files modified: `src/ise/manifest.rs`, `src/ise/mod.rs`,
`src/gherkin/content.rs` (the pinned richer checksum requires dropping the two
delimiter-adjacent docstring breaks, so the digest rule was corrected here),
`docs/reference.md`, `docs/arc42/08-crosscutting-concepts.md` (the stated
docstring rule corrected to match the pinned bytes),
`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [x] Evidence for this batch: paste the RED run for `Preparation records the
      canonical checksums for a pinned fixture` and the GREEN run of all three
      scenarios, with the full 64-hex values visible.
      ```text
      RED (before implementation):
      $ GIVN_FEATURES=givn/changes/strengthen-ise-specification cargo nextest run --locked --test features_runner \
          'specs/prepare-an-ise-test/clean-room-preparation.feature::Preparation records the canonical checksums for a pinned fixture'
      ✘ the manifest's scenario checksum for "givn/specs/demo.feature" is "791f7b34…88006c"
        left: None
       right: Some("791f7b34866c710ad541e06fb1d354d3ed4fedd7c46404465183d0a68d88006c")

      $ GIVN_FEATURES=givn/changes/strengthen-ise-specification cargo nextest run --locked --test features_runner \
          'specs/prepare-an-ise-test/clean-room-preparation.feature::Preparation checksums a repository link in the End-to-end set'
      ✘ the command succeeds and prints "End-to-end tests: not copied (2 files)"
        left: Some(1)
       right: Some(0)
        stderr=ise rebuild-all: 'tests/e2e/vendor/dep' is a repository link; a clean-room trial copies no repository link

      GREEN:
      $ GIVN_FEATURES=givn/changes/strengthen-ise-specification cargo nextest run --locked --test features_runner \
          'specs/prepare-an-ise-test/clean-room-preparation.feature::Preparation records the canonical checksums' \
          'specs/prepare-an-ise-test/clean-room-preparation.feature::Preparation checksums a repository link in the End-to-end set'
      PASS …::Preparation records the canonical checksums for a pinned fixture
      PASS …::Preparation checksums a repository link in the End-to-end set
      PASS …::Preparation records the canonical checksums for a richer feature
      Summary [1.017s] 3 tests run: 3 passed, 95 skipped

      Pinned values asserted: scenario 791f7b34866c710ad541e06fb1d354d3ed4fedd7c46404465183d0a68d88006c,
      specification 35444598f1c4238a9db8c2cdea5b5cf5391604a3608aebd6fe2e9f91f105b917,
      end-to-end 00580162d8a019161c476e2772c5a85e7506fc0c254cbc28424299343b479b07,
      richer scenario d5e995848ed1491e131b5f13504c3295e79f23646909ff5d4fb3b548d03e0520,
      richer specification c1102713bbf0a24482eaf6e3b8ff5b787ffa32d9ff414dc1c9434cba7a192e0e,
      richer end-to-end 2c16dfedf4203acf3a1d2f67340b4897d5e1c04c26a8d212fa95f1b8e0eb2844.

      Byte mismatch found before GREEN: the parser keeps the line break after the
      opening docstring delimiter and the one before the closing delimiter, so the
      pre-batch `e2e_scenario_digest` produced `c24173c6…70b71a` for the richer
      feature while the pinned value is `d5e99584…e0520` and the design's
      Independent Second Implementation records the docstring as
      `"line one\n  indented\nline four"`. `content.rs` now drops exactly those two
      breaks; the two stated rules name that normalization. No pinned value changed.
      ```

- [x] Scenario: `Preparation records the canonical checksums for a pinned fixture`
  - Design constraints: `canonical_checksum_project()` writes the normative
    bytes from the use-case example, excludes `givn/config.yaml` through
    `.git/info/exclude`, and commits; Then the manifest's scenario checksum for
    `givn/specs/demo.feature` is `791f7b34…88006c`, `specification_checksum` is
    `35444598…b917`, `end_to_end_checksum` is `00580162…9b07`, the three maps
    assert exact equality (an extra or missing row fails), and `end_to_end_files`
    holds the feature and the E2E manifest. Contract: `path_checksum` and the two
    rollups; no helper imports `crate::ise`.
  - Files: `src/ise/manifest.rs`, `tests/steps/ise_clean_room_support.rs`,
    `tests/steps/clean_room_preparation_steps.rs`.
  - [x] COMMIT: `1c6439a26121b42b375a4ffdbc8446a8300e49be` -- Batch: `checksums`

- [x] Scenario: `Preparation records the canonical checksums for a richer feature`
  - Design constraints: `rich_checksum_project()` replaces the feature with the
    background, docstring with delimiter-adjacent breaks, escaped table, and
    outline; Then the scenario checksum is `d5e99584…e0520`,
    `specification_checksum` is `c1102713…2e0e`, and `end_to_end_checksum` is
    `2c16dfed…2844`. Contract: the scenario checksum hashes title, NUL,
    executable content, NUL per selected scenario in file order.
  - Files: `src/ise/manifest.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1c6439a26121b42b375a4ffdbc8446a8300e49be` -- Batch: `checksums`

- [x] Scenario: `Preparation checksums a repository link in the End-to-end set`
  - Design constraints: `e2e: ["tests/e2e"]`, a committed
    `tests/e2e/Cargo.toml` and a gitlink at `tests/e2e/vendor/dep`; Then
    `End-to-end tests: not copied (2 files)`, the copy omits the gitlink, and
    `checksums.end_to_end` holds a 64-hex value for it. Contract: a gitlink in
    the E2E set hashes as kind `g` with the recorded commit id; a gitlink in the
    copy set still refuses.
  - Files: `src/ise/manifest.rs`, `src/ise/mod.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [x] COMMIT: `1c6439a26121b42b375a4ffdbc8446a8300e49be` -- Batch: `checksums`

### Batch 8: `write-targets`

Files modified: `src/ise/mod.rs`, `src/cli/ise.rs`, `src/main.rs`,
`tests/steps/clean_room_preparation_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [ ] Evidence for this batch: paste the RED run for `Preparation refuses when
      the manifest cannot be written` and the GREEN run of all nine scenarios.
      ```text
      <!-- paste: RED command + assertion failure; GREEN commands + pass summary -->
      ```

- [ ] Scenario: `Preparation refuses a target inside the checkout`
  - Design constraints: `--into` a directory inside the checkout; Then exit 1,
    stderr names the target, and the checkout holds no copy directory and no
    manifest. Contract: `require_write_targets`, existing behaviour protected
    with the new prefix.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses an existing spec copy directory`
  - Design constraints: the target directory exists; Then exit 1, stderr names
    it, and the directory is left unchanged. Contract: create-new semantics.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses an existing spec copy file`
  - Design constraints: the target path is a regular file; Then exit 1, stderr
    names it, and the file is left unchanged. Contract: create-new semantics.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses a spec copy target symlinked into the checkout`
  - Design constraints: a symlink outside the checkout points into it, and the
    target is named through that symlink; Then exit 1, stderr names the target,
    and the checkout holds no copy directory. Contract: every existing ancestor
    is canonicalized before the containment test.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses a manifest inside the spec copy`
  - Design constraints: `--manifest <copy>/manifest.yaml`; Then exit 1, stderr
    names the manifest, and both targets stay absent. Contract: the manifest
    must lie outside the copy.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses a manifest inside the checkout`
  - Design constraints: the manifest path is inside the checkout; Then exit 1,
    stderr names the manifest, and the checkout holds no manifest. Contract: the
    checkout is never written.
  - Files: `src/ise/mod.rs`, `tests/steps/clean_room_preparation_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses a manifest that is an ancestor of the spec copy`
  - Design constraints: the manifest path is an ancestor directory of the copy;
    Then exit 1, stderr names the manifest, and both targets stay absent.
    Contract: the copy cannot be created inside the manifest path.
  - Files: `src/ise/mod.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses an existing manifest`
  - Design constraints: the manifest file exists; Then exit 1, stderr names it,
    and the existing bytes are unchanged. Contract: the manifest is created new,
    never overwritten.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

- [ ] Scenario: `Preparation refuses when the manifest cannot be written`
  - Design constraints: the manifest's parent directory is a regular file; Then
    exit 1, stderr names the manifest path, and the copy directory the command
    created is removed. Contract: a failed manifest write rolls back the copy
    and reports no success.
  - Files: `src/ise/mod.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `write-targets`

### Batch 9: `compare-report`

Files modified: `src/ise/compare.rs` (renamed from `src/ise/verify.rs`),
`src/ise/mod.rs`, `src/cli/ise.rs`, `src/main.rs`,
`tests/steps/trial_input_verification_steps.rs`,
`tests/steps/ise_clean_room_support.rs`,
`e2e/tests/steps/trial_input_verification_e2e_steps.rs`.

- [ ] Evidence for this batch: paste the RED run for `Comparison names a changed
      Specification file in the copy` and for `Verifier compares the spec copy
      with the manifest` (`@e2e`), then the GREEN run of all nine scenarios and
      the transcript path.
      ```text
      <!-- paste: RED commands + assertion failures; GREEN commands + pass summary -->
      ```

- [ ] Scenario: `Comparison names a changed Specification file in the copy`
  - Design constraints: the prepared copy is built with `rebuild-component
    src/keys/parser.rs --manifest`, so `src/keys/steps.rs` is kept and the
    parser removed; the agent edits `givn/specs/demo/spec.feature` in the copy;
    Then `Specification files: 1 changed`, `- givn/specs/demo/spec.feature`,
    `End-to-end tests: match`, exit 1. Contract: the copy's content is compared
    with the manifest's per-path checksum.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`,
    `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison names a missing Specification file in the copy`
  - Design constraints: the agent deletes `givn/specs/demo/spec.feature` in the
    copy; Then `Specification files: 1 missing`, `- <path>`, `End-to-end tests:
    match`, exit 1. Contract: a recorded path absent from the copy is `missing`.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison reports no difference after the agent adds Source code`
  - Design constraints: the agent writes a file at a `removed_paths` entry;
    Then both subjects say `match` and the exit is 0. Contract: the rebuild's
    Source code is not a frozen input.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison reports no difference after a kept Source code file is edited`
  - Design constraints: the agent edits `src/keys/steps.rs` (kept); Then both
    subjects say `match` and the exit is 0. Contract: kept paths carry no
    checksum (R-162).
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison refuses a file added under a declared Specification path`
  - Design constraints: the agent adds a file under a declared role in the copy;
    Then exit 1, stderr names the added path as a Specification change, and no
    `Specification files: match` line prints. Contract: the copy's declaration
    derives the added-file check.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison refuses a file added under an End-to-end role`
  - Design constraints: the agent adds a file under the declared `e2e` role in
    the copy; Then exit 1, the added path is named as a Specification change, and
    no clean line prints. Contract: an `e2e` match is a declared Specification
    path for the refusal even though the copy never carries one.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison refuses a deleted kept Source code file`
  - Design constraints: the agent deletes the kept `src/keys/steps.rs`; Then
    exit 1, stderr names the path, and no `Specification files:` line prints.
    Contract: a recorded kept path that is gone is refused before any count.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Comparison prints one line for a path drifting in the copy and the checkout`
  - Design constraints: `docs/reference.md` is edited in the copy and in the
    checkout; Then `Specification files: 1 changed`, exactly one
    `- docs/reference.md` line under that subject, `Specification changed after
    the copy was written`, `End-to-end tests: match`, exit 1. Contract: one
    observation per path, owned by the subject that owns the path.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

- [ ] Scenario: `Verifier compares the spec copy with the manifest`
  - Design constraints: the prepared copy is unmodified; the When runs `givn ise
    compare --manifest <file> --copy <dir>` as a subprocess of the compiled
    binary; Then `Specification files: match` and `End-to-end tests: match`, exit
    0, and no line beginning `- `. Evidence: write
    `evidence/visual/verifier-compares-the-spec-copy-with-the-manifest/transcript.txt`
    from the real run.
  - Files: `src/cli/ise.rs`, `src/ise/compare.rs`,
    `tests/steps/trial_input_verification_steps.rs`,
    `e2e/tests/steps/trial_input_verification_e2e_steps.rs`,
    `givn/changes/strengthen-ise-specification/evidence/visual/verifier-compares-the-spec-copy-with-the-manifest/transcript.txt`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-report`

### Batch 10: `compare-checkout`

Files modified: `src/scm.rs`, `src/git.rs`, `src/fossil.rs`,
`src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [ ] Evidence for this batch: paste the RED run for `Comparison names an added
      @e2e scenario file in the checkout` and `Comparison reports a
      Specification changed after the copy was written`, then the GREEN run of
      all thirteen scenarios.
      ```text
      <!-- paste: RED commands + assertion failures; GREEN commands + pass summary -->
      ```

- [ ] Scenario: `Comparison names an End-to-end test changed in the checkout`
  - Design constraints: edit `tests/e2e/steps/keys.rs` in the checkout; Then
    `End-to-end tests: 1 changed`, `- tests/e2e/steps/keys.rs`, exit 1.
    Contract: the recorded content checksum differs on a checkout content path.
  - Files: `src/ise/compare.rs`, `src/scm.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names an @e2e scenario changed in the checkout`
  - Design constraints: edit a selected `@e2e` scenario in
    `givn/specs/demo/spec.feature` in the checkout; Then `Specification files:
    match`, `End-to-end tests: 1 changed`, `- <feature path>`, exit 1. Contract:
    the scenario checksum differs and the E2E subject owns the feature path.
  - Files: `src/ise/compare.rs`, `src/gherkin/content.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names a content-only change to an @e2e feature in the checkout`
  - Design constraints: rewrite only the `Feature:` description; the selected
    scenario text is unchanged; Then `Specification files: match`, `End-to-end
    tests: 1 changed`, `- <feature path>`, exit 1. Contract: the content checksum
    differs even when the scenario checksum matches.
  - Files: `src/ise/compare.rs`, `src/ise/manifest.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names an added End-to-end file in the checkout`
  - Design constraints: add `tests/e2e/steps/extra.rs` under the declared
    `tests/e2e` package; Then `Specification files: match`, `End-to-end tests: 1
    added`, `+ tests/e2e/steps/extra.rs`, exit 1. Contract: an added content path
    in the declared E2E set is owned by the E2E subject and contributes no
    Specification count.
  - Files: `src/ise/compare.rs`, `src/scm.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names an added @e2e scenario file in the checkout`
  - Design constraints: add `givn/specs/demo/extra.feature` carrying one `@e2e`
    scenario; Then `Specification files: match`, `End-to-end tests: 1 added`,
    `+ <path>`, exit 1. Contract: a feature path carrying a selected `@e2e`
    scenario is E2E-owned even when a role matches it.
  - Files: `src/ise/compare.rs`, `src/gherkin/content.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names an added End-to-end root in the checkout declaration`
  - Design constraints: add `e2e: ["tests/extra-e2e"]` and commit
    `tests/extra-e2e/Cargo.toml`; Then `Specification files: 1 changed`,
    `- givn/specification.yaml`, `+ tests/extra-e2e/Cargo.toml`, `End-to-end
    tests: 1 added`, exit 1. Contract: the checkout declaration derives the
    added-path scan; its own edit is a changed Specification file.
  - Files: `src/ise/compare.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names an added Specification file in the checkout`
  - Design constraints: add `docs/extra.md` under the `docs/**` contracts role;
    Then `Specification files: 1 added`, `+ docs/extra.md`, `End-to-end tests:
    match`, exit 1. Contract: a role match outside the E2E set is Specification
    -owned.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names a missing Specification file in the checkout`
  - Design constraints: delete `docs/reference.md` from the checkout; Then
    `Specification files: 1 missing`, `- docs/reference.md`, `End-to-end tests:
    match`, exit 1. Contract: a recorded path absent from the checkout is
    missing for its subject.
  - Files: `src/ise/compare.rs`, `src/scm.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names a missing End-to-end test in the checkout`
  - Design constraints: delete `tests/e2e/steps/keys.rs` from the checkout; Then
    `End-to-end tests: 1 missing`, `- <path>`, exit 1.
  - Files: `src/ise/compare.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names a missing @e2e feature in the checkout`
  - Design constraints: delete `givn/specs/demo/spec.feature` from the checkout;
    Then `Specification files: match`, `End-to-end tests: 1 missing`, `- <path>`,
    exit 1. Contract: the recorded `@e2e` feature path is E2E-owned.
  - Files: `src/ise/compare.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison names every drifting path in one run`
  - Design constraints: edit a copy Specification file, edit, delete, and add
    checkout paths, and edit an E2E test; Then `Specification files: 2 changed,
    1 missing, 1 added` and `End-to-end tests: 1 changed`, one `-` or `+` line
    per path, exit 1. Contract: the fixed count order and one run for every
    difference.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison ignores an ignored file under the End-to-end package`
  - Design constraints: add an ignored file under the checkout's `e2e` package;
    Then `End-to-end tests: match` and exit 0. Contract: content paths are
    tracked plus untracked non-ignored, never ignored files. This is the
    discrimination check for the enumeration.
  - Files: `src/scm.rs`, `src/git.rs`, `src/ise/compare.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

- [ ] Scenario: `Comparison reports a Specification changed after the copy was written`
  - Design constraints: rewrite the checkout declaration's `structure` role to
    `config/kept.toml`; Then `Specification files: 1 changed, 1 added`,
    `- givn/specification.yaml`, `Specification changed after the copy was
    written`, `+ config/kept.toml`, exit 1. Contract: the declaration edit is a
    changed Specification file and its new entry an added path.
  - Files: `src/ise/compare.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-checkout`

### Batch 11: `compare-refusals`

Files modified: `src/ise/compare.rs`, `src/ise/manifest.rs`,
`tests/steps/trial_input_verification_steps.rs`,
`tests/steps/ise_clean_room_support.rs`.

- [ ] Evidence for this batch: paste the RED run for `Comparison refuses a
      manifest whose scope is not ise-manifest/v1` and `Comparison refuses a
      manifest with an unknown field`, then the GREEN run of all nine scenarios.
      ```text
      <!-- paste: RED commands + assertion failures; GREEN commands + pass summary -->
      ```

- [ ] Scenario: `Comparison refuses while the ise addon is off`
  - Design constraints: the project disables `ise` after preparation; Then exit
    1, stderr names `ise`, and no `Specification files:` line prints. Contract:
    the add-on pair is checked before the manifest read.
  - Files: `src/ise/compare.rs`, `tests/steps/trial_input_verification_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses while architecture principles are off`
  - Design constraints: the project disables `architecture_principles` after
    preparation; Then exit 1, stderr names `architecture_principles`, and no
    subject line prints.
  - Files: `src/ise/compare.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses a manifest whose scope is not ise-manifest/v1`
  - Design constraints: rewrite the manifest scope to `ise-trial/v1`; Then exit
    1, stderr names the manifest and `ise-manifest/v1`, and no subject line
    prints. Contract: the scope is refused by name and not migrated.
  - Files: `src/ise/manifest.rs`, `src/ise/compare.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses a manifest with an unknown field`
  - Design constraints: add an unknown top-level field; Then exit 1, stderr
    names the manifest and the field. Contract: `deny_unknown_fields`.
  - Files: `src/ise/manifest.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses a manifest with a missing field`
  - Design constraints: remove the `counts` field; Then exit 1, stderr names the
    manifest and `counts`. Contract: every listed field is required in this
    scope.
  - Files: `src/ise/manifest.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses a manifest with a malformed checksum`
  - Design constraints: put a non-64-hex value in the Specification checksums
    map; Then exit 1, stderr names the manifest and the malformed checksum.
    Contract: every checksum and rollup is validated before any comparison.
  - Files: `src/ise/manifest.rs`, `src/ise/compare.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses an unreadable manifest`
  - Design constraints: write bytes that are not valid YAML; Then exit 1, stderr
    names the manifest and the parse error.
  - Files: `src/ise/manifest.rs`, `src/ise/compare.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses a manifest whose copy directory is not the copy`
  - Design constraints: rewrite `copy_directory` to another directory; Then exit
    1, stderr names the manifest and the copy. Contract: the manifest must belong
    to the given copy.
  - Files: `src/ise/compare.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

- [ ] Scenario: `Comparison refuses a manifest inside the copy`
  - Design constraints: the manifest is stored inside the copy; Then exit 1,
    stderr names the manifest, and no subject line prints. Contract: the
    manifest is evidence outside the copy.
  - Files: `src/ise/compare.rs`, `tests/steps/ise_clean_room_support.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `compare-refusals`

### Batch 12: `declaration-boundary`

Files modified: `tests/steps/specification_declaration_steps.rs`
(`src/cli/lint/specification.rs` only if the exact exit code is not already 1).

- [ ] Evidence for this batch: paste the GREEN run; the scenario is existing
      behaviour protected (the delta pins the code to exactly 1).
      ```text
      <!-- paste: command + pass summary, and the lint exit code observed -->
      ```

- [ ] Scenario: `An unreadable declaration is refused by name`
  - Design constraints: an `ise`-enabled project selecting `opencode` whose
    declaration path is a directory; the When runs `givn lint`; Then the exit
    code is exactly 1 and stderr names `givn/specification.yaml`. Contract:
    `SpecificationDeclaration::load` owns the working-tree read and returns the
    malformed case; `src/cli/lint/specification.rs` maps it to exit 1. Existing
    behaviour protected: the refusal exists; only the code assertion is new.
  - Files: `tests/steps/specification_declaration_steps.rs`.
  - [ ] COMMIT: `<hash>` -- Batch: `declaration-boundary`

## E2E smoke tests

Two `@e2e` scenarios carry the real-interface proof, both driving the compiled
binary as a subprocess and committing a terminal transcript under
`givn/changes/strengthen-ise-specification/evidence/visual/<scenario-slug>/transcript.txt`:

| Scenario | Batch | Batch home of the evidence slot |
|---|---|---|
| `Developer rebuilds the whole product from its Specification` | `report-and-manifest` | `report-and-manifest` |
| `Verifier compares the spec copy with the manifest` | `compare-report` | `compare-report` |

`./run-tests.sh --e2e --package e2e` builds the binary and exports `GIVN_BIN`;
a bare nextest run fails the runner preflight until `cargo build --locked` has
produced the executable.

## Closeout

- [ ] Merged step definitions. No delta step file exists: every phrase is bound
      in the permanent per-capability file from its batch, one file per
      capability, so the archive gate finds nothing to merge and nothing
      missing. Record the six files and their phrase counts.
      ```text
      <!-- record: phrase counts per file; no givn/changes/**/steps file exists -->
      ```

- [ ] Run the complete change suite and the two scopes.
      ```sh
      cargo fmt --all -- --check
      cargo clippy --workspace --all-targets -- -D warnings
      cargo nextest run
      ./run-tests.sh
      ./run-tests.sh --e2e --package e2e
      ```
      ```text
      <!-- paste: exit 0 for each command and the pass/skip summaries -->
      ```

- [ ] Change gate: every scenario is unchecked-free, every scenario task carries
      its batch revision, and no delta scenario stays `@wip`.
      ```sh
      givn check tasks --change strengthen-ise-specification
      givn status --change strengthen-ise-specification
      ```
      ```text
      <!-- paste: check tasks exit 0 and the status artifact checklist -->
      ```

- [ ] Durable-doc verification: the served instruction and the reference state
      the implemented surface; the glossary, ideation topic, arc42 chapters,
      ADR-076 amendment and registers, README, and the guidance use case name the
      three commands and the retired vocabulary is gone.
      ```sh
      givn instructions ise
      grep -n "rebuild-all\|rebuild-component\|givn ise compare" docs/reference.md
      grep -rn "trial record\|--receipt\|ise replace\|ise verify" docs assets README.md
      ```
      ```text
      <!-- paste: the served command table, the reference rows, and the empty
           retired-token grep outside the ADR historical note -->
      ```
