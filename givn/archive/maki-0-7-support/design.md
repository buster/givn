# Design: maki-0-7-support

## Overview

The change makes the generated Maki reporter and the development-record intake
follow Maki 0.7.0 while remaining inert on older Maki versions:

1. The reporter reads the transcript's `tool_result` blocks, answers, moved-call
   receipts, session titles, subagent sessions, and task costs.
2. The reporter records one subagent span from two event sources, keeps paused
   tasks open, records stopped tasks as skipped, and records still-open tasks at
   session end as unknown with their observed elapsed time.
3. The intake accepts a reported session point entry, a per-event session
   override, a tool background marker, and a reported usage cost.
4. Project initialization and synchronization name the reporter's
   `session_read` grant and revoke the capabilities it never uses.
5. The acceptance harness pins Maki 0.7.0.

The verified host facts this design relies on:

- Maki 0.7.0's `maki.session.messages` returns `{ role, content = { blocks } }`
  with roles `user` and `assistant` only; a tool result is a
  `{ type = "tool_result", tool_use_id, content = <string>, is_error? }` block
  in a user message (verified from a real imported session, 2026-11-01).
- `maki.task.list()` exposes each task's `id`, `session`, `plugin`, `status`,
  `started_at`, `ended_at`, and `cost`; a tool-spawned task has no `plugin`.
- `TaskStatusChanged` reports `working`, `idle`, `done`, `error`, `cancelled`;
  `idle` belongs to plugin-owned tasks; `TaskStopped` is new in 0.7.0.
- A moved call emits one `ToolDone` at move time with the pre-move duration;
  `backgrounded` is not in the Lua payload; the transcript's `receipts` list
  names the moved call.
- A self-written project `plugin.toml` grants unnamed keys; `session_read`
  gates `maki.session.messages` and the content fields of events.

## Visual Design Contract

- Interface: web
- Viewports: mobile (360x780), desktop (1920x1080)
- Design system: docs/design/design-system.md

The development report is the rendered screen this change feeds. It adds no
layout, control, or style: session point entries let the existing lane-name
rule show a Maki session's title, and the existing details view shows the newly
recorded answer, background marker, and cost through its generic data renderer.
The existing `@e2e` Maki row of "Consumer installs the agent reporter and its
reported activity lands in the record" remains the interaction evidence and
keeps its committed screenshot pair; no delta `@e2e` scenario is added because
the interaction inventory row is unchanged.

## Use-Case Traceability

Owning use case: `inspect-development-history`, capabilities
`agent-activity-reporting` and `development-record`; supporting owner
`initialize-project` for generated grants.

| Use-case contract element | Technical decision | Capability / E2E evidence |
|---|---|---|
| Every supported harness reports through the same intake | Maki 0.7.0 event vocabulary and transcript reads feed the existing payload | `agent-activity-reporting`, all delta scenarios |
| A signal a harness cannot deliver is recorded as missing, never zero | Absent cost stays absent; unclosed tasks become unknown; backgrounded calls become unknown | `agent-activity-reporting`, cost/background/session-end scenarios |
| A recorded duration is measured, never inferred from disagreeing entries | One subagent entry carries the host duration where the tool reports it, else the reporter's own stamp | `agent-activity-reporting`, subagent span scenario |
| The record is append-only | Subagent tool entries are written once at the subagent close; nothing rewrites an entry | `agent-activity-reporting`, subagent lane scenario |
| Every entry belongs to exactly one scope and one session | Per-event session override; payload header remains the default | `development-record`, per-event session scenario |
| Recording never changes the observed work | Every new transcript or task read is wrapped; failure degrades to absent metadata | `agent-activity-reporting`, all scenarios; existing stalled-helper scenarios |
| Generated installation carries the reporter and its grants | Manifest names `session_read` and the revocations; sync adds only the grant | `initialize-project`, grant scenarios |

## Technology Decisions

| Technology | Version | Resolution |
|---|---|---|
| Rust | workspace toolchain | `Cargo.toml` / `Cargo.lock` |
| Maki (reporter host) | 0.7.0 | GitHub release `tontinton/maki` v0.7.0, checked 2026-10-31; x86_64 musl asset sha256 `3e0a1d1a4bc2bbaafdeb94f2a551bcc2d0f6ed81edfd9dd13272d9caa5add787` |
| cucumber-rs | existing workspace dependency | `tests/features_runner.rs`, `.fail_on_skipped()` |
| serde_json | existing workspace dependency | Intake payload parsing and record values |

No new dependency. The reporter stays a Lua module loaded by Maki's Lua host;
no new Maki permission beyond `session_read`, which it now names.

## Decisions

- Decides D1: the intake accepts `kind = "session"` point entries (a non-empty
  title is required), a per-event `session` override, `backgrounded = true` on
  tool events, and a numeric `cost` inside a reported usage. A non-numeric cost
  and a title-less session event are rejected per event, with the reason on
  stderr — realized in `src/cli/history.rs`.
- Decides D2: a backgrounded call is detected from the transcript's `receipts`
  at the turn boundary (`TurnEnd`), where the buffered tool event is mutated to
  `outcome = "unknown"` with `data.backgrounded = true`. `ToolDone` alone cannot
  identify a moved call: the receipt is written after the completion. A
  `task` call whose task is still listed at its completion has moved and keeps
  its subagent span open until the task's terminal status or the session's end.
- Decides D3: subagent cost is read from `maki.task.list()` at the span's close
  (and at a moved task's tool completion, while the task is still listed),
  recorded as a cost-only usage value, and absent when the host no longer lists
  the task. A start-time read is never recorded, because the cost is nil there.
- Decides D4: a subagent's tool calls are recorded from its own transcript under
  its own session at close, metadata always and content only in the full mode.
  The synthesis is skipped for a call id already buffered for that session, is
  capped at 100 calls and 256 KiB of result text per lane, and runs once per
  session — realized in the reporter; the intake accepts the per-event session.
- Decides D5: no version floor; the new handlers are inert on older hosts, and
  the manifest stays valid on 0.6.x — realized by handler registration and the
  unchanged manifest shape. Inertness was verified manually against 0.6.3 and
  0.7.0; the suite pins 0.7.0 only, so older-host drift is not suite-enforced.

### Span correlation

A task is addressed by its `tool_use_id` (Maki's own task model: "a task is a
subagent chat, addressed by its `tool_use_id`"), and a tool-spawned task's
`maki.task.list()` id is that tool call's id. The record's `subagent` identity
is the task id. The reporter closes one span per task under this rule:

1. A terminal `TaskStatusChanged` while the spawning tool call is still open
   (`state.tools[id]`) stores the terminal status and defers.
2. A `ToolDone` for a `task` whose task is still listed (working or idle) has
   moved to the background: it stores the title and host duration and defers.
3. A `ToolDone` for a task no longer listed closes the span with the stored
   status/title and the host duration; a terminal status for a task with no
   pending tool call closes with the task's own measured duration.
4. `TaskStopped` closes immediately as `skipped`; a later completion cannot
   reopen it.
5. `SessionEnd` closes every still-open task as `unknown` with the observed
   elapsed time.

A per-session `closed` set makes the rule idempotent, so no event order records
the same task twice.

## Architecture Impact

### Contracts and Components

| Contract | Consumer use case | Consumers | Implementing component today | Split trigger |
|---|---|---|---|---|
| Development-record intake | inspect-development-history | `givn history ingest` (agent reporters), `givn history harness-hook` (process-per-hook harnesses) | `src/cli/history.rs` | A second transport or a non-JSON record format appears |
| Maki activity reporting | inspect-development-history | Maki Lua host; two external mechanisms — the autocmd stream and the live transcript/task-list reads | `assets/agent-integrations/maki/givn-timeline.lua` | Maki's autocmd surface is replaced by an SDK, or the host exposes per-event durations, which removes the transcript adapter |
| Project installation | initialize-a-project | `givn init`, `givn skills sync` | `src/skills/targets.rs` | A second agent-target installer path appears |

Each capability keeps one implementing component. Maki loads exactly one plugin
file per project, so the transcript and task-list readers live inside that file
as named internal helpers; the contract is split only if a second host adapter
is added. The harness-hook path is unaffected by this change: its payload header
is the harness `session_id`, an event-level `session` wins where present, and
its events carry no `backgrounded`, cost, or session-title fields.

### Module changes

- `assets/agent-integrations/maki/givn-timeline.lua`: transcript block helpers,
  answer extraction, receipt marking, session title entry, task span state,
  task cost/session lookup, subagent tool synthesis.
- `assets/agent-integrations/maki/plugin.toml`: explicit grant and revocations.
- `src/skills/targets.rs`: four reporter grants; updated merge comment.
- `src/cli/history.rs`: `IngestedEvent.session`, `.backgrounded`, `Usage.cost`,
  `kind = "session"` validation/mapping, per-event session stamping, session in
  span starts, backgrounded tool data.
- `setup-e2e-harness.sh`: pinned v0.7.0 version and checksum.
- `docs/arc42/11-risks-and-technical-debt.md`: R-90 re-verification record.
- `tests/steps/agent_activity_reporting_steps.rs`: reporter delta bindings.
- `tests/steps/development_record_steps.rs`: intake delta bindings.
- `tests/steps/initialize_a_project_steps.rs`: manifest grant bindings (new
  permanent per-capability file, registered in `tests/steps/mod.rs`).
- `tests/steps/development_timeline_support.rs`: shared Maki driver helpers for
  transcript overrides, task-list overrides, and elapsed-time fixtures.

## Data Model Changes

The intake payload (`givn history ingest`) gains, all optional and backward
compatible:

| Location | Field | Meaning |
|---|---|---|
| event | `session` | The session this event's entries belong to; wins over the payload header |
| event (`tool`) | `backgrounded` | The call moved to the background; recorded under `data.backgrounded` |
| event (`session`) | `title` | The session point entry's title, recorded under `data.title` |
| usage | `cost` | The reported dollar cost, recorded under `data.cost` |
| event (`session`) | kind arm | A point entry (no span), named by its title |

Record entries gain no new top-level field. `HistoryEntry.session` already
exists; the per-event override only changes its source.

## Step Definitions

| Capability | Step definition file | Permanent merge target |
|---|---|---|
| agent-activity-reporting | `tests/steps/agent_activity_reporting_steps.rs` | same (durable project source) |
| development-record | `tests/steps/development_record_steps.rs` | same |
| initialize-a-project | `tests/steps/initialize_a_project_steps.rs` | same (new file, registered in `tests/steps/mod.rs`) |

Shared Maki driver code stays in `tests/steps/development_timeline_support.rs`;
it is a support module, not a step binding.

## Interaction Coverage Matrix

All inventory entries are unchanged as interactions; the delta scenarios are
regular evidence for behaviours behind them, and the pinned 0.7.0 binary makes
the existing Maki `@e2e` row a real regression guard for the old paths.

| Inventory entry (owning use case) | Evidence |
|---|---|
| inspect-development-history / development-record: run a lifecycle command | unchanged; existing `@e2e` "Consumer runs a lifecycle step and reads its recorded actions" |
| inspect-development-history / development-record: read the development record | unchanged; existing `@e2e` "Consumer reads the development record as machine-readable output"; delta intake scenarios drive the real CLI |
| inspect-development-history / agent-activity-reporting: report agent activity from a supported harness | unchanged; existing `@e2e` Maki row, re-run on 0.7.0; new behaviours are non-`@e2e` variants |
| inspect-development-history / development-report: generate the development report for a change | unchanged; existing `@e2e` report generation |
| inspect-development-history / development-report: open the report in a browser | unchanged; existing `@e2e` browser scenario |
| inspect-development-history / development-report: read a change's linked ideation | unchanged; existing `@e2e` ideation-group scenario |
| inspect-development-history / development-report: open the report of a scope with no recorded activity | unchanged; existing `@e2e` empty-state scenario |
| inspect-development-history / development-report: expand a workflow phase | unchanged; existing `@e2e` lane-expansion scenario |
| inspect-development-history / development-report: select a timeline element | unchanged; existing `@e2e` selection scenario |
| inspect-development-history / development-report: zoom the timeline | unchanged; existing `@e2e` zoom scenario |
| initialize-project / initialize-a-project: select agent targets in the interactive terminal | unchanged; existing `@e2e` interactive-selection scenario |
| initialize-project / initialize-a-project: initialize with an explicit noninteractive target selection | unchanged; existing `@e2e` noninteractive scenario; the new manifest scenarios are filesystem variants |
| initialize-project / initialize-a-project: resynchronize selected targets | unchanged; existing `@e2e` re-sync scenario; the legacy-manifest scenario is its variant |
| initialize-project / enable-optional-addons: enable the outline addon | unchanged; existing `@e2e` addon scenario |

## External Boundary Cases

The external boundaries are the Maki Lua host (untrusted payload shapes and
transcript content) and the intake's stdin JSON.

| Boundary | Adversarial case | Handling |
|---|---|---|
| Maki transcript | missing `maki.session.messages` or an error pair | `pcall` returns nil; no answer, output, receipts, lanes, or cost |
| Maki transcript | a `tool_result` without a matching `tool_use` | no synthesized tool entry; no content attached |
| Maki task list | host has no interactive UI or returns nil | no session, no cost; span still recorded |
| Maki task events | `idle` after `working` | task stays open; no entry until a terminal event |
| Maki task events | `TaskStopped` for a tool-spawned task | the stop closes the span as skipped; a later completion is ignored by the closed set |
| Maki task events | session end while a task is open | one unknown entry with the measured elapsed time |
| Maki task events | a `task` call moved to the background | the still-listed task keeps the span open; its terminal status closes with the stored title and host duration |
| Maki `ToolDone` | call moved to the background | unknown outcome and `backgrounded` marker at the turn boundary |
| Maki payload | transcript receipts contain another session's call | only buffered tool events with a matching id are marked |
| Maki transcript | a subagent tool call already delivered live | the synthesized lane entry is skipped when the buffer holds that session and call id |
| Maki transcript | a subagent with more than 100 tool calls or 256 KiB of result text | synthesis stops at the cap; live entries are unaffected; the flush stays bounded |
| Maki permissions | the user sets `session_read = false` | transcript reads return nil; metadata-only recording continues, and the user's authority is not overridden |
| Intake payload | event `session` empty or absent | payload header applies |
| Intake payload | `session` kind without a title | rejected by validation with the reason on stderr; no entry |
| Intake payload | `cost` as a non-number | typed `Option<f64>`; serde rejects the event with the reason on stderr; no usage entry |
| Intake payload | duplicate `tool_use` ids in a subagent transcript | each call id is emitted once; a repeated id is skipped |

## Local Runnability

- Regular suite: `./run-tests.sh` (cucumber-rs, `.fail_on_skipped()`).
- E2E tier: `./run-tests.sh --e2e`; requires `./setup-e2e-browser.sh` and
  `./setup-e2e-harness.sh`.
- Single scenario:
  `GIVN_FEATURES=<feature> cargo test --locked --test features_runner -- --name '^<title>$'`.
- Maki binary: `./setup-e2e-harness.sh` installs the pinned 0.7.0 to
  `$GIVN_E2E_BIN` (default `~/.cache/givn-e2e/bin`).
- No server, port, database, or container is introduced.

## Coverage Process Boundaries

The delta adds no coverage exclusion. Reporter branches exercised by the driver
scripts count as regular coverage; the intake arms are exercised by the
development-record deltas. The archive coverage gate runs the existing
`measure-coverage.sh` and `merge-coverages.sh` unchanged.

## Out-of-Scope Technical Decisions

- An upstream Maki payload field for backgrounded calls.
- Per-subagent token counts; only cost is recorded.
- A structured parse of Maki's free-text answer into `selected`; the answer text,
  source, and time are recorded, and `selected` stays absent.
- Any change to the other harness reporters.
