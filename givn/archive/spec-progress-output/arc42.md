# Arc42 Impact Assessment: spec-progress-output

This change adds phase status and per-file download progress to the stderr of
long-running retrieval commands. It changes runtime output behavior and one
cross-cutting output-channel convention; it changes no data model, no
deployment topology, and no architecture decisions.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | No | Goals and product positioning unchanged; progress output is a usability fix, not a new goal. | |
| 2 | 02 architecture-constraints | No | No technical or product boundary changes. | |
| 3 | 03 context-and-scope | No | No new user-facing surface or stakeholder boundary; existing retrieval commands gain output. | |
| 4 | 04 solution-strategy | No | Strategy (local retrieval, validated cache, deterministic gates) unchanged. | |
| 5 | 05 building-block-view | Yes | `spec::retrieval` gains the stderr progress-reporting responsibility. | Extended the `spec::retrieval` responsibility row with phase status and per-file download progress. |
| 6 | 06 runtime-view | Yes | The `givn spec index` runtime sequence now emits phase announcements during acquisition and embedding. | Documented the stderr phase sequence, warm-cache silence, and disabled-layer silence in 6.8.1. |
| 7 | 07 deployment-view | No | Binary, feature builds, model caches, and hosts unchanged. | |
| 8 | 08 crosscutting-concepts | Yes | A new output-channel convention: stdout stays the result channel, stderr carries progress, always. | Added section 8.5a.1 documenting the channel separation and announced phases. |
| 9 | 09 architecture-decisions | No | No new ADR; the output-channel rule is a documented convention in chapter 8, not an independent architecture decision. | |
| 10 | 10 quality-requirements | Yes | Usability quality gains explicit scenarios for progress during long phases and silence on warm caches. | Added Q-63 and Q-64. |
| 11 | 11 risks-and-technical-debt | No | No new risk or debt; progress output reduces the perceived-hang failure mode. | |
| 12 | 12 glossary | No | No new domain terms; "phase status" and "download progress" are plain language. | |

## Notes

- stdout remains byte-identical: the duplicate-audit JSON, index summaries,
  search rankings, and review reports are untouched.
- Status messages are emitted unconditionally (no terminal detection) by
  decision of the requester; this is an observable contract, not a
  configuration option.
- Design review resolved the model-load announcement into a generic
  retrieval-embedder preparation phase announced before scenario processing
  in every backend; a real-embedder E2E with an ONNX fixture is out of scope.
- The duplicates-purity and advisory-review E2E scenarios run under the
  `@nli-combined` build, which is the only build compiling the second-stage
  layers.
- Chapter edits were applied to the durable files under `docs/arc42/`:
  05-building-block-view.md, 06-runtime-view.md, 08-crosscutting-concepts.md,
  10-quality-requirements.md.

STATUS: DONE
