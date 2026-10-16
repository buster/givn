---

status: accepted
date: 2026-10-16
decision-makers: buster (project owner)
consulted: none
informed: none

# Own the development record inside Givn and let harnesses report through one intake interface

## Qualification

ADR-QUALIFICATION: PASS

### Qualification verdict

qualification: QUALIFIED
alternatives: PASS
architectural_impact: PASS
durable_consequence: PASS
lower_level_artifact: PASS
existing_adr_check: PASS
must_be_shared: SUPPORTING
routing: NEW_ADR
canonical_artifact: null
target_adr: null
replacement_adr: null

Evidence: the `development-timeline` proposal weighed three materially
different delivery options for agent-reported activity — a Givn-owned intake
interface (chosen), each harness integration appending record lines itself,
and Givn reading the harness's private session log at report time (proposal
D3); no lower-level artifact owns the boundary: the delta Gherkin specifies
the observable recording and reporting behaviour, `design.md` owns the record
schema and the capture points, and no process document or existing Arc42
chapter decides who may write the record or how a harness delivers to it; the
existing register was searched in full — ADR-014 owns filesystem-derived state
and explicitly no state file, ADR-013 owns generated agent-integration assets
and `AGENTS.md` ownership, ADR-028 owns the post-hook versus mandatory-check
boundary, and ADR-070/ADR-072 own the archive receipt and its witness-bounded
publication — and none of them owns a durable record of development activity or
its delivery contract: the record consumes the receipt's scope durations as step
entries and owns no part of that proof boundary, so the route is a new record
rather than an amendment.

### MADR checklist evidence

- [x] **Hard to reverse** - Reverting later would cost significant effort
      because: the record's on-disk format becomes durable project data, and the
      reporter asset is already installed in every project that selected the
      `omp` target; replacing the intake means re-shipping a generated asset and
      migrating or discarding existing records.
- [x] **Shapes quality attributes** - It materially affects maintainability and
      privacy because: one owner of the record format keeps every harness
      integration thin, and the intake is the single place where reportable
      detail is bounded and validated.
- [x] **Structural** - It defines a boundary, contract, and dependency
      direction because: the record module owns the store and its schema, the
      intake command is the only writer for reported activity, and harness
      integrations depend on that contract instead of on the file format.
- [x] **Must be shared** - Developers unfamiliar with it would systematically
      violate the architecture because: adding a second reporter by appending
      record lines directly, or reading another product's session files at
      report time, would fork the format and break every existing record.

## Context and Problem Statement

Givn can time a verification scope and record two timestamps in an archive
receipt, but it keeps no chronological record of the development process: not
of the commands it runs, not of the phases inside them, and not of what a coding
agent does while it works. State is recomputed from the filesystem on every
invocation (ADR-014), so there is deliberately no place that already holds a
history. A report over the development of one change or ideation needs such a
record, and the activity that matters most — the agent's work — happens in a
separate process owned by a harness Givn does not control.

## Decision Drivers

- One owner of the record's on-disk format and its validation surface.
- Thin, replaceable harness integrations: adding a second reporter should not
  mean re-implementing the record.
- Recording must never change the outcome of the work it records, and must not
  add a gate.
- The record is development data; it must not require a listening port, a
  network service, or a database.
- The rejected alternatives must stay available as decisions for a later
  reporter without reopening this boundary.

## Considered Options

- **A Givn-owned intake interface**: the harness integration buffers its events
  and hands them to a Givn command that validates and appends them.
- **Each integration appends record lines directly**: the generated harness
  asset writes the record's line format itself.
- **Read the harness's private session log at report time**: Givn parses the
  coding agent's own session files when the report is generated.

## Decision Outcome

Chosen option: "A Givn-owned intake interface", because the record's format then
has exactly one owner and a defect or a version skew in an installed harness
integration degrades to rejected events with a warning instead of a corrupted
record that the report cannot read.

### Consequences

- Good, because every harness integration stays a thin event buffer: it needs no
  knowledge of the file format, the schema, or the scope rules.
- Good, because validation, the bounded-label rule, and the scope resolution
  live in one place, so the record's privacy boundary is auditable.
- Good, because the same boundary now carries three reporters — omp, opencode,
  and Claude Code — with no per-harness format work: each reporter is one
  generated asset, and the harnesses' material differences (in-process buffering
  versus one process per hook, timestamps present or absent on their events)
  stay behind the same intake.
- Bad, because one short-lived process per flush is added to the agent's path
  (bounded by batching at turn end and session end), and a stale installed
  reporter is a real compatibility surface that must be detected rather than
  assumed current — once per harness, so three surfaces to watch (R-84, R-88).
- Bad, because a record that cannot be written is not repaired; the change
  accepts gaps with a warning instead of failing the work it was recording
  (D-22).

### Confirmation

The `agent-activity-reporting` capability's `@e2e` scenario installs the
generated reporter, drives it through `node` against the real intake command
with a faked harness context, and asserts that the reported tool call, subagent,
and question wait are readable through `givn history --change <id> --json`; the
same capability's rejection scenarios assert that an event outside the contract
is named and the stored history is unchanged. The reporter-versus-embedded-asset
equality pin used by the existing attribution E2E is extended to the new asset.

## Pros and Cons of the Options

### A Givn-owned intake interface

- Good, because the format has one owner and one validator.
- Good, because a second harness is added without touching the record.
- Neutral, because the reporter must hold a small buffer and batch its flushes.
- Bad, because it introduces a process per flush and a version relationship
  between the installed reporter and the installed binary.

### Each integration appends record lines directly

- Good, because it needs no process per flush and is the cheapest to run.
- Bad, because the line format is re-implemented per harness and a defect there
  corrupts the store silently.
- Bad, because validation, labelling, and scope rules would be duplicated where
  they cannot be reviewed together.

### Read the harness's private session log at report time

- Good, because nothing is written while working.
- Bad, because it couples the record's content to another product's private
  on-disk format, so one harness upgrade breaks every existing report at once.
- Bad, because a harness that keeps no such log contributes nothing, which
  contradicts the goal of one report across harnesses.

## More Information

- Change: `givn/changes/development-timeline/` — proposal D3, design "Record
  Model And Intake Contract" and "Capture Architecture".
- Related records: ADR-014 (filesystem-derived state, no state file),
  ADR-013 (generated agent integrations), ADR-028 (hooks and mandatory checks —
  the record is explicitly not a gate), and ADR-070/ADR-072 (the archive receipt
  and its witness-bounded publication boundary, whose scope durations the record
  consumes as step entries — the record never writes, replaces, or re-validates
  the receipt).
- External contract: the omp extension surface (`pre` hook factories under
  `.omp/hooks/pre/`, tool, turn, subagent, and message events) was read from the
  installed package's shipped types on 2026-10-16; drift is tracked as R-84.
