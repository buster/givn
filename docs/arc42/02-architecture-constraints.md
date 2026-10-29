# 2. Architecture Constraints

These constraints describe the current checkout and its explicit product
boundaries. Rationale and relationships are maintained in the [ADR
register](adr/README.md).

## 2.1 Technical Constraints

| ID | Constraint | Current evidence |
|---|---|---|
| TC-1 | Production implementation is Rust. | `Cargo.toml`, `src/` |
| TC-2 | The default product unit is one native binary with embedded assets. | `src/config/embedded.rs`, Cargo package |
| TC-3 | The build contract is one feature-free binary: no Cargo features are declared and no model dependencies exist. | `Cargo.toml` |
| TC-4 | Project lifecycle/configuration data lives under visible `givn/`. | `src/cli/init.rs`, `src/state.rs` |
| TC-5 | Embedded defaults are fallback; project facets and overlays win. | `src/config/cascade.rs`, `src/config/overlay.rs` |
| TC-6 | Manifest/config parsing uses YAML through `serde_yaml_ng`. | `src/config/manifest.rs` |
| TC-7 | Gherkin parsing uses the `gherkin` crate. | `src/gherkin/` |
| TC-8 | CLI parsing uses `clap`; shell completions use `clap_complete`. | `src/main.rs` |
| TC-9 | Lifecycle state is filesystem-derived; there is no state database. | `src/state.rs` |
| TC-10 | Delta tags use the `@givn.*` namespace. | `src/gherkin/delta.rs`, `src/gherkin/merge.rs` |
| TC-11 | `givn lint` is static and never runs the project test suite. | `src/cli/lint.rs` |
| TC-13 | The dependency-age policy checks locked registry crates through `scripts/check-dep-age.sh`. | Script and project policy |
| TC-21 | Automatic managed upgrades and declared migrations require a version-controlled project selected at the project root and a clean checkout; they run on Git and Fossil with the same guarantees. | `src/scm.rs`, `src/cli/migration.rs`, migration E2E scenarios |
| TC-22 | Ignored generated paths are force-staged only through the calculated Givn-owned allowlist. | `src/cli/migration.rs`, ignored-path migration E2E fixture |
| TC-25 | `givn run` signal forwarding (SIGINT/SIGTERM) is unix-only via a direct `libc` dependency; on other platforms server products refuse to start at preflight while cli/library products work. | `Cargo.toml` `[target.'cfg(unix)'.dependencies]`, `src/cli/run.rs` |
| TC-26 | givn never derives run-declaration content from repository sources, build manifests, container files, or framework conventions; the declaration is author-owned configuration validated only by deterministic rules. | `src/cli/run_block.rs`, run-command feature specs |
| TC-27 | Shared commit attribution is checked into `.githooks/`; local Git activation preserves and chains an existing `commit-msg` hook, while attribution failures never block a commit. | `givn/changes/llm-attribution/design.md`, commit-attribution feature specs |
| TC-28 | Runtime attribution is process-scoped and optional; it is never persisted as model selection in `givn/config.yaml`. | OpenCode/Claude integrations and commit-attribution protocol |
| TC-29 | The OpenCode 2 attribution integration observes the selected provider and model per session through the plugin's session lifecycle, links each shell request to its issuing session, and stamps the environment when the shell is created; payload drift remains a best-effort compatibility risk that never blocks a shell. | `assets/agent-integrations/opencode/givn-attribution.js`, OpenCode plugin scenario |
| TC-30 | The Claude attribution integration resolves the latest assistant model from the PreToolUse payload's transcript before using process fallbacks, exports context for the complete Bash command, and remains non-blocking when transcript metadata is missing or unreadable. | `src/cli/attribution.rs`, Claude attribution scenario |
| TC-31 | Archive publication is forward-only and runs on the selected backend. Givn publishes the merged archive candidate revision before verification, records the pending/proven state in `givn/archive/<id>/verification.json`, and never rolls the permanent corpus back. Publication is witness-bounded: Git stages exactly the candidate paths, Fossil records all changes and commits the witnessed dirty set (ADR-072). | `src/scm.rs`, `src/fossil.rs`, `src/proof.rs`, `src/cli/archive.rs`, archive-proof feature specs |
| TC-32 | The givn project must be the checkout root (the Git worktree top level or the Fossil checkout root); the archive refuses a project nested inside a larger repository. A root inside an ancestor Fossil checkout carries no marker at the root and is refused as having no version-controlled project. | `src/scm.rs`, archive-proof nested-project scenario |
| TC-33 | Repository-state commands select the backend through one seam: a Git worktree selects Git and a Fossil checkout with the `fossil` executable available selects Fossil; both markers at the root, no version control, a nested Git project root, and a marker without its executable are refused by name before any mutation. No configuration key declares the backend. | `src/scm.rs`, `scm-backend-selection` fragment scenarios |
| TC-34 | The development record is local development data: it is append-only, written best-effort, never a gate, and kept out of repository history by Givn's own ignore entry (`# givn:begin:history` in the Git destinations; the entry `givn/history/*` in a Fossil checkout's managed list), independently of the `rules` policy. The recorder never writes a versioned file: for a Git worktree it re-asserts the block in `.git/info/exclude` on every write (through the existing local-exclude mechanism, `git rev-parse --git-path info/exclude`), for a Fossil checkout it writes the repository-level `ignore-glob` setting only while no managed file overrides it, and it uses `.gitignore` only when no repository exists yet (`givn init` in a bare directory, nothing tracked). When a Fossil checkout carries `.fossil-settings/ignore-glob`, the entry `givn/history/*` joins that file's managed entry list through the owning commands (`givn init`, `givn skills sync`, `givn upgrade`). Repository existence is decided by the backend markers at the project root, never by backend selection succeeding. Recording therefore cannot dirty a worktree and cannot affect `givn upgrade`'s clean-checkout precondition. The recorder engages only when the project root carries the Givn scaffold (`givn/`): before the scaffold exists it writes nothing and creates no directory, so a refused command in a bare root leaves that root untouched, while a scaffolded project records every command including refusals and failures (the rule keys on the scaffold, not on the VCS markers). Harness integrations report through the one intake command and never write the record themselves. The store stays one dependency-free JSONL file — no database engine is embedded in the binary — and that file is itself the interface external tools (jq, DuckDB, SQLite) read for ad-hoc analysis (ADR-073). | `src/history.rs`, `src/cli/history.rs`, `givn/history/events.jsonl` |
| TC-35 | The development report is one self-contained file: no server, no port, no remote resource, and no third-party JavaScript. It renders from the embedded record data alone, so its pixels are reproducible for a given record. | `src/cli/timeline.rs`, `assets/timeline/report.html` |
| TC-36 | The Maki target is a full agent target: it writes skills to `.maki/skills/<name>/SKILL.md` and commands to `.maki/commands/<name>.md` (Markdown with optional `---` frontmatter and `$ARGUMENTS`), and it also installs the development reporter — the Lua module `.maki/lua/givn-timeline.lua`, its grants merged into `.maki/plugin.toml`, and a managed `-- givn:begin:timeline` fence in `.maki/init.lua` (Maki runs only that file). The module path and the guidance files are managed generated paths; `init.lua` and `plugin.toml` are merged user-owned files, so only the fenced region and grant keys are touched. Maki loads nothing from `.maki/` in a folder the developer has not trusted, so the reporter is inactive there by the harness's own gate. | `assets/agent-integrations/maki/givn-timeline.lua`, `assets/agent-integrations/maki/plugin.toml`, `src/skills/targets.rs` |
| TC-37 | The record's capture control is one project key, `record.mode: on \| off \| full` in `givn/config.yaml`, defaulting to **`on`** when absent or unparseable (one warning names the fallback). The key is parsed leniently by `OverlayDoc` (`src/config/overlay.rs`), so an invalid value is a warning, never a config-parse failure, and `givn upgrade` preserves it. `off` is absolute: no append, no pending state, no record-side ignore write, and `givn history ingest --with-full-content` is ignored with one warning and exit 0 (reads still work); lowering the mode retains already-captured entries and never purges them. `on` records every available metadata field with the documented bounds (label 120, text metadata 240, error text 1 000 characters) and question text in full (4 000 characters per question entry), and `full` adds the complete LLM-facing content capped at 64 KiB per field and 256 KiB per entry. The intake is the single enforcement site for every bound and enforces the mode for payloads, while Claude's in-process `harness_hook` is the second `off` site, so a stale reporter cannot bloat or leak the store; `givn history settings [--json]` reports the resolved mode and its source (`config` or `default`). Content is stored inline in the same record line under `givn/history/`, so the existing managed ignore block covers it with no new destination. | `src/history.rs`, `src/cli/history.rs`, `src/config/overlay.rs`, `givn/config.yaml` |
| TC-39 | `givn check tasks --change <id>` requires a clean committed tree except evidence/verification paths and writes a change verification receipt bound to the tested revision, delta-feature digest, verify-command digest, tested-content digest, content scope `change-proof/v1`, and green counts. Review and archive refuse a missing or stale receipt before mutating the permanent corpus; they accept a receipt whose tested content is unchanged across a documentation-only revision. Schema 1 receipts keep strict revision equality. | `src/cli/check.rs`, `src/cli/archive.rs`, `src/proof.rs`, `src/scm.rs` |
| TC-40 | A checked scenario names one real, unique revision whose commit message body contains the scenario title. The commit must exist and be reachable from HEAD; missing, duplicate, placeholder, unreachable, and untitled revisions block check tasks and review. A shared revision is rejected. Plain-text CLI transcripts under the change evidence tree are declared evidence. | `src/cli/check.rs`, `src/scm.rs`, `src/git.rs`, `src/fossil.rs` |
| TC-41 | Development-record event fields stay additive at schema 1: a linked usage entry is marked `derived: true`, an answered question carries `selected`/`source`/`answered_at`, a failed command carries a bounded `error`, and `provider`/`variant` join known model identity. Absent values stay absent; `usage: unavailable` stays explicit. | `src/history.rs`, `src/cli/history.rs`, `src/main.rs` |
| TC-38 | The archive moves the active change into the archive location with one same-filesystem rename and no copy-and-delete fallback. A cross-filesystem or permission failure refuses by name and leaves the active change in place with the pending receipt, which is an exempt evidence path; a rerun re-runs the gates and retries the rename. | `src/cli/archive.rs`, `src/proof.rs`, archive-proof transition scenarios |

| TC-42 | The outline report supports production Rust and Python, is offline, deterministic, and advisory: `syn` is the first parser, a lazily started `python3` standard-library `ast` extractor is the second, `SourceAnalyzer` is the neutral port, the report prints each unit's per-component call tree first, then metrics and classification as hints, emits no action or recommendation at schema version 3, stores no baseline, and report hints never change the exit code. A Rust-only scope never starts `python3`. The outline addon is default-off and opt-in; `architecture_principles` is
another default-off addon. | `src/outline/`, `src/cli/outline.rs`, `src/cli/addons.rs`, outline-report non-E2E and E2E scenarios |
| TC-43 | The outline review gate runs only inside `givn check review --change <id>` and only while `addons.outline` is enabled. It is change-relative: the base is the parent of the first reachable commit that touched `givn/changes/<id>/` (Git first parent; Fossil `ancestors current`), the head is the committed `HEAD`, and a dirty production path fails before analysis. A missing first touch, a root commit's empty base, a shallow clone, and a merge/rebase edge each have one deterministic rule with no fallback to the working tree or a merge-base. Every touched unit with methods is printed as a per-component call tree and needs exactly one `OUTLINE: PASS <unit> \| <responsibility; call order>` or `OUTLINE: FAIL <unit> \| <reason>` from the review context recorded for the change; when the record names a kind, `AUTHOR` and `REVIEWER` must match the recorded `Author` and `Reviewer context` (a self-review accepts the self context; an independent review requires the recorded reviewer and a differing author; a legacy record keeps the reviewer-differs rule), a missing verdict, an empty PASS explanation, a FAIL, or conflicting verdicts block, and there is no justification path. Metric hints never block. It stores no baseline, watchlist, classification history, or feedback ledger and is deterministic and offline; its only writes are two temporary trees outside the project, removed before the command returns. History and revision files come from the existing SCM seam, so the gate runs no `git` or `fossil` command itself and branches on no backend. The gate adds no CLI command, flag, JSON schema, or archive gate. | `src/outline/gate/`, `src/cli/check.rs`, `src/scm.rs`, `src/git.rs`, `src/fossil.rs`, outline-review-gate scenarios |
| TC-44 | A change may record `Review kind: self-review` or `Review kind: independent` in `proposal.md`, `design-review.md`, and `review.md`, with `Author` and `Reviewer context` for both. A proposal recording `independent` accepts only an independent review record whose reviewer context differs from the author; a self-review is accepted only when no independent kind is recorded. Design review and final review share one independent reviewer context. A legacy record with no recorded kind keeps the previous gate result. The gate never infers independence from markers and never upgrades a self-review. | `src/review_context.rs`, `src/integrity_hook.rs`, `src/cli/archive.rs`, review-context scenarios |

## 2.2 Configuration and Gate Constraints

The effective manifest is assembled in this order:

1. Parse the embedded default manifest.
2. Apply `givn/config.yaml` as a project overlay.
3. If present, apply `givn/commands.yaml` as another manifest overlay. In the
   current project this file is the source for `verify.command`,
   `verify.e2e_command`, coverage command/path values, and the `run:` block
   declaring how the built product is started.

Artifact `post` lists are configurable additional hooks. They do not replace
the binary's mandatory behavior:

- `givn check tasks` runs WIP protection and `verify.command`.
- `givn check review` runs `integrity`, the run-declaration check, and, for a
  named change, the overlap disposition gate. Scenario execution is proven once
  per archive by the verification receipt, not re-run in review.
- Built-in hook names cannot be made best-effort with `allow_failure`.
- `givn check` without an artifact runs only configured `post` hooks; it does
  not call the artifact-specific mandatory path.
- `givn archive` runs configured hooks for `apply.requires` and then its own
  two-stage pipeline: it commits the merged archive candidate, runs each
  verification scope once with a fail-closed runner result, runs the optional
  coverage gate, and commits the verification receipt as the finalization.
  The run-declaration gate runs before the README completeness gate and
  independently of the readme addon.

The embedded manifest has empty `post` lists for the standard `tasks` and
`review` artifacts. Documentation in an older generated config may still show
the former hook defaults; the source code and effective manifest above are the
runtime contract.

## 2.3 Addon Constraint

The embedded manifest starts with addons disabled, but `givn init` currently
enables every default-on addon unless `--no-addons` is passed. The `arc42` addon:

- expands to a required `arc42-docs` artifact after `design`;
- adds `arc42-docs` to `apply.requires` and to `design-review` dependencies;
- scaffolds missing chapters at the fixed `docs/arc42/` path;
- does not implement `arc42.output_dir`, a chapter list, or a chapter-file
  cascade; and
- leaves existing Arc42 files when disabled.

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

The automatic migration route may write and commit only the config
scaffold, generated agent integrations, Givn-owned fenced blocks, and its own
migration plan. It may force-stage an ignored managed path only after the path
has been calculated explicitly, and only under the `versioned` rules policy;
under the default `local-only` policy the managed revision carries the scaffold,
the ignore-block changes, and the removals of tracked rules, and never stages
`AGENTS.md` or a generated rule. It must refuse a dirty or non-version-controlled
project before writing and must never force-stage a directory or the complete
worktree.
Domain specs, source code, ejected facets, durable Arc42 content, and
architecture decisions remain project-owned and are changed only through
LLM-guided project changes — the maintenance change or the follow-up changes
it names.

*[Architecture documentation index](README.md)*
