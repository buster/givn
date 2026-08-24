# Design: search-evidence

## Technology decisions

- Language/runtime: Rust, latest stable toolchain of this repository
  (`rust-toolchain.toml`). No new crates or external services.
- The BGE reranker (`rozgo/bge-reranker-v2-m3`, via `fastembed` +
  `ort`) and the NLI model (`MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7`,
  via `ort`) already exist in `src/spec/retrieval.rs` behind the
  `cross-encoder-reranker` and `nli-evidence` features. No model changes.
- `cucumber-rs` runner, already configured in this repository with
  `.fail_on_skipped()` (strict mode; see `tests/features_runner.rs`).
  Runner commands:
  - Unit/integration (verify.command): `./run-tests.sh`
  - E2E smoke (verify.e2e_command): `./run-tests.sh --e2e`
  - Strict mechanism: `GivnWorld::cucumber().fail_on_skipped()`.
  - Single scenario:
    `cargo test --no-default-features --features <set> --test features_runner -- --name "<scenario title>" --tags <tag>`
    where `<set>`/`<tag>` follow the tagging matrix below.

## What changes

### CLI surface

`SpecCommands::Search` in `src/main.rs` gains:

```rust
Search {
    query: String,
    /// Also run BGE and NLI cross-encoder evidence on the listed candidates.
    #[arg(long)]
    evidence: bool,
},
```

`cli::spec::run_search(query, evidence)` passes the flag through to
`spec::retrieval::search(root, query, evidence)`. The flag exists in every
build; in a build without `spec-retrieval` the existing
`spec retrieval is unavailable` error is unchanged.

### Search flow (`src/spec/retrieval.rs`)

- `search_enabled(root, query, evidence)` keeps the existing index-read,
  query-embed, `rank_entries` flow and keeps `print_search_results` byte-for-byte
  when `evidence` is false.
- With `evidence` true the ranked block is printed exactly as today, followed
  by the evidence block:
  - The displayed candidates are the top `RERANKER_TOP_K` (5) ranked entries —
    the same selection as explore.
  - Candidate full text comes from `collect_scenarios(root)` keyed by
    `(file, title)`, falling back to the title like
    `explore_enabled` does.
  - The query is wrapped in an `ExploreQuery` (title = trimmed raw query,
    source = "search", file = None, content_hash = sha256 hex of the raw query,
    text = raw query). The pair endpoint for the query is therefore
    `("__query__", <raw query>)` via `explore_query_endpoint`.
  - `bge_explore_run(root, &query, &candidates)` runs under
    `#[cfg(feature = "cross-encoder-reranker")]`, `nli_explore_run` under
    `#[cfg(feature = "nli-evidence")]` — the same cfg pattern as
    `explore_enabled`. Both reuse `max_rerank_pairs()` (default 2048,
    env `GIVN_SPEC_MAX_RERANK_PAIRS`) and `preflight_pair_budgets`, so the
    existing pair-budget and token-budget fixtures apply unchanged.
  - Per-candidate evidence is looked up via `canonical_pair_key`.

### Evidence output mapping (deterministic)

For every displayed candidate, in rank order, exactly one status line per
layer that produced a run:

| Run state | Printed lines |
|---|---|
| layer produced no run (feature not compiled, or backend disabled) | one summary line `BGE evidence: unavailable` / `NLI evidence: unavailable`, no per-candidate lines for that layer |
| run exists, status `BGE_ENABLED` / `NLI_ENABLED` | per-candidate status lines (see below) |
| run exists, status `BGE_UNAVAILABLE` / `NLI_UNAVAILABLE` (artifact missing or all pairs over cap) | per-candidate lines PLUS the summary line `BGE evidence: unavailable` / `NLI evidence: unavailable` |

Per-candidate status lines:

| Line | When |
|---|---|
| `  BGE: BGE_RECOMMENDED (symmetric raw <s>, forward <f>, reverse <r>)` | scored at ≥ `BGE_ADMISSION_THRESHOLD` (3.0) |
| `  BGE: BGE_FILTERED (symmetric raw <s>, forward <f>, reverse <r>)` | scored below threshold |
| `  BGE: BGE_TOKEN_CAP (pair <p> tokens, left <l>, right <r>, limit <m>)` | pair over token limit |
| `  BGE: BGE_UNSCORED` | candidate selected but has no pair (beyond budget) |
| `  NLI: NLI_RECOMMENDED (entailment <e>, neutral <n>, contradiction <c>)` | symmetric entailment ≥ `NLI_ADMISSION_THRESHOLD` (0.5) |
| `  NLI: NLI_FILTERED (...)`, `  NLI: NLI_TOKEN_CAP (...)`, `  NLI: NLI_UNSCORED` | NLI analogues |

An unscored or token-cap candidate is never presented as
`BGE_FILTERED`/`NLI_FILTERED`. The `BGE evidence: unavailable` summary is
printed for a run with status `BGE_UNAVAILABLE` and for a layer with no run
at all, so the summary appears regardless of whether the root cause is a
missing artifact, an all-over-cap selection, a disabled backend, or a build
without the feature.

Example rendering:

```
rank 1: The green border follows API key focus (givn/specs/.../x.feature, score 0.8481)
  BGE: BGE_RECOMMENDED (symmetric raw 4.2134, forward 4.1000, reverse 4.2500)
  NLI: NLI_FILTERED (entailment 0.0200, neutral 0.0500, contradiction 0.9300)
rank 2: ...
BGE evidence: unavailable
```

### Runner tagging

`run-tests.sh` compiles four feature sets and filters by tags. The new
scenarios are tagged so each runs under a binary containing the layer it
asserts:

| Scenario | Tags | Compiled feature set |
|---|---|---|
| Search reports BGE and NLI evidence through the givn CLI | `@nli-combined @e2e` | all three |
| Search without the evidence flag prints no evidence lines | — (e5) | `spec-retrieval` |
| Evidence search reports a BGE verdict for every listed candidate | `@bge` | `cross-encoder-reranker` |
| Evidence search reports an NLI verdict for every listed candidate | `@nli` | `nli-evidence` |
| Absent BGE evidence is reported but never presented as a verdict | `@nli` | `nli-evidence` |
| Evidence search without any cross-encoder answers with the plain ranking | — (e5) | `spec-retrieval` |
| Candidates beyond the evidence pair budget stay unscored | `@bge` | `cross-encoder-reranker` |
| An over-limit search pair receives token-cap evidence | `@bge` | `cross-encoder-reranker` |
| An over-limit search pair receives NLI token-cap evidence | `@nli` | `nli-evidence` |

Run commands per scenario (RED/GREEN checks in tasks.md):

- e5-tagged: `cargo test --no-default-features --features spec-retrieval --test features_runner -- --name "<title>"`
- `@bge`: `cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker --test features_runner -- --tags '@bge' --name "<title>"`
- `@nli`: `cargo test --no-default-features --features spec-retrieval,nli-evidence --test features_runner -- --tags '@nli' --name "<title>"`
- `@nli-combined @e2e`: `cargo test --no-default-features --features spec-retrieval,cross-encoder-reranker,nli-evidence --test features_runner -- --tags '@nli-combined and @e2e' --name "<title>"`

## Step definitions

Two new capability files, registered in `tests/steps/mod.rs`:

- `tests/steps/search_evidence_steps.rs` (regular glue; drives the real CLI via
  `common::run_givn_with_envs`, assembling the environment from the world's
  `retrieval_*` fields exactly like `user_runs` in `tests/steps/mod.rs` —
  NOT like `search_retrieval`, which only forwards the embedder):
  - `given "local NLI evidence is enabled"` → `world.retrieval_reranker = Some("nli-stub")`.
  - `given "every BGE pair exceeds the token limit"` → `world.retrieval_bge_token_budget = Some("all-513")`.
  - `given "every NLI pair exceeds the token limit"` → `world.retrieval_nli_token_budget = Some("all-513")`.
  - `given "BGE evidence recommends all listed candidates"` →
    `world.retrieval_bge_scores = Some("<directed fixture>")` with
    `GIVN_SPEC_BGE_DIRECTIONAL_SCORES` entries for the query endpoint
    `__query__|the command rejects an unknown provider` against all three
    retrieval fixture candidates
    (`givn/specs/retrieval.feature|Provider name is unknown`,
    `...Authentication token is missing`, `...Provider configuration is valid`)
    in BOTH directions, score `3.1000` (≥ `BGE_ADMISSION_THRESHOLD` 3.0).
  - `given "NLI evidence recommends all listed candidates"` →
    `world.retrieval_nli_scores = Some(...)` with
    `GIVN_SPEC_NLI_DIRECTIONAL_SCORES` entries for the same six directed
    pairs, logits `4.0000,0.0000,0.0000` each way (softmax → entailment
    ≈ 0.98 ≥ `NLI_ADMISSION_THRESHOLD` 0.5).
  - `when the user searches with evidence for "<query>"` → run
    `givn spec search --evidence "<query>"` with that assembled environment.
  - `then` matrix: existing generic output steps (`the command exits 0`, `stdout
    contains`, `stdout does not contain`) in `tests/steps/` are reused verbatim.

- `tests/steps/search_evidence_e2e_steps.rs` (e2e glue):
  - `given "an end-to-end project with a permanent spec index for evidence search"` →
    delegate to the regular fixture (`a givn project with permanent scenarios
    for retrieval` + `the local spec index has been built` semantics).
  - `given "BGE and NLI evidence are enabled for the end-to-end search"` →
    `retrieval_bge = stub`, `retrieval_reranker = nli-stub`.
  - `given "BGE and NLI evidence recommend the listed candidates"` → both
    directional fixtures.
  - `when the end-to-end user searches "<query>" with evidence` → subprocess
    `givn spec search --evidence "<query>"`.

Reused existing steps: `a givn project with permanent scenarios for retrieval`,
`the local spec index has been built`, `local BGE evidence is enabled`,
`the second-stage pair budget is 2` (registered by
`semantic_reranker_budget_steps.rs`).

The E2E scenario uses its own step text in `tests/steps/search_evidence_e2e_steps.rs`
(so its RED run is genuine before the regular steps exist), delegating project
setup to the shared fixture and spawning the `CARGO_BIN_EXE_givn` subprocess —
the same binary the repository's other retrieval e2e scenarios drive
(`tests/common/mod.rs`); no installation step is involved. The generic output
steps (`the command exits 0`, `stdout contains`) are shared with the regular
suite.

## Digital twins / external dependencies

None. All model backends are either real local ONNX artifacts or driver-level
stubs selected by env vars (GIVN_SPEC_BGE=stub, GIVN_SPEC_RERANKER=nli-stub,
GIVN_SPEC_EMBEDDER=stub) — no network service involved; runs offline exactly
like the existing explore/review flows.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| search permanent scenarios with optional cross-encoder evidence through the givn CLI | Search reports BGE and NLI evidence through the givn CLI | CLI | `common::run_givn_with_env` subprocess of the real `givn` binary: `givn spec search --evidence "<query>"` against a built stub-indexed project; assert `rank 1`, `BGE_RECOMMENDED`, `NLI_RECOMMENDED` in stdout |

## Black-box-first note

All scenarios drive the real CLI subprocess (same as existing retrieval
steps). No internal unit tests are added; the evidence lookup itself is
covered through the command output, and the underlying BGE/NLI run
machinery is already exercised by explore/review scenarios.

## Open design notes

- `GIVN_SPEC_MAX_RERANK_PAIRS` applies to search evidence like to explore —
  pairs above the budget become `BGE_UNSCORED`/`NLI_UNSCORED` per candidate.
- The query title shown in evidence endpoint identity is the raw query string;
  collisions with candidate titles are impossible because the query file
  identity is `"__query__"` (from `explore_query_endpoint`).

## Architecture decisions (ADR impact)

No new MADR. The BGE/NLI evidence contract (thresholds, directional scoring,
status vocabulary) already has ADRs:
- `docs/arc42/adr/ADR-052-bge-advisory-evidence.md` — extend the surface
  enumeration ("Explore applies BGE to its at-most-five ranked results") with
  `givn spec search --evidence` scoring the displayed candidates.
- `docs/arc42/adr/ADR-058-nli-evidence.md` — same surface addition.
- `docs/arc42/adr/ADR-063-reranker-pair-budget.md` — the pair budget now also
  bounds search pairs; noted in the durable chapter.

The arc42-docs artifact for this change records these refinements in chapter
09 and updates the durable chapters listed in `arc42.md`.