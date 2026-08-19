# Arc42 Assessment: semantic-token-cap-review

## Assessment

| # | Chapter | Affected? | Assessment and durable documentation update |
|---:|---|:---:|---|
| 1 | 01 introduction-and-goals | Yes | Added the complete semantic-input fidelity, E5 hash freshness, tokenizer-cap safety, and visible blocking token-cap goals and requirement. |
| 2 | 02 architecture-constraints | Yes | Added constraints for one complete serialization, exact content-hash invalidation, actual untruncated/unpadded tokenizer counts, independent BGE/NLI layers, and the deterministic no-network test seam. |
| 3 | 03 context-and-scope | Yes | Documented the model-input and budget-guard interfaces, token-cap report contract, shared four-surface scope, and automatic chunking/editing non-goal. |
| 4 | 04 solution-strategy | Yes | Added the complete semantic serialization/hash strategy, actual-tokenizer preflight, per-candidate cap handling, and independent BGE/NLI intersection strategy. |
| 5 | 05 building-block-view | Yes | Updated retrieval responsibilities and added the serializer, hash boundary, token-budget guard, per-candidate evidence, partial/all-over-cap states, and deterministic test seam building block. |
| 6 | 06 runtime-view | Yes | Added runtime behavior for complete serialization, pre-model tokenizer checks, shared duplicate/Explore/review guard flow, cap evidence, all-over-cap refusal, and independent BGE/NLI scoring. |
| 7 | 07 deployment-view | No | No deployment topology, distribution channel, process, port, or runtime-artifact topology changes; local model/tokenizer and test-stub deployment boundaries remain unchanged. |
| 8 | 08 crosscutting-concepts | Yes | Added the cross-cutting serializer, metadata/fingerprint separation, actual-tokenizer guard, cap-state distinction, shared surfaces, remediation guidance, and deterministic test boundary. |
| 9 | 09 architecture-decisions | Yes | Added MADR entries ADR-059 for serialization/hash, ADR-060 for actual-tokenizer preflight and token-cap states, and ADR-061 for independent BGE/NLI pool/intersection semantics. |
| 10 | 10 quality-requirements | Yes | Added quality-tree branches and scenarios for complete input/hash behavior, exact 512/513 boundaries, pre-inference refusal, partial/all-over-cap states, pool independence, shared surfaces, and bounded deterministic tests. |
| 11 | 11 risks-and-technical-debt | Yes | Added risks and mitigations for serializer/hash drift, tokenizer mismatch, cap-state collapse, sequential or divergent pools, test-seam leakage, and unauthorized automatic edits. |
| 12 | 12 glossary | Yes | Added terms for complete semantic serialization, semantic content hash, actual-tokenizer preflight, pair counts, token budgets, shared pools, intersections, and BGE/NLI cap states. |

## Notes

- Assessment basis: the hardened `proposal.md`, complete Gherkin feature delta,
  `design.md`, `design-review.md`, and all twelve durable chapter files were
  read before this update.
- The existing E5 candidate-generation algorithm, deterministic fingerprints,
  agent-owned classifications, and read-only duplicate-audit boundary remain
  unchanged. The complete semantic serialization is the model-input boundary;
  paths, tags, scores, classifications, and run statuses remain metadata, and
  deterministic fingerprints remain separate evidence.
- Actual local model tokenizers count untruncated and unpadded E5 passages and
  BGE/NLI pair encodings, including pair special tokens. Exactly 512 fits and
  513 is over. Per-candidate token-cap evidence stays visible, unresolved,
  non-filtered, and blocking; agents shorten or split and rerun without
  automatic chunking or feature edits.
- BGE and NLI independently score the same E5 pool. Combined recommendations
  intersect participating recommendations; NLI never receives only BGE
  survivors. A run-level unavailable state means that layer scored no pair.
- No ASCII-art diagram was introduced. Any new diagram is a Mermaid fenced
  block; existing plain directory listings remain listings rather than
  diagrams.

## Status

STATUS: DONE
