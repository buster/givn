---
status: superseded by ADR-069
date: 2026-08-19
decision-makers:
  - givn maintainers
consulted:
  - token-cap implementation, fixtures, and design review
informed:
  - agents, CI users, and local model maintainers
---

# Preflight actual tokenizer limits and expose cap evidence

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - cap states and remediation behavior are part of retrieval and blocking-review contracts.
- [x] **Shapes quality attributes** - it prevents silent truncation and bounds model correctness.
- [x] **Structural** - E5 passages and BGE/NLI pair encodings have explicit pre-inference guards.
- [x] **Must be shared** - all semantic surfaces need identical cap semantics.

## Context and Problem Statement

Character estimates, summed endpoint counts, padding, or truncation can make incomplete input appear scoreable. A single run-level failure also hides which candidates fit.

## Decision Drivers

- consuming tokenizer as authority
- no truncation or padding for the guard
- partial-run visibility
- classification-free remediation evidence
- deterministic test seam

## Considered Options

- Let the model truncate or pad.
- Estimate pair length by adding endpoint counts.
- Preflight complete input with the consuming tokenizer and record per-candidate cap state.

## Decision Outcome

Chosen option: **Preflight untruncated, unpadded inputs with the actual consuming tokenizer before construction or inference**. E5 passages and BGE/NLI pair encodings include pair special tokens where applicable. Exactly `512` fits; `513` is over.

Fitting E5 passages are embedded and indexed. Over-cap passages become additive `E5_TOKEN_CAP` metadata with no vector or semantic pair score. Selected over-cap BGE/NLI pairs become `BGE_TOKEN_CAP`/`NLI_TOKEN_CAP` with exact left/right/pair/limit counts, null score/probabilities, and blocking unresolved status. A partial second-stage run remains enabled; a layer with no fitting pair is run-level unavailable with reason `token-cap`. No automatic shortening, splitting, chunking, or feature edit occurs.

### Consequences

- Cap findings remain visible in index, audit, Explore, advisory review, and blocking report output.
- Fitting vectors survive when another E5 passage is over the limit.
- Production uses real local tokenizers; deterministic exact-boundary fixtures are test-only.

### Confirmation

Regular and E2E scenarios assert exact 512/513 handling, pair special tokens, partial/all-over-cap behavior, no model construction on all-over-cap runs, fitting-index preservation, and blocking cap output.

## Pros and Cons of the Options

### Actual-tokenizer preflight

- Good, because the guard matches the consuming model and exposes the remediation boundary.
- Bad, because each model artifact and tokenizer must be available locally.

### Truncation or padding

- Good, because inference can process every input.
- Bad, because scores no longer represent complete scenarios and clipping is hidden.

### Summed endpoint counts

- Good, because calculation is simple.
- Bad, because pair special tokens and tokenizer pair behavior are omitted.

## More Information

See [ADR-059](ADR-059-complete-semantic-serialization.md) and [chapter 8.5h](../../08-crosscutting-concepts.md#85h-token-budget-and-cap-state-details).
