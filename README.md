<!-- givn:begin:coverage-badge -->
[![Line Coverage: 81%](https://img.shields.io/badge/line%20coverage-81%25-brightgreen)](coverage/cobertura-coverage.xml)
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
  stays in place and is repaired forward. Tagged delta scenarios merge into
  the permanent specs and the tags are stripped. Optionally preceded by
  `/givn-ideation` — a domain-driven event storming flow that turns a vague
  idea into change-ready use cases.
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
  are emitted for AI agents in `agents` neutral plus `claude`, `opencode`, and
  `omp` formats.
- **Configurable without forking.** Embedded defaults overlaid CSS-cascade-style
  by `givn/config.yaml`; the workflow graph itself (steps, order, hooks) is
  data. One feature-free binary — `[features] default = []`.

## Install

Build from source (not published to crates.io or as release binaries). Requires
a recent stable Rust toolchain; installs to `~/.cargo/bin`.

```sh
git clone <this repository's URL>
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

`givn init` asks which agent targets receive the generated skills and commands;
the list opens with `.opencode` marked and re-asks until at least one target is
marked. Scripted runs state the selection explicitly:
`givn init --agents opencode,claude,omp` (valid targets: `opencode`, `agents`,
`claude`, `omp`) — a missing or unknown-only selection is
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

- [`docs/workflow.md`](docs/workflow.md) — the process in detail: every step in
  order, what is created, what is checked, and how givn enforces process
  quality.
- [`docs/reference.md`](docs/reference.md) — full CLI reference, agent skills,
  addons, configuration keys, project layout, dev scripts.
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

Line coverage: 81% (8225/10207)
Branch coverage: n/a (0/0)

Reproduce the result:

```sh
./measure-coverage.sh
./merge-coverages.sh
```
<!-- givn:end:coverage -->
