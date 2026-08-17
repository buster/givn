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

An exact duplicate has the same canonical behaviour content after its title,
file location, and tags are ignored. A structural overlap has the same masked
assertion pattern or a strict assertion-subset relationship, with the
available fingerprint facts and any distinguishing values shown as evidence.
The structural contract does not compare an unreported Given/When action
structure. A semantic candidate is a remaining pair whose local E5 similarity
is at least `0.9500`, inclusive; that evidence is advisory and does not decide
that the pair is a duplicate.

The report is deterministic so repeated audits of unchanged specifications
produce the same ordering and content. It includes every candidate in the
corpus rather than limiting the result to an interactive top-five view. I can
request either the fixed text report or one valid machine-readable JSON report
when another tool needs to consume the audit. Both formats expose the same
scanned count, candidate count, contiguous ranks, endpoint identities, class,
relation, structural evidence, distinguishing facts, and score/null value.

The audit reads only permanent specifications under `givn/specs/`. A missing
permanent-specification directory is an empty successful corpus. A malformed
or unreadable permanent feature fails with an error naming that path. Active
changes, archive contents, and `.spec-index.json` are not evidence and are not
changed by the audit. The local retrieval model cache may be created as an
existing retrieval side effect.

The audit is advisory and read-only. Running it leaves permanent
specifications and active changes unchanged and does not block an ordinary
change workflow.

## Out of Scope

- Automatically deleting, merging, renaming, or rewriting scenarios.
- Choosing the canonical scenario for a reported relationship.
- Applying remediation to permanent specifications or active changes.
- Comparing Given/When action structure as a separate structural relationship.
- Treating a semantic score as an automatic duplicate decision.
- Auditing scenarios from an active change as part of this command.
- Using persistent index entries as audit evidence or refreshing the persistent
  index during the audit.
- Blocking lint, review, archive, or ordinary change creation because a
  candidate exists.
- Adding BGE, NLI, or another reranker to this audit.
- Replacing maintainer review with an unvalidated automatic decision.

## Open Questions

No product-level questions remain for the hardened planning contract. The
implementation must prove the byte-level text/JSON grammar and the documented
test-only score-control seam without weakening the production local-E5 path;
that implementation proof is tracked in `design-review.md` rather than
treated as a product decision here.
