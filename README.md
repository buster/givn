<!-- givn:begin:coverage-badge -->
[![Line Coverage: 85%](https://img.shields.io/badge/line%20coverage-85%25-brightgreen)](coverage/cobertura-coverage.xml)
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

**Givn** is a Rust CLI that enforces strict, Gherkin-based TDD for AI coding agents.

Instead of letting agents jump straight from a prompt to unverified code, Givn structures every change into a gated workflow (`propose` → `spec` → `design` → `tasks` → `implement` → `review` → `archive`) backed by executable `.feature` files and hard CLI checks.

## Why Givn?

- **Specs are executable tests, not dead docs.** Changes are defined as Gherkin `.feature` deltas. Your test runner executes them directly, and `givn archive` merges passing deltas into your permanent specification under `givn/specs/`.
- **Enforced Red-Green-Refactor.** Agents implement behavior in small batches: observe a failing test (RED), write minimal code to pass (GREEN), refactor, and commit via `givn commit` with explicit file paths.
- **Hard CLI gates.** Agents cannot mark work done on their own. `givn check tasks`, `givn check review`, and `givn archive` verify test receipts, scenario coverage, and review sign-off before a change can land.
- **File-based state.** Workflow state lives entirely in plain files inside your repository (Git or Fossil)—no external database or daemon.
- **Thin agent skills.** `givn skills sync` generates slash commands/skills for Claude Code, OpenCode, OMP, Maki, or generic `AGENTS.md` setups. Rules are served dynamically via `givn instructions <id>` and customized in `givn/config.yaml`.
- **Built-in architecture & code health addons:**
  - `arc42`: Keeps architecture docs (`docs/arc42/`) and ADRs in sync with design changes.
  - `domain_modeling`: Maintains the standalone `docs/glossary.md` and `docs/domain-model.md`; independent of arc42.
  - `outline`: Generates advisory call-tree and coupling/cohesion reports for Rust, Python, Java, TypeScript, and JavaScript that make changed units readable from their declarations, names, and call order, so REFACTOR and review judge comprehension rather than only metrics.
  - `history` / `timeline`: Logs local agent activity (tool calls, turns, token usage) to `givn/history/` and exports self-contained HTML timelines.

## Install

Requires a stable Rust toolchain (installs to `~/.cargo/bin`):

```sh
git clone https://github.com/buster/givn.git
cd givn
cargo install --path . --locked
```

## Quick Start

### 1. Initialize your repository

```sh
givn init
```

`givn init` creates `givn/config.yaml` and `givn/commands.yaml`, generates skills for your chosen coding agent, sets up doc scaffolds, and installs commit-attribution hooks. (To disable local activity logging in `givn/history/`, set `record.mode: off` in `givn/config.yaml`.)

### 2. Start a change from your agent

Invoke the start skill in your coding agent and describe the problem, question, or improvement:

| Agent Harness | Command |
|---|---|
| Claude Code, OpenCode | `/givn-start` |
| OMP | `/skill:givn-start` |
| Maki | Ask to run the `givn-start` skill |
| Generic `agents` target | Skill discovery by name |

> **Example:** *"I want a CLI that validates CSV files and reports invalid rows."*

- **`givn-start`** investigates without writing files and recommends a proportionate outcome: answer and stop, direct work without a Givn change, **ideation** for durable domain work, or **`givn-propose`** for a formal change.
- For a change whose goal and scope are already clear, run `givn new <change-id>` directly.
- **`givn-propose`** scaffolds the change and guides the agent through writing `.feature` specs, design, tasks, and TDD implementation batches.
- On your first change, the agent wires up your project's Gherkin test runner and coverage commands in `givn/commands.yaml` (see the [runner contract](docs/reference.md#runner-contract)).

### Adopting Givn in an existing codebase

You don't need to write tests for every legacy feature up front. Run an initial change to wire up your test runner in `givn/commands.yaml`, migrate existing architecture notes into `docs/arc42/`, and set up one working E2E test boundary. See the [adoption guide](docs/workflow.md#initial-adoption-and-artifact-ownership) for details.

## Projects using Givn

- [`duty2cal`](https://github.com/duty2cal/duty2cal) — Converts a cabin-crew duty plan PDF into a calendar file.
- [`passuff`](https://github.com/buster/passuff) — Multi-account GitHub PR inbox and desktop notifier for the terminal.
- [`watn`](https://github.com/buster/watn) — Turns plain-language questions into shell commands or text.

## Configuration

Add-on configuration is operator-owned. A new change records the effective add-ons in
`givn/changes/<id>/config-lock.yaml`; `givn addons enable|disable` and a changing
`givn init --force` are refused while that change is active. Review and archive
fail if the project configuration drifts from the record. An operator can use
`--operator --reason "<reason>"` on an interactive terminal.

| File / Command | Purpose |
|---|---|
| `givn/config.yaml` | Toggle addons (`false` to opt out), configure agent targets, and extend agent instructions. |
| `givn/commands.yaml` | Define project-specific test runner, coverage, and startup commands. |
| `givn skills sync` | Regenerate agent skill files after changing config. |
| `givn commit -m "<msg>" <paths...>` | Commit explicit paths for a behavior batch without sweeping in unrelated working-tree edits. |

## Documentation

- [`docs/INDEX.md`](docs/INDEX.md) — Full documentation index
- [`docs/workflow.md`](docs/workflow.md) — Lifecycle phases and blocking gates
- [`docs/reference.md`](docs/reference.md) — CLI commands, configuration, and runner contracts
- [`docs/outline.md`](docs/outline.md) — Static code structure reports (Rust, Python, Java, TypeScript, JavaScript)
- [`docs/development-report.md`](docs/development-report.md) — Local activity recording and HTML timelines
- [`docs/use-case-model.md`](docs/use-case-model.md) — Use-case scoping and review criteria
- [`docs/arc42/README.md`](docs/arc42/README.md) — Givn's own architecture and ADR log
- [`CHANGELOG.md`](CHANGELOG.md) · [`docs/migrations/README.md`](docs/migrations/README.md) — Release history and migration guides

### Workflow Overview

![Givn change lifecycle: start to archive](docs/workflow.svg)

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

As a special exception under section 7 of the GNU General Public License, version 3, text that `givn` generates, writes, or emits — the managed blocks it maintains in files such as `AGENTS.md` and `README.md`, the generated skills, integrations, and hooks, the instruction text it prints, and the scaffolds it instantiates such as `commands.yaml` and the `run` block — is not subject to the conditions of this license. You may use, modify, and distribute that text under terms of your choice, and it does not affect the license of the project it appears in. The exception covers `givn`'s output, not `givn` itself. It does not apply when that text, or a modified version of it, is used as the rules, skills, or instructions of another spec-driven workflow tool; in that use, the text remains subject to the conditions of this license.

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 85% (18936/22247)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
