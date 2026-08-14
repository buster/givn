# Design: spec-retrieval

## Architecture

Add a `spec` CLI namespace with three operations:

- `givn spec index`: scan permanent `.feature` files, embed scenario text, and
  write `givn/.spec-index.json`.
- `givn spec search "<query>"`: embed the query, rank indexed scenarios by
  cosine similarity, and print the top five results with title, file, and rank.
- `givn spec review --change <id>`: refresh the permanent index and report
  advisory candidates for the active change.

The index is local and advisory. CI and deterministic lint never require it.
The review path refreshes the index and queries the current permanent tree when
the index is available; an unavailable model/index is reported as a setup
failure, never converted into a blocking score.

Generated `givn/.spec-index.json` and `givn/.spec-model-cache/` are ignored by
the project `.gitignore`; users may inspect or delete them without changing
permanent specifications.

## Embeddings and Cache

Use `fastembed-rs` `5.17.2`, the oldest available release carrying the required
`MultilingualE5Large` model and old enough for TC-13 at design time. Select the
model from the crate's `EmbeddingModel` enum. The model is downloaded on first
retrieval use into a configurable local cache and reused thereafter; no external
service is called during normal search. Indexing uses the E5 `passage:` prefix
and queries use the `query:` prefix because the measured model behavior depends
on those roles.

The dependency is declared with `default-features = false` and only the
`hf-hub-rustls-tls` and `ort-download-binaries-rustls-tls` features. This
change explicitly overrides the repository's 30-day dependency-age threshold
for the complete resolved `Cargo.lock` with a lockfile-derived
`GIVN_DEP_AGE_ALLOWLIST`. The exception is recorded in the tasks evidence
because the user authorized latest compatible versions and the crates.io API
was rate-limited in this environment.

## Build and Feature Gating

The `spec-retrieval` Cargo feature is opt-in and enables the optional
`fastembed` dependency. The default release and musl builds do not link the
native ONNX Runtime dependency and retain the shipped single-binary posture.
An author who wants production retrieval builds with
`cargo build --features spec-retrieval`; the feature is not required for
`givn lint`, deterministic gates, or normal installation. Retrieval commands
remain visible in the CLI without the feature and return explicit setup
guidance instead of pretending that an index exists.

The opt-in build uses the native ONNX Runtime supplied by fastembed and is
therefore not declared to be a portable musl release artifact until a target
build proves that link mode. This is an authoring capability, not a change to
the default distribution contract.

Index entries contain:

| Field | Meaning |
|---|---|
| `title` | Scenario title shown to the author |
| `file` | Relative permanent-spec path |
| `content_hash` | Hash of normalized scenario text for incremental updates |
| `embedding` | Persisted vector used for cosine ranking |

The index is rebuilt incrementally: unchanged hashes reuse their vectors,
changed scenarios are re-embedded, and deleted scenarios are removed. The
current change's own delta is excluded from both indexing and query results.
Open deltas from other changes are excluded by default; a future flag may opt
into them without changing the default CI behavior.

## Internal Primitives

The retrieval implementation introduces these internal primitives in
`src/spec/retrieval.rs`: `SpecIndex` for the persisted document,
`IndexEntry` for one scenario vector, `ScenarioText` for parsed normalized
input, and `EmbeddingRole` for E5 passage/query prefixes. The public operations
are `index`, `search`, and `review`; private helpers cover scenario collection,
hashing, index I/O, stub/production embedding, vector normalization, cosine
similarity, and advisory ranking. Content hashes and deterministic stub vectors
use an explicit FNV-1a implementation rather than a runtime-default hasher, so
the persisted index remains reproducible across processes and toolchain builds.
The CLI exposes these through
`src/cli/spec.rs` and `SpecCommands` in `src/main.rs`.

## Review Integration

`givn spec review --change <id>` refreshes the permanent index, embeds each
added or modified scenario in the active change, and prints its top candidates
as advisory findings. A candidate with an existing disposition row is not
reported as undisposed. `givn check review --change <id>` invokes the same
advisory pass after the deterministic overlap gate. Retrieval setup failures
are printed as advisory setup messages and never change the review exit code.
The review pass never indexes the active delta into the permanent index.

## CLI and Files

| Component | File |
|---|---|
| CLI dispatch | `src/main.rs` and `src/cli/spec.rs` |
| Index/search implementation | `src/spec/retrieval.rs` |
| Persisted index | `givn/.spec-index.json` |
| Feature step definitions | `tests/steps/spec_retrieval_steps.rs` |

The step definitions invoke the compiled `givn` subprocess. They do not call
the embedding library directly.

## Runner and Strict Mode

The existing Gherkin runner remains:

```text
./run-tests.sh
./run-tests.sh --e2e
./run-tests.sh --non-e2e
```

The no-argument command is the configured full `verify.command`; the explicit
`--e2e` command is the strict e2e subset, and `--non-e2e` is reserved for the
coverage split.

Strict mode is `.fail_on_skipped()` in `tests/features_runner.rs`; Rust
unimplemented steps use `unimplemented!()`. Single-scenario command:

```text
GIVN_FEATURES=givn/changes/spec-retrieval/specs cargo test \
  --features spec-retrieval --test features_runner -- --name '^<scenario title>$'
```

The test runner sets `GIVN_SPEC_EMBEDDER=stub`. This deterministic embedder
uses fixed, token-derived vectors and exercises the real compiled subprocess,
index file, hash reuse, ranking, and exclusion behavior without downloading a
model. A test can set `GIVN_SPEC_EMBEDDER=missing` to exercise setup failure.
Production commands use the configured local model cache when the variable is
unset; no live third-party API is used by the e2e gate.

## Local Runnability and Digital Twins

The real interface is the CLI subprocess. `./run-tests.sh` starts the complete
local test run; no server, database, queue, or third-party service exists. The
embedding model registry is the only external download during first-time
production setup; it is cached locally and is not contacted by CI. No digital
twin is needed for a runtime service.

If the model is unavailable, `givn spec index` fails with explicit setup
guidance. It does not silently return empty search results or block unrelated
lint/test commands.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| Run `givn spec index` to build a local search index over permanent scenarios | Local index reports the permanent scenario inventory | CLI | Real `givn spec index` subprocess and stdout/index-file assertions |
| Run `givn spec search` to find semantically related existing scenarios | Search returns a semantically related scenario | CLI | Real `givn spec search` subprocess and ranked stdout assertions |
| Run `givn spec review --change` to surface advisory candidates for a delta | Review reports an advisory retrieval candidate | CLI | Real `givn spec review --change` and `givn check review --change` subprocesses and advisory stdout assertions |

The own-delta exclusion scenario is a regular variant of the search action.
The index refresh and unavailable-model scenarios are non-e2e error and
incremental-behavior coverage.

## Coverage Process

| Process | Started by | Instrumented artifact | Profile output | Merge step | Non-zero production probe |
|---|---|---|---|---|---|
| Gherkin runner and givn subprocesses | `./measure-coverage.sh` | instrumented givn binary and runner | separate Cobertura reports | `./merge-coverages.sh` | `src/spec/retrieval.rs` index and cosine-ranking paths |
