# Arc42 Impact Assessment: semantic-scoring-throughput

This change makes semantic model scoring faster and tunable on multicore
machines: one `GIVN_SPEC_THREADS` environment setting controls the compute
threads of the E5 embedder and the BGE/NLI second-stage scorers, defaulting to
the machine's available parallelism and refusing invalid values by name before
any model work. BGE additionally groups pairs that share a query-side scenario
into one rerank call in each direction. NLI receives only the shared thread
setting; batched-tensor NLI inference is a named follow-up. Candidate
identities, admission labels, accounting, distributions, JSON shapes, and exit
codes stay byte-identical; only status lines gain the resolved count.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | Yes | The configurable compute budget is a user-facing functional requirement of the retrieval surface. | Added FR-40 describing the one `GIVN_SPEC_THREADS` setting, its available-parallelism default, and its by-name rejection of invalid values. No new quality scenario was added to chapter 10. |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint changes; no new dependencies. | |
| 3 | 03 context-and-scope | No | No external system or interface change; the new environment setting is internal runtime configuration owned by the cross-cutting chapter 08. | |
| 4 | 04 solution-strategy | No | Strategy (local retrieval, deterministic E5 gate, non-authoritative second-stage evidence) is unchanged; grouped BGE is an internal optimization. | |
| 5 | 05 building-block-view | No | No new component: the thread resolver and the grouped BGE loop live inside the existing `spec::retrieval` building block. | |
| 6 | 06 runtime-view | No | No new user-visible flow or sequence; the review flow is unchanged, only faster. | |
| 7 | 07 deployment-view | No | Binary, feature builds, model caches, and hosts unchanged; the setting is runtime configuration. | |
| 8 | 08 crosscutting-concepts | Yes | A new cross-cutting configuration rule controls compute resources across every model stage. | Added section 8.5i documenting the shared thread budget, its default, its by-name rejection, the observable resolved count on status lines, the sequential-budget constraint, grouped BGE, and the NLI deferral. |
| 9 | 09 architecture-decisions | Yes | One shared thread budget across three model stages is a durable cross-cutting config boundary with real alternatives. | Added proposed ADR-064 (shared model thread budget) with full MADR qualification, the register row, the ADR directory index entry, and the ADR-063 relationship. |
| 10 | 10 quality-requirements | No | No new quality scenario; the performance improvement has no new acceptance scenario in this change. | |
| 11 | 11 risks-and-technical-debt | Yes | The single-budget tradeoff is a risk, and the deferred NLI batched inference is deliberate technical debt. | Added R-40 (thread oversubscription and per-stage granularity tradeoffs) and D-15 (deferred `semantic-nli-batched-inference`). |
| 12 | 12 glossary | No | No new user-facing domain terms; "thread budget" and "grouped scoring" are implementation terms. | |

## Notes

- The observable contract (default resolution, per-stage status counts, by-name
  refusal of malformed and zero budgets, evidence identity across thread
  settings) is owned by the change's Gherkin delta. The ADR records only the
  durable cross-cutting decision rationale: one shared setting, its
  available-parallelism default, and the split-on-overlap constraint.
- `GIVN_SPEC_THREADS` follows the `GIVN_SPEC_MAX_RERANK_PAIRS` validation
  pattern (reject before artifact resolution), so invalid input fails fast
  without touching cache, downloads, or inference.
- The `@wip` scenarios in the delta are the pending executable contract for the
  implement step; the ADR stays `proposed` until that change passes review.

STATUS: DONE
