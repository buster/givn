# Proposal: finding-severity-routing-signal

## Use-Case Context

- Use-case ID: none — cross-cutting tooling change; it extends the lint
  command, the routing query, and the proposal guidance.
- Ideation topic: none — the change originates from the rules-refinement
  handover `docs/rulesrefinement/08-finding-severity-routing-signal.md`.
- Confirmed Personas: agent--technical-role, givn-maintainer--owner — both are
  confirmed in the permanent persona collection; the change alters CLI output
  and the maintainer's tooling.

## Problem / Opportunity

givn mixes hard gates and hints in the same output channel without severity
labels, and the weakest tool — route, based on content-word overlap — is
given the most prominent per-change ritual. Every proposal records a routing
verdict with a verbatim "its ranking is advisory" disclaimer, and lint
reports around twenty advisories that exit 0 and are ignored. Agents learn
that givn output is boilerplate, which degrades every current and future
check.

The evidence is concrete: all 18 active changes in a real project record the
same advisory disclaimer while their decisions were correct, one proposal
shows a six-way tie at the same score, and lint reports about twenty shape
and subset advisories that everyone ignores because they carry no severity.

## Proposed Solution

- Every lint finding carries a severity: `advisory`, `finding`, or
  `blocking`. The severity is rendered in the human-readable output and
  exposed in the JSON output; the existing exit-code contract is preserved
  and documented against the mapping. Advisory findings never fail the
  command. The mapping is one data-driven table so every finding kind
  registers in one place.
- Route reports its confidence. When the top candidates tie or the overlap is
  below the signal threshold, it reports "no signal" instead of a ranked
  list. When it has signal, the recommendation names the confidence
  explicitly, so the verbatim advisory disclaimer is no longer needed.
- Route weights ownership evidence above lexical overlap: the candidate
  owner's goal and rules text, the capability's interaction inventory, and
  the capability name with its owning use case. A candidate whose owning use
  case goal it does not advance ranks below a goal-aligned candidate.
- Proposal planning uses the confidence: with signal, the routing table
  records route's recommendation and the decision as today; with no signal,
  the recommendation is omitted and the decision is recorded with its
  rationale directly. The recorded decision stays mandatory either way.

## Capability Routing

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| lint-command | not surfaced (route's ranking is lexical) | `EXTEND lint-command` | The capability owns the static lint findings; the severity classification extends them without changing the exit-code contract. |
| spec-explore-change | not surfaced (same ranking) | `NEW in fragments` | The capability lives in the reusable support fragment and owns the `givn spec route` behavior that gains confidence and ownership weighting. |
| instruction-integrity | not surfaced (same ranking) | `NEW in fragments` | The capability owns the proposal guidance checks; the no-signal routing rule is asserted there. |

## Out of Scope

- Removing `givn spec route` or the capability-routing decision record.
- Hard enforcement of routing correctness; the check that a decision was
  recorded stays as is.
- Re-running route during review to compare decisions.
- Any LLM-based semantic routing.
- A conditional-template mechanism for route confidence; the proposal
  guidance records the no-signal rule as text because the existing addon
  conditional cannot express it.

## Open Questions

None. The exact severity mapping and the confidence threshold are design
decisions; the required observable outcomes are the severity labels, the
no-signal report, and the ownership-weighted ranking.
