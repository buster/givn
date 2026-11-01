# Tasks: maki-0-7-support

## Setup

- [x] **Runner, strict mode, and pinned harness**
  - Design constraints: `./run-tests.sh --non-e2e` is `verify.command`; `./run-tests.sh --e2e` is `verify.e2e_command`; cucumber-rs uses `.fail_on_skipped()` in `tests/features_runner.rs`; single scenario via `GIVN_FEATURES=<feature> cargo test --locked --test features_runner -- --name '^<title>$'`; RED bodies use `unimplemented!()`.
  - Strictness proof: `GIVN_FEATURES=/tmp/strict-probe.feature cargo test --locked --test features_runner` on a feature with one undefined step exits 1 (`✘ Given an undefined strictness probe step` / `Step doesn't match any function` / `1 scenario (1 failed)`).
  - Harness: `./setup-e2e-harness.sh` installs pinned Maki v0.7.0 (sha256 `3e0a1d1a4bc2bbaafdeb94f2a551bcc2d0f6ed81edfd9dd13272d9caa5add787`) to `$GIVN_E2E_BIN`; `maki --version` prints `maki 0.7.0`.
  - E2E setup: `./setup-e2e-browser.sh` provides Playwright/Chromium; the Maki `@e2e` row already exists in the permanent corpus and is the retained interaction evidence. The delta adds no `@e2e` scenario, so the e2e count is strictly less than the full corpus count.

## Batch: intake-schema

Contract: development-record intake (`src/cli/history.rs`), consumed by
`givn history ingest` and `givn history harness-hook`; entries are append-only;
the payload header session applies unless the event names its own.

- [x] **A reported session point entry is recorded with its title**
  - Design constraints: accept `kind = "session"` only with a non-empty `title`; map one point entry with `data.title`, no duration, no span; validation runs before mutation.
  - COMMIT: `a0a1868c` -- Batch: intake-schema
- [x] **A reported event may name the session it belongs to**
  - Design constraints: `IngestedEvent.session` wins over the payload header in both intake paths (`ingest`, `harness-hook`); the header fills only entries with no session; span starts keep their event session.
  - COMMIT: `a0a1868c` -- Batch: intake-schema
- [x] **A tool the agent moved to the background is marked**
  - Design constraints: `IngestedEvent.backgrounded = true` is stored under tool `data.backgrounded`; the entry is otherwise unchanged and never rewritten.
  - COMMIT: `a0a1868c` -- Batch: intake-schema
- [x] **A reported usage carries its cost**
  - Design constraints: `Usage.cost` is `Option<f64>`; a subagent span's inline usage creates one `usage` entry with `data.cost`; absent cost stays absent.
  - COMMIT: `a0a1868c` -- Batch: intake-schema
- [x] **A reported session entry without a title is rejected**
  - Design constraints: the event is rejected per event with `warning: the intake rejected event 1 (kind `session`)` and the missing field; exit stays 0; no session entry is appended.
  - COMMIT: `a0a1868c` -- Batch: intake-schema
- [x] **A reported usage cost that is not a number is rejected**
  - Design constraints: serde rejects `usage.cost` typed as a number; the warning names the event; no usage entry is appended.
  - COMMIT: `a0a1868c` -- Batch: intake-schema

## Batch: reporter

Contract: Maki activity reporting (the generated Lua module), consumed by the
Maki host; every read is wrapped and degrades to an absent value.

- [x] **Complete mode records a Maki tool call's output from the session transcript**
  - Design constraints: read `tool_result` blocks by `tool_use_id` (roles are only user/assistant); content only under `record.mode: full`; the intake re-applies caps.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A Maki tool call moved to the background is not recorded as completed**
  - Design constraints: read `receipts` at `TurnEnd`; mutate the buffered tool event to `outcome = "unknown"` and `backgrounded = true`; `ToolDone` alone cannot detect the move.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A Maki question records its answer, source, and time**
  - Design constraints: find the last `question` tool_use and its `tool_result` in the transcript; send bounded `answer`, `answered_by = "user"` (reporter classification), `answered_at` (reporter resume stamp); keep the existing label bound.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A Maki session title is recorded as a session entry**
  - Design constraints: handle `SessionTitleChanged`; deduplicate consecutive titles; emit one point entry with the session id and title; never emit on status changes.
  - COMMIT: `17673fcf` -- Batch: reporter


- [x] **A tool-spawned Maki subagent is recorded once with the harness's own duration**
  - Design constraints: defer a terminal status while `state.tools[id]` is pending; close on `ToolDone` with host duration, title from `input.description`, and agent name from the task state; one entry regardless of event order.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A paused Maki subagent stays open until it finishes**
  - Design constraints: `idle` is non-terminal; a plugin task closes on its terminal status with the reporter-measured duration.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A stopped Maki subagent is recorded as skipped**
  - Design constraints: `TaskStopped` closes any open task as `cancelled`/`skipped`; the closed set prevents a later completion from reopening it.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A Maki subagent moved to the background stays open until it finishes**
  - Design constraints: a task still listed at its `task` completion is moved; store title and host duration, keep the span open, close on the terminal status with the stored values.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A Maki subagent still running when the session ends is recorded as unknown**
  - Design constraints: `SessionEnd` closes every open task once with `status="unknown"`, `outcome="unknown"`, and the observed elapsed time; deterministic key order.
  - COMMIT: `17673fcf` -- Batch: reporter


- [x] **A Maki subagent's tool calls are recorded under its own session**
  - Design constraints: at close, read `maki.task.list()` for the task's `session`, read that transcript, emit each completed `tool_use` once with `session=<subagent session>`; skip ids already buffered; cap 100 calls and 256 KiB; content only in full mode.
  - COMMIT: `17673fcf` -- Batch: reporter
- [x] **A Maki subagent's cost is recorded where the harness reports it**
  - Design constraints: read `cost` from the task list at the close (never at start); send a cost-only inline usage; absent stays absent.
  - COMMIT: `17673fcf` -- Batch: reporter

## Batch: maki-install

Contract: project installation (`src/skills/targets.rs`), consumed by `givn init`
and `givn skills sync`; the manifest is directory-wide and user-owned.

- [x] **The generated Maki permission manifest names the session read**
  - Design constraints: the created asset grants `fs_read`, `fs_write`, `run`, `session_read` and revokes `net`, `env`, `agent`, `session_write`, `session_settings`, `input`; `MAKI_REPORTER_GRANTS` gains only `session_read`.
  - COMMIT: `80742b9e` -- Batch: maki-install
- [x] **Synchronizing an existing Maki manifest adds only the session read grant**
  - Design constraints: merge per key; add `session_read = true` to a legacy six-key manifest; never flip or remove a user key; add no revocation.
  - COMMIT: `80742b9e` -- Batch: maki-install
