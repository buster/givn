---
status: accepted
date: 2026-08-16
decision-makers:
  - givn maintainers
consulted:
  - semantic-review implementation, fixtures, and design
informed:
  - agents and CI users running blocking review
---

# Generate a change-local semantic review worklist

## Qualification

- [x] **Hard to reverse** - report grammar, evidence, and carry-forward behavior form a durable workflow contract.
- [x] **Shapes quality attributes** - stale classifications must not silently approve changed scenarios.
- [x] **Structural** - the report is a distinct blocking-check output separate from feature files and the permanent index.
- [x] **Must be shared** - agents need one deterministic document for semantic evidence and classification.

## Context and Problem Statement

Terminal retrieval output is difficult to revisit, and a previous classification can become stale after a candidate changes. Blocking review needs a generated change-local worklist while keeping classification and feature edits agent-owned.

## Decision Drivers

- deterministic report regeneration
- complete endpoint evidence and change-local scope
- no feature-file or permanent-index report state
- explicit agent classification
- evidence retained when the change is archived

## Considered Options

- Keep semantic evidence only in terminal output.
- Let Givn classify candidates or edit features.
- Generate a blocking change-local report and carry classifications only when endpoint evidence matches.

## Decision Outcome

Chosen option: **Generate `semantic-review.md` only from blocking `givn check review --change <id>`**. The report compares active Added/Modified deltas with permanent endpoints (`delta-to-permanent`) and with active delta pairs (`delta-to-delta`) at E5 `0.9500`; Removed and permanent-permanent pairs are excluded. It writes the report atomically before returning a blocked result and leaves classification/remediation to the agent.

The writer uses fixed headers and field order. The current carry-forward implementation is not a full strict grammar parser: it scans recognized `- candidate_id`, classification, rationale, and evidence-hash lines. It accepts only `DUPLIKAT`, `VARIANTE`, and `VALID-BOUNDARY` for carry-forward, requires a non-empty rationale and matching left/right evidence hashes, and refuses carry-forward if duplicate candidate-id lines are found. Unknown or reordered content is not generally rejected by this extraction path.

The semantic candidate id is a 16-character lowercase hexadecimal FNV-1a hash of the scope and canonical endpoint scopes/files/titles. It does **not** include the score, evidence hashes, or report schema. Evidence-hash matching is a separate carry-forward condition.

### Consequences

- New or changed evidence remains `UNCERTAIN` and blocks until classified.
- Filtered BGE/NLI evidence is rendered outside the blocking classification set; unscored and cap states remain visible and blocking.
- The report is retained by the existing whole-change archive move as historical evidence.
- The current remediation verifier supports prior `current-change` to `permanent` entries; prior `delta-to-delta` entries are parsed as remediation entries but fail the verifier's scope precondition rather than being remediated.

### Confirmation

Semantic-review scenarios cover report creation, exact output, evidence invalidation, carry-forward, filtering, unscored/cap blocking, atomic write, feature/index non-mutation, archive retention, and feature-free failure.

## Pros and Cons of the Options

### Change-local blocking report

- Good, because evidence is revisitable and stale state is visible.
- Bad, because agents must classify every blocking entry and maintain generated working state.

### Terminal-only evidence

- Good, because no report format is needed.
- Bad, because prior decisions are lost or reconstructed inconsistently.

### Automatic classification or edits

- Good, because obvious cases could be faster.
- Bad, because model evidence is not a validated domain decision and automatic edits violate feature ownership.

## More Information

See [ADR-047](ADR-047-projected-remediation.md), [chapter 6.9](../06-runtime-view.md#69-blocking-semantic-review-report), and the parser limitation in [chapter 11](../11-risks-and-technical-debt.md).
