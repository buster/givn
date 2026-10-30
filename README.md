<!-- givn:begin:coverage-badge -->
[![Line Coverage: 79%](https://img.shields.io/badge/line%20coverage-79%25-brightgreen)](coverage/cobertura-coverage.xml)
[![Branch Coverage: n/a](https://img.shields.io/badge/branch%20coverage-n--a-brightgreen)](coverage/cobertura-coverage.xml)
<!-- givn:end:coverage-badge -->

```
  ________  .__
 /  _____/  |__|__  __ ____
/   \  ___  |  \  \/ //    \
\    \_\  \ |  |\   /|   |  \
 \______  / |__| \_/ |___|  /
        \/                \/

  from Given to done.
```

An opinionated, agent-assisted development method built around executable Gherkin specifications.

## What it is

- **Executable behavior, durable context.** A `.feature` file is the change
  delta, the executable test the runner executes, and — after archiving — the
  permanent behavior specification under `givn/specs/`. Its owning use case
  records actor goals and guarantees; Arc42 records durable implementation
  structure. None is a parallel `spec.md`.
- **One change lifecycle.** proposal → spec → design → design-review → tasks →
  implementation → review → archive. State is derived from files on disk; there
  is no separate state database. With the arc42 addon enabled, the design step
  also maintains durable architecture documentation.
- **Skills are the agent entry surface.** `givn skills sync` generates skills
  for the selected agent targets. Each skill carries a trigger and points the
  agent at the rules served by `givn instructions <id>`; rules are not copied
  into the skill. Project rules extend the served instruction through
  `extensions:` in `givn/config.yaml`.
- **Genuine test-first behavior batches.** Related scenarios form one batch with
  an observed RED, a minimum GREEN, optional REFACTOR, and one coherent commit
  with explicit paths. Existing requirements may be protected by immediately
  passing regression tests without a production edit for them.
- **Forward-only, SCM-backed archive.** Archive publishes the merged candidate
  on Git or Fossil, runs the regular and E2E verification scopes against it
  under a fail-closed runner-result contract, and records a verification
  receipt. A failed candidate remains in place and is repaired forward.
- **Advisory structure evidence.** The `outline` addon produces a
  deterministic report of production Rust and Python units: each unit's
  per-component call tree first, then size, cohesion, coupling, and method
  metrics as hints. It never blocks review or archive and stores no baseline.
- **A local development record.** Givn records its own commands and phases plus
  the activity supported coding-agent harnesses report — tool calls, turns,
  subagent spans, question waits, and token usage — in an append-only local
  store. `givn history` reads it and `givn timeline` renders one change or
  ideation topic as a self-contained HTML report.
- **Configuration without forking.** Embedded defaults are overlaid by
  `givn/config.yaml` for policy, graph, addons, targets, and extensions, and by
  `givn/commands.yaml` for test, coverage, and product-start commands. Every
  registered addon is enabled by default; an explicit `false` is the opt-out.

## Install

Build from source (not published to crates.io or as release binaries). Requires
a recent stable Rust toolchain; installs to `~/.cargo/bin`.

```sh
git clone https://github.com/buster/givn.git
cd givn
cargo install --path . --locked
```

## Before initialization

`givn init` writes Givn configuration and scaffolds, generated skills for the
selected agent targets, and enabled documentation scaffolds. It also installs
commit-attribution integration, applies the configured local-only tracking
policy, and may merge supported harness configuration. Generated output is
refreshed by `givn skills sync`; see the [reference](docs/reference.md) for the
exact paths, ownership, and regeneration rules.

Development recording is enabled by default. It records bounded activity
metadata and question text under `givn/history/`; it is ignored by default, not
private. Set `record.mode: off` before work that must not create new entries.
See [development-record settings](docs/development-report.md#settings).

## Quick start: a fresh project

Create a version-controlled project, then initialize Givn:

```sh
givn init
```

In your agent, invoke the generated exploration skill and describe what you want
to build:

| Harness | Invocation |
|---|---|
| Claude Code, OpenCode | `/givn-explore` |
| OMP | `/skill:givn-explore` |
| neutral `agents` target | skill discovery by name |
| Maki | an explicit request, or an agent-selected `givn-explore` skill |

For example: “I want a CLI that validates CSV files and reports invalid rows.”
Explore investigates the request without artifacts, then routes to ideation when
the domain needs durable discovery or to proposal when a concrete change is
justified. Experienced users may enter ideation or proposal directly.

`givn-propose` creates the change and continues through specs and design. The
complete lifecycle is proposal → spec → design → design-review → tasks →
implementation → review → archive. During the first useful product change, the
agent establishes the project's Gherkin runner, coverage commands, and product
run declaration. Runner/bootstrap failures are setup failures; an observed RED
for a behavior must fail its assertion after that setup works.

The configured runner must execute `GIVN_FEATURES`, handle the `change`,
`regular`, and `e2e` scopes, and write fresh actual scenario counts. The
[runner contract](docs/reference.md#runner-contract) is fail-closed. Givn does
not invent project runner or start scripts.

## Adopting Givn in an existing project

Use an initial formal change to establish Givn's canonical project baseline:
migrate architecture, domain, and persona material into their canonical owners;
record explicit unknowns; adapt existing test/startup scripts; and establish
meaningful E2E testing through the product's public boundary. Retire or redirect
superseded documentation rather than maintaining parallel authorities. This does
not require executable characterization of every legacy behavior. The
[workflow guide](docs/workflow.md#initial-adoption-and-artifact-ownership)
explains the ownership and migration path.

## Configuration and attribution

Every registered addon is enabled by default; set its value to `false` in
`givn/config.yaml` to opt out. `givn/commands.yaml` owns runner, coverage, and
product-start commands. The [reference](docs/reference.md) distinguishes feature
toggles, graph customization, configuration ownership, and files retained after
a feature is disabled.

`givn commit -m "<release note>" <explicit paths...>` creates one revision
through the selected backend and preserves unrelated staged and working-tree
changes. Givn does not require Conventional Commits syntax, but formal-change
commits must carry the batch or scenario traceability required by the workflow.
Git attribution integration is installed and activated separately from whether
its generated hook is versioned; Fossil attribution is available through
`givn commit`. Missing attribution context never blocks a commit.

## Documentation

- [`docs/INDEX.md`](docs/INDEX.md) — the documentation index, current and
  historical.
- [`docs/workflow.md`](docs/workflow.md) — the lifecycle overview and gates.
- [`docs/reference.md`](docs/reference.md) — authoritative CLI, configuration,
  skills, runner, and recovery contracts.
- [`docs/outline.md`](docs/outline.md) — the advisory outline report: examples,
  supported languages, limitations, and how it supports REFACTOR and review.
- [`docs/development-report.md`](docs/development-report.md) — the local record,
  its settings, and analytics over the JSONL store.
- [`docs/use-case-model.md`](docs/use-case-model.md) — use-case identity,
  scoping, and review criteria.
- [`docs/arc42/README.md`](docs/arc42/README.md) — current architecture and the
  ADR register.
- [`CHANGELOG.md`](CHANGELOG.md) · [`docs/migrations/README.md`](docs/migrations/README.md) —
  release history and the manual 1.0 transition.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

As a special exception under section 7 of the GNU General Public License,
version 3, text that givn generates, writes, or emits — the managed blocks
it maintains in files such as `AGENTS.md` and `README.md`, the generated
skills, integrations, and hooks, the instruction text it prints, and the
scaffolds it instantiates such as `commands.yaml` and the `run` block — is not
subject to the conditions of this license. You may use, modify, and distribute
that text under terms of your choice, and it does not affect the license of the
project it appears in. The exception covers givn's output, not givn itself. It
does not apply when that text, or a modified version of it, is used as the
rules, skills, or instructions of another spec-driven workflow tool; in that
use, the text remains subject to the conditions of this license.

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 79% (16678/21177)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
