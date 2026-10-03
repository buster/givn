# Design: require-agent-target

## Overview

`givn init` currently resolves an empty agent-target selection to OpenCode on
every input path: the interactive confirmation falls back to the pre-marked
`.opencode` entry, a piped answer that contains no known name falls back, and
EOF without `--agents` falls back. This change makes at least one selected
agent target a precondition of initialization:

1. Every input path — interactive confirmation, piped answer line, and
   `--agents` — resolves to a target set; an empty set is an error, not a
   default.
2. The error is raised **before the first write**, so a refused
   initialization leaves the project byte-identical (the use case's minimal
   guarantee). Today the prompt runs after scaffolding and after
   `givn/config.yaml` is written.
3. The interactive prompt starts with no entry marked; the pre-marked
   `.opencode` default and the `DEFAULT_TARGETS` fallback constant are
   removed.
4. `givn skills sync` keeps its `[opencode]` fallback for a config without a
   `skills:` block: existing projects upgraded from older Givn versions stay
   runnable. Only the initialization precondition changes.

A refused run prints one error naming the valid targets and exits non-zero
(`main.rs` already maps `Err` to `eprintln!("error: {e:#}")` plus exit 1).

## Technology Decisions

- No new dependencies. The prompt stays `dialoguer::MultiSelect` (already in
  the dependency tree); the only change is that it no longer receives
  `defaults`.
- The refusal is an `anyhow` error with a single stable sentence:

  ```text
  no agent target selected; givn init needs at least one agent target (.opencode, .agents, .claude) — select one in the prompt or pass --agents <opencode|agents|claude|all>
  ```

  The delta scenarios assert on the stable fragment `no agent target
  selected` and on the target list (`.agents`).
- Resolution order in `run()`: compute `config_created` from the filesystem,
  then resolve targets (prompt or flag) when a fresh config would be written,
  and only then scaffold and write. The existing idempotent re-run path
  (config exists, no `--force`) is untouched and never prompts.
- `--force` runs the same resolution before overwriting the config, so
  `givn init --force` without a selection refuses and leaves the persisted
  selection intact.

| Technology | Version | Resolution |
|---|---|---|
| Rust | toolchain pinned by `rust-toolchain.toml` / `Cargo.lock` | resolved at build time by the existing lockfile |
| `dialoguer` | already a dependency, version pinned in `Cargo.lock` | unchanged — no upgrade, no new crate |

## Internal Primitives

| Primitive | Type | Change |
|---|---|---|
| `DEFAULT_TARGETS` | `const` | deleted — no fallback target set exists any more |
| `prompt_for_targets()` | `fn() -> Result<Vec<AgentTarget>>` | returns `Err` on an empty interactive selection, EOF, or an answer that resolves to zero targets; no `defaults` on the dialog |
| `resolve_agents_flag()` | `fn(&[String]) -> Result<Vec<AgentTarget>>` | returns `Err` when the flag resolves to zero known targets |
| `resolve_target_names()` | `fn(...) -> (Vec<AgentTarget>, Vec<String>)` | unchanged: parses, dedupes, reports unknown names; the callers now decide that an empty result is a refusal |
| `require_agent_target()` | `fn(Vec<AgentTarget>) -> Result<Vec<AgentTarget>>` | new small helper: empty → the refusal error, otherwise the set |
| `run()` | `fn(bool, bool, &[String]) -> Result<()>` | resolves targets before the first write; removes the post-scaffold prompt block |

## Step Definitions

One file per capability is already respected; this change adds one step text
(the PTY confirmation step), and every other step it needs exists:

| Capability | Step definition file |
|---|---|
| initialize-a-project | `tests/steps/init_agent_target_selection_steps.rs` |
| declare-the-product-start | `tests/steps/mod.rs` |
| measure-coverage-per-project | `tests/steps/mod.rs` |
| rules-policy | `tests/steps/mod.rs` |
| enable-optional-addons | `tests/steps/init_agent_target_selection_steps.rs` |
| readme-enforcement | `tests/steps/mod.rs` |

`tests/steps/mod.rs` carries the generic steps (`the user runs`, exit codes,
`stderr contains`, file/directory existence); the init-specific scenarios bind
their capability steps in `tests/steps/init_agent_target_selection_steps.rs`.

Reused steps (kept as-is): `the user runs \`givn init\``,
`the user runs \`givn init\` and answers the agent prompt with "<answer>"`,
`the consumer runs \`...\``, `the command exits non-zero`,
`stderr contains "..."`, `the directory "givn" does not exist`,
`givn skills targets in givn/config.yaml equal "..."`.

New step: `the user runs \`givn init\` in a terminal and confirms without
marking a target` in `tests/steps/init_agent_target_selection_steps.rs` — it
spawns `script -qec` with a piped newline (the PTY seam above), waits with a
60-second deadline, and stores the transcript as the command output for the
existing exit-code and stderr assertions.

Deleted step: `consumer_init_silent` (step text `the consumer invokes \`givn
init\` without answering a prompt`) in
`tests/steps/init_agent_target_selection_steps.rs` — the delta replaces its
only scenario (`givn/specs/initialize-project/initialize-a-project.feature:150`)
with the new non-interactive form that names the selection. The deletion lands
in the same change as the scenario replacement and must not precede the merge,
because the permanent corpus still references the step until then; the
change-scoped runs (`GIVN_FEATURES=givn/changes/require-agent-target/specs/`)
never load the unmerged permanent file.

Test-side fixture updates (the fixtures initialize a project, so they now
state the selection): `tests/common/mod.rs` (`make_init_root_without_git`),
`tests/steps/mod.rs` (no-addons fixture), `tests/steps/archive_proof_steps.rs`,
`tests/steps/local_only_rules_steps.rs`,
`tests/steps/release_0_3_0_e2e_steps.rs`,
`tests/steps/repository_ownership_fixtures_support.rs`,
`tests/steps/run_execution_e2e_steps.rs`,
`tests/steps/scm_backend_selection_steps.rs` — each appends
`--agents opencode`.

## Test Runner

Unit/integration (`verify.command`): `./run-tests.sh` — the cucumber-rs
runner `tests/features_runner.rs`, which loads `GIVN_FEATURES` (default
`givn/specs`) and is the sole scenario identity source.

Delta scope during this change:

```sh
GIVN_FEATURES=givn/changes/require-agent-target/specs/ ./run-tests.sh
```

Single-scenario run command (used for every RED/GREEN check in `tasks.md`):

```sh
GIVN_FEATURES=givn/changes/require-agent-target/specs/ cargo test --locked --test features_runner -- --name "<scenario title regex>"
```

E2E runner (`verify.e2e_command`): `./run-tests.sh --e2e` — the same runner
with the `@e2e` tag expression.

## Strict Mode

`cucumber-rs`: `.fail_on_skipped()` is set on the `Cucumber` builder in
`tests/features_runner.rs:55` and `:129`. Scenarios tagged `@wip` are excluded
from the normal tag scope, so they cannot silently pass; a RED check runs the
named scenario directly (name filter, no tag filter) and must fail. No new
step bodies are added by this change, so the `todo!()` stub discipline does
not apply; every touched scenario binds to an existing, already-asserting
step.

## Local Runnability & Digital Twins

**Local run command** — the system under test is the CLI binary; nothing else
to start:

```sh
cargo build --release && ./target/release/givn init --agents opencode
```

The interactive prompt is exercised from a real terminal (`./target/release/givn init`).

**Isolated network:** not applicable — the binary has no network dependency.

**Digital twins:** none — no external or third-party service is involved.

**Anticipated interface obstacles and their fix:** the interactive branch
needs a controlling terminal (`dialoguer` refuses to run without one). The fix
is a real PTY, not a weaker assertion: the step
`the user runs \`givn init\` in a terminal and confirms without marking a
target` spawns `script -qec "<givn> init" <transcript>` (util-linux) with a
piped `\n`, so the child sees a terminal on stdin, `dialoguer` renders its
checkbox list, and the bare confirmation is delivered as a keypress. The
transcript becomes the command output the existing stderr assertions read.
The scenario then proves the reported case end to end: nothing marked,
confirm → non-zero exit, `no agent target selected`, no `givn/` directory.
The step waits with a 60-second deadline and kills a hung dialog rather than
letting the suite block. `script` (util-linux) joins `git`, `fossil`, and
`git-filter-repo` as an acceptance-suite prerequisite, documented in
`AGENTS.md`; Ubuntu 24.04 (the CI image) ships it in the base system.

## E2E Smoke Test Infrastructure

**Interface type:** CLI — a real subprocess invocation of the compiled `givn`
binary, with real stdin.

**E2E runner command** (→ `verify.e2e_command`): `./run-tests.sh --e2e`.

**E2E step definition location:** `tests/steps/init_agent_target_selection_steps.rs`
plus the generic subprocess steps in `tests/steps/mod.rs` (established pattern
for this capability).

**Local test infrastructure:** the existing cucumber harness only; each
scenario gets a fresh `tempfile::TempDir` as the project root and spawns the
`givn` binary (`env!("CARGO_BIN_EXE_givn")`). No browser, server, database, or
container.

**E2E framework choice and justification:** cucumber-rs subprocess harness —
the CLI subprocess is the real interface for this capability, and it is the
project's single Gherkin runner (executable-spec invariant).

**E2E strict-mode proof:** `.fail_on_skipped()` (above) fails undefined or
skipped scenarios; the delta's e2e scenario is added without `@wip` only when
it passes.

### Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| select agent targets | Consumer selects agent targets during interactive init | CLI | subprocess with piped stdin — write `opencode, .claude\n`, assert installed targets |
| initialize non-interactively | Consumer runs init non-interactively and gets opencode skills | CLI | subprocess `givn init --agents opencode` with closed stdin, assert `.opencode` files exist and `.claude` does not |
| resynchronize selected targets | Consumer re-syncs skills after init and the selection is honoured | CLI | subprocess `givn skills sync`, assert both selected targets |
| initialize a Fossil project | Consumer initializes a Fossil checkout and receives backend-aware guidance | CLI | subprocess `givn init` inside a Fossil checkout (step spawns with `--agents opencode,claude`) |
| initialize without SCM | Consumer initializes a plain directory and receives a named warning | CLI | subprocess `givn init` in a plain directory, assert the named warning |

The refusal scenarios are regular (non-e2e) scenarios: they are error cases of
the same `initialize` action, not additional consumer actions.

Coverage boundaries:

- The explicit empty `--agents ""` value (clap yields one empty string) travels
  the same `resolve_target_names` → `require_agent_target` path as the covered
  `--agents nope` case (`src/skills/targets.rs` drops empty parts); it gets no
  separate scenario.
- Two delta scenarios are already GREEN before the change and are regression
  guards, not RED evidence: `givn init --force re-applies the explicitly given
  selection` (the current code honours a given flag) and the modified e2e
  scenario `Consumer runs init non-interactively and gets opencode skills`
  (`--agents opencode` already works). `tasks.md` records this so the RED/GREEN
  loop does not claim a RED proof for them.

## Decisions

- Decides D1: an empty selection refuses initialization — non-zero exit, one
  error sentence, no writes — instead of keeping the prompt open. Realized in
  `require_agent_target()`/`no_agent_target_error()` in `src/cli/init.rs` and
  in the delta scenarios (`... refuses initialization`).
- Decides D2: the interactive prompt starts with no entry marked. Realized by
  dropping `MultiSelect::defaults(...)` and the `DEFAULT_TARGETS` constant in
  `src/cli/init.rs`.

## Use-Case Traceability

| Contract element (`givn/specs/initialize-project/usecase.md`) | Technical decision | E2E evidence |
|---|---|---|
| Main flow 1 — the developer selects at least one agent target | targets resolved before the first write; refusal on an empty set | Consumer selects agent targets during interactive init; Consumer runs init non-interactively and gets opencode skills |
| Extension 1 (empty or unknown-only set refuses with the valid targets, writes nothing) | `require_agent_target()` error path before scaffolding | regular delta scenarios `... refuses initialization` |
| Minimal guarantee — the project remains unchanged when a request is rejected | resolution precedes directory scaffolding, config write, and skills write | refusal scenarios assert the `givn` directory does not exist / the persisted selection is unchanged |
| Success guarantee — initialized with the chosen agent integrations | unchanged resolution and sync path | unchanged e2e rows |
| Persona `developer--primary-user` (CLI text surface) | the refusal names the valid targets and the `--agents` escape hatch in the consumer's vocabulary | stderr assertions in the refusal scenarios |

## Architecture Impact

- `src/cli/init.rs` — resolution reordered before writes; dialog default
  removed; `DEFAULT_TARGETS` deleted; refusal error added. No other module
  changes: `src/skills/targets.rs`, `src/config/overlay.rs`, `src/cli/skills.rs`
  keep their behaviour (including the sync fallback).
- `assets/config.yaml.tmpl` — the scaffolded `skills:` comment stops calling
  OpenCode the init default; it states the sync fallback for configs without a
  block.
- Durable docs — `docs/arc42/08-crosscutting-concepts.md` §8.7 (the target
  selection rule) and the accepted record
  `docs/arc42/adr/ADR-013-agent-integration.md` (refinement of an existing
  decision: `AMEND_ADR`, not a new record; the qualification gate routes
  refinements to an amendment). The registers
  (`docs/arc42/09-architecture-decisions.md`, `docs/arc42/adr/README.md`)
  record the amendment.
- Delta specs — one file per affected capability under
  `givn/changes/require-agent-target/specs/`:
  `initialize-project/initialize-a-project.feature`,
  `initialize-project/enable-optional-addons.feature`,
  `operate-product/declare-the-product-start.feature`,
  `fragments/measure-coverage-per-project.feature`,
  `fragments/rules-policy.feature`,
  `keep-documentation-current/readme-enforcement.feature`, and
  `initialize-project/usecase.md`.
  The scenarios in the five non-init capabilities keep their assertions and
  only gain `--agents opencode` in the `When` line, because their `givn init`
  fixture action now names the selection.
- Tests — the fixture files listed under Step Definitions.

## Data Model Changes

None. `givn/config.yaml` keeps its `skills.targets` block and the
`[opencode]` fallback for an absent block; only the initialization
precondition changes.

## Visual Design Contract

Not present: user-visible surface is CLI text only (the refusal sentence), no
rendered screen, so the visual review gate stays inactive and no screenshot or
transcript is committed — the same precedent as the `change-release-note` and
`maintain-project` changes.

## Open Technical Questions

None — D1 and D2 are decided above.
