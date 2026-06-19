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
instructions/context/rules, delta+merge, Gherkin lint, state, skill generation).

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

**Rationale.** Explicit and portable; `givn lint` stays a pure Gherkin static
checker while the runner (`verify.command`) is unambiguous for agents and CI.

**Default value (refined — see ADR-021).** The shipped default `verify.command`
is a deliberate **sentinel** that is not a real runner:

```yaml
verify:
  command: "givn missing-testrunner"
```

`givn missing-testrunner` is a built-in subcommand that prints actionable guidance
("no test runner configured — set `verify.command` in `givn/config.yaml` to your
project's command, e.g. `cargo test`, `mvn test`, `npx cucumber-js ...`,
`behave`") and exits non-zero. This makes the missing configuration loud and
self-documenting instead of silently running nothing or guessing wrong.

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

## ADR-019: `givn lint` exit-code contract (renamed from `givn verify`)

**Decision.** The static Gherkin-check command is named **`givn lint`** (renamed
from `givn verify` to remove the collision with the `verify.command` config key —
see ADR-027). It exits **0** (clean), **1** (parse error), **2** (`@wip`/PENDING).
Parse error outranks wip outranks clean. "GREEN" is determined by the test runner
(`verify.command`), not by `givn lint`.

**Rationale.** Preserves the prototype's CI-friendly contract while making clear
this command checks spec well-formedness and never runs tests.

---

## ADR-020: Delta tag namespace is `@givn.*`

**Decision.** Tags are `@givn.delta`, `@givn.added`, `@givn.modified`,
`@givn.removed`. Renamed from the prototype's `@openspec.*` for the clean break.
Test-runner feature path env var is `GIVN_FEATURES` (default `givn/specs`).

---

## ADR-021: Default `verify.command` is a self-documenting sentinel

**Context.** A freshly initialised project has no idea how to execute the
project's step definitions (the runner is language-specific: cargo/maven/npm/
behave/...). Silently doing nothing, or guessing, both produce confusing
false-GREEN states.

**Decision.** The shipped default `verify.command` is `givn missing-testrunner`,
a built-in subcommand that:
- prints a clear message that no test runner is configured,
- shows per-language examples of what to put in `givn/config.yaml`
  (`cargo test`, `mvn test`, `npx cucumber-js "givn/specs/**/*.feature"`,
  `behave`, `go test ./...`, ...),
- exits with a non-zero status so CI and agents treat it as "not yet GREEN".

**Rationale.** Makes the one mandatory per-project decision impossible to miss,
turns a config gap into guidance, and keeps `givn lint` (static Gherkin check)
cleanly separate from running the actual test runner (`verify.command`).

**Consequences.** `missing-testrunner` is a real (hidden/utility) subcommand.
The generated `givn/config.yaml` documents `verify.command` thoroughly (ADR-022).

---

## ADR-022: Generated config ships fully commented with documented defaults

**Context.** "All config in one place" only helps if users can discover what is
configurable. With eject-on-demand (ADR-006), the artifact facets are pulled in
as needed — but the top-level `givn/config.yaml` is created by `init` and is the
primary knob surface.

**Decision.** `givn init` writes a `givn/config.yaml` in which **every supported
key is present but commented out**, each with:
- a short description of what it does,
- the effective default value shown (commented),
- allowed values / format notes,
- a cross-reference to the relevant arc42/README section where useful.

Only the minimum a project must set (notably `verify.command`) is written
uncommented (or written uncommented as the sentinel default with a prominent
"CHANGE ME" note).

**Rationale.** Self-documenting config; users override exactly what they want by
uncommenting and editing; no need to read external docs to discover options. The
file doubles as living documentation of the configuration surface.

**Consequences.** The config template lives in `assets/` and must be kept in sync
with the manifest schema. A test asserts that every documented key corresponds to
a real schema key (and vice versa) so the docs cannot drift.

---

## ADR-023: Configuration is versioned with an upgrade path

**Context.** A project initialised with givn 1.0.0 should be able to adopt
improvements introduced in 1.1.0 (new keys, changed defaults, new commented
documentation) without manual archaeology. Because all config lives in the
project repo, git is the conflict-resolution surface.

**Decision.**
- The generated `givn/config.yaml` records the givn version that wrote/last
  upgraded it, via a top-level key, e.g.:
  ```yaml
  # Managed by givn. Do not delete this line.
  givn_config_version: "1.0.0"
  ```
- A `givn upgrade` command re-renders the managed/commented scaffolding of
  `givn/config.yaml` (and other generated managed files) from the current
  binary's templates, **preserving the user's uncommented overrides**, and bumps
  `givn_config_version`.
- The upgrade is performed as ordinary file writes in the project repo, so the
  user reviews the result via `git diff` and resolves any conflicts there.
- Upgrades are idempotent and never silently discard user-set values; when a key
  was removed or renamed between versions, `givn upgrade` leaves the old value in
  place and emits a note (and, where possible, a migration hint) rather than
  deleting it.

**Rationale.** Gives a real version-to-version upgrade path while keeping git as
the trusted diff/merge tool. Honours "project files always win" (ADR-005): user
overrides are preserved; only the documentation/scaffolding layer is refreshed.

**Consequences.** Adds a `givn upgrade` command and a `givn_config_version`
marker. Managed-but-editable files need a clear convention distinguishing
user-owned regions (uncommented overrides) from givn-managed regions (the
commented documentation scaffold). See chapter 8.10.

**Scope note.** v1 targets `givn/config.yaml` and generated agent files
(via `givn skills sync`). Upgrading ejected artifact facets is left to the user
through `git diff` against a re-eject; a future enhancement may assist this.

---

## ADR-024: Artifacts (the workflow) are first-class configurable data

**Context.** The default lifecycle is propose → spec → design → tasks →
implement → review → archive. Projects legitimately need to vary this: skip the
review gate, make review optional, add a dedicated refactor step after
implementation, or insert custom artifacts (e.g. a risk assessment). Prior tools
hardcoded the workflow, forcing a source fork to change it.

**Decision.** The lifecycle is **entirely described by the manifest** (artifacts,
their `requires` graph, the `apply` gate) and is fully reconfigurable per project
via the overlay (ADR-010). The artifact schema gains:
- `optional: bool` — an artifact that may be skipped without blocking the lifecycle.
- `remove: true` — an overlay directive deleting a default artifact by id.

Order is derived from `requires` (a DAG), so changing dependencies reorders the
workflow. `apply.requires` independently controls which artifacts gate archive.

`givn` validates the effective graph: no cycles, no dangling `requires`, and
`apply.requires` may not reference a removed non-optional artifact; otherwise it
refuses with a clear error. Filesystem-derived state (ADR-014) is computed from
the effective artifact set, not a hardcoded list.

**Rationale.** Makes "add/remove/reorder steps" a config change, never a fork.
Directly satisfies the requirement to keep artifacts configurable (e.g. optional
review, extra refactor step).

**Consequences.** Manifest model adds `optional` and the `remove` overlay
directive; graph validation is required (Phase 2). Custom artifacts need a
project-provided template/instruction (or a minimal generated stub). See ch.8.2.

---

## ADR-025: Strict separation of specification and implementation

**Context.** Gherkin `.feature` files and proposals tend to accumulate
implementation detail (class/function names, frameworks, routes, step mechanics),
which couples the specification to one implementation and undermines the
"`.feature` is the durable source of truth" principle.

**Decision.** Enforce a hard boundary:
- **Specification layer** — `proposal` and `specs` (`.feature`) — describe WHAT
  and WHY (observable behaviour, domain language) and must be
  implementation-independent.
- **Implementation layer** — `design` and `tasks` (and any custom impl
  artifacts) — carry the HOW (technology, step definitions, runner, build steps).
- The **only** permitted implementation-flavoured content in the spec is an
  aspect that is itself an externally observable requirement (e.g. "response is
  valid JSON", "data persists across restart", "responds within 200 ms").
  Heuristic: if it would still hold under a reasonable reimplementation, it is a
  spec; otherwise it is design/tasks.

This is enforced through the **default artifact instructions**: proposal and spec
instructions explicitly forbid implementation detail; design and tasks
instructions explicitly own it. Instructions are cascade-resolved and ejectable,
so projects may adjust the wording.

**Rationale.** Keeps proposals and specs valid across reimplementations; makes
specs reviewable by domain experts; preserves the spec as the durable contract.

**Consequences.** The default template/instruction text for proposal and spec
(Phase 1) must state the prohibition and the narrow exception; design and tasks
text must explicitly capture the "how". Reviews should flag leakage of
implementation detail into the spec layer.

---

## ADR-026: Optional arc42 documentation artifact (addon)

**Context.** Beyond per-change artifacts, projects often want durable, evolving
architecture documentation. A natural fit is an artifact that creates and
maintains arc42 documents — e.g. running after the `design` step so that the
project-level architecture docs reflect the change. This must be opt-in and not
imposed on projects that do not want it.

**Decision.** Ship an **optional `arc42-docs` artifact** (an addon enabled via
config), realising the configurable-artifacts mechanism (ADR-024):
- It is **not** part of the default active gate; projects enable it by adding it
  to the overlay (and, if desired, to `apply.requires`).
- It `requires: [design]` by default, so it runs after design (the change's "how"
  is known) and keeps the durable arc42 docs in sync.
- It maintains **project-level** arc42 documents (the living architecture of the
  whole system), distinct from per-change artifacts. Target location is
  configurable; default `docs/arc42/` (one file per chapter, matching this very
  project's convention).
- It ships with templates for the 12 arc42 chapters and an index, plus an
  instruction to **create missing chapters and update affected ones** based on
  the change's proposal/spec/design — not to rewrite everything each time.
- Like all artifacts, its templates and instructions are cascade-resolved and
  ejectable; the chapter set and output path are configurable.

**Rationale.** Gives teams first-class, maintained architecture docs as a
composable step, demonstrates the addon/extension model, and stays fully optional.

**Consequences.** Adds embedded assets: `assets/addons/arc42/` (chapter
templates + index + instruction). Adds an "addons" notion in docs/config: an
addon is simply a pre-packaged artifact (or set of artifacts) the user enables
via overlay. The arc42 docs it manages are durable (live under `docs/`), updated
across changes, and are not archived per-change. See ch.8.11.

---

## ADR-027: Lint command renamed `givn verify` → `givn lint`; `verify.command` kept

**Context.** The prototype had a `givn verify` command (static Gherkin check) and
a `verify.command` config key (the command that runs the actual tests). Same word
"verify" for two different operations — and "verify" implies running tests, which
the command does **not** do. This caused real confusion.

**Decision.**
- Rename the static-check command **`givn verify` → `givn lint`**. It checks
  `.feature` syntax, `@wip`/PENDING, and delta tags only; it runs no tests.
- **Keep** the config key **`verify.command`** as the command that runs the actual
  tests (default sentinel `givn missing-testrunner`).
- There is intentionally **no** `givn verify` or `givn test` command: givn does
  not execute the test suite (ADR-014); the agent/CI invokes `verify.command`.

**Rationale.** Two distinct words for two distinct operations removes the
collision. `lint` clearly signals "static check, no execution"; `verify.command`
clearly signals "the command that verifies behaviour by running tests".

**Consequences.** Module `cli::verify` → `cli::lint` (`verify.rs` → `lint.rs`).
All docs/skills/instructions reference `givn lint` for the static check and
`verify.command` for the runner. The `verify.command` key name is unchanged, so
existing config semantics are preserved.
