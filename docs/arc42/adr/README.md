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

Create an ADR only when at least one of these criteria has a clear, project-
specific justification. The qualification section in every ADR records all
four criteria, including why an unchecked criterion does not apply.

1. **Hard to reverse:** reversal would require substantial code, data, interface,
   tooling, or team migration effort.
2. **Shapes quality attributes:** the decision materially sets a floor or ceiling
   for performance, security, availability, maintainability, testability, or
   another lasting quality attribute.
3. **Structural:** the decision defines a component boundary, contract,
   responsibility, or dependency direction.
4. **Must be shared:** an unfamiliar contributor would systematically violate the
   architecture without knowing the decision.

The following are not sufficient on their own: affecting the system, being hard
to implement, being discussed at length, or using a familiar pattern. A logging
library, naming convention, formatting rule, private single-module refactor, or
swappable HTTP client is not an ADR merely because it exists in the codebase.
Pure product planning, repository process, licensing, and test/coverage
governance are retained only as flagged historical records when useful.

## Active Register

| ID | Status | Decision | Qualification basis |
|---|---|---|---|
| [ADR-001](ADR-001-standalone-tool.md) | accepted | Build givn as a standalone tool | hard to reverse; structural product boundary |
| [ADR-002](ADR-002-rust-implementation.md) | accepted | Implement the CLI in Rust | hard to reverse; quality and structural constraints |
| [ADR-004](ADR-004-single-static-binary.md) | accepted | Ship one native binary without runtime prerequisites | installability and deployment boundary |
| [ADR-005](ADR-005-facet-cascade.md) | accepted | Resolve project facets through a cascade | structural precedence; upgradeability |
| [ADR-006](ADR-006-eject-defaults.md) | accepted | Materialize defaults only through eject | project ownership and upgrade boundary |
| [ADR-007](ADR-007-per-facet-overrides.md) | accepted | Override templates and instructions independently | structural facet boundary; maintainability |
| [ADR-008](ADR-008-visible-givn-directory.md) | accepted | Keep configuration in visible `givn/` | structural filesystem boundary |
| [ADR-009](ADR-009-yaml-manifest.md) | accepted | Use YAML for the artifact manifest | durable configuration and graph contract |
| [ADR-010](ADR-010-manifest-overlay.md) | accepted | Customize the manifest through an overlay | structural workflow graph; upgradeability |
| [ADR-011](ADR-011-feature-is-spec.md) | accepted | Make `.feature` the specification | durable executable contract |
| [ADR-013](ADR-013-agent-integration.md) | accepted | Generate skills and preserve user-owned `AGENTS.md` content | agent integration ownership boundary |
| [ADR-014](ADR-014-filesystem-derived-state.md) | accepted | Derive lifecycle state from the filesystem | lifecycle state boundary |
| [ADR-015](ADR-015-manifest-test-runner.md) | accepted | Declare the test runner explicitly | public runner contract |
| [ADR-017](ADR-017-openspec-clean-break.md) | accepted | Keep a clean break from OpenSpec | parser and project-layout boundary |
| [ADR-020](ADR-020-givn-delta-namespace.md) | accepted | Use the `@givn.*` delta namespace | merge and parser contract |
| [ADR-021](ADR-021-missing-testrunner-sentinel.md) | accepted | Fail explicitly for the sentinel runner | fail-closed gate behavior |
| [ADR-022](ADR-022-documented-config-scaffold.md) | accepted | Generate a self-documenting config scaffold | configuration maintenance contract |
| [ADR-023](ADR-023-versioned-config-upgrade.md) | accepted | Preserve overrides during versioned upgrade | upgrade safety boundary |
| [ADR-024](ADR-024-configurable-artifact-graph.md) | accepted | Represent workflow as a configurable artifact graph | lifecycle topology |
| [ADR-025](ADR-025-specification-implementation-separation.md) | accepted | Separate specification and implementation concerns | durable contract boundary |
| [ADR-026](ADR-026-arc42-addon.md) | accepted | Provide durable Arc42 documentation as an addon | addon and documentation lifecycle boundary |
| [ADR-027](ADR-027-lint-command-name.md) | accepted | Name the static check `givn lint` | public CLI contract |
| [ADR-028](ADR-028-hooks-and-mandatory-checks.md) | accepted | Separate post hooks from mandatory checks | non-deactivatable gate boundary |
| [ADR-029](ADR-029-design-constraints-in-tasks.md) | accepted | Carry design constraints into scenario tasks | shared implementation conformance |
| [ADR-030](ADR-030-design-review-gate.md) | accepted | Stress-test design before tasks | lifecycle planning gate |
| [ADR-031](ADR-031-explore-pre-lifecycle.md) | accepted | Keep Explore pre-lifecycle | workflow boundary |
| [ADR-033](ADR-033-reverse-engineering-peer-flow.md) | accepted | Model reverse engineering as a peer flow | descriptive-specification boundary |
| [ADR-035](ADR-035-arc42-conformance-review.md) | accepted | Review Arc42 conformance after implementation | architecture evidence checkpoint |
| [ADR-036](ADR-036-agent-target-selection.md) | accepted | Persist agent target selection | generated-file ownership boundary |
| [ADR-037](ADR-037-deterministic-overlap-fingerprint.md) | accepted | Use deterministic fingerprints for structural overlap | review evidence boundary |
| [ADR-038](ADR-038-canonical-authoring-policy.md) | accepted | Keep one normative authoring policy | generated guidance contract |
| [ADR-039](ADR-039-local-retrieval.md) | accepted | Provide local E5 scenario retrieval | model/index subsystem boundary |
| [ADR-040](ADR-040-repository-owned-fixtures.md) | accepted | Keep CLI acceptance fixtures in Givn | public process boundary |
| [ADR-041](ADR-041-e5-uniqueness-gate.md) | accepted | Enforce a binary-fixed E5 uniqueness boundary | semantic review quality gate |
| [ADR-043](ADR-043-permanent-explore.md) | accepted | Explore one complete scenario against permanent specs | authoring query contract |
| [ADR-044](ADR-044-current-change-explore.md) | accepted | Add active changes request-locally to Explore | persistence boundary |
| [ADR-045](ADR-045-project-relative-identity.md) | accepted | Use one project-relative scenario identity | identity and filtering contract |
| [ADR-046](ADR-046-semantic-review-report.md) | accepted | Generate a change-local semantic worklist | blocking report contract |
| [ADR-047](ADR-047-projected-remediation.md) | accepted | Verify remediation against the projected tree | classification evidence contract |
| [ADR-048](ADR-048-instruction-json-contract.md) | accepted | Keep effective policy and instruction metadata explicit | agent interface contract |
| [ADR-049](ADR-049-permanent-duplicate-audit.md) | accepted | Audit the complete permanent corpus without writing audit state | corpus and persistence boundary |
| [ADR-052](ADR-052-bge-advisory-evidence.md) | accepted | Use BGE as advisory admission evidence | second-stage model boundary |
| [ADR-054](ADR-054-versioned-evidence-output.md) | accepted | Version opt-in second-stage evidence output | machine-readable compatibility boundary |
| [ADR-055](ADR-055-independent-semantic-features.md) | accepted | Keep semantic capabilities independent | Cargo/runtime feature boundary |
| [ADR-056](ADR-056-bge-approval.md) | accepted | Require matching benchmark approval for default BGE | product admission boundary |
| [ADR-057](ADR-057-bge-local-artifacts.md) | accepted | Load BGE from preflighted local artifacts | model deployment boundary |
| [ADR-058](ADR-058-nli-evidence.md) | accepted | Add independent NLI admission evidence | second-stage evidence boundary |
| [ADR-059](ADR-059-complete-semantic-serialization.md) | accepted | Use one complete semantic serialization and exact E5 hash | model-input/cache boundary |
| [ADR-060](ADR-060-actual-tokenizer-caps.md) | accepted | Preflight actual tokenizer limits | inference safety boundary |
| [ADR-061](ADR-061-surface-specific-independent-pools.md) | accepted | Score BGE/NLI independently over each surface's E5 pool | candidate-pool boundary |
| [ADR-062](ADR-062-omit-empty-optional-fields.md) | accepted | Omit empty optional semantic content | serializer/hash and quality boundary |
| [ADR-063](ADR-063-reranker-pair-budget.md) | proposed | Bound second-stage inference with an E5-ranked budget | runtime/review decision in open verification |

## Relationships

| Relationship | Meaning |
|---|---|
| ADR-013 -> ADR-036 | ADR-036 refines target selection and default output; the user-owned `AGENTS.md` rule in ADR-013 remains. |
| ADR-015 -> ADR-021 | ADR-021 refines the default command behavior without changing the explicit runner contract. |
| ADR-019 -> ADR-027 | ADR-027 supersedes the old `givn verify` naming and carries the final lint contract. |
| ADR-032 -> ADR-033 | ADR-033 supersedes the normal-change reverse-engineering flow. |
| ADR-037 -> ADR-041 | ADR-041 adds the strict E5 layer; ADR-037 still governs deterministic overlap findings. |
| ADR-039 -> ADR-041 | ADR-041 supersedes the advisory-only semantic review portion; local retrieval and search remain in ADR-039. |
| ADR-042 -> ADR-052 | ADR-052 supersedes the reranker experiment's classifier/gate interpretation. |
| ADR-053 -> ADR-063 | ADR-063 supersedes fixed pair-cap refusal; ADR-063 is still proposed until its change is complete. |
| ADR-054 -> ADR-058 | ADR-058 supersedes ADR-054's NLI deferral while ADR-054 remains the output-compatibility rule. |
| ADR-059 -> ADR-062 | ADR-062 amends the complete serializer with omission of empty optional content. |
| ADR-061 -> ADR-063 | ADR-063 adds a configurable ordered budget to the surface-specific independent branches. |
| ADR-029 (late label) -> ADR-064 | The later coverage-command record was assigned ADR-064 because ADR-029 already identifies task design constraints. |
| ADR-030 (late label) -> ADR-065 | The later coverage-README record was assigned ADR-065 because ADR-030 already identifies design review. |

## Evidence

| Area | Current checkout evidence |
|---|---|
| Manifest and addons | [`src/config/overlay.rs`](../../../src/config/overlay.rs), [`src/config/manifest.rs`](../../../src/config/manifest.rs), [`src/cli/init.rs`](../../../src/cli/init.rs), [`givn/commands.yaml`](../../../givn/commands.yaml) |
| Hooks and mandatory checks | [`src/cli/check.rs`](../../../src/cli/check.rs), [`src/hooks.rs`](../../../src/hooks.rs), [`src/cli/archive.rs`](../../../src/cli/archive.rs) |
| Agent ownership | [`src/skills/targets.rs`](../../../src/skills/targets.rs), [`src/cli/agents_block.rs`](../../../src/cli/agents_block.rs), [`src/cli/absolute_mode.rs`](../../../src/cli/absolute_mode.rs) |
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

| ID | Status | Reason for archive |
|---|---|---|
| [ADR-003](archive/ADR-003-product-name.md) | deprecated | Product naming; retained as flagged history. |
| [ADR-012](archive/ADR-012-coverage-completeness.md) | deprecated | Pure coverage/test governance. |
| [ADR-016](archive/ADR-016-repository-process.md) | deprecated | Repository setup and version-control process. |
| [ADR-018](archive/ADR-018-mvp-full-stack.md) | deprecated | MVP/product scope. |
| [ADR-019](archive/ADR-019-lint-contract.md) | superseded by ADR-027 | Original static-check record. |
| [ADR-032](archive/ADR-032-reverse-engineering-old-flow.md) | superseded by ADR-033 | Replaced reverse-engineering flow. |
| [ADR-034](archive/ADR-034-dual-license.md) | deprecated | Licensing/legal governance. |
| [ADR-042](archive/ADR-042-bge-experiment.md) | superseded by ADR-052 | Experimental BGE gate interpretation. |
| [ADR-053](archive/ADR-053-bge-pair-cap.md) | superseded by ADR-063 | Fixed pair-cap refusal. |
| [ADR-064](archive/ADR-064-coverage-commands-id-migration.md) | deprecated | Later duplicate `ADR-029` label; coverage governance. |
| [ADR-065](archive/ADR-065-coverage-readme-blocks-id-migration.md) | deprecated | Later duplicate `ADR-030` label; coverage governance. |

## Reserved IDs

`ADR-050` and `ADR-051` are reserved and currently unassigned. No decision,
placeholder file, or inferred historical record is attached to either ID.

## ID History And Migration

The original register assigned `ADR-029` to the design-constraints decision and
`ADR-030` to the design-review gate. A later coverage change reused both numbers
for unrelated decisions. This migration does not renumber the established
history: the original decisions remain the active `ADR-029` and `ADR-030`, while
the later coverage records are preserved as `ADR-064` and `ADR-065` in the
archive. References to the later records must use their new IDs; the register
does not treat either coverage record as current architecture.
