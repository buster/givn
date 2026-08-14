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

**Never apply (a) or (b) to `design-review`.** It is a planning-layer gate
that blocks `tasks`, not an archive gate — removing or making it optional
does not save time, it defers a flawed plan's problems to implementation,
where they cost a rewrite instead of a markdown edit.

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

### 8.3a Normative authoring guidance

The E2E interaction policy has one normative source in the `specs` instruction.
Design, task, review, skill, and slash-command guidance references that source
and supplies only artifact-specific choreography. This prevents generated
entry points from drifting into contradictory rules. The existing cascade
still lets a consuming project override the effective instruction deliberately.

## 8.4 Project directory layout

```
givn/
+-- config.yaml                       # manifest OVERLAY + verify.command
+-- specs/<cap>/<cap>.feature         # permanent merged spec (source of truth)
+-- changes/<id>/                     # active change (state = which files exist)
|   +-- proposal.md
|   +-- specs/<cap>/<cap>.feature     # delta: @givn.delta + added|modified|removed
|   +-- design.md
|   +-- tasks.md
|   +-- design-review.md
|   +-- review.md
+-- archive/<id>/                     # archived changes (moved here on archive)
+-- artifacts/<id>/                   # OPTIONAL ejected overrides
    +-- template.*                    #   ejected template facet
    +-- instructions.md               #   ejected instructions facet
givn/skills/<name>/SKILL.md.tmpl      # OPTIONAL ejected skill template

# Generated agent files (outside givn/):
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

## 8.5a Local scenario retrieval

Retrieval is an authoring aid, not a correctness gate. `givn spec index` owns a
local cache of model-backed vectors and content hashes under `givn/`; `givn
spec search` reads that index and prints ranked candidates. CI, lint, and
archive do not require a model or a similarity score. A missing model/cache is
an explicit setup error for retrieval, never a false clean result or a block on
unrelated workflow commands.

## 8.6 Filesystem-derived state

There is no state file. Lifecycle state is computed from which files exist:

| Observation | Derived state |
| --- | --- |
| `givn/changes/<id>/proposal.md` exists | proposal done |
| `givn/changes/<id>/specs/**.feature` exist | spec drafted |
| `design.md` / `design-review.md` / `tasks.md` / `review.md` exist | respective stage reached |
| all `tasks.md` checkboxes `[x]` (case-insensitive: `[X]` also counts as done) | implementation complete |
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

### Input validation

All commands accepting `--change` validate the id as kebab-case (lowercase
ASCII, digits, hyphens; no leading/trailing hyphens) before constructing any
filesystem path. The validation uses the shared `resolve_change_dir` helper in
`src/state.rs`, which is called by every command that constructs a
`givn/changes/<id>/` path. This prevents path traversal (e.g. `--change ../etc`)
from escaping the `givn/changes/` directory.

`givn check <artifact>` and `givn eject <artifact>:instruction` validate the
artifact id against the effective manifest before any further processing,
producing a consistent "unknown artifact" error that lists known artifact ids.
This mirrors the validation already performed by `givn instructions`.

## 8.7 Skill & rule generation

Standards-aligned, configurable-target emit (see ADR-013, ADR-036):

- **`AGENTS.md`** at repo root is **user-owned** — givn never writes or
  overwrites it. Users author whatever project-level rules/context they need.
- **Target selection** — the agent targets that receive generated skills are
  configured in the `skills:` block of `givn/config.yaml` (`targets:` list of
  `opencode`/`agents`/`claude`). `givn init` asks for the selection (default:
  opencode only); `givn skills sync` installs to exactly the persisted
  selection. Absent block → opencode only.
- **Skills →** written to `.opencode/skills/<name>/SKILL.md` when opencode is
  selected, `.agents/skills/<name>/SKILL.md` when agents is selected,
  `.claude/skills/<name>/SKILL.md` when claude is selected.
- **Commands →** `.opencode/commands/<name>.md`, written only when opencode
  is selected.
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
- `givn lint` exit codes: **0** clean, **1** parse error, **3** duplicate
  scenario title (overlap hard error), **2** `@wip`/PENDING — precedence
  1 > 3 > 2. Overlap **warnings** (shape match, subset, long scenario) never
  change the exit code; they require a disposition at review time
  (see ADR-037). `--json` emits all findings as a JSON array on stdout
  (one object per finding: `kind`, `level`, `files`, `titles`), exit codes
  unchanged. These describe spec well-formedness, not test results — "GREEN"
  comes from the test runner, not from `givn lint`.
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

### Project-owned coverage commands

Coverage commands live in `givn/commands.yaml`, alongside the test commands,
rather than as long inline invocations in `config.yaml`:

```yaml
verify:
  command: "./run-tests.sh"
  e2e_command: "./run-tests.sh --e2e"
coverage:
  measure_command: "./measure-coverage.sh"
  merge_command: "./merge-coverages.sh"
  non_e2e_path: "coverage/non-e2e-cobertura.xml"
  e2e_path: "coverage/e2e-cobertura.xml"
  output_path: "coverage/cobertura-coverage.xml"
```

The measurement command writes both source Cobertura reports. The merge
command writes the final report. Archive removes an existing final report
before invoking the merge command, so a successful command must create fresh
output. The final report is parsed for line and branch counters and is the sole
source for the README badges and managed coverage summary. The scripts receive
`GIVN_FEATURES`, `GIVN_COVERAGE_NON_E2E_PATH`, `GIVN_COVERAGE_E2E_PATH`, and
`GIVN_COVERAGE_OUTPUT_PATH`.

The coverage transaction backs up and restores the two source reports, final
report, README, and merged permanent specs when any command, parse, validation,
or README update fails. A branch denominator of zero is rendered as `n/a`, not
as a misleading `0%` badge.

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

### Post-merge e2e gate

After the per-change hooks and delta merge, `givn archive` also runs `verify.command`
and `verify.e2e_command` against the merged permanent specs (`givn/specs/`). The
post-merge e2e gate mirrors the per-change `verify-e2e` hook behaviour: if
`verify.e2e_command` is empty and the merged permanent specs contain `@e2e`
scenarios, the archive is blocked with a clear error — the same "unenforced gap"
message the per-change hook produces. This prevents a delta that adds `@e2e`
scenarios from creating a gap when it becomes permanent.

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

## 8.13 Reverse-engineering flow (ADR-033, supersedes ADR-032)

Reverse-engineering creates the **initial baseline spec** for a capability by
characterizing its observed black-box behaviour. It is used when an existing
codebase has no givn specs yet — most notably for the Phase 7 dogfood (givn on
givn). Reverse-engineering is a **peer flow** to the normal change lifecycle,
not a variant of it. It does not produce proposals or designs — only
descriptive Gherkin specs and characterization tasks.

### Two peer flows

```mermaid
flowchart LR
    subgraph STANDARD
        direction LR
        s1[propose] --> s2[spec] --> s3[design] --> s4[design-review] --> s5[tasks] --> s6[implement] --> s7[review] --> s8[archive]
    end
    subgraph REVERSE
        direction LR
        r1[reverse-engineer] --> r2[characterize] --> r3[archive]
    end
```

Reverse artifacts (3 only): `specs/<cap>/<cap>.feature`, `tasks.md`,
`review.md`. No proposal, no design, no design-review.

### Where content lives (vs. the old ADR-032 flow)

| Content | Old home (ADR-032) | New home (ADR-033) |
|---|---|---|
| Black-box boundary | `proposal.md` Capability section | `.feature` Feature: description block |
| Suspected bugs (from discovery) | `proposal.md` Out of Scope | `.feature` Feature: description block |
| Suspected bugs/smells (from characterization) | `proposal.md` + `review.md` | Production code `TODO: REVERSE-ENGINEERING NOTE:` comments + `review.md` |
| Seam strategy | `design.md` | Per-task "Seam" blocks in `tasks.md` |
| verify.command choice | `design.md` Test harness section | First task in `tasks.md` (harness setup) |

### Characterization vs. TDD

Both flows use the same RED/GREEN/REFACTOR loop structure (shared via the
`givn-steps` skill). What differs is the GREEN and REFACTOR content:

| Aspect | Normal TDD (`/givn-implement`) | Characterization (`/givn-characterize`) |
|---|---|---|
| Test stance | Prescriptive — test asserts desired behaviour | Descriptive — test asserts **observed** behaviour |
| RED | Unimplemented assertions for new steps; reused steps kept as-is | Same (shared via `givn-steps` skill) |
| GREEN | Real assertions + write minimum production code | Real assertions probing observed behaviour; no production code changes |
| If test fails | RED is good (proves test is real); GREEN failure → fix production code | Diagnose: infrastructure problem or spec/mismatch (correct the spec, never the code) |
| REFACTOR | General cleanup (names, duplication, clarity) | Testability seams only (extract interface, DI, make public) |
| Bugs | Fixed as part of the change | Flagged as follow-up `@givn.modified` candidates; never fixed |

### The two reverse commands

- **`/givn-reverse-engineer`** — discovery phase (interactive: explores the
  codebase, proposes one capability with a black-box boundary, confirms with
  the user). Then writes the `.feature` file directly (descriptive scenarios,
  boundary + suspected bugs in Feature: description block) and `tasks.md`
  (harness setup + one RED/GREEN/REFACTOR cycle per scenario). Uses
  `givn new --skip proposal,design,design-review` to skip unneeded artifacts.
  No architect subagent, no proposal, no design, no design-review.
- **`/givn-characterize`** — execution + review orchestrator. Phase 1:
  characterization loop (RED/GREEN/REFACTOR with step reuse via `givn-steps`
  skill, testability seams, spec/implementation mismatch diagnostic). Phase 2:
  reverse review epilogue (coverage mechanics via `givn-review` skill, reverse
  gap-classification rules: no dead-code deletion, bug flagging,
  behaviour-preservation invariant).

### The `givn-steps` skill

Shared step-writing mechanics, referenced by both `/givn-implement` and
`/givn-characterize`. Contains: the RED/GREEN/REFACTOR loop structure, step
reuse (check existing definitions first), lint-vs-test distinction, step
definition location conventions. Each orchestrator overlays what "GREEN" and
"REFACTOR" mean for its workflow.

### Skipped artifacts and `givn status`

`givn new --skip proposal,design,design-review` writes a per-change
`.givn-skip` file. `givn status` shows skipped artifacts as ⊘ skipped (not
pending), so `next_required` skips them. The `ArtifactStatus::Skipped` enum
variant is treated as satisfied by `all_required_done()` and the archive gate.

### Empty-baseline merge

All scenarios are `@givn.added` against an empty permanent spec. On archive,
the merge engine creates the initial `givn/specs/<cap>/<cap>.feature` baseline.
The merge engine uses the delta feature's name as the baseline header (not the
generic `"Feature: (merged)"` placeholder — fixed in ADR-032, retained in
ADR-033).

### Spec/implementation mismatch diagnostic

When `verify.command` fails during characterization (and infrastructure is
sound), the subagent diagnoses:

1. **Documentation was wrong** — docs said Y, code correctly does Z. Update
   tasks.md, the .feature scenario, and the source documentation.
2. **Code is ambiguous/misleading** — code does Z but reads like Y. Update the
   .feature scenario to AS-IS behaviour. Add a `TODO: REVERSE-ENGINEERING
   NOTE:` comment (language-agnostic comment syntax) at the misleading point.
   Do NOT change code behaviour.

### TODO: REVERSE-ENGINEERING NOTE markers

Standardized code comment prefix for debt tracking. The `TODO:` prefix ensures
IDE discoverability (VS Code, IntelliJ, etc. surface TODOs in task lists). The
`REVERSE-ENGINEERING NOTE:` suffix is grep-able
(`grep -rn "REVERSE-ENGINEERING NOTE"`) and distinguishes reverse-engineering
debt from regular TODOs. All markers are listed in `review.md`. They are
intentional — never removed during characterization. Follow-up refactoring
changes address them.

### Testability-only refactor boundary

Allowed: extract interface, dependency injection, make a function/method public,
add a test-only constructor, extract a pure function from a side-effectful one.

NOT allowed: rename, deduplicate, simplify logic, restructure for clarity, fix
bugs, change error messages, change return types, change control flow.

### Prompt-layer with minimal binary support

Reverse-engineering is primarily prompt-layer: the reverse-mode instruction
files (`assets/instructions/*-reverse.md`) are canonical source, inlined into
command prompts. The only binary changes are: the `--skip` flag on `givn new`,
the `ArtifactStatus::Skipped` variant in `state.rs`, and the `givn-steps` skill
added to the sync lists. No manifest changes, no new CLI subcommands for the
reverse flow itself.

---

## 8.14 Arc42 implementation conformance review (ADR-035)

The `arc42` addon has two distinct review points. `design-review` checks the
planned chapter impact before tasks are written. The completion review checks
whether the durable architecture and the completed change still agree after
implementation.

### Conditional review block

The core review assets contain an explicit Arc42 implementation conformance
block. If `addons.arc42` is disabled, the block is recorded as not applicable.
If it is enabled, the block is mandatory and compares:

| Source | Question |
| --- | --- |
| Durable `docs/arc42/` chapters | What architecture facts and decisions are currently documented? |
| Change-level `arc42.md` | Which chapters does this change claim to affect? |
| `design.md` | Does the planned solution account for every relevant fact? |
| `tasks.md` | Do implementation tasks map to the documented decisions and consequences? |
| Completed implementation | Does the built result follow the documented and designed architecture? |

The reviewer independently derives relevant facts before accepting the
change-level claims. Every chapter named by either assessment is opened and
checked for placeholder text, stale content, and contradictions. The review
records source evidence and a match/finding result for each fact.

### Finding and remediation semantics

The review catches both directions of drift: a completed change missing from
durable Arc42 documentation, and an implementation that ignores or contradicts
an Arc42 fact. Any omission, contradiction, stale chapter, or missing task
mapping is a finding. The affected documentation, design, task list, or code is
corrected and the comparison is repeated. If the design changes, the planning
design-review is run again before the completion review can pass.

This is a human evidence review, not automatic source-code inference. The
architecture facts require context and trade-off judgment that a text scan
cannot establish reliably.
