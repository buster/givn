# Proposal: no-flat-fallback-for-new-capabilities

## Problem / Opportunity

The predecessor change made the archive merge honor a new capability's
declared group — but kept a fallback: when the proposal records no routing
decision at all, the merge still lands in a flat directory named after the
capability. Investigation shows that path is reachable only in two cases:

- A proposal without any Capability Routing table (a foreign or pre-routing
  proposal) — the review gate only warns for a missing table, so such a
  delta reaches the merge unchallenged.
- An off-workflow archive that never passed the review gate's routing
  checks.

In both cases the flat merge silently reintroduces the pre-migration layout
that no group owns, misattributes the merged scenarios' `@e2e` coverage in
the per-group statistics, and makes a later `EXTEND` of the capability
unresolvable. Silent re-creation of a retired layout is worse than refusal.

## Proposed Solution

The archive refuses to merge a new capability without a declared group: when
the capability exists nowhere in the permanent corpus and its proposal
records no `NEW in <group>` decision, the archive is blocked with an
actionable message naming the missing decision. Nothing is written; the
change stays active.

Capabilities that already live somewhere in the corpus (grouped or legacy
flat) are unaffected, as are retirements. The review gate keeps its warning
for a missing table — the archive is the fail-closed backstop.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| grouped-merge-for-new-capabilities | EXTEND deduplicate-change-dir-detection (group verification, score 4.00) | `EXTEND grouped-merge-for-new-capabilities` | Route matched on generic `@e2e`/archive vocabulary — false positive at minimal score. The touched capability is the archive merge target behavior, owned by `grouped-merge-for-new-capabilities` in the `archival` group; this change modifies its no-decision branch. |

## Out of Scope

- No change to the review gate's warn-vs-error policy for a missing routing
  table (the archive backstop covers it).
- No change to retirement handling or to capabilities that already exist in
  the corpus.
- No migration tooling — the corpus contains no flat capability directories.

## Open Questions

- None.