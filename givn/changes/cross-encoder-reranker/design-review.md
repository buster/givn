# Design Review: cross-encoder-reranker

## Findings and Resolutions

| Area | Finding | Resolution |
|---|---|---|
| Model choice | FastEmbed-rs exposes a local multilingual `TextRerank` model and pair API. | Use `BGERerankerV2M3`; record the locked API/model source and keep the model feature-gated. |
| Score semantics | Reranker output is a raw relevance logit, not a probability or embedding cosine. | Score both pair directions, average raw logits, display sigmoid only diagnostically, and use a separate private branch boundary. |
| Candidate recall | A cross-encoder cannot inspect all 21k pairs cheaply. | Use deterministic top-K embedding retrieval and record K/recall as benchmark evidence. |
| False positives | Relevance does not imply duplicate. | Require a labeled benchmark and do not merge the branch if the duplicate/boundary confusion remains high. |
| Runtime | The real Rust probe is slow on CPU. | Batch documents per query, cap threads, measure integrated review duration, and treat impractical latency as a failed adoption criterion. |
| Scope | Same-file structural variants already have deterministic overlap checks. | Rerank cross-feature semantic candidates; keep the existing deterministic same-file gate. |
| Security/configuration | Agents must not weaken the experiment boundary. | Keep model boundary private; do not expose project/CLI/environment overrides. |
| E2E | The user-facing boundary is the CLI subprocess. | New scenarios invoke the compiled CLI and assert stdout/stderr/exit status. |
| Documentation | The branch changes the semantic review model and cache footprint. | Update PLAN, affected Arc42 chapters, ADR-042, glossary, and benchmark report. |

## Benchmark Result

The real local model benchmark completed. It processed 839 integrated review
candidates in `750.67 s` and produced seven candidates above raw-logit `3.0`.
Only one of the seven was a clear duplicate; a valid `status`/`new` boundary
scored higher than the known duplicate. This is a documented negative result,
not evidence to adopt the branch as the final gate.

## Status

DESIGN-REVIEW: PASS
