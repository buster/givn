# 4. Solution Strategy

This chapter summarises the key strategic decisions. Full rationale per decision
is in chapter 9 (Architecture Decisions). Mechanics are in chapter 8
(Cross-cutting Concepts).

## 4.1 Strategy at a glance

| Concern | Strategy |
| --- | --- |
| Language & packaging | Single static **Rust** binary; defaults embedded via `rust-embed`. |
| Configuration model | **CSS-style cascade**: embedded defaults ← project `givn/` files win, per-facet. |
| Discovery of defaults | **`eject` on demand** — project dir starts empty of overrides. |
| Artifact graph | **YAML manifest**, embedded default graph + project **overlay/patch**. |
| Spec format | **`.feature` is the spec** — delta tags + merge-on-archive; no `spec.md`. |
| TDD | **Scenario-by-scenario** RED/GREEN/REFACTOR loop. |
| Coverage | **Completeness of gap classification**, not a percentage threshold. |
| State | **Filesystem-derived** — file presence determines lifecycle state. |
| Agent integration | `AGENTS.md` is user-owned (never written); generated skills go to the configured agent targets (`.opencode`, `.agents`, `.claude` — selectable at init, default opencode, see ADR-036). |
| Test running | Manifest-declared `verify.command` runs tests; `givn lint` only statically checks Gherkin (separate concerns). |
| Architecture conformance | When the `arc42` addon is enabled, completion review cross-checks durable Arc42 facts against design, tasks, and the completed implementation. |
| Scenario retrieval | Local multilingual embeddings and a persisted content-hash index retrieve candidates; permanent-scenario exploration and interactive search remain embedding-only and advisory, while a local multilingual pair reranker is reserved for experimental review. |
| Current-change exploration | Merge active `Added`/`Modified` delta candidates into a request-local exploration corpus; keep the persisted catalogue permanent-only and label each result's source additively. |
| Distribution | Release binaries + curl\|sh + brew + cargo; `cargo-dist`. |

## 4.2 How the strategy meets the top quality goals

### Configurability without forking (Goal 1)

The cascade engine resolves each *facet* (template, instructions, deps, skill
template) independently: a project file in `givn/` wins; otherwise the embedded
default is used. The artifact graph itself is customised by a YAML overlay that
patches the embedded default graph (add/modify/remove artifacts, reorder, change
deps, set `apply.requires`, set `verify.command`). Nothing requires editing the
binary's source.

### Installability (Goal 2)

A single Rust binary with no runtime, distributed through four channels (release
binary, curl\|sh, Homebrew, cargo). `cargo-dist` builds multi-platform artifacts.

### Upgradeability (Goal 3)

Defaults live only in the binary (the fallback layer). Projects own only the
files they ejected. Upgrading the binary changes the fallback, never the project
files. `givn skills sync` regenerates agent files from current templates on demand.

## 4.3 Key patterns

- **Cascade / override-file resolution** — the central pattern (chapter 8.1).
- **Overlay/patch merge** for the manifest (chapter 8.2).
- **Delta + merge** for Gherkin specs (chapter 8.3).
- **Filesystem-as-state** (chapter 8.6).
- **Template rendering with multi-target emit** for skills (chapter 8.7).
- **Arc42-to-change conformance review** — post-implementation evidence
  cross-check for Arc42-enabled projects (chapter 8.14).

Repository workflow contracts are verified at their owning boundary: Givn's
own acceptance fixtures invoke the public CLI, while consumer repositories
retain only their domain-specific permanent specifications. This keeps the
deterministic gate implementation and its regression evidence in one place.

The first semantic authoring slice deliberately uses the existing embedding
representation and content-hash refresh. It does not introduce a second model,
project-specific text heuristic, or a blocking decision into the exploratory
path.

The current-change slice extends that same advisory path without broadening the
persisted catalogue. Permanent entries are refreshed as before; active delta
vectors exist only for the request, and lifecycle identity filtering prevents a
scenario from being compared with its own source or projected permanent target.
