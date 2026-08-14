# Proposal: skill-rewrites

## Problem / Opportunity

The workflow skills currently stop at generic choreography. They do not tell an
author how to respond to the new overlap gates, how to document a long
scenario, or how to explain a removed-and-added replacement. They also do not
make the Black-Box-First decision visible: authors may add internal tests that
duplicate an end-to-end scenario without explaining what unique case the
internal test covers. The gates therefore produce findings without providing a
consistent path to resolve them.

## Proposed Solution

- The review guidance tells the user to classify every overlap disposition,
  every split-or-keep decision, and every removed-and-added pair before
  sign-off.
- The design and step-writing guidance tells the user to prefer the real
  interface and answer which unique case an internal test covers when it is
  retained.
- The specification guidance continues to use the deterministic gates and does
  not introduce retrieval instructions before the retrieval change exists.
- The generated guidance remains consistent across configured agent targets
  and slash-command entry points.

## Out of Scope

- Changes to overlap-gate behavior or the archive engine.
- The normative prose consolidation completed by the preceding change.
- Embedding/retrieval implementation; the later retrieval change owns its
  search steps.
- Watn step-mechanics consolidation.

## Open Questions

- None. Until retrieval is implemented, `givn-spec` guidance points to the
  deterministic gates only.
