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
    let project_path = project_override_path(artifact_id, facet); // givn/artifacts/<id>/<facet>
    if exists(project_path) { return read(project_path); }
    return embedded_default(artifact_id, facet);
}
```

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
  accepts them; `givn verify` suppresses lint warnings for them.
- On archive, **all `@givn.*` tags are stripped** from the permanent output; the
  result is clean, runnable Gherkin.

### Merge on archive (chapter 6.6)

For each delta scenario, apply added/modified/removed to
`givn/specs/<cap>/<cap>.feature` by scenario title, then strip tags. Merge
preconditions that fail abort the archive without partial writes.

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
- `givn verify` — lints `.feature` syntax and `@wip` only. Never runs tests.
- `<verify.command>` (e.g. `cargo test`) — executes step definitions against code.

`givn verify` alone is **never** sufficient to confirm GREEN.

## 8.6 Filesystem-derived state

There is no state file. Lifecycle state is computed from which files exist:

| Observation | Derived state |
| --- | --- |
| `givn/changes/<id>/proposal.md` exists | proposal done |
| `givn/changes/<id>/specs/**.feature` exist | spec drafted |
| `design.md` / `tasks.md` / `review.md` exist | respective stage reached |
| all `tasks.md` checkboxes `[x]` | implementation complete |
| `givn/archive/<id>/` exists | archived |

Benefits: no drift, git-friendly, trivially correct. `apply.requires` (e.g.
`[tasks, review]`) is checked by file presence (and review PASS) before archive.

## 8.7 Skill & rule generation

Standards-aligned, multi-target emit (see ADR-013):

- **Rules/context →** `AGENTS.md` at repo root (the ratified standard).
- **Skills →** neutral `.agents/skills/<name>/SKILL.md` (primary), mirrored to
  `.claude/skills/<name>/SKILL.md` and `.opencode/skills/<name>/SKILL.md`.
- Default skills, one per workflow action: `givn-propose`, `givn-spec`,
  `givn-design`, `givn-tasks`, `givn-implement`, `givn-review`, `givn-archive`.
- Skill templates are **ejectable** (`givn eject skill:<name>`), so users can
  retarget tools givn does not ship support for.
- `SKILL.md` frontmatter: `name`, `description` (kebab-case name matching the
  directory; description 1–1024 chars).

## 8.8 Error handling and exit codes

- Library errors via `thiserror`; application context via `anyhow`.
- `verify` exit codes: **0** GREEN, **1** parse error, **2** `@wip`/PENDING.
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
```

A test asserts the documented keys and the manifest schema stay in sync (ADR-022),
so the file cannot drift from reality.

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
