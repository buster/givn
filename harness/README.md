# givn-test-harness

Automated end-to-end test harness for the [givn](..) spec-driven workflow tool.
Treats givn's agent-facing assets — skill templates, artifact instructions, CLI
behaviour — as the system under test. Runs complete autoresearch loops:
an AI agent (opencode, driven by ralph) receives a task and must carry it from
proposal through archive using givn's workflow, after which automated checks and
an LLM judge score the result.

The harness lives in the `harness/` directory of the givn repository. It is a
**separate Python project** with its own dependency lifecycle. The givn binary
source resides in the parent directory; each run creates a fresh workspace and
initializes it with the current binary.

## Directory layout

```
givn/                             # givn Rust binary (parent project)
  src/
  tests/
  assets/

harness/                          # test harness (this project)
  src/givn_harness/               # Python package
  tests/features/harness/         # behave specs and step definitions
  tasks/                          # task prompt files
  givn/                           # this project's own givn workflow state
  docs/arc42/                     # architecture documentation
```

## Prerequisites

- **Python 3.12+** — `uv` manages the virtualenv and dependencies.
- **givn** — resolved as `GIVN_BINARY` or `givn` on `PATH`. Each workspace runs
  `givn init --agents <selected-agent>`, so the selected agent gets its own
  generated integration (opencode V2 plugin, omp hooks and skills, or maki
  plugin and skills).
- **ralph 1.2.2+** — the opencode task-loop wrapper. Default
  `/usr/local/bin/ralph`; override with `--ralph-bin`. ralph only supports
  opencode/claude-code/codex/copilot, so omp and maki use their native print
  modes through the harness's own one-step loop.
- **opencode 2.0+** — the judge always uses opencode, and the default agent is
  opencode; they need a provider login in opencode's auth store
  (`opencode auth login`). omp and maki use their own auth stores. The harness
  holds no API key of its own.
- **bubblewrap** — `bwrap` is auto-detected and used when available. Setting
  `GIVN_HARNESS_NO_SANDBOX=1` (or a missing `bwrap`) runs without the sandbox;
  the legacy `GIVN_HARNESS_NO_SANDLOCK` name still works. There is no sandlock
  dependency and no `--no-sandbox` flag.

## Tasks

- `tasks/trivial.md` — write one file; proves the agent/ralph transport without
  a givn change.
- `tasks/shopping-list.md` — a full product task: build a mobile-friendly
  shopping-list website through the givn workflow until archive. The design
  inputs are `tasks/shopping-list-design.png` and `tasks/shoppinglist_design.zip`.
- `tasks/smoke.md` — a minimal givn change that produces proposal, specs,
  design, design-review, tasks, and review artifacts without implementing or
  archiving. Use it to exercise the automated checks, the LLM judge, and
  full-record capture.

Every run creates a fresh workspace and initializes it with
`record.mode: full`, so the workspace's development record stores complete
tool input/output, assistant text, prompts, and other captured content. The
harness project's own `givn/config.yaml` also sets full capture.

## Quick start

```sh
cd harness
uv sync

GIVN_BINARY=/home/buster/projects/givn/target/release/givn \
  uv run givn-harness run \
    --task tasks/shopping-list.md \
    --output reports/ \
    --max-iterations 40
```

Every agent runs direct DeepSeek V4 Flash. opencode (agent or judge) is pinned
to **`deepseek/deepseek-flash#max`** (reasoning effort `max`); omp is pinned to
`deepseek/deepseek-flash` with `--thinking max`; maki is pinned to
`deepseek/deepseek-flash`. There is no rotation and no fallback, and `--model`
accepts only the selected agent's exact value. A real run uses a real model:
tokens and time, and the result is nondeterministic.

## Usage

```
uv run givn-harness run [OPTIONS]

Options:
  --agent [opencode|omp|maki]
                          Agent under test; default opencode
  --driver [ralph|native]
                          ralph (opencode only) or the harness's native one-step
                          loop; use native for comparisons
  --profile [full|smoke]
                          Task acceptance profile; smoke checks the planning
                          artifacts, full keeps the archive gates
  --task PATH             Task prompt file (required for a new run)
  --output DIR            Report output directory (default: reports/)
  --workspace-root DIR    Temp workspace root (default: /tmp/givn-harness-workspaces)
  --workspace PATH        Resume an existing workspace
  --continue              Resume the last run from .givn-harness-run.json
  --ralph-bin PATH        Path to ralph (default: /usr/local/bin/ralph)
  --fail                  Simulate an agent failure (check wiring)
  --timeout INTEGER       Agent execution timeout in seconds
  --max-iterations INTEGER  Maximum one-step iterations (native default 10)
  --no-agent-output       Suppress streaming agent output
  --model MODEL           Pinned model for the selected agent; other values are refused
  --prompt-template PATH  Custom one-step prompt template
  --help                  Show this message and exit.
```

If the agent writes `BLOCKED.md` with a question, answer it (add
`## ANSWERED` at the top) and rerun with `--continue`.

## Provider and model

The harness has **no API-key setting**. The judge and the opencode agent use
opencode's credential store:

```sh
opencode auth list      # e.g. DeepSeek and OpenRouter stored
opencode models         # models the configured providers expose
```

omp and maki use their own credential stores and their own model commands. The
only allowed model is direct DeepSeek V4 Flash: `deepseek/deepseek-flash#max`
for opencode, `deepseek/deepseek-flash` with `--thinking max` for omp, and
`deepseek/deepseek-flash` for maki. The harness refuses
`deepseek/deepseek-v4-pro`, the `openrouter/*` variants, `litellm/*`, and every
other model. To switch the DeepSeek key, run `opencode auth login` for DeepSeek
and re-check `opencode models`; for omp/maki use their own auth command. Do not
put a key into the harness or into a task file.

## Sandbox

When `bwrap` is available, the selected agent runs inside bubblewrap: the
workspace is read-write, `/usr`, `/etc`, the opencode install and config,
`~/.cargo`, `~/.local` (including the opencode and maki stores), `~/.omp`,
`~/.config/maki` and the package caches are mounted. `/tmp` is a private
tmpfs containing only this run's workspace; previous harness runs and unrelated
`/tmp` fixtures are not visible. The network is shared. If the sandbox is unavailable, the harness
prints a warning and runs the agent directly. `GIVN_HARNESS_NO_SANDBOX=1`
forces the direct mode; `GIVN_HARNESS_NO_SANDLOCK=1` remains accepted for
compatibility.

## Output

Each run creates a directory under `--output`:

```
reports/
├── <run-id>/
│   └── report.md          # checks, judge scores, agent log, workspace path
└── index.json             # cross-run summary: one entry per run
```

The report and `index.json` now carry the comparable manifest: profile, driver,
model/thinking, judge, agent/Givn versions, harness commit, task and prompt
hashes, iterations, wall time, termination, capture coverage, process counts,
and the generated development report. The judge uses the same
`deepseek/deepseek-flash#max` for every agent.

The smoke profile reports these task-specific checks:

- artifact existence
- `DESIGN-REVIEW: PASS`
- `REVIEW: PASS`
- addons enabled
- exactly one unchecked task and no implementation (the smoke task's contract)
- no production code outside the Givn artifacts
- full capture: at least 90% of tool+turn events carry content and at least 50%
  of turns carry assistant or thinking text (the report records recorded and
  tool-level subagent starts separately)

It deliberately excludes archive success, tasks-all-done, commit checks, and
`verify.command`, because the smoke task forbids implementation and archiving.
The task contract in the agent prompt is authoritative: for the smoke it
forbids production code and archive, requires exactly one unchecked task, and
allows at most one design-review grilling subagent.

## Comparing agents: opencode, omp, maki

Use the campaign command. It runs every agent through the **same native
one-step loop**, the same prompt template, the same task/profile checks, the same
judge, the same DeepSeek Flash settings, and the same budget:

```sh
cd harness
uv sync
GIVN_BINARY=/home/buster/projects/givn/target/release/givn \
  uv run givn-harness compare \
    --task tasks/smoke.md \
    --output /tmp/givn-harness-campaign \
    --agents opencode,omp,maki \
    --repeats 3 \
    --driver native \
    --profile smoke \
    --max-iterations 10 \
    --timeout 1200 \
    --no-agent-output
```

`/tmp/givn-harness-campaign/campaign.md` holds the per-run and aggregate table;
`campaign.json` holds the same rows. Each run directory has `report.md`, the
Givn development report, and `index.json`. The report's termination is the
task oracle: `completed` when the profile's artifacts are present,
`max_iterations` or `timeout` otherwise; the agent's own promise is not the
stop signal.

The reporter fix is in the Givn workspace integration: opencode uses a private
server (`--standalone`) and the event subscription starts before the mode query.
Turn text, thinking, tool names, and tool output reach the record, and opencode
and maki `subagent`/`task` tool calls now produce real subagent spans. The
canonical `kind=subagent` query is still only half the story for older records;
count both forms:

```sh
jq -s 'map(select((.kind=="subagent") or
                 (.kind=="tool" and (.name=="subagent" or .name=="task"))))
       | group_by(.session)
       | map({session:.[0].session, starts:length, forms:(map(.kind)|unique)})' \
  givn/history/events.jsonl
```

Maki's first-party assistant/thinking text is captured through
`maki.session.messages`; its tool output remains best-effort because the maki
`ToolDone` event does not carry it. Maki exposes no reasoning-effort flag; the
manifest records `provider default (no max flag exposed)`. opencode and omp run
at max.

## Smoke run: report + judge, full capture

```sh
cd harness
uv sync
GIVN_BINARY=/home/buster/projects/givn/target/release/givn \
  uv run givn-harness run \
    --agent opencode \
    --driver native \
    --profile smoke \
    --task tasks/smoke.md \
    --output /tmp/givn-harness-smoke \
    --max-iterations 10 \
    --timeout 1200
```

The run writes `<output>/<run-id>/report.md` with the automated checks, the five
LLM-judge scores, and the run manifest. It also writes the workspace's
`development-report.html` next to it. The workspace path is printed.

## Development

This project uses givn for its own workflow; its `givn/` state is the harness's
development workflow and is **not** used for the generated test workspaces (each
run initializes a fresh one with the current binary).

```sh
uv sync

# Project-owned test scripts
./run-tests.sh
./run-tests.sh --e2e        # requires a real LLM, ralph, opencode, network

# Coverage
./measure-coverage.sh
./merge-coverages.sh

# Single scenario
uv run behave tests/features/ --no-capture -n "<scenario title>"

# Step-body audit
bash scripts/check-step-bodies.sh
```

## License

TBD
