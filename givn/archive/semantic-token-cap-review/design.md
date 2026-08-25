# Design: Complete Semantic Token-Cap Evidence

## Technical Summary

Extend the existing local semantic retrieval pipeline with one complete,
deterministic Gherkin serializer and a pre-inference token-budget guard. The
same complete text feeds E5 indexing and BGE/NLI pair evaluation. Oversized
pairs remain visible and blocking, but receive explicit per-candidate cap
evidence instead of silently truncated model scores.

The existing E5 candidate-generation algorithm, deterministic exclusions,
thresholds, agent-owned classifications, and read-only duplicate-audit boundary
remain unchanged. This design is planning-only; it does not create tasks or
modify production Rust during this change.

## Scope And Invariants

The shared guard applies to all four retrieval surfaces:

1. Permanent duplicate audit.
2. Permanent and change-aware Explore.
3. Advisory change review.
4. Blocking change review.

The E5 candidate pool is the single source for BGE and NLI evidence. BGE and NLI
are independent evidence branches over that same pool. A model score is evidence
only: it never assigns `DUPLIKAT`, `VARIANTE`, `VALID-BOUNDARY`, or `UNCERTAIN`,
and it never edits a feature file.

The candidate remains unresolved when any enabled second-stage layer has not
scored it. A candidate is non-blocking only when the applicable evidence layer
actually scored it and emitted that layer's filtered status. Token-cap and
unavailable evidence never become filtered evidence.

## Complete Semantic Serialization

Replace the current short semantic projection with one canonical serializer
that accepts the Feature, optional Rule, inherited Feature/Rule Backgrounds, and
Scenario AST nodes. Its stable section order is:

1. Feature keyword, name, context, and description.
2. Feature Background steps.
3. Rule keyword, name, context, and description.
4. Rule Background steps.
5. Scenario keyword, title, and description.
6. Scenario behavior steps with resolved `Given`, `When`, and `Then` roles.
7. Step data tables in header/row order.
8. Step docstrings with their complete content.
9. Scenario Outline template and concrete Examples rows in source order.

`And` and `But` inherit the semantic role of the preceding resolved step. Table
cells, example rows, and docstrings are whitespace-normalized deterministically
without dropping their content. Missing optional sections use one stable empty
representation. The serializer is deterministic for the same parsed AST.

Paths, tags, line numbers, delta operations, model scores, evidence statuses,
and Givn classifications are metadata and are not included in model text.
Deterministic exact and structural fingerprints continue to use their existing
richer canonical representation; the semantic serializer does not replace that
evidence.

## E5 Index And Hash Invalidation

The E5 candidate-generation algorithm remains the existing algorithm: the same
candidate scopes, top-K selection, identity exclusions, deterministic findings,
ordering, and similarity threshold are retained. Only the text passed to E5
changes to the complete semantic serialization.

For every permanent scenario, compute `content_hash` from the exact complete
serialized text used as the E5 passage input. Refresh the permanent index as
follows:

- Reuse an existing vector only when its file/title identity and stored
  `content_hash` equal the newly serialized scenario's hash.
- Re-embed an entry when the complete text hash differs, including entries
  created by the old short projection.
- Remove entries for deleted scenarios.
- Do not add a separate serializer-version field or require a serializer version
  to invalidate an entry; the content hash is the invalidation boundary.

The complete serializer is also used for active-delta E5 passage input and for
the BGE/NLI text map. Existing deterministic fingerprint and semantic evidence
hashes remain separate values with their existing purposes.

## Token-Budget Guard

Before model construction or inference, the guard encodes every input with the
actual tokenizer for the model that will consume it. The budget measurement is
untruncated and unpadded. An over-cap input is never made valid by tokenizer
truncation, batch padding, or model-side silent clipping.

| Model input | Limit | Count |
|---|---:|---|
| E5 individual passage | 512 | The complete serialized passage with the model's required passage role input. |
| BGE pair | 512 | The complete left/right pair encoding, including pair special tokens. |
| NLI pair | 512 | The complete left/right pair encoding, including pair special tokens. |

Exactly `512` valid tokens fit. `513` is over the cap. For each BGE/NLI pair,
record `left_tokens`, `right_tokens`, `pair_tokens`, and `limit`. `pair_tokens`
is the tokenizer's pair encoding length and is not inferred by adding the two
individual counts.

The guard runs before constructing a BGE or NLI model. It partitions the same E5
candidate pool into fitting and over-cap pairs. If at least one pair fits, the
model is constructed once and only fitting pairs are scored. If no pair fits, no
model is constructed and the layer is run-level unavailable with reason
`token-cap`. The same preflight applies to real local artifacts; a missing or
invalid artifact also produces the existing unavailable result before model
construction.

E5 passages use the same 512-token check before E5 model construction or
inference. An E5 passage over the limit is an explicit retrieval input-cap
failure rather than a truncated candidate; BGE/NLI candidate states apply only
after a valid E5 candidate pool exists.

## Evidence States

### BGE

- A fitting, scored pair retains the existing `BGE_RECOMMENDED` or
  `BGE_FILTERED` status based on the existing symmetric raw-logit threshold.
- An over-cap pair receives per-candidate `BGE_TOKEN_CAP` evidence with no BGE
  score. It remains in the raw candidate collection, is absent from the BGE
  filtered collection, and remains unresolved and blocking.
- If at least one BGE pair fits, the run status remains `BGE_ENABLED`; cap
  candidates are not converted into run-level unavailable state.
- If no BGE pair can be scored, the run status is `BGE_UNAVAILABLE`. When the
  reason is `token-cap`, every affected candidate still carries
  `BGE_TOKEN_CAP`; the run summary does not replace candidate evidence.
- Artifact refusal or another no-score condition retains the existing
  `BGE_UNAVAILABLE` run summary and uses the existing unscored fallback rather
  than inventing a token-cap status.

### NLI

NLI follows the same rules independently with `NLI_RECOMMENDED`,
`NLI_FILTERED`, `NLI_TOKEN_CAP`, `NLI_UNAVAILABLE`, and the existing unscored
fallback. An NLI cap candidate has null NLI probabilities, remains visible, is
not placed in `nli_filtered_candidates`, and remains blocking.

Run-level `BGE_UNAVAILABLE` and `NLI_UNAVAILABLE` are reserved for a layer that
scored no pair. A partial token-cap run is never represented only by a run-level
unavailable status.

### Combined Evidence

The combined path is explicitly non-sequential:

1. E5 creates the raw candidate pool once.
2. BGE receives the eligible pool, checks its own tokenizer budget, and scores
   all fitting BGE pairs.
3. NLI independently receives the same eligible E5 pool, checks its own
   tokenizer budget, and scores all fitting NLI pairs.
4. The combined recommended set is the intersection of the enabled layers'
   recommended sets. A layer that is disabled is absent from the intersection;
   a layer that is unavailable or has a cap/unscored candidate does not disprove
   that candidate.

NLI never receives only the BGE survivors. A candidate recommended by one layer
but capped, unavailable, or unscored in the other remains visible and blocking.
Only a candidate filtered by every applicable scored layer can leave the
blocking recommendation set.

## Surface And Output Contracts

The human-readable duplicate and semantic-review output contains the following
for every token-cap candidate:

```text
token-cap: <BGE_TOKEN_CAP|NLI_TOKEN_CAP> complete semantic pair requires <pair_tokens> tokens (limit 512)
token-cap left: <left_tokens>, right: <right_tokens>
refactoring required: shorten or split the scenarios, then rerun semantic review
```

The machine-readable action is explicit. For example, the duplicate JSON
contract is exercised through:

```text
givn spec duplicates --format json
```

Participating JSON output keeps the existing E5 candidate identity and appends
model evidence objects. A cap object contains the per-candidate status, token
counts, limit, null score/probability fields, and `refactoring_required: true`.
The run object contains the layer status and reason. A partial cap run has a
successful run status and per-candidate cap objects. An all-over-cap run has
run-level `BGE_UNAVAILABLE` or `NLI_UNAVAILABLE` with reason `token-cap` and
still emits per-candidate cap objects.

Token-cap candidates stay in the normal `candidates` collection, not a filtered
collection. Blocking semantic reports retain their normal unresolved candidate
block, record `BGE_TOKEN_CAP`/`NLI_TOKEN_CAP` admission values and the exact
counts, and do not assign a classification. The report remains non-zero while
the candidate is unresolved. Advisory Explore and advisory review retain their
advisory exit behavior while exposing the same evidence.

Token-cap fields are added through the existing opt-in BGE/NLI/combined output
schema-version mechanism. E5-only output remains its existing contract. These
output schema versions are distinct from the serializer content hash; no
separate serializer version is introduced.

## Guidance Contract

The canonical `assets/instructions/specs.md` retrieval section must define:

- complete serialization and actual-tokenizer cap semantics;
- `BGE_TOKEN_CAP` and `NLI_TOKEN_CAP` as per-candidate, visible, unresolved,
  non-filtered, blocking evidence;
- run-level unavailable state only when that layer scores no pair;
- independent BGE/NLI scoring over the same E5 pool and intersection semantics;
- the distinction between token-cap, unscored, unavailable, and filtered states;
- the instruction to shorten or split scenarios and rerun, without automatic
  chunking, scenario edits, or verbosity inflation.

Derived guidance must carry the same contract without creating a competing
policy. This includes the spec command, generated spec skill, review guidance,
and configured agent-target mirrors. Guidance updates are part of the eventual
implementation acceptance, not a reason to add cross-product scenarios here.

## Architecture Impact

Expected implementation impact is limited to the existing retrieval and test
surfaces:

| Area | Planned responsibility |
|---|---|
| Retrieval module | Complete serializer, content-hash E5 refresh, actual-tokenizer budget guard, independent BGE/NLI application, cap evidence, JSON/text/report rendering, and additive schema versions. |
| Regular step definitions | Deterministic token-budget seam, exact 512/513 fixtures, JSON/report assertions, and no-load/no-inference assertions. |
| E2E step definitions | Real compiled CLI subprocess for the duplicate audit and blocking review inventory actions; deterministic local seams only. |
| Runner matrix | Route the active feature through the retrieval-enabled BGE/NLI matrix without network or model downloads. |
| Guidance assets | Update canonical retrieval/spec instructions and all derived copies listed above. |

No external service, browser, database, digital twin, or new runtime model
artifact is required for the regular or E2E tests. Production uses the existing
local model/tokenizer artifacts. No automatic chunking or scenario mutation is
planned.

## Deterministic Test Seam

The tests use a test-only token-budget provider selected only by the deterministic
fixture backend. It returns model-specific individual and pair counts for named
fixture endpoints, including exact `512` and over-cap `513` cases. It must not be
available on the production tokenizer path and must not alter production model
selection, serialization, or score behavior.

The seam proves:

- BGE and NLI use the same E5 candidate identity set;
- each layer checks its own tokenizer budget before model construction;
- a partial cap run scores fitting pairs and keeps cap candidates visible;
- an all-over-cap run performs zero model construction/inference;
- exactly 512 is scored normally and 513 receives the cap state;
- no regular or E2E test downloads a model, contacts a network service, or
  requires a multi-gigabyte inference artifact.

## Test Runner And E2E Boundary

The executable specification remains the Cucumber feature delta and is run
through `tests/features_runner.rs` in strict `.fail_on_skipped()` mode. New step
definitions begin with `unimplemented!()` during RED; no empty or no-op step
body is permitted.

Configured commands remain:

```text
verify.command: ./run-tests.sh
verify.e2e_command: ./run-tests.sh --e2e
```

The focused regular command uses the active feature path and deterministic
retrieval features, for example:

```sh
GIVN_FEATURES=givn/changes/semantic-token-cap-review/specs \
  GIVN_SPEC_EMBEDDER=stub \
  cargo test --features spec-retrieval,cross-encoder-reranker,nli-evidence \
  --test features_runner -- \
  --name "Permanent duplicate audit keeps partial BGE token-cap evidence per candidate"
```

The E2E command invokes the same compiled CLI in an isolated temporary project.
The two inventory actions remain the only `@e2e` scenarios in this change:

| Inventory entry | E2E scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run the permanent duplicate audit with complete semantic-pair token-cap evidence | Permanent duplicate audit keeps partial BGE token-cap evidence per candidate | CLI | Compiled `givn spec duplicates --format json` subprocess; stdout/JSON is the primary assertion. |
| run a blocking change review with a complete semantic-pair token-cap finding | Blocking change review keeps an all-over-cap candidate unresolved | CLI | Compiled `givn check review --change running-change` subprocess; exit status/CLI output is primary and the report is secondary evidence. |

The regular scenarios cover NLI independence and the exact boundary without
creating an E2E scenario for each model or retrieval surface. The active feature
file remains `@wip` until each scenario has its own RED/GREEN/REFACTOR cycle.

## Scenario Execution Order

The executable feature is the source of scenario titles. The task list executes
the two non-E2E scenarios first, followed by the two `@e2e` scenarios, while
preserving their source-level behavior and exact titles:

1. `Permanent duplicate audit preserves BGE evidence when NLI reaches the cap`
   (regular).
2. `Exactly 512 tokens keeps normal second-stage behavior` (regular).
3. `Permanent duplicate audit keeps partial BGE token-cap evidence per candidate`
   (E2E).
4. `Blocking change review keeps an all-over-cap candidate unresolved` (E2E).

Each title is used verbatim in its atomic scenario commit message and in the
focused runner invocation recorded in `tasks.md`.
