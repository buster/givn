# Retrieval Model Setup

Givn's normal Cargo installation includes E5 retrieval, the strict E5 gate,
BGE evidence, and NLI evidence. Model files are downloaded only when an
explicit semantic command needs them. They are shared between projects in the
user cache; the permanent semantic index remains project-local.

## Cache Locations

The default role-specific cache locations are:

```text
$XDG_CACHE_HOME/givn/e5
$XDG_CACHE_HOME/givn/bge
$XDG_CACHE_HOME/givn/nli
$XDG_CACHE_HOME/givn/nli-exported
```

When `XDG_CACHE_HOME` is unset, Givn uses:

```text
$HOME/.cache/givn/e5
$HOME/.cache/givn/bge
$HOME/.cache/givn/nli
$HOME/.cache/givn/nli-exported
```

`XDG_CACHE_HOME` must be an absolute path. Givn fails with setup guidance when
neither a usable XDG cache nor `HOME` is available. Givn does not create model
caches inside `givn/` by default. The project-local index is
`givn/.spec-index.json`.

## Model Sources

E5 uses the FastEmbed mapping for `Qdrant/multilingual-e5-large-onnx` and the
logical provenance `intfloat/multilingual-e5-large`. BGE uses the pinned
`rozgo/bge-reranker-v2-m3` artifact revision. NLI uses the pinned
`MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7` artifact revision.
Givn validates every required file, tokenizer, metadata document, external
ONNX data file, snapshot revision, size, and SHA-256 digest before inference.

`HF_ENDPOINT` may point to an approved mirror or local artifact source. Givn
does not provide hosted inference; models run locally through the compiled
retrieval features.

## Overrides

Use explicit overrides when a project or maintainer owns a different artifact
location:

| Variable | Purpose |
|---|---|
| `GIVN_SPEC_MODEL_CACHE` | E5 FastEmbed/HF cache root |
| `GIVN_SPEC_E5_TOKENIZER` | Exact E5 tokenizer file for token-cap measurement |
| `GIVN_SPEC_BGE_MODEL_DIR` | Complete flat BGE artifact directory; disables automatic acquisition for BGE |
| `GIVN_SPEC_BGE_CACHE` | BGE HF cache root for automatic acquisition |
| `GIVN_SPEC_NLI_MODEL_DIR` | Complete flat NLI artifact directory; disables automatic acquisition for NLI |
| `GIVN_SPEC_NLI_CACHE` | NLI HF cache root for automatic acquisition |
| `HF_HOME` | Lower-priority compatibility cache root when no role-specific Givn path is supplied |

Relative explicit paths are resolved from the command's working directory.
Flat model-directory overrides are validated on every use and are never
silently replaced.

## Evidence Selectors

The existing offline selectors retain their meaning:

- `GIVN_SPEC_BGE=disabled` or `stub` does not download.
- `GIVN_SPEC_BGE=local` validates an explicitly provisioned flat artifact set
  and does not download.
- An unset `GIVN_SPEC_BGE` selects automatic BGE acquisition when the
  `cross-encoder-reranker` feature is compiled.
- `GIVN_SPEC_BGE=auto` explicitly selects the same automatic XDG BGE path.
- `GIVN_SPEC_RERANKER=disabled`, `stub`, `nli-stub`, or `nli-missing` does not
  download.
- `GIVN_SPEC_RERANKER=nli` validates an explicitly provisioned flat artifact
  set and does not download.
- An unset `GIVN_SPEC_RERANKER` selects automatic NLI acquisition when the
  `nli-evidence` feature is compiled.
- `GIVN_SPEC_RERANKER=nli-auto` explicitly selects the same automatic XDG NLI
  path.

Benchmark approval remains a separate evidence and release decision; it does
not gate runtime participation. NLI remains independent from BGE.

## Installed Builds And Fixture Requests

A development build (compiled with debug assertions, e.g. `cargo run` or
`cargo test`) honours every override above, including the simulated and
disabled-evidence selectors used by the test suite.

An installed build — the binary produced by `cargo install`, `cargo build
--release`, or CI artifacts — always evaluates the semantic gates with the
real models. It ignores the gate-bypassing fixture requests:

- `GIVN_SPEC_EMBEDDER` (any value),
- `GIVN_SPEC_BGE=stub|disabled|missing`,
- `GIVN_SPEC_RERANKER=stub|disabled|missing|nli-stub|nli-missing`,
- `GIVN_SPEC_BGE_APPROVAL=failed`,
- and the fixture-only channels (`*_LOCAL_STUB`, `*_TEST_POOL`,
  `*_DIRECTIONAL_SCORES`, `STUB_PAIR_SCORES`, `DUPLICATES_TEST_SCORES`,
  `*_TOKEN_BUDGET`, `NLI_REPORT_LIMIT`, `MAX_RERANK_PAIRS`,
  `RESOLUTION_NOTICES`, `DOWNLOAD_EVENTS`).

Blocked requests behave as if the variable were unset, so an installed build
proceeds with automatic real-model acquisition; a one-shot stderr line names
each ignored request (for example `GIVN_SPEC_BGE is ignored in installed
builds; the real model output is always used`). The model-location requests
(`GIVN_SPEC_BGE=auto|local`, `GIVN_SPEC_RERANKER=nli|nli-auto`,
`*_MODEL_DIR`, `*_CACHE`, `GIVN_SPEC_MODEL_CACHE`, `GIVN_SPEC_E5_TOKENIZER`,
`THREADS`) stay honoured, as do `HF_ENDPOINT`, `HF_HOME`, `XDG_CACHE_HOME`,
and `HOME`.

## Offline And Failure Behavior

After a successful setup, later commands reuse the validated snapshot without
network access. A missing, corrupt, partial, mixed-revision, or invalid cache
is repaired from the configured source when the selector permits acquisition.
If acquisition is disabled or the source cannot provide a valid snapshot, the
command reports the model, cache, failed artifact, and local override
alternative. Selected BGE/NLI layers retain their `*_UNAVAILABLE` fallback;
E5 remains a command failure because it generates the raw candidate set.

The `--no-default-features` build contains no retrieval model path and reports
`spec retrieval is unavailable` for semantic commands.

The repository benchmark helpers use `givn/nli-exported` for flat ONNX exports
and `givn/e5`/`givn/bge` for model caches. Their defaults are derived from
`XDG_CACHE_HOME` or `HOME`; they do not use `/tmp` or a project-local model
directory. Passing an explicit `--nli-model`, `--embed-cache`, or `--bge-cache`
is a deliberate override.

## Index Guidance

Run `givn spec index` to build or refresh the project-local permanent index. A
missing index is reported consistently as:

```text
spec index is missing at <path>; run `givn spec index` first
```

`spec search` stops when the index is absent. Explore, advisory review, blocking
review, and Duplicate Audit retain their established refresh or transient
fallback behavior while showing the same repair command. Archive maintenance
does not download models implicitly.
