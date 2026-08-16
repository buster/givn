# NLI Experiment Benchmark

This document records the NLI comparison branch against the existing
`cross-encoder-reranker` branch. It is an experiment document, not a givn
change. No permanent `.feature` file was modified.

## Implementation

The branch adds an opt-in backend to the existing review path:

```sh
GIVN_SPEC_RERANKER=nli
GIVN_SPEC_NLI_MODEL_DIR=/path/to/exported-model
givn spec review --change cross-encoder-reranker
```

The candidate generator is unchanged. It retrieves the same top-five embedding
neighbours and evaluates each candidate in both directions. The NLI model emits
three logits. The implementation maps labels by `config.json:id2label`, applies
softmax to each direction, and applies softmax to the average of the two logit
vectors for the symmetric summary.

The mode is diagnostic only:

- it does not apply the existing `0.95` embedding gate;
- it does not apply the BGE reranker raw-logit gate;
- it does not produce an automatic `DUPLICATE`, `VARIANT`, or `BOUNDARY`
  decision;
- a successful inference run exits zero;
- missing model artifacts fail explicitly.

`GIVN_SPEC_RERANKER=nli-stub` exercises the output path without a model. The
default `GIVN_SPEC_RERANKER` behavior remains the existing reranker path.

## Model Artifact

Model:

```text
MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7
```

The local ONNX artifact was exported with the Optimum sequence-classification
export path:

```sh
optimum-cli export onnx \
  --model MoritzLaurer/mDeBERTa-v3-base-xnli-multilingual-nli-2mil7 \
  --task text-classification \
  /tmp/opencode/nli-model
```

Observed artifact:

| Property | Value |
|---|---:|
| ONNX model size | 1.1 GB |
| Labels | `entailment`, `neutral`, `contradiction` |
| Joint maximum length | 512 tokens |
| Export reference-vs-ONNX max logit difference | `2.7e-5` |
| Model directory | `/tmp/opencode/nli-model` |

Artifact hashes from this run:

```text
model.onnx          141998dbc1f3f282a9432d2f6678a9b8116c9e101b2493671dd7c0fd48016626
tokenizer.json      ac9bd80d04d21b4df95917bdf7c750cbea28acc30a4462c76ca2b9c86d371863
config.json         14877ff8e4699989ef08bed8fa570a803e1caef0cc0a35cb4372856deabba958
tokenizer_config   fc66a38a04a7e271944043a7d09ca0d6e30be0ed3c31c24b1d8bef842fcc1ea6
```

The model directory is intentionally not committed. A reproducible production
artifact would need a pinned Hub revision, SHA-256 hashes, and an explicit model
distribution policy.

## Corpus Run

The run used the same repository corpus, active change, embedding index, and
top-five candidate generation as the earlier reranker benchmark:

```sh
/usr/bin/time -f 'NLI_REVIEW_SECONDS=%e' \
  env -u GIVN_SPEC_EMBEDDER \
    GIVN_SPEC_RERANKER=nli \
    GIVN_SPEC_NLI_MODEL_DIR=/tmp/opencode/nli-model \
    GIVN_SPEC_NLI_REPORT_LIMIT=1000 \
    target/debug/givn spec review --change cross-encoder-reranker
```

Result:

| Measurement | NLI | Existing BGE reranker branch |
|---|---:|---:|
| Permanent scenarios | 208 | 208 |
| Candidate pairs | 839 | 839 |
| Full review duration | 552.60 s | 750.67 s |
| Final decision threshold | none; diagnostic | raw logit `3.0` |
| Reported label distribution | 171 entailment / 218 neutral / 450 contradiction | 7 candidates above boundary |

The NLI run is faster than the BGE reranker run on this machine, but still takes
approximately nine minutes for 839 candidate pairs. The output distribution is
not a duplicate distribution: `contradiction` is the largest class even though
many candidates are merely unrelated or test-structure neighbours.

## Known Pairs

The symmetric probabilities below are from the Rust ONNX run. The order is
`entailment`, `neutral`, `contradiction`.

| Pair | Embedding | NLI symmetric probabilities | Highest label | Manual meaning |
|---|---:|---|---|---|
| Placeholder COMMIT hash scenarios | `0.9664` | `0.1034 / 0.0670 / 0.8296` | contradiction | True duplicate |
| Uninitialized `status` versus `new` | `0.9693` | `0.2989 / 0.4370 / 0.2641` | neutral | Same rejection invariant; command variant to merge/parameterize |
| Parse error stderr versus JSON | `0.9634` | `0.0238 / 0.9009 / 0.0753` | neutral | Same invariant; output variant to merge/parameterize |
| Skipped-artifact status scenarios | `0.9660` | `0.0463 / 0.9285 / 0.0252` | neutral | Same status invariant; likely merge/parameterize |
| Coverage example versus badge placement | `0.9599` | `0.5515 / 0.4432 / 0.0053` | entailment | Same README coverage area; overlapping variant |
| Invalid dot id versus path-traversal status id | `0.9597` | `0.2387 / 0.5258 / 0.2355` | neutral | Similar validation family; distinct input/command cases |

The model therefore does not provide the required monotonic signal:

- the true duplicate is predominantly `contradiction`;
- the parse pair that should be merged is predominantly `neutral`;
- the skipped-artifact pair that should be merged is even more strongly
  `neutral`;
- the coverage variant is split between `entailment` and `neutral`;
- the invalid-id pair is also `neutral`.

No single NLI label or obvious fixed probability threshold separates these
manual categories. The model is answering a natural-language premise/hypothesis
question, not whether two Gherkin scenarios belong to the same executable
invariant and production boundary.

## Comparison Decision

NLI is a better diagnostic feature set than one relevance logit because it exposes
three different relationship signals and order asymmetry. It is not a replacement
for the existing reranker gate, and it is not an off-the-shelf duplicate
classifier.

The branch is useful for a later supervised model. The directional and symmetric
NLI values can become features alongside deterministic Gherkin facts such as
command, exit status, output channel, and persistence/transport assertions. A
future four-class model still requires manually labeled `duplicate`, `variant`,
`boundary`, and `unrelated` pairs with hard negatives grouped by invariant.

## Verification

Passed on the experiment branch:

```text
cargo check --features spec-retrieval
cargo check --no-default-features --locked
cargo test --features spec-retrieval --lib
cargo test --no-default-features --lib
cargo clippy --workspace --all-targets --features spec-retrieval -- -D warnings
targeted cross-encoder feature suite: 3 features, 7 scenarios, 41 steps
real NLI Rust review: 839 pairs, exit 0
```

The two library suites reported 144 and 142 passing tests respectively.

The full repository feature suite remains at the known baseline for the
`cross-encoder-reranker` branch: one existing review-disposition scenario still
expects advisory behavior while the branch's reranker gate correctly fails it.
The NLI backend is not selected by that suite and did not introduce that failure.
