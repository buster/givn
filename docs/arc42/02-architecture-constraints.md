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
enables all known addons unless `--no-addons` is passed. The `arc42` addon:

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
