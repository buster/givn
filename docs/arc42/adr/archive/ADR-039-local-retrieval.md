---
status: superseded by ADR-069
date: 2026-08-14
decision-makers:
  - givn maintainers
consulted:
  - retrieval implementation and local model setup
informed:
  - authors and maintainers using semantic evidence
---

# Provide local model-backed scenario retrieval

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

- [x] **Hard to reverse** - the index schema, model cache, and feature boundary are persistent project interfaces.
- [x] **Shapes quality attributes** - local retrieval changes authoring recall, runtime, and dependency footprint.
- [x] **Structural** - `spec-retrieval`, E5 vectors, and `.spec-index.json` form a retrieval subsystem boundary.
- [x] **Must be shared** - authors must distinguish advisory retrieval from blocking review and know its feature requirement.

## Context and Problem Statement

Deterministic fingerprints miss some paraphrases. Authors need local related-scenario evidence without adding a hosted service or forcing the default binary to link ONNX Runtime.

## Decision Drivers

- local and offline-capable operation after model setup
- incremental permanent index refresh
- multilingual scenario retrieval
- feature-gated dependency footprint

## Considered Options

- Rely only on deterministic overlap.
- Shell out to an external Python indexer.
- Embed local multilingual E5 retrieval behind an opt-in Cargo feature.

## Decision Outcome

Chosen option: **Use local multilingual E5 embeddings with a persistent permanent-scenario index behind `spec-retrieval`**. The index stores scenario identities, complete semantic content hashes, and vectors; `spec search` remains advisory. The strict E5 gate is a separate feature decision. Model artifacts are shared user data in the XDG cache by default, while the index remains project-local.

### Consequences

- Retrieval-enabled builds acquire or reuse local model/tokenizer artifacts in
  the XDG cache by default; the default feature-free build fails explicitly for
  retrieval commands.
- Index refresh reuses matching identity/hash entries and removes deleted permanent entries.
- Local model runtime, cache size, and token limits are operational concerns.

### Confirmation

Retrieval, exploration, index-refresh, missing-model, and feature-free scenarios assert local setup, ranking, hash reuse, and explicit unavailable behavior.

## Pros and Cons of the Options

### Local E5 feature

- Good, because paraphrase evidence is available without a hosted service.
- Bad, because native dependencies and model artifacts are optional but heavy.

### Deterministic overlap only

- Good, because setup and runtime remain minimal.
- Bad, because paraphrases are invisible.

### External Python indexer

- Good, because Python model tooling is broad.
- Bad, because it adds a runtime and process boundary to a single-binary tool.

## More Information

The strict gate is [ADR-041](ADR-041-e5-uniqueness-gate.md); retrieval mechanics are in [chapter 8.5a](../../08-crosscutting-concepts.md#85a-local-scenario-retrieval).

## Amendment (2026-08-23)

The retrieval/index boundary is refined so model artifacts are not copied into
each project. Explicit semantic operations resolve E5 artifacts from
`$XDG_CACHE_HOME/givn/e5`, or `$HOME/.cache/givn/e5` when the XDG variable is
unset; explicit model/cache overrides remain supported. The project-local
`givn/.spec-index.json` continues to own only permanent scenario vectors and
cap metadata. This amendment does not change the feature gate, serialization,
or index schema.
