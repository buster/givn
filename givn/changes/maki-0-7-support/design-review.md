Review kind: self-review
Author: omp agent (maki-0-7-support author)
Reviewer context: omp agent (maki-0-7-support author)

A fresh read-only grill (`GrillMakiDesign`) was consulted for phase 1; the author applied and hardened the outcomes. The grill received the proposal, delta specs, design, and served rules.

# Design review: maki-0-7-support

## Phase 1 — grilling

The grill returned nine ranked findings, two blocking correctness gaps, one
blocking scope gap, one blocking process gap, and several high/medium
weaknesses. Each is recorded below with its disposition.

### F1 — Capability identity (blocking)

- Claim: the delta was tagged `@initialize-project`; the capability is
  `initialize-a-project`, and the archive would create a new permanent file
  beside the real one.
- Disposition: fixed. Feature tag and proposal/design rows now name
  `initialize-a-project`; the delta path and merge target are unchanged.

### F2 — Subagent identity and dedupe (blocking)

- Claim: the design never stated how a task id, its tool id, and the record's
  `subagent` identity relate, so the wrong id could be recorded.
- Disposition: fixed. Maki's own task model states "a task is a subagent chat,
  addressed by its `tool_use_id`"; the design records `subagent` = task id =
  tool id and states the five-case close rule with a per-session closed set.
  The span scenario asserts the identity, agent, title, duration, and that
  exactly one entry exists.

### F3 — Backgrounded subagent (blocking)

- Claim: a `task` call moved to the background emits its `ToolDone` at move
  time, so closing on it ends the span early and the later status is dropped.
- Disposition: fixed. At a task call's completion the reporter consults
  `maki.task.list()`; a task still listed is moved, so the span stays open with
  the completion's title and host duration and closes on its terminal status or
  the session-end sweep. Verified by a new scenario whose prelude keeps the
  task listed through the completion.

### F4 — Lane synthesis duplication and bounds (blocking)

- Claim: live subagent-session tool events plus synthesis can duplicate; the
  design named no count or byte bound.
- Disposition: fixed. Synthesis skips a call id already buffered for that
  session, runs once per session, caps at 100 calls and 256 KiB of result text,
  and the lane scenario asserts exactly one entry. The bound and the skip rule
  are in the design's boundary table.

### F5 — RED baseline (blocking process)

- Claim: four intake scenarios were already green in the dirty worktree during
  the review, so RED could not be claimed.
- Disposition: the change's baseline is `main` (9eefd069), not the in-flight
  worktree. On that baseline `kind:"session"` is rejected, `Usage` has no cost,
  tool events have no background marker, and events have no session override.
  The RED evidence recorded in `tasks.md` is reproduced against the committed
  pre-change revision; the worktree moving during review is not used as proof.

### F6 — Unresolved cost validation (blocking)

- Claim: the boundary table left the malformed-cost handling as a question.
- Disposition: fixed. `Usage.cost` is `Option<f64>`; a non-numeric value rejects
  the event with the reason on stderr. A scenario proves the rejection and the
  absence of a usage entry.

### F7 — Answer source and time

- Claim: the transcript has no answer timestamp or provenance.
- Disposition: recorded. `answered_by = "user"` is the reporter's classification
  of Maki's question tool, and `answered_at` is the reporter's observed resume
  stamp; the design and glossary say so. The scenario asserts the text, the
  source word, and the presence of the time.

### F8 — Background detection timing

- Claim: the scenario did not exercise the turn boundary where receipts are
  read.
- Disposition: fixed. The scenario includes `TurnEnd`; D2 states that the
  buffered tool event is mutated at `TurnEnd`, because the receipt is written
  after the completion.

### F9 — Missing scenarios

- Disposition: added the backgrounded-subagent, title-less session rejection,
  non-numeric cost rejection, and "sync adds no revocation" scenarios. The pin
  itself is recorded as setup/CI evidence, not scenario evidence; older-host
  inertness is recorded as manual verification in D5 and R-90 because the suite
  pins one binary.

### F10 — Testability

- Disposition: fixed. The subagent, lane, cost, answer, and session-end
  assertions name concrete fixture values (500 ms, `Explore`, `find callers`,
  0.42, `Gate: archive`, `unknown` + elapsed > 0). The design names
  `.fail_on_skipped()` as the strict-mode mechanism, `unimplemented!()` as the
  RED stub pattern, and the single-scenario command.

### F11 — Interaction coverage matrix

- Disposition: fixed. The matrix now has one row for each of the 14 inventory
  entries in both use cases, each naming the existing `@e2e` evidence or the
  delta variant.

### F12 — Contracts and consumers

- Disposition: fixed. The intake's second consumer is named
  `givn history harness-hook` (not "Maki lifecycle hook"); the Maki contract
  names the autocmd stream and the transcript/task-list read mechanisms; the
  single-file justification and split trigger are recorded.

### F13 — Cost read timing

- Disposition: fixed. Cost is read at the span's close, and at a moved task's
  completion while it is still listed; a start-time read is never recorded.

### F14 — `session_read = false`

- Disposition: recorded as a boundary. The user's manifest authority is not
  overridden; transcript reads degrade to absent metadata, metadata recording
  continues, and the manifest comment says so.

### F15 — Glossary and arc42

- Disposition: fixed. Five terms were added to `docs/glossary.md`; chapters 02,
  05, 06, 07, 08, 11, and 12 were updated and `arc42.md` records the
  assessment. No ADR: the schema extension stays inside the existing record
  contract.

## Branch coverage

- Scope: every proposal bullet maps to a delta scenario or a recorded
  setup/manual-evidence waiver; no extra product scope.
- Tech choices: existing Rust/Lua/cucumber-rs stack; no new dependency; Maki
  0.7.0 pin verified by checksum.
- Missing scenarios: the boundary table's adversarial cases now have scenarios
  for title-less sessions, non-numeric cost, duplicate lane calls, moved calls,
  stopped/paused/backgrounded/open tasks, and missing transcripts (degrade).
- Testability: concrete values, stub pattern, strict mode, and single-scenario
  command are named.
- E2E fidelity: the existing inventory actions are unchanged; the pinned Maki
  row is the regression guard, and no delta `@e2e` duplicates it.
- Risk: the single most likely failure is task-identity correlation across
  three sources; the closed set and the deferred-close rule are the mitigation,
  with scenarios for foreground, moved, stopped, idle, and session-end order.
- Use case and Persona: owning use cases and the two confirmed Personas are
  recorded; no Persona biography is used as an actor.
- Decision ledger: D1–D5 all resolve; D2/D3's timing and cost rules were
  tightened after the grill.
- Contracts and components: three contracts, one implementing component each;
  the reporter's two mechanisms are named inside the one-file constraint.
- External boundaries and version freshness: both walked; the pin's checksum
  and release date are recorded with the lookup source.

## Phase 2 — hardening

Applied after the grill:

- Renamed the capability tag and routing rows.
- Added the span-correlation rule and the backgrounded-task deferral.
- Added the buffered-entry skip and the lane cap.
- Typed the usage cost and made malformed values a rejection.
- Added the four missing scenarios and the sync no-revocation assertion.
- Filled the interaction matrix, contract consumers, glossary, arc42, and R-90.
- Tightened the assertions to concrete fixture values.

DESIGN-REVIEW: PASS

Hardening re-pass date: 2026-11-01.

DESIGN-REVIEW: PASS
