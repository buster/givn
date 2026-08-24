# Arc42 Assessment: semantic-model-download

## Chapter Impact

| # | Chapter | Affected | Assessment |
|---:|---|---|---|
| 1 | Introduction and Goals | Yes | Adds model availability, shared-cache, and actionable-failure goals to the functional and non-functional requirements. |
| 2 | Architecture Constraints | Yes | Establishes XDG user-cache storage, explicit override precedence, and the no-download-on-implicit-use constraint. |
| 3 | Context and Scope | Yes | Adds the model artifact source and user cache as a boundary while keeping hosted inference out of scope. |
| 4 | Solution Strategy | Yes | Adds cache-aware acquisition before tokenizer/model use and separates project index state from user model state. |
| 5 | Building Block View | Yes | Adds the artifact resolver responsibility and direct HF Hub dependency to the retrieval building block. |
| 6 | Runtime View | Yes | Documents cold-cache acquisition, validation, model construction order, and missing-index diagnostics. |
| 7 | Deployment View | Yes | Replaces project-local default model caches with XDG user-cache role directories and documents overrides. |
| 8 | Crosscutting Concepts | Yes | Updates filesystem layout, cache ownership, artifact validation, error handling, and retrieval setup mechanics. |
| 9 | Architecture Decisions | Yes | Amends ADR-057; no parallel ADR is created because the model deployment boundary already has a governing decision. |
| 10 | Quality Requirements | Yes | Adds cold-start availability, cache reuse, invalid-artifact safety, and missing-index guidance scenarios. |
| 11 | Risks and Technical Debt | Yes | Adds shared-cache corruption, disk/network cost, and incomplete-XDG risks with mitigations. |
| 12 | Glossary | Yes | Adds XDG model cache, role cache, artifact resolver, and missing-index diagnostic terms. |

## ADR Qualification

The change contains two refinements of existing retrieval/deployment
boundaries. They are recorded separately; NLI path mechanics and the shared
diagnostic are canonical design/specification concerns and do not create
parallel ADRs.

### E5 Cache Ownership

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "AMEND_ADR",
  "canonical_artifact": null,
  "target_adr": "ADR-039",
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Keep project-local model caches and require manual provisioning",
      "Acquire validated artifacts into a shared user cache with explicit local overrides"
    ],
    "architectural_impact": [
      "The retrieval model deployment boundary changes from project-local defaults to XDG user-owned cache data",
      "The permanent index remains a separate project-owned persistence boundary"
    ],
    "durable_consequence": [
      "Cache paths and override precedence affect all semantic commands and multiple projects",
      "Reversal would require cache migration or broad deployment/documentation changes"
    ],
    "lower_level_artifact": [
      "The durable E5 cache ownership is not fully owned by a feature scenario or local implementation detail"
    ],
    "existing_adr_check": [
      "ADR-039 governs local E5 retrieval and is the direct amendment target",
      "ADR-056, ADR-057, and ADR-058 were checked for admission, BGE artifact, and NLI boundary overlap"
    ]
  }
}
```

### BGE Acquisition Boundary

```json
{
  "qualification": "QUALIFIED",
  "alternatives": "PASS",
  "architectural_impact": "PASS",
  "durable_consequence": "PASS",
  "lower_level_artifact": "PASS",
  "existing_adr_check": "PASS",
  "must_be_shared": "SUPPORTING",
  "routing": "AMEND_ADR",
  "canonical_artifact": null,
  "target_adr": "ADR-057",
  "replacement_adr": null,
  "evidence": {
    "alternatives": [
      "Require every project to provision a flat BGE artifact set manually",
      "Acquire a validated snapshot into shared user cache before the existing local loader"
    ],
    "architectural_impact": [
      "BGE acquisition becomes a separate pre-loader boundary",
      "BGE approval remains independent from artifact availability"
    ],
    "durable_consequence": [
      "The cache layout, selector semantics, and validation boundary affect every BGE surface",
      "Reversal would require cache and deployment migration across projects"
    ],
    "lower_level_artifact": [
      "The BGE artifact deployment boundary is not fully represented by one local implementation or scenario"
    ],
    "existing_adr_check": [
      "ADR-057 governs validated BGE local artifacts and is the direct amendment target",
      "ADR-056 governs approval and remains unchanged; ADR-058 governs independent NLI evidence"
    ]
  }
}
```

NLI snapshot-versus-flat resolution, explicit-operation download policy, and
missing-index diagnostics are owned by `design.md` and the executable feature
specification. They do not qualify as additional independent ADRs in this
change. Both amendments retain BGE approval, the local loader, the existing
evidence contracts, and the feature-free fallback.

## Status

STATUS: DONE
