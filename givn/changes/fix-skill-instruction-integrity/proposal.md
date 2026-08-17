# Proposal: fix-skill-instruction-integrity

## Problem / Opportunity

The authoring guidance is inconsistent across the lifecycle. The canonical
specification guidance claims to define real-interface assertions and driver
fidelity, but does not state all of the rules required to prevent a weaker
repository-only or API-only test from being accepted as browser E2E evidence.
The design guidance also omits part of the information an author must record
about E2E setup.

The retrieval guidance still describes a pre-retrieval state even though
retrieval-capable builds exist. Review guidance describes semantic
classification, but does not make the required remediation evidence visible in
the general review artifact. Several descriptions of valid interfaces conflict
with each other, and project overrides or generated guidance can make the
effective policy unclear.

Agents also receive instructions to parse structured metadata that is not
available in the machine-readable instruction response. This makes automated
workflow choreography unreliable.

## Proposed Solution

Authors receive one complete, internally consistent interaction and E2E policy
from the canonical guidance. It explicitly distinguishes the real interface
from repository state, requires primary assertions on the interface output,
and prevents browser UI coverage from being replaced by direct backend calls.

Design authors receive all required E2E setup fields, including the runner,
driving mechanism, infrastructure, isolation, and evidence matrix. Retrieval
guidance distinguishes feature-enabled and feature-free environments and
explains when authoring and blocking review commands apply.

Review artifacts record both semantic classification and the observable
remediation evidence required for each classification. Interface terminology,
project override behavior, generated guidance freshness, and machine-readable
instruction metadata are documented consistently so an agent can determine
the effective rules and next action without guessing.

## Out of Scope

- The semantic similarity algorithm, model, score boundary, and projected-tree
  remediation implementation are not changed by this proposal.
- The existing project override precedence is not changed.
- Application behavior and permanent domain specifications are not changed.
- The lifecycle order of Givn artifacts is not changed.

## Open Questions

None. The machine-readable instruction contract will expose the metadata
already expected by the workflow commands; the human-readable output remains
unchanged.
