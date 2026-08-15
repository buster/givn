# Proposal: repository-wide-ownership-e2e

## Problem / Opportunity

The repository-wide specification ownership workflow is part of Givn's
observable product contract, but its review, archive, and rollback behaviour is
currently exercised from a consumer repository. That makes Givn's own gates
depend on a private consumer checkout and leaves the source of truth for the
workflow's acceptance behaviour in the wrong repository.

## Proposed Solution

Givn proves its repository-wide ownership workflow in its own test suite using
an isolated example project. A maintainer can observe that:

- a complete set of overlap dispositions is accepted and produces a
  deterministic net-change receipt;
- archiving publishes the retained specification, removes the obsolete
  specification, and leaves no duplicate scenario title;
- a pre-merge archive failure leaves the permanent specification collection
  unchanged;
- the checks use the same public Givn commands that a maintainer invokes, so a
  green result proves the shipped workflow rather than a consumer-specific
  imitation.

The example project contains generic scenario titles and behavior. It does not
depend on Watn, a provider, a server, a database, or a network service.

## Out of Scope

- Changes to Watn runtime behavior or Watn's provider integrations.
- Re-deciding which Watn scenarios are canonical; those decisions remain in
  Watn's own history and permanent specifications.
- Embedding retrieval, similarity scoring, or model setup into the
  deterministic review and archive gate.
- Changing the public syntax or semantics of existing Givn commands beyond the
  behavior required to make these workflows independently verifiable.

## Open Questions

None. The consumer-specific fixture is replaced by a generic Givn-owned
example project, and the existing deterministic review, archive, and rollback
contracts remain the source of truth.
