# Proposal: grouped-merge-for-new-capabilities

## Problem / Opportunity

The review gate enforces that a new capability's delta lives under the group
its routing decision declares (`NEW in <group>` → `specs/<group>/`). The
archive merge, however, ignores the declared group: it looks for the
capability in the permanent corpus and, finding nothing (it is new), falls
back to a flat directory named after the capability. The consequence was
observed in change `add-ideation-skill`: the merged permanent spec landed in
`givn/specs/ideation-skill/` instead of `givn/specs/support/`.

Three follow-on damages, all observed:

- The corpus layout contradicts the group model — a group directory is the
  use case's identity; a flat capability directory is a pre-migration layout
  that no group owns.
- The per-group e2e pairing statistics miscount: the merged scenario's
  `@e2e` tag is not attributed to its declared group until someone moves the
  file by hand.
- A later `EXTEND` of the same capability cannot resolve its group — the
  router finds the capability in no group and tells the author to record
  `NEW` instead.

Every future change that declares `NEW in <group>` reproduces the defect.

## Proposed Solution

When the archive merges a new capability — one that exists nowhere in the
permanent corpus — the merge reads the change's recorded routing decision and
merges into the group the decision declares: a capability declared
`NEW in <group>` is merged into `givn/specs/<group>/<capability>.feature`.

Changes that record no routing decision for the capability keep the current
flat layout, so existing workflows (and the pre-migration corpus) behave
unchanged. Capabilities that already live in a group are unaffected — the
merge continues to target their existing group file.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| grouped-merge-for-new-capabilities | EXTEND spec-explore-change (score 10.00) | `NEW in archival` | Route matched the literal keywords `EXTEND`/`NEW` in this proposal's routing-table template — false positive. The touched surface is the archive merge into the permanent corpus, owned by the `archival` group ("Merge deltas into `givn/specs/`"). `spec-explore-change` owns `givn spec explore` candidate discovery, untouched by this change. |

## Out of Scope

- No change to the routing gate itself — the review-time consistency check
  stays as it is.
- No change to retirement (`@givn.retired`) handling — its grouped and flat
  paths already behave correctly.
- No migration of existing flat capability directories — the corpus holds
  none (verified; the one mis-placed file was moved by hand before this
  change).
- No change to what the merge validates (delta tags, WIP, overlap gates) —
  only the merge target's directory changes for new capabilities with a
  declared group.

## Open Questions

- None. The fallback rule (no recorded decision → flat layout) preserves the
  pre-migration corpus behavior and maintenance changes that skip specs.
