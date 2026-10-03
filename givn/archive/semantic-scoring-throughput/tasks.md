# Tasks: semantic-scoring-throughput

Runner: `./run-tests.sh`; E2E smoke: `./run-tests.sh --e2e`
(`givn/commands.yaml`).

## Scope decision (user-direct, recorded)

The original scenario spec (resolved thread count reported on stage status
lines, grouped BGE rerank, parity across thread settings) was dropped during
implementation as untestable: thread counts are not observable through the
stub backends, and probing real ONNX models for thread counts is not viable.
The user directed zero scenarios for this change. The retained behaviour is
code-level:

- Each model stage constructs with the machine's available parallelism as its
  intra-op thread budget by default.
- `GIVN_SPEC_THREADS` optionally overrides that budget; malformed or zero
  values are refused by name, before index/model work, when an eager gate can
  reach them (semantic review path).
- Applied only in the real-model paths (fastembed E5, local BGE reranker,
  local NLI ONNX session); the stub backends are untouched.

Interaction-coverage check: no User Interaction Inventory entries remain (no
observable interactions asserted).

Runner-group wiring: none needed. No `@scoring-throughput` group was added to
`run-tests.sh`. Instead a zero-scenario scope guard was added so the change's
code-only scope exits 0 without a cucumber zero-match run.

Production surface (design.md): `src/spec/retrieval.rs` —
`resolve_thread_budget()` (default `available_parallelism()`, override via
`GIVN_SPEC_THREADS`), eager validation at the top of `review_enabled`,
applied to the E5 fastembed, BGE reranker, and NLI ONNX `with_intra_threads`
sites. No status-line/thread-count reporting. No grouped BGE rerank. No
fingerprint/attestation change; Python tooling out of scope (user decision).

---

## Implementation (code-only, all done)

- [x] `resolve_thread_budget()` added to `src/spec/retrieval.rs`: const
      `GIVN_SPEC_THREADS`; unset → `std::thread::available_parallelism()`
      (quota-aware on Linux, fallback 1); parse-into-`usize` with named
      errors matching the `max_rerank_pairs` vocabulary; rejects zero.
- [x] Fail-fast gate: `resolve_thread_budget()?` at the top of
      `review_enabled` — a malformed/zero `GIVN_SPEC_THREADS` fails the
      semantic-review command by name before any index or model work.
- [x] Applied at the three real-model sites:
      - E5 fastembed: `.with_intra_threads(resolve_thread_budget()?)`
        (`fastembed_model`).
      - BGE reranker: `.with_intra_threads(resolve_thread_budget()?)`
        (user-defined reranking model options).
      - NLI ONNX: `.with_intra_threads(resolve_thread_budget()?)`
        (ORT session builder).
- [x] Stub backends untouched: the thread budget is never exercised through
      the E5/BGE/NLI stubs.
- [x] `run-tests.sh`: zero-scenario scope guard — a scope with no
      `Scenario` line returns 0 without invoking cucumber (a code-only
      change cannot be run by cucumber).
- [x] Spec zeroed: `specs/semantic-scoring-throughput/
      semantic-scoring-throughput.feature` reduced to a heading plus a
      rationale comment; no scenarios, no `@wip` tags.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [x] `cargo check` for the interest feature combos
      (`spec-retrieval`, `+cross-encoder-reranker`, `+nli-evidence`)
      compiles; partial-combo dead-code warnings are pre-existing.
- [x] `cargo fmt` applied.
- [x] Reinstalled the dogfood binary: `cargo install --path . --locked
      --force`; `givn skills sync`; `givn instructions`.

---

## Gates

- [x] `./run-tests.sh` (full suite + change scope) exits 0.
- [x] `./run-tests.sh --e2e` exits 0.
- [x] `givn lint` (all specs incl. permanent) passes.
- [x] `givn check tasks --change semantic-scoring-throughput` passes.
- [x] `givn check review --change semantic-scoring-throughput` passes.
- [x] `givn status --change semantic-scoring-throughput` shows every
      apply.requires artifact complete.

## Commits

Code-only change; one atomic implementation commit:

- [x] COMMIT: 4d7f74f -- implementation (resolver, three model sites, eager
  review-path validation, run-tests.sh zero-scenario guard, zeroed spec,
  design/arc42/ADR trim). No per-scenario commits exist because the change
  carries zero scenarios by decision.

---

## Archive

- [x] `givn archive --change semantic-scoring-throughput` succeeds with no
      remaining `@wip` and no unchecked boxes.
