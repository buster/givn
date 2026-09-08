# Proposal: make-usecases-canonical

## Problem / Opportunity

Givn's active specifications describe a use case through a directory named as a
group and a `group.md` narrative. That vocabulary hides the domain goal, makes
capability ownership ambiguous, and cannot represent reusable behavior or
summary use cases without pretending that every directory is a user goal.
Agents and project maintainers therefore have to infer meaning from paths, and
moving a capability can silently break relationships and interaction coverage.

## Proposed Solution

Givn treats a stable use-case ID as the identity shared by a use-case narrative,
its capabilities, typed relationships, and the change that proposes a move.
Users can describe a leaf or summary use case with an explicit goal level,
actors, guarantees, rules, examples, interactions, capability references,
includes, extends, and an out-of-scope boundary. Reusable behavior can be
declared as a fragment instead of being misrepresented as a user goal.

The specification tree, routing advice, lint findings, relationship checks,
interaction coverage, and archive workflow expose use-case terminology and
preserve exact capability ownership. Typed relationships resolve to use cases
or fragments, reject dangling and cyclic references, and report their source
entry. A new or replaced use-case narrative is archived atomically with its
feature files, refuses stale replacement, and rolls back without losing the
previous specification tree.

Existing archived history remains unchanged. Active specifications do not
continue to teach or accept the former group narrative contract.

## Capability Routing

> Run `givn spec route` and record its recommendation and your decision
> for every capability this change touches. Editing this table IS the
> declaration — `givn check review` compares what you write here against
> the delta you actually author, not against a re-run of `route`.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| canonical-usecase-contract | NEW in make-usecases-canonical | NEW in make-usecases-canonical | This change establishes the active use-case contract. |
| capability discovery and ownership | EXTEND spec-discovery | EXTEND spec-discovery | Preserve existing discovery behavior while changing its domain vocabulary. |
| archive of specifications | EXTEND archival | EXTEND archival | Preserve existing archive guarantees while adding atomic use-case documents. |

## Out of Scope

The observable behavior described by existing feature scenarios is not being
rewritten. Historical content under `givn/archive/` is not migrated. Ideation
handoff and Persona promotion are handled by the separate ideation change.
Specification coverage comparison and release migration bundles are handled by
their separate ordered changes.

## Open Questions

The exact classification of Givn's current support collection into leaf use
cases, summary use cases, and fragments must be confirmed in the design and
corpus migration ledger. The release target version remains unspecified until
the release bundle is authored.
