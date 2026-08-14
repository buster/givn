# arc42 Impact Assessment: spec-retrieval

This change adds two local CLI interactions and a persisted local retrieval
index. It affects the CLI context, building blocks, runtime view, cross-cutting
configuration, quality scenarios, risks, and glossary.

| # | Chapter | Affected | Reason |
|---|---|---|---|
| 1 | Introduction and Goals / Quality Requirements | Yes | Advisory semantic retrieval becomes an agent-authoring capability. |
| 2 | Architecture Constraints | Yes | The model/cache and setup-free CI constraints are new technical constraints. |
| 3 | Context and Scope | Yes | `givn spec index` and `givn spec search` are new CLI surfaces. |
| 4 | Solution Strategy | Yes | Local in-binary embeddings with advisory, non-blocking results are a new strategy. |
| 5 | Building-Block View | Yes | New CLI and retrieval/index blocks are added. |
| 6 | Runtime View | Yes | Index and search flows are new runtime sequences. |
| 7 | Deployment View | Yes | The default artifact remains unchanged, but the opt-in native runtime and local model cache alter the authoring build and installed footprint. |
| 8 | Crosscutting Concepts | Yes | Local cache, model setup, persistence, and non-blocking failure semantics are new. |
| 9 | Architecture Decisions | Yes | Local model/index and advisory search decisions require ADRs. |
| 10 | Quality Requirements | Yes | Index and top-result quality scenarios are added. |
| 11 | Risks and Technical Debt | Yes | Model download/cache availability and index staleness are new risks. |
| 12 | Glossary | Yes | Embedding, retrieval index, and advisory match become project terms. |

## Status

STATUS: DONE
