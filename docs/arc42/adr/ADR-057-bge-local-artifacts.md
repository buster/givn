---
status: accepted
date: 2026-08-18
decision-makers:
  - givn maintainers
consulted:
  - FastEmbed-rs API and local artifact requirements
informed:
  - local model maintainers and test authors
---

# Load BGE from preflighted local artifacts

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

- [x] **Hard to reverse** - the local artifact layout and loader determine the product's model deployment boundary.
- [x] **Shapes quality attributes** - preflight prevents hidden downloads and opaque model-load failures.
- [x] **Structural** - model, tokenizer, configuration, and optional external-data files form the BGE input boundary.
- [x] **Must be shared** - local and test runs must distinguish approved artifacts from stubs.

## Context and Problem Statement

The generic model constructor can download artifacts. Product BGE must use a known local artifact set after admission, and incomplete exports must fail before ONNX session/model construction.

## Decision Drivers

- no hidden download in the product path
- exact artifact validation
- deterministic tokenizer preflight
- local CPU execution

## Considered Options

- Use a download-capable model constructor.
- Trust the cache and let construction report errors.
- Validate local artifacts and use FastEmbed's user-defined loader.

## Decision Outcome

Chosen option: **Preflight local BGE artifacts and load them with `try_new_from_user_defined`**. The current preflight reads `model.onnx`, `tokenizer.json`, `config.json`, `special_tokens_map.json`, and `tokenizer_config.json`, and checks `model.onnx.data` when the ONNX file references it. The local loader uses a 512-token limit and four intra-op threads; missing/corrupt artifacts map to `BGE_UNAVAILABLE` before model loading.

### Consequences

- Artifacts live under `givn/.spec-reranker-cache` by default or a selected model directory.
- A local runtime failure remains distinct from failed benchmark approval.
- Tests use stubs or explicit missing-artifact paths and do not download model weights.

### Confirmation

Artifact scenarios assert valid local loading, missing/corrupt preflight failure, zero model-load attempts on refusal, and no network-dependent test path.

## Pros and Cons of the Options

### Preflighted user-defined loader

- Good, because the no-download boundary is explicit.
- Bad, because all support files and external weights must be provisioned correctly.

### Download-capable constructor

- Good, because first setup can be automatic.
- Bad, because offline behavior and reproducibility are unclear.

### Trust cache and construct

- Good, because implementation is smaller.
- Bad, because incomplete artifacts fail late and opaquely.

## More Information

See [ADR-056](ADR-056-bge-approval.md) and [chapter 7.1](../07-deployment-view.md#71-product-artifact).
