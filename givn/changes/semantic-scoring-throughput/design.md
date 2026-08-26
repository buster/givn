# Design: semantic-scoring-throughput

## Technical Approach

Rust retrieval pipeline in `src/spec/retrieval.rs`. No new
dependencies: `fastembed =5.17.2` and `ort =2.0.0-rc.12` are pinned in
`Cargo.toml` (checked there, not recalled); thread resolution uses
`std::thread::available_parallelism()` (stable std API, cgroup-quota
aware on Linux).

### Thread budget

One resolver replaces the three hardcoded sites:

- `GIVN_SPEC_THREADS` env var; unset → `available_parallelism()`.
- Parse failure, zero, or overflow → error naming the setting, raised
  before any model artifact resolution or download (mirrors the
  `GIVN_SPEC_MAX_RERANK_PAIRS` validation pattern).
- Applied to:
  - E5 session options, replacing `with_intra_threads(1)`
    (`src/spec/retrieval.rs:8808`),
  - BGE `RerankInitOptionsUserDefined`, replacing
    `.with_intra_threads(4)` (`src/spec/retrieval.rs:5151`),
  - NLI ort `Session::builder`, replacing `.with_intra_threads(4)`
    (`src/spec/retrieval.rs:8040`).

Stages run sequentially, so the full budget is never contended between
models. If inter-stage overlap ever lands, the budget must be split;
this constraint is documented at the resolver.

### BGE grouped batching

`score_bge_pairs` currently issues two isolated calls per pair:

```rust
model.rerank(left_text, [right_text], false, Some(32))
model.rerank(right_text, [left_text], false, Some(32))
```

New flow: group the pair set into a `BTreeMap<&SimilarityEndpoint,
Vec<&SimilarityEndpoint>>` keyed by left endpoint; for each left, one
`rerank(left_text, rights_in_deterministic_order, false, Some(32))`.
Repeat with an identical map keyed by right endpoint for reverse
directions. Evidence insertion into
`BTreeMap<canonical_pair_key, BgePairEvidence>` is unchanged, so
grouping order cannot affect output identity. The existing trailing
`Some(32)` batch-size hint finally engages fastembed's internal
chunking because calls now carry more than one document.

Stub and local-stub backends keep their per-pair loops; grouping applies
only to the real-model path.

### NLI

Deferred by decision (see Decisions). Only the session builder's thread
count changes.

### Parity sync

No parity sync is required. The fingerprinted BGE candidate-pipeline
descriptor has no thread field, so the thread budget is configuration- and
attestation-independent: `pipeline_fingerprint` and the candidate-pipeline
attestation version must not change, and no approval fixtures regenerate.
The Python benchmark helpers (`tools/approach-comparison/compare.py`,
`tools/approach-comparison/evaluate_bge_admission.py`) hardcode their own
thread values but are legacy and no longer used; their sync is out of scope
for this change and deliberately deferred.

## Architecture Impact

- `src/spec/retrieval.rs`: thread-budget resolver; three call sites;
  grouped BGE loop on the real-model path; `threads` field on
  `BgeRun`/`NliRun` summaries and E5 status output.
- `tests/steps/mod.rs`: register new capability bindings.
- `tests/steps/semantic_scoring_throughput_steps.rs`: regular bindings —
  env override honoured, invalid values rejected before artifact work,
  status lines expose resolved counts, stub evidence equality under any
  setting.
- `tests/steps/semantic_scoring_throughput_e2e_steps.rs`: separate
  compiled-CLI subprocess binding asserting `threads N` in stdout.
- `docs/arc42/08-crosscutting-concepts.md`,
  `docs/arc42/09-architecture-decisions.md`: document the setting and a
  short ADR.

No index schema change. Cached vector reuse keys on text content
hashes, not thread config. Bitwise embedding drift across config
changes is ~1e-6; admission decisions separated by ≥0.04 cannot flip.

## Surface Contract

Status lines gain the resolved budget as `threads N`: the BGE and NLI stage
status lines append `, threads N` (append-only, never mid-line, so existing
`starts_with`/`contains` assertions on `BGE: model …`, `nli: model …`,
`BGE status:`, and `NLI status:` stay valid), and the advisory review's
non-JSON path prints an embedder status line `E5 status: E5_ENABLED,
threads N` in every mode where the embedder participates, using the stub
backend unchanged. Everything else stays byte-identical: candidate
identities, admission labels, accounting, distributions, JSON shapes, exit
codes. An invalid `GIVN_SPEC_THREADS` fails with the setting named before
any cache, download, or inference work.

## Test Runner And Strictness

Runner: `./run-tests.sh`; E2E smoke: `./run-tests.sh --e2e`
(`givn/commands.yaml`). Strict mode:
`GivnWorld::cucumber().fail_on_skipped()` in
`tests/features_runner.rs`; not-implemented step bodies use
`unimplemented!()`.

Single-scenario command template:

```sh
GIVN_FEATURES=givn/changes/semantic-scoring-throughput/specs \
  cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  --test features_runner -- \
  --name "<scenario title>"
```

Local runnability: fully self-contained through deterministic stub
backends (`GIVN_SPEC_EMBEDDER=stub`, BGE/NLI local stubs and
directional-score fixtures already exist). No external services,
databases, queues, browsers, or digital twins required — none apply.
Real-model wall-time measurement happens outside the Gherkin suite;
specs assert evidence equality and observable status fields, never
durations.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run review with `GIVN_SPEC_THREADS=3` and observe the budget in stage status output | The real CLI reports the configured thread budget on its stage status lines | CLI / terminal | Cucumber E2E binding starts the compiled `givn spec review --change <id>` subprocess with the env set and asserts its stdout |

Invalid-budget rejection is an error case of the same action, so it is a
regular scenario (malformed and zero budgets refused by name before model
work) and has no separate inventory entry or `@e2e` scenario, per the
interaction-inventory normalization policy.

## Decisions

- Default = `available_parallelism()` rather than physical-core
  estimation: std-only, quota-correct under CI containers;
  hyperthread oversubscription is acceptable and measurable later.
- Grouping is invisible by construction (key-sorted evidence maps). The
  executable spec guards thread-invariance: one scenario asserts identical
  candidate identities, admission labels, accounting counters, and label
  distributions across thread settings on the stub path. Grouped-batch
  verification stays design-level by decision — no added real-model task.
- NLI batched-tensor inference deferred to a named follow-up change
  (`semantic-nli-batched-inference`): it is the only item requiring new
  tensor code (pad/bucket/mask/chunk/unpad), its absolute win on
  current corpora is ~1 minute, and the repo principle forbids
  optimizing before measuring. Revisit after this change lands with a
  large-corpus timing run.
- Status-line exposure of `threads N` makes the setting observable and
  testable without duration assertions.
