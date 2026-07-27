# givn-test-harness

Automated end-to-end test harness for the [givn](..) spec-driven workflow tool.
Treats givn's agent-facing assets — skill templates, artifact instructions, CLI
behaviour — as the system under test. Runs complete autoresearch loops:
an AI agent (opencode, driven by ralph) receives a task and must carry it from
proposal through archive using givn's workflow.

The harness lives in the `harness/` directory of the givn repository. It is a
**separate Python project** with its own dependency lifecycle and its own givn
workflow. The givn binary source resides in the parent directory.

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
  givn/                           # this project's givn workflow state
  docs/arc42/                     # architecture documentation
```

## Prerequisites

- **Python 3.12+** — `uv` manages the virtualenv and dependencies
- **uv** — Python package manager (installed on host)
- **givn** — the givn binary is resolved at runtime. Set `GIVN_BINARY` to
  the path of a debug/release build (defaults to `givn` on `PATH`).
- **ralph** — AI agent loop wrapper. Default path `/usr/local/bin/ralph`;
  override via `--ralph-bin` or `RALPH_BINARY` env var.
- **opencode** — LLM agent CLI. Expected on `PATH`.
- **sandlock** — Linux sandbox (learn-then-confine filesystem + default-allow
  network). Declared as a git dependency in `pyproject.toml`. Set
  `SANDLOCK_PATH` env var for local development overrides.
  Run without sandlock via `--no-sandbox`.

## Quick start

```sh
uv run givn-harness run --task tasks/shopping-list.md --output reports/ --profile shopping-list
```

## Usage

```
uv run givn-harness run [OPTIONS]

  Run an autoresearch loop against a givn task.

Options:
  --task PATH             Task prompt file (required)
  --output DIR            Report output directory (default: reports/)
  --workspace-root DIR    Temp workspace root (default: /tmp/givn-harness-workspaces/)
  --profile TEXT          Sandlock profile name for learn-then-confine
  --timeout INTEGER       Max agent execution time in seconds
  --max-iterations INTEGER  Max ralph iterations
  --ralph-bin PATH        Path to ralph binary (default: /usr/local/bin/ralph)
  --givn-bin PATH         Path to givn binary (default: givn on PATH)
  --no-sandbox            Run without sandlock (for debugging)
  --help                  Show this message and exit.
```

## Output

Each run creates a directory under `--output`:

```
reports/
├── <run-id>/
│   └── report.md          # Checks, judge scores, agent log, workspace path
└── index.json             # Cross-run summary: one entry per run
```

## Development

This project uses **givn** for its own workflow. Run `givn instructions`
from this directory to see the current lifecycle state.

```sh
# Install deps
uv sync

# Run behave tests (non-e2e)
uv run behave tests/features/ --no-capture

# Run behave tests (e2e only — requires real LLM + sandlock + ralph)
uv run behave tests/features/ --no-capture --tags=@e2e

# Single scenario
uv run behave tests/features/ --no-capture -n "<scenario title>"

# Step-body audit
bash scripts/check-step-bodies.sh
```

## License

TBD
