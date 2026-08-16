# Proposal: Generate the Semantic Review Report

## Problem / Opportunity

`givn check review --change <id>` currently runs the structural and retrieval
checks, but it does not leave a semantic worklist that an agent can inspect,
classify, and revisit after editing the current change. The evidence is mixed
into terminal output, candidate endpoints are easy to lose, and there is no
explicit indication of which candidates still need a decision.

Without a complete report, a change can appear reviewed even though one of its
semantic candidates was never considered. A later run also cannot distinguish
an unchanged candidate with an existing classification from a new or changed
candidate whose previous decision is no longer trustworthy.

## Proposed Solution

Extend only the blocking `givn check review --change <id>` path with a dedicated
semantic report gate. Every retrieval-available report-generation attempt,
including a blocked classification result, completely regenerates the
change-local working document. The existing advisory
`givn spec review --change <id>` path remains unchanged and does not create this
document.

```text
givn/changes/<id>/semantic-review.md
```

The report contains every relevant candidate involving an active Added or
Modified scenario in the current change whose existing E5 cosine score is at
least the binary-fixed boundary `0.9500`. It compares those scenarios with
permanent specifications (`delta-to-permanent`) and with other active scenarios
in the same change (`delta-to-delta`). Removed scenarios and
permanent-permanent pairs are excluded. For a Modified scenario, only its own
projected target endpoint is excluded; other scenarios in that target file
remain eligible.

Each entry identifies both endpoints, their scopes, normalized paths and
titles, the score, fixed-format content and full evidence hashes, and the
available deterministic facts. Endpoint evidence covers feature `Background`,
applicable `Rule` `Background`, and scenario steps in that order, including
every step docstring and every data-table row and cell in source order. It also
covers the delta operation, projected target identity, content hash, model and
index metadata, fingerprint/evidence schema version, and report schema
version. The report uses an exact Markdown wire grammar: fixed headers and
field order, compact JSON-style escaping/serialization for strings, arrays,
step data, and table cells, one `null` representation for absent values,
exactly one final newline, fixed four-decimal scores, and exactly 16 lowercase
hexadecimal hash characters. No timestamp or environment field is permitted.

Every candidate has one classification: `DUPLIKAT`, `VARIANTE`,
`VALID-BOUNDARY`, or `UNCERTAIN`. New candidates start as `UNCERTAIN`, and a
review with any missing, invalid, or `UNCERTAIN` classification exits non-zero.
Every resolved classification requires an agent-authored rationale. A candidate
that appears unrelated but remains above the E5 boundary is not dismissible:
the agent must sharpen or qualify the domain vocabulary and scenario invariant
until the candidate is no longer above the boundary, or resolve it as a
duplicate, variant, or explicitly valid boundary. A classification is carried
forward only when the candidate identity and both complete endpoint evidence
hashes are unchanged. Any changed content, table, background, operation,
projected target, model, index, fingerprint/evidence schema, or report schema
invalidates the prior classification and its rationale.

Candidates are ordered by ascending lexical scope (`delta-to-delta` before
`delta-to-permanent`), descending score, left endpoint tuple, right endpoint
tuple, and candidate id. Scores are rendered with exactly four decimal
places, including the exact admitted boundary `0.9500`, and
candidate/content/evidence hashes are exactly 16 lowercase hexadecimal
characters. Prior report entries are accepted only under the strict report
grammar; missing, invalid, duplicate, stale, unknown-field, or structurally
malformed entries do not carry a classification forward. Status and count are
recomputed, and the complete report is atomically replaced before a blocked
result is returned.

The report is generated in the active change and retained as historical
evidence under `givn/archive/<id>/` when the existing archive move runs. It does
not change feature files, permanent specs, `review.md`, or disposition rows.
The existing permanent-only index refresh remains allowed. A missing,
malformed, or incompatible index is treated as stale, rebuilt from the
permanent specifications, and followed by normal semantic review; the rebuilt
index must remain structurally permanent-only and must not contain Delta
vectors, classifications, or report state. The agent owns classification
decisions and all remediation edits.

## Out of Scope

- Automatic classification, feature-file edits, deletion, consolidation, or
  remediation verification.
- Semantic remediation verification, projected archive semantics, and
  permanent-permanent auditing. Existing archive movement retains the generated
  report, but archive implementation changes are outside this slice.
- BGE reranking, NLI evidence, coverage changes, and a new model threshold.
- Changes to `givn spec index`, `givn spec search`, or the existing structural
  overlap and coverage review contracts.

## Open Questions

None. The report path, dedicated blocking entrypoint, E5 boundary, candidate
scopes, deterministic identity/order, complete evidence invalidation rule,
strict prior grammar, classification rationale requirement, no-feature failure,
index boundary, classification values, and archive retention are fixed by the
Semantic Scenario Consolidation Plan and the hardening decisions for this
change.
