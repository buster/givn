# givn

> from Given to done.

`givn` is a single-binary, Gherkin-first spec-driven workflow tool. It manages a
change lifecycle where **the `.feature` file IS the specification** — the delta
document, the executable test, and the permanent archived record, all in one.

It distills a proven Gherkin-first workflow into a standalone, configurable,
easily-installable Rust binary: delta lifecycle → scenario-by-scenario TDD →
completeness-based coverage review → automatic delta merge on archive.
Language- and test-runner-agnostic.

**Status:** planning complete, implementation not started. See the
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
| A parallel `spec.md` format duplicating the `.feature` | The `.feature` file **is** the spec — nothing duplicated |
| Coverage gated on an arbitrary percentage | Gate is **completeness of gap classification**, not a number |

## Core ideas

- **The `.feature` file is the single source of truth.** No `spec.md`. The same
  file is the delta you write, the cucumber test you run, and (after archive) the
  permanent spec.
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
- **Standards-aligned agent integration.** Rules/context are emitted to
  `AGENTS.md` (the Linux-Foundation-stewarded standard). Invokable skills are
  emitted to the neutral `.agents/skills/<name>/SKILL.md` path, with mirrors for
  Claude Code and opencode.

## The workflow

```
propose → spec → design → tasks → implement → review → archive
```

`archive` is gated: `apply.requires: [tasks, review]` — review must be complete
before a change can be archived.

## Install (planned)

No npm. No root. No runtime. Pick whichever you like:

| Method | Command | Audience |
| --- | --- | --- |
| Release binary | download from GitHub releases (linux/macos x64+arm64, windows-x64) | everyone |
| Shell installer | `curl -fsSL https://.../install.sh \| sh` | anyone with a terminal |
| Homebrew | `brew install <tap>/givn` | macOS + Linux |
| Cargo | `cargo install givn` | Rust developers |

> The crate name `givn` is verified available on crates.io and Homebrew.

## Quickstart (planned CLI)

```sh
givn init                       # scaffold givn/ + generate AGENTS.md and skills
givn new add-user-auth          # create a change scaffold
givn instructions proposal --change add-user-auth --json
givn verify                     # lint .feature files (exit 0 GREEN / 1 parse / 2 wip)
givn status                     # show lifecycle state (derived from the filesystem)
givn apply --change add-user-auth     # track task progress
givn archive --change add-user-auth   # merge deltas into givn/specs/, move to archive/
givn eject design:template      # pull an embedded default into your project to edit
givn skills sync                # regenerate AGENTS.md + skills after a binary update
```

## What the tool creates in your project

```
givn/
├── config.yaml                       # manifest OVERLAY + verify.command
├── specs/<cap>/<cap>.feature         # permanent merged spec (source of truth)
├── changes/<id>/                     # active change (state = which files exist)
│   ├── proposal.md
│   ├── specs/<cap>/<cap>.feature     # delta: @givn.delta + added|modified|removed
│   ├── design.md  tasks.md  review.md
├── archive/<id>/                     # archived changes
└── (ejected overrides, on demand)
    artifacts/<id>/template.*
    artifacts/<id>/instructions.md
    skills/<name>/SKILL.md.tmpl
AGENTS.md                             # generated rules/context (repo root)
.agents/skills/<name>/SKILL.md        # neutral skills (primary)
.claude/skills/<name>/SKILL.md        # mirror for Claude Code
.opencode/skills/<name>/SKILL.md      # mirror for opencode
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
