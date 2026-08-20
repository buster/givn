# Architecture Documentation Update: semantic-e5-token-cap

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The change adds a user-visible retrieval-resilience requirement. | Added FR-36 for partial E5 indexing and visible classification-free cap findings. |
| 2 | 02 architecture-constraints | Yes | The E5 input-cap handling becomes a binding safety constraint. | Added TC-26 requiring partitioned E5 preflight, no truncation/sentinel vectors, and blocking additive evidence. |
| 3 | 03 context-and-scope | Yes | Index, duplicate, Explore, advisory review, and blocking review output changes at the CLI boundary. | Documented additive E5 cap output and the distinction between fitting vectors and cap metadata. |
| 4 | 04 solution-strategy | Yes | The retrieval strategy must continue useful work when a passage is over the E5 limit. | Updated retrieval and semantic-report strategy to partition passages and exclude capped endpoints from scoring. |
| 5 | 05 building-block-view | Yes | `spec::retrieval` gains cap-aware embedding results and persistent metadata. | Added the E5 passage-cap evidence building-block section and backward-compatible index model. |
| 6 | 06 runtime-view | Yes | Index, duplicate, Explore, advisory review, and blocking review each gain a non-aborting cap branch. | Updated runtime sequences, flow diagrams, and report behavior for `E5_TOKEN_CAP`. |
| 7 | 07 deployment-view | No | No binary packaging, target, model-cache location, service, or deployment topology changes. | |
| 8 | 08 crosscutting-concepts | Yes | Token-budget safety, output compatibility, error handling, and blocking semantics cross multiple surfaces. | Updated retrieval, Explore, duplicate, semantic-report, and error-handling concepts. |
| 9 | 09 architecture-decisions | Yes | ADR-060 is extended with a new E5 passage partition and metadata decision. | Recorded the no-vector/no-score rule, additive metadata, deterministic findings, and consequences. |
| 10 | 10 quality-requirements | Yes | The change adds explicit resilience, pair-integrity, surface-visibility, and blocking-quality scenarios. | Added SEM7 and Q-104 through Q-107. |
| 11 | 11 risks-and-technical-debt | Yes | Separating cap metadata from vector entries introduces a consistency risk. | Added R-72 covering stale cap metadata, fabricated scores, and fitting-vector loss. |
| 12 | 12 glossary | Yes | `E5 token-cap candidate` and `E5 token-cap metadata` become domain terms. | Added definitions for the new evidence and persistence concepts. |

## Notes

The existing BGE/NLI token-cap decision remains unchanged. The E5 extension
uses the same actual-tokenizer, inclusive-512, no-automatic-edit principles but
handles passage caps before vector creation so one bad passage cannot abort the
fitting index or hide deterministic findings.

---

## Status

STATUS: DONE
