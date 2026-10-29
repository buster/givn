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

A single-binary, Gherkin-first spec-driven workflow tool.

## What it is

- **The `.feature` file is the specification.** The same file is the change's
  delta, the executable test the runner executes, and — after archiving — the
  permanent spec under `givn/specs/`. There is no parallel `spec.md`.
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
  `givn/commands.yaml` for the test, coverage, and product-start commands. Every
  registered addon is enabled by default; an explicit `false` is the opt-out.

## Install

Build from source (not published to crates.io or as release binaries). Requires
a recent stable Rust toolchain; installs to `~/.cargo/bin`.

```sh
git clone https://github.com/buster/givn.git
cd givn
cargo install --path . --locked
```

## Quick start

Run `givn init` in your repository, then invoke the proposal skill in your
agent. The agent loads the rules from `givn instructions` and handles the rest.

```sh
givn init                            # scaffold givn/, write config and commands, generate skills
givn instructions                    # first action of every session: the next required command
```

In your agent, invoke the skills it received:

| Harness | Invocation |
|---|---|
| Claude Code, OpenCode | the skill slash, for example `/givn-propose` |
| OMP | skill syntax, for example `/skill:givn-propose` |
| neutral `agents` target | skill discovery by name |
| Maki | an explicit request, or an agent-selected skill such as `givn-propose` |

Typical sequence: `givn-propose` → `givn-spec` → `givn-design` →
`givn-design-review` → `givn-tasks` → `givn-implement` → `givn-review`, then
`givn archive`. `givn-ideation` is the optional pre-change exploration skill;
`givn-explore` clarifies an idea without producing lifecycle artifacts.

Before the first archive, set `verify.command` in `givn/commands.yaml` to a
Gherkin runner that executes the `.feature` files, and set `run:` if the project
has a startable product. Both fields ship with sentinel commands
(`givn missing-testrunner`, `givn missing-run-command`) that print setup
guidance and exit non-zero. `givn init` does not invent runner or start scripts.

Watch the work as it happens or afterwards:

```sh
givn timeline --change <id>          # self-contained HTML report
givn history --change <id> --json    # the recorded entries
```

The record lives under `givn/history/`, is ignored by the selected backend, and
never enters repository history.

## Agent targets and commit attribution

`givn init` asks which targets receive generated skills: `opencode`, `agents`,
`claude`, `omp`, and `maki`. The interactive prompt opens with `.opencode`
marked and re-asks until something is marked; scripted runs state the selection
explicitly, for example `givn init --agents opencode,claude,omp`. `givn skills
sync` writes only to the persisted selection.

Skills are written under `.opencode/`, `.agents/`, `.claude/`, `.omp/`, and
`.maki/`. OpenCode, Claude Code, omp, and Maki also receive best-effort
development reporters. Maki additionally loads Givn only after the folder is
trusted in Maki.

`givn commit -m "<release note>" <explicit paths...>` creates one revision
through the selected backend. It requires explicit paths and preserves unrelated
staged and working-tree changes. Git also has a checked-in `commit-msg` hook that
stamps attribution for raw Git commits when an OpenCode, Claude Code, or omp
integration supplies context; Fossil check-ins carry attribution only through
`givn commit`. Missing attribution context never blocks a commit. Givn does not
enforce a commit-message convention.

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
