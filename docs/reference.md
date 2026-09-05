# Reference

Factual reference material extracted from the README: CLI commands, agent
skills, addons, extras, configuration keys, generated project layout, and this
repository's development scripts.

## CLI commands

| Command | Purpose |
| --- | --- |
| `givn init [--force] [--no-extras] [--no-addons]` | Scaffold `givn/{specs,changes,archive}`, write `givn/config.yaml`, generate skills/commands, add the AGENTS.md hint block. Enables all addons and extras by default. |
| `givn new <id> [--skip a,b,c]` | Create a change scaffold under `givn/changes/<id>/`. `--skip` marks artifacts as intentionally not produced (reverse engineering). |
| `givn instructions [<artifact>] [--change <id>] [--json]` | No artifact: context-aware guidance on what to do next. With an artifact id: that artifact's writing instructions. |
| `givn lint [--change <id>] [paths...]` | Static Gherkin check only (syntax, `@wip`, delta tags). Exit 0 clean / 1 parse error / 2 `@wip` present. Runs no tests. |
| `givn check [<artifact>] [--change <id>]` | Run an artifact's `post` hooks on demand; blocking `givn check review --change <id>` also runs the semantic report gate. |
| `givn status [--change <id>] [--json]` | Lifecycle state: artifact checklist, current stage, task progress with pending task descriptions, next required/optional steps. Filesystem-derived, no separate state file. |
| `givn graph [--change <id>] [--json]` | Show the artifact workflow graph: nodes, `requires` edges, optional/gates-archive flags, post hooks; overlays live status with `--change`. |
| `givn archive --change <id>` | Archive a completed change. Gated: `apply.requires` artifacts must exist, `REVIEW: PASS` present, all their hooks pass; the run declaration must be complete. |
| `givn run [--port <n>] [--print]` | Start the product declared in the `run:` block of `givn/commands.yaml`. Hands the port to the product through `port_env` (ephemeral port when the caller passes none), prints `givn run: ready at <url>` once readiness is observed, forwards SIGINT/SIGTERM (exit 130/143), and forwards the product's exit code. `--print` emits one sh-eval-able start line without starting. |
| `givn eject <target> [--force]` | Write an embedded default facet into the project for editing (`design:template`, `review:instruction`, `skill:givn-propose`, `manifest`). |
| `givn addons list` / `enable <name>` / `disable <name>` | Manage opt-in addon artifact/policy sets. |
| `givn extras install` / `uninstall` | Enable/disable the curated extras skill+command bundle. |
| `givn skills sync` | Regenerate all skills, commands, and AGENTS.md blocks from current templates. |
| `givn upgrade` | Refresh and commit managed config/agent files in a clean Git project; apply declared migration bundles when required; backfill the run-declaration scaffold into an existing `givn/commands.yaml` when the `run:` key is absent (idempotent, committed with the managed upgrade). |
| `givn completions <bash\|zsh\|fish>` | Print a shell completion script. |

## Agent skills

Emitted by `givn init` / `givn skills sync` to `.agents/skills/<name>/SKILL.md`
(neutral), with mirrors for Claude Code (`.claude/skills/`) and opencode
(`.opencode/skills/`, `.opencode/commands/`):

| Skill | When to use it |
| --- | --- |
| `givn-explore` | Think through ideas, investigate problems, clarify requirements before proposing a change. |
| `givn-ideation` | Work a vague idea into change-ready use cases — structured, multi-session domain exploration with personas, reviews, and change seeds. The artifact-producing alternative to `givn-explore`. |
| `givn-propose` | Write the proposal — WHAT and WHY, in domain language, no implementation detail. |
| `givn-spec` | Write the Gherkin `.feature` delta — observable behaviour, no implementation detail. Includes retrieval-aware authoring guidance (`spec index`, `spec search`, `spec explore`). |
| `givn-design` | Write the technical design — the HOW layer: tech decisions, architecture, step-def locations. |
| `givn-design-review` | Stress-test the plan one question at a time before tasks are written; harden the artifacts; sign off. |
| `givn-tasks` | Break the design down into a scenario-by-scenario RED/GREEN/REFACTOR/COMMIT task list. |
| `givn-implement` | Execute the TDD loop, one scenario at a time, ending in an atomic commit per scenario. |
| `givn-steps` | Shared step-writing mechanics (step reuse, RED/GREEN/REFACTOR loop, never-empty-step rule) — referenced by `givn-implement` and `givn-characterize`. |
| `givn-review` | Complete the coverage review: fabrication audit, gap classification, sign-off. |
| `givn-archive` | Archive a completed change: gate hooks, merge deltas, move to `archive/`. |
| `givn-reverse-engineer` | Discover an existing capability's black-box boundary and write descriptive baseline specs. |
| `givn-characterize` | Execute the characterization loop against existing behaviour, with testability seams and a reverse review. |

## Addons

Opt-in sets enabled via `givn addons enable <name>` or `addons: { <name>: true }`
in `givn/config.yaml`:

| Addon | Effect |
| --- | --- |
| `arc42` | Adds an `arc42-docs` artifact (after `design`) maintaining durable arc42 documentation under `docs/arc42/`; adds the integrity arc42 conformance check. |
| `absolute_mode` | Manages a fenced block in `AGENTS.md` with a blunt, no-filler communication policy. |
| `readme` | Scaffolds an initial project README (existing content preserved). |
| coverage gate | Archive-level addon: runs measurement/merge scripts, validates Cobertura output, updates managed README badges/blocks, rolls back on failure. |

## Extras

Curated opt-in bundle of project-level skills and commands, independent of the
change lifecycle. Enabled by default on `givn init` (disable with
`--no-extras`); toggle with `givn extras install` / `uninstall`.

| Extra | Kind | Purpose |
| --- | --- | --- |
| `givn-domain-modeling` | skill | Model a change's domain before designing — ubiquitous language, aggregates, invariants. |
| `givn-dev-principles` | skill | Engineering principles while implementing — correctness first, boring when called for, no unnecessary abstractions. |
| `/grill-me` | command | Socratically grill the current plan or design, one hard question at a time. |
| `/grill-with-docs` | command | Grill the plan or design against the project's own docs and specs, citing sources. |

## Configuration

`givn/config.yaml` is generated fully-commented by `givn init`: commented
lines are the managed documentation scaffold (refreshed by `givn upgrade`);
uncommented key/value lines are user-owned and never touched by upgrades.
Carries `givn_config_version`.

The only value that must be set for a working project is `verify.command`.

Key sections:

- `verify.command` / `verify.e2e_command` — test runner(s). Must be a
  Gherkin/Cucumber runner executing the `.feature` files, not a bare
  unit-test command.
- `verify.features_path` — where the runner reads `.feature` files from
  (default `givn/specs`).
- `coverage.measure_command` / `coverage.merge_command` — project-owned
  scripts producing Cobertura reports for the archive coverage gate.
- `run:` — the run declaration: `command`, `product_type` (`server`/`cli`/
  `library`), exactly one of `port` (fixed) or `port_env` (the caller hands
  the port over) for servers, `ready_path`, and `ready_timeout_s`. Validated
  by rules R1–R10; gating `givn check review` and `givn archive`; executed by
  `givn run`; documented in the README's run managed block.
- `artifacts:` — patch the embedded workflow graph: remove/reorder steps, make
  steps optional, add custom steps with own template and instruction.
- `apply.requires` — which artifacts gate `givn archive` (default
  `[tasks, review]`).
- Hooks (`post:` per artifact) — built-ins `verify`, `verify-e2e`, `lint`,
  `integrity`, or any shell command; object form supports `allow_failure` and
  display `name` (built-in hook names always ignore `allow_failure`).
- `addons:` / `extras:` — see tables above.

Semantic retrieval knobs are environment variables, not config keys — see
[`retrieval-models.md`](retrieval-models.md).

## What givn creates in your project

```
givn/
├── config.yaml                       # documented + versioned: overlay + verify.command
├── commands.yaml                     # project-owned test and coverage scripts
├── specs/<group>/<cap>.feature       # permanent merged spec (source of truth)
├── .spec-index.json                  # E5 embedding index (incremental, content-hashed)
├── changes/<id>/                     # active change (state = which files exist)
│   ├── proposal.md
│   ├── specs/<group>/<cap>.feature   # delta: @givn.delta + added|modified|removed
│   ├── design.md  tasks.md  design-review.md  review.md
│   ├── semantic-review.md            # generated evidence/classification worklist
│   └── arc42.md                      # optional: arc42 addon marker (STATUS: DONE)
├── archive/<id>/                     # archived changes, reports retained
└── (ejected overrides, on demand)
    artifacts/<id>/template.*
    artifacts/<id>/instructions.md
    skills/<name>/SKILL.md.tmpl

.agents/skills/<name>/SKILL.md        # neutral skills (primary)
.claude/skills/<name>/SKILL.md        # mirror for Claude Code
.opencode/skills/<name>/SKILL.md      # mirror for opencode
.opencode/commands/<name>.md          # opencode slash commands

AGENTS.md                             # user-owned; givn maintains only its own
                                      #   fenced blocks at the top:
                                      #   <!-- givn:begin:givn-project -->
```

Project command scripts live at the repo root and are referenced by
`givn/commands.yaml`. Conventional names: `run-tests.sh`,
`measure-coverage.sh`, `merge-coverages.sh`.

## Repository development scripts

This repository's own verify commands:

```sh
./run-tests.sh            # cargo test matrix across feature combinations
./run-tests.sh --e2e      # @e2e-tagged subsets per feature combination
./measure-coverage.sh     # writes non-E2E and E2E Cobertura reports
./merge-coverages.sh      # writes merged report used by README badges/blocks
```

After archiving every change, update the local binary and regenerate agent
integrations before starting the next one:

```sh
cargo install --path . --locked --force
givn skills sync
givn instructions
./run-tests.sh
./run-tests.sh --e2e
```
