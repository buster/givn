# Proposal: semantic-review-output

## Problem / Opportunity

The second-stage evidence of semantic review is reported
inconsistently and buries its own signal:

- NLI caps its per-pair detail listing at 20 entries ordered by
  embedding score. When both stronger models agree that a flagged pair
  is not a duplicate (observed: BGE recommends 0 of 113, NLI 1 of 113),
  the one interesting recommendation can be invisible while the
  accounting line still claims it exists. Conversely BGE prints every
  scored pair unconditionally — over a hundred rows for a clean corpus.
- A failing similarity gate floods stdout with the full E5 candidate
  list before any second-stage evidence appears.
- Model artifact resolution chatter repeats up to three times per run
  regardless of outcome.
- The complete evidence disappears with the terminal session, although
  reproducing it costs minutes of model inference.

## Proposed Solution

Default output becomes decision-first: per-stage status lines,
accounting counters, distributions — and only those pairs the models
recommend, ordered by model confidence rather than embedding rank.
The raw E5 candidate list drops out of the default output entirely
(behind `--verbose`): an E5 score alone is not actionable signal, the
recommendation worklists are. Retained text lines keep their current
wording byte-for-byte.

The problem signal moves from the E5 threshold to the second stage.
When both evidence layers participate and scored every candidate, the
review fails only for the union of BGE- or NLI-recommended pairs —
E5-only hits above 0.95 that both models filter are noise, not
problems, and no longer fail the run or mask the recommendation
worklist. When either layer is missing, disabled, or unable to score,
the E5 threshold stays binding (no silent pass). This applies to the
authoring command and equally to the blocking workflow gates
(`givn check review`, `givn archive`), whose report already exempts
filtered pairs from the unresolved set; the corroboration rule becomes
uniform across both surfaces.

A `--format json` option emits the whole review as one structured
document: verdict, per-stage summaries, and every candidate with its
admission labels and evidence scores.

When the review fails, givn additionally writes that same document to
`givn/changes/<id>/review-findings.json` and names the path in the
failure message, so the evidence survives without rerunning model
inference. Repeated resolution chatter collapses to one status line
per stage unless `--verbose` is given.

## Out of Scope

- Thresholds, admission rules, candidate selection, pairing, and the
  semantic-report wire grammar/classification contract (its
  unresolved-set already exempts filtered pairs).
- Performance work (thread budget, grouped batching) — separate active
  change.
- `givn lint` / deterministic overlap output formats.

## Open Questions

None. Flag surface follows the existing `spec explore` /
`spec duplicates` conventions (`--format`, long flags). Gate
corroboration was resolved during design review: union signal, both
surfaces, fail-safe fallback.
