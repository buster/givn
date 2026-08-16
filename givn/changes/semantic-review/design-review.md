# Design Review: semantic-review

## Scope

The planning artifacts describe one PR-3 slice: a semantic evidence report and
classification gate for `givn check review --change <id>`. The advisory
`givn spec review --change <id>` path remains unchanged. The report contains
only E5 candidates at or above `0.9500` in delta-to-permanent and delta-to-delta
scopes. It excludes removed and permanent-permanent pairs.

The agent owns classifications and feature-file edits. There is no
`UNRELATED` escape classification. A pair that appears unrelated but remains
above the boundary stays unresolved until domain vocabulary and observable
invariants are sharpened, or it is resolved as `DUPLIKAT`, `VARIANTE`, or
`VALID-BOUNDARY` with a rationale.

## Decisions Resolved

### Candidate evidence

- The E5 boundary is fixed at `0.9500` for this binary.
- Candidate order is scope, descending score, left endpoint tuple, right
  endpoint tuple, then candidate id.
- Candidate ids and hashes reuse the existing deterministic `stable_hash` over
  canonical UTF-8 bytes.
- Endpoint evidence is rebuilt by reparsing permanent and delta feature trees;
  it is not recovered from the narrow permanent index entry.
- Evidence includes descriptions, tags, Scenario Outline names and Examples,
  feature/Rule backgrounds, scenario steps, docstrings, tables, delta operation,
  projected target, model/index metadata, facts, and schema versions. Permanent
  evidence is reparsed from the AST and is separate from the narrow index hash.
- Pair contradiction is candidate-level evidence and is excluded from endpoint
  evidence hashes.
- The exact-boundary fixture uses a score-layer override keyed by canonical
  endpoint pairs and enabled only with the deterministic stub embedder. It does
  not alter real E5 embeddings.
- The existing stable hash is used for canonical UTF-8 content, evidence, and
  candidate ids.

### Report grammar and lifecycle

- The Markdown report has fixed headers, field order, compact JSON values,
  `null` as the only absent representation, exact score/hash formats, LF line
  endings, and exactly one final newline.
- Prior reports are parsed strictly. Missing, invalid, duplicate, stale,
  unknown-field, or structurally malformed entries are discarded.
- Classifications and rationales carry forward only for unchanged candidate ids
  and complete endpoint evidence hashes.
- Reports are written through a same-directory temporary file and atomic rename
  before a blocked result is returned.
- A malformed or incompatible permanent index is treated as stale, validated
  structurally, rebuilt from permanent specs, and kept free of Delta state,
  delta vectors, classifications, and report state.

### Gate and operational compatibility

- Only `givn check review --change` calls the semantic report entrypoint.
- `givn spec review --change` remains advisory and does not create a report.
- Builds without `spec-retrieval` fail explicitly with the existing unavailable
  error and never claim semantic PASS.
- The repository dogfood binary is installed with
  `cargo install --path . --locked --force --features spec-retrieval`, followed
  by `givn skills sync`, `givn instructions`, and both test runners.
- The generated report is retained as historical evidence by the existing
  archive move.
- The default feature-free binary remains an explicit unavailable path; the
  self-dogfood install enables `spec-retrieval` before skill synchronization.

### Arc42

Arc42 is enabled. The change marker contains all twelve rows and
`STATUS: DONE`. Chapter 2 now records the retrieval-enabled blocking-review
constraint. Chapter 6 old arrow-flow blocks were converted to numbered prose or
Mermaid. Chapter 9 decisions have corresponding chapter 11 risks. The affected
durable chapters contain current semantic-report, vocabulary, no-feature,
evidence, index, and archive facts.

## Review Matrix

| Branch | Result |
|---|---|
| Scope | PASS. Proposal, feature, design, and plan agree on the E5-bound delta-only report and no automatic remediation. |
| Candidate universe | PASS. The exact boundary, scopes, exclusions, Modified target rule, order, and score-layer test seam are explicit. |
| Evidence and invalidation | PASS. AST reparse, full Gherkin evidence, candidate-level contradiction, version fields, and carry-forward hash are explicit. |
| Report grammar | PASS. Field order, JSON escaping, null representation, formats, final newline, strict prior parser, and atomic replacement are executable requirements. |
| Gate compatibility | PASS. Blocking check and advisory spec review have separate entrypoints; no-feature failure and retrieval-enabled dogfood are documented. |
| Missing scenarios | PASS. 25 regular scenarios plus one CLI E2E scenario cover boundary equality, ties, Rule nesting, complete AST evidence, Modified targets, removed/no-candidate, rationale enforcement, malformed report/index/delta, unavailable model/feature, archive retention, advisory separation, and full evidence invalidation. |
| Testability | PASS. `features_runner` uses `.fail_on_skipped()` and every new step will start with `unimplemented!()` during RED. |
| E2E fidelity | PASS. One CLI inventory entry maps to one E2E scenario driven by a real compiled subprocess and asserting CLI/report output. |
| Agent guidance | PASS. Canonical specs/review instructions and synchronized generated integrations describe rationale, vocabulary sharpening, and the absence of `UNRELATED`. |
| Arc42 | PASS. Independent chapter assessment matches the marker; affected chapters, ADRs, risks, quality scenarios, and Mermaid/prose policy are aligned. |

## Verification

`givn lint --change semantic-review` reports only intentional `@wip` findings
and no parse or structural errors. `git diff --check` passes. Production code
has not been written before tasks; the missing no-feature behavior and score
seam remain explicit implementation tasks rather than hidden assumptions.

DESIGN-REVIEW: PASS
