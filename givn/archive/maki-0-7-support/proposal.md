# Proposal: maki-0-7-support

## Release note

The Maki reporter now follows Maki 0.7.0, records full-mode tool output, question answers, subagent lanes, session titles, backgrounded calls, and subagent cost, and counts each subagent once.

## Use-Case Context

- Use-case ID: `inspect-development-history` — capabilities `agent-activity-reporting` and `development-record`; supporting owner `initialize-project` for the generated grants.
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user`, `agent--technical-role`.
- Source: the operator's investigation of the Maki 0.7.0 release and the follow-up instruction to land every verified compatibility and extension item.

## Review Context

- Review kind: self-review
- Reason: extends an existing local-only recording contract; the record modes, content caps, scope rules, and best-effort failure semantics are unchanged, and no new lifecycle artifact, persistent product data, or external boundary is introduced.

## Problem / Opportunity

Maki 0.7.0 changed the contracts the generated reporter depends on:

- Session content of events and the live transcript now requires an explicitly named `session_read` grant. A project manifest that does not name it relies on Maki's permissive default, so the grant is invisible and disappears silently if that default changes or the manifest is copied into a package.
- The live transcript returns `tool_result` blocks in user messages. The reporter's full-mode tool-output lookup expects a role and field Maki never emits, so complete mode has never recorded a Maki tool output.
- A subagent now reports through `TaskStatusChanged` until its end and through a new `TaskStopped` event when the user stops it. The reporter records a tool-spawned subagent twice — once on status and once on the tool completion — marks a paused task failed, and drops a stopped task.
- A call moved to the background reports one completion at move time, with no outcome for the work that continues. The reporter records it as completed.
- A question's answer is available in the transcript, but the reporter records only the question text, so the record cannot say what was answered.
- Session titles and subagent costs are now exposed to plugins, but the record has no way to accept a session point entry, a per-event session, or a reported usage cost, so lanes stay unnamed and subagent spend stays unavailable.
- The end-to-end harness pins Maki 0.6.3. Maki 0.7.0 keeps the same plugin surface for the reporter's existing paths, but the pin no longer matches the version the mapping is verified against.

## Proposed Solution

The Maki reporter and the record intake follow Maki 0.7.0 while staying inert on older Maki versions:

- The generated Maki permission manifest names `session_read` and explicitly revokes `session_write`, `session_settings`, and `input`; synchronization adds only the grant to an existing manifest, never a revocation.
- Complete mode records a tool call's output from the matching transcript result block.
- One subagent span is recorded once: a tool-spawned subagent closes on its tool completion with the host duration, title, and agent name; a plugin-owned task closes on its own status or stop; a task that only pauses stays open; a stopped task is recorded as skipped; a task still running when the session ends is recorded once as unknown with the elapsed time observed so far.
- A tool call that the transcript shows moved to the background is recorded as unknown rather than completed.
- A Maki question wait records the answer text, the answering source, and the answer time when the transcript holds the matching result.
- A Maki session title is recorded as a session point entry, and a subagent's own transcript contributes its tool calls as entries belonging to that subagent's session.
- A subagent's reported cost is recorded as a usage value when the host exposes it.
- The end-to-end harness installs the pinned Maki 0.7.0 binary.

## Capability Routing

`givn spec route --change maki-0-7-support` reports `EXTEND givn-workflow-enforcement` (signal). That signal comes from the proposal's review vocabulary, not from the behavior: the change extends recording and generated installation, not the workflow gates.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| `agent-activity-reporting` | `EXTEND givn-workflow-enforcement` | `EXTEND agent-activity-reporting` | The behavior is a harness reporter's activity mapping and its E2E installation, which this capability owns. |
| `development-record` | `EXTEND givn-workflow-enforcement` | `EXTEND development-record` | The intake additions (session point entries, per-event session, usage cost) change what the record accepts and stores. |
| `initialize-a-project` | `EXTEND givn-workflow-enforcement` | `EXTEND initialize-a-project` | The generated Maki permission manifest is written by project initialization and synchronization. |

## Out of Scope

- An upstream Maki payload field naming a backgrounded call; the reporter infers the move from the transcript instead.
- Per-subagent token counts; only the host-reported cost is recorded where available.
- Changing other harness reporters or their event vocabularies.
- Session export/import, chat-input programming, keymap modes, provider registration, and other Maki 0.7.0 surfaces the reporter does not use.
- Cryptographic protection of the record or its permissions.
- Capturing a question's selected option as a single value; Maki's answer text may carry several questions and labels.

## Open Questions

### D1 — How far does the intake schema extend?

- Status: answered 2026-11-01 (operator instruction to land every verified extension).
- Disposition: accept a reported `session` point entry, a per-event session override, a tool background marker, and a usage cost, each with its own validation and tests.

### D2 — How is a backgrounded call detected without a host field?

- Status: answered 2026-11-01 (reproduced 0.7.0 behavior; no `backgrounded` in the Lua payload).
- Disposition: read the transcript at the turn boundary and mark calls its moved-call receipts name as unknown.

### D3 — Where does subagent cost come from?

- Status: answered 2026-11-01 (0.7.0 `maki.task.list()` exposes `cost`).
- Disposition: query the task list at the subagent close and record the cost in a usage value when present; absence stays absent.

### D4 — Are subagent tool calls recorded?

- Status: answered 2026-11-01 (operator instruction to land the per-subagent lane extension).
- Disposition: record the subagent transcript's tool calls under the subagent's own session, metadata always and content only in the full mode.

### D5 — Is a Maki version floor required?

- Status: answered 2026-11-01 (registration and synthetic delivery verified on 0.6.3 and 0.7.0).
- Disposition: none; new handlers are inert on older versions and the manifest stays loadable.
