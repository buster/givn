# Migration Proposal: 0.4.0 to 0.5.0

## Problem / Opportunity

Givn 0.5.0 is a major behavioural release. It retires the embedding-based
semantic similarity gate from the default build, reorganizes the permanent
specification corpus into use-case groups (`givn/specs/<group>/<capability>.feature`
plus one `group.md` per group), adds deterministic discovery
(`givn spec tree`, `spec find`, `spec route`, `spec regroup`, and a
deterministic `spec duplicates`), introduces capability retirement
(`@givn.retired`), and tightens several gates. A project created with
Givn 0.4.x has a flat specification corpus, may still carry
semantic-review guidance in ejected overrides, and may have active
changes whose disposition tables use free-form decision words. Running
the 0.5.0 binary against such a project without a migration leaves the
project on a contract it no longer matches: discovery tools see no
capabilities, stale semantic instructions keep steering authors, and
review.md disposition tables silently stop clearing findings.

## Proposed Solution

The machine-safe part (managed config scaffold, generated skill/command
targets, `givn_config_version` bump) is performed automatically by
`givn upgrade` and recorded in Commit A. This bundle then walks the
project LLM through the project-specific decisions that must never be
automatic: reconciling ejected guidance with the 0.5.0 instructions,
deciding whether the specification corpus migrates to the grouped layout
(using `givn spec regroup` as a proposal, confirmed by a human), deciding
whether the semantic features remain wanted as an explicit opt-in,
converting existing disposition tables to the enforced decision
vocabulary, moving interaction inventories into `group.md`, and fixing
any overlay that removes the now-protected `review` artifact.

This bundle is one phase in an aggregate migration plan when a project
skips additional versions.

## Out of Scope

- Automatically moving or rewriting project specification files. The
  grouped-layout migration applies only after a human confirms the
  proposed grouping; all moves are explicit `git mv` renames.
- Choosing group names or writing group narratives automatically. The
  proposal output is advisory; a human names the groups and writes real
  Actor/Goal/Interactions prose.
- Deleting project-authored content, ADRs, Arc42 facts, ejected facets,
  or user documentation.
- Adding a synthetic migration feature to the permanent specification
  tree.

## Completion

The LLM completes this phase when the project's specification layout
decision is applied or explicitly deferred, all ejected guidance matches
the 0.5.0 contract, every active change's review evidence satisfies the
deterministic gates, and this maintenance change passes its own review
and archive gates.
