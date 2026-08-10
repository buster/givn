# Design: init-agent-target-selection

## Overview

`givn init` and `givn skills sync` currently write workflow skills and
commands to every agent directory unconditionally (`.agents/`, `.claude/`,
`.opencode/`). This change makes the set of agent targets configurable:

1. When `givn init` creates a fresh `givn/config.yaml`, it asks the user
   which agents to install to: a multi-select prompt (opencode, .agents,
   .claude) with opencode as the default. The answer is persisted in
   `givn/config.yaml` under a new `skills:` block.
2. `givn skills sync` writes skills/commands only to the persisted targets.
   Missing/absent config falls back to `[opencode]`. Sync is **write-only**:
   generated files under targets that are no longer selected are deliberately
   left in place (their removal is a manual/user decision — see ADR-036 and
   R-12; an upgrade that narrows the selection must not delete a project's
   existing skill files).
3. The skill-addon disable path keeps removing generated files from all
   three targets (cleanup semantics — uninstall is not gated by the target
   selection; leftover files from a previously wider selection must go).

Non-interactive `givn init` (piped/closed stdin) never blocks: it reads an
answer line if one is available, and falls back to the opencode default on
EOF. Re-running `givn init` on an existing project skips the prompt entirely
and keeps the persisted selection; `givn init --force` replaces the config
and therefore re-prompts (EOF → default opencode).

## Technology Decisions

No new dependencies. The prompt is a plain numbered list read line-by-line
from stdin (std only); the project already favours std-only line-based YAML
editing (`set_addon` in `src/cli/addons.rs`), and a checkbox TUI (e.g.
dialoguer) would add a dependency + a termios-based test seam for no
observable benefit.

Prompt semantics (single pass, no re-prompt loop): unknown names are dropped
with a warning that lists the valid agents; an answer with zero known agents
(or an empty/EOF input) falls back to the default `[opencode]`.

| Technology | Version | Resolution |
|---|---|---|
| Rust std (`std::io` line reading) | latest toolchain in use | resolved at setup time via existing rust-toolchain/Cargo.lock |
| No new crates | — | decided here — std-only prompt and config editing |

## Step Definitions

| Capability | Step definition file |
|---|---|
| init-agent-target-selection | `tests/steps/init_agent_target_selection_steps.rs` |
| install-extras (modified scenarios) | `tests/steps/init_agent_target_selection_steps.rs` |

New steps in `init_agent_target_selection_steps.rs`:

- `#[when] the user runs `givn init` and answers the agent prompt with "<answer>"` — spawns the binary with piped stdin, writes `<answer>\n`, closes stdin.
- `#[when] the consumer runs `givn init` and answers the agent prompt with "<answer>"` — same handler (e2e phrasing).
- `#[when] the consumer invokes `givn init` without answering a prompt` — same handler as `the user runs` (stdin null → default).
- `#[given] / #[then] givn skills targets in givn/config.yaml equal "<a, b, c>"` — given: writes the `skills:` block into config; then: asserts the parsed targets match.
- `#[given] a givn/config.yaml with skills targets "<a, b, c>"` — scaffolds `givn/` and writes a minimal `givn/config.yaml` containing the `skills:` block (clean state for the sync-exclusion scenario, without running init).
- `#[when] the user runs `givn init`` (existing, in `mod.rs`) — stdin is null → the default path (opencode) is exercised.

All other steps used by the new scenarios (file/dir existence, `stdout
mentions`, exit codes) already exist in `tests/steps/mod.rs`.

Test runner command — during this change, the delta specs live under the
change's `specs/` dir; the runner must be pointed at them via
`GIVN_FEATURES` (matching what `givn check` sets):

```
GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner
```

Single-scenario run command (exact invocation to run ONE named scenario,
against the change's delta specs — `--name` with zero matches exits 0, so
the change dir MUST be in GIVN_FEATURES for RED to be visible):

```
GIVN_FEATURES=givn/changes/init-agent-target-selection/specs/ cargo test --test features_runner -- --name "<scenario title regex>"
```

E2E runner command (from `verify.e2e_command`):
```
cargo test --test features_runner -- --tags @e2e
```

## Internal Primitives

| Primitive | Type | Purpose |
|---|---|---|
| `AgentTarget` | `enum { Opencode, Agents, Claude }` | Canonical agent target; `name()` → `opencode`/`agents`/`claude` (config values), `dir()` → `.opencode`/`.agents`/`.claude`, `display()` → `opencode`/`.agents`/`.claude` (prompt options) |
| `Skills { targets: Vec<String> }` | config struct | Parsed from the `skills:` block in `givn/config.yaml` via `OverlayDoc` (added as `Option<Skills>`, lenient overlay — no `deny_unknown_fields`) |
| `load_skills_targets()` | `fn` | Reads `givn/config.yaml`, normalizes names (accepts dotted and bare forms), validates against the known set; returns `Vec<AgentTarget>` defaulting to `[Opencode]` |
| `prompt_for_targets()` | `fn` | Reads one stdin line, splits on commas, normalizes; prints the numbered prompt on TTY; drops unknown names with a warning listing valid agents; empty answer / zero known agents / EOF → default `[opencode]`; returns the target set |
| `write_skills_block(targets)` | `fn` | Appends/updates the `skills:` block in `givn/config.yaml` (line-based, preserves comments, same strategy as `set_addon`) |

## Strict Mode

The cucumber-rs runner already fails on undefined/pending steps via
`.fail_on_skipped()` on the `Cucumber` builder in
`tests/features_runner.rs:20`. Not-implemented stubs for new steps use
`todo!()` (Rust) until the real body is written — never an empty body.

## Local Runnability & Digital Twins

**Local run command** — the system under test is a CLI binary; nothing to
start:
```
cargo build --release && ./target/release/givn init
```

**Isolated network:** not applicable — the binary has no network
dependencies.

**Digital twins:** none — no external/third-party dependencies.

**Anticipated interface obstacles and their fix:**
```
The agent prompt reads from stdin. In the cucumber harness, every `givn`
invocation goes through `Command::output()`, which gives the child null
stdin (immediate EOF). The prompt implementation must therefore treat EOF
as "use the default selection", not as an error — this is both the
non-interactive CI behaviour and the deterministic test seam. New steps
that need a non-default answer spawn the binary with piped stdin and write
the answer before closing the pipe. No PTY is required because the prompt
reads plain lines, not raw keypresses.
```

## E2E Smoke Test Infrastructure

**Interface type:**
```
CLI — subprocess invocation of the compiled `givn` binary is the real
interface; a subprocess with piped stdin is exactly how a consumer
interacts with `givn init`.
```

**E2E runner command** (→ `verify.e2e_command`):
```
cargo test --test features_runner -- --tags @e2e
```

**E2E step location:** `tests/steps/init_agent_target_selection_steps.rs`
(e2e steps share the same file — they use the same subprocess harness as
the unit scenarios, matching the established pattern in
`tests/steps/validate_change_id_input_steps.rs`).

**Local test infrastructure:** none beyond the existing cucumber harness —
each scenario gets a fresh `tempfile::TempDir` as the project root; the
`givn` binary is invoked via `env!("CARGO_BIN_EXE_givn")`; interactive
answers are piped through stdin.

**E2E framework choice and justification:** the existing cucumber-rs
harness (`tests/features_runner.rs`). No browser/HTTP layer exists for a
CLI tool; the subprocess invocation is the real interface, and `--tags @e2e`
selects the smoke scenarios. This matches how every other givn capability
runs its e2e coverage.

**Interaction Coverage Matrix** — one row per User Interaction Inventory
entry:

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| run `givn init` interactively and answer the agent-selection prompt | Consumer selects agent targets during interactive init | CLI | subprocess with piped stdin — write `opencode, .claude\n`, then assert installed files and absence of unselected target |
| run `givn init` non-interactively (no prompt; falls back to opencode) | Consumer runs init non-interactively and gets opencode skills | CLI | subprocess with null stdin (`Command::output()`), assert opencode files exist and .claude does not |
| run `givn skills sync` after the selection has been persisted | Consumer re-syncs skills after init and the selection is honoured | CLI | subprocess `givn skills sync` against a project whose config carries `skills.targets: [opencode, claude]`, assert both targets got files |

**Real-interface assertion rule:** every `@e2e` scenario's primary
assertions are on what the CLI produced (exit code + the filesystem state
the CLI wrote + stdout), i.e. the observable results of the real
invocation. No internal state is inspected.

**Real-interface driving rule:** the driving mechanism for a CLI capability
is the subprocess invocation with real stdin — no in-process calls, no
mocking.

**E2E scope:** one `@e2e` scenario per inventory entry, as tabled above.

## Architecture Impact

- `src/cli/init.rs` — after writing a fresh `givn/config.yaml` (created or
  `--force`-replaced), prompt for agent targets, write the `skills:` block,
  and report the selection in the summary. Re-init with an existing config
  skips the prompt.
- `src/skills/targets.rs` — `sync()` loads configured targets and writes
  skills only to them; commands only when `opencode` is selected. The
  `write_skill_to_targets` helper becomes target-driven.
- `src/config/overlay.rs` — add `Skills` struct + `skills: Option<Skills>`
  to `OverlayDoc`; add `load_skills_targets()` accessor (default
  `[opencode]`).
- `src/config/manifest.rs` — unchanged (skills is not a manifest field; it
  is read via the lenient overlay).
- `src/cli/addons.rs` — `run_disable` for skill addons keeps removing from
  all three targets (unchanged cleanup semantics).
- `src/cli/skills.rs` — summary output reflects the configured targets.
- `assets/config.yaml.tmpl` — document the `skills:` block in the scaffold.
- Tests: new `tests/steps/init_agent_target_selection_steps.rs`; the shared
  step `the generated review guidance contains` in
  `tests/steps/arc42_review_alignment_steps.rs` narrows from "all three
  targets" to "the configured targets".
- Audit: permanent specs reference agent directories only in
  `install-extras.feature` (all handled by this change's delta) and the
  arc42 step above (narrowed). No other permanent scenario breaks under the
  opencode-only default.
- Self-hosting: this repo's own `givn/config.yaml` gains
  `skills: { targets: [opencode, agents, claude] }` so its three existing
  agent directories keep being synced.

## Data Model Changes

`givn/config.yaml` gains an optional block:

```yaml
skills:
  targets: [opencode, agents, claude]
```

- Values are canonical target names without leading dots: `opencode`,
  `agents`, `claude`.
- Absent block → `[opencode]`.
- The block is read only by `load_skills_targets()`; it is ignored by the
  manifest overlay (lenient `OverlayDoc`), so existing tooling is
  unaffected.

## Open Technical Questions

None — resolved above.
