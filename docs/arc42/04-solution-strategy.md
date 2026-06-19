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
| Agent integration | `AGENTS.md` for rules; `.agents/skills` (neutral) + claude/opencode mirrors. |
| Test running | Manifest-declared `verify.command` runs tests; `givn lint` only statically checks Gherkin (separate concerns). |
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
