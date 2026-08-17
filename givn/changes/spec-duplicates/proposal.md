# Proposal: Read-Only Permanent Scenario Duplicate Audit

## Problem / Opportunity

The permanent scenario corpus can contain repeated, overlapping, or closely
related behaviour without giving maintainers a complete view of those
relationships. The existing exploration workflow starts from one proposed
scenario and is not a complete inventory of relationships already present in
the permanent corpus.

Maintainers therefore cannot reliably discover all duplicate and overlap
candidates before deciding which behaviour should remain canonical. They need a
safe audit that provides evidence without changing project specifications or
blocking ordinary change work.

## Proposed Solution

As a maintainer, I can run `givn spec duplicates` to inspect relationships
across the complete permanent scenario corpus.

The audit reports candidates from scenarios within the same feature and across
different features. It distinguishes exact duplicates, structural overlaps,
and semantic candidates, and identifies the scenarios and feature files that
must be reviewed together.

The report is deterministic so repeated audits of unchanged specifications
produce the same ordering and content. It includes every candidate in the
corpus rather than limiting the result to an interactive top-five view. I can
request a valid machine-readable JSON report when another tool needs to
consume the audit.

The audit is advisory and read-only. Running it leaves permanent
specifications and active changes unchanged and does not block an ordinary
change workflow.

## Out of Scope

- Automatically deleting, merging, renaming, or rewriting scenarios.
- Choosing the canonical scenario for a reported relationship.
- Applying remediation to permanent specifications or active changes.
- Auditing scenarios from an active change as part of this command.
- Blocking lint, review, archive, or ordinary change creation because a
  candidate exists.
- Replacing maintainer review with an unvalidated automatic decision.

## Open Questions

- Which evidence fields and labels are required in human-readable output for
  each candidate class?
- Which evidence fields must be present in the JSON representation, and what
  is the stable ordering contract for nested results?
- How should the audit report an empty corpus or a corpus with no candidates?
- Which similarity and structural boundaries distinguish an overlap from a
  semantic candidate?
