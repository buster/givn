# 2. Architecture Constraints

These constraints describe the current checkout and its explicit product
boundaries. Rationale and relationships are maintained in the [ADR
register](adr/README.md).

## 2.1 Technical Constraints

| ID | Constraint | Current evidence |
|---|---|---|
| TC-1 | Production implementation is Rust. | `Cargo.toml`, `src/` |
| TC-2 | The default product unit is one native binary with embedded assets. | `src/config/embedded.rs`, Cargo package |
| TC-3 | The default build has no retrieval model or ONNX dependency. | `Cargo.toml` default features are empty |
| TC-4 | Project lifecycle/configuration data lives under visible `givn/`. | `src/cli/init.rs`, `src/state.rs` |
| TC-5 | Embedded defaults are fallback; project facets and overlays win. | `src/config/cascade.rs`, `src/config/overlay.rs` |
| TC-6 | Manifest/config parsing uses YAML through `serde_yaml_ng`. | `src/config/manifest.rs` |
| TC-7 | Gherkin parsing uses the `gherkin` crate. | `src/gherkin/` |
| TC-8 | CLI parsing uses `clap`; shell completions use `clap_complete`. | `src/main.rs` |
| TC-9 | Lifecycle state is filesystem-derived; there is no state database. | `src/state.rs` |
| TC-10 | Delta tags use the `@givn.*` namespace. | `src/gherkin/delta.rs`, `src/gherkin/merge.rs` |
| TC-11 | `givn lint` is static and never runs the project test suite. | `src/cli/lint.rs` |
| TC-12 | Retrieval is opt-in through `spec-retrieval`; the E5 strict gate is independently selected by `spec-similarity-gate`. | `Cargo.toml`, `src/spec/retrieval.rs` |
| TC-13 | The dependency-age policy checks locked registry crates through `scripts/check-dep-age.sh`. | Script and project policy |
| TC-14 | E5 uses the binary-fixed similarity value `0.95` where the strict gate is compiled. | `src/spec/retrieval.rs` |
| TC-15 | BGE is optional local evidence; its operating point is `3.0`, not a Givn classification. | `src/spec/retrieval.rs` |
| TC-16 | NLI is optional local evidence selected independently from BGE. | `Cargo.toml`, `src/spec/retrieval.rs` |
| TC-17 | Complete semantic serialization is the E5 content-hash boundary. | `src/spec/retrieval.rs` |
| TC-18 | Actual model tokenizers preflight E5 passages and selected BGE/NLI pairs; `512` fits and `513` is over. | `src/spec/retrieval.rs` and token-cap features |
| TC-19 | Over-cap E5 passages have no vector; fitting passages continue indexing. | `SpecIndex.e5_token_caps` and refresh path |
| TC-20 | `GIVN_SPEC_MAX_RERANK_PAIRS` accepts positive decimal values and defaults to `2048` when unset. | Current PR #27 implementation; decision remains proposed |

## 2.2 Configuration and Gate Constraints

The effective manifest is assembled in this order:

1. Parse the embedded default manifest.
2. Apply `givn/config.yaml` as a project overlay.
3. If present, apply `givn/commands.yaml` as another manifest overlay. In the
   current project this file is the source for `verify.command`,
   `verify.e2e_command`, and coverage command/path values.

Artifact `post` lists are configurable additional hooks. They do not replace
the binary's mandatory behavior:

- `givn check tasks` runs WIP protection and `verify.command`.
- `givn check review` runs `verify.command`, `verify-e2e`, `integrity`, and,
  for a named change, overlap and retrieval checks.
- Built-in hook names cannot be made best-effort with `allow_failure`.
- `givn check` without an artifact runs only configured `post` hooks; it does
  not call the artifact-specific mandatory path.
- `givn archive` runs configured hooks for `apply.requires` and then its own
  archive stages, including post-merge verification.

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

## 2.4 Semantic Constraints

Second-stage candidate selection depends on the command surface:

| Surface | Candidate rule |
|---|---|
| Duplicate Audit | Deterministic exact/structural findings first; E5 semantic candidates use the full in-memory E5 comparison. BGE uses the endpoint top-five union; NLI uses the full E5 semantic set. |
| Explore | The command ranks permanent plus optional request-local candidates and retains at most five results before BGE/NLI selection. |
| Advisory review | The existing retrieval path builds its own delta/permanent and projected candidate pool; BGE and NLI operate independently over that pool. |
| Blocking semantic report | Active Added/Modified deltas are compared with permanent and other active delta endpoints at E5 `0.9500`; BGE/NLI receive those report candidates, not a universal top-five pool. |

All second-stage layers retain raw E5 identity and distinguish recommended,
filtered, unscored, token-cap, and unavailable states. They never classify or
edit feature files.

## 2.5 Acceptance Boundary

Regular and E2E tests use deterministic stubs and test-only token-budget seams.
Production retrieval uses local model artifacts. The current full and E2E
verification state is not inferred from this document; it is read from
`givn status --change semantic-reranker-budget` and the active change tasks.
