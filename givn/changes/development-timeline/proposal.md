# Proposal: development-timeline

## Release note

> Givn records the development process of every change and ideation — each step
> it runs, the activity reported by every supported coding-agent harness,
> question waits, and token usage — and renders it as an interactive timeline
> report that opens in a browser.

## Use-Case Context

- Use-case ID: `inspect-development-history` (created by this change)
- Ideation topic: none
- Seed path: none
- Confirmed Personas: `developer--primary-user` (the requester asked for this
  report for their own changes and ideations — "I want to see when givn
  ideation started, when propose started … how much tokens were used"), and
  `agent--technical-role` (the coding-agent harness whose activity is reported
  and that drives the intake as an automation caller). No Persona biography is
  copied into this proposal.

## Problem / Opportunity

A Givn change takes hours to days: ideation, propose, specs, design, design
review, tasks, implementation, review, archive. The shape of that time is
invisible. Givn prints only a current-state answer — which artifacts are done,
what to do next — and the archive receipt keeps two scope durations for one
command. Nothing answers "where did the time go?".

The developer pays for that blindness in three concrete ways:

- **Slow steps are undiscoverable.** Archiving runs the archive gate, the
  regular suite, the e2e suite, and the coverage gate; a single one of those can
  dominate a day. The developer has no way to see that the verify scope took
  forty minutes while writing the spec took two.
- **Agent-work shape is invisible.** The coding agent may spend a long stretch
  in one tight loop around one file, or fan a step out into a dozen tiny
  subagents whose combined overhead exceeds the work. Nothing records either
  pattern, so neither can be corrected.
- **Waiting is indistinguishable from working.** When the agent stops to ask a
  question, the run looks identical to a run that is thinking. The developer
  cannot see how much of the elapsed time was spent waiting for a human answer.

The same blindness applies to ideation, where the work happens before any change
directory exists, so even the current-state answer has nothing to say.

## Proposed Solution

- **Givn records the development process durably.** While the developer and the
  agent work, Givn keeps a time-ordered record of what happened: each command it
  runs and each step inside it (a gate hook, a verification run, an archive
  phase, a commit), each reported agent activity (a tool call, a turn, a
  subagent start and end), each question wait, and the token usage reported for
  a turn or a subagent. Each recorded entry carries its start, its duration, its
  scope (the change, the ideation topic, or the project), and its outcome.
- **The coding agent reports its own activity.** The agent integration is
  installed with the other generated integrations and reports through one Givn
  interface, so a harness that is not the agent's is not needed to see its work,
  and a second harness can report later without changing the record's shape.
- **Recording is never in the way.** A record that cannot be written never fails
  the command or the agent action it was recording, and it never changes that
  action's outcome. The record is append-only: a recorded entry is never
  rewritten.
- **Every supported harness reports its own activity through the same intake.**
  "Supported" is the repository's own contract, not an assumption: the agent
  targets are `opencode`, `agents`, `claude`, and `omp`, and three of them
  execute the agent's work and therefore have activity to report — opencode,
  Claude Code, and omp. Each receives its own generated reporter alongside the
  commit-attribution integration it already gets, and every reporter delivers
  through one Givn interface, so the record has one shape and one privacy bound
  regardless of which harness produced the activity. The `agents` target is a
  skills mirror that no harness executes; it receives no reporter, and the
  report simply has no activity for it. A harness that cannot expose a signal
  (token usage, subagent identity, or a question wait) contributes what it
  does expose, and the report states the gap rather than inventing a value.
- **The developer asks for the report and reads it in a browser.** For a given
  change or ideation, Givn produces one interactive report: a zoomable timeline
  of the whole development, the phases in order, the longest steps, the
  subagent work, the question waits, and the token totals. Selecting an element
  reveals its details; zooming narrows the time range. The report is one
  self-contained file — it needs no server, no port, and no network access — and
  its path is printed so the developer decides whether to open it.
- **Report views that answer the stated questions.** The report ranks the
  longest single steps (so an outsized archive or verification run is obvious),
  groups the agent's small units of work (so fan-out into many tiny subagents
  is obvious), and marks every wait for a question as its own span with the
  total time spent waiting.
- **A change with no recorded activity still reports.** A scope whose record is
  empty renders an explicit empty state instead of a blank page or an error.

## Capability Routing

`givn spec route --change development-timeline` reports no signal — its top
candidates tie at 15.00 and no ranked list is printed — so each decision is
recorded directly with its rationale. All three capabilities are new, and they
share one actor goal: understanding how a change or an ideation was developed
over time. Recording is a supporting behavior of that goal (the same shape as
`validate-change-ids` under `inspect-change-state`), not a goal of its own, so
it lives under the same use case rather than in the reusable `fragments` home:
the record has exactly one consumer-facing story (the history of one scope) and
no second use case includes it.

| Proposed capability | Route's recommendation | Decision | Rationale (if you deviated from route) |
|---|---|---|---|
| development-record | no signal | `NEW in inspect-development-history` | No existing capability owns recording what Givn does; `inspect-change-state` owns current artifact state, not history, and cannot cover ideation. |
| agent-activity-reporting | no signal | `NEW in inspect-development-history` | Reporting the agent's work only matters as input to the same history; a separate use case would have no independent actor goal. |
| development-report | no signal | `NEW in inspect-development-history` | Rendering the history for the developer is the goal of the same use case; the browser view and the CLI generation are two actions under that one goal, not two capabilities. |

## Out of Scope

- **No reconstruction of the past.** Changes archived before this feature have no
  record; the archive receipt keeps its current two scope durations and nothing
  backfills them from revision history.
- **No reporter outside the repository's supported harness set.** A harness the
  project does not support as an agent target gets no generated reporter, even
  though the intake interface is not specific to any one harness.
- **No remote or shared history.** Nothing is uploaded, aggregated across
  repositories, or published by continuous integration. The report is a local
  file.
- **No change to the archive proof.** The archive receipt, the verification
  scopes, the coverage gate, and the reachability rules are unchanged; the
  record is a consumer of that information, never a replacement.
- **No change to what a command does.** Instrumentation observes; it does not
  alter exit codes, gate order, or artifact lifecycle.

## Open Questions

### D1 — Where the development record lives, and whether it becomes repository content

- Question: Should the development record be a local, project-scoped store that
  version control ignores, a set of files committed inside the change (and
  archived with it), or a user-level store outside the project?
- Context: the record holds timing, tool names, token counts, and short labels
  for everything the developer and the agent did — including work on private
  code, and the ordinary contents of shell commands and questions. It is
  consulted per change or per ideation, so it must sit next to the project, but
  whether it enters the repository history is a different decision: a committed
  record is reviewable and permanent, and cannot be unpublished without a
  history rewrite. The choice fixes the store's path, its ignore entry, and
  whether a fresh clone has a timeline at all.
- Options:
  1. **Local project store (ignored by version control)** — one store under the
     project that version control ignores.
     Gain: agent telemetry and short command labels never enter the repository
     or its history, while the record still lives next to the code it describes.
     Cost: a fresh clone, a second machine, or a deleted store has no history,
     and a report for a change developed elsewhere is empty.
     In practice: after cloning the project on a new laptop, the developer opens
     the report for an old change and sees the empty state.
  2. **Committed with the change** — the record is written into the change's own
     directory and archived with it, exactly like the archive proof.
     Gain: the development history is reviewable by a teammate and permanent;
     the archived change explains how it was built.
     Cost: every scenario commit carries a telemetry file, the repository grows
     with agent activity, and removing it later needs a history rewrite.
     In practice: `git log` for one change shows the timeline file changing once
     per scenario commit, and a reviewer sees the agent's tool calls in the
     diff.
  3. **User-level store outside the project** — one store per developer, keyed
     by project path.
     Gain: no repository footprint at all, and the history survives deleting
     the checkout; one place to look across projects.
     Cost: the record is invisible to project review and is not part of the
     archived change, and the archived change cannot be re-read by a teammate.
     In practice: reinstalling the machine loses every timeline with the home
     directory.
- Disposition: answered — the operator chose **option 1, the local project
  store** (`givn/history/events.jsonl`, kept out of repository history by the
  managed ignore block) on 2026-10-16. Rejected: option 2, committed with the
  change, because agent telemetry would enter repository history and could then
  only be removed with a history rewrite; and option 3, the user-level store,
  because it is invisible to project review and is lost with the home
  directory. Recorded as `Decides D1` in `design.md`.

### D2 — How much detail the record keeps about agent activity

- Question: Should each agent activity entry keep metadata and a short label, the
  complete tool input and question text, or counts and timings only?
- Context: the report must let the developer recognize *what* the agent was
  working on — that is how a loop around one file, or fan-out into tiny
  subagents, becomes visible. The same entries, however, pass through shell
  commands, file paths, and question text, which can contain credentials, private
  code, or third-party content. The detail level decides how sensitive the store
  is, which matters most if D1 chooses a committed record.
- Options:
  1. **Metadata and a bounded label** — kind, timing, outcome, model, token
     counts, subagent identity, plus a one-line label for the subject (truncated
     to a fixed length).
     Gain: enough to see loops, long steps, and fan-out, without storing
     transcripts; the store stays small.
     Cost: the label is derived and truncated, so an unusual action can be
     unrecognizable later.
     In practice: a shell step appears as "run: cargo test (1.2 s)" instead of
     the complete command line.
  2. **Complete payloads** — the full tool input, the command text, and the
     complete question text.
     Gain: complete forensics; the developer can replay exactly what the agent
     did and read every question the agent asked.
     Cost: secrets, tokens, and third-party content that pass through a command
     land in the record — and, with D1 option 2, in the repository; the store
     grows with every tool call.
     In practice: a command containing a database URL or an API key is written
     into the record verbatim.
  3. **Counts only** — how many tool calls, how long, how many tokens, with no
     subject at all.
     Gain: the smallest and least sensitive record.
     Cost: the developer cannot see which task the agent looped around, which is
     one of the stated goals.
     In practice: the timeline shows "14 tool calls, 3.1 s" with no hint of what
     they were.
- Disposition: answered — the operator chose **option 1, metadata plus a bounded
  120-character label**, on 2026-10-16. Rejected: option 2, complete payloads,
  because credentials and private code that pass through a command would be
  written into the record; and option 3, counts only, because the developer
  could not see which task the agent looped around — one of the stated goals.
  Recorded as `Decides D2` in `design.md`.

### D3 — How agent-reported activity reaches the record

- Question: Should the installed agent reporter hand its events to a Givn
  interface that validates and appends them, append them to the record file
  itself, or write nothing and have Givn read the harness's own session log when
  the report is generated?
- Context: the reporter observes the tool calls, turns, subagent work, question
  waits, and token usage of the coding agent, and the record has to receive them
  without slowing the agent down. This decides who owns the record's format, how
  many processes are started per session, and how tightly Givn is coupled to the
  harness it reports from.
- Options:
  1. **A Givn-owned intake interface** — the reporter buffers events and hands
     them to a Givn command that validates and appends them (batched at the end
     of a turn and at session end).
     Gain: one owner of the record's format; the reporter stays thin; a second
     harness reuses the same interface.
     Cost: one process per batch, and a version relationship between the
     reporter and the installed Givn.
     In practice: at the end of a turn the reporter sends that turn's events to
     Givn, which writes them; a stale reporter's unknown event kind is rejected
     with a warning while the rest are kept.
  2. **The reporter writes the record file directly** — the installed
     integration appends record lines itself.
     Gain: no process per batch; the lowest overhead.
     Cost: the record's format is re-implemented in every harness integration,
     and a defect there corrupts the record Givn later reads.
     In practice: an upstream harness change silently writes lines the report
     cannot read.
  3. **Read the harness's own session log at report time** — Givn parses the
     coding agent's private session files when the report is generated.
     Gain: nothing is written while working.
     Cost: the record's content is coupled to another product's private on-disk
     format, and activity is invisible for a harness that keeps no such log.
     In practice: a harness upgrade that reshapes its session file breaks every
     existing report at once.
- Disposition: carried — decided in `design.md` (D3).

### D4 — How the report is produced and shown

- Question: Should the report be produced on demand by a command that writes one
  self-contained file and prints its path, published automatically whenever a
  change is archived, or served live by a local server?
- Context: the developer wants to look at a specific change or ideation,
  sometimes while it is still running. This decides whether Givn gains a
  listening port, whether the report can go stale, and how much a viewer has to
  do before seeing a timeline.
- Options:
  1. **On demand, one self-contained file** — the developer asks for the report
     of a scope, Givn writes a single file that needs no server and no network,
     and prints its path (optionally opening the browser).
     Gain: works offline, no port or process to manage, and the file can be kept
     or handed to someone else.
     Cost: the developer must ask for it, and a saved file goes stale as work
     continues.
     In practice: the developer runs the report command, opens the printed path,
     and sees the timeline as of that moment.
  2. **Automatically at archive (and at review)** — every archived change
     publishes its report next to the archived artifacts.
     Gain: every archived change has a report without anyone asking.
     Cost: the archive gate grows a rendering step, and a frozen report cannot
     benefit from a later fix to the report itself — the in-flight case still
     needs the on-demand path.
     In practice: improving the timeline's layout means re-archiving changes to
     see the improvement, or living with two report generations.
  3. **A local server with a live view** — Givn serves the report and updates it
     as the run proceeds.
     Gain: a view that follows a running change.
     Cost: a listening port, a lifecycle to supervise, and a network surface in
     a tool that has none today.
     In practice: a stray server keeps the port after the run ends, and every
     debugging session starts with "is the server still up?".
- Disposition: carried — decided in `design.md` (D4).

### D5 — What the record captures outside a change or ideation

- Question: Should Givn record everything that happens in the project, only what
  happens inside a change or an ideation, or only its own commands outside those
  scopes?
- Context: the requester asked for "the overall development process", but the
  report is read per change and per ideation. Work that happens between two
  changes — investigating a bug, fixing a build, reading the code base — is
  still development time, and the answer decides whether the record has entries
  that belong to no scope, and whether the report needs an unattributed band.
- Options:
  1. **Everything, with an unattributed band** — the record keeps every action,
     and activity that belongs to no change or ideation is shown as its own
     band.
     Gain: the developer sees where time went even outside the workflow.
     Cost: the record grows with unrelated work and the report needs an
     "outside a scope" concept to explain those entries.
     In practice: an afternoon of debugging a failing build appears as
     unattributed time next to the change that followed it.
  2. **Only inside a change or an ideation** — recording starts when a scope
     starts and stops when it ends.
     Gain: the record stays small and every entry is attributable.
     Cost: work done outside the workflow, and the time between two changes, is
     invisible.
     In practice: the session where the developer investigated a defect before
     opening a change does not appear anywhere.
  3. **Givn's own commands everywhere, agent activity only in a scope** — two
     implicit rules.
     Gain: the cheapest capture with some coverage outside a scope.
     Cost: a reader cannot tell the rule from the report, and the agent's work
     outside a scope is lost anyway.
     In practice: the report shows commands around a window but no agent work in
     it.
- Disposition: carried — decided in `design.md` (D5).

### D6 — Which visual direction the report takes

- Question: Which of the proposed rendered directions should the development
  report use — the dense timeline, the phase-lane view, or the dashboard?
- Context: this change introduces Givn's first browser screen, so the project has
  no design system yet; the direction decides the tokens, the primitives, and
  the reference view recorded in `docs/design/design-system.md`, and the
  rendering rules every later browser surface inherits. The design session
  cannot approve its own visual direction; three rendered options are prepared
  in the design system and the operator picks one.
- Options:
  1. **Dense timeline** — one horizontal time axis with nested spans (commands,
     steps, gate phases, agent ticks, question waits), zoom and pan, and a
     details panel on selection.
     Gain: the shape of the whole development is readable at a glance, and the
     longest bar is visually obvious; nesting shows which subagent work belongs
     to which step.
     Cost: with hundreds of entries it needs aggregation rules, and very short
     agent ticks are invisible until zoomed.
     In practice: the archive gate is the widest bar; zooming into it shows the
     regular suite and the e2e suite inside.
  2. **Phase-lane view** — one lane per workflow phase, each lane listing its
     steps and their durations, with the waits and subagent work as their own
     lanes.
     Gain: it mirrors the workflow the developer already knows, and comparing
     phases is direct.
     Cost: it hides the absolute time axis, so "when did this happen" and
     overlap between phases are not readable.
     In practice: the developer sees "Implementation: 12 steps, 4 h 20 m" but
     not that half of it was a question wait.
  3. **Dashboard** — a summary of totals, the longest steps, the wait share, and
     the subagent fan-out, with the timeline behind a click.
     Gain: the stated questions are answered with numbers immediately.
     Cost: the shape of the development is one click away, and a dashboard
     invites more widgets later.
     In practice: the first screen shows "longest step: archive gate — 41 m" and
     the developer must click to see when that gate ran.
- Disposition: answered — the operator approved **option 1, the dense
  timeline**, on 2026-10-16 after reviewing the three rendered reference views
  prepared in `docs/design/design-system.md`. Rejected: option 2, the phase-lane
  view, because it hides the absolute time axis and the overlap between phases;
  and option 3, the dashboard, because the shape of the development is what the
  requester wants to see first. Recorded as `Decides D6` in `design.md`, which
  also records the approval.
