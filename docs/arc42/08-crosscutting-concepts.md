# 8. Cross-cutting Concepts

This is the heart of the design. Read this chapter before implementing.

## 8.1 The cascade engine (override files)

Customisation follows a **CSS-style cascade**. For any requested *facet*, the
resolver returns the project's override if present, otherwise the embedded
default.

```
Resolution order (later wins):
1. Binary built-in defaults (embedded in the givn binary)
2. Project config files under givn/ (project wins)
```

- **Per-facet granularity.** The smallest overridable unit is a single facet —
  a template, an instruction, or a dependency declaration — *not* a whole
  artifact bundle. A project may override just the template for `design` while
  keeping the default instructions, and vice versa.
- **On-demand discovery via `eject`.** The project directory starts with *no*
  overrides. `givn eject <artifact>:<facet>` writes the embedded default into the
  project so it can be edited. Nothing is materialised until ejected.
- **Upgrade safety.** Because defaults live only in the binary, upgrading never
  touches project files. Project overrides always win regardless of binary version.

Pseudocode:

```
fn resolve_facet(artifact_id, facet) -> Bytes {
    // 1. Project override wins.
    let project_path = project_override_path(artifact_id, facet); // givn/artifacts/<id>/<facet>
    if exists(project_path) { return read(project_path); }

    // 2. Embedded core default (templates/ or instructions/).
    if let Some(content) = embedded_core(artifact_id, facet) { return content; }

    // 3. Embedded addon fallback (addons/<artifact_id>/template.md or instructions.md).
    return embedded_addon(artifact_id, facet);   // error if not found
}
```

The third layer exists for addon artifacts (e.g. `arc42-docs`) whose assets live
under `assets/addons/<id>/` rather than the core `assets/templates/` and
`assets/instructions/` directories. Project overrides still win — ejecting an
addon facet places a file at `givn/artifacts/arc42-docs/instructions.md` which
takes precedence over the embedded addon default.

## 8.2 Manifest and overlay/patch

The artifact graph is the one piece that is not a pure per-file override; it is a
structured document. It is handled as:

```
effective_manifest = overlay( embedded_default_manifest, project_config_yaml )
```

- The embedded `default-manifest.yaml` defines the default graph: artifacts,
  their `generates` globs, `requires` (deps), `template` facet name,
  `instruction`, the `apply` block (`requires`, `tracks`), and `verify`.
- The project's `givn/config.yaml` is an **overlay/patch**: it declares only
  additions, changes, or removals, merged onto the embedded graph. This lets
  projects benefit from default-graph improvements on upgrade while still
  customising. It is also where `verify.command` is set.

Overlay semantics (to specify precisely in Phase 2):
- Artifacts are keyed by `id`. An overlay artifact with an existing id is a deep
  merge (field-by-field). A new id is appended. A removal directive deletes by id.
- Scalar fields in the overlay replace the default; the `verify.command` is set
  here by most projects.

### Artifacts are first-class configurable (the workflow is data, not code)

The lifecycle is **not hardcoded**. The sequence of artifacts, their
dependencies, and the gates between them are entirely described by the manifest
and therefore fully reconfigurable per project via the overlay — without touching
the binary. This is a primary feature, not an afterthought. Each artifact node
carries:

| Field | Meaning |
| --- | --- |
| `id` | Unique key (e.g. `proposal`, `specs`, `design`, `tasks`, `review`). |
| `generates` | File(s) the artifact produces (path or glob). |
| `requires` | Artifact ids that must exist first (defines the graph/order). |
| `template` | Facet name of the embedded/ejected template. |
| `instruction` | The guidance text (cascade-resolved). |
| `optional` | If `true`, the artifact may be skipped without blocking the lifecycle. |

The `apply` block separately defines the archive gate via `apply.requires`
(which artifacts must be complete before archiving) and `apply.tracks` (which
file holds the checklist).

Common reconfigurations — all via `givn/config.yaml` overlay only:

**a) Remove the review step (no review gate).**
```yaml
artifacts:
  - id: review
    remove: true
apply:
  requires: [tasks]          # was [tasks, review]
```

**b) Make review optional rather than removing it.**
```yaml
artifacts:
  - id: review
    optional: true
apply:
  requires: [tasks]          # review no longer gates archive
```

**c) Add a dedicated Refactor step after implementation.**
```yaml
artifacts:
  - id: refactor
    generates: refactor.md
    requires: [tasks]        # runs after implementation/tasks
    template: refactor.md    # eject/author this template
    instruction: |
      Record refactorings performed after GREEN: what was cleaned up,
      why, and confirmation that all scenarios still pass. No behaviour change.
apply:
  requires: [tasks, refactor, review]   # refactor now gates archive too
```

**d) Reorder / add any artifact.** Order is derived from `requires`, so changing
dependencies reorders the graph. New ids are appended and slotted by their deps.

Rules and guarantees:
- Removing or reordering artifacts must keep the dependency graph acyclic;
  `givn` validates the effective graph and refuses an inconsistent overlay
  (e.g. a dangling `requires`, a cycle, or an `apply.requires` referencing a
  removed non-optional artifact) with a clear error.
- Default templates/instructions exist only for the built-in artifacts. A
  custom artifact (e.g. `refactor`) needs its template/instruction provided by
  the project (author the file under `givn/artifacts/<id>/`, or rely on a minimal
  generated stub).
- Filesystem-derived state (8.6) adapts automatically: state is computed from the
  effective artifact set, not a hardcoded list.

## 8.3 The Gherkin delta model

The `.feature` file is the spec. During an active change, `.feature` files are
*deltas* tagged for the merge engine.

| Tag | On | Meaning |
| --- | --- | --- |
| `@givn.delta` | Feature | Marks this file as a delta document. |
| `@<capability>` | Feature | Capability identifier (kebab-case). |
| `@givn.added` | Scenario | Append to permanent spec (error if title already exists). |
| `@givn.modified` | Scenario | Replace scenario by title (error if not found). |
| `@givn.removed` | Scenario | Delete scenario by title (error if not found). |
| `@wip` | Scenario | Step definitions not yet implemented. |

Rules:
- A scenario with no `@givn.*` delta tag defaults to `@givn.added`.
- `@givn.removed` scenarios include exactly one placeholder step so the parser
  accepts them; `givn lint` suppresses warnings for them.
- On archive, **all `@givn.*` tags are stripped** from the permanent output; the
  result is clean, runnable Gherkin.

### Merge on archive (chapter 6.6)

For each delta scenario, apply added/modified/removed to
`givn/specs/<cap>/<cap>.feature` by scenario title, then strip tags. Merge
preconditions that fail abort the archive without partial writes.

### Specification vs implementation — strict separation of concerns

There is a hard conceptual boundary between *what/why* and *how*. The artifacts
are assigned to exactly one side of it:

| Layer | Artifacts | Concern | Must NOT contain |
| --- | --- | --- | --- |
| **Specification** | `proposal`, `specs` (`.feature`) | WHAT the system does and WHY (observable behaviour, user value). | Implementation details: classes, function names, frameworks, DB schemas, HTTP routes, step-definition mechanics, file paths, libraries. |
| **Implementation** | `design`, `tasks` (and any custom impl artifacts) | HOW it is built. | — (this is where all the "how" belongs). |

Principles:
- **Proposal and `.feature` files are implementation-independent** — they describe
  behaviour a domain expert could read and validate, independent of the chosen
  technology. They should remain valid even if the entire implementation is
  rewritten.
- **Scenarios assert observable behaviour**, in domain language (Given/When/Then
  about inputs, outputs, state visible to a user/client) — not internal calls,
  not specific code structures.
- **The only permitted exception** is when an implementation aspect is *itself*
  the externally-required behaviour (e.g. "the response is valid JSON", "data is
  persisted across restart", "the endpoint responds within 200 ms"). Such
  constraints are part of the spec because they are observable requirements, not
  implementation choices. When in doubt: if it would still be true under a
  reasonable reimplementation, it belongs in the spec; otherwise it belongs in
  design/tasks.
- **Design owns the technology decisions**: language, test runner, step-definition
  locations, world/context object, coverage setup, runner commands. **Tasks own
  the build steps**: the scenario-by-scenario RED/GREEN/REFACTOR plan.

This separation is enforced *by the instructions* shipped with each artifact
(the proposal and spec instructions explicitly forbid implementation detail; the
design and tasks instructions explicitly carry it). The instructions are
cascade-resolved and ejectable, so a project can tighten or relax the wording —
but the default templates make the boundary explicit.

## 8.4 Project directory layout

```
givn/
├── config.yaml                       # manifest OVERLAY + verify.command
├── specs/<cap>/<cap>.feature         # permanent merged spec (source of truth)
├── changes/<id>/                     # active change (state = which files exist)
│   ├── proposal.md
│   ├── specs/<cap>/<cap>.feature     # delta: @givn.delta + added|modified|removed
│   ├── design.md
│   ├── tasks.md
│   ├── design-review.md
│   └── review.md
├── archive/<id>/                     # archived changes (moved here on archive)
└── artifacts/<id>/                   # OPTIONAL ejected overrides
    ├── template.*                    #   ejected template facet
    └── instructions.md               #   ejected instructions facet
givn/skills/<name>/SKILL.md.tmpl      # OPTIONAL ejected skill template

# Generated agent files (outside givn/):
AGENTS.md                             # repo root — agent rules/context
.agents/skills/<name>/SKILL.md        # neutral skills (primary)
.claude/skills/<name>/SKILL.md        # mirror for Claude Code
.opencode/skills/<name>/SKILL.md      # mirror for opencode
```

## 8.5 The TDD loop (scenario-by-scenario)

The implement phase runs **one scenario at a time**, in `.feature` order:

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions WITH real assertions (Then asserts concrete values).
            Run the test runner → this scenario MUST FAIL (no production code yet).
            If it passes here, the assertions are not real — fix the steps first.
  GREEN:    Write the minimum production code → run runner → this scenario PASSES.
  REFACTOR: Clean up without changing behaviour → runner still PASSES.
Then move to the next scenario.
```

Two distinct tools, never conflated:
- **`givn lint`** — a built-in command. Statically checks `.feature` syntax,
  `@wip`, and delta tags only. **Never runs tests.** Exit 0/1/2.
- **The test runner** — the command in `verify.command` (e.g. `cargo test`).
  Executes step definitions against production code. This is what confirms GREEN.
  It is **not** a givn subcommand; givn does not run it (the agent/CI does).

`givn lint` alone is **never** sufficient to confirm GREEN — that requires the
test runner (`verify.command`).

## 8.6 Filesystem-derived state

There is no state file. Lifecycle state is computed from which files exist:

| Observation | Derived state |
| --- | --- |
| `givn/changes/<id>/proposal.md` exists | proposal done |
| `givn/changes/<id>/specs/**.feature` exist | spec drafted |
| `design.md` / `design-review.md` / `tasks.md` / `review.md` exist | respective stage reached |
| all `tasks.md` checkboxes `[x]` | implementation complete |
| `design-review.md` contains `DESIGN-REVIEW: PASS` | design review signed off |
| `review.md` contains `REVIEW: PASS` | coverage review signed off |
| `arc42.md` (in change dir) contains `STATUS: DONE` | arc42-docs update complete |
| `givn/archive/<id>/` exists | archived |

Benefits: no drift, git-friendly, trivially correct. `apply.requires`
(`[tasks, review]`) is checked by file presence (and marker strings) before
archive. `design-review` gates `tasks` via the dependency graph — it is not in
`apply.requires` because it is a planning-layer gate, not an archive gate.

Marker conventions for completion signals beyond mere file presence:
- **`design-review.md`** — must contain the string `DESIGN-REVIEW: PASS`
  (written by the design-review agent at sign-off; absence or `FAIL` → Partial).
- **`review.md`** — must contain the string `REVIEW: PASS` (written by the
  review agent at sign-off; `REVIEW: FAIL` or absence → Partial).
- **`arc42.md`** (change-level) — must contain `STATUS: DONE` (written by the
  arc42-docs agent; `STATUS: PARTIAL` or absence → Partial). The durable
  `docs/arc42/` chapter files are not themselves tracked — only this marker.

## 8.7 Skill & rule generation

Standards-aligned, multi-target emit (see ADR-013):

- **Rules/context →** `AGENTS.md` at repo root (the ratified standard).
- **Skills →** neutral `.agents/skills/<name>/SKILL.md` (primary), mirrored to
  `.claude/skills/<name>/SKILL.md` and `.opencode/skills/<name>/SKILL.md`.
- Default skills, one per workflow action: `givn-explore`, `givn-propose`,
  `givn-spec`, `givn-design`, `givn-design-review`, `givn-tasks`,
  `givn-implement`, `givn-review`, `givn-archive`.
- `givn-explore` is a pre-lifecycle skill — a thinking stance, not a manifest
  artifact. It produces no tracked output and has no gate. It runs before
  `/givn-propose` when the problem is still vague. See ADR-031.
- Skill templates are **ejectable** (`givn eject skill:<name>`), so users can
  retarget tools givn does not ship support for.
- `SKILL.md` frontmatter: `name`, `description` (kebab-case name matching the
  directory; description 1–1024 chars).

## 8.8 Error handling and exit codes

- Library errors via `thiserror`; application context via `anyhow`.
- `givn lint` exit codes: **0** clean, **1** parse error, **2** `@wip`/PENDING
  (these describe spec well-formedness, not test results — "GREEN" comes from the
  test runner, not from `givn lint`).
- Other commands: **0** success, non-zero on failure with a clear message.
- Destructive operations (archive) validate all preconditions before any write.

## 8.9 Coverage gate (completeness, not percentage)

The review phase classifies every uncovered line/branch into exactly one of:

1. **Dead code** (YAGNI/KISS) → delete it.
2. **Missing test coverage** → fix step definitions, or add a scenario and run
   the full RED/GREEN/REFACTOR loop.
3. **Legitimately hard to test** (rare; needs concrete technical justification).

Coverage is **complete** when every gap is classified and resolved. There is no
minimum percentage; a high number is a side effect, not the target.

## 8.10 Configuration: documented defaults, versioning, and upgrade

The project's `givn/config.yaml` is the primary configuration surface. It is
generated by `givn init` and maintained across versions.

### Self-documenting, fully commented (ADR-022)

Every supported key is present in the generated file but **commented out**, each
with: a one-line description, the effective default (shown commented), allowed
values/format, and a cross-reference where helpful. Users override by uncommenting
and editing. The only thing typically written uncommented is `verify.command`,
prominently flagged as the one value to change.

Sketch of the generated file:

```yaml
# ============================================================================
#  givn configuration  —  https://.../docs/arc42/08-crosscutting-concepts.md
#  Managed by givn. Uncomment a key to override the embedded default.
# ============================================================================

# Managed by givn. Do not delete the next line.
givn_config_version: "1.0.0"

# --- Test runner -------------------------------------------------------------
# Command givn reports as the way to execute step definitions. The shipped
# default is a sentinel that prints setup guidance and fails (see ADR-021).
# CHANGE ME to your project's command, e.g.:
#   cargo test
#   mvn test
#   npx cucumber-js "givn/specs/**/*.feature"
#   behave
verify:
  command: "givn missing-testrunner"   # <-- replace this

# --- Feature path ------------------------------------------------------------
# Path the test runner reads features from (also exported as GIVN_FEATURES).
# Default: givn/specs
# features_path: "givn/specs"

# --- Artifact graph overlay --------------------------------------------------
# Patch the embedded default graph: add/modify/remove artifacts, change deps,
# set apply.requires. See arc42 ch.8.2. Example (commented):
# artifacts:
#   - id: risk-assessment
#     generates: risk-assessment.md
#     requires: [proposal]
#
# apply:
#   requires: [tasks, review]

# --- Hooks (run verify.command / lint / shell at gates) ----------------------
# Each artifact may declare `post:` hooks run after it completes. Built-ins:
#   verify  -> runs verify.command (the test runner)
#   lint    -> runs `givn lint`
# or any shell command string. Object form supports allow_failure / name.
# `givn check <artifact>` runs them; `givn archive` runs hooks of every
# apply.requires artifact as the gate (failure blocks archive). See arc42 ch.8.12.
# Defaults (already in the embedded graph): tasks.post: [verify], review.post: [verify]
# artifacts:
#   - id: tasks
#     post: [lint, verify]
#   - id: review
#     post:
#       - { run: verify, name: "full suite green" }
#       - { run: "scripts/coverage.sh", allow_failure: true }
```

A test asserts the documented keys and the manifest schema stay in sync (ADR-022),
so the file cannot drift from reality.

### `givn lint` vs. the test runner (`verify.command`) — no overlap

These are two different operations and must never be conflated:

| | `givn lint` (command) | `verify.command` (config key) |
| --- | --- | --- |
| Purpose | Static check of `.feature` files: syntax, `@wip`/PENDING, delta tags. | Run the actual tests (execute step definitions against code). |
| Runs code? | No. | Yes. |
| Who runs it | givn (built-in subcommand). | The developer / AI agent / CI — **not** givn itself. |
| Configurable? | No (built-in). | Yes — set in `givn/config.yaml`. |
| Exit codes | 0 clean / 1 parse / 2 wip (spec well-formedness). | Whatever the runner returns (sentinel: non-zero). |
| Confirms GREEN? | Never. | Yes — this is the source of truth for behaviour. |

There is intentionally **no `givn verify`/`givn test` command** that runs tests.
givn does not execute the test suite (ADR-014: givn tracks and reports; the agent
or CI drives the RED/GREEN/REFACTOR loop). The test command lives in config so it
stays language-agnostic and explicit.

### Where `verify.command` is read and used

- **Resolved** by `config::manifest` after `config::overlay` applies the project's
  `givn/config.yaml` onto the embedded default manifest. Effective value = project
  override if present, otherwise the embedded default (the sentinel).
- **Written** into `givn/config.yaml` by `givn init`, refreshed by `givn upgrade`
  (documented and commented; ADR-022/023).
- **Invoked** by the developer, the AI agent (each RED/GREEN/REFACTOR step), and
  CI to run the tests. givn surfaces the value (e.g. in `instructions` and skills)
  but does not run it.
- **Surfaced** to agents via `givn instructions` and the generated skills, so the
  exact runner command is unambiguous wherever it is needed.

### The missing-testrunner sentinel (ADR-021)

The default `verify.command` is `givn missing-testrunner`, a built-in subcommand
that prints per-language guidance and exits non-zero. A fresh project therefore
fails loudly with instructions until a real runner is configured, instead of
silently doing nothing.

### Versioning and upgrade (ADR-023)

- `givn_config_version` records the givn version that wrote/last upgraded the file.
- `givn upgrade` re-renders the **managed** (commented documentation) scaffolding
  of `givn/config.yaml` from the current binary's templates while **preserving
  the user's uncommented overrides**, then bumps `givn_config_version`.
- Because the file lives in the project repo, the user reviews changes via
  `git diff` and resolves any conflicts there — git is the merge surface.
- Upgrades are idempotent and never delete user-set values. Removed/renamed keys
  are left in place with a note rather than dropped.

**Managed vs user regions.** Commented documentation lines are givn-managed and
may be rewritten by `givn upgrade`. Uncommented key/value lines are user-owned and
are preserved verbatim. This convention keeps upgrades safe and `git diff` readable.

## 8.11 Addons and the arc42 documentation artifact

### What an addon is

An **addon** is a pre-packaged artifact (or set of artifacts) that ships embedded
in the binary but is **off by default**. Enabling it is just an overlay edit
(ADR-024) — there is no separate plugin runtime. This is givn's extension model:
the workflow is data, so an addon is "data the user opts into".

Enabling an addon — three equivalent ways:

```bash
givn addons enable arc42          # CLI: writes addons.arc42: true into config.yaml
givn addons disable arc42         # CLI: writes addons.arc42: false
givn addons list                  # CLI: shows all addons + enabled/disabled status
```

```yaml
# givn/config.yaml — direct edit (equivalent to givn addons enable arc42):
addons:
  arc42: true          # expands to the arc42-docs artifact definition

# For full control, add the artifact directly in the overlay:
artifacts:
  - id: arc42-docs
    optional: true
    requires: [design]
```

The `addons:` block is convenience sugar that expands to the addon's artifact
definition during overlay processing; advanced users can declare the artifact
directly and customise every field.

### The `arc42-docs` artifact (ADR-026)

Purpose: create and maintain **durable, project-level arc42 architecture
documentation** as the system evolves — e.g. running after the `design` step so
the docs reflect each change.

| Property | Default | Notes |
| --- | --- | --- |
| `id` | `arc42-docs` | |
| `requires` | `[design]` | Runs after the change's "how" is known. |
| `optional` | `true` | Opt-in; does not gate archive unless added to `apply.requires`. |
| Output location | `docs/arc42/` | Configurable (`arc42.output_dir`). Durable, **not** per-change. |
| Chapter set | 12 arc42 chapters + index | Configurable; chapters are individual files. |
| Templates/instruction | embedded `assets/addons/arc42-docs/` | Cascade-resolved & ejectable. |

Behaviour (driven by the artifact instruction):
- On first run, **create missing** arc42 chapter files from templates and an index.
- On subsequent changes, **update only the affected chapters** based on the
  change's proposal/spec/design (e.g. new decisions → chapter 9; new building
  blocks → chapter 5). Do not rewrite untouched chapters.
- These documents are **project-level and durable**: they live under `docs/`
  (outside `givn/changes/`), evolve across changes, and are **not** archived or
  merged per change. The change lifecycle merely triggers their maintenance.

Distinction from per-change artifacts:
- Per-change artifacts (proposal, spec, design, tasks, review) live under
  `givn/changes/<id>/` and are archived with the change.
- The arc42 docs are a *side effect target* the artifact maintains in the repo's
  durable docs tree; the artifact's own marker file (e.g. an `arc42.md` note in
  the change, or a checklist entry) records that the docs were updated for that
  change, which is what filesystem-derived state observes.

Configuration knobs (all optional, documented in `givn/config.yaml`):
```yaml
addons:
  arc42: true
# arc42:
#   output_dir: "docs/arc42"      # where the durable docs live
#   chapters: [introduction-and-goals, ..., glossary]  # subset/superset allowed
#   require_for_archive: false    # if true, adds arc42-docs to apply.requires
```

Because it is an ordinary configurable artifact, everything in ch.8.2 applies:
reorder via `requires`, make it gate archive via `apply.requires`, eject its
templates/instruction to customise wording or the chapter structure.

## 8.12 Hooks — running `verify.command` at gates (ADR-028)

To make the flow self-contained, artifacts can declare **`post` hooks**: commands
givn runs after an artifact is completed. This is how `verify.command` actually
gets executed *by givn* (e.g. to confirm tests are GREEN after implementation and
after review), rather than only being invoked by the agent/CI.

### Hook entries

A `post` list contains entries that are one of:
- a **named built-in**:
  - `verify` → run the resolved `verify.command` (the test runner),
  - `lint` → run `givn lint` on the relevant `.feature` files,
- an **arbitrary shell command string** (e.g. `"npm run typecheck"`),
- or an **object** with options:
  ```yaml
  { run: verify, allow_failure: false, name: "tests green" }
  ```

### Defaults

```yaml
# (embedded default-manifest, shown as it would appear)
artifacts:
  - id: tasks
    post: [verify]      # run the test runner after implementation
  - id: review
    post: [verify]      # run the test runner again after review
```

A project overrides these like any other artifact field via the overlay — add
hooks, remove them, or add custom ones:

```yaml
# givn/config.yaml
artifacts:
  - id: tasks
    post: [lint, verify, "npm run typecheck"]
  - id: review
    post:
      - { run: verify, name: "full suite green" }
      - { run: "scripts/coverage.sh", allow_failure: true }
```

### Execution and failure semantics

- `givn check <artifact>` runs that artifact's `post` hooks on demand.
- `givn check` (no argument) runs hooks for every artifact whose output exists.
- `givn archive` **auto-runs the hooks of all `apply.requires` artifacts** as the
  archive gate; archive is refused if any required hook fails.
- A hook that exits non-zero **blocks the gate** and givn exits non-zero, unless
  that hook has `allow_failure: true` (then it only warns).
- Hooks run with `GIVN_FEATURES` set to the right target (the active change's
  features during development; `givn/specs` after archive), so the runner tests
  the correct `.feature` files.

### Interaction with the sentinel

Because the default `verify.command` is the sentinel `givn missing-testrunner`
(ch.8.10), `givn check` and `givn archive` **fail loudly** on a fresh project
until a real runner is configured. This is the enforcement mechanism behind
"nothing ever falsely reports green" — now checked by givn itself, not just by
convention.

### Relationship to the TDD loop

Hooks do **not** replace or drive the scenario-by-scenario RED/GREEN/REFACTOR loop
(ch.8.5); the agent still does that. Hooks are *gate checks*: after the loop is
done (tasks) and after review, givn runs the whole suite to confirm it is green
before letting the change proceed to archive.

### Security note

Hooks are shell commands declared in the project's own `givn/config.yaml` — the
same trust level as a `Makefile`/`justfile` committed to the repo. givn does not
fetch or execute remote commands.

---

## 8.13 Reverse-engineering mode (ADR-032)

Reverse-engineering mode creates the **initial baseline spec** for a capability
by characterizing its observed black-box behaviour. It is used when an existing
codebase has no givn specs yet — most notably for the Phase 7 dogfood (givn on
givn).

### Characterization vs. TDD

| Aspect | Normal TDD (`/givn-implement`) | Characterization (`/givn-characterize`) |
|---|---|---|
| Test stance | Prescriptive — test asserts desired behaviour | Descriptive — test asserts **observed** behaviour |
| First run | RED (must fail — no production code) | GREEN (must pass — matching reality) |
| If test fails | Good — RED proves the test is real | Diagnose: infrastructure problem or spec/mismatch |
| Production code | Written during GREEN to satisfy the test | Refactored ONLY for testability seams; never to satisfy a test |
| Bugs | Fixed as part of the change | Flagged as follow-up `@givn.modified` candidates; never fixed |

### The two commands

- **`/givn-reverse-engineer`** — planning orchestrator with an interactive
  **discovery phase** (explores the codebase, proposes one capability with a
  black-box boundary, confirms with the user). Delegates to an architect
  subagent using reverse-mode instructions (proposal, specs, design, tasks).
  Reuses `/givn-design-review` as-is for plan stress-testing.
- **`/givn-characterize`** — execution + review orchestrator. Phase 1:
  characterization loop (GREEN-first tests, testability seams, spec/implementation
  mismatch diagnostic). Phase 2: reverse review epilogue (coverage with no
  dead-code deletion, bug flagging, behaviour-preservation invariant).

### Empty-baseline merge

All scenarios are `@givn.added` against an empty permanent spec. On archive,
the merge engine creates the initial `givn/specs/<cap>/<cap>.feature` baseline.
The merge engine uses the delta feature's name as the baseline header (not the
generic `"Feature: (merged)"` placeholder — fixed in ADR-032).

### Spec/implementation mismatch diagnostic

When `verify.command` fails during characterization (and infrastructure is
sound), the subagent diagnoses:

1. **Documentation was wrong** — docs said Y, code correctly does Z. Update
   design.md, tasks.md, the .feature scenario, and the source documentation.
2. **Code is ambiguous/misleading** — code does Z but reads like Y. Update the
   .feature scenario to AS-IS behaviour. Add a `REVERSE-ENGINEERING NOTE:`
   comment (language-agnostic comment syntax) at the misleading point. Do NOT
   change code behaviour.

### REVERSE-ENGINEERING NOTE markers

Standardized code comment prefix for debt tracking. All markers are grep-able
(`grep -rn "REVERSE-ENGINEERING NOTE"`) and listed in `review.md`. They are
intentional — never removed during characterization. Follow-up refactoring
changes address them.

### Testability-only refactor boundary

Allowed: extract interface, dependency injection, make a function/method public,
add a test-only constructor, extract a pure function from a side-effectful one.

NOT allowed: rename, deduplicate, simplify logic, restructure for clarity, fix
bugs, change error messages, change return types, change control flow.

### Prompt-layer only

Reverse-engineering mode is entirely prompt-layer: no new CLI subcommands, no
manifest changes, no new artifacts. It drives existing `givn new`/`givn
apply`/`givn archive` with reverse-mode instructions. The reverse-mode
instruction files (`assets/instructions/*-reverse.md`) are canonical source,
inlined into command prompts (not served by the CLI today).
