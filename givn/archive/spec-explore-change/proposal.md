# Proposal: Explore Scenarios in the Current Change

## Problem / Opportunity

Before creating or revising a scenario, an agent can currently compare a draft
only with the permanent specifications. That leaves the scenarios already being
written in the current change invisible, so two new scenarios can repeat the
same intent before either one is archived.

The current change also contains lifecycle information that matters to the
comparison. A removed scenario is no longer an active candidate, and a modified
scenario must not be reported as a match against itself. Without those rules,
the exploration result contains misleading candidates and cannot be used while
an agent is building a change incrementally.

## Proposed Solution

Extend `givn spec explore` with an optional current-change scope:

```text
givn spec explore --change <id> --file <draft.feature>
givn spec explore --change <id> --stdin
```

The command keeps the existing permanent-spec exploration behavior and, when a
change is supplied, also compares the draft with the active scenarios in that
change. Candidates identify whether they come from permanent specifications or
from the current change, including the candidate's project-relative source
location and owning change when applicable.

Only active scenarios participate. Scenarios marked as removed are excluded.
When a modified scenario is explored from its own feature file, its previous
identity is excluded so the scenario is not reported against itself; other
scenarios in that file remain eligible.

The permanent catalogue remains permanent-only. Exploring a change may use its
active scenarios for that request, but it does not make those scenarios appear
as permanent entries or modify any feature file. The command remains advisory,
read-only, and deterministic. The output metadata is intentionally additive, not
byte-for-byte PR1 output compatibility: every JSON result carries its scope and
optional owning change, including permanent-only requests, and every text result
identifies its source. Existing input validation, permanent ranking and
candidate set, and all PR1 fields and their ordering remain stable; the added
metadata makes the source distinction explicit for both output consumers.

Unsafe change identifiers, missing changes, changes without a specs directory,
and malformed delta features fail with clear non-zero errors before a rejected
request can update the permanent exploration index.

## Out of Scope

- Semantic review reports, classifications, remediation verification, and
  review gating.
- Projected archive semantics or automatic feature-file edits.
- BGE reranking, NLI evidence, coverage analysis, and step-definition
  resolution.
- Permanent duplicate auditing and changes to the existing `spec index`,
  `spec search`, or `spec review --change` contracts.

## Open Questions

None. The `--change` scope, lifecycle-tag handling, source labeling, and
permanent-only catalogue boundary are fixed by the Semantic Scenario
Consolidation Plan.
