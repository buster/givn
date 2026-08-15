# Proposal: enforce-spec-similarity-gate

## Problem / Opportunity

The retrieval index currently helps an author find related scenarios, but it does
not protect the permanent specification from accumulating semantic duplicates.
Two scenarios can describe the same invariant with different wording and live in
different feature files, where title and file-based checks do not expose the
overlap early enough.

Valid boundary scenarios can also be semantically close. They must not be
silently accepted just because a reviewer calls them a boundary: the scenario
must be rewritten so that its observable distinction is clear and its
similarity falls below the fixed safety threshold.

## Proposed Solution

During every review, the system checks both similarity scopes:

- all permanent scenarios against one another, including scenarios in different
  feature files;
- every added or modified scenario in the current change against the permanent
  scenarios.

The binary contains one fixed similarity threshold: `0.95`. It is not exposed
through project configuration, command-line flags, environment variables, or
review dispositions. An agent must not be able to raise the threshold to make a
failing review pass.

Similarity at or above the threshold blocks the review. The report classifies
the required remediation as one of:

- `MERGE`: the scenarios express the same invariant or value variant. The
  duplicate scenario must be deleted; it must not be kept as a second copy;
- `KEEP-AS-BOUNDARY`: the scenarios exercise different production or interface
  boundaries. The scenario must be rewritten, including a boundary-specific
  title and steps, until the similarity is below `0.95`.

There are no exceptions for a candidate that remains above the threshold. A
classification is guidance for the remediation, not permission to keep the
candidate. The review is green only after the projected permanent
specification, including the current change's additions, modifications, and
removals, contains no forbidden pair.

After a successful archive, the retrieval index is refreshed so the permanent
specification and its similarity inventory are current. Every subsequent review
re-evaluates the relevant pairs; prior classifications are explanatory history,
not permission to keep a pair above the threshold.

## Out of Scope

This change does not make embedding similarity a proof that two scenarios are
duplicates. Human review remains responsible for choosing whether to delete a
duplicate or rewrite a valid boundary, but neither decision bypasses the gate.

This change does not call an external embedding service, inspect implementation
code or step-definition bodies, provide a configurable threshold, or replace
deterministic title and structural overlap checks.

## Open Questions

The threshold is fixed at `0.95` in the binary. No other product decision is
blocking the executable scenarios.
