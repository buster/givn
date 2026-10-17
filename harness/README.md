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
  `givn init --agents opencode`, so the generated opencode integration is the
  installed binary's V2 plugin.
- **ralph 1.2.2+** — task loop wrapper. Default `/usr/local/bin/ralph`;
  override with `--ralph-bin`. Its opencode agent runs `opencode run -m <model>`.
- **opencode 2.0+** — the agent and the judge both call opencode; it needs a
  provider login in its own auth store (`opencode auth login`). The harness
  holds no API key of its own.
- **bubblewrap** — `bwrap` is auto-detected and used when available. Setting
  `GIVN_HARNESS_NO_SANDBOX=1` (or a missing `bwrap`) runs without the sandbox;
  the legacy `GIVN_HARNESS_NO_SANDLOCK` name still works. There is no sandlock
  dependency and no `--no-sandbox` flag.

## Quick start

```sh
cd harness
uv sync

GIVN_BINARY=/home/buster/projects/givn/target/release/givn \
  uv run givn-harness run \
    --task tasks/shopping-list.md \
    --output reports/ \
    --model deepseek/deepseek-flash \
    --max-iterations 40
```

`--model` is passed to ralph (the agent) and to the judge. A real run uses a
real model: tokens and time, and the result is nondeterministic.

## Usage

```
uv run givn-harness run [OPTIONS]

Options:
  --task PATH             Task prompt file (required for a new run)
  --output DIR            Report output directory (default: reports/)
  --workspace-root DIR    Temp workspace root (default: /tmp/givn-harness-workspaces)
  --workspace PATH        Resume an existing workspace
  --continue              Resume the last run from .givn-harness-run.json
  --ralph-bin PATH        Path to ralph (default: /usr/local/bin/ralph)
  --fail                  Simulate an agent failure (check wiring)
  --timeout INTEGER       Agent execution timeout in seconds
  --max-iterations INTEGER  Maximum ralph iterations
  --no-agent-output       Suppress streaming agent output
  --model MODEL           Model for the agent and the judge, e.g. deepseek/deepseek-flash
  --prompt-template PATH  Custom ralph prompt template
  --help                  Show this message and exit.
```

If the agent writes `BLOCKED.md` with a question, answer it (add
`## ANSWERED` at the top) and rerun with `--continue`.

## Provider and model

The harness has **no API-key setting**. Both the agent (through ralph) and the
judge (a direct `opencode run`) use opencode's own credential store:

```sh
opencode auth list      # e.g. DeepSeek and OpenRouter stored
opencode models         # models the configured providers expose
```

Direct DeepSeek models currently exposed here are `deepseek/deepseek-flash` and
`deepseek/deepseek-v4-pro`; `deepseek/deepseek-chat` is not available in this
configuration. Use `--model deepseek/deepseek-flash` (fast) or
`deepseek/deepseek-v4-pro` (stronger). To switch the DeepSeek key, run
`opencode auth login` for DeepSeek and re-check `opencode models`. Do not put a
key into the harness or into a task file.

## Sandbox

When `bwrap` is available, ralph runs inside bubblewrap: the workspace is
read-write, `/usr`, `/etc`, the opencode install and config, `~/.cargo`,
`~/.local` (including the opencode auth store), `/tmp`, and the package caches
are mounted; the network is shared. If the sandbox is unavailable, the harness
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

The report's judge scores come from five artifact evaluations (proposal, specs,
design, tasks, review) via opencode with the same model.

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
