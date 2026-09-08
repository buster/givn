---
status: superseded by ADR-069
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

The generic model constructor can download artifacts. Product BGE must use a
known, validated artifact set after admission, and incomplete exports must fail
before ONNX session/model construction. The artifact set may be acquired by an
explicit semantic command into the shared XDG user cache; it must not be
downloaded implicitly by model construction or unrelated commands.

## Decision Drivers

- no hidden download in the product path
- exact artifact validation
- deterministic tokenizer preflight
- local CPU execution
- shared user-level cache rather than project duplication

## Considered Options

- Use a download-capable model constructor.
- Trust the cache and let construction report errors.
- Validate local artifacts and use FastEmbed's user-defined loader.

## Decision Outcome

Chosen option: **Resolve or acquire a complete BGE artifact set, preflight it,
and load it with `try_new_from_user_defined`**. An explicit semantic operation
uses the XDG cache by default and may populate it through the synchronous HF
Hub resolver. The preflight reads `model.onnx`, `tokenizer.json`, `config.json`,
`special_tokens_map.json`, and `tokenizer_config.json`, and checks
`model.onnx.data` when the ONNX file references it. The local loader uses a
512-token limit and four intra-op threads; missing/corrupt artifacts map to
`BGE_UNAVAILABLE` before model loading.

### Consequences

- Artifacts live under `$XDG_CACHE_HOME/givn/bge` or `$HOME/.cache/givn/bge` by
  default, or a selected model/cache directory.
- A local runtime failure remains distinct from failed benchmark approval.
- Tests use stubs or a deterministic local artifact-source fixture and do not
  contact the live model service.

### Confirmation

Artifact scenarios assert valid local loading, missing/corrupt preflight failure, zero model-load attempts on refusal, and no network-dependent test path.

## Pros and Cons of the Options

### Preflighted user-defined loader

- Good, because the validated loader boundary is explicit and shared cache data
  is not duplicated per project.
- Bad, because all support files and external weights must be acquired or
  provisioned correctly.

### Download-capable constructor

- Good, because first setup can be automatic.
- Bad, because it hides acquisition inside inference and makes offline behavior
  and reproducibility unclear.

### Trust cache and construct

- Good, because implementation is smaller.
- Bad, because incomplete artifacts fail late and opaquely.

## More Information

See [ADR-056](ADR-056-bge-approval.md) and [chapter 7.1](../07-deployment-view.md#71-product-artifact).

## Amendment (2026-08-23)

The original local-artifact boundary is retained after admission, but the
availability step now uses a validated, explicit-operation resolver. Missing
files may be downloaded into the shared XDG BGE cache before the existing local
loader runs. The project-local `givn/.spec-reranker-cache` is no longer the
default destination; explicit `GIVN_SPEC_BGE_MODEL_DIR` and
`GIVN_SPEC_BGE_CACHE` overrides remain supported.
