# Architecture Decision Register

This directory is the Arc42 chapter 9 register. One architecturally significant
decision is one ADR file. The active register below is numeric, preserves the
historical identifiers, and contains only current or explicitly proposed
architecture records. Historical records are retained in [`archive/`](archive/).

## Current Contract

Only ADRs with `status: accepted` and no supersession relationship are the
current architecture contract. `proposed` records describe work in progress and
are not accepted constraints. `deprecated` and `superseded by ADR-NNNN` records
are historical context only.

## Qualification Rule

The complete qualification procedure, LLM verdict schema, routing values, and
MADR template are owned by the resolved Arc42 addon instruction. Run
`givn instructions arc42-docs --change <id>` before drafting.

This register records status, relationships, and links; it does not duplicate
the procedure. A chapter being affected does not by itself require a new ADR.
Observable behavior belongs in Gherkin, current architecture facts in Arc42,
implementation details in code/design, and process/project rules in their
owning documents.

## Active Register

The active register contains only independent durable architecture decisions:
25 accepted records. The fifteen semantic-engine records were
superseded by [ADR-069](ADR-069-single-feature-free-binary.md) and moved to
[`archive/`](archive/) by the `drop-embedding-features` change.

| ID | Status | Decision | Qualification basis |
|---|---|---|---|
| [ADR-001](ADR-001-standalone-tool.md) | accepted | Build givn as a standalone tool | hard to reverse; structural product boundary |
| [ADR-002](ADR-002-rust-implementation.md) | accepted | Implement the CLI in Rust | hard to reverse; quality and structural constraints |
| [ADR-004](ADR-004-single-static-binary.md) | accepted | Ship one native binary without runtime prerequisites | installability and deployment boundary |
| [ADR-005](ADR-005-facet-cascade.md) | accepted | Resolve project facets through a cascade | structural precedence; upgradeability |
| [ADR-008](ADR-008-visible-givn-directory.md) | accepted | Keep configuration in visible `givn/` | structural filesystem boundary |
| [ADR-009](ADR-009-yaml-manifest.md) | accepted | Use YAML for the artifact manifest | durable configuration and graph contract |
| [ADR-010](ADR-010-manifest-overlay.md) | accepted | Customize the manifest through an overlay | structural workflow graph; upgradeability |
| [ADR-011](ADR-011-feature-is-spec.md) | accepted | Make `.feature` the specification | durable executable contract |
| [ADR-013](ADR-013-agent-integration.md) | accepted | Generate skills and preserve user-owned `AGENTS.md` content | agent integration ownership boundary |
| [ADR-014](ADR-014-filesystem-derived-state.md) | accepted | Derive lifecycle state from the filesystem | lifecycle state boundary |
| [ADR-015](ADR-015-manifest-test-runner.md) | accepted | Declare the test runner explicitly | public runner contract |
| [ADR-017](ADR-017-openspec-clean-break.md) | accepted | Keep a clean break from OpenSpec | parser and project-layout boundary |
| [ADR-020](ADR-020-givn-delta-namespace.md) | accepted | Use the `@givn.*` delta namespace | merge and parser contract |
| [ADR-023](ADR-023-versioned-config-upgrade.md) | accepted | Preserve overrides during versioned upgrade | upgrade safety boundary |
| [ADR-024](ADR-024-configurable-artifact-graph.md) | accepted | Represent workflow as a configurable artifact graph | lifecycle topology |
| [ADR-026](ADR-026-arc42-addon.md) | accepted | Provide durable Arc42 documentation as an addon | addon and documentation lifecycle boundary |
| [ADR-027](ADR-027-lint-command-name.md) | accepted | Name the static check `givn lint` | public CLI contract |
| [ADR-028](ADR-028-hooks-and-mandatory-checks.md) | accepted | Separate post hooks from mandatory checks | non-deactivatable gate boundary |
| [ADR-037](ADR-037-deterministic-overlap-fingerprint.md) | accepted | Use deterministic fingerprints for structural overlap | review evidence boundary |
| [ADR-048](ADR-048-instruction-json-contract.md) | accepted | Keep effective policy and instruction metadata explicit | agent interface contract |
| [ADR-067](ADR-067-givn-declares-and-executes-the-product-start.md) | accepted | Declare and execute the product start in the same tool | start-command contract boundary |
| [ADR-069](ADR-069-single-feature-free-binary.md) | accepted | Ship one feature-free binary without the semantic retrieval engine | build-contract and dependency boundary |
| [ADR-070](ADR-070-forward-only-archive-proof.md) | accepted | Publish archives forward-only and bind proof to a committed candidate | publication boundary and durable receipt contract |
| [ADR-071](ADR-071-scm-backend-selection.md) | accepted | Select the SCM backend from project markers through one seam | component boundary and backend-selection contract |
| [ADR-072](ADR-072-witness-bounded-publication.md) | accepted | Publish revisions through the SCM seam with witness-bounded commits | publication contract and candidate-exactness invariant |

## Relationships

| Relationship | Meaning |
|---|---|
| ADR-019 -> ADR-027 | ADR-027 supersedes the old `givn verify` naming and carries the final lint contract. |
| ADR-039, 041, 044, 045, 046, 047, 052, 054, 055-061, 063, 064, 066, 068 -> ADR-069 | ADR-069 supersedes the semantic-engine records; the deterministic overlap boundary of ADR-037 remains current. |
| ADR-070 -> ADR-072 | ADR-072 generalizes ADR-070's Git-specific publication consequence onto the backend seam; ADR-070 keeps the forward-only proof decision. |
| ADR-071 (amended by maintain-project) | The seam decision now also governs the managed upgrade and migration bundles on Fossil: allowlist staging, pending-change detection, and commit run on both backends. |
| ADR-023 (amended by 0.3.0) | The versioned-config decision now also governs managed-only commits, catalog-selected migrations, aggregate plans, and their clean-worktree boundary. |

## Evidence

| Area | Current checkout evidence |
|---|---|
| Manifest and addons | [`src/config/overlay.rs`](../../../src/config/overlay.rs), [`src/config/manifest.rs`](../../../src/config/manifest.rs), [`src/cli/init.rs`](../../../src/cli/init.rs), [`givn/commands.yaml`](../../../givn/commands.yaml) |
| Hooks and mandatory checks | [`src/cli/check.rs`](../../../src/cli/check.rs), [`src/hooks.rs`](../../../src/hooks.rs), [`src/cli/archive.rs`](../../../src/cli/archive.rs) |
| Archive proof | [`src/scm.rs`](../../../src/scm.rs), [`src/fossil.rs`](../../../src/fossil.rs), [`src/proof.rs`](../../../src/proof.rs), [`src/cli/archive.rs`](../../../src/cli/archive.rs), [`givn/archive/archive-proof/`](../../../givn/archive/archive-proof/) |
| SCM backend selection | [`src/scm.rs`](../../../src/scm.rs), [`src/cli/migration.rs`](../../../src/cli/migration.rs), [`givn/specs/fragments/scm-backend-selection.feature`](../../../givn/specs/fragments/scm-backend-selection.feature) |
| Agent ownership | [`src/skills/targets.rs`](../../../src/skills/targets.rs), [`src/cli/agents_block.rs`](../../../src/cli/agents_block.rs), [`src/cli/absolute_mode.rs`](../../../src/cli/absolute_mode.rs) |
| Release migration | [`src/cli/migration.rs`](../../../src/cli/migration.rs), [`assets/migrations/catalog.yaml`](../../../assets/migrations/catalog.yaml), [`docs/migrations/0.2.0-to-0.3.0.md`](../../migrations/0.2.0-to-0.3.0.md), and release migration feature scenarios |
| PR #27 measurements | [`semantic-duplicates-real-model-analysis.md`](../../archive/analysis/semantic-duplicates-real-model-analysis.md) and its linked benchmark/manual reports; raw JSON reports are local and ephemeral under the paths stated there. |

## Archived ADRs

The archive contains only genuinely architectural decisions that were
superseded by an explicit replacement. Non-ADR records are not retained there.

| ID | Status | Reason for archive |
|---|---|---|
| [ADR-019](archive/ADR-019-lint-contract.md) | superseded by ADR-027 | Original static-check record. |
| [ADR-042](archive/ADR-042-bge-experiment.md) | superseded by ADR-052 | Experimental BGE gate interpretation. |
| [ADR-053](archive/ADR-053-bge-pair-cap.md) | superseded by ADR-063 | Fixed pair-cap refusal. |
| [ADR-039](archive/ADR-039-local-retrieval.md) | superseded by ADR-069 | Local E5 retrieval engine. |
| [ADR-044](archive/ADR-044-current-change-explore.md) | superseded by ADR-069 | Request-local Explore corpus for the removed command. |
| [ADR-045](archive/ADR-045-project-relative-identity.md) | superseded by ADR-069 | Retrieval identity for the removed index. |
| [ADR-054](archive/ADR-054-versioned-evidence-output.md) | superseded by ADR-069 | E5/BGE/NLI output schemas for the removed engine. |
| [ADR-041](archive/ADR-041-e5-uniqueness-gate.md) | superseded by ADR-069 | E5 uniqueness gate. |
| [ADR-046](archive/ADR-046-semantic-review-report.md) | superseded by ADR-069 | Blocking semantic worklist report. |
| [ADR-047](archive/ADR-047-projected-remediation.md) | superseded by ADR-069 | Semantic remediation against the projected tree. |
| [ADR-052](archive/ADR-052-bge-advisory-evidence.md) | superseded by ADR-069 | BGE advisory evidence. |
| [ADR-055](archive/ADR-055-independent-semantic-features.md) | superseded by ADR-069 | Independent semantic Cargo features. |
| [ADR-056](archive/ADR-056-bge-approval.md) | superseded by ADR-069 | Benchmark approval for default BGE. |
| [ADR-057](archive/ADR-057-bge-local-artifacts.md) | superseded by ADR-069 | BGE local-artifact loader. |
| [ADR-058](archive/ADR-058-nli-evidence.md) | superseded by ADR-069 | NLI admission evidence. |
| [ADR-059](archive/ADR-059-complete-semantic-serialization.md) | superseded by ADR-069 | Complete semantic serialization and E5 hash. |
| [ADR-060](archive/ADR-060-actual-tokenizer-caps.md) | superseded by ADR-069 | Tokenizer preflight limits. |
| [ADR-061](archive/ADR-061-surface-specific-independent-pools.md) | superseded by ADR-069 | Surface-specific scoring pools. |
| [ADR-063](archive/ADR-063-reranker-pair-budget.md) | superseded by ADR-069 | Second-stage pair budget. |
| [ADR-064](archive/ADR-064-shared-model-thread-budget.md) | superseded by ADR-069 | Shared model thread budget. |
| [ADR-066](archive/ADR-066-unified-semantic-rulebook.md) | superseded by ADR-069 | Unified semantic rulebook. |
| [ADR-068](archive/ADR-068-remediation-log.md) | superseded by ADR-069 | Append-only remediation log. |

## Reserved IDs

`ADR-050` and `ADR-051` are reserved and currently unassigned. No decision,
placeholder file, or inferred historical record is attached to either ID.

## ID History And Migration

The original register assigned `ADR-029` to the design-constraints decision and
`ADR-030` to the design-review gate. A later coverage change reused both numbers
for unrelated decisions. The current migration applies the strict qualification
gate to both records: the design-constraints and design-review material has one
canonical process/documentation home, and the later coverage records have no
architecture value. Their ADR files and index entries are removed rather than
preserved as duplicate historical records. `ADR-050` and `ADR-051` remain
reserved and unassigned.
