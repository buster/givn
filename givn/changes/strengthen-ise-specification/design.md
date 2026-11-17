# Design: strengthen-ise-specification

## Scope

The change renames the clean-room command surface, cuts the manifest to a new scope, states the canonical checksum bytes, and adds the `compare` report. It edits production code, the served instruction, the reference, the glossary, the ideation topic, the architecture constraints, and the delta scenarios.

| Area | Current | After |
|---|---|---|
| Commands | `givn ise replace` with `--full`/`--path`, and `givn ise verify` | `givn ise rebuild-all`, `givn ise rebuild-component <path>...`, and `givn ise compare` |
| Evidence | `ise-trial/v1` record written by `--receipt` or printed to stdout | `ise-manifest/v1` manifest written by `--manifest` only |
| Report | ten lines ending in `Written path`/`Receipt path` | eleven fixed lines beginning `Backend`/`Revision` plus `Manifest` (twelve with the manifest path), with dry-run labels |
| Comparison | per-path drift report against a trial | two subject lines with `changed`, `missing`, and `added` counts, plus one `- <path>` or `+ <path>` line per difference |

`src/ise/partition.rs` keeps the set arithmetic; the change renames its vocabulary and adds the two command scopes. `src/config/declaration.rs` and `src/e2e_boundary/**` keep their behaviour.

## Out of Scope

- The delta carries the declaration use case's swept narrative (`givn/specs/declare-the-specification/usecase.md`), so archive replaces the permanent narrative: the `--replace src/cli/archive.rs` example becomes a component rebuild naming the same path, and the `retained`/`trial` nouns leave that file. That narrative is the change's second use-case narrative and, after the merge, an edited `behavior`-role path beside the four `## Retained-Content Effect` names (a second `behavior`-role path); the permanent capability's scenarios needed only one modified title because they carry no retired token. The earlier deferral premise, that those scenarios assert the use case's surface, was false. The remaining `--replace src/cli/archive.rs` text lives in the ideation planning document `givn/ideation/ise-rebuildability/use-cases/declare-the-specification.md` and in archived change records, which stay historical. One post-archive commit sweeps the ideation document, triggered by this change's archive, and it rebinds that invocation to `givn ise rebuild-component src/cli/archive.rs` and the nouns, not only the vocabulary.

## Architecture Impact

| Artifact | Kind | Effect |
|---|---|---|
| `src/main.rs` | CLI surface | `IseCommands` becomes `RebuildAll`, `RebuildComponent`, and `Compare`; the removed flags disappear from the parser. |
| `src/cli/ise.rs` | CLI adapter | Refusal prefixes follow the verb; the report and comparison formats change. |
| `src/ise/mod.rs` | domain entry | `RebuildRequest` replaces `ReplaceRequest`; `Manifest` replaces the trial record; the report struct carries the new fields. |
| `src/ise/partition.rs` | domain | The scope is `All` or `Paths(Vec<String>)`; the vocabulary moves to Specification files, Source code kept, and Source code removed. |
| `src/ise/record.rs` → `src/ise/manifest.rs` | domain | The record becomes the manifest: new scope, field set, writer, and checksum maps. |
| `src/ise/verify.rs` → `src/ise/compare.rs` | domain | The re-check becomes the three-subject comparison. |
| `tests/steps/clean_room_preparation_steps.rs` | step definitions | Binds the new command phrases and report lines. |
| `tests/steps/trial_input_verification_steps.rs` | step definitions | Binds the comparison phrases. |
| `tests/steps/ise_clean_room_support.rs` | fixtures | `canonical_checksum_project()`, `remove_declaration_blob()`, and the manifest readers. |
| `e2e/tests/steps/clean_room_preparation_e2e_steps.rs`, `e2e/tests/steps/trial_input_verification_e2e_steps.rs` | E2E step definitions | Bind the two `@e2e` scenarios that carry the new command names. |
| `assets/instructions/ise.md` | served instruction | Names the three commands, one sentence of purpose each, and the four rule sentences. |
| `assets/instructions/archive.md` | served instruction | Vocabulary-only: the completion-workflow gate names the Specification-role entry sweep with the settled nouns. |
| `docs/reference.md` | contracts role | Command table, both command sections, the report lines, the five count definitions, the manifest schema, and the refusals. |
| `docs/glossary.md` | structure role (declared) | Retires the trial vocabulary and adds `Spec copy` and `Checksum`. |
| `docs/ideation/ise-rebuildability.md` | ideation topic | Restates §3.3 and §3.4 in the settled surface. |
| `docs/arc42/02-architecture-constraints.md` | limits role (declared) | TC-46, TC-47, TC-49, and TC-51 name the new commands, the manifest scope, and the report. |
| `docs/arc42/08-crosscutting-concepts.md` §8.29 | durable architecture fact | Carries the canonical checksum form, the enumeration, and the compatibility rule. |
| `docs/arc42/adr/ADR-076-clean-room-trial-record.md` and its registers | ADR amendment | Records the rename and the manifest cut; the historical note keeps the retired command names. |
| `docs/arc42/10-quality-requirements.md` | quality scenario | Adds Q-127: a second implementation reproduces the manifest's checksums byte for byte. |
| `docs/arc42/11-risks-and-technical-debt.md` | risk register | Extends D-9 and adds R-160 (pinned-fixture re-pin), R-161 (command rename), and R-162 (kept Source code is not verified). |
| `givn/changes/strengthen-ise-specification/specs/**` | delta | Rewritten in the settled vocabulary; every permanent scenario that carries a retired token or a changed assertion has a `@givn.removed` or `@givn.modified` entry. |
| `README.md` | project orientation | Vocabulary-only: the command table names the three commands. |
| `givn/specs/obtain-effective-guidance/usecase.md` | permanent use case (`behavior` role) | Vocabulary-only: the served instruction's command list. |
| `docs/arc42/03-context-and-scope.md`, `04-solution-strategy.md`, `05-building-block-view.md`, `09-architecture-decisions.md` | arc42 chapters | Renamed commands, manifest scope, report lines, and comparison subjects. |
| `docs/arc42/adr/ADR-078-frozen-package-outside-the-trial.md` | ADR register | Vocabulary-only cross-reference to the renamed command. |
| `docs/arc42/README.md`, `docs/arc42/adr/README.md` | registers | The amended ADR-076 row and the chapter index. |
| `docs/domain-model.md` | structure role (declared) | No change: the command surface is not a domain entity; `Spec copy`, `Manifest`, and `Checksum` are glossary terms, not model nodes. |

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| `SpecCopy` (rebuild-all, rebuild-component, compare) | prepare-an-ise-test | `crate::cli::ise`, the independent verifier | `crate::ise` (`mod`, `partition`, `manifest`, `compare`) | A third consumer of the manifest appears, or a second copy-resolution rule is needed |
| `SpecificationDeclaration` | declare-the-specification | `crate::ise`, `crate::cli::lint`, `crate::e2e_boundary` | `crate::config::declaration` | A second declaration format appears, or a reader must not share the strict parser |
| `E2eBoundary` (`resolve`, `roots`, `contains`, `e2e_file_set`) | prepare-an-ise-test, verify-change | `crate::cli::check`, `crate::cli::lint`, `crate::ise` | `crate::e2e_boundary` | The boundary gains a second package-resolution strategy |

`crate::ise` keeps one contract and one writer. `cli::ise` stays the formatter and exit-code adapter. Renaming the module files is the cutover; no shim module keeps the old names.

## Decisions

Decides D1: a `rebuild-*` command writes a manifest only when `--manifest <file>` is given, and never prints it. Realized by the report lines, the CLI parser, and `A command without a manifest path writes no manifest`. Rejected: printing the manifest after the report (two output shapes for every consumer, and a manifest with no path to name).

Decides D2: the capability identifiers `clean-room-preparation` and `trial-input-verification` keep their names. Realized by unchanged delta paths, the interaction inventory, and the proposal's out-of-scope entry. Rejected: renaming `trial-input-verification` (a permanent corpus move with no behaviour change, and two paths for one capability during the merge).

Carried D3: a `Rule:`-nested `@e2e` scenario stays outside the per-feature scenario checksum. Default in this change: the parser exposes no `Rule` scenarios, so the selection excludes them and no current corpus file is affected. Owner: the independent verifier. Trigger: a `Rule:`-nested `@e2e` scenario enters the frozen corpus, or the acceptance runner executes one. Why switching is cheap: the selection is one filter in `src/gherkin/content.rs`; changing it changes checksum bytes and therefore needs a new scope. Recorded as the extended debt D-9.

Decides D4: `compare` refuses an `ise-trial/v1` file by name. Realized by the manifest reader, the reference's scope rule, `Comparison refuses a manifest whose scope is not ise-manifest/v1`, and `Comparison refuses a retired ise-trial/v1 record by scope`. Rejected: reading both shapes (two field sets, ambiguous count mapping).

Decides D5: the command surface is `rebuild-all`, `rebuild-component`, and `compare`. `rebuild-all` takes no path; `rebuild-component` takes at least one path and refuses an empty list as a usage error with exit 2; `--full` and the `replace`/`verify` names do not exist. Realized by the parser, the reference, the instruction, and the delta scenarios, including `The retired command names and flags exit 2`, which runs `givn ise replace`, `givn ise verify`, and `givn ise rebuild-all --full` and asserts exit 2 with no copy. Refusal prefixes are `ise rebuild-all:`, `ise rebuild-component:`, and `ise compare:`, each matching the command that refused. A path `rebuild-component` refuses because it falls outside every declared source root names `rebuild-all` in the refusal sentence. Rejected: keeping `replace` as a hidden alias (the retired name would survive and two surfaces would need tests); keeping `--full` (a flag would select the action the command name already states).

Decides D6: the checksum bytes are the canonical rule. One path checksum is `sha256(path, NUL, kind, NUL, executable, NUL, content)` with kind `f` (regular), `x` (executable regular), `l` (symlink), or `g` (gitlink); for `g` the content bytes are the recorded commit id and the executable marker is `0`, and a gitlink in the End-to-end (E2E) set is hashed this way, while a gitlink in the copy set refuses because the copy cannot hold history. A rollup is `sha256` over each sorted `path, NUL, checksum, LF`; `end_to_end_checksum` rolls up the union of `checksums.end_to_end` and `checksums.scenarios` keyed by path, with the scenario checksum winning when a path is in both maps; the per-feature scenario checksum hashes the feature path and each selected scenario's title and compact JSON executable content. The report prints the first 12 lowercase hex characters with no ellipsis; the manifest keeps 64. Realized by the delta rules, `docs/reference.md` Checksums, §8.29, the ADR-076 amendment, and `Preparation records the canonical checksums for a pinned fixture`. Rejected: simplifying the digest input (a behaviour change that invalidates every manifest and fixture); keeping the ellipsis (the settled report line is `<12 hex>`, and the ellipsis is not part of a checksum).

Decides D7: the manifest is `ise-manifest/v1` with the twelve fields the proposal lists, including the five counts nested under `counts` and the three `checksums` maps. `specification_files`, `source_kept_files`, and `end_to_end_files` are sorted path lists; `end_to_end_files` is the union of the keys of `checksums.end_to_end` and `checksums.scenarios`; `source_kept_files` carries no checksums, so a kept edit is not a difference and a deleted kept path is refused; `checksums.end_to_end` holds one path checksum per `end_to_end_files` path, including a feature path carrying a selected `@e2e` scenario, and `checksums.scenarios` holds one checksum per feature scenario set. The scope has no optional field: the reader requires every listed field, and the older `ise-trial/v1` shape is refused by name. A field added later is optional only when the scope states its default; a field with no derivable default opens a new scope, so an older manifest is never reinterpreted. The prepare-time backend is no longer evidence: `compare` re-selects the backend, so the retired `backend` field has no replacement. Realized by `src/ise/manifest.rs`, the reference schema, and the golden scenario. Rejected: keeping `replaced_digest` (the settled field list has no removed checksum; `removed_paths` plus the copy's absence is the evidence); keeping `backend` and `written_path` (`copy_directory` and the selected backend at compare time carry them; the manifest is ephemeral).

Decides D8: the End-to-end (E2E) set is read from the selected revision's tracked paths, and `compare` reads the checkout's content paths. An entry that matches the project root covers the whole tracked tree, and an End-to-end (E2E) match under a `source:` root is neither copied nor removed. Realized by the delta rules, the reference, `A project-root End-to-end entry covers the whole tracked tree`, `An End-to-end entry that overlaps a source root wins over the removal`, and `Comparison ignores an ignored file under the End-to-end package`. The two enumeration scenarios are not merged: the project-root fixture asserts an out-of-root file covered by `.` and an E2E match under `src/**` (2 E2E files, 0 removed), while the overlap fixture asserts a non-E2E sibling under the same source root is removed (1 E2E file, 1 removed); their fixtures and counts differ. Rejected: reading the working tree for the copy (ignored and untracked files would enter) or reading the revision for the checkout (an uncommitted edit would be invisible).

Decides D9: `compare` prints two subject lines, `Specification files` and `End-to-end tests`, and refuses a copy that adds a file under a declared Specification path or loses a recorded kept path. Each subject prints `match` alone when it has no difference, or its non-zero counts in the fixed order `changed`, `missing`, `added`, comma-separated (for example `Specification files: 1 changed, 1 added`); one `- <path>` line prints per changed or missing path, one `+ <path>` line per added path, and one line per path when the copy and the checkout both differ. When a path differs on both sides, one line and one count belong to the subject that owns the path: `End-to-end tests` when the checkout side owns it, `Specification files` otherwise; the copy-side and checkout-side observations are one difference, not two. The copy's Specification files are compared per path; its `source_kept_files` are checked for presence only, so a kept deletion refuses and a kept edit is not a difference (R-162). The added-file refusal tests a declared Specification path matched by any of the five roles, including an `e2e` path, although the copy never carries an `e2e` match. The checkout side compares the recorded Specification paths and the recorded End-to-end (E2E) paths with the checkout's content paths. The End-to-end (E2E) side compares each recorded path's `checksums.end_to_end` content checksum and, for a feature path carrying a selected `@e2e` scenario, its `checksums.scenarios` checksum; either difference is `changed`, so an edit outside the selected scenario is named. A recorded Specification path carrying a selected `@e2e` scenario is owned on the checkout side by `End-to-end tests` and contributes no `Specification files` difference there. Every other differing recorded Specification file prints `Specification changed after the copy was written` and one `- <path>` line; every other absent recorded Specification path is `missing`. An added content path matched by a declared Specification role prints `added` and one `+ <path>` line under `Specification files`, except a content path in the checkout's declared End-to-end (E2E) set or a feature path carrying a selected `@e2e` scenario, which `End-to-end tests` owns and which prints the single `+ <path>` line even when another Specification role also matches it. Realized by `src/ise/compare.rs`, the reference, and the comparison scenarios from the clean report through the missing, added-path, mixed-kind, multi-path, and manifest-strictness cases. Rejected: reporting added Source code as drift (the rebuild creates it; the rule sentences say it is the product); reporting a checkout Specification change as a copy difference (the copy did not change; the checkout did); verifying kept content (it is the product, and R-162 records the exposure).

Decides D10: the unreadable declaration has four distinct refusals, not one. The revision not recording the declaration names the path, the revision, and `does not record`; the backend returning no blob names the path, the revision, and `returned no blob`; non-UTF-8 names the path and the decoding error; malformed names the path and the loader's reason. Each exits 1 and writes nothing. The working-tree declaration is not the rebuild commands' input; `givn lint` owns its unreadable case and exits 1. Realized by the delta rules, `Preparation refuses a declaration the revision records but cannot read`, `Preparation refuses a revision that does not record the declaration`, `Preparation refuses a malformed declaration at the selected revision`, and `Preparation refuses a declaration that is not UTF-8`. Rejected: treating "unreadable" as malformed (it would name the wrong reason and hide a repository-object failure); treating it as absent (the revision does record the path); giving the no-blob and not-recorded cases one sentence (no scenario can distinguish them). The two scenarios share the lint's masked assertion shape because the mask drops quoted literals; their phrases assert different literals (`does not record` versus `returned no blob`), so they are not mergeable.

Decides D11: the three corrected scenarios state the Specification role that keeps the tracked file under `rebuild-all`, and their asserted behaviour does not change. `src/kept.rs` is a Specification file through the `structure` role; the executable script and symlink are Specification files through the `structure` role; each sits under the `source: src/**` root, so the Specification-role-wins rule is the reason it survives. Realized by `Preparation leaves an ignored file out of the spec copy`, `Preparation preserves the executable bit and the symlink target`, and `Preparation reads an explicit revision and reports it`; `A whole-product rebuild keeps a Specification file under a source root` additionally asserts `Source code kept: 0` and `Files beside removed paths: 1`, the reference's direct-parent containment count, with a nested Specification file that a broader reader would count as 2, so both counts are discriminated. Rejected: changing the fixture to remove the Specification role (the scenario would assert a file the copy cannot hold, so the asserted behaviour would change); rewording the rule to exempt these files (the rule is correct and the scenarios were wrong).

Decides D12: `change strategy` is `incremental modification`. Realized by a command-surface and evidence-format edit with no component split, dependency, storage, or deployment change. Affected consumers: `crate::cli::ise` and the independent verifier read the new manifest; `crate::config::declaration` keeps its strict parser. Persisted state: none; the manifest is ephemeral. Rebuild boundary: none; no Ise Test runs as part of this change. Rejected: `Constant Ise` (nothing is rebuilt) and `whole-product rebuild` (no source is regenerated).

Decides D13: the permanent corpus is rebound in the delta. Every permanent scenario whose feature text carries `Replaced:`, `Retained:`, `Kept source:`, `Outside the declaration:`, `Not replaced:`, `Backend:`, `receipt`, `--full`, `--path`, `trial`, or `full replacement` has a delta entry: 29 `clean-room-preparation` scenarios are removed (each replaced by a named added scenario, or retired with the two-flag conflict) and 27 are modified with the new command phrase and report nouns, and the permanent declaration feature holds one modified title; all 27 `trial-input-verification` scenarios are removed and replaced by the comparison set. The five former step-definition survivors are renamed into the delta, so every permanent title the token sweep selects has a removed or modified op. `## Corpus Rebinding` names every permanent title and its disposition; the verification run in this revision parses the permanent features and the delta, selects every permanent scenario whose text carries one of those tokens, and reports zero that lack a removed or modified row. Rejected: leaving a token-bearing scenario on step-definition rebinding (its asserted `Replaced:` line would fail after the merge); rewriting the 27 verification scenarios in place (their subjects changed with the record).

Decides D14: the rebuild commands refuse to write a symlink with an absolute target, even when that target resolves inside the spec copy, and a symlink whose relative target resolves outside the spec copy. An absolute target always refuses; a relative target refuses when resolving it against the link's parent directory leaves the copy. The refusal names the link path and the target, exits 1, and writes nothing. A relative target that resolves inside the copy is written as recorded. Realized by the delta Rules, the reference refusal row, `Preparation refuses a symlink whose target resolves outside the spec copy`, and `Preparation refuses an absolute symlink target`. Relation to R-150: R-150 covers the archive guard reading a declared Specification path that is a symlink into the change tree; this refusal covers a symlink entry written into the copy. The two checks read different surfaces, so R-150 stays accepted and the copy gains a write-time containment boundary. Rejected: writing the target verbatim (a committed link to the checkout hands the builder the removed Source code); resolving an absolute target and accepting it when it lands inside the copy (the copy is a directory a caller may move, so a recorded absolute target can leave the copy after the write); recording the risk only (the copy's purpose is to withhold that Source code).

## Use-Case Traceability

| Use-case guarantee or rule | Decision and artifact | Evidence |
|---|---|---|
| The spec copy is written with the project's Specification files | D6, D7; delta Rules, `docs/reference.md` Manifest | `Developer rebuilds the whole product from its Specification`; the two golden scenarios |
| The agent adds files and those are the rebuilt Source code | D9; delta Rules, reference Compare | `Comparison reports no difference after the agent adds Source code` |
| The agent may edit Source code kept because it is the product | D9, R-162; four rule sentences | `Comparison reports no difference after a kept Source code file is edited` |
| The agent does not delete a file the copy was written with | D9; `source_kept_files`, reference Compare | `Comparison refuses a deleted kept Source code file`; `Comparison names a missing Specification file in the copy` |
| The agent does not modify a Specification file the copy was written with | D9; four rule sentences | `Comparison names a changed Specification file in the copy`; `Comparison reports a Specification changed after the copy was written` |
| An added Specification file in the copy is refused | D9; four rule sentences | `Comparison refuses a file added under a declared Specification path`; `Comparison refuses a file added under an End-to-end role` |
| The checkout's added paths are differences | D9; reference Compare | `Comparison names an added End-to-end file in the checkout`; `Comparison names an added @e2e scenario file in the checkout`; `Comparison names an added End-to-end root in the checkout declaration`; `Comparison names an added Specification file in the checkout` |
| A recorded `@e2e` feature path is owned by `End-to-end tests` on the checkout side | D9; reference Compare | `Comparison names an @e2e scenario changed in the checkout`; `Comparison names an added @e2e scenario file in the checkout`; `Comparison names a missing @e2e feature in the checkout` |
| A recorded path absent from the checkout is a difference for its subject | D9; reference Compare | `Comparison names a missing Specification file in the checkout`; `Comparison names a missing End-to-end test in the checkout`; `Comparison names a missing @e2e feature in the checkout` |
| A structure-role edit changes the declaration and adds the path it newly matches | D9; reference Compare; fixture path `config/kept.toml` | `Comparison reports a Specification changed after the copy was written` |
| A subject line carries every non-zero difference kind in the fixed order | D9; reference Compare | `Comparison names every drifting path in one run`; `Comparison reports a Specification changed after the copy was written` |
| One run names every difference | D9; reference Compare | `Comparison names every drifting path in one run` |
| A path drifting on both sides prints one line | D9; reference Compare | `Comparison prints one line for a path drifting in the copy and the checkout` |
| Checksums are reproducible byte for byte | D6; two golden scenarios | Golden values below; independent implementation |
| A gitlink in the End-to-end set is checksummed with kind `g` | D6, D8; delta Rules | `Preparation checksums a repository link in the End-to-end set` |
| The End-to-end (E2E) set is the revision's tracked tree | D8; delta Rules | The `@e2e` scenario's `End-to-end tests: not copied (2 files)` line; the golden fixture; `A project-root End-to-end entry covers the whole tracked tree`; `An End-to-end entry that overlaps a source root wins over the removal` |
| The comparison reads the checkout's content paths | D8; delta Rules | `Comparison ignores an ignored file under the End-to-end package`; `Comparison names an End-to-end test changed in the checkout` |
| A recorded End-to-end path compares its content checksum and, when it carries a selected `@e2e` scenario, its scenario checksum | D6, D7, D9; reference Compare | `Comparison names a content-only change to an @e2e feature in the checkout`; `Comparison names an @e2e scenario changed in the checkout` |
| A pre-scope manifest is refused by name | D4; reference Manifest | `Comparison refuses a manifest whose scope is not ise-manifest/v1` |
| The unreadable declaration's four refusals are distinct in output | D10; delta Rules and Extensions | `Preparation refuses a declaration the revision records but cannot read` asserts `returned no blob`; `Preparation refuses a revision that does not record the declaration` asserts `does not record`; `Preparation refuses a malformed declaration at the selected revision` asserts the loader's reason; `Preparation refuses a declaration that is not UTF-8` asserts the decoding error |
| A Specification file under a source root survives `rebuild-all` | D11; corrected scenarios | `Preparation leaves an ignored file out of the spec copy`; `Preparation preserves the executable bit and the symlink target`; `Preparation reads an explicit revision and reports it`; `A whole-product rebuild keeps a Specification file under a source root` |
| A symlink with an absolute target, or a relative target outside the copy, is refused | D14; delta Rules and reference Refusals | `Preparation refuses a symlink whose target resolves outside the spec copy`; `Preparation refuses an absolute symlink target` |
| The retired command names and flags exit 2 | D5 | `The retired command names and flags exit 2` |
| The served ise guidance names the spec copy as the declaration's carrier | D5, D13; `assets/instructions/ise.md`, `docs/reference.md` | `Served ise guidance states that the trial always carries the declaration` |
| The served ise instruction states that a declared Specification path survives `givn archive` and names the move | D5, D13; `assets/instructions/ise.md`, `docs/glossary.md` | `The ise instruction says declared retained paths survive archiving` |
| No rebuild runs for this change | D12 | No Ise Test mode; no spec copy is prepared by this change |

Persona impact: the `developer--primary-user` reads a manifest they can reproduce and commands that name their action, evidenced by the clean `compare` scenario and the report lines; the `agent--technical-role` reads a Specification that states the bytes and the four rule sentences, evidenced by the golden checksum scenario and the delta Rules; the `givn-maintainer--owner` gains a scope cut and a proof, evidenced by `The retired command names and flags exit 2` and the corpus table. No Persona biography enters Gherkin.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| `clean-room-preparation` (non-`@e2e`) | `tests/steps/clean_room_preparation_steps.rs` | n/a - permanent file, rewritten in place |
| `clean-room-preparation` (`@e2e`) | `e2e/tests/steps/clean_room_preparation_e2e_steps.rs` | n/a - permanent file, rewritten in place |
| `trial-input-verification` (non-`@e2e`) | `tests/steps/trial_input_verification_steps.rs` | n/a - permanent file, rewritten in place |
| `trial-input-verification` (`@e2e`) | `e2e/tests/steps/trial_input_verification_e2e_steps.rs` | n/a - permanent file, rewritten in place |
| shared fixtures | `tests/steps/ise_clean_room_support.rs` | n/a - permanent file, edited in place |
| `specification-declaration` | `tests/steps/specification_declaration_steps.rs` | n/a - permanent file, edited in place |
| `prose-consolidation` (non-`@e2e`) | `tests/steps/prose_consolidation_steps.rs` | n/a - permanent file, edited in place |
| `instruction-integrity` (non-`@e2e`) | `tests/steps/instruction_integrity_steps.rs` | n/a - permanent file, edited in place |

The feature files are authoritative for the exact phrase literals; this table lists the new and changed families, not every variant. A phrase in the delta that is absent here is still bound by the named file before GREEN, and the task setup task proves strictness once.

New and changed phrases:

| Phrase (exact) | Binding file | Note |
|---|---|---|
| `the developer rebuilds the whole product from "HEAD" into a spec copy with a manifest` | `clean_room_preparation_steps.rs` | Runs `ise rebuild-all --into <copy> --manifest <file>`. |
| `the developer rebuilds the whole product from "HEAD" into a spec copy` | `clean_room_preparation_steps.rs` | Same, no manifest. |
| `the developer rebuilds the component path "<path>" into a spec copy with a manifest` | `clean_room_preparation_steps.rs` | Runs `ise rebuild-component <path> ...`. |
| `the developer rebuilds the component path "<path>" into a spec copy` | `clean_room_preparation_steps.rs` | Same, no manifest. |
| `the developer runs "givn ise rebuild-component" with no path and a target` | `clean_room_preparation_steps.rs` | Asserts the usage exit 2. |
| `the developer runs a dry run of the whole product from "HEAD" into a spec copy with a manifest` | `clean_room_preparation_steps.rs` | Asserts the wet-run labels are absent. |
| `the developer rebuilds the whole product from the first revision into a spec copy` | `clean_room_preparation_steps.rs` | `--at <first>`. |
| `the spec copy holds "<path>"`, `the spec copy omits "<path>"` | `clean_room_preparation_steps.rs` | Replaces the trial-path assertions. |
| `the report prints "Specification files: 2", ...` | `clean_room_preparation_steps.rs` | Existing multi-line report step, new literals. |
| `the report prints a "Specification checksum:" line and an "End-to-end checksum:" line of 12 lowercase hex characters each` | `clean_room_preparation_steps.rs` | Exact 12-hex match, no ellipsis. |
| `the manifest records the scope, the revision, the removed paths, the five counts, the checksums, the copy directory, and the Givn version` | `clean_room_preparation_steps.rs` | Reads the YAML manifest. |
| `the manifest's specification checksum is "<hex>"`, `the manifest's end-to-end checksum is "<hex>"` | `clean_room_preparation_steps.rs` | Full 64 hex. |
| `the manifest's removed paths are:` | `clean_room_preparation_steps.rs` | Table of paths. |
| `the manifest's specification checksums are:`, `the manifest's end-to-end checksums are:`, `the manifest's scenario checksums are:` | `clean_room_preparation_steps.rs` | Tables of path and checksum; each step asserts exact map equality, so an extra or missing row fails. |
| `the pinned canonical checksum fixture` | `clean_room_preparation_steps.rs` | Calls `canonical_checksum_project()`. |
| `the pinned richer checksum fixture` | `clean_room_preparation_steps.rs` | Calls `rich_checksum_project()`, the canonical fixture with the background/docstring/table/outline feature. |
| `the manifest's scenario checksum for "<path>" is "<hex>"` | `clean_room_preparation_steps.rs` | Reads `checksums.scenarios`. |
| `the committed declaration's blob is absent from the object store` | `clean_room_preparation_steps.rs` | Calls `remove_declaration_blob()`. |
| `the developer rebuilds the whole product from "<revision>" into a spec copy` | `clean_room_preparation_steps.rs` | `--at` with an explicit revision, including the zero revision. |
| `the developer rebuilds the component paths "<path>" and "<path>" into a spec copy` | `clean_room_preparation_steps.rs` | Repeated-path case. |
| `the developer runs rebuild-all from "HEAD" with no target`, `the developer runs rebuild-all into a directory inside the checkout`, `the developer runs rebuild-all into that target`, `the developer runs rebuild-all through the symlinked target` | `clean_room_preparation_steps.rs` | Target refusals; no valid copy is written. |
| `the developer runs a dry run of the whole product from "HEAD" into a spec copy with a manifest`, `... without a manifest` | `clean_room_preparation_steps.rs` | Asserts the conditional wet-run labels. |
| `the developer runs the retired command "<command>" with a target`, `... with a manifest`, `the developer runs "givn ise rebuild-all --full" with a target` | `clean_room_preparation_steps.rs` | Exit 2, unknown command or flag, no copy. |
| `a Git project with ise enabled and a declaration whose "<role>" role names "<glob>"` and its source-root and multi-role variants | `clean_room_preparation_steps.rs` | Fixture declaration in settled vocabulary; replaces the `retains`/`declares`/`replaces` phrase family. |
| `a Git project with ise enabled and a declaration with one source root "<glob>"` | `clean_room_preparation_steps.rs` | Source-only declaration with five empty roles. |
| `a declaration whose "<role>" role names "<path>" and "<path>" and whose source root is "<glob>"` | `clean_room_preparation_steps.rs` | Multi-entry role for the containment-count fixture. |
| `the spec copy holds no ".git" entry and no ".fslckout" entry` | `clean_room_preparation_steps.rs` | No version-control metadata in the copy. |
| `the guidance states that the spec copy always carries the declaration itself` | `prose_consolidation_steps.rs` | Asserts the served ise sentence. |
| `the guidance states that it is an implicit Specification entry, not a sixth role` | `prose_consolidation_steps.rs` | Asserts the second served ise sentence. |
| "the served instruction says a declared Specification path must survive `givn archive`" | `instruction_integrity_steps.rs` | Asserts the path noun and the archive refusal. |
| `the spec copy's "<path>" is the declaration committed at the selected revision` | `clean_room_preparation_steps.rs` | The copy carries the revision's declaration. |
| `a committed revision with "<path>", a Specification file through the "<role>" role` | `clean_room_preparation_steps.rs` | Fixture role qualifier; decides whether the path is a Specification file. |
| `a committed revision with "<path>" and a symlink "<path>" to an absolute target inside the spec copy` | `clean_room_preparation_steps.rs` | Commits the file and a symlink whose target is the absolute path under the planned spec copy; the link resolves inside the copy. |
| `the spec copy holds "<path>"`, `the spec copy omits "<path>"`, `the spec copy's "<path>" is executable`, `the spec copy's "<path>" is a symlink to "<target>"` | `clean_room_preparation_steps.rs` | Copy-content assertions. |
| `the spec copy path and the manifest path remain absent`, `the checkout holds no spec copy directory and no manifest`, `the checkout holds no manifest` | `clean_room_preparation_steps.rs` | Refusal leaves both targets absent. |
| `the manifest lists "<path>" among the Specification files` | `clean_room_preparation_steps.rs` | Reads `specification_files`. |
| `the manifest's End-to-end files hold ...`, `the manifest's End-to-end files are empty` | `clean_room_preparation_steps.rs` | Reads `end_to_end_files`; the listed paths are the complete sorted inventory, so an extra or missing path fails. |
| `the manifest's end-to-end checksums hold a 64-hex checksum for "<path>"` | `clean_room_preparation_steps.rs` | Reads `checksums.end_to_end`. |
| `the command exits 2 and names the missing path list`, `... names the unknown command`, `... names the unknown flag` | `clean_room_preparation_steps.rs` | Usage errors. |
| `the command fails and exits 1, naming "<path>" and "<word>"` | `clean_room_preparation_steps.rs` | Assertion on exit, stderr path, and the named command, target, or revision. |
| `the command fails and names "givn/specification.yaml", the revision, and "does not record"` | `clean_room_preparation_steps.rs` | Asserts exit 1, the declaration path, the revision, and `does not record`; the blob-absent phrase must not satisfy it. |
| `the command fails and names "givn/specification.yaml", the revision, and "returned no blob"` | `clean_room_preparation_steps.rs` | Asserts exit 1, the declaration path, the revision, and `returned no blob`; the not-recorded phrase must not satisfy it. |
| `the command fails and exits 1, naming "<path>" and its absolute target` | `clean_room_preparation_steps.rs` | Asserts stderr names the link and the absolute target text the fixture recorded. |
| `a prepared spec copy with a manifest` | `trial_input_verification_steps.rs` | Prepares with `ise rebuild-component src/keys/parser.rs --manifest`, so `src/keys/steps.rs` is kept and `src/keys/parser.rs` is removed. The fixture declares `behavior: ["givn/specs/**"]`, `structure: []`, `contracts: ["docs/**"]`, `limits: []`, `e2e: ["tests/e2e"]`, and `source: ["src/**"]`, and commits `givn/specs/demo/spec.feature` with one `@e2e` scenario, `docs/reference.md`, `docs/arc42/02-architecture-constraints.md`, `tests/e2e/Cargo.toml`, `tests/e2e/steps/keys.rs`, `src/keys/steps.rs`, `src/keys/parser.rs`, and `config/kept.toml`. |
| `a prepared spec copy whose manifest names the scope "<scope>"`, `... carries an unknown field`, `... is missing the "<field>" field`, `... holds a malformed Specification checksum`, `... is not readable YAML`, `... names another copy directory`, `... with its manifest inside the copy` | `trial_input_verification_steps.rs` | Manifest mutations after preparation. |
| `the verifier compares the spec copy with the manifest` | `trial_input_verification_steps.rs` | Runs `ise compare --manifest <file> --copy <dir>`. |
| `the agent edits the Specification file "<path>" the spec copy was written with`, `the agent deletes the Specification file "<path>" the spec copy was written with`, `the agent edits the kept Source code file "<path>" the spec copy was written with`, `the agent deletes the kept Source code file "<path>" the spec copy was written with` | `trial_input_verification_steps.rs` | Copy mutations at the named path. |
| `the agent adds a rebuild file at a removed path` | `trial_input_verification_steps.rs` | Writes at a `removed_paths` entry; the rebuild's Source code. |
| `the agent adds a file under a declared Specification path`, `the agent adds a file under the declared End-to-end role` | `trial_input_verification_steps.rs` | Added-file refusal across the five roles. |
| `the developer edits the End-to-end (E2E) test "<path>" in the checkout`, `the developer edits the Specification file "<path>" in the checkout`, `the developer adds an ignored file under the checkout's End-to-end (E2E) package` | `trial_input_verification_steps.rs` | Checkout mutations at the named path. |
| `the developer edits the feature description of "<path>" in the checkout` | `trial_input_verification_steps.rs` | Rewrites only the `Feature:` line; the selected scenario text is unchanged. |
| `the developer adds the End-to-end (E2E) file "<path>" in the checkout` | `trial_input_verification_steps.rs` | Added content path in the declared E2E set. |
| `the developer adds the feature file "<path>" with an "@e2e" scenario in the checkout` | `trial_input_verification_steps.rs` | Added feature path owned by `End-to-end tests`. |
| `the developer adds the e2e root "<root>" to the checkout declaration with the file "<path>"` | `trial_input_verification_steps.rs` | Checkout declaration edit and the file the added root brings. |
| `the developer adds the contract file "<path>" in the checkout` | `trial_input_verification_steps.rs` | Added checkout Specification file. |
| `the developer deletes the Specification file "<path>" in the checkout`, `the developer deletes the End-to-end (E2E) test "<path>" in the checkout` | `trial_input_verification_steps.rs` | Deletes the named tracked file from the checkout; the file was recorded at preparation. |
| `the developer rewrites the checkout declaration's "structure" role to "<path>"` | `trial_input_verification_steps.rs` | Rewrites the role in the checkout declaration; the named path is committed outside every declared role and source root. |
| `the project disables ise after preparation`, `the project disables architecture principles after preparation` | `trial_input_verification_steps.rs` | Addon mutations. |
| `a prepared spec copy whose manifest is a retired ise-trial/v1 record` | `trial_input_verification_steps.rs` | Writes the retired field set (`schema`, `content_scope`, `backend`, the three digests, the three maps); the reader checks `content_scope` before deserialization. |
| `comparison prints "<line>"`, `comparison prints "<line>" and "<line>"`, `comparison prints "<line>", "<line>", and "<line>"` | `trial_input_verification_steps.rs` | Exact stdout lines, including the literal `- <path>` and `+ <path>` difference lines; the retired `comparison prints the changed path` phrase has no binding. |
| `comparison prints exactly one "<line>" line under the "<subject>" subject` | `trial_input_verification_steps.rs` | Counts the exact line once and asserts the subject line that owns it; a duplicate line fails. |
| `comparison exits zero`, `comparison exits 1` | `trial_input_verification_steps.rs` | Exit code. |
| `comparison prints no line beginning "- "` | `trial_input_verification_steps.rs` | Clean report has no difference line. |
| `comparison prints no "Specification files:" line`, `comparison prints no "Specification files: match" line` | `trial_input_verification_steps.rs` | Refusal before the report. |
| `comparison fails and exits 1, naming the added path as a Specification change`, `... naming the manifest and "<word>"`, `... naming the manifest and the unknown field`, `... naming the manifest and the missing field`, `... naming the manifest and the malformed checksum`, `... naming the manifest and the parse error`, `... naming the manifest and the copy`, `... naming the manifest`, `... naming "<path>"`, `... naming "ise"`, `... naming "architecture_principles"` | `trial_input_verification_steps.rs` | Refusal reasons. |
| `the command exits 1` | `specification_declaration_steps.rs` | Existing addition; asserts the process code is exactly 1. |

`canonical_checksum_project()` writes the bytes the delta use case example names. It excludes `givn/config.yaml` through `.git/info/exclude`, writes the declaration and the six committed files, then commits. `remove_declaration_blob()` resolves `HEAD:givn/specification.yaml`, removes its loose object, and leaves the revision tree listing the path. Neither helper imports `crate::ise`.

## Test Runner

- `verify.command`: `./run-tests.sh --non-e2e`.
- `verify.e2e_command`: `./run-tests.sh --e2e --package e2e`.
- The regular runner is `tests/features_runner.rs` with `cucumber-rs`; it executes the permanent `givn/specs/**` corpus and, when `GIVN_FEATURES` names the change, the change's `specs/**`. The change check uses `GIVN_FEATURES=givn/changes/strengthen-ise-specification`; the archive runs both scopes over the merged corpus.
- The E2E runner is `e2e/tests/e2e_runner.rs`; it selects `@e2e` scenarios from the same feature corpus and drives the built binary through `GIVN_BIN`.
- Single-scenario run (regular):
  ```sh
  GIVN_FEATURES=givn/changes/strengthen-ise-specification \
    cargo nextest run --test features_runner \
    'prepare-an-ise-test/clean-room-preparation.feature::Preparation records the canonical checksums for a pinned fixture'
  ```
- Single-scenario run (`@e2e`):
  ```sh
  GIVN_FEATURES=givn/changes/strengthen-ise-specification \
    GIVN_BIN=target/debug/givn \
    cargo nextest run --manifest-path e2e/Cargo.toml --test e2e_runner \
    'prepare-an-ise-test/clean-room-preparation.feature::Developer rebuilds the whole product from its Specification'
  ```
- The change replaces two `@e2e` scenarios with two: the whole-product rebuild and the comparison. `Developer rebuilds one component and leaves its neighbours` is a regular variant of the prepare action, because `rebuild-component` and `rebuild-all` differ in scope, not in the action a consumer performs, and the inventory holds one prepare action.

## Strict Mode

```text
cucumber-rs: .fail_on_skipped() on both Cucumber builders in tests/features_runner.rs
cucumber-rs: .fail_on_skipped() on the e2e runner builder in e2e/tests/e2e_runner.rs
```

An undefined or pending step fails the run; `skipped > 0` exits nonzero in both runners. Step bodies are never empty. A not-yet-implemented phrase uses `todo!("TODO: implement the <phrase> step")` while RED; the GREEN commit replaces it. The tasks setup task proves strictness once with a probe scenario whose step binds nothing and expects a non-zero exit.

## Visual Design Contract

- Interface: none
- Viewports: none; no rendered screen changes. The changed interface is the CLI text output.
- Design system: not applicable
- Screens covered: none
- Rendered reference: none
- Capture mechanism and evidence path: the two `@e2e` scenarios commit a captured terminal transcript under their evidence paths; the report and comparison output are asserted in the scenarios.
- Motion: not applicable

Reason for `none` and Persona: the change edits the CLI report and the Specification text; it adds no browser or terminal screen. The owning use case names `developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner`; the change serves all three. Operator confirmation: this change introduces no screen, so no visual contract is needed.

## E2E Smoke Test Infrastructure

- **E2E runner command**: `./run-tests.sh --e2e --package e2e`, with `GIVN_RESULT_SCOPE=e2e` set by the runner when the caller does not supply one.
- **E2E step definition location**: `e2e/tests/steps/clean_room_preparation_e2e_steps.rs` and `e2e/tests/steps/trial_input_verification_e2e_steps.rs`, one file per capability, separate from the regular step files.
- **Local test infrastructure**: the compiled `givn` binary under `target/`, `git` and `fossil` on `PATH`, and a temporary directory per scenario. No port, database, container, or service.
- **E2E framework choice and justification**: `cucumber-rs` is the project's Gherkin runner; the CLI subprocess is the real interface for a terminal product. The `e2e` package links no production crate.
- **E2E strict-mode proof**: `.fail_on_skipped()` plus the `failed > 0 || skipped > 0 || total == 0` exit rule, proven once by the setup task's probe.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| `clean-room-preparation`: write the spec copy for the whole product or for named component paths | Developer rebuilds the whole product from its Specification | CLI | Run the compiled `givn` binary as a subprocess (`ise rebuild-all --into <copy> --manifest <file>`) against a temporary Git fixture and assert stdout, exit code, and the written tree |
| `trial-input-verification`: compare the spec copy and the checkout with the manifest | Verifier compares the spec copy with the manifest | CLI | Run the compiled `givn` binary as a subprocess (`ise compare --manifest <file> --copy <dir>`) and assert the two subject lines and the exit code |
| `specification-declaration`: declare the five roles and the replaceable source roots | The declared roles are read by the project-health pass | CLI | Run the compiled `givn` binary as a subprocess (`givn lint`) against a temporary project fixture selecting opencode and assert the exit code, stderr, and the captured terminal transcript |

`rebuild-component` is the same prepare action under a narrower scope: `Developer rebuilds one component and leaves its neighbours` drives the binary and asserts the neighbouring files, the counts, and `removed_paths`, and is a regular scenario because it differs from `rebuild-all` in scope, not in the action. The refusals are regular variants of the same action.

The `specification-declaration` inventory is unchanged: the five roles, the source roots, and the consumer action stay as declared, and this change only pins the unreadable working-tree declaration's exit code in `An unreadable declaration is refused by name`.

## Black-Box-First

Every new scenario drives the compiled binary through `tests/common::run_givn` and asserts stdout, stderr, the exit code, or the written manifest. No scenario calls `crate::ise` or `crate::config::declaration` in process. The independent second implementation is a design-time script, not a committed test: it reads the fixture bytes and the feature text, applies the stated rule, and compares the values; its inputs and outputs are recorded in `## Independent Second Implementation`, and this change claims no `results/` directory. The unit tests inside `src/ise/partition.rs` stay: they cover the set arithmetic over synthetic entries that whole fixture projects cannot distinguish.

## External Boundary Cases

| Boundary | Adversarial cases that must exist |
|---|---|
| Declaration blob at the revision | Absent from the revision, recorded but unreadable, non-UTF-8, malformed |
| Working-tree declaration | Path is a directory (unreadable), malformed, unknown key |
| Manifest | Unknown field, missing field, wrong scope, malformed checksum, unreadable file, `ise-trial/v1` scope, copy directory mismatch, manifest inside the copy |
| Spec copy input | Ignored file, untracked non-ignored file, uncommitted edit, symlink, executable bit |
| Symlink target | An absolute target that resolves inside the copy, a relative target that escapes the copy, a relative target that stays inside |
| Spec copy after the rebuild | Added Source code, added file under a Specification path, added file under an `e2e` role path, edited Specification file, deleted Specification file, deleted kept Source code file |
| Checkout after the copy | Edited E2E file, edited scenario, content-only edit of a feature carrying a selected `@e2e` scenario, edited Specification file, edited declaration (role entry moved), added E2E file, added `@e2e` feature file, added E2E root in the declaration, added Specification path, deleted Specification file, deleted `@e2e` feature file, deleted End-to-end file, ignored file |
| End-to-end repository link | A gitlink in the End-to-end set is checksummed as kind `g`; a gitlink in the copy set refuses |
| End-to-end enumeration | An entry that matches the project root, an entry that overlaps a `source:` root, an entry that covers the declaration, an entry that matches nothing |
| Both-side drift | A Specification path edited in the copy and in the checkout in one run; one `- <path>` line |
| Revision read | Declaration not recorded, recorded blob absent, non-UTF-8, malformed; missing revision |
| Command line | `rebuild-component` with no path, an omitted target, a path outside every source root, a path matched by a Specification role, the retired `replace`/`verify` names, `--full` |

The manifest's cases are sensitivity checks: the unknown-field, missing-field, wrong-scope, and copy-directory cases each distinguish a plausible but wrong reader.

## Local Runnability

- All scopes: `./run-tests.sh`.
- Regular scope: `./run-tests.sh --non-e2e`.
- E2E scope: `./run-tests.sh --e2e --package e2e`.
- Prerequisites: `git`, `fossil`, `git-cliff`, and `script` on `PATH`; the Rust toolchain the lockfile pins.
- Isolated network: none needed. Every fixture is a temporary local directory; no external service, no digital twin, no network call.
- No interface obstacle is foreseen: the new scenarios drive the CLI through `std::process::Command` and read local files. The one subtlety is that `remove_declaration_blob()` must remove a loose object before any Git maintenance packs it; the fixture uses a fresh repository with no `gc`.

## Coverage

Coverage is enabled for this project. The change starts no new production process: every scenario runs the compiled `givn` binary, exactly as the existing ISE scenarios do.

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| `givn` CLI | `tests/common::run_givn` inside each scenario | the `cargo llvm-cov`-instrumented binary built by `./measure-coverage.sh` | `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml` | `merge-coverages.sh` | `src/gherkin/content.rs` `e2e_scenario_digest`, exercised by the golden scenario, and `src/ise/manifest.rs` `path_checksum`, exercised by every preparation scenario and `src/ise/compare.rs` `compare`, exercised by the comparison scenarios |

Measured population: `src/` production code.

Excluded: `tests/`, `e2e/`, generated code, and fixture modules. Declared before the first run; not changed after a result.

## Corpus Rebinding

The delta performs the rebinding. Every permanent scenario whose feature text contains `Replaced:`, `Retained:`, `Kept source:`, `Outside the declaration:`, `Not replaced:`, `Backend:`, `receipt`, `--full`, `--path`, `trial`, or `full replacement` has a `@givn.removed` or `@givn.modified` entry in this change. Counts: `clean-room-preparation` 56 permanent scenarios = 29 removed + 27 modified; `trial-input-verification` 27 permanent scenarios = 27 removed, replaced by the comparison set; the delta adds 72 scenarios: 39 in `clean-room-preparation` and 33 in `trial-input-verification`, two of the latter added by the review-blocker batch (`Comparison names a missing declaration in the copy` and `Comparison refuses a retired ise-trial/v1 record by scope`); beside them the delta holds 30 `@givn.modified` entries, two of them cross-capability (`create-change/prose-consolidation.feature` and `fragments/instruction-integrity.feature`) whose permanent titles assert the same retired nouns in served text, and 56 `@givn.removed` entries, so the delta holds 158 scenarios. Every permanent title in the two prepare features has one op; the declaration feature holds one modified title and six unmodified scenarios. The five former step-definition survivors are renamed with a removed entry and a new added scenario.

The `@e2e` fixtures follow the same binding. `e2e/tests/steps/clean_room_preparation_steps.rs` prepares with `ise rebuild-component <path>... --manifest <file>` and reads `Copy directory:`/`Manifest:`; `e2e/tests/steps/trial_input_verification_steps.rs` compares with `ise compare --manifest <file> --copy <dir>`; and `e2e/tests/steps/ise_clean_room_support.rs` writes the existing and blocked manifest fixtures as `ise-manifest/v1` at the manifest path. The pre-merge permanent `@e2e` report assertions read the settled labels, so the tier runs the settled commands until the delta replaces those scenarios.

| Permanent scenario (exact title) | Plane | Edit |
|---|---|---|
| `Developer prepares a clean-room trial from a recorded revision` | delta | Removed; replaced by `Developer rebuilds the whole product from its Specification`. |
| `Preparation refuses when the declaration is missing` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a malformed declaration` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses while the ise addon is off` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses while architecture principles are off` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a dirty checkout and names every dirty path` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a project with no version-control backend` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a project root nested inside a Git worktree` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a checkout carrying both Git and Fossil markers` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a retained path missing at the revision` | delta | Removed; replaced by `Preparation refuses a Specification path missing at the revision`. |
| `Preparation refuses a path outside every source root` | delta | Modified: `--path` command phrase → `rebuild-component`; `trial`/`receipt` absent nouns → `spec copy`/`manifest`. |
| `Preparation refuses a path matched by a retained role` | delta | Removed; replaced by `Preparation refuses a path matched by a Specification role`. |
| `Preparation refuses a directory path that contains a retained path` | delta | Removed; replaced by `Preparation refuses a directory path that contains a Specification path`. |
| `Preparation refuses when neither replacement form is given` | delta | Removed; replaced by `A component rebuild refuses an empty path list`. |
| `Preparation refuses when both replacement forms are given` | delta | Removed; the two-flag conflict is retired. |
| `Preparation refuses an omitted target` | delta | Modified: `--full` command phrase → `rebuild-all`; `trial`/`receipt` absent nouns → `spec copy`/`manifest`. |
| `Preparation refuses a target inside the checkout` | delta | Modified: `prepares a trial` → `runs rebuild-all`; `trial directory`/`receipt` → `spec copy directory`/`manifest`. |
| `Preparation refuses an existing target directory` | delta | Removed; replaced by `Preparation refuses an existing spec copy directory`. |
| `Preparation refuses an existing target file` | delta | Removed; replaced by `Preparation refuses an existing spec copy file`. |
| `Preparation refuses a receipt inside the trial directory` | delta | Removed; replaced by `Preparation refuses a manifest inside the spec copy`. |
| `Preparation refuses a receipt inside the checkout` | delta | Removed; replaced by `Preparation refuses a manifest inside the checkout`. |
| `Preparation refuses a receipt that is an ancestor of the trial directory` | delta | Removed; replaced by `Preparation refuses a manifest that is an ancestor of the spec copy`. |
| `Preparation refuses a symlinked target that resolves inside the checkout` | delta | Removed; replaced by `Preparation refuses a spec copy target symlinked into the checkout`. |
| `Preparation refuses an existing receipt` | delta | Removed; replaced by `Preparation refuses an existing manifest`. |
| `Preparation refuses a repository link in the copy set` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a nested repository marker in the copy set` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation counts the files left in a replaced path's directory` | delta | Removed; replaced by `Preparation counts the files left in a removed path's directory`. |
| `Preparation expands a directory path to the tracked files beneath it` | delta | Modified: `Replaced:`/`Not replaced:` → `Source code removed:`/`Files beside removed paths:`; `trial` → `spec copy`. |
| `A full replacement removes every tracked file under the source roots` | delta | Removed; replaced by `A whole-product rebuild removes every tracked file under the source roots`. |
| `A full replacement counts a retained file under a source root as not replaced` | delta | Removed; replaced by `A whole-product rebuild keeps a Specification file under a source root`. |
| `Preparation collapses a repeated path` | delta | Modified: `Replaced: 1` → `Source code removed: 1`; `receipt records` → `manifest records one removed path`. |
| `Preparation does not expand a glob-shaped path` | delta | Modified: `Replaced: 0` → `Source code removed: 0`; `trial` → `spec copy`. |
| `Preparation reads the revision and leaves an ignored file out of the trial` | delta | Removed; replaced by `Preparation leaves an ignored file out of the spec copy`. |
| `Preparation preserves the executable bit and the symlink target` | delta | Modified: `trial` → `spec copy`; assertions unchanged. |
| `Preparation writes no version-control metadata into the trial` | delta | Removed; replaced by `Preparation writes no version-control metadata into the spec copy`. |
| `Preparation prints the trial record to stdout when no receipt path is given` | delta | Removed; replaced by `Preparation writes no manifest when no manifest path is given`. |
| `A dry run performs every read and writes no trial or record` | delta | Removed; replaced by `A dry run writes no spec copy and no manifest`. |
| `A named path absent from the revision removes nothing` | delta | Modified: `Replaced: 0` → `Source code removed: 0`; `trial` → `spec copy`. |
| `Preparation reads an explicit revision and reports it` | delta | Modified: adds `Backend: git`; `trial` → `spec copy`; `Revision:` assertion kept. |
| `Preparation refuses an absent revision` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses when a revision blob cannot be read` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses when the record cannot be written` | delta | Removed; replaced by `Preparation refuses when the manifest cannot be written`. |
| `Preparation reads a Fossil revision and preserves an executable file and a symlink` | delta | Modified: noun sweep `retained`/`kept source`/`replaced` → `Specification`/`Source code kept`/`removed`; `Backend: fossil` kept. |
| `The trial carries the declaration when no role names it` | delta | Removed; replaced by `The spec copy carries the declaration when no role names it`. |
| `The trial record lists the declaration in its retained set` | delta | Removed; replaced by `The manifest lists the declaration among the Specification files`. |
| `Preparation refuses a replacement path that names the declaration` | delta | Removed; replaced by `Preparation refuses a component path that names the declaration`. |
| `Preparation refuses a directory that contains the declaration` | delta | Modified: `names ... as retained` → `names ... as a Specification file`; command phrase. |
| `Preparation partitions with the declaration recorded at the selected revision` | delta | Removed; replaced by `Preparation partitions with the declaration at the selected revision`. |
| `Preparation refuses a revision that does not record the declaration` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; the Then adds `does not record`, so the blob-absent `returned no blob` scenario cannot satisfy it. |
| `Preparation refuses a malformed declaration at the selected revision` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `Preparation refuses a declaration that is not UTF-8` | delta | Modified: `preparation`/`trial`/`receipt` → `the command`/`spec copy`/`manifest`; assertions unchanged. |
| `A full replacement keeps the declaration under a source root` | delta | Removed; replaced by `A whole-product rebuild keeps the declaration under a source root`. |
| `A role that matches the declaration counts it once` | delta | Modified: `Retained: 2` → `Specification files: 2`; `trial` → `spec copy`. |
| `The trial record names the frozen role its trial omits` | delta | Removed; replaced by `The manifest lists the End-to-end tests it does not copy`. |
| `A declaration with no frozen package excludes no role` | delta | Removed; replaced by `A declaration with no End-to-end role copies every Specification file`. |
| `A frozen package root that covers the declaration keeps it in the trial` | delta | Removed; replaced by `An End-to-end role that covers the declaration keeps it among the Specification files`. |
| `Verifier re-checks a prepared trial and receives the frozen-input report` | delta | Removed; replaced by `Verifier compares the spec copy with the manifest`. |
| `Verification names a retained file changed in the trial` | delta | Removed; replaced by `Comparison names a changed Specification file in the copy`. |
| `Verification names an e2e package file changed in the trial` | delta | Removed; replaced by `Comparison refuses a file added under an End-to-end role`. |
| `Verification names an e2e package file changed in the checkout` | delta | Removed; replaced by `Comparison names an End-to-end test changed in the checkout`. |
| `Verification names an e2e package path the record does not list` | delta | Removed; replaced by `Comparison names an added End-to-end file in the checkout`. |
| `Verification names an e2e scenario changed in the checkout` | delta | Removed; replaced by `Comparison names an @e2e scenario changed in the checkout`. |
| `Verification names an e2e scenario the record does not list` | delta | Removed; replaced by `Comparison names an added @e2e scenario file in the checkout`. |
| `Verification reports no drift after a kept source file changes` | delta | Removed; replaced by `Comparison reports no difference after a kept Source code file is edited`. |
| `Verification names a recorded path missing from the trial` | delta | Removed; replaced by `Comparison names a missing Specification file in the copy`. |
| `Verification refuses while the ise addon is off` | delta | Removed; replaced by `Comparison refuses while the ise addon is off`. |
| `Verification refuses while architecture principles are off` | delta | Removed; replaced by `Comparison refuses while architecture principles are off`. |
| `Verification refuses a record it cannot read` | delta | Removed; replaced by `Comparison refuses an unreadable manifest`. |
| `Verification refuses a record with another schema` | delta | Removed; replaced by `Comparison refuses a manifest whose scope is not ise-manifest/v1`. |
| `Verification refuses a record without a replaced digest` | delta | Removed; replaced by `Comparison refuses a manifest with a malformed checksum`. |
| `Verification refuses a record with an unknown field` | delta | Removed; replaced by `Comparison refuses a manifest with an unknown field`. |
| `Verification refuses a record with a missing field` | delta | Removed; replaced by `Comparison refuses a manifest with a missing field`. |
| `Verification refuses a record placed inside the trial` | delta | Removed; replaced by `Comparison refuses a manifest inside the copy`. |
| `Verification refuses a record whose written path is not the trial` | delta | Removed; replaced by `Comparison refuses a manifest whose copy directory is not the copy`. |
| `Verification refuses a record whose backend differs from the selected backend` | delta | Removed; the prepare-time backend is no longer evidence; replaced by `Comparison refuses a manifest with an unknown field`. |
| `Verification names every drifting path in one run` | delta | Removed; replaced by `Comparison names every drifting path in one run`. |
| `Verification names a declaration changed in the trial` | delta | Removed; replaced by `Comparison names a changed Specification file in the copy`. |
| `Verification names a declaration missing from the trial` | delta | Removed; replaced by `Comparison names a missing Specification file in the copy`. |
| `Verification reports no drift after only the checkout's structure role changes` | delta | Removed; replaced by `Comparison reports a Specification changed after the copy was written`. |
| `Verification names the e2e package a checkout declaration's added root brings` | delta | Removed; replaced by `Comparison names an added End-to-end root in the checkout declaration`. |
| `Verification bounds the frozen paths it refuses` | delta | Removed; replaced by `Comparison refuses a file added under a declared Specification path`. |
| `Verification keeps a trial prepared before the package was excluded re-checkable` | delta | Removed; the `ise-trial/v1` shape is refused by name; replaced by `Comparison refuses a manifest whose scope is not ise-manifest/v1`. |
| `A frozen role that covers the declaration re-checks clean` | delta | Removed; replaced by `Verifier compares the spec copy with the manifest`. |
| `Served ise guidance states that the trial always carries the declaration` | delta | Modified: the served text says `The spec copy always carries the declaration itself` and `implicit Specification entry, not a sixth role`; the title stays. |
| `The ise instruction says declared retained paths survive archiving` | delta | Modified: the served text says `a declared Specification path` must survive `givn archive`; the title stays. |

## Retained-Content Effect

Four edited paths are matched by `givn/specification.yaml`, so a spec copy built from a revision holding this change records different per-path checksums for them, and a different `specification_checksum`, than one built from the revision before it: `docs/glossary.md` (`structure`), `docs/reference.md` (`contracts`), `docs/arc42/02-architecture-constraints.md` (`limits`), and `givn/specs/obtain-effective-guidance/usecase.md` (`behavior`). `docs/domain-model.md` is declared under `structure` and is unchanged. This change prepares no spec copy, so no manifest is written; the next copy records the new values.

The delta use case and feature files live under `givn/changes/strengthen-ise-specification/`, outside every declared role. Archive merges them into `givn/specs/declare-the-specification/` and `givn/specs/prepare-an-ise-test/`, both under the `behavior` role, so the next copy after the merge changes those paths' checksums and the per-feature scenario checksums as well.

`README.md`, `assets/instructions/**`, `docs/ideation/ise-rebuildability.md`, the arc42 chapters other than 02, the arc42 registers, and `docs/arc42/adr/**` are outside the declaration (`limits` names only `docs/arc42/02-architecture-constraints.md`), so editing them does not change the copy content. Once implemented, files added, removed, or renamed under the `source` roots `src/**` and `tests/**` change the Source code kept and removed sets and the counts; an edit to a kept file changes no checksum, because kept paths carry none.

The pinned canonical and richer fixtures commit to their own stated bytes, not to this repository's live declaration, content, or `specification_checksum`; each golden scenario builds its fixture project and never reads this repository.

## Risks

| ID | Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|---|
| R-160 | The pinned golden values change if the canonical rule or the fixture bytes change; a wrong re-pin can make a divergent implementation pass. | Medium | Medium | The fixture bytes are normative in the use case example; the design computed the values twice, once with the binary and once with an independent implementation of the stated rule; a rule change opens a new scope under D7 and re-derives the values. |
| R-161 | The command rename breaks an operator's script or habit that still calls `givn ise replace` or `givn ise verify`. | Medium | Low | The retired names exit 2 as unknown commands, proven by `The retired command names and flags exit 2`; the served instruction, reference, and ideation topic name only the new commands; the delta removes or modifies every token-bearing permanent scenario, and the five former step-definition survivors are renamed into the delta (D13). |
| R-162 | `compare` does not verify Source code kept; an agent can edit a kept neighbour, including its step definitions, and the comparison still exits 0. | Medium | Medium | The End-to-end (E2E) suite is the acceptance boundary, and the manifest records kept paths in `source_kept_files`, so a deletion is refused. **Accepted residual (operator, design review 2026-11-17):** a component-test-level edit that no frozen `@e2e` scenario exercises is not verified. |

The `Rule:`-nested selection limit is the existing debt D-9, extended with the scenario-checksum consequence. It is not a new risk record.

## Independent Second Implementation

The design computed every pinned value twice. The production run used the binary against the fixtures; the independent implementation read the same fixture bytes and feature text and applied the stated rule in Python.

Canonical fixture:

```text
executable content: ["Scenario",null,["e2e"],[["Given","Given","nothing",null,null]],[]]
per-path checksums:
  givn/specification.yaml 592befb4303513eda36dd47519487011745c9710e7a56703677e630045c52e12
  givn/specs/demo.feature 95b647b4f91f9ceb15887db09ecfa08caaff14d5fb5228b25cf1aeab2889a540
  docs/glossary.md 42383f180e9413e0a02ff8ed07df04fd639d9b7274f1a8932d518656616ab40d
  docs/reference.md 910604b25ba07632849d98cd3131ce26f016156a57082fdfce816d3a70714277
  docs/arc42/02-architecture-constraints.md 645638aef3715e807e8831dbb90b65b9b6183e6cbd1d8de4df16f7ab5abdd1c6
  tests/e2e/Cargo.toml 7fb79f1e5eaf0e42fd94ebcd1044ac2fb64412680ea8ebbd0fb4fb443a7aea18
scenario checksum:  791f7b34866c710ad541e06fb1d354d3ed4fedd7c46404465183d0a68d88006c
checksums.end_to_end:
  givn/specs/demo.feature 95b647b4f91f9ceb15887db09ecfa08caaff14d5fb5228b25cf1aeab2889a540
  tests/e2e/Cargo.toml 7fb79f1e5eaf0e42fd94ebcd1044ac2fb64412680ea8ebbd0fb4fb443a7aea18
end_to_end_files: givn/specs/demo.feature, tests/e2e/Cargo.toml
specification_checksum: 35444598f1c4238a9db8c2cdea5b5cf5391604a3608aebd6fe2e9f91f105b917
end_to_end_checksum:    00580162d8a019161c476e2772c5a85e7506fc0c254cbc28424299343b479b07
```

Richer fixture:

```text
plain executable content: ["Scenario",[["Given","Given","a background step",null,null]],["e2e"],[["Given","Given","a docstring step","line one\n  indented\nline four",null],["And","Given","a table step",null,[["name","value"],["pipe","a|b"],["slash","a\\b"],["new","a\nb"]]]],[]]
outline executable content: ["Scenario Outline",[["Given","Given","a background step",null,null]],["e2e"],[["Given","Given","a value <value>",null,null],["Then","Then","the value is <value>",null,null]],[[[],[["value"],["one"],["two"]]]]]
scenario checksum:  d5e995848ed1491e131b5f13504c3295e79f23646909ff5d4fb3b548d03e0520
feature path checksum: 1825ffb98301111d35a66915563151034930b56d8c7a6cd63c51f1c4d841713a
checksums.end_to_end:
  givn/specs/demo.feature 1825ffb98301111d35a66915563151034930b56d8c7a6cd63c51f1c4d841713a
  tests/e2e/Cargo.toml 7fb79f1e5eaf0e42fd94ebcd1044ac2fb64412680ea8ebbd0fb4fb443a7aea18
end_to_end_files: givn/specs/demo.feature, tests/e2e/Cargo.toml
specification_checksum: c1102713bbf0a24482eaf6e3b8ff5b787ffa32d9ff414dc1c9434cba7a192e0e
end_to_end_checksum:    2c16dfedf4203acf3a1d2f67340b4897d5e1c04c26a8d212fa95f1b8e0eb2844
```

The canonical per-path values, the canonical scenario checksum, and both canonical rollups were recomputed from the stated bytes and rule. The `checksums.end_to_end` map alone rolls up to `da67f5bcd40fe6e87d2ad95142ef132a61dd165f1abc510691fe23c8c5aa603e` (canonical) and `0e25d84777effc27ebd8d405651b6cc77447b87d0d2277fed85359ca7f5db723` (richer), and `checksums.scenarios` alone to `6a8b112df9ce56280eb197f2b391583542f608748e65be2e5a1b2a286eb11d29` (canonical) and `c7dad610d4b4eb9968640f1ced4fe8900d2b58a268a53d74c78cf4d8e9c9e138` (richer), so the asserted rollups and per-path rows together pin the union and the scenario-checksum precedence. The earlier `tests/e2e/Cargo.toml` row did not reproduce: it carried `5b8fe63f...`, while the stated bytes and rule give `7fb79f1e...`, and only that value yields the asserted `00580162...` rollup. The row is corrected, and the block above carries every per-path value so the next check does not depend on the design's prose. The richer feature's bytes are stated line for line in the use case; its path checksum and both rollups are re-derived from those bytes, and its scenario checksum is unchanged because the parsed executable content is unchanged. The richer feature still fixes the docstring's delimiter-adjacent line breaks, the `And` step-type inheritance, the table's Gherkin escapes, and the `Examples:` pair; `Preparation records the canonical checksums for a richer feature` asserts its pinned values, beside the canonical scenario. The retired `replaced_digest` is not re-derived: the manifest carries `removed_paths` and no removed checksum. The design-review repeats the independent computation from the stated rule before sign-off.
