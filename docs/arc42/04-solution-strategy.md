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
| Scenario retrieval | Local multilingual E5 embeddings and a persisted permanent-only content-hash index retrieve candidates; the semantic report admits only scores `>= 0.9500`, while permanent-scenario exploration and the existing advisory `givn spec review` contract remain unchanged. The strict similarity gate itself is enabled by `spec-similarity-gate`, not by `spec-retrieval` alone. |
| Permanent duplicate audit | `givn spec duplicates` scans every permanent pair, resolves exact and existing-Fingerprint structural relationships before advisory E5 candidates, emits all ranked results through one fixed text/JSON contract, and computes the report in memory without reading, refreshing, or writing the persisted index. |
| Current-change exploration | Merge active `Added`/`Modified` delta candidates into a request-local exploration corpus; keep the persisted catalogue permanent-only and label each result's source additively. |
| Semantic review gate | A dedicated entrypoint called only by blocking `givn check review` completely regenerates the change-local report in scope/score/endpoint order; carry a classification only when the candidate id and both complete endpoint evidence hashes are unchanged, parse prior entries strictly, atomically replace the report, and block unresolved classifications without editing feature files. It does not replace or suppress the existing strict gate: a `spec-similarity-gate` build runs that gate afterward. |
| Semantic remediation gate | Before returning PASS, verify every prior resolved classification against a read-only projected tree. Recompute prior pairs even below `0.9500`; require duplicate removal, a parameterized variant contract, or changed observable facts for a boundary below the threshold. |
| Retrieval compatibility | A binary without `spec-retrieval` returns the existing unavailable error from blocking check review and creates no semantic report; `spec-retrieval` alone does not enable the strict similarity gate, which requires `spec-similarity-gate`; the advisory `givn spec review` behavior remains separate. |
| Historical evidence | The generated report remains in the change directory and is retained by the existing archive move under `givn/archive/<id>/`; archive implementation is not changed by this slice. |
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

The semantic authoring and review slices deliberately reuse the existing E5
embedding representation, binary-fixed `0.9500` boundary, and content-hash
refresh. The blocking report adds no BGE or NLI evidence and does not introduce
automatic semantic decisions. The advisory exploratory path remains non-blocking
and the existing `givn spec review --change <id>` behavior remains unchanged.

The permanent duplicate audit is a separate advisory surface over the complete
permanent corpus. It reuses the existing Gherkin parser, deterministic
`Fingerprint` masked assertion shape/subset and fact model, and E5 passage
embeddings, but keeps the audit request-local so an inspection cannot alter
permanent specs, active changes, archive contents, or the persistent index.
Missing `givn/specs/` is an empty successful corpus; malformed or unreadable
features fail with their path. Its complete result is not truncated to the
interactive top-five view, and semantic scores are evidence rather than
automatic duplicate decisions.

The audit's stable text header and candidate grammar are shared conceptually
with the stable JSON fields and order: endpoint identities, kind, relation,
structural evidence, distinguishing facts, contradiction flag, and score/null
value. The relation enum includes `identical_behavior`,
`same_assertion_shape`, both direction-specific assertion-subset relations,
and `semantic_similarity`. The raw score is thresholded before four-decimal
serialization, so `0.9500` is admitted and `0.9499` is not. A semantic
candidate has null structural evidence.

Each eligible scenario is embedded once and remaining pairwise cosines are
computed in memory. The current baseline is approximately 288 scenarios and
41,328 pairs; a bounded/indexed approach is required before 500 scenarios or
approximately 124,750 pairs. Labeled fixtures record precision and recall at
the threshold, while the command remains advisory.

The current-change slice extends that same advisory path without broadening the
persisted catalogue. Permanent entries are refreshed as before; active delta
vectors exist only for the request, and lifecycle identity filtering prevents a
scenario from being compared with its own source or projected permanent target.
The semantic report reuses the active-delta corpus for two explicit scopes,
excludes only a Modified scenario's own projected target rather than its whole
target file, records complete source-ordered evidence and Fingerprint facts, and
keeps all feature-file and permanent-spec edits with the agent. Its Markdown wire
grammar fixes headers, field order, compact JSON values, `null` absence, final
newline, score/hash formats, and the absence of unknown/timestamp/environment
fields. A malformed or incompatible index is treated as stale and rebuilt from
the permanent specifications before report generation continues; its persisted
entries remain permanent-only and contain no `Delta` state, delta vectors,
classifications, or report state.

The remediation slice adds a second, prior-entry verification path after report
generation. It resolves prior endpoint identities against an in-memory projected
tree, distinguishes exact matches, changed evidence, disappearance, and no
match, and only then accepts a classification whose observable contract is
satisfied. This prevents a vanished or below-boundary candidate from being
treated as an automatic resolution.
