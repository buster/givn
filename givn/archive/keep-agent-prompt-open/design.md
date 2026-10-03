# Design: keep-agent-prompt-open

## Overview

The archived `require-agent-target` change refused any empty target set before
the first write, including an unmarked interactive confirmation, and removed
the pre-marked `.opencode` entry. This change restores a usable dialog while
keeping the scripted refusal:

1. The interactive list opens with `.opencode` marked again
   (`MultiSelect::defaults`).
2. An empty confirmation does not end initialization: it prints
   `error: no agent target selected; mark at least one agent target
   (.opencode, .agents, .claude) and confirm` and shows the list again. The
   loop exits only with a non-empty selection; Ctrl-C (SIGINT) aborts with a
   non-zero exit before any write. Esc is ignored by
   `MultiSelect::interact()` (its quit arm is guarded by `allow_quit=false`),
   so it is not advertised as a cancel key.
3. Every non-terminal path is unchanged: EOF without `--agents`, an empty
   answer line, and an unknown-only answer or flag value still refuse with the
   non-zero exit and the `no agent target selected; pass --agents …` error,
   before the first write.

The refusal for a bad *scripted* selection and the re-ask for the *interactive*
confirmation are deliberately different: a read loop for piped stdin could
block automation on EOF, while the terminal can always ask again.

## Technology Decisions

- `dialoguer::MultiSelect` stays; the TTY branch builds a fresh
  `MultiSelect` (with `.defaults(...)`) inside a `loop` — `interact(self)`
  consumes the builder and initialises `checked` from `defaults` at entry, so
  the rebuild is what makes the pre-marked entry reappear after a re-ask —
  and returns only a non-empty selection. No new dependency.
- The prompt default is a marked entry, not a fallback target set: the new
  constant is `DEFAULT_PROMPT_TARGET: AgentTarget = AgentTarget::Opencode`,
  used only to build the `defaults` vector. `require_agent_target` remains the
  guard for the non-terminal paths and is not applied to the interactive
  result (the loop guarantees it is non-empty).
- The re-ask message is written with `eprintln!` to stderr — the scripted
  refusal also writes there (`main.rs` maps the `anyhow` error to
  `eprintln!("error: {e:#}")`), so a transcript assertion sees one channel.

| Technology | Version | Resolution |
|---|---|---|
| Rust | toolchain pinned by `rust-toolchain.toml` / `Cargo.lock` | resolved at build time by the existing lockfile |
| `dialoguer` | pinned in `Cargo.lock` | unchanged — no upgrade, no new crate |

## Internal Primitives

| Primitive | Type | Change |
|---|---|---|
| `prompt_for_targets()` | `fn() -> Result<Vec<AgentTarget>>` | TTY branch: `defaults` with `.opencode` marked; `loop` around `MultiSelect` that re-asks on an empty selection and prints the requirement to stderr; piped branch unchanged (refusal) |
| `DEFAULT_PROMPT_TARGET` | `const AgentTarget` | new — the single pre-marked entry; not a fallback target set |
| `require_agent_target()` | unchanged | still guards EOF, empty answers, and flag values |

## Step Definitions

| Capability | Step definition file |
|---|---|
| initialize-a-project | `tests/steps/init_agent_target_selection_steps.rs` |

The PTY helper is generalized to take the key sequence; the existing PTY step
(`… confirms without marking a target`) is replaced by two steps:

- `the user runs `givn init` in a terminal and confirms the prompt` — keys
  `"\n"`; proves the pre-marked default path.
- `the user runs `givn init` in a terminal, unmarks every target, and confirms
  twice` — keys `"\x20\n\n"`; the space toggles the highlighted `.opencode`
  entry off, the first confirmation re-asks, the second accepts the re-marked
  default.

The key sequence relies on a stated invariant: `DEFAULT_PROMPT_TARGET` is
`AgentTarget::ALL[0]` and the only pre-marked entry, so a single space toggles
exactly the default off (`MultiSelect` highlights item 0 initially and Space
toggles the highlighted item). The step documents this invariant; if a second
default is ever added, the sequence must toggle each marked index.

Known harness limitation: the PTY step stores the transcript as both
`stdout` and `stderr`, so a channel assertion cannot distinguish the streams —
the message channel (`eprintln!`) is fixed here by design and covered
channel-real by the non-terminal scenarios, which assert on the real stderr.

Both steps keep the 60-second deadline.

## Test Runner

Unit/integration (`verify.command`): `./run-tests.sh`.
Delta scope during this change:

```sh
GIVN_FEATURES=givn/changes/keep-agent-prompt-open/specs/ ./run-tests.sh
```

Single-scenario run command:

```sh
GIVN_FEATURES=givn/changes/keep-agent-prompt-open/specs/ cargo test --locked --test features_runner -- --name "<scenario title regex>"
```

E2E runner (`verify.e2e_command`): `./run-tests.sh --e2e` (unchanged).

## Strict Mode

`cucumber-rs`: `.fail_on_skipped()` in `tests/features_runner.rs:55,129`.
`@wip` scenarios stay outside the normal tag scope; the RED check runs the
named scenario directly. No new step body starts empty.

## Local Runnability & Digital Twins

**Local run command:** `cargo build --release && ./target/release/givn init`
(interactive, needs a terminal); scripted form
`./target/release/givn init --agents opencode`.

**Isolated network:** not applicable — no network dependency.

**Digital twins:** none.

**Anticipated interface obstacles and their fix:** the re-ask needs a real
terminal to be observable; the PTY seam from the previous change
(`script -qec "<givn> init" <transcript>` with piped keys) already provides
it, now with a multi-key sequence. `script` (util-linux) stays a suite
prerequisite documented in `AGENTS.md`.

## E2E Smoke Test Infrastructure

**Interface type:** CLI subprocess; the interactive path is driven through the
`script` PTY.

**E2E runner command:** `./run-tests.sh --e2e`.

**E2E step definition location:** `tests/steps/init_agent_target_selection_steps.rs`
(PTY and init-specific steps) plus `tests/steps/mod.rs` (generic steps).

**Local test infrastructure:** cucumber-rs harness only; fresh `TempDir` per
scenario; no browser/server/database/container.

**E2E framework choice and justification:** cucumber-rs subprocess harness —
the real interface for a CLI.

**E2E strict-mode proof:** `.fail_on_skipped()` (above).

### Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| select agent targets | Consumer selects agent targets during interactive init | CLI | subprocess with piped stdin — write `opencode, .claude\n` |
| initialize non-interactively | Consumer runs init non-interactively and gets opencode skills | CLI | subprocess `givn init --agents opencode` |
| resynchronize selected targets | Consumer re-syncs skills after init and the selection is honoured | CLI | subprocess `givn skills sync` |
| initialize a Fossil project | Consumer initializes a Fossil checkout and receives backend-aware guidance | CLI | subprocess inside a Fossil checkout |
| initialize without SCM | Consumer initializes a plain directory and receives a named warning | CLI | subprocess in a plain directory |

The two new scenarios are regular (non-e2e): input variants of the
`initialize` action, not additional consumer actions. The inventory rows and
their `@e2e` scenarios are unchanged by this change.

## Decisions

- Decides D1: the re-ask applies to the interactive terminal only; piped
  answers and `--agents` keep the fast non-zero refusal. Realized in
  `src/cli/init.rs` (`prompt_for_targets` branch split) and in the delta
  scenarios.

## Use-Case Traceability

| Contract element (`specs/initialize-project/usecase.md`) | Technical decision | E2E evidence |
|---|---|---|
| Main flow 1 — the list opens with `.opencode` marked and re-asks | `defaults` + `loop` in the TTY branch | regular PTY scenarios (pre-marked default; re-ask) |
| Extension 1 (interactive empty confirmation re-asks) | error to stderr + repeat prompt | scenario `The interactive prompt keeps asking until a target is marked` |
| Extension 1 (non-interactive empty/unknown-only refuses) | `require_agent_target` on the piped and flag paths | unchanged scenarios from `require-agent-target` |
| Minimal guarantee — unchanged on rejection | resolution before the first write | unchanged refusal scenarios |

## Architecture Impact

- `src/cli/init.rs` — TTY branch of `prompt_for_targets` (fresh builder with
  `defaults` inside the loop + stderr message), the `DEFAULT_PROMPT_TARGET`
  constant, and the function doc comment that still describes the unmarked
  prompt.
- `tests/steps/init_agent_target_selection_steps.rs` — PTY helper takes a key
  sequence; two steps replace the removed one.
- Delta spec + usecase copy in `givn/changes/keep-agent-prompt-open/specs/`.
- Durable docs: `docs/arc42/06-runtime-view.md` §6.1, `docs/arc42/08-crosscutting-concepts.md`
  §8.7, the ADR-013 amendment paragraph and its register rows (the archived
  `require-agent-target` sentence said the prompt starts unmarked), the
  README Quickstart, and `assets/config.yaml.tmpl`.

## Data Model Changes

None.

## Visual Design Contract

Not present: CLI text only, no rendered screen; the visual review gate stays
inactive (same precedent as `require-agent-target`).

## Open Technical Questions

None — D1 is decided above.
