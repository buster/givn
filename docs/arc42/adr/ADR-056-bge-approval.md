---
status: accepted
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - BGE benchmark and production-risk review
informed:
  - release maintainers and users of BGE evidence
---

# Require matching benchmark approval for default product BGE

## Qualification

- [x] **Hard to reverse** - enabling BGE changes which candidates are blocking across all supported surfaces.
- [x] **Shapes quality attributes** - approval controls false-negative recall and CPU cost.
- [x] **Structural** - the approval document gates the default local BGE participation path.
- [x] **Must be shared** - maintainers must not treat a model cache or an unrelated benchmark as approval.

## Context and Problem Statement

BGE can narrow the blocking worklist. A benchmark produced by a different candidate pipeline or budget cannot justify enabling that behavior in the Rust product.

## Decision Drivers

- fail closed at product admission
- candidate-pipeline parity
- exact model, threshold, top-k, and budget matching
- explicit E5-only fallback

## Considered Options

- Enable BGE whenever its feature and model are present.
- Trust a manual environment switch and any benchmark report.
- Require a matching approval artifact before default product participation.

## Decision Outcome

Chosen option: **Require `givn/.spec-bge-admission.json` for default BGE participation**. The current runtime accepts it only when schema, model, threshold `3.0`, top-k `5`, active `max_unique_pairs`, `rust-production-v2`, and `acceptance.pass: true` match. A missing or failed approval leaves the E5-only contract. Explicit `GIVN_SPEC_BGE=stub` and `GIVN_SPEC_BGE=local` selectors are separate test/local controls; `local` still requires artifact preflight.

### Consequences

- A Python-reference report alone cannot enable default BGE.
- Changing the pair budget invalidates the runtime's approval match.
- Runtime approval validation does not independently recheck benchmark provenance or file hashes; artifact integrity is handled by the local preflight.

### Confirmation

Approval scenarios assert failed approval fallback, matching metadata, explicit selector behavior, and rejection of a missing Rust production parity marker in the approval tool.

## Pros and Cons of the Options

### Matching approval artifact

- Good, because product admission is tied to the active pipeline and budget.
- Bad, because maintainers must produce and maintain approval evidence.

### Feature/model presence only

- Good, because setup is short.
- Bad, because false negatives can silently remove required review work.

### Manual switch plus benchmark trust

- Good, because maintainers retain control.
- Bad, because the switch does not prove candidate-pipeline parity.

## More Information

See [ADR-052](ADR-052-bge-advisory-evidence.md), [ADR-057](ADR-057-bge-local-artifacts.md), and [chapter 7](../07-deployment-view.md).
