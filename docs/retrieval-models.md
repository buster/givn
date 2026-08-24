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
```

When `XDG_CACHE_HOME` is unset, Givn uses:

```text
$HOME/.cache/givn/e5
$HOME/.cache/givn/bge
$HOME/.cache/givn/nli
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
- `GIVN_SPEC_BGE=auto` acquires and validates the XDG BGE snapshot.
- `GIVN_SPEC_RERANKER=disabled`, `stub`, `nli-stub`, or `nli-missing` does not
  download.
- `GIVN_SPEC_RERANKER=nli` validates an explicitly provisioned flat artifact
  set and does not download.
- `GIVN_SPEC_RERANKER=nli-auto` acquires and validates the XDG NLI snapshot.

An approved default BGE admission document may select the automatic BGE path,
but model availability never substitutes for benchmark approval. NLI remains
independent from BGE.

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
