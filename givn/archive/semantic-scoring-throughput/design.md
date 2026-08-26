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
    (`fastembed_model`),
  - BGE `RerankInitOptionsUserDefined`, replacing
    `.with_intra_threads(4)`,
  - NLI ort `Session::builder`, replacing `.with_intra_threads(4)`.

Validation is also raised eagerly at the top of `review_enabled`, so a
malformed or zero `GIVN_SPEC_THREADS` fails the semantic-review command
by name before any index or model work. The budget is applied only in
the real-model paths; the stub backends are untouched.

Stages run sequentially, so the full budget is never contended between
models. If inter-stage overlap ever lands, the budget must be split;
this constraint is documented at the resolver.

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

- `src/spec/retrieval.rs`: thread-budget resolver; three call sites; eager
  validation at the top of `review_enabled`.
- `run-tests.sh`: a zero-scenario scope guard so a code-only change scope
  exits 0 without invoking cucumber (no scenario groups are added for this
  change).
- `docs/arc42/08-crosscutting-concepts.md`,
  `docs/arc42/09-architecture-decisions.md`: document the setting and a
  short ADR (ADR-064).

No index schema change. Cached vector reuse keys on text content
hashes, not thread config. Bitwise embedding drift across config
changes is ~1e-6; admission decisions separated by ≥0.04 cannot flip.

## Verifiability (zero-scenario decision)

This change is guarded by no scenario tests, by user decision: the
originally drafted observable surface (resolved thread count reported on
stage status lines) is not testable — thread counts are not observable
through the stub backends, and probing the real ONNX models for thread
counts is not viable (`givn/changes/semantic-scoring-throughput/specs/`
records the rationale). Verification is code-level:

- `cargo clippy --workspace --all-targets -- -D warnings` clean;
- `cargo check` compiles for the interest feature combos
  (`spec-retrieval`, `+cross-encoder-reranker`, `+nli-evidence`);
- `./run-tests.sh` and `./run-tests.sh --e2e` exit 0 (full suite and
  change scope, which is code-only);
- the `verify` / `verify-e2e` gates run through `givn check tasks` /
  `givn check review`.

Local runnability: fully self-contained through deterministic stub
backends; no external services required. Real-model wall-time
measurement happens outside any Gherkin suite.

## Interaction Coverage Matrix

None. No observable user interaction is asserted; the retained
behaviour is verified at the code level only.

## Decisions

- Default = `available_parallelism()` rather than physical-core
  estimation: std-only, quota-correct under CI containers;
  hyperthread oversubscription is acceptable and measurable later.
- Zero scenario coverage for this change (user decision): the only
  observably testable surface would be the `GIVN_SPEC_THREADS`
  rejection, which was also declined in favour of zero scenarios.
- NLI batched-tensor inference deferred to a named follow-up change
  (`semantic-nli-batched-inference`): it is the only item requiring new
  tensor code (pad/bucket/mask/chunk/unpad), its absolute win on
  current corpora is ~1 minute, and the repo principle forbids
  optimizing before measuring. Revisit after this change lands with a
  large-corpus timing run.
- No grouped BGE batching: the rerank path is unchanged apart from the
  thread count, keeping this change surgical and reverting to a
  future, separately-designed optimisation if timing evidence warrants.
