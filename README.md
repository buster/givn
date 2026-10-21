<!-- givn:begin:coverage-badge -->
[![Line Coverage: 84%](https://img.shields.io/badge/line%20coverage-84%25-brightgreen)](coverage/cobertura-coverage.xml)
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

- **The `.feature` file IS the specification.** The same file is the change's
  delta document, the executable test your runner executes, and — after
  archiving — the permanent spec under `givn/specs/`. No parallel `spec.md`.
- **Rules are served, not copied.** The rules an agent follows for a step
  come from the instruction served by `givn instructions <id>`; each
  installed skill and command carries its trigger and orchestration and
  directs the agent there. A project extends any instruction with its own
  rules in `givn/config.yaml` — synchronization and upgrades never fight
  them.
- **One change lifecycle.** propose → spec → design → design-review → tasks →
  implement → review → archive. Archive is forward-only and SCM-backed (Git or Fossil): it
  commits the merged archive candidate, runs each verification scope once
  against that commit, and records a verification receipt; a failed candidate
  stays in place and is repaired forward. Review names the root cause of an
  invalid use-case document first and then all actionable schema and
  capability-routing problems in one pass, and review/archive reuse
  a green change verification receipt when a later revision leaves the tested
  content unchanged; the merged archive suite still runs once. Tagged delta
  scenarios merge into the permanent specs and the tags are stripped. Optionally preceded by
  `/givn-ideation` — a domain-driven event storming flow that turns a vague
  idea into change-ready use cases.
- **A development record and report.** Givn records the steps it runs and the
  activity each supported coding-agent harness reports — tool calls, subagent
  spans, question waits, and token usage — in one local, append-only record
  kept out of repository history. `givn history` reads it (text or `--json`),
  and `givn timeline` renders one change or ideation topic as a self-contained
  interactive report: the workflow top to bottom, each phase's vertical
  session lanes and lane log, the longest steps, and where the time went.
- **An opt-in outline report and review gate.** The default-off
  `outline` addon adds `givn outline report`: an advisory snapshot of
  production Rust units that prints each unit's outline — method declarations,
  signatures, and internal calls — first, then size, public surface,
  cohesion/coupling, and method-complexity metrics as hints. When the addon is enabled,
  `givn check review` prints the outline of every touched unit with methods and
  requires a fresh-context reviewer's per-unit `OUTLINE: PASS`, with a
  responsibility and call-order explanation, or `FAIL`. Metrics are hints;
  an unreadable outline has no justification path. No baseline is
  stored. See [the outline addon guide](docs/outline.md#outline-comprehension--the-core-idea)
  for an example outline, the phases, expected outcome, and warning signs.
- **Scenario-by-scenario TDD with proof.** Each scenario runs RED (targeted,
  genuinely failing) → GREEN (minimum code) → REFACTOR → one atomic commit.
  The runner's strict mode is proven up front so an empty step definition can
  never report GREEN.
- **Blocking gates.** Artifact post hooks (`verify`, `verify-e2e`, `lint`),
  integrity checks (commit hashes, promised files), overlap dispositions, a
  visual review gate (a declared visual interface needs a `VISUAL-REVIEW`
  verdict, a confirmed-persona disposition, and committed per-viewport and
  per-state evidence; a deferral needs a recorded operator decision), and a
  README completeness gate: `givn readme check` reports open scaffold
  placeholders and managed-block damage, the review records a
  machine-checkable README impact decision, and the archive refuses an
  unfinished README.
- **Runner-agnostic.** A standalone Rust binary driving any Cucumber-family
  runner (cucumber-rs, behave, cucumber-js, …); it does not replace it. Skills
  are emitted for AI agents in `agents` neutral plus `claude`, `opencode`,
  `omp`, and `maki` formats.
- **Configurable without forking.** Embedded defaults overlaid CSS-cascade-style
  by `givn/config.yaml`; the workflow graph itself (steps, order, hooks) is
  data. One feature-free binary — `[features] default = []`.

## Install

Build from source (not published to crates.io or as release binaries). Requires
a recent stable Rust toolchain; installs to `~/.cargo/bin`.

```sh
git clone https://github.com/buster/givn.git
cd givn
cargo install --path .                        # the single feature-free binary
```

## Quickstart

Run "givn init" in your repository, then /givn-propose in your agent. Let the agent handle the rest.

```sh
git init
givn init                             # scaffold givn/ + generate skills + AGENTS.md hint
/givn-ideation my-domain              # (optional, in your AI agent) event-storm a vague idea into use cases
/givn-propose add-user-auth           # (in your AI agent) proposal → spec → design → tasks
/givn-implement                       # (in your AI agent) scenario-by-scenario TDD loop
/givn-review                          # (in your AI agent) coverage review, sign off
/givn-archive
```

Before the first archive, set `verify.command` in `givn/commands.yaml` to a
Gherkin runner that executes the `.feature` files; the shipped sentinel fails
on purpose until you do.

Watch the work as it happens, or afterwards: `givn timeline --change <id>` (or
`--topic <id>`) writes one self-contained HTML report of the development
record — the workflow top to bottom, each phase's lanes and lane log, and where
the time went — and `givn history --change <id> --json` prints the record
itself. The record lives under `givn/history/` as local runtime state and stays
out of repository history.

[![Development report of an example change: current state, KPI tiles, and the top-to-bottom workflow phase stack.](docs/development-report-mobile.png)](docs/development-report.md)

`givn init` asks which agent targets receive the generated skills and commands;
the list opens with `.opencode` marked and re-asks until at least one target is
marked. Scripted runs state the selection explicitly:
`givn init --agents opencode,claude,omp` (valid targets: `opencode`, `agents`,
`claude`, `omp`, `maki`) — a missing or unknown-only selection is
refused before anything is written. `givn skills sync` keeps writing to the
persisted selection and falls back to `opencode` only for a config without a
skills block.

Commit each scenario with `givn commit -m "feat(<capability>): <release note>"`:
the revision message carries the release note as its message subject, with the
scenario title in the message body. It works on Git and Fossil, records the
named paths or every pending change, and stamps the managed attribution
trailer when the harness provides context.

`givn init` checks in shared Git commit-attribution plumbing and activates it
locally when the project is a Git worktree; a Fossil checkout receives no hook
and reports `commit attribution: givn commit path only` (a raw `fossil commit`
carries no attribution). OpenCode, Claude Code, and omp targets receive
best-effort runtime integrations; when an agent exposes attribution context,
the commit committer and message show the LLM, reasoning mode, harness, and
Givn version.
Missing context never blocks a human commit.

## Default workflow

![Givn change lifecycle: proposal to archive](docs/workflow.svg)

## Documentation

- [`docs/INDEX.md`](docs/INDEX.md) — the documentation index: every durable
  document under `docs/`, current or archived.
- [`docs/workflow.md`](docs/workflow.md) — the process in detail: every step in
  order, what is created, what is checked, and how givn enforces process
  quality.
- [`docs/reference.md`](docs/reference.md) — full CLI reference, agent skills,
  addons, configuration keys, project layout, dev scripts.
- [`docs/architecture-principles.md`](docs/architecture-principles.md) — the
  opt-in architecture-principles experiment: last-responsible-moment decision
  timing, consumer contracts now, component splits later, the phases it affects,
  and the signals that its rules need revision.
- [`docs/outline.md`](docs/outline.md) — the experimental outline
  addon: the advisory report, the change-relative review gate, its phases, and
  the warning signs that the rules are not working.
- [`docs/development-report.md`](docs/development-report.md) — the record
  settings (mode, default, privacy), the report screens, and analytics over the
  JSONL store with jq, SQLite, and DuckDB.
- [`docs/arc42/README.md`](docs/arc42/README.md) — architecture documentation,
  one file per chapter.
- [`CHANGELOG.md`](CHANGELOG.md) · [`docs/migrations/README.md`](docs/migrations/README.md) —
  release history and upgrade guides.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).

As a special exception under section 7 of the GNU General Public License,
version 3, text that givn generates, writes, or emits — the managed blocks
it maintains in files such as `AGENTS.md` and `README.md`, the generated
rules (skills, commands, plugins, hooks), the instruction text it prints,
and the scaffolds it instantiates such as `commands.yaml` and the `run`
block — is not subject to the conditions of this license. You may use,
modify, and distribute that text under terms of your choice, and it does
not affect the license of the project it appears in. The exception covers
givn's output, not givn itself. It does not apply when that text, or a
modified version of it, is used as the rules, skills, or instructions of
another spec-driven workflow tool; in that use, the text remains subject
to the conditions of this license.

<!-- givn:begin:coverage -->
## Coverage

Merged report: [coverage/cobertura-coverage.xml](coverage/cobertura-coverage.xml)

Line coverage: 84% (15726/18679)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
