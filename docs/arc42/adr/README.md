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

`ADR-063` is deliberately still `proposed`: the current checkout's
`givn status --change semantic-reranker-budget` reports open implementation and
final-verification tasks. Its implementation is present in the checkout, but
the decision is not presented as completed or accepted until that change passes
its remaining gates.

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
35 accepted records and the explicitly proposed `ADR-063` owned by the open
`semantic-reranker-budget` change.

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
| [ADR-039](ADR-039-local-retrieval.md) | accepted | Provide local E5 scenario retrieval | model/index subsystem boundary |
| [ADR-041](ADR-041-e5-uniqueness-gate.md) | accepted | Enforce a binary-fixed E5 uniqueness boundary | semantic review quality gate |
| [ADR-044](ADR-044-current-change-explore.md) | accepted | Add active changes request-locally to Explore | persistence boundary |
| [ADR-045](ADR-045-project-relative-identity.md) | accepted | Use one project-relative scenario identity | identity and filtering contract |
| [ADR-046](ADR-046-semantic-review-report.md) | accepted | Generate a change-local semantic worklist | blocking report contract |
| [ADR-047](ADR-047-projected-remediation.md) | accepted | Verify remediation against the projected tree | classification evidence contract |
| [ADR-048](ADR-048-instruction-json-contract.md) | accepted | Keep effective policy and instruction metadata explicit | agent interface contract |
| [ADR-052](ADR-052-bge-advisory-evidence.md) | accepted | Use BGE as advisory admission evidence | second-stage model boundary |
| [ADR-054](ADR-054-versioned-evidence-output.md) | accepted | Version opt-in second-stage evidence output | machine-readable compatibility boundary |
| [ADR-055](ADR-055-independent-semantic-features.md) | accepted | Keep semantic capabilities independent | Cargo/runtime feature boundary |
| [ADR-056](ADR-056-bge-approval.md) | accepted | Require matching benchmark approval for default BGE | product admission boundary |
| [ADR-057](ADR-057-bge-local-artifacts.md) | accepted | Load BGE from preflighted local artifacts | model deployment boundary |
| [ADR-058](ADR-058-nli-evidence.md) | accepted | Add independent NLI admission evidence | second-stage evidence boundary |
| [ADR-059](ADR-059-complete-semantic-serialization.md) | accepted | Use one complete semantic serialization and exact E5 hash | model-input/cache boundary |
| [ADR-060](ADR-060-actual-tokenizer-caps.md) | accepted | Preflight actual tokenizer limits | inference safety boundary |
| [ADR-061](ADR-061-surface-specific-independent-pools.md) | accepted | Score BGE/NLI independently over each surface's E5 pool | candidate-pool boundary |
| [ADR-063](ADR-063-reranker-pair-budget.md) | proposed | Bound second-stage inference with an E5-ranked budget | runtime/review decision in open verification |

## Relationships

| Relationship | Meaning |
|---|---|
| ADR-019 -> ADR-027 | ADR-027 supersedes the old `givn verify` naming and carries the final lint contract. |
| ADR-037 -> ADR-041 | ADR-041 adds the strict E5 layer; ADR-037 still governs deterministic overlap findings. |
| ADR-039 -> ADR-041 | ADR-041 supersedes the advisory-only semantic review portion; local retrieval and search remain in ADR-039. |
| ADR-042 -> ADR-052 | ADR-052 supersedes the reranker experiment's classifier/gate interpretation. |
| ADR-053 -> ADR-063 | ADR-063 supersedes fixed pair-cap refusal; ADR-063 is still proposed until its change is complete. |
| ADR-054 -> ADR-058 | ADR-058 supersedes ADR-054's NLI deferral while ADR-054 remains the output-compatibility rule. |
| ADR-061 -> ADR-063 | ADR-063 adds a configurable ordered budget to the surface-specific independent branches. |
| ADR-023 (amended by 0.3.0) | The versioned-config decision now also governs managed-only commits, catalog-selected migrations, aggregate plans, and their clean-worktree boundary. |

## Evidence

| Area | Current checkout evidence |
|---|---|
| Manifest and addons | [`src/config/overlay.rs`](../../../src/config/overlay.rs), [`src/config/manifest.rs`](../../../src/config/manifest.rs), [`src/cli/init.rs`](../../../src/cli/init.rs), [`givn/commands.yaml`](../../../givn/commands.yaml) |
| Hooks and mandatory checks | [`src/cli/check.rs`](../../../src/cli/check.rs), [`src/hooks.rs`](../../../src/hooks.rs), [`src/cli/archive.rs`](../../../src/cli/archive.rs) |
| Agent ownership | [`src/skills/targets.rs`](../../../src/skills/targets.rs), [`src/cli/agents_block.rs`](../../../src/cli/agents_block.rs), [`src/cli/absolute_mode.rs`](../../../src/cli/absolute_mode.rs) |
| Release migration | [`src/cli/migration.rs`](../../../src/cli/migration.rs), [`assets/migrations/catalog.yaml`](../../../assets/migrations/catalog.yaml), [`docs/migrations/0.2.0-to-0.3.0.md`](../../migrations/0.2.0-to-0.3.0.md), and release migration feature scenarios |
| Retrieval and semantic evidence | [`src/spec/retrieval.rs`](../../../src/spec/retrieval.rs), retrieval feature files under [`givn/specs/`](../../../givn/specs/) |
| Active decision status | [`semantic-reranker-budget/tasks.md`](../../../givn/changes/semantic-reranker-budget/tasks.md), [`semantic-reranker-budget/design.md`](../../../givn/changes/semantic-reranker-budget/design.md) |
| PR #27 measurements | [`semantic-duplicates-real-model-filter-report.md`](../../semantic-duplicates-real-model-filter-report.md) and its linked benchmark/manual reports; raw JSON reports are local and ephemeral under the paths stated there. |

The PR #27 report is the current measured evidence available in this checkout:
`394` permanent scenarios, `1274` E5 candidates, and `795` BGE top-five-union
pairs. These numbers are report evidence, not an unconditional live invariant;
the raw model reports are not stored in this repository. Older values such as
`288/414?`, `385/100`, and `397/1311/803` are historical planning or earlier
measurement snapshots and must not be used as the current baseline.

## Archived ADRs

The archive contains only genuinely architectural decisions that were
superseded by an explicit replacement. Non-ADR records are not retained there.

| ID | Status | Reason for archive |
|---|---|---|
| [ADR-019](archive/ADR-019-lint-contract.md) | superseded by ADR-027 | Original static-check record. |
| [ADR-042](archive/ADR-042-bge-experiment.md) | superseded by ADR-052 | Experimental BGE gate interpretation. |
| [ADR-053](archive/ADR-053-bge-pair-cap.md) | superseded by ADR-063 | Fixed pair-cap refusal. |

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
