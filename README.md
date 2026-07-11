# givn

> from Given to done.

`givn` is a single-binary, Gherkin-first spec-driven workflow tool. It manages a
change lifecycle where **the `.feature` file IS the specification** — the delta
document, the executable test, and the permanent archived record, all in one.

It distills a proven Gherkin-first workflow into a standalone, configurable,
easily-installable Rust binary: delta lifecycle → scenario-by-scenario TDD →
completeness-based coverage review → automatic delta merge on archive.
Language- and test-runner-agnostic.

**Status:** implementation complete (phases 0–5.5). Single binary, all commands
working, 47 tests passing. See the
[arc42 architecture docs](docs/arc42/README.md) and the
[implementation task breakdown](docs/TASKS.md).

---

## Why givn

Existing spec-driven tools have pain points this project fixes deliberately:

| Pain point (in prior tools) | How givn fixes it |
| --- | --- |
| Too much hardcoded behaviour | Everything customisable via override files (CSS cascade) |
| npm + root/sudo install | Single static binary; no npm, no root, no runtime |
| Schema customisation requires forking TS source | Eject any facet into your project dir; edit freely |
| Skill templates hardcoded; overwritten on update | Templates are ejectable override files; updates touch only the embedded fallback |
| Config scattered across 3 layers | One visible `givn/` directory holds all project config |
| Opaque config; unclear what's changeable | `givn/config.yaml` is fully commented with documented defaults for every key |
| No upgrade path when the tool evolves | Config is versioned; `givn upgrade` refreshes it, with `git diff` as the merge surface |
| Silent/guessed test running | Default runner is a sentinel that prints setup guidance and fails until configured |
| A parallel `spec.md` format duplicating the `.feature` | The `.feature` file **is** the spec — nothing duplicated |
| Coverage gated on an arbitrary percentage | Gate is **completeness of gap classification**, not a number |

## Core ideas

- **The `.feature` file is the single source of truth.** No `spec.md`. The same
  file is the delta you write, the cucumber test you run, and (after archive) the
  permanent spec. `verify.command` MUST execute these `.feature` files via a
  Gherkin/Cucumber runner — never a parallel hand-maintained test file.
- **Delta lifecycle with tags.** Deltas are tagged `@givn.delta` (Feature) and
  `@givn.added` / `@givn.modified` / `@givn.removed` (Scenario). On archive they
  are merged into the permanent spec and the tags are stripped, leaving clean,
  runnable Gherkin.
- **Scenario-by-scenario TDD.** The implement loop runs one scenario at a time:
  RED (real failing assertions) → GREEN (minimum code) → REFACTOR.
- **Completeness-based coverage gate.** Every uncovered line is classified as
  dead code (delete), missing test (add scenario / fix steps), or
  legitimately-hard-to-test (justify). There is no percentage threshold.
- **CSS-style cascade config.** The binary ships sensible embedded defaults.
  Your project's `givn/` directory overrides any facet (template, instructions,
  deps, skill templates) — and only the facets you eject. Binary updates never
  clobber your files.
- **Standards-aligned agent integration.** Invokable skills are emitted to the
  neutral `.agents/skills/<name>/SKILL.md` path, with mirrors for Claude Code
  and opencode. `AGENTS.md` at the repo root is otherwise left for the user to
  author. The one exception: `givn init` and `givn skills sync` maintain a
  short, fenced `<!-- givn:begin:givn-project -->` block in it — a pointer
  telling any agent that this project uses givn and to run
  `givn instructions` for what to do next. Nothing outside that block is
  ever touched. Opt-in addons (e.g. `absolute_mode`) manage their own
  separate fenced blocks the same way.
- **Context-aware entry point.** `givn instructions` (no arguments) inspects
  the project's current state — no `givn/` yet, no active changes, one
  change mid-workflow, several active changes — and tells you exactly which
  command or skill to run next. `givn instructions <artifact>` is unchanged:
  it prints that artifact's writing instructions.
- **Self-documenting, versioned config.** `givn init` writes `givn/config.yaml`
  with every key present-but-commented, documented inline (description, default,
  allowed values). It carries `givn_config_version`; `givn upgrade` provides a
  version-to-version upgrade path that preserves your overrides.
- **Lint and test are separate, by design.** `givn lint` is a built-in command
  that statically checks your `.feature` files (syntax, `@wip`, delta tags) and
  **runs no tests**. Running the actual tests is a separate command you configure
  as `verify.command` in `givn/config.yaml`. That command MUST be a
  Gherkin/Cucumber runner that executes the `.feature` files (e.g.
  `cargo test --test features_runner` with `cucumber-rs`, `behave`, or
  `npx cucumber-js "givn/specs/**/*.feature"`) — a bare unit-test runner
  (`cargo test` alone, bare `pytest`) is not sufficient, because the
  `.feature` file is the executable spec.
- **Self-contained gates via hooks.** Artifacts can declare `post` hooks; by
  default `tasks` and `review` run `verify.command` after they complete. `givn
  check <artifact>` runs those hooks, and `givn archive` runs the hooks of every
  required artifact as the gate — so a change can only archive when the suite is
  green. Hooks are fully configurable (add `lint`, custom shell commands, or
  `allow_failure`).
- **No silent test running.** The default `verify.command` is the sentinel
  `givn missing-testrunner`, which prints per-language setup guidance and exits
  non-zero until you configure a real runner — so nothing ever falsely reports green.
- **Configurable workflow (artifacts are data).** Add, remove, reorder, or make
  optional any lifecycle step purely via `givn/config.yaml` — e.g. drop the review
  gate or add a dedicated refactor step after implementation. No fork required.
- **Spec/implementation separation.** Proposals and `.feature` files describe
  *what* and *why* (implementation-independent, domain language); *how* lives in
  design and tasks. The default instructions enforce the boundary.
- **Opt-in addons.** Enable extras via `addons:` — e.g. the bundled `arc42`
  addon adds an `arc42-docs` step (after design) that creates and maintains
  durable project-level arc42 architecture documentation under `docs/arc42/`.

## The workflow

```
propose → spec → design → design-review → tasks → implement → review → archive
```

`archive` is gated: `apply.requires: [tasks, review]`.

- **design-review** runs after design but before tasks: a grilling subagent reads
  the full plan and returns ranked questions with recommended answers; the
  orchestrator presents them one at a time to the user; a hardening subagent
  applies decisions and writes `design-review.md`. Gate: `DESIGN-REVIEW: PASS`.
- **review** is the post-implementation coverage review: run tests, classify
  every gap, resolve. Gate: `REVIEW: PASS`.

### Reverse-engineering an existing codebase

When a codebase has no givn specs yet (e.g. for the first dogfood),
`/givn-reverse-engineer` creates the **initial baseline spec** by
characterizing observed behaviour. Reverse-engineering is a **peer flow** to
the normal change lifecycle — it does not produce proposals or designs, only
descriptive Gherkin specs and characterization tasks:

```
reverse-engineer → characterize → archive (creates baseline spec)
```

- **reverse-engineer** discovers a capability and its black-box boundary
  interactively, then writes descriptive `.feature` scenarios (AS-IS behaviour)
  and characterization tasks. No proposal, no design, no design-review — the
  boundary and suspected bugs go in the Feature: description block; seam
  strategy goes per-task in tasks.md. `givn new --skip proposal,design,design-review`
  skips the unneeded artifacts so `givn status` shows them as ⊘ skipped.
- **characterize** executes the RED/GREEN/REFACTOR loop (shared mechanics via
  the `givn-steps` skill): writes step definitions matching observed behaviour,
  diagnoses spec/implementation mismatches (corrects the spec, never the code),
  applies testability seams, and runs a reverse review (no dead-code deletion,
  bugs flagged not fixed). Suspected bugs and code smells are marked in
  production code as `TODO: REVERSE-ENGINEERING NOTE:` comments for IDE
  discoverability.
- On archive, the all-`@givn.added` delta merges into an empty permanent spec,
  creating the initial `givn/specs/<cap>/<cap>.feature` baseline.

After the baseline exists, future changes use the normal workflow
(`/givn-propose` with deltas against the baseline).

## Install (planned)

No npm. No root. No runtime. Pick whichever you like:

| Method | Command | Audience |
| --- | --- | --- |
| Release binary | download from GitHub releases (linux/macos x64+arm64, windows-x64) | everyone |
| Shell installer | `curl -fsSL https://.../install.sh \| sh` | anyone with a terminal |
| Homebrew | `brew install <tap>/givn` | macOS + Linux |
| Cargo | `cargo install givn` | Rust developers |

> The crate name `givn` is verified available on crates.io and Homebrew.

## Quickstart

```sh
givn init                       # scaffold givn/ + generate skills
givn instructions               # context-aware: what to do next, given current state
givn new add-user-auth          # create a change scaffold
givn new reverse-cap --skip proposal,design,design-review  # create a change skipping artifacts (reverse-engineering)
givn instructions proposal --change add-user-auth --json
givn lint                       # static check of .feature files (exit 0 clean / 1 parse / 2 wip) — runs NO tests
givn check tasks                # run an artifact's post hooks (e.g. the test suite via verify.command)
givn status                     # show lifecycle state (derived from the filesystem)
givn status --change add-user-auth --json   # machine-readable state for commands/scripts
givn apply --change add-user-auth     # track task progress
givn archive --change add-user-auth   # merge deltas into givn/specs/, move to archive/
givn eject design:template      # pull an embedded default into your project to edit
givn skills sync                # regenerate skills after a binary update
givn upgrade                    # refresh documented config to the new givn version
givn-explore                    # think through a problem before proposing (run as a skill)
givn-design-review              # stress-test the plan before implementing (run as a skill)
givn-steps                      # shared step-writing mechanics reference (read by implement/characterize)
givn-reverse-engineer           # reverse-engineer an existing capability into baseline specs (run as a skill)
givn-characterize               # execute the characterization loop + reverse review (run as a skill)
givn addons list                # show available addons (e.g. arc42) and their status
givn addons enable arc42        # enable the arc42 documentation addon
givn completions bash           # print bash completion script (eval in ~/.bashrc)
givn completions zsh            # print zsh completion script  (eval in ~/.zshrc)
givn completions fish           # print fish completion script (source or save to completions/)
```

> **First thing after `init`:** set `verify.command` in `givn/config.yaml` to a
> Gherkin/Cucumber runner that executes your `.feature` files (e.g.
> `cargo test --test features_runner` with `cucumber-rs`, `behave`,
> `npx cucumber-js "givn/specs/**/*.feature"`). A bare unit-test runner
> (`cargo test` alone, `pytest`) is NOT sufficient — the `.feature` file is the
> executable spec, so the runner must consume it. Until you configure a real
> runner, the default is the sentinel `givn missing-testrunner`, which prints
> setup guidance and fails on purpose so nothing falsely reports green.

> `givn/config.yaml` ships **fully commented** — every option is documented inline
> with its default and allowed values. It is **versioned** (`givn_config_version`);
> `givn upgrade` refreshes the documentation scaffold to a newer givn release while
> preserving your overrides, with `git diff` as the review/merge surface.

## What the tool creates in your project

```
givn/
├── config.yaml                       # documented + versioned: overlay + verify.command
├── specs/<cap>/<cap>.feature         # permanent merged spec (source of truth)
├── changes/<id>/                     # active change (state = which files exist)
│   ├── proposal.md
│   ├── specs/<cap>/<cap>.feature     # delta: @givn.delta + added|modified|removed
│   ├── design.md  tasks.md  design-review.md  review.md
│   └── arc42.md                      # optional: arc42-docs addon marker (STATUS: DONE)
├── archive/<id>/                     # archived changes
└── (ejected overrides, on demand)
    artifacts/<id>/template.*
    artifacts/<id>/instructions.md
    skills/<name>/SKILL.md.tmpl
.agents/skills/<name>/SKILL.md        # neutral skills (primary)
.claude/skills/<name>/SKILL.md        # mirror for Claude Code
.opencode/skills/<name>/SKILL.md      # mirror for opencode
AGENTS.md                             # user-owned; givn only maintains its own
                                       #   fenced <!-- givn:begin:givn-project -->
                                       #   block (+ <!-- givn:begin:absolute-mode -->
                                       #   if that addon is enabled)
```

## Documentation

- [`docs/arc42/`](docs/arc42/README.md) — full architecture documentation
  (arc42), one file per chapter. Start with the index.
  - [Introduction & Goals](docs/arc42/01-introduction-and-goals.md)
  - [Solution Strategy](docs/arc42/04-solution-strategy.md)
  - [Cross-cutting Concepts](docs/arc42/08-crosscutting-concepts.md) — the core mechanics.
  - [Architecture Decisions](docs/arc42/09-architecture-decisions.md) — every decision, with rationale.
- [`docs/TASKS.md`](docs/TASKS.md) — phase-by-phase implementation task breakdown.

## License

TBD (intended open source for the broader community).
