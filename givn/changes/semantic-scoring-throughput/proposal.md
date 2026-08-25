# Proposal: semantic-scoring-throughput

## Problem / Opportunity

Model-based scoring in semantic review is unnecessarily slow on
multicore machines. All three model stages run with hardcoded thread
budgets — E5 pinned to a single thread since its introduction (no
recorded rationale), BGE and NLI to four — regardless of how many cores
the machine provides. On top of that, BGE scores every candidate pair
in isolation: each pair triggers two separate rerank calls, each
tokenizing, building tensors, and running an inference for a single
sequence pair. Measured on a 113-pair corpus: BGE 167 s, NLI 79 s,
with one core observed saturated during embedding phases. Scaled to the
configured 2048-pair budget this reaches tens of minutes of mostly idle
hardware.

## Proposed Solution

One environment setting, `GIVN_SPEC_THREADS`, controls how many compute
threads every model stage may use. When unset it resolves to the
machine's available parallelism. Malformed or zero values are rejected
before any model artifact is resolved, naming the setting. Status lines
report the active thread count so behaviour is observable per stage.

BGE scoring groups pairs that share a query-side scenario and scores
all partners in one rerank call instead of one call per pair; the same
grouping applies to the reverse direction. Results, their ordering,
admission labels, accounting counters, and all report formats remain
identical — only wall time drops.

NLI keeps its current per-sequence inference loop in this change and
receives only the shared thread setting. True batched-tensor NLI
inference is deferred to a dedicated follow-up change so that wall-time
gains can be measured before new tensor code is written.

## Out of Scope

- Thresholds, admission labels, report formats, and gate verdict logic.
- Running E5/BGE/NLI stages concurrently with each other.
- Candidate-level multi-threading across pairs.
- E5 embedding batching (already batched) and serialization.
- Output verbosity redesign (`--verbose`/`--json`).
- Batched-tensor NLI inference (follow-up change).

## Open Questions

None. Thread resolution is uniform across stages; grouping is invisible
by construction because evidence is collected into key-sorted maps.
