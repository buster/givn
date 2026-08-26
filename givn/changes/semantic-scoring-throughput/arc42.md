# Arc42 Impact Assessment: semantic-scoring-throughput

This change makes semantic model scoring faster and tunable on multicore
machines: one `GIVN_SPEC_THREADS` environment setting controls the compute
threads of the E5 embedder and the BGE/NLI second-stage scorers, defaulting to
the machine's available parallelism and refusing invalid values by name before
any model work. NLI receives only the shared thread setting; batched-tensor NLI
inference is a named follow-up. The budget is applied only in the real-model
paths; stub backends, candidate identities, admission labels, accounting,
distributions, JSON shapes, and exit codes are unchanged.

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---:|---|:---:|---|---|
| 1 | 01 introduction-and-goals | Yes | The configurable compute budget is a user-facing functional requirement of the retrieval surface. | Added FR-40 describing the one `GIVN_SPEC_THREADS` setting, its available-parallelism default, and its by-name rejection of invalid values. No new quality scenario was added to chapter 10. |
| 2 | 02 architecture-constraints | No | No legal, technical, or organizational constraint changes; no new dependencies. | |
| 3 | 03 context-and-scope | No | No external system or interface change; the new environment setting is internal runtime configuration owned by the cross-cutting chapter 08. | |
| 4 | 04 solution-strategy | No | Strategy (local retrieval, deterministic E5 gate, non-authoritative second-stage evidence) is unchanged; the thread budget is an internal tuning change. | |
| 5 | 05 building-block-view | No | No new component: the thread resolver lives inside the existing `spec::retrieval` building block. | |
| 6 | 06 runtime-view | No | No new user-visible flow or sequence; the review flow is unchanged, only faster. | |
| 7 | 07 deployment-view | No | Binary, feature builds, model caches, and hosts unchanged; the setting is runtime configuration. | |
| 8 | 08 crosscutting-concepts | Yes | A new cross-cutting configuration rule controls compute resources across every model stage. | Added section 8.5i documenting the shared thread budget, its default, its by-name rejection (validated eagerly on the semantic-review path), the sequential-budget constraint, and the NLI deferral. |
| 9 | 09 architecture-decisions | Yes | One shared thread budget across three model stages is a durable cross-cutting config boundary with real alternatives. | Added proposed ADR-064 (shared model thread budget) with full MADR qualification, the register row, the ADR directory index entry, and the ADR-063 relationship. |
| 10 | 10 quality-requirements | No | No new quality scenario; the performance improvement has no new acceptance scenario in this change. | |
| 11 | 11 risks-and-technical-debt | Yes | The single-budget tradeoff is a risk, and the deferred NLI batched inference is deliberate technical debt. | Added R-40 (thread oversubscription and per-stage granularity tradeoffs) and D-15 (deferred `semantic-nli-batched-inference`). |
| 12 | 12 glossary | No | No new user-facing domain terms; "thread budget" is an implementation term. | |

## Notes

- The retained behaviour (available-parallelism default, `GIVN_SPEC_THREADS`
  override, by-name refusal of invalid values, code-level application at the
  three model sites) is verified at the code level and by the full-suite and
  E2E smoke runs; the change carries no scenario tests by user decision. The
  rationale is recorded in the change's specs file and tasks.md.
- `GIVN_SPEC_THREADS` follows the `GIVN_SPEC_MAX_RERANK_PAIRS` validation
  pattern (reject before artifact resolution), so invalid input fails fast
  without touching cache, downloads, or inference.
- The ADR stays `proposed` until the change passes review and is archived.

STATUS: DONE
