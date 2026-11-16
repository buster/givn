# 2. Architecture Constraints

These constraints describe the current checkout and its explicit product
boundaries. Rationale and relationships are maintained in the [ADR
register](adr/README.md).

## 2.1 Technical Constraints

| ID | Constraint | Current evidence |
|---|---|---|
| TC-1 | Production implementation is Rust. | `Cargo.toml`, `src/` |
| TC-2 | The default product package is one native binary with embedded assets. | `src/config/embedded.rs`, Cargo package |
| TC-3 | The build contract is one feature-free binary: no Cargo features are declared and no model dependencies exist. | `Cargo.toml` |
| TC-4 | Project lifecycle/configuration data lives under visible `givn/`. | `src/cli/init.rs`, `src/state.rs` |
| TC-5 | Embedded defaults are fallback; project facets and overlays win. | `src/config/cascade.rs`, `src/config/overlay.rs` |
| TC-6 | Manifest/config parsing uses YAML through `serde_yaml_ng`. | `src/config/manifest.rs` |
| TC-7 | Gherkin parsing uses the `gherkin` crate. | `src/gherkin/` |
| TC-8 | CLI parsing uses `clap`; shell completions use `clap_complete`. | `src/main.rs` |
| TC-9 | Lifecycle state is filesystem-derived; there is no state database. | `src/state.rs` |
| TC-10 | Delta tags use the `@givn.*` namespace. | `src/gherkin/delta.rs`, `src/gherkin/merge.rs` |
| TC-11 | `givn lint` is static and never runs the project test suite. | `src/cli/lint/mod.rs` |
| TC-12 | `givn/commands.yaml` is parsed with a dedicated schema for `verify`, `coverage`, and `run`; policy keys there and executable keys in `givn/config.yaml` are refused by name. | `src/config/overlay.rs` |
| TC-13 | The dependency-age policy checks locked registry crates through `scripts/check-dep-age.sh`. | Script and project policy |
| TC-14 | Every registered addon except an Opt-in addon is enabled by default; a missing key in a partial `addons:` block resolves that addon's declared default, and only an explicit `false` disables a default-on addon. | `src/config/manifest.rs`, `src/cli/addons.rs` |
| TC-15 | Generated guidance is skills-only. No command files are generated; recognizable old command files are removed, custom files preserved with a notice. | `src/skills/targets.rs` |
| TC-16 | The selected agent targets are `opencode`, `agents`, `claude`, `omp`, and `maki`; each receives `<dir>/skills/<name>/SKILL.md`. Invocation is harness-owned. | `src/skills/targets.rs` |
| TC-17 | `givn commit` requires explicit paths and uses selected-path publication; it never stages every pending change. | `src/cli/commit.rs`, `src/git.rs`, `src/fossil.rs` |
| TC-18 | Commit attribution is optional and non-blocking. Git may stamp raw commits through the checked-in hook; a raw Fossil check-in has no attribution. | `src/cli/attribution.rs`, `src/cli/git.rs` |
| TC-19 | The outline report is advisory. It has no review/archive gate, verdict vocabulary, waiver, baseline, or forced reviewer context. | `src/cli/outline.rs`, `src/outline/` |
| TC-20 | The shipped migration catalog is empty at the 1.0 baseline; no embedded 0.x route or alias exists. The resolver and bundle-validation contract remain for future routes. | `assets/migrations/catalog.yaml`, `src/cli/migration.rs` |
| TC-21 | Automatic managed upgrades and declared future migrations require a version-controlled project selected at the project root and a clean checkout; they run on Git and Fossil with the same guarantees. | `src/scm.rs`, `src/cli/migration.rs`, migration E2E scenarios |
| TC-22 | Ignored generated paths are force-staged only through the calculated Givn-owned allowlist. | `src/cli/migration.rs`, ignored-path migration E2E fixture |
| TC-25 | `givn run` signal forwarding (SIGINT/SIGTERM) is unix-only via a direct `libc` dependency; on other platforms server products refuse to start at preflight while cli/library products work. | `Cargo.toml` `[target.'cfg(unix)'.dependencies]`, `src/cli/run.rs` |
| TC-26 | givn never derives run-declaration content from repository sources, build manifests, container files, or framework conventions; the declaration is author-owned configuration validated only by deterministic rules. | `src/cli/run_block.rs`, run-command feature specs |
| TC-27 | Shared commit attribution is checked into `.githooks/`; local Git activation preserves and chains an existing `commit-msg` hook, while attribution failures never block a commit. | `src/cli/git.rs`, `assets/hooks/`, commit-attribution feature specs |
| TC-28 | Runtime attribution is process-scoped and optional; it is never persisted as model selection in `givn/config.yaml`. | OpenCode/Claude/omp/Maki integrations and commit-attribution protocol |
| TC-29 | The OpenCode 2 attribution integration observes the selected provider and model per session through the plugin's session lifecycle, links each shell request to its issuing session, and stamps the environment when the shell is created; payload drift remains a best-effort compatibility risk that never blocks a shell. | `assets/agent-integrations/opencode/givn-attribution.js`, OpenCode plugin scenario |
| TC-30 | The Claude attribution integration resolves the latest assistant model from the PreToolUse payload's transcript before using process fallbacks, exports context for the complete Bash command, and remains non-blocking when transcript metadata is missing or unreadable. | `src/cli/attribution.rs`, Claude attribution scenario |
| TC-31 | Archive publication is forward-only and runs on the selected backend. Givn publishes the merged archive candidate revision before verification, records the pending/proven state in `givn/archive/<id>/verification.json`, and never rolls the permanent corpus back. Publication is witness-bounded: Git stages exactly the candidate paths, Fossil records all changes and commits the witnessed dirty set (ADR-072). | `src/scm.rs`, `src/fossil.rs`, `src/proof.rs`, `src/cli/archive.rs`, archive-proof feature specs |
| TC-32 | The givn project must be the checkout root (the Git worktree top level or the Fossil checkout root); the archive refuses a project nested inside a larger repository. | `src/scm.rs`, archive-proof nested-project scenario |
| TC-33 | Repository-state commands select the backend through one seam: a Git worktree selects Git and a Fossil checkout with the `fossil` executable available selects Fossil; both markers at the root, no version control, a nested Git project root, and a marker without its executable are refused by name before any mutation. No configuration key declares the backend. | `src/scm.rs`, `scm-backend-selection` fragment scenarios |
| TC-34 | The development record is local development data: best-effort, append-only, scope-tagged, never a gate, and kept out of repository history by Givn's own ignore entries. The recorder never writes a versioned file; it re-asserts `.git/info/exclude` for Git and uses the repository-level Fossil `ignore-glob` only while no managed file overrides it. The owning commands (`givn init`, `givn skills sync`, `givn upgrade`) write the tracked `.gitignore` block for Git and the managed `givn/history/*` entry for Fossil. | `src/history.rs`, `src/skills/rules.rs`, `src/cli/history.rs`, `givn/history/events.jsonl` |
| TC-35 | The development report is one self-contained file: no server, no port, no remote resource, and no third-party JavaScript. | `src/cli/timeline.rs`, `assets/timeline/report.html` |
| TC-36 | The Maki target is a full skills target and receives the development reporter and commit-attribution integration: the Lua modules `.maki/lua/givn-timeline.lua` and `.maki/lua/givn-attribution.lua`, grants merged into `.maki/plugin.toml` (including the Maki-required `session_write`), and managed `-- givn:begin:timeline` and `-- givn:begin:attribution` fences in `.maki/init.lua`. It receives no generated command files. Maki loads nothing from `.maki/` in an untrusted folder. | `assets/agent-integrations/maki/`, `src/skills/targets.rs` |
| TC-37 | The record's capture control is one project key, `record.mode: on \| off \| full`, defaulting to `on` when absent or unparseable. `off` is absolute; `on` records bounded metadata and full question text; `full` adds complete LLM-facing content under 64 KiB per field / 256 KiB per entry. The intake enforces the mode and bounds; lowering the mode never purges. The record is not privacy-filtered. | `src/history.rs`, `src/cli/history.rs`, `src/config/overlay.rs` |
| TC-38 | The archive moves the active change into the archive location with one same-filesystem rename and no copy-and-delete fallback. A cross-filesystem or permission failure refuses by name and leaves the active change in place with the pending receipt. | `src/cli/archive.rs`, `src/proof.rs`, archive-proof transition scenarios |
| TC-39 | `givn check tasks --change <id>` requires a clean committed tree except evidence/verification paths and writes a change verification receipt bound to the tested revision, delta-feature digest, verify-command digest, tested-content digest, content scope `change-proof/v1`, and green counts. With `ise` effective for the change's locked add-on set it also binds the declared `e2e` package digests, the E2E runner command digest, the running Givn version, and the E2E scope outcome. Review and archive refuse a missing or stale receipt; a documentation-only revision may reuse an unchanged proof. | `src/cli/check.rs`, `src/cli/archive.rs`, `src/proof.rs`, `src/e2e_boundary/`, `src/config/lock.rs`, `src/scm.rs` |
| TC-40 | A checked scenario names one real, unique revision reachable from `HEAD`; a shared batch revision is valid when the `COMMIT:` line names its batch. Missing, duplicate, placeholder, unreachable, untitled, and unbatch-labelled shared revisions block. | `src/cli/check.rs`, `src/scm.rs`, `src/git.rs`, `src/fossil.rs` |
| TC-41 | Development-record event fields stay additive at schema 1: derived usage is marked, answered questions carry selection/source/time, failed commands carry bounded error text, and provider/variant join model identity. Absent values stay absent; `usage: unavailable` stays explicit. | `src/history.rs`, `src/cli/history.rs`, `src/main.rs` |
| TC-42 | The outline report supports production Rust, Python, Java, TypeScript, and JavaScript, is offline, deterministic, and advisory. `syn` parses Rust; a lazily started `python3` standard-library `ast` extractor parses Python; bundled Tree-sitter Java, TypeScript/TSX, and JavaScript grammars parse the added languages; `SourceAnalyzer` is the neutral port. The report prints each package's per-component call tree first, then metric hints; it emits no action, recommendation, verdict, or waiver, stores no baseline, and never changes the exit code for a hint. A Rust-only scope never starts `python3`. | `src/outline/`, `src/cli/outline.rs`, outline-report scenarios |
| TC-43 | Outline metrics are optional: an unavailable measurement is `unavailable` in text and `null` in JSON, never zero. LCC is not implemented and is omitted rather than substituted by TCC. | `src/outline/report.rs`, `src/outline/metrics.rs`, outline-report scenarios |
| TC-44 | The review context record (`Review kind`, `Author`, `Reviewer context`) is mandatory when a proposal requests independent review; a self-review is never upgraded by inference. This is independent of the retired outline gate. | `src/review_context.rs`, `src/integrity_hook.rs` |
| TC-45 | With `ise` effective for the change's locked add-on set, `givn check tasks` requires the declared `e2e` role to resolve at least one package, each readable on its own manifest and disjoint from every `source` root both ways, and `verify.e2e_command` to name every resolved root; a path that resolves no package is checked by its path-resolving references. It runs the change-scoped E2E execution when the delta has `@e2e` scenarios. The advisory placement findings stay advisory and veto nothing. | `src/e2e_boundary/`, `src/cli/check.rs`, `src/cli/lint/specification.rs` |
| TC-46 | `givn ise replace` reads one selected revision (`HEAD` when omitted) through the SCM seam, requires exactly one of `--full` (every tracked file under every `source:` root) or repeated `--path` (a tracked file, or a directory expanded to every tracked file beneath it with the retained-role check over the expansion), partitions the tracked paths into the non-frozen retained roles, kept source, replaced paths, the frozen `e2e` package, and outside-the-declaration paths, and refuses nine named conditions before any write. It writes a plain trial directory outside the checkout with no version-control metadata and no ignored or untracked file, preserves relative paths, the executable bit, and symlink targets, re-reads every written file against the revision, and records the retained, `e2e`, and replaced identities. The checkout is never written, including the receipt. | `src/cli/ise.rs`, `src/ise/`, `src/scm.rs`, `src/e2e_boundary/`, `src/gherkin/` |
| TC-47 | `givn ise verify` refuses unless the record's `written_path` is the given trial directory, its `backend` is the selected backend, and the record path lies outside the trial; it then re-checks the recorded retained files against the written trial and the recorded `e2e` package and scenario identities against the current checkout, names every drifting path, and exits nonzero on drift. A record whose `excluded_roles` names `e2e` is refused before any comparison when the trial holds a path the recorded `e2e` set or the checkout's resolved `e2e` roots cover, except `givn/specification.yaml` and every path the record's `retained_files` lists, naming at most twenty such paths and the remainder count. Both clean-room commands are bound to `addons.ise` and its `architecture_principles` prerequisite, refuse with either off, and use the effective project configuration rather than a change lock. Git and Fossil expose one `RevisionEntry` shape: Git derives kind and executable from `ls-tree` modes, Fossil from the check-in manifest's `F` lines, with `\s` decoded in a path and a 40-hex or 64-hex blob hash accepted. | `src/cli/ise.rs`, `src/ise/`, `src/scm.rs`, `src/git.rs`, `src/fossil.rs` |

| TC-48 | The four core step instructions (`specs`, `steps`, `design`, `review`) carry the universal rules and name each stack instruction id with the load condition that selects it, and carry no per-runner body. A stack instruction (`steps-rust`, `steps-python`, `steps-node`, `steps-java`, `steps-go`, `steps-ruby`) is a policy instruction id resolved through the same cascade with no change context, overridable per id, and its body reaches the caller only on a request for that id. Instruction length carries no numeric cap; prose that adds no rule is the length defect, and a rule that states valid behavior is not removed or split to satisfy a line count. | `assets/instructions/`, `src/cli/instructions.rs`, `tests/common/instruction_references.rs`, `instruction-integrity` composition and reference scenarios |
| TC-49 | `givn ise replace` retains `givn/specification.yaml` as an implicit retained entry beside the five declared roles, counted once in the union and re-checked by `givn ise verify` like every recorded retained path. The partition reads, and the trial carries, the declaration recorded at the selected revision; a revision that does not record it refuses by path and revision, a malformed one refuses with the loader reason, and a replacement that names it or a directory containing it refuses. `Retained` equals `retained_files.len()`, and no record field or content scope changes. | `src/ise/partition.rs`, `src/ise/mod.rs`, `src/ise/verify.rs`, `src/config/declaration.rs`, `prepare-an-ise-test` scenarios |
| TC-50 | The trial record carries one delivery identity, written by `givn ise deliver`, over every regular file and symlink under the trial directory that the recorded `retained_files` does not list; the two identities partition the trial directory. `replaced_digest` stays the trial-input identity and no field is renamed, so the block adds no schema version, a record without it keeps its meaning, and an older reader refuses a delivery record by the unknown field instead of ignoring it. A seal requires clean frozen inputs and is recorded once, in a new file outside the checkout and the trial. | `src/ise/delivery.rs`, `src/ise/record.rs`, `src/ise/verify.rs`, `src/ise/mod.rs`, `prepare-an-ise-test` scenarios |
| TC-51 | The frozen set is every tracked file of the `e2e` role's resolved package at the selected revision except `givn/specification.yaml`. `givn ise replace` never copies it, counts it in no placement bucket, and reports it on one `Frozen package: <role>, not copied (<n> files)` line printed only when the set is non-empty; the count is the frozen set's size, not `e2e_files.len()`, which may include `givn/specification.yaml`. The record keeps `ise-trial/v1` and gains `excluded_roles`, which names `e2e` when the set is non-empty; `Retained` equals `retained_files.len()` and counts no frozen path. A record written before the rule defaults to an empty list and keeps its meaning, so its trial's frozen files stay retained entries and keep re-checking; an earlier reader refuses a record carrying the field. | `src/ise/partition.rs`, `src/ise/mod.rs`, `src/ise/record.rs`, `src/ise/verify.rs`, `src/cli/ise.rs`, `src/e2e_boundary/`, `prepare-an-ise-test` scenarios |

## 2.2 Configuration and Gate Constraints

The effective manifest is assembled in this order:

1. Parse the embedded default manifest.
2. Apply `givn/config.yaml` as the policy/graph overlay.
3. Apply `givn/commands.yaml` through its dedicated `verify`/`coverage`/`run`
   schema.

Artifact `post` lists are configurable additional hooks. They do not replace
the binary's mandatory behavior:

- `givn check tasks` runs WIP protection and change verification.
- `givn check review` runs integrity, the run-declaration check, change-scoped
  lint, overlap, retirement, routing, the receipt, and scenario-commit checks.
- Built-in hook names cannot be made best-effort with `allow_failure`.
- `givn check` without an artifact executes nothing.
- `givn archive` runs configured hooks for `apply.requires` and then its own
  candidate, verification, coverage, and finalization stages.

The embedded manifest has empty `post` lists for the standard `tasks` and
`review` artifacts. Documentation in an older generated config may still show
former hook defaults; the source code and effective manifest are the runtime
contract.

## 2.3 Addon Constraint

Every registered addon except an Opt-in addon is enabled by default; `ise` is
the one Opt-in addon. The `arc42` addon:

- expands to a required `arc42-docs` artifact after `design`;
- adds `arc42-docs` to `apply.requires` and to `design-review` dependencies;
- scaffolds missing chapters at the fixed `docs/arc42/` path;
- does not implement `arc42.output_dir`, a chapter list, or a chapter-file
  cascade; and
- leaves existing Arc42 files when disabled.

The `outline` addon is advisory and adds no gate. The `communication_policy`
addon controls only a presentation and documentation block; the dialog
obligation is independent.
The `challenge` addon supplies a skill and instruction, not a lifecycle gate.

## 2.5 Acceptance Boundary

Regular and E2E tests invoke the compiled CLI in temporary projects and run as a
single feature-free pass; coverage still produces distinct non-E2E and E2E
reports. The current verification state is not inferred from this document; it
is read from `givn status` and the active change tasks.

## 2.6 Architecture Documentation Constraints

The architecture register is selective. The complete qualification procedure,
including its mandatory dimensions and verdict schema, is owned by the resolved
Arc42 addon instruction; this chapter does not duplicate it. One ADR records
one independent durable boundary or trade-off. Refinements amend the governing
ADR instead of creating parallel records. Observable behavior is owned by
Gherkin specifications, current architecture facts by the relevant Arc42
chapter, implementation details by code/design, and process rules by their own
guidance. The qualification policy is authoring guidance and is not inferred or
enforced by the runtime.

## 2.7 Migration Ownership Constraint

The managed upgrade may write and commit only the config scaffold, generated
agent integrations, Givn-owned fenced blocks, and its own allowlisted paths. It
may force-stage an ignored managed path only after the path has been calculated
explicitly, and only under the `versioned` rules policy; under the default
`local-only` policy the managed revision carries the scaffold, the ignore-block
changes, and the removals of tracked rules, and never stages `AGENTS.md` or a
generated rule. It must refuse a dirty or non-version-controlled project before
writing and must never force-stage a directory or the complete worktree.

The 1.0 baseline ships no pre-1.0 route. Retired configuration requires manual
repair; Givn names the replacement and refuses before writing. Future routes are
release-owned bundles resolved by the same catalog contract. Domain specs,
source code, ejected facets, durable Arc42 content, and architecture decisions
remain project-owned and are changed only through normal project changes.

*[Architecture documentation index](README.md)*
