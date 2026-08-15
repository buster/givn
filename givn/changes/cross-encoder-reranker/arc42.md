# arc42 Impact Assessment: cross-encoder-reranker

This change adds a local multilingual pair-reranker stage after embedding
candidate retrieval. It changes the review score, model/cache footprint, runtime
flow, quality evidence, risks, and architecture decision while preserving the
CLI boundary and feature-gated default build.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | Yes | Semantic review quality now depends on a jointly scored scenario pair rather than only independent embeddings. |
| 2 | Architecture Constraints | Yes | The optional feature gains a second local ONNX model and a binary-fixed reranker boundary. |
| 3 | Context and Scope | Yes | The existing review CLI now exposes reranker scores and model setup failures. |
| 4 | Solution Strategy | Yes | Retrieve-then-rerank becomes the semantic review strategy. |
| 5 | Building Block View | Yes | `spec::retrieval` gains candidate batching, pair scoring, and a reranker cache. |
| 6 | Runtime View | Yes | Review performs top-K retrieval, two-direction reranking, and averaged-logit gating. |
| 7 | Deployment View | Yes | A separate project-local reranker model cache is added to the opt-in build. |
| 8 | Crosscutting Concepts | Yes | Score calibration, ordering symmetry, feature gating, and setup failure semantics change. |
| 9 | Architecture Decisions | Yes | The reranker model and retrieve-then-rerank tradeoff require a new ADR. |
| 10 | Quality Requirements | Yes | Candidate recall, false-positive reduction, latency, and reranker setup become quality scenarios. |
| 11 | Risks and Technical Debt | Yes | Reranker cost, model relevance-vs-duplication mismatch, and top-K recall are new risks. |
| 12 | Glossary | Yes | Pair reranker, raw logit, and retrieve-then-rerank require definitions. |

## Status

STATUS: DONE
