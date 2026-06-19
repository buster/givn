# 9. Architecture Decisions

Every decision below was settled during the design Q&A. Status of all: **Accepted**.
These are the binding decisions — do not silently change them during implementation.

---

## ADR-001: Build a standalone tool, not extend OpenSpec

**Context.** The workflow was prototyped as a fork of OpenSpec (TypeScript, npm).
Pain points: too much hardcoded, npm + root install, schema customisation requires
forking TS source, hardcoded skill templates overwritten on update, config scattered
across three layers.

**Decision.** Extract the essence into a new standalone tool for the broader community.

**Consequences.** Clean slate; freedom to fix the install/config model; loses
nothing essential (lifecycle, pluggable artifacts, graph, templates,
instructions/context/rules, delta+merge, verify, state, skill generation).

---

## ADR-002: Implementation language is Rust

**Context.** Need a single binary, broad installability, correctness for merge/state.
Candidates: Rust, Go, Zig, Python(bundled), Nim, Crystal.

**Decision.** **Rust.**

**Rationale.** Small static binaries, strong type system for merge/state logic,
author familiarity, excellent CLI/serde ecosystem and distribution tooling
(`cargo-dist`). Go was the main alternative (easier contributor onboarding) but
correctness and familiarity won.

---

## ADR-003: Tool name is `givn`

**Context.** Need a short, memorable, fast-to-type, on-theme, available name.
Shortlist checked against crates.io, GitHub, Homebrew. Most short names
(`gild`, `canon`, `brine`, `dill`, `vine`, `plumb`) were squatted on crates.io;
`givn` and `gherk` were fully free.

**Decision.** **`givn`** (from Gherkin's `Given`). Tagline: *"from Given to done."*

**Consequences.** Binary, crate, and config directory all named `givn`. Verified
free on crates.io (HTTP 404) and Homebrew (404).

---

## ADR-004: Single static binary; no npm, no root, no runtime

**Decision.** Ship one statically-linked executable; install requires no npm, no
root/sudo, no language runtime.

**Rationale.** Direct fix of the prototype's worst install friction; maximises
adoption.

---

## ADR-005: Customisation via override files (CSS cascade)

**Context.** Options were (A) override files / cascade, (B) one monolithic config
file, (C) fixed lifecycle with hooks.

**Decision.** **(A) Override files, CSS-style cascade**: embedded defaults are the
fallback layer; project files in `givn/` win.

**Rationale.** Zero-config works out of the box; override only what you care about;
binary updates never clobber project files.

---

## ADR-006: Discover defaults via `eject` on demand

**Context.** Options: (A) `eject` on demand, (B) `init` writes ALL defaults, (C) hybrid.

**Decision.** **(A) `eject` on demand.** The project dir starts with no overrides;
`givn eject <name>` materialises a default for editing.

**Rationale.** Keeps the project dir minimal; users manage only what they changed.

---

## ADR-007: Per-facet override granularity

**Context.** Smallest overridable unit: whole artifact bundle vs each facet.

**Decision.** **Each facet** (template / instructions / deps) overridable
independently.

**Consequence.** Overrides are addressable per facet (e.g.
`givn/artifacts/<id>/template.*`, `givn/artifacts/<id>/instructions.md`); the
config cannot be one monolithic file per artifact.

---

## ADR-008: Config directory is a visible `givn/`

**Decision.** The project config directory is named after the tool and is
**visible** (`givn/`, not a dotdir).

**Rationale.** Discoverability for newcomers browsing the repo.

---

## ADR-009: Manifest is YAML

**Context.** Options: TOML, YAML, KDL.

**Decision.** **YAML** for the artifact-graph manifest.

**Rationale.** Familiar (the prototype used `schema.yaml`), handles nesting well,
comments allowed. Note (TC-6): `serde_yaml` is deprecated → use the maintained
fork **`serde_yaml_ng`**.

---

## ADR-010: Manifest customised by overlay/patch

**Context.** Options: (A) eject and fully replace the manifest, (B) overlay/patch
merged onto the embedded default graph.

**Decision.** **(B) Overlay/patch.** `givn/config.yaml` declares only
additions/changes/removals, merged onto the embedded default graph.

**Rationale.** Projects keep benefiting from default-graph improvements on upgrade
while still customising.

---

## ADR-011: The `.feature` file IS the spec

**Decision.** No `spec.md`. The `.feature` file is the delta, the executable test,
and the permanent archived record. Deltas use `@givn.*` tags; tags are stripped on
archive into clean Gherkin.

**Rationale.** Single source of truth; eliminates the duplicate-format weight of
the prototype.

---

## ADR-012: Coverage gate is completeness of classification, not a percentage

**Decision.** The review phase classifies every uncovered line as dead code /
missing test / hard-to-test and resolves each. There is **no** percentage threshold.

**Rationale.** A number is a side effect; the real goal is that every externally
visible behaviour is specified, exercised, and covered — or explicitly justified.

---

## ADR-013: Agent integration — AGENTS.md + neutral skills with mirrors

**Context.** Two layers exist: rules/context (well standardised) and invokable
skills (emerging convention). Research findings:
- `AGENTS.md` is the ratified standard for rules/context (Linux Foundation; ~40
  tools; 60k+ repos).
- `SKILL.md`-in-a-folder is an emerging convention read from three parallel paths:
  `.claude/skills/`, `.agents/skills/` (vendor-neutral), `.opencode/skills/`.
- Slash commands are not standardised (per-tool).

**Decision.**
- Rules/context → **`AGENTS.md`** at repo root.
- Skills → primary neutral path **`.agents/skills/<name>/SKILL.md`** (ADR-013a)
  with **built-in mirrors for `.claude/skills/` and `.opencode/skills/`** (ADR-013b).
- Skill templates are ejectable so users can retarget other tools.

**Rationale.** Maximise reach via the neutral standard while supporting the two
most common tools out of the box; remain extensible without shipping every target.

---

## ADR-014: State is derived from the filesystem

**Context.** Options: (A) filesystem-derived, (B) explicit state file.

**Decision.** **(A) Filesystem-derived.** Presence of files determines lifecycle
state; no separate state file.

**Rationale.** No drift, git-friendly, trivially correct.

---

## ADR-015: Test-runner command declared in the manifest

**Context.** Options: (A) manifest-declared `verify.command`, (B) auto-detect with
override, (C) givn only lints, runner entirely external.

**Decision.** **(A)** `verify.command` is declared in the manifest, language-agnostic.

**Rationale.** Explicit and portable; `givn verify` stays a pure Gherkin linter
while the runner is unambiguous for agents and CI.

---

## ADR-016: Repository location and version control

**Decision.** New project at `~/projects/givn`; local `git init` only, no remote
for now.

---

## ADR-017: Clean break from OpenSpec; migration deferred

**Decision.** No backward-compatible reading of `openspec/` directories or
`@openspec.*` tags. A `givn migrate` command is a deferred nice-to-have.

---

## ADR-018: MVP scope includes the full stack

**Decision.** The first version includes core engine + lifecycle + skills + install
(not a reduced core-only milestone).

---

## ADR-019: `verify` exit-code contract

**Decision.** `givn verify` exits **0** (GREEN), **1** (parse error), **2**
(`@wip`/PENDING). Parse error outranks wip outranks clean.

**Rationale.** Preserves the prototype's CI-friendly contract.

---

## ADR-020: Delta tag namespace is `@givn.*`

**Decision.** Tags are `@givn.delta`, `@givn.added`, `@givn.modified`,
`@givn.removed`. Renamed from the prototype's `@openspec.*` for the clean break.
Test-runner feature path env var is `GIVN_FEATURES` (default `givn/specs`).
