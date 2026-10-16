# Tasks: development-timeline

Scenario-by-scenario RED→GREEN→REFACTOR→COMMIT loop. One section per executable
scenario: the 34 non-`@e2e` scenario sections first, in `.feature` order (the one
`@givn.modified` entry is reconciled in S2b, not as a scenario task), then the
E2E setup task, then the 8 `@e2e` scenarios in the same order.

Legend: an unchecked box means pending; a box is checked only when the pasted
evidence and the revision identifier are filled in. Check each box immediately
after completing the step — tasks.md must always reflect true state; never
batch-check boxes at the end. An unchecked box means that scenario is not done.

**Release note (the shared message subject of every revision in this change,
copied verbatim from `proposal.md` §Release note):**

> Givn records the development process of every change and ideation — each step
> it runs, the activity reported by every supported coding-agent harness,
> question waits, and token usage — and renders it as an interactive timeline
> report that opens in a browser.

**Delta inventory (verified mechanically against every delta `.feature` under
`givn/changes/development-timeline/specs/`):** 43 executable scenario entries in
4 features = 42 `@givn.added`, 1 `@givn.modified` (the S2b corpus-pin
reconciliation), 0 `@givn.removed`; 35 non-`@e2e` + 8 `@e2e`. A `Scenario
Outline` counts as one entry (its Examples rows are input variation, not
additional scenarios). Every still-pending entry carries `@wip` today and stays
`@wip` until its own RED step removes the tag from that one scenario; the one
`@givn.modified` entry carries no `@wip` and is reconciled green in S2b.

| Feature (path relative to `GIVN_FEATURES`) | Entries | Non-`@e2e` | `@e2e` |
|---|---|---|---|
| `inspect-development-history/development-record.feature` | 14 | 12 | 2 |
| `inspect-development-history/agent-activity-reporting.feature` | 14 | 13 | 1 |
| `inspect-development-history/development-report.feature` | 14 | 9 | 5 |
| `fragments/adr-qualification-gate.feature` (`@givn.modified`) | 1 | 1 | 0 |
| **Total** | **43** | **35** | **8** |

**Work order (recorded choice).** `design.md` names no implementation sequencing
section, so this file follows `.feature` order — the features in the inventory
order above, scenarios in their in-file order — in three phases: Tasks 1–34
(the 35 non-`@e2e` scenarios, numbered Tasks 1–33 plus **Task 11a** inserted
after Task 11 so the later task numbers stay stable), Task E1 (E2E setup), Tasks
E2–E9 (the `@e2e` scenarios, same feature order). The one `@givn.modified` entry
is not a scenario task: ADR-073 raises the register count that the permanent
`adr-qualification-gate` scenario pins, so setup item **S2b** carries the
`@givn.modified` record and the permanent patch, before any scenario work.

## Preconditions verified before writing this file (2026-10-16)

- `givn status --change development-timeline --json` (read before writing this
  file): `proposal` done, `specs` done, `design` done, `arc42-docs` done,
  `design-review` done, `tasks` pending — this file is that artifact's
  deliverable. The tasks precondition holds (design-review is `done`), and no
  `design.md` edit un-gated it.
- **Interaction-coverage pre-flight (the tasks instruction's mandatory check).**
  The three sources were cross-checked row by row: the owning use case's
  `specs/inspect-development-history/usecase.md` §Interactions (the durable pool
  of interaction facts — no `.feature` file carries an inventory comment), the
  `design.md` §Interaction Coverage Matrix, and the delta `@e2e` scenario
  titles. They agree 1:1: **8 rows each**, no inventory row without a matrix
  row, no matrix row without an existing `@e2e` scenario, and no `@e2e` scenario
  without an inventory row. This file therefore schedules exactly 8 `@e2e`
  sections and invents none.

| # | `usecase.md` §Interactions — capability: consumer action | `design.md` matrix — real interface / driving mechanism | `@e2e` title (`.feature`) |
|---|---|---|---|
| 1 | development-record: run a lifecycle command | CLI / `common::run_givn`: `givn status --change demo`, then `givn history --change demo --json` | `Consumer runs a lifecycle step and reads its recorded actions` (`development-record.feature`) |
| 2 | development-record: read the development record | CLI / `givn history --change demo --json` against a seeded record | `Consumer reads the development record as machine-readable output` (`development-record.feature`) |
| 3 | agent-activity-reporting: report agent activity from a supported harness | CLI (harness integration) / one `@e2e` outline, one Examples row per harness, driving each generated reporter through its real entry point | `Consumer installs the agent reporter and its reported activity lands in the record` (`agent-activity-reporting.feature`) |
| 4 | development-report: generate the development report for a change | CLI / `givn timeline --change demo --out report.html`, assertion reads the written file | `Consumer generates the development report for a change` (`development-report.feature`) |
| 5 | development-report: open the report in a browser | Web UI / Playwright `page.goto("file://<root>/report.html")`, one outline with a `change` and a `topic` row | `Consumer opens the generated development report in a browser` (`development-report.feature`) |
| 6 | development-report: open the report of a scope with no recorded activity | Web UI / Playwright: empty-state heading, no axis in the DOM | `Consumer opens the report of a scope with no recorded activity` (`development-report.feature`) |
| 7 | development-report: select a timeline element | Web UI / Playwright: click the longest step's span (or focus + `Enter`), read the details panel | `Consumer selects a timeline element and reads its details` (`development-report.feature`) |
| 8 | development-report: zoom the timeline | Web UI / Playwright: click the zoom-in control, compare the rendered range label before/after | `Consumer zooms the timeline to a shorter time range` (`development-report.feature`) |

- **Guarantee/rule coverage.** Every guarantee, rule, and extension of the
  owning use case is carried by at least one scheduled scenario (see the Domain
  constraints table below); nothing is left to prose. The one deliberate
  inventory-classification note: pan is an implementation detail of the zoomable
  axis (design-review Q9), so it adds no inventory row, no matrix row, and no
  `@e2e` scenario — the zoom scenario is the axis-state evidence.
- **Design system present.** `docs/design/design-system.md` (the change's
  declared design system, referenced by `design.md`'s Visual Design Contract
  machine-read lines: `Interface: web`, `Viewports: mobile (360x780), desktop
  (1920x1080)`, `States: empty`) is authored but untracked; Task P0 commits it.

## Record and report contract (copied from `design.md`; no other source is needed)

**Store.** One file per project: `givn/history/events.jsonl` — one JSON object
per line, opened append-only, never rewritten, never reordered. Best-effort by
construction: a write that cannot happen warns and continues, and never changes
the outcome of the work it records.

**Entry fields** (serde-serialized `HistoryEntry`): `schema` (integer schema
version; a reader skips a line whose schema it does not know), `id` (integer,
unique per project, minted **at append time under the store's exclusive append
lock** from the store's current last line), `at` (RFC 3339 UTC end timestamp,
millisecond precision), `scope`
(`{"kind": "change"|"ideation"|"project", "id": "<change-id>|<topic-id>|<project-name>"}`),
`source` (`givn` = Givn recorded it, `agent` = reported through the intake),
`kind`, `name`, `label` (optional, the bounded subject label — ≤ 120 characters,
truncation marked with `…`), `duration_ms` (optional; absent for a point entry),
`outcome` (`ok`|`failed`|`skipped`|`unknown`), `parent` (optional id of the
enclosing entry; a parent is only ever an id minted by the same process, so a
parent link always names an existing entry), `data` (only where the report needs
a non-generic value, e.g. `exit_code` on a step; `kind: "usage"` entries carry
`{input, output, cache_read, cache_write, total_tokens, model, subagent}`).

**Kinds:** `command`, `phase`, `hook`, `step`, `revision`, `session`, `turn`,
`tool`, `subagent`, `question`, `usage`.

**Scope resolution (`resolve_scope`)** binds every entry to exactly one scope
from the project's own state, no marker file: exactly one active change (the
rule `givn instructions <artifact>` already applies) → exactly one *unfinished*
ideation topic (`givn/ideation/<topic>/session.md` exists and carries no
`## Completion` section) → otherwise the project. Several candidates fall
through (a recorder never guesses). The intake payload's `change` and `topic`
are optional overrides: a present value wins over the resolution, the change
wins over the topic, and a named scope that does not exist is a per-event
rejection — never a silent fallback.

**Capture points (self-instrumentation):** `src/main.rs` command dispatch → one
`command` entry per `givn` invocation; `src/subprocess.rs::sh_streamed` (the
single shell-execution choke point) → one `step` entry per shell command with
its existing label and its measured duration; `src/hooks.rs::run_artifact_hooks`
/ `run_archive_gate_hooks` → one `hook` entry per hook, parented to the command
entry; `src/cli/archive.rs::run` → one `phase` entry per gate phase (the ten
numbered phases) parented to the archive command; `src/cli/archive.rs::run_scope`
→ the scope `duration_ms` that already exists becomes the step's duration;
`src/cli/commit.rs::commit` → one `revision` entry per created revision
(backend, revision id); `src/cli/check.rs::run` → the mandatory-check boundary
as `phase` entries. Durations are measured with `std::time::Instant` around the
step itself (`HistoryRecorder::span`), never inferred from two entries.

**Deliberate exclusion:** `givn history` and `givn timeline` run **without** a
recorder — reading the record never mutates it. Every other command, including
`givn history ingest`, records normally.

**Intake (`givn history ingest`):** reads one JSON object from stdin
(`schema`, `reporter`, `session`, `change`, `topic`, `events`). An event whose
`kind` is unknown, whose `at` is unparsable, or that lacks a required field for
its kind is rejected **individually**: it is named on stderr and the remaining
events are appended. Exit codes: `0` when the payload was read (rejected events
are warnings), `2` when stdin is not readable JSON at all; a record that cannot
be written warns and still exits `0`.

**Process-per-hook pending state** (Claude Code): one file per session at
`givn/history/pending/<harness>-<session-id>.json`, shaped
`{<tool_use_id>: {started_at, kind, label}}` — written atomically before the tool
runs, consumed by the matching end event, pruned by age (older than 24 h) and at
session end, no payload content. Losing it costs a duration, never an entry.

**Ignore block (D1, corrected):** keeping the store out of repository history is
split by who owns the destination. **The recorder never writes a versioned
file**; the record path reaches the versioned destinations only through the
commands that own them (`givn init`, `givn skills sync`, `givn upgrade` — the
existing `write_ignores` path), while the recorder asserts the never-dirty
destinations on every write.

| State of the project | Destination | Writer |
|---|---|---|
| Git worktree | the `# givn:begin:history` block in `.git/info/exclude` (resolved with `git rev-parse --git-path info/exclude`, the existing `upsert_local_exclude`), **re-asserted idempotently on every record write** | the recorder |
| No repository yet (`givn init` in a bare directory) | the `# givn:begin:history` block in `.gitignore` (scaffold-time safety net; nothing is tracked) | the recorder at its first write |
| Fossil checkout, `.fossil-settings/ignore-glob` absent | the repository-level `ignore-glob` setting (`fossil settings --value ignore-glob` … `fossil settings ignore-glob <value>`, the existing `upsert_fossil_local_ignore`), never-tracked and effective while no file overrides it | the recorder |
| Fossil checkout, `.fossil-settings/ignore-glob` present | `givn/history/*` in that file's **managed entry list** — a committed file overrides the repository setting, so writing the setting alone would leave the store an `EXTRA` | `givn init` / `givn skills sync` / `givn upgrade` through `write_ignores` |

The Fossil entry form is **`givn/history/*`**, not `givn/history` (a Fossil glob
must match the whole path and `*` crosses `/`, so a bare directory path matches
nothing). The Fossil destinations are a deduplicated entry list without fence
markers; only the Git destinations are fenced `# givn:begin:history` blocks.
Repository existence is decided by the backend markers at the project root, the
same markers `scm::select` reads — never by `scm::select` succeeding, because
select also bails for a nested root, both markers, or a missing executable, and
a select-keyed branch would write a versioned file while a worktree exists.
Realized in `src/history.rs::store_path` and `src/history.rs::ensure_ignored`.

**Report (`givn timeline`):** `givn timeline [--change <id>|--topic <id>] [--out
<path>]`. With `--out` omitted the report goes to the default path
`givn/history/reports/<scope>.html` (chapter 07's project footprint); the
command prints the written path and gains **no** `--open` flag and **no** `--json`
mode (the machine-readable view of the record is `givn history --json`). Flagless
`givn timeline` renders the **project report**, whose "outside a scope" band
carries every project-scope entry. Bands are the report's rows: one per archived
change, one per change or ideation topic in flight, and the "outside a scope"
band in the project report. Within every band spans are ordered by `at`, never
by file position. Phase derivation: one `phase` entry is recorded when an
artifact file of the change appears, and an entry belongs to the stage of the
latest such transition at or before its own `at`; entries before the band's
first transition stay in the opening stage, and a scope with no transitions gets
one unnamed stage instead of an invented phase name. Aggregation: at the initial
zoom, entries shorter than the time-per-pixel collapse into one aggregate marker
per band and time bucket that names how many entries it covers and their
combined duration; a click on an aggregate zooms to its bucket; the summary and
the longest-step ranking always cover the full record. Rendering: the generator
fills the asset's single data slot (`/*__GIVN_REPORT_DATA__*/`) with the JSON
model; the inline script builds an SVG time axis, nests spans by `parent`, and
wires zoom (wheel, buttons), pan (drag), selection (click/`Enter`) and the
details panel — it never calls `fetch`, never loads a remote resource, and never
uses `Date.now()`. Summary: total elapsed, ranked longest steps, entry count,
question-wait total and count, subagent count and combined duration, token
totals overall / per model / per subagent. Accessibility: a labelled `svg`
inside `main`, every span focusable with an accessible name, the details panel a
labelled region with a close control, `prefers-reduced-motion` disabling the
zoom transition and the panel animation.

## Global constraints (apply to every task)

- **Runner and strict mode.** `tests/features_runner.rs` (cucumber-rs 0.23) is
  the single runner for the permanent `givn/specs/` corpus and this change's
  delta; there is no parallel hand-maintained test file. Strict mode is
  `.fail_on_skipped()` on the Cucumber builder in **both** paths (the plain
  `cargo test` path and the nextest `--exact` path) plus the non-zero exit when
  `stats.failed > 0 || stats.skipped > 0`. Confirm both call sites before
  starting and paste them in Task S1; never weaken either. An undefined step
  therefore fails the runner — a scenario can never silently pass or skip.
- **Canonical single-scenario command** (design.md §Step Definitions; use it for
  every RED/GREEN/REFACTOR run — never `--name`, never an inference from
  whole-suite output), with this scenario's relative feature path and title:

  ```sh
  GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/<feature>.feature::<scenario title>"
  ```
  Scope commands: full delta `GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e`
  (35 scenarios, `verify.command` after Task S2 — the 34 non-`@e2e` scenario
  sections plus the `@givn.modified` entry) and
  `GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e`
  (8 scenarios, `verify.e2e_command`). Run a `Scenario Outline`'s unit once: it
  executes all Examples rows in one process.
- **Step-definition ownership — one file per capability** (`design.md` §Step
  Definitions). Unit/integration steps:
  `tests/steps/development_record_steps.rs`,
  `tests/steps/agent_activity_reporting_steps.rs`,
  `tests/steps/development_report_steps.rs`. Shared, not per capability:
  `tests/steps/development_timeline_support.rs` (record fixtures, the
  seeded-history builder, and the three harness drivers — a fake `pi` extension
  context for omp, a fake plugin context capturing `session.hook` /
  `tool.hook` / `shell.hook` plus the event subscription for opencode, and a
  payload-on-stdin invocation of the generated Claude shim with a fixture
  transcript). Every module is registered in `tests/steps/mod.rs` and
  `GivnWorld` gains one slot per capability. The `@e2e` step files
  (`tests/steps/development_record_e2e_steps.rs`,
  `tests/steps/agent_activity_reporting_e2e_steps.rs`,
  `tests/steps/development_report_e2e_steps.rs`) and `tests/common/browser.rs`
  are created in Task E1, never before — never one shared file for everything.
- **Step bodies are never empty.** `{}`, bare `pass`, bare `return` are
  indistinguishable from PASS and banned at every phase, not just RED. An
  unimplemented Rust step uses exactly
  `unimplemented!("TODO: development-timeline")`. A step body is exempt only
  once it has a real assertion (GREEN) or performs a real reused setup action;
  reuse is never an excuse for a new step to be empty.
- **RED discipline.** Remove `@wip` from **this scenario only** (every other
  scenario keeps it), run the canonical single-scenario command, paste the
  output, and it MUST exit non-zero — exit 0 means the runner is not strict or a
  step is empty: hard stop, fix before continuing. Record the RED category
  honestly: for a genuinely new behaviour the undefined step / `unimplemented!()`
  panic is the RED; where a scenario's contract already holds, prove
  discrimination with a controlled omission in an isolated fixture, restore the
  fixture immediately (`OLD_PRODUCT_RED` / `ORACLE_RED`, never a production
  switch, exception, fake success, or source-text test).
- **GREEN discipline.** Replace the stubs with real assertions and write the
  minimum production code. Compiled language: run the compile step first
  (`cargo build --locked`) and confirm it succeeds, then run the canonical
  single-scenario command and paste the output — MUST exit zero. List every
  production file created or modified; an empty list on a non-reuse scenario
  means investigate, not proceed. The scenario's own `@wip` tag stays removed
  from here on.
- **REFACTOR discipline.** Clean up with no behaviour change, re-run the
  canonical single-scenario command, paste the output — MUST still exit zero.
  `cargo fmt --all -- --check` and `cargo clippy --locked --all-targets -- -D
  warnings` are the repository's CI gates: keep them green from the scenario
  that introduces the code.
- **Commit discipline (resident rule).** One atomic commit for
  RED+GREEN+REFACTOR per scenario via `givn commit -m "…"`:
  `feat(<capability>): <release note>` for a regular scenario,
  `test(e2e): <release note>` for an `@e2e` scenario — the release note above is
  the message subject verbatim, and the scenario title goes in the message body.
  Record the revision identifier immediately, never reuse one, and check the
  scenario's box only once evidence and revision are filled in. No commit, or a
  commit touching only the spec or a stub, means the task is not done.
- **Production edit set** (`design.md` §Architecture Impact — the complete
  change, nothing else): new `src/history.rs`, `src/cli/history.rs`,
  `src/cli/timeline.rs`, `assets/timeline/report.html`,
  `assets/agent-integrations/omp/givn-timeline.js`,
  `assets/agent-integrations/opencode/givn-timeline.js`,
  `assets/agent-integrations/claude/givn-timeline.sh`,
  `docs/design/design-system.md`, `setup-e2e-browser.sh`, `tests/e2e/`
  (driver + `package.json`); modified `src/main.rs` (two new commands plus the
  recording wrapper around dispatch), `src/subprocess.rs`, `src/hooks.rs`,
  `src/cli/archive.rs`, `src/cli/check.rs`, `src/cli/commit.rs`,
  `src/skills/targets.rs`, `src/skills/rules.rs`, `tests/` (six step files, one
  support file, `tests/common/browser.rs`), `givn/commands.yaml`,
  `.github/workflows/ci.yml`, `.github/workflows/release.yml`. No new Rust
  dependency, no second runner, no server, no port, no database.
- **Never silently deviate from `design.md`.** The exact commands, file layout,
  framework, and mechanisms above are design decisions, not suggestions. If one
  turns out wrong, STOP — do not work around it. Update `design.md` with the
  corrected decision (naming the exact mechanism/command/file), re-run
  `design-review` (and reassess the arc42 chapter table), and only then resume
  the task. Example: `verify.e2e_command` is never pointed at the same string as
  `verify.command`, and one shared step file is never substituted for the
  per-capability files `design.md` names.
- **`@e2e` tags are immutable.** Never remove an `@e2e` tag; the `verify-e2e`
  hook and `givn lint --change development-timeline` both detect tag removal by
  cross-referencing this file. If e2e infrastructure blocks a scenario, fix the
  infrastructure — never delete the tag.
- **Visual evidence (the visual gate is change-wide).** Every `@e2e` scenario
  commits final-state screenshots at both viewports under
  `givn/changes/development-timeline/evidence/visual/<scenario-slug>/`:
  `mobile.png` (360x780) and `desktop.png` (1920x1080) — including the CLI ones,
  which commit the report they produced, rendered in the browser. The declared
  `empty` state is captured by the empty-scope scenario as `mobile--empty.png`
  and `desktop--empty.png`. `scenario_slug` follows
  `givn::integrity_hook::scenario_slug`: lowercase ASCII, every run of
  non-alphanumeric characters becomes one `-`, leading and trailing `-` removed.
  Intermediate captures, if any, go to `reports/visual/` and are not review
  evidence. `src/integrity_hook.rs::check_visual_evidence` fails the gate for a
  missing or zero-byte file.
- **Local runnability and digital twins.** One-time browser setup, then the
  whole verification surface: `./setup-e2e-browser.sh` (installs the
  Playwright-pinned Chromium), `./run-tests.sh --non-e2e`,
  `./run-tests.sh --e2e`. No container, no listening port, no external service,
  and no digital twin exists or is needed: the report and the record are local
  files and the browser loads the report from a `file://` URL inside the
  temporary project root the scenario created. `./setup-e2e-browser.sh` is the
  only step that touches the network and it runs once, outside the suite.
- **Coverage.** The addon is enabled (`measure_command: ./measure-coverage.sh`,
  `merge_command: ./merge-coverages.sh`), so new code must be exercised in the
  pass that owns it (`coverage/non-e2e-cobertura.xml`, `coverage/e2e-cobertura.xml`,
  merged into `coverage/cobertura-coverage.xml`). The new scenarios keep the
  existing tag split; no coverage-script change is required. Non-zero production
  probes named by `design.md`: `src/history.rs::bound_label` and
  `src/cli/timeline.rs::build_report_model` (Tasks 8 and 25), plus
  `src/history.rs::record` (Task 1), `src/cli/history.rs::ingest` (Tasks 12–20)
  and `src/cli/history.rs::map_harness_event` (Tasks 23–24).
- **Personas are review lenses, never Gherkin actors.** The scenarios' actors
  (`the user`, `the agent`, `the consumer`, `the developer`, `the harness`) stay
  distinct from `developer--primary-user` and `agent--technical-role`.

## Domain constraints (from the owning use case)

`specs/inspect-development-history/usecase.md` — goal: "See how a change or an
ideation was developed — which steps ran, what the agent did, where the time
went — without reading raw logs." Actors: Developer; supporting Authoring agent
(whichever harness runs it). Every scenario task below carries the row(s) that
name it.

| Task group | Use-case guarantee or rule | Scenario tasks | Persona impact |
|---|---|---|---|
| `development-record` recording | Recording never changes the outcome of the command it records (minimal guarantee) | Tasks 3, 18 | `agent--technical-role` never fails a command because the record failed. |
| `development-record` recording | A recorded duration is measured, never inferred from two entries that disagree | Tasks 1, 2, 23, 24 | `developer--primary-user` can trust the durations the report ranks. |
| `development-record` recording | The record is append-only: a recorded entry is never rewritten | Task 4 | `developer--primary-user` keeps history across restarts ("A record written before a restart is still in the report afterwards"). |
| `development-record` scope | Every recorded entry belongs to exactly one scope: a change, an ideation topic, or the project | Tasks 1, 5, 7, E2 | `developer--primary-user` sees one scope's history, not a mixture. |
| `development-record` privacy | The record keeps metadata plus one bounded label; the complete command text is never stored (D2) | Task 8 | `developer--primary-user`'s credentials and private code stay out of the store. |
| `development-record` observation | Recording is the observation substrate: no gate, no proof, no exit code, no artifact lifecycle changes | Tasks 3, 6, 9, 21, 22 | `agent--technical-role` executes the same workflow; nothing it observes mutates. |
| `development-record` reading | Reading the record never extends it (`givn history`, `givn timeline`) | Task 9 | `developer--primary-user` can re-read a report as often as wanted. |
| `development-record` hygiene | One local project store, kept out of repository history through both backends' ignore destinations (D1) | Tasks 10, 11 | `developer--primary-user`'s telemetry never enters repository history; a Fossil project is covered too. |
| `agent-activity-reporting` intake | Extension: a reported agent event that does not match the intake contract is rejected with a warning and the record is not corrupted; the previously recorded history is unchanged | Tasks 17, 20 | `agent--technical-role`'s stale reporter cannot damage the record. |
| `agent-activity-reporting` intake | Extension: the record cannot be written → the reported agent action continues and reports its own outcome | Task 18 | `agent--technical-role`'s tool call is never blocked by Givn's disk. |
| `agent-activity-reporting` intake | Every reported entry is scoped to the change (payload override wins; a named scope that does not exist is a per-event rejection) | Tasks 12–16 | `developer--primary-user` sees agent work under the right change. |
| `agent-activity-reporting` intake | The intake is a machine contract: stdin JSON, stable exit codes (`0` read, `2` unparsable) | Tasks 19, 20 | `agent--technical-role` reacts to a deterministic exit code. |
| `agent-activity-reporting` harnesses | Every supported harness reports its own activity through the same intake; the `agents` skills mirror receives no reporter | Tasks 21, 22, E4 | `agent--technical-role` reports from opencode, Claude Code, or omp identically; no invented activity for a mirror. |
| `agent-activity-reporting` gap honesty | A signal a harness cannot deliver (a wait it does not time, token usage in its own session record) is recorded as measured/unavailable rather than zero, and the gap is stated | Tasks 16, 23, 24, 27, E4 | `developer--primary-user` never reads an invented value as a measurement. |
| `development-report` goal | Success guarantee: the developer can see what was worked on and how long it took for the requested scope (ranked longest steps, question waits, subagent fan-out, token totals) | Tasks 25–30, E5–E9 | `developer--primary-user` answers "where did the time go?" without raw logs. |
| `development-report` scope | An ideation topic is reportable before any change exists; the project report renders the "outside a scope" band (D5, design-review Q2/Q4) | Tasks 7, 28, E6 | `developer--primary-user` sees ideation work and work between changes. |
| `development-report` ordering | Report ordering follows `at`, never store position (two writers make them differ) | Tasks 6, 29 | `developer--primary-user` reads a truthful sequence. |
| `development-report` empty state | Extension: no activity recorded → the report renders an explicit empty state instead of a blank page or an error; no timeline axis | Tasks 32, E9 | `developer--primary-user` gets a truthful, non-alarming screen. |
| `development-report` rendering | The report is one self-contained file: no server, no port, no network access, no third-party JavaScript, deterministic pixels | Tasks 31, E4–E9 | `developer--primary-user` opens a local file and can hand it to someone else. |
| `development-report` rendering | A dense record is reported as aggregates instead of raw ticks, while the summary still counts every entry | Task 33 | `developer--primary-user` reads a long run without an unreadable axis. |

## Strict-mode proof and setup (do first)

- [x] **P0 (planning, before the setup tasks). Commit the planning artifacts,
      separate from any scenario work.**
      The whole change directory is untracked today
      (`givn/changes/development-timeline/**`, `docs/design/design-system.md`,
      the arc42 chapter updates and `docs/arc42/adr/ADR-073-development-record-ownership.md`).
      Commit exactly those plan/design files — `proposal.md`, `specs/**`,
      `design.md`, `design-review.md`, `arc42.md`, `docs/design/design-system.md`,
      the arc42 chapters + ADR-073, and this `tasks.md` — in one commit whose
      subject names the change plan (`docs(development-timeline): …`), so the
      first scenario commit cannot sweep the plan in and no scenario commit
      touches only the spec. Evidence:
      ```
      $ git status --porcelain          # before
       M docs/arc42/01-introduction-and-goals.md
       M docs/arc42/02-architecture-constraints.md
       M docs/arc42/03-context-and-scope.md
       M docs/arc42/04-solution-strategy.md
       M docs/arc42/05-building-block-view.md
       M docs/arc42/06-runtime-view.md
       M docs/arc42/07-deployment-view.md
       M docs/arc42/08-crosscutting-concepts.md
       M docs/arc42/09-architecture-decisions.md
       M docs/arc42/10-quality-requirements.md
       M docs/arc42/11-risks-and-technical-debt.md
       M docs/arc42/12-glossary.md
       M docs/arc42/README.md
       M docs/arc42/adr/README.md
      ?? docs/arc42/adr/ADR-073-development-record-ownership.md
      ?? docs/design/
      ?? givn/changes/

      $ git add givn/changes/development-timeline docs/design/design-system.md docs/arc42
      $ git commit -m "docs(development-timeline): plan the development record and timeline report"
      [main 8ceaaba6] docs(development-timeline): plan the development record and timeline report
       25 files changed, 5570 insertions(+), 6 deletions(-)
       create mode 100644 docs/arc42/adr/ADR-073-development-record-ownership.md
       create mode 100644 docs/design/design-system.md
       create mode 100644 givn/changes/development-timeline/arc42.md
       create mode 100644 givn/changes/development-timeline/design-review.md
       create mode 100644 givn/changes/development-timeline/design.md
       create mode 100644 givn/changes/development-timeline/proposal.md
       create mode 100644 givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature
       create mode 100644 givn/changes/development-timeline/specs/inspect-development-history/development-record.feature
       create mode 100644 givn/changes/development-timeline/specs/inspect-development-history/development-report.feature
       create mode 100644 givn/changes/development-timeline/specs/inspect-development-history/usecase.md
       create mode 100644 givn/changes/development-timeline/tasks.md

      $ git status --porcelain          # after
      (empty)
      $ git rev-parse HEAD
      8ceaaba6cc754a4a0742fa05720bd5c48adec030
      ```
- [x] **S1. Prove strict mode on this change's runner (mandatory, non-zero exit
      required).**
      - Runner config: none to add — `tests/features_runner.rs` already carries
        `.fail_on_skipped()` on the Cucumber builder in both entry paths (the
        plain `cargo test` path and the nextest `--exact` path) and exits
        non-zero when `stats.failed > 0 || stats.skipped > 0`. Paste both call
        sites and the exit condition; never weaken them.
      - **Probe A — an undefined step fails.** Create the throwaway probe
        `givn/changes/development-timeline/probe/probe.feature` (outside
        `specs/`, so nothing in the delta corpus, lint, or counts changes) with
        one scenario carrying one step that has no binding, then run it scoped:
        `GIVN_FEATURES=givn/changes/development-timeline/probe cargo test --locked --test features_runner`
        → expect a non-zero exit (the unmatched step is skipped, and
        `.fail_on_skipped()` turns that into a failure).
      - **Probe B — an `unimplemented!()` stub fails.** Temporarily bind the
        probe's step in `tests/steps/development_record_steps.rs` with exactly
        `unimplemented!("TODO: development-timeline")`, re-run the scoped probe
        command → expect a non-zero exit (the panic is a failed scenario, never
        a silent pass).
      - **Restore (back to a passing baseline, real scenarios still `@wip`).**
        Delete `givn/changes/development-timeline/probe/` and the probe binding,
        then confirm: the delta listing is unchanged —
        `GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner`
        → 41 units, `GIVN_CUCUMBER_TAGS='not @e2e'` → 33, `GIVN_CUCUMBER_TAGS='@e2e'`
        → 8 — with no probe title anywhere; every delta scenario still carries
        `@wip` (41 tag lines); and the permanent scope still exits 0 —
        `./run-tests.sh --non-e2e` (the same command Task S2 then declares as
        `verify.command`). Paste every command with its output.
      ```
      Strict-mode call sites (tests/features_runner.rs, both entry paths, never weakened):
        plain `cargo test` path, line 54:  GivnWorld::cucumber()
                                             .fail_on_skipped()          # line 55
                                             .filter_run(features_path, ...)
        exit condition, line 64:           if stats.failed > 0 || stats.skipped > 0 { std::process::exit(1); }
        nextest `--exact` path, line 149:  GivnWorld::cucumber()
                                             .with_default_cli()
                                             .fail_on_skipped()          # line 151
                                             .filter_run_and_exit(features_path(), ...)

      Probe A — an undefined step fails:
      $ GIVN_FEATURES=givn/changes/development-timeline/probe cargo test --locked --test features_runner
      Feature: Strict-mode probe
        Scenario: An undefined step fails the runner
         ✘  Given a strict-mode probe step that has no binding
            Step failed:
            Defined: givn/changes/development-timeline/probe/probe.feature:5:5
            Step doesn't match any function
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `--test features_runner`
      EXIT 1

      Probe B — an `unimplemented!()` stub fails (binding added to
      tests/steps/development_record_steps.rs as `fn strict_mode_probe_step(...) {
      unimplemented!("TODO: development-timeline"); }`):
      $ GIVN_FEATURES=givn/changes/development-timeline/probe cargo test --locked --test features_runner
      Feature: Strict-mode probe
        Scenario: An undefined step fails the runner
         ✘  Given a strict-mode probe step that has no binding
            Step failed:
            Defined: givn/changes/development-timeline/probe/probe.feature:5:5
            Matched: tests/steps/development_record_steps.rs:6:1
            Step panicked. Captured output: not implemented: TODO: development-timeline
      [Summary]
      1 feature
      1 scenario (1 failed)
      1 step (1 failed)
      error: test failed, to rerun pass `--test features_runner`
      EXIT 1

      Restore (probe directory + probe binding deleted; identical to the pre-S1 state):
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner
         Compiling givn v0.9.0 (/home/buster/projects/givn)
          Finished `test` profile [unoptimized + debuginfo] target(s) in 5.82s
      givn::features_runner <unit>          # 43 non-empty lines = 2 cargo lines + 41 scenario units
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='not @e2e' cargo nextest list --locked --test features_runner
          Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
      givn::features_runner <unit>          # 34 non-empty lines = 1 cargo line + 33 scenario units
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='@e2e' cargo nextest list --locked --test features_runner
          Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
      givn::features_runner <unit>          # 9 non-empty lines = 1 cargo line + 8 scenario units
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner | grep -c 'Strict-mode probe'
      0
      $ grep -rho '@wip' givn/changes/development-timeline/specs | wc -l
      41
      $ find givn/changes/development-timeline -type f
      givn/changes/development-timeline/{arc42,design-review,design,proposal,tasks}.md
      givn/changes/development-timeline/specs/inspect-development-history/{agent-activity-reporting,development-record,development-report}.feature
      givn/changes/development-timeline/specs/inspect-development-history/usecase.md
      (the probe/ directory is gone; delta units 41 = 41 `@givn.added` at this point)

      Baseline note: `./run-tests.sh --non-e2e` (permanent corpus) was still red at this point for a
      reason outside the delta — the P0 ADR-073 register row lifts the ADR register to 25 accepted
      records while `givn/specs/fragments/adr-qualification-gate.feature` pinned 24 (assertion
      `left: 25 right: 24`). The delta counts above are therefore the pre-reconciliation numbers;
      S2b carries the `@givn.modified` record, the re-run listings (42 / 34 / 8) and the exit-0 baseline.
      ```
      If Probe A or B exits 0, stop and fix strictness before any scenario work.
- [x] **S2. Configure `verify.command` and document it as implementation work.**
      - `givn/commands.yaml`: change `verify.command` from `./run-tests.sh` to
        `./run-tests.sh --non-e2e`; leave `verify.e2e_command` as
        `./run-tests.sh --e2e` (design.md §Local Runnability & Digital Twins).
        The regular scope runs exactly the `not @e2e` tag set, so the regular
        archive receipt no longer depends on a browser, and the two receipts
        match the tags they claim.
      - Document the same command in the project's command table — the
        development-script list that names `run-tests.sh`,
        `measure-coverage.sh`, and `merge-coverages.sh`, which in this
        repository is `docs/reference.md` §Repository development scripts
        (`README.md`'s documentation index points at it as the CLI/dev-script
        reference; `design.md` calls this surface "the README's command table").
        The table must show the regular scope as `./run-tests.sh --non-e2e`
        beside `./run-tests.sh --e2e`.
      - `./setup-e2e-browser.sh` does not exist yet; do **not** invent it here —
        Task E1 owns it. The baseline run below must therefore be the
        non-browser scope only.
      - Paste `git diff givn/commands.yaml` plus the doc change and the exit-0
        baseline: `./run-tests.sh --non-e2e` (permanent corpus, no
        `GIVN_FEATURES` override).
      ```
      $ git diff givn/commands.yaml
      diff --git a/givn/commands.yaml b/givn/commands.yaml
      index 781b8284..f4d732b9 100644
      --- a/givn/commands.yaml
      +++ b/givn/commands.yaml
      @@ -1,6 +1,6 @@
       # Project-owned command scripts. The README documents these same commands.
       verify:
      -  command: "./run-tests.sh"
      +  command: "./run-tests.sh --non-e2e"
         e2e_command: "./run-tests.sh --e2e"
       
       coverage:

      $ git diff docs/reference.md
      diff --git a/docs/reference.md b/docs/reference.md
      index c031037b..223dc9de 100644
      --- a/docs/reference.md
      +++ b/docs/reference.md
      @@ -169,8 +169,8 @@ Project command scripts live at the repo root and are referenced by
       This repository's own verify commands:
       
       ```sh
      -./run-tests.sh            # cargo test matrix across feature combinations
      -./run-tests.sh --e2e      # @e2e-tagged subsets per feature combination
      +./run-tests.sh --non-e2e  # the regular scope (verify.command): every non-@e2e scenario
      +./run-tests.sh --e2e      # the e2e scope (verify.e2e_command): the @e2e scenarios, browser included
       ./measure-coverage.sh     # writes non-E2E and E2E Cobertura reports
       ./merge-coverages.sh      # writes merged report used by README badges/blocks
       ```

      Exit-0 baseline: `./run-tests.sh --non-e2e` (permanent corpus, no GIVN_FEATURES override) →
      `Summary [  43.022s] 533 tests run: 533 passed, 94 skipped`, EXIT=0. Pasted in full under S2b,
      which is where it first became green: the P0 ADR-073 register row broke the pin until the S2b
      `@givn.modified` record and the permanent patch landed (see S2b's RED).
      ```
- [x] **S2b. Reconcile the permanent ADR-register pin with ADR-073 (the delta's
      one `@givn.modified` record).**
      P0 added ADR-073 and its row in `docs/arc42/adr/README.md`, lifting the
      active register to 25 accepted records, while
      `givn/specs/fragments/adr-qualification-gate.feature` pinned 24 — so the
      permanent scope failed. The repository's own precedent is
      `retire-lint-command-adr` (f062c23f): a `@givn.modified` delta record next
      to a patch of the permanent fragment scenario, in the same change. The
      reconciliation is exactly that:
      - new `givn/changes/development-timeline/specs/fragments/adr-qualification-gate.feature`
        — `@givn.delta @adr-qualification-gate`, the Feature title copied
        verbatim, and one `@givn.modified` scenario titled exactly `The
        migration preserves independent architecture history across three
        proposed decisions` copied verbatim from the permanent file with exactly
        one change: `24` → `25`;
      - `givn/specs/fragments/adr-qualification-gate.feature` — the same single
        line (24 → 25);
      - `proposal.md` §Capability Routing gains the `adr-qualification-gate` row
        (`EXTEND adr-qualification-gate`) so the table matches the delta.
      The three edits ride in the setup commit with S2–S4; no spec-only commit
      exists. Evidence:
      ```
      RED — the permanent scope failed before the reconciliation (P0 only):
      $ ./run-tests.sh --non-e2e
      ...
        FAIL [   0.803s] (121/533) givn::features_runner fragments/adr-qualification-gate.feature::The migration preserves independent architecture history across three proposed decisions
      ────────────
           Summary [  10.758s] 132/533 tests run: 131 passed, 1 failed, 94 skipped
      error: test run failed
      EXIT=100

      reproduced in isolation:
      $ cargo nextest run --no-fail-fast --locked --test features_runner -- 'fragments/adr-qualification-gate.feature::The migration preserves independent architecture history across three proposed decisions'
         ✘  Then the final register has 24 accepted decisions and no proposed decisions
            Step panicked. Captured output: assertion `left == right` failed: unexpected accepted ADR count
              left: 25
             right: 24

      GREEN — the delta-scope targeted run of the modified record:
      $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "fragments/adr-qualification-gate.feature::The migration preserves independent architecture history across three proposed decisions"
      Feature: Strict qualification and canonical routing of architecture decisions
        Scenario: The migration preserves independent architecture history across three proposed decisions
         ✔  Given the remediation case carries the distinguishing marker "harness jib mandrel nozzle oboe yacht yarn zither"
         ✔  Given the consolidated ADR register and canonical sources are available
         ✔  When a maintainer compares the register with the canonical sources
         ✔  Then the final register has 25 accepted decisions and no proposed decisions
         ✔  And the change's migration manifest gives every assigned identifier exactly one disposition
         ✔  And reserved identifiers `ADR-050` and `ADR-051` have no record
         ✔  And non-ADR records are removed from the active register and ADR archive
         ✔  And absorbed records are absent and their refinements remain readable
         ✔  And superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
         ✔  And `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
         ✔  And the superseded semantic-engine records remain archived with replacement links to their replacement record
      [Summary]
      1 feature
      1 scenario (1 passed)
      11 steps (11 passed)
      EXIT 0

      GREEN — the S2 baseline (permanent corpus, no GIVN_FEATURES override):
      $ ./run-tests.sh --non-e2e
      ...
           Summary [  43.022s] 533 tests run: 533 passed, 94 skipped
      EXIT=0

      post-reconciliation delta listings:
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner | grep -c 'givn::features_runner'
      42
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='not @e2e' cargo nextest list --locked --test features_runner | grep -c 'givn::features_runner'
      34
      $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='@e2e' cargo nextest list --locked --test features_runner | grep -c 'givn::features_runner'
      8
      $ grep -rho '@wip' givn/changes/development-timeline/specs | wc -l
      41          # the 41 `@givn.added` entries; the 1 `@givn.modified` entry carries no `@wip`

      $ git diff givn/specs/fragments/adr-qualification-gate.feature
      -    Then  the final register has 24 accepted decisions and no proposed decisions
      +    Then  the final register has 25 accepted decisions and no proposed decisions

      $ cat givn/changes/development-timeline/specs/fragments/adr-qualification-gate.feature
      @givn.delta @adr-qualification-gate

      Feature: Strict qualification and canonical routing of architecture decisions

        @givn.modified
        Scenario: The migration preserves independent architecture history across three proposed decisions
          Given  the remediation case carries the distinguishing marker "harness jib mandrel nozzle oboe yacht yarn zither"
          Given  the consolidated ADR register and canonical sources are available
          When  a maintainer compares the register with the canonical sources
          Then  the final register has 25 accepted decisions and no proposed decisions
          And  the change's migration manifest gives every assigned identifier exactly one disposition
          And  reserved identifiers `ADR-050` and `ADR-051` have no record
          And  non-ADR records are removed from the active register and ADR archive
          And  absorbed records are absent and their refinements remain readable
          And  superseded architectural records `ADR-019`, `ADR-042`, and `ADR-053` remain archived with replacement links
          And  `ADR-043` and `ADR-049` are absent because their behavior is owned by permanent specifications
          And  the superseded semantic-engine records remain archived with replacement links to their replacement record

      $ givn lint --change development-timeline
      givn lint: 4 file(s) checked, 41 finding(s)                  # the 41 @wip entries; no finding kind for the @givn.modified record
      LINT_EXIT=2
      ```
- [x] **S3. Create the step-definition skeletons — one file per capability
      (design.md §Step Definitions).**
      - Create `tests/steps/development_record_steps.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`,
        `tests/steps/development_report_steps.rs` and the shared
        `tests/steps/development_timeline_support.rs` (record fixtures, the
        seeded-history builder, the three harness drivers). Skeleton = module
        doc comment, the `GivnWorld` state slot the capability needs, and the
        helper signatures the scenarios will call; step bindings are added
        scenario by scenario at RED.
      - Register all four in `tests/steps/mod.rs` and give `GivnWorld` one slot
        per capability (the existing pattern: a state type owned by the
        capability's module, held as an `Option<…State>` field).
      - Do **not** create the `@e2e` step files or `tests/common/browser.rs`
        here — Task E1 owns them.
      - Confirm the harness still builds and the permanent scope stays green:
        `cargo build --locked --tests` and `cargo clippy --locked --all-targets
        -- -D warnings`.
      ```
      Module registrations (tests/steps/mod.rs):
        line 15: pub mod agent_activity_reporting_steps;
        line 27: pub mod development_record_steps;
        line 28: pub mod development_report_steps;
        line 29: pub mod development_timeline_support;

      GivnWorld slots (tests/steps/mod.rs, the existing `Option<…State>` pattern, one per capability):
        pub(crate) development_record: Option<development_record_steps::DevelopmentRecordState>,
        pub(crate) agent_activity_reporting: Option<agent_activity_reporting_steps::AgentActivityReportingState>,
        pub(crate) development_report: Option<development_report_steps::DevelopmentReportState>,

      Skeleton shape: each capability file carries its module doc comment and its state type (derived
      `Debug + Default`, the field the capability's steps will exchange); development_timeline_support.rs
      carries the shared module doc naming the record fixtures, the seeded-history builder, and the three
      harness drivers, which arrive with the scenarios that call them (a helper with no caller would be
      the unused helper S4 forbids). No step bindings yet — they are added scenario by scenario at RED.
      No `@e2e` step files and no `tests/common/browser.rs` (Task E1 owns them).
      The GivnWorld slots and the three state types carry `#[allow(dead_code)]` with a comment naming S3
      as their origin: nothing reads a slot before the capability's first scenario does, and that
      scenario drops the allowance (otherwise `cargo clippy … -D warnings` is red on "field is never
      read", as observed before the allowances were added).

      $ cargo build --locked --tests
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s        EXIT 0
      $ cargo clippy --locked --all-targets -- -D warnings
          Checking givn v0.9.0 (/home/buster/projects/givn)
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.29s       EXIT 0
      $ cargo fmt --all -- --check
      (no output)                                                                     EXIT 0
      ```
- [x] **S4. Initialise the production module skeletons at the exact paths
      `design.md` names.**
      - `src/history.rs` — the record owner: `HistoryEntry` (the field set in
        *Record and report contract*), `Scope` (`Change(id)`, `Ideation(id)`,
        `Project(name)`), `HistoryStore` (append + read of
        `givn/history/events.jsonl`, id minting under the append lock),
        `HistoryRecorder` (`record`, `span`), `resolve_scope`, `bound_label`,
        `PendingState`, `store_path`, `ensure_ignored`.
      - `src/cli/history.rs` — `givn history` (read) + `givn history ingest`
        (the one intake) + `givn history harness-hook --harness claude` (the raw
        payload mapper and transcript reader); `IngestedEvent`, `HarnessPayload`,
        `map_harness_event`.
      - `src/cli/timeline.rs` — `ReportModel`, `build_report_model`,
        `render_report`.
      - `assets/timeline/report.html` — the self-contained asset: inline
        CSS/JS/SVG, exactly one data slot `/*__GIVN_REPORT_DATA__*/`, no
        third-party JavaScript, no CDN, no build step.
      - Declare the modules so the crate compiles: `mod history;` in
        `src/main.rs` and `pub mod history;` in `src/lib.rs`; `pub mod history;`
        and `pub mod timeline;` in `src/cli/mod.rs`. Command dispatch wiring
        (`src/main.rs` `Commands` variants) is added by the first scenario that
        needs each command — a skeleton must not be reachable from a command
        whose scenario has not been implemented.
      - Skeleton bodies are `todo!("TODO: development-timeline")` on the
        production side (`unimplemented!("TODO: development-timeline")` on the
        step side). Keep them minimal — module doc, signatures, no unused
        imports or helpers — so `cargo fmt --all -- --check`,
        `cargo build --locked` and `cargo clippy --locked --all-targets -- -D
        warnings` stay green (CI runs both). A skeleton may never survive the
        scenario that covers it.
      ```
      Module declarations:
        src/main.rs:12    mod history;                       (binary copy; #[allow(dead_code)] + comment —
                                                              not wired into dispatch until the command
                                                              scenarios need it, per this task's rule)
        src/lib.rs:14     pub mod history;
        src/cli/mod.rs:16 pub mod history;                   (#[allow(dead_code)] skeleton allowance)
        src/cli/mod.rs:33 pub mod timeline;                  (#[allow(dead_code)] skeleton allowance)

      src/history.rs (163 lines) — every body `todo!("TODO: development-timeline")`:
        16  pub struct HistoryEntry      { schema, id, at, scope, source, kind, name, label,
                                           duration_ms, outcome, parent, data }   (the contract's field set)
        50  pub enum Scope               { Change { id }, Ideation { id }, Project { id } }
        66  pub struct HistoryStore      + impl HistoryStore { read, append }
        91  pub struct HistoryRecorder   + impl HistoryRecorder { record, span }
        115 pub struct Span              (the Instant guard)
        121 pub fn resolve_scope(root: &Path) -> Scope
        127 pub fn bound_label(subject: &str) -> String
        132 pub fn store_path(root: &Path) -> PathBuf
        138 pub fn ensure_ignored(root: &Path) -> std::io::Result<()>
        149 pub struct PendingState      156 pub struct PendingStart { started_at, kind, label }

      src/cli/history.rs (44 lines):
        8   pub struct IngestedEvent     18  pub struct HarnessPayload
        24  pub fn run(change, topic, json)     30  pub fn ingest()
        36  pub fn harness_hook(harness)        42  pub fn map_harness_event(payload)

      src/cli/timeline.rs (25 lines):
        10  pub struct ReportModel       13  pub fn build_report_model()
        18  pub fn render_report(model)  23  pub fn run(change, topic, out)

      assets/timeline/report.html (35 lines): self-contained document — inline CSS, inline script, an
      SVG timeline inside `main`, the labelled details region, `prefers-reduced-motion` disabling
      transitions, and exactly one data slot at line 32: /*__GIVN_REPORT_DATA__*/ (no CDN, no
      third-party JavaScript, no build step).

      Allowance note: the S4 skeletons are declared but nothing reads them yet, so `cargo build` was red
      with "function is never used" / "field is never read" in both the lib and the bin crate until the
      allowances above were added (the lib warns for private fields of public structs, the bin for the
      whole private module copy). Each says which task drops it; the scenario that first reads an item
      removes its allowance.

      $ cargo build --locked
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.69s       EXIT 0
      $ cargo clippy --locked --all-targets -- -D warnings
          Checking givn v0.9.0 (/home/buster/projects/givn)
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.55s       EXIT 0
      $ cargo fmt --all -- --check        EXIT 0
      $ cargo build --locked --tests
          Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.91s       EXIT 0
      ```

## Task 1 — `A recorded step keeps a measured duration and its outcome` (`development-record`, added)

- [x] **Scenario: `A recorded step keeps a measured duration and its outcome`**
  - Design constraints:
    - Feature `inspect-development-history/development-record.feature`; steps in
      `tests/steps/development_record_steps.rs`; fixtures in
      `tests/steps/development_timeline_support.rs` (which uses the existing
      black-box seam `common::run_givn` — the compiled binary in a `tempfile`
      project root — never an in-process library call).
    - Capture point: `src/main.rs` command dispatch emits one `command` entry per
      `givn` invocation — `kind: "command"`, `name` the command name,
      `source: "givn"`, `scope` resolved by `resolve_scope` (given the change
      `demo` with `proposal.md`, that is exactly one active change → change
      scope `{"kind":"change","id":"demo"}`), `outcome` from the invocation
      (`ok` on exit 0), `at` an RFC 3339 UTC end timestamp with millisecond
      precision.
    - `src/history.rs::HistoryRecorder` is created once per process, holds the
      store path and the resolved scope, exposes `record(entry)` and
      `span(name)`, mints `id` at append time under the store's append lock, and
      never returns an error to its caller.
    - Duration is *measured* (`std::time::Instant` around the step itself via the
      `span` guard), never inferred from two entries or from wall-clock
      arithmetic; the architecture already forbids a fake clock.
    - Store: `givn/history/events.jsonl`, one JSON object per line, appended
      only.
    - Assertion shape: after the run, exactly one entry names this command; its
      `duration_ms` is present and numeric; its `outcome` is `ok`. "Measured"
      is asserted as a real non-fabricated number produced by the run, so the
      step reads the store the way a consumer would (documented helper in the
      support file), not a mocked recorder.
    - RED expected failure: both Then steps are undefined → skipped → the
      `.fail_on_skipped()` contract makes the runner exit non-zero.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded step keeps a measured duration and its outcome"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded step keeps a measured duration and its outcome"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A recorded step keeps a measured duration and its outcome
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✘  Then the development record holds one entry for that command
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:10:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (2006625) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step (the genuinely new behaviour). The two Then bindings did not exist, so
        the scenario failed instead of passing or skipping — the `.fail_on_skipped()` contract is the one
        S1's Probe A proved live on this same runner.
        ```
  - [x] **GREEN** — replace the stubs with the real assertions; implement the
        record owner and the dispatch wrapper. Files: `src/history.rs`,
        `src/main.rs`, `tests/steps/development_record_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Compile first
        (`cargo build --locked`), then re-run the same command → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.87s          EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded step keeps a measured duration and its outcome"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A recorded step keeps a measured duration and its outcome
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  Then the development record holds one entry for that command
           ✔  And that entry carries a measured duration and its outcome
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files created/modified by this scenario:
          src/history.rs       NEW content behind the S4 skeleton — HistoryStore (path/read/append, the
                               append minting ids from the store's last line), HistoryRecorder
                               (start/finish + the process-wide install/current cell), resolve_scope
                               (one active change → change scope, else the project), store_path,
                               rfc3339_ms + civil_from_days (the RFC 3339 UTC ms `at`)
          src/main.rs          the recording wrapper around dispatch: the clap subcommand name and the
                               invocation text, one `command` entry per invocation with duration,
                               outcome, and exit code (data.exit_code)
          tests/steps/development_timeline_support.rs  record_entries — the store read the way a
                               consumer reads it (parse the JSONL, never the recorder's state)
          tests/steps/development_record_steps.rs      the two Then bindings: exactly one `command`
                               entry for the command the scenario ran; duration present + numeric
                               (< 600 s) and `outcome` = `ok`
          tests/steps/mod.rs   GivnWorld gains `last_command`, set by the generic `the user runs …`
                               step, so "that command" is the command the scenario actually ran
        ```
  - [x] **REFACTOR** — remove duplication in the recorder/store surface and the
        fixture helper, no behaviour change; re-run the same command → still
        exit 0. Paste it.
        ```
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.45s       EXIT 0
        $ cargo fmt --all -- --check                                                 EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded step keeps a measured duration and its outcome"
        Feature: Development Record
          Scenario: A recorded step keeps a measured duration and its outcome
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  Then the development record holds one entry for that command
           ✔  And that entry carries a measured duration and its outcome
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Cleanups, no behaviour change: `HistoryStore::text()` is now the single missing-file rule that
        `read()` and `last_id()` share (the "store does not exist yet" arm existed twice);
        `last_id()` mints from the store's last line instead of parsing every line through `read()`;
        the speculative, unused `scope()`/`store()` accessors were deleted; the id counter in `append`
        became `enumerate()` (clippy's `explicit_counter_loop` flagged the counter form, which is why
        clippy is pasted here — it was red for that one lint and is green now).
        ```
  - [x] **COMMIT** — `83fcaf172a55f6dc8b81183d7225b8917d9e8d80` — `feat(development-record): <release note>` (the
        release note verbatim as the subject; body carries the scenario title
        `A recorded step keeps a measured duration and its outcome`). Record the
        revision identifier here, and check this box only when the evidence
        above and the revision are filled in.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A recorded step keeps a measured duration and its outcome"
        [13:16:18] committed revision on the Git backend
        $ git rev-parse HEAD
        83fcaf172a55f6dc8b81183d7225b8917d9e8d80
        ```
        One atomic revision: RED+GREEN+REFACTOR together, touching the delta spec (this scenario's
        `@wip` removed), the production files (`src/history.rs`, `src/main.rs`), the step files, and
        this file's evidence — 7 files, not a spec-only or stub-only commit.

## Task 2 — `A verification run is recorded as its own step` (`development-record`, added)

- [x] **Scenario: `A verification run is recorded as its own step`**
  - Design constraints:
    - Capture point: `src/subprocess.rs::sh_streamed` is the single
      shell-execution choke point → one `step` entry per shell command with its
      existing `label` (gate hook, verification scope, coverage pass) and its
      measured duration. `givn check tasks --change demo` runs the declared
      verification command through it.
    - Second capture point involved: `src/cli/check.rs::run` records the
      mandatory-check boundary as `phase` entries (parented to the command);
      the verification scope's own `duration_ms` already exists and becomes the
      step's duration (the same shape `src/cli/archive.rs::run_scope` uses).
    - Fixture: `givn/commands.yaml` in the temp project declares a
      `verify.command` that takes measurable time (a real subprocess sleeping at
      least one second) — a real measurement, never a mocked clock.
    - Assertion shape: one entry with `kind: "step"`, a bounded `label` naming
      the verification run, `duration_ms >= 1000`, and an `outcome`; the entry is
      parented to the `givn check tasks` command entry.
    - RED expected failure: the verification-step Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A verification run is recorded as its own step"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A verification run is recorded as its own step"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A verification run is recorded as its own step
           ✔  Given a project root with givn/ initialised
           ✘  And a change "demo" with a verification command that takes measurable time
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:16:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2013002) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step (a genuinely new Given — the fixture's measurable verification command —
        plus the two new Then steps). Exit 101, not 0 and not a silent skip.
        ```
  - [x] **GREEN** — wire the choke point and the check boundary. Files:
        `src/subprocess.rs`, `src/cli/check.rs`, `src/history.rs` (if the span
        guard needs the parent link), the step file
        `tests/steps/development_record_steps.rs` and the support file's
        `commands.yaml` fixture. Compile first, re-run the same command → exit 0.
        Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.61s        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A verification run is recorded as its own step"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A verification run is recorded as its own step
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a verification command that takes measurable time
           ✔  When the user runs `givn check tasks --change demo`
           ✔  Then the development record holds one entry for the verification run
           ✔  And that entry carries a duration of at least one second
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files created/modified by this scenario:
          src/subprocess.rs    the shell choke point records one `step` entry per shell command: the
                               caller's existing label as the entry's `name`, the command's text as
                               the bounded subject `label`, the duration measured with
                               `Instant::now()..drop` — the same measurement shape
                               `src/cli/archive.rs::run_scope` already uses — and `ok`/`failed` from
                               the exit status. No caller gained a branch: hooks.rs/archive.rs/
                               coverage_gate.rs call it unchanged.
          src/cli/check.rs     the mandatory-check boundary is one `phase` entry (name "mandatory
                               checks", label = the artifact id), measured across
                               `run_mandatory_checks` and parented to the check command's entry
          src/history.rs       `HistoryRecorder` now buffers entries (`record`) and flushes them in
                               `finish`: the process's `command` entry is appended first, and every
                               buffered entry follows it with `parent` = the command's minted id, so
                               every parent link names an entry this process wrote; `Span` is the
                               real `Instant::now()..drop` guard (kind/label/mark, records on drop,
                               `unknown` when unmarked); `HistoryStore::append` returns the first
                               minted id so the parent link needs no second id source
          tests/steps/development_timeline_support.rs  `SLOW_VERIFY_COMMAND = "sleep 2"` (a real
                               subprocess, never a mocked clock) + `declare_verify_command`, which
                               writes the temp project's `verify.command` through the existing
                               fixture helper `common::write_commands_yaml`
          tests/steps/development_record_steps.rs      the new Given (change + measurable verification
                               command) and the two Then bindings: exactly one `step` entry under the
                               check command's id (which is also the parent-link assertion), its label
                               names the verification run and is ≤ 120 characters; duration ≥ 1000 ms
                               and a valid outcome
        ```
  - [x] **REFACTOR** — tighten the choke point so no caller gains an
        instrumentation branch, no behaviour change; re-run → still exit 0.
        Paste it.
        ```
        $ cargo fmt --all && cargo fmt --all -- --check                                  EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.49s        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A verification run is recorded as its own step"
        Feature: Development Record
          Scenario: A verification run is recorded as its own step
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a verification command that takes measurable time
           ✔  When the user runs `givn check tasks --change demo`
           ✔  Then the development record holds one entry for the verification run
           ✔  And that entry carries a duration of at least one second
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Cleanups, no behaviour change: the two places that built a `HistoryEntry` from scratch
        (`finish` and `Span::drop`) now share `HistoryRecorder::entry` — the schema version, the end
        timestamp, `givn` as source, no parent, no duration — so an entry's base shape has one home;
        the choke point keeps its single guard site and its callers stay branch-free; the new test
        code was formatted (`cargo fmt --all -- --check` was red before this step).
        ```
  - [x] **COMMIT** — `c3aa8dc5a547e28d35c40a53a1d5c94ca2209f11` — `feat(development-record): <release note>` with
        the scenario title `A verification run is recorded as its own step` in
        the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A verification run is recorded as its own step"
        [13:18:56] committed revision on the Git backend
        $ git rev-parse HEAD
        c3aa8dc5a547e28d35c40a53a1d5c94ca2209f11
        ```
        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this scenario's `@wip`
        removed), `src/subprocess.rs`, `src/cli/check.rs`, `src/history.rs`, both test files, and this
        file's Task 1 revision record.

## Task 3 — `A record that cannot be written never fails the command it records` (`development-record`, added)

- [x] **Scenario: `A record that cannot be written never fails the command it records`**
  - Design constraints:
    - Minimal guarantee of the owning use case: recording never changes the
      outcome of the command it records. Every write is best-effort and
      swallowed with a warning; the command's own exit code is untouched
      (`src/history.rs::record`).
    - Fixture: make the store unwritable inside the temp project — the support
      file owns the mechanism; the recommended one is creating a directory at
      the store path (`givn/history/events.jsonl/`) so the append fails
      deterministically (`EISDIR`) without depending on file permissions (a
      `chmod` fixture is unreliable when the suite runs as root). Record the
      chosen mechanism in the step.
    - Assertion shape: `givn status` exits 0 even though the record could not be
      written; stderr carries a warning line naming the development record (and
      the store path); no entry is written.
    - The warning is the *only* observable effect: no re-run, no retry loop, no
      silent fallback that writes elsewhere.
    - RED expected failure: the exit-0 step and the stderr-warning step are
      undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A record that cannot be written never fails the command it records"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A record that cannot be written never fails the command it records"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A record that cannot be written never fails the command it records
           ✔  Given a project root with givn/ initialised
           ✘  And the development record cannot be written
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:24:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2018555) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step — the fixture Given (`the development record cannot be written`) and
        the stderr-warning Then do not exist yet, so the scenario fails instead of passing or
        skipping (exit 101, never 0). Every other scenario kept its `@wip` tag.
        ```
  - [x] **GREEN** — the best-effort write path and its warning already exist
        from Task 1 (commit `83fcaf17`, `src/history.rs`); this scenario pins
        them, so this commit carries no production diff. Work here:
        `tests/steps/development_record_steps.rs` plus the unwritable-store
        fixture in the support file. RED category: contract already holds —
        discrimination is proven by the controlled omission pasted below.
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A record that cannot be written never fails the command it records"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 3.34s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A record that cannot be written never fails the command it records
           ✔  Given a project root with givn/ initialised
           ✔  And the development record cannot be written
           ✔  When the user runs `givn status`
           ✔  Then the command exits 0
           ✔  And stderr warns that the development record could not be written
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files created/modified by this scenario: none — investigated, not skipped.
        The best-effort arm and its single warning already exist: `HistoryRecorder::finish`
        (src/history.rs) routes a failed append to `HistoryRecorder::warn`, which prints
        `warning: the development record could not be written to <store path>: <error>`; that
        arm is what Task 2's flush introduced. This scenario therefore pins the minimal
        guarantee black-box rather than re-implementing it, and all GREEN work is the real
        fixture plus real assertions (never a mocked recorder).

        Discrimination proof — controlled omission (the guarantee already holds, so a green
        run alone could also mean "the fixture never failed the write"). For exactly one run
        the best-effort arm was replaced by a hard failure
        (`Err(error) => { eprintln!("error: …"); std::process::exit(1); }`) and the scenario
        went red:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A record that cannot be written never fails the command it records"   # with the controlled omission applied
        Feature: Development Record
          Scenario: A record that cannot be written never fails the command it records
           ✔  Given a project root with givn/ initialised
           ✔  And the development record cannot be written
           ✔  When the user runs `givn status`
           ✘  Then the command exits 0
              Step panicked. Captured output: expected exit 0, got Some(1): stderr=error: the development record could not be written: Is a directory (os error 21)
        [Summary]
        1 scenario (1 failed)
        EXIT 101

        The omission was reverted immediately (`git diff src/history.rs` is empty) and the
        same command re-ran green (exit 0, pasted above). Together this proves the fixture
        really fails the append (`EISDIR`) and the scenario really discriminates best-effort
        from fail-fast.

        Test/fixture files touched:
          tests/steps/development_timeline_support.rs  `store_file` (the store's own path,
                               shared with `record_entries`) and `make_record_unwritable`:
                               the store's file is replaced by a directory, so the append fails
                               with `EISDIR` deterministically — no `chmod`, so the fixture also
                               holds when the suite runs as root. The fixture project records its
                               own setup commands, so the helper clears the file it already wrote.
          tests/steps/development_record_steps.rs      the Given (applies the fixture) and the
                               Then: exactly one stderr line names the development record (no
                               retry loop), it is a `warning:` naming the store path, and the
                               store path is still the blocking directory — no entry, and no
                               silent fallback that writes elsewhere.
        ```
  - [x] **REFACTOR** — ensure the warning is emitted from exactly one place and
        no caller checks a recorder result; re-run → still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check                                                 EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.42s      EXIT 0

        The two properties this step names, checked rather than asserted:
        $ grep -rn "could not be written" src/
        src/history.rs:275:            "warning: the development record could not be written to {}: {error}",
        $ grep -n "pub fn record\|pub fn span\|pub fn finish\|fn warn\|self.warn(\|recorder.finish\|history::install" src/history.rs src/main.rs
        src/history.rs:216:    pub fn record(&self, entry: HistoryEntry) {
        src/history.rs:224:    pub fn span(&self, name: &str) -> Span<'_> {
        src/history.rs:243:    pub fn finish(&self, outcome: &str, exit_code: i32) {
        src/history.rs:250:            Err(error) => return self.warn(&error),
        src/history.rs:259:            self.warn(&error);
        src/history.rs:273:    fn warn(&self, error: &std::io::Error) {
        src/main.rs:324:    let recorder = history::install(&command_name, &invocation_label());
        src/main.rs:331:    recorder.finish(outcome, exit_code);
        → one emitter (`warn`), reached only from `finish`; `record`, `span` and `finish` all
        return `()`, so no caller can branch on a recorder result (main.rs drops the `()`).

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A record that cannot be written never fails the command it records"
        Feature: Development Record
          Scenario: A record that cannot be written never fails the command it records
           ✔  Given a project root with givn/ initialised
           ✔  And the development record cannot be written
           ✔  When the user runs `givn status`
           ✔  Then the command exits 0
           ✔  And stderr warns that the development record could not be written
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Cleanups, no behaviour change: `store_file` is now the one place the fixtures write the
        store's path (`record_entries` uses it instead of repeating the literal, so the read
        path and the blocking fixture can never disagree); the step file's two `crate::steps`
        imports are one `use` group. Nothing else in the GREEN diff needed cleaning — the
        warning already had exactly one emitter, which is why this step is a proof step here.
        ```
  - [x] **COMMIT** — `ac094e722262711c23bce7b25606070edfe7d501` — `feat(development-record): <release note>` with
        the scenario title `A record that cannot be written never fails the
        command it records` in the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A record that cannot be written never fails the command it records"
        [13:25:14] committed revision on the Git backend
        $ git rev-parse HEAD
        ac094e722262711c23bce7b25606070edfe7d501
        ```
        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this scenario's
        `@wip` removed), the real fixture in `tests/steps/development_timeline_support.rs`, the
        real assertions in `tests/steps/development_record_steps.rs`, and this file's Task 2
        revision record. No production file: the best-effort write path and its single warning
        already exist in `src/history.rs`, and the controlled omission in the GREEN block proves
        the scenario discriminates that arm instead of merely restating it.

## Task 4 — `The record is append-only across commands` (`development-record`, added)

- [x] **Scenario: `The record is append-only across commands`**
  - Design constraints:
    - One JSONL line per entry, opened append-only: no rewrite, no reordering,
      no compaction. `id` is minted at append time under the store's exclusive
      append lock from the store's current last line.
    - Claim discipline (design-review Q5): this scenario claims only that the
      earlier entry is still present **unchanged** and that the new entry is
      **appended without rewriting the store** — it deliberately does not claim
      time ordering, because concurrent writers make file order and time order
      differ (that rule is Task 29's).
    - Assertion shape: read the store bytes before the second command, run
      `givn status`, then assert the earlier line is byte-identical and the file
      grew by exactly one appended line (previous bytes are a prefix of the new
      bytes); also assert the earlier entry's own field values are unchanged when
      read back.
    - RED expected failure: the unchanged/append assertions are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record is append-only across commands"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record is append-only across commands"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record is append-only across commands
           ✔  Given a project root with givn/ initialised
           ✘  And the development record holds an entry from an earlier command
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:32:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2023540) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step — the earlier-command fixture Given and the two append-only Then
        steps do not exist yet. Every other scenario kept its `@wip` tag.
        ```
  - [x] **GREEN** — implement the append path and the byte-prefix assertions;
        Files: `src/history.rs`, `tests/steps/development_record_steps.rs`.
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.63s       EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record is append-only across commands"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.84s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record is append-only across commands
           ✔  Given a project root with givn/ initialised
           ✔  And the development record holds an entry from an earlier command
           ✔  When the user runs `givn status`
           ✔  Then the earlier entry is still present unchanged
           ✔  And the new entry is appended after it without rewriting the store
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files created/modified by this scenario: none — the append path itself is
        `HistoryStore::append` (src/history.rs: `create_dir_all` of the store's directory, then
        `OpenOptions::create(true).append(true)` and one `writeln!` per entry), which already
        opens the store append-only and never rewrites it; this scenario pins that property
        from the outside. Test/fixture files:

          tests/steps/development_timeline_support.rs  `store_bytes` — the store's raw bytes (the
                               claim is about bytes, so the fixture reads the file, not a
                               re-serialized view of its entries) and `parse_entries`, the one JSONL
                               rule both `record_entries` and the byte path share.
          tests/steps/development_record_steps.rs      the three steps: the Given runs a real earlier
                               `givn status` and snapshots the store's bytes plus the earlier entry's
                               own field values; "still present unchanged" asserts the earlier bytes
                               are a byte-identical prefix AND that the entry's field values read
                               back equal; "appended after it without rewriting the store" asserts
                               the file grew by exactly one appended newline-terminated JSON object
                               naming this command, with the earlier bytes untouched.
                               Claim discipline (design-review Q5): the steps claim bytes and field
                               values only — never time ordering, which concurrent writers make
                               differ from file order (that is Task 29's rule).
        ```
  - [x] **REFACTOR** — make the store the only writer of the file (no second
        write path, no read-modify-write); re-run → still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check                                                  EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.69s       EXIT 0

        One writer, verified rather than asserted:
        $ grep -rn "events.jsonl\|store_path" src/*.rs src/cli/*.rs src/skills/*.rs | grep -v "^src/history.rs"
        (no output — no other module names, opens or writes the store)
        $ grep -n "OpenOptions\|fs::write\|File::create" src/history.rs
        src/history.rs:122:        let mut file = std::fs::OpenOptions::new()
        → exactly one writer (`HistoryStore::append`, `create(true).append(true)`); there is no
        `fs::write` and no truncating `File::create` anywhere in the store, so no read-modify-write
        path exists to remove.

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record is append-only across commands"
        Feature: Development Record
          Scenario: The record is append-only across commands
           ✔  Given a project root with givn/ initialised
           ✔  And the development record holds an entry from an earlier command
           ✔  When the user runs `givn status`
           ✔  Then the earlier entry is still present unchanged
           ✔  And the new entry is appended after it without rewriting the store
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Cleanups, no behaviour change: the byte-prefix assertion both Then steps need is now one
        named helper (`assert_store_prefix_unchanged`), so "the earlier bytes were rewritten" has
        one message and one definition instead of two copies; the JSONL parsing rule moved into
        `parse_entries`, which `record_entries` (parsed view) and the byte snapshot share.
        ```
  - [x] **COMMIT** — `49debc5da829d8618270fffc1a8fbef96256f088` — `feat(development-record): <release note>` with
        the scenario title `The record is append-only across commands` in the
        body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The record is append-only across commands"
        [13:26:52] committed revision on the Git backend
        $ git rev-parse HEAD
        49debc5da829d8618270fffc1a8fbef96256f088
        ```
        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this scenario's `@wip`
        removed), both test files, and this file's Task 3 revision record.

## Task 5 — `Activity outside a change belongs to the project scope` (`development-record`, added)

- [x] **Scenario: `Activity outside a change belongs to the project scope`**
  - Design constraints:
    - `resolve_scope` binds every entry to exactly one scope from the project's
      own state, in one order: change → ideation topic → project. Zero active
      changes and no unfinished ideation topic → the **project** scope, written
      as `{"kind":"project","id":"<project-name>"}`; the project name is the
      project root's own name (as the project's own state derives it — read it
      from the fixture root, never hard-coded).
    - No marker file exists or will be created: the record derives scope from
      the filesystem (the rule `givn instructions <artifact>` already applies for
      the change case; `givn/ideation/<topic>/session.md` without `## Completion`
      for the topic case).
    - Fixture: a bare initialised project — no `givn/changes/*` change directory
      and no `givn/ideation/*/session.md` — provided by the support file, so the
      fallback is exercised rather than assumed.
    - Assertion shape: the recorded entry for `givn status` names the project as
      its scope (kind `project`, id equal to the fixture project name).
    - RED expected failure: the scope Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity outside a change belongs to the project scope"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity outside a change belongs to the project scope"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Activity outside a change belongs to the project scope
           ✔  Given a project root with givn/ initialised
           ✘  And no change and no ideation is active for this project
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:40:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2025375) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step — the bare-project Given and the scope Then do not exist yet.
        Every other scenario kept its `@wip` tag.
        ```
  - [x] **GREEN** — implement `resolve_scope`'s fallback and the bare-project
        fixture; Files: `src/history.rs`,
        `tests/steps/development_record_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity outside a change belongs to the project scope"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.46s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Activity outside a change belongs to the project scope
           ✔  Given a project root with givn/ initialised
           ✔  And no change and no ideation is active for this project
           ✔  When the user runs `givn status`
           ✔  Then the recorded entry names the project as its scope
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Production files created/modified by this scenario: none — `resolve_scope`'s fallback
        (`Scope::Project { id: project_name(root) }`, the root directory's own name) already
        exists in `src/history.rs`; the topic arm between change and project is Task 7's, and this
        scenario pins the fallback itself. The GREEN work is the fixture and the real assertions.

        Discrimination proof — controlled omission (the fallback already holds, so the fixture is
        what has to discriminate): `project_name` was made to return the constant `"project"` for
        exactly one run, and the scenario went red on the id assertion:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity outside a change belongs to the project scope"   # with the controlled omission applied
        Feature: Development Record
          Scenario: Activity outside a change belongs to the project scope
           ✔  Given a project root with givn/ initialised
           ✔  And no change and no ideation is active for this project
           ✔  When the user runs `givn status`
           ✘  Then the recorded entry names the project as its scope
              Step panicked. Captured output: assertion `left == right` failed: the entry's scope does not name the fixture's own project: {"id":"project","kind":"project"}
                left: String("project")
               right: String(".tmpTQURSc")
        [Summary]
        1 scenario (1 failed)
        EXIT 101

        The omission was reverted immediately (`git diff src/history.rs` is empty) and the same
        command re-ran green (exit 0, pasted above) — so the scenario reads the recorded scope and
        compares it against the project's own name, never against a constant.

        Test/fixture files touched:
          tests/steps/development_timeline_support.rs  `assert_bare_project` (no directory under
                               `givn/changes/`, no `givn/ideation/<topic>/session.md` — the two
                               states `resolve_scope` could bind to, asserted so the fallback is
                               exercised rather than assumed) and `project_name` (the fixture root's
                               own name, never hard-coded)
          tests/steps/development_record_steps.rs      the Given (asserts the bare project) and the
                               Then: exactly one `command` entry for the command the scenario ran,
                               its `scope.kind` is `project`, and its `scope.id` is the fixture
                               root's own name
        ```
  - [x] **REFACTOR** — express the three scope cases as one rule in one place
        (no duplicated "is a change active?" logic); re-run → still exit 0. Paste
        it.
        ```
        $ cargo fmt --all -- --check                                                  EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.50s       EXIT 0

        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.65s       EXIT 0
        (binary rebuilt after the refactor: src/history.rs 13:28:15 → target/debug/givn 13:28:37)

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity outside a change belongs to the project scope"
        Feature: Development Record
          Scenario: Activity outside a change belongs to the project scope
           ✔  Given a project root with givn/ initialised
           ✔  And no change and no ideation is active for this project
           ✔  When the user runs `givn status`
           ✔  Then the recorded entry names the project as its scope
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Cleanup, no behaviour change: the scope cases are now one list of candidate
        resolvers in the recorder's order — `resolve_scope` takes the first unambiguous one and
        falls through to the project — so adding the ideation topic in Task 7 is one more entry
        in that list, not a second `if let`. The exactly-one rule itself moved into `single`,
        which `change_scope` applies to the change ids (the previous `single_active_change`
        mixed the active-change question with the exactly-one rule, and is gone), so the
        "is a change active?" logic exists once.
        ```
  - [x] **COMMIT** — `80ca8a7b4b7e1b2856d4e7f5702d10471295a912` — `feat(development-record): <release note>` with
        the scenario title `Activity outside a change belongs to the project
        scope` in the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Activity outside a change belongs to the project scope"
        [13:28:46] committed revision on the Git backend
        $ git rev-parse HEAD
        80ca8a7b4b7e1b2856d4e7f5702d10471295a912
        ```
        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this scenario's `@wip`
        removed), `src/history.rs` (the refactored candidate order and the shared exactly-one
        rule), both test files, and this file's Task 4 revision record.

## Task 6 — `Two writers never mint the same entry id` (`development-record`, added)

- [x] **Scenario: `Two writers never mint the same entry id`**
  - Design constraints:
    - Concurrency contract (design-review Q5): `id` is minted **at append time
      under the store's exclusive append lock** from the store's current last
      line — never from a count captured at process start, which is the duplicate
      minting this scenario pins.
    - `parent` links are scoped to ids minted by the same process, so every
      parent link always names an entry that exists in the store; a cross-process
      parent is not produced by construction.
    - Fixture: two `givn` invocations overlap (one of them the intake flush shape
      in the design — a command plus a `givn history ingest` payload), spawned
      concurrently by the step; repeat a few rounds so the overlap is real rather
      than lucky, and record the round count in the step.
    - Assertion shape, over every round: every entry carries a distinct `id`, and
      every `parent` value that appears resolves to an entry present in the
      record; no line is lost (the file parses as one JSON object per line).
    - Do not weaken this into a single-process test: the property is about two
      processes, so the fixture must spawn two.
    - RED expected failure: the distinct-id and parent-resolution Then steps are
      undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Two writers never mint the same entry id"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Two writers never mint the same entry id"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Two writers never mint the same entry id
           ✔  Given a project root with givn/ initialised
           ✘  And two givn commands record at the same time
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:47:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2030750) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step — the two-writer fixture Given, the `when both commands finish`
        step and the two concurrency Then steps do not exist yet. Every other scenario kept its
        `@wip` tag.

        The property is real, not hypothetical: the same shape (6 rounds of two concurrent
        `givn check tasks --change demo` runs, held at a barrier so both append at the same
        moment) run by hand against the current, unlocked binary mints the same id many times
        and interleaves two JSON objects on one line:

        $ for r in $(seq 1 6); do rm -f .race-go; (givn check tasks --change demo &); (givn check tasks --change demo &); sleep 0.3; touch .race-go; wait; done   # in the probe project
        $ python3 -c "import json; [json.loads(l) for l in open('givn/history/events.jsonl')]"
        JSONDecodeError: Extra data: line 1 column 230 (char 229) -> {"schema":1,"id":1,…,"kind":"command","name":"check",…,"data":{"exit_code":0}}{"schema":1,"id":1,…
        raw lines: 24   entries found: 36   ids seen: [1,1,1,1,2,2,3,3,1,1,2,2,1,2,3,2,3,4,5,5,…]
        duplicates: {1: 15, 2: 12, 3: 6, 5: 2}
        ids per line: [2,2,1,1,2,2,2,1,1,1,1,1,1,2,1,1,1,1,2,2,2,2,2,2]
        ← `id` 1 minted 15 times, and 12 lines carry two objects

        This is the controlled omission the GREEN block's fixture reproduces inside the suite
        (barrier-synchronized appends); the append lock is what the GREEN step adds.
        ```
  - [x] **GREEN** — implement the append lock and append-time id minting.
        Files: `src/history.rs`, `tests/steps/development_record_steps.rs`,
        `tests/steps/development_timeline_support.rs` (the concurrent-spawn
        fixture). Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.32s       EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Two writers never mint the same entry id"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.56s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Two writers never mint the same entry id
           ✔  Given a project root with givn/ initialised
           ✔  And two givn commands record at the same time
           ✔  When both commands finish
           ✔  Then every entry in the development record has a distinct id
           ✔  And every parent link names an entry that exists in the record
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Discrimination proof — the same fixture with the lock held out turns the scenario red
        (controlled omission: `file.lock()?;` deleted for exactly one run, everything else
        unchanged):

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Two writers never mint the same entry id"   # with the lock omitted
        Feature: Development Record
          Scenario: Two writers never mint the same entry id
           ✔  Given a project root with givn/ initialised
           ✔  And two givn commands record at the same time
           ✔  When both commands finish
           ✘  Then every entry in the development record has a distinct id
              Step panicked. Captured output: store line is not a JSON object: trailing characters at line 1 column 230
              {"schema":1,"id":3,…,"name":"check",…}{"schema":1,"id":3,…,"name":"check",…}{"schema":1,"id":3,…,"name":"check",…
        [Summary]
        1 scenario (1 failed)
        EXIT 101

        The lock was restored immediately (`grep -c "CONTROLLED OMISSION" src/history.rs` → 0) and
        the same command re-ran green (exit 0, pasted above): the fixture races real processes, and
        the lock is what keeps the ids distinct.

        Production files created/modified by this scenario:
          src/history.rs       `HistoryStore::append` now takes the store's exclusive append lock
                               (`File::lock`) on one handle opened `create(true).read(true)
                               .append(true)`, and only then reads the store's current last line
                               (through that same handle, not a second open of the path) and
                               mints each id from it — the id minted *at append time under the
                               lock*, never from a count captured at process start. The lock is
                               released when the handle drops. `last_id` became a free function
                               over the text the locked handle read.
        Test/fixture files touched:
          tests/steps/development_timeline_support.rs  `RACING_ROUNDS = 6`, `RACE_BARRIER_COMMAND`
                               (`until [ -e .givn-race-go ]; do :; done` — the declared verify
                               command that holds every racer inside `givn check tasks` until the
                               fixture releases them together, so the appends really overlap),
                               `start_racing_commands` (spawns both commands per round, sleeps until
                               they are at the barrier, then releases them) and
                               `finish_racing_commands` (waits, asserts each racer exited 0).
          tests/steps/development_record_steps.rs      the Given (creates the change, starts the
                               race, snapshots the entry count), `when both commands finish`, and
                               the two Then steps: every id distinct and the record's entry count
                               exactly `before + RACING_ROUNDS * 2 * 3` (no line lost or invented);
                               every `parent` value resolves to an id in the record, with the link
                               count asserted so the claim is not vacuous.
        ```
  - [x] **REFACTOR** — keep exactly one id-minting site and one lock scope;
        re-run → still exit 0. Paste it.
        ```
        $ cargo fmt --all && cargo fmt --all -- --check                                 EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s        EXIT 0
        (`cargo fmt --all` was red before this step on the new assertion's filter chain — the
        step's only code change.)

        $ grep -n "file.lock\|entry.id =\|last_id" src/history.rs
        src/history.rs:131:        file.lock()?;
        src/history.rs:134:        let first_id = last_id(&text) + 1;
        src/history.rs:136:            entry.id = first_id + offset as u64;
        src/history.rs:145:fn last_id(text: &str) -> u64 {
        → one lock site and one minting site, both inside `HistoryStore::append`; `last_id`
        (now a free function over the text the locked handle read) is the only other reader of
        ids, and the lock's scope is that single handle's lifetime — taken before the store's
        last line is read, released only after the lines are written.

        Regression check over every scenario this bundle has turned green, in one process (the
        append path changed shape — `O_RDWR|O_APPEND` plus the lock — so the earlier scenarios
        are re-run, including Task 3's `EISDIR` fixture):
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        2 features
        7 scenarios (7 passed)
        40 steps (40 passed)
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Two writers never mint the same entry id"
        Feature: Development Record
          Scenario: Two writers never mint the same entry id
           ✔  Given a project root with givn/ initialised
           ✔  And two givn commands record at the same time
           ✔  When both commands finish
           ✔  Then every entry in the development record has a distinct id
           ✔  And every parent link names an entry that exists in the record
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Cleanups, no behaviour change: the id-minting site and the store read are now the same
        handle's (the second open of the store path is gone — `last_id` no longer opens the file
        behind the lock's back), so the lock covers exactly one read-and-append scope; the test
        code was formatted. One id-minting site and one lock scope, verified by the greps above.
        ```
  - [x] **COMMIT** — `e7c44231cc1816a0ad17637da2bc89529691a60d` — `feat(development-record): <release note>` with
        the scenario title `Two writers never mint the same entry id` in the
        body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Two writers never mint the same entry id"
        [13:33:42] committed revision on the Git backend
        $ git rev-parse HEAD
        e7c44231cc1816a0ad17637da2bc89529691a60d
        ```
        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this scenario's `@wip`
        removed), `src/history.rs` (the append lock), both test files (the barrier fixture and the
        concurrency assertions), and this file's Task 5 revision record.

## Task 7 — `Activity during an ideation belongs to its topic scope` (`development-record`, added)

- [x] **Scenario: `Activity during an ideation belongs to its topic scope`**
  - Design constraints:
    - Ideation scope rule: exactly one **unfinished** topic —
      `givn/ideation/<topic>/session.md` exists and carries no `## Completion`
      section (the section the handoff step appends beside its "handoff
      complete" phase line); the record then writes
      `{"kind":"ideation","id":"<topic-id>"}`. Several unfinished topics fall
      through to the project for the same ambiguity reason as several active
      changes.
    - This is what makes an ideation reportable before any change exists — the
      guarantee Task 28 reports on.
    - Fixture: `givn/ideation/timeline-ideas/session.md` **without** `## Completion`
      (a written-but-unfinished session), plus no active change; the support file
      owns the builder so Task 28 can reuse it.
    - Assertion shape: the entry recorded for `givn status` names the ideation
      topic `timeline-ideas` as its scope.
    - RED expected failure: the topic-scope Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity during an ideation belongs to its topic scope"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity during an ideation belongs to its topic scope"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Activity during an ideation belongs to its topic scope
           ✔  Given a project root with givn/ initialised
           ✘  And an ideation topic "timeline-ideas" whose session is unfinished
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:55:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2036342) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        Category: undefined step — the ideation fixture Given and the topic-scope Then do not
        exist yet. Every other scenario kept its `@wip` tag.
        ```
  - [x] **GREEN** — implement the unfinished-topic rule in `resolve_scope`.
        Files: `src/history.rs`, `tests/steps/development_record_steps.rs`,
        `tests/steps/development_timeline_support.rs` (the ideation fixture
        builder). Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.35s       EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity during an ideation belongs to its topic scope"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 7.95s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Activity during an ideation belongs to its topic scope
           ✔  Given a project root with givn/ initialised
           ✔  And an ideation topic "timeline-ideas" whose session is unfinished
           ✔  When the user runs `givn status`
           ✔  Then the recorded entry names the ideation topic as its scope
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Discrimination proof — controlled omission (the new candidate is what the scenario pins):
        `resolve_scope`'s candidate list was reduced to `[change_scope]` for exactly one run, and
        the scenario went red because the entry then fell through to the project scope:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity during an ideation belongs to its topic scope"   # with the topic candidate omitted
        Feature: Development Record
          Scenario: Activity during an ideation belongs to its topic scope
           ✔  Given a project root with givn/ initialised
           ✔  And an ideation topic "timeline-ideas" whose session is unfinished
           ✔  When the user runs `givn status`
           ✘  Then the recorded entry names the ideation topic as its scope
              Step panicked. Captured output: assertion `left == right` failed: the entry's scope does not name the unfinished ideation topic: {"id":".tmp8DUFmw","kind":"project"}
                left: Object {"id": String(".tmp8DUFmw"), "kind": String("project")}
               right: Object {"id": String("timeline-ideas"), "kind": String("ideation")}
        [Summary]
        1 scenario (1 failed)
        EXIT 101

        The omission was reverted immediately (the git diff of `src/history.rs` is back to the
        topic arm) and the same command re-ran green (exit 0, pasted above).

        Production files created/modified by this scenario:
          src/history.rs       `resolve_scope` gains the second candidate — `topic_scope`, which
                               binds the scope to `{"kind":"ideation","id":"<topic>"}` when
                               exactly one topic under `givn/ideation/` has a `session.md`
                               without a `## Completion` section (`topic_is_unfinished`); several
                               unfinished topics are ambiguous and fall through to the project,
                               the same rule the change case applies.
        Test/fixture files touched:
          tests/steps/development_timeline_support.rs  `declare_unfinished_ideation_topic` (the
                               builder Task 28 reuses: `givn/ideation/<topic>/session.md` with the
                               frame phase and progress lines, no `## Completion`) and the split
                               `assert_no_active_change` / `assert_bare_project`, so the topic
                               fixture can assert "no active change" without claiming the project is
                               bare.
          tests/steps/development_record_steps.rs      the Given (asserts no change is active, builds
                               the topic, remembers its id) and the Then: exactly one `command`
                               entry for the command the scenario ran, whose whole scope object
                               equals `{"kind":"ideation","id":"timeline-ideas"}` — the scope comes
                               from the fixture's topic, not from a literal in the assertion.
        ```
  - [x] **REFACTOR** — share the "exactly one candidate, otherwise fall through"
        shape between the change and topic cases; re-run → still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check                                                   EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.63s        EXIT 0

        The shape is one list and one rule, verified rather than asserted:
        $ grep -n "candidates:\|fn single<\|single(" src/history.rs
        src/history.rs:336:    let candidates: [fn(&Path) -> Option<Scope>; 2] = [change_scope, topic_scope];
        src/history.rs:350:    single(changes.into_iter().map(|change| change.id)).map(|id| Scope::Change { id })
        src/history.rs:363:    single(unfinished).map(|id| Scope::Ideation { id })
        src/history.rs:381:fn single<T>(mut candidates: impl Iterator<Item = T>) -> Option<T> {
        → both cases produce candidates and hand them to the same `single`; neither repeats
        "exactly one, otherwise fall through".

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"   # every scenario this bundle has greened, one process
        [Summary]
        2 features
        8 scenarios (8 passed)
        44 steps (44 passed)
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Activity during an ideation belongs to its topic scope"
        Feature: Development Record
          Scenario: Activity during an ideation belongs to its topic scope
           ✔  Given a project root with givn/ initialised
           ✔  And an ideation topic "timeline-ideas" whose session is unfinished
           ✔  When the user runs `givn status`
           ✔  Then the recorded entry names the ideation topic as its scope
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Cleanups, no behaviour change: the "exactly one `command` entry for the command the
        scenario ran" block existed three times (Task 1's reading step and both scope steps) and is
        now one `recorded_command_entry` helper, so every step finds the entry the same way — by
        the command name the `the user runs …` step captured, never by position, so a fixture's own
        setup commands can never be mistaken for it. The scope steps also read only the scope out of
        that entry, which is what their titles claim.
        ```
  - [x] **COMMIT** — `56b05a26dd90ec1d17a6cb544df833dc8d9a5be6` — `feat(development-record): <release note>` with
        the scenario title `Activity during an ideation belongs to its topic
        scope` in the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Activity during an ideation belongs to its topic scope"
        [13:35:57] committed revision on the Git backend
        $ git rev-parse HEAD
        56b05a26dd90ec1d17a6cb544df833dc8d9a5be6
        ```
        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this scenario's `@wip`
        removed), `src/history.rs` (the topic candidate), both test files, and this file's Task 6
        revision record.

## Task 8 — `A recorded label is bounded to 120 characters` (`development-record`, added)

- [x] **Scenario: `A recorded label is bounded to 120 characters`**
  - Design constraints:
    - Privacy bound (D2, design-review Q7): an entry keeps kind, timestamps,
      duration, outcome, model, token counts, subagent identity, and a single
      label derived from the subject and truncated to ≤ 120 characters with the
      truncation marked (`…`). Complete tool inputs, complete command text, and
      complete question text are **never** stored; a question's label is only its
      first line.
    - `src/history.rs::bound_label` is the only place raw subjects are touched —
      it is a named non-zero production probe in `design.md` §Coverage Process
      Boundaries, so the assertion must read the store the way a consumer does
      (parse the JSONL), never a private field.
    - Fixture: a change `demo` whose command text is longer than 120 characters
      (a `verify.command` or an intake tool label with a long subject) so
      truncation must happen; the step asserts the length in `chars()` (120
      characters, not 120 bytes) and the `…` mark.
    - Second assertion: the store's bytes contain no line carrying the complete
      command text (the strong form of the guarantee: it is absent, not merely
      truncated in the label field).
    - RED expected failure: the bounded-label and absence Then steps are
      undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded label is bounded to 120 characters"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded label is bounded to 120 characters"
        Feature: Development Record
          Scenario: A recorded label is bounded to 120 characters
           ✔  Given a project root with givn/ initialised
           ✘  And a change "demo" with a command whose full text is longer than 120 characters
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:62:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2055048) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        RED category: undefined step — the bounded-label contract is genuinely new
        (`bound_label` is still `todo!()` and no step binds the fixture), so the
        runner fails at the first undefined step, not via an oracle omission.
        ```
  - [x] **GREEN** — implement `bound_label` and route every label through it
        (command label, step label, tool label, question first line). Files:
        `src/history.rs`, `src/subprocess.rs` (label source),
        `tests/steps/development_record_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.37s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded label is bounded to 120 characters"
        Feature: Development Record
          Scenario: A recorded label is bounded to 120 characters
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a command whose full text is longer than 120 characters
           ✔  When the user runs that command
           ✔  Then the recorded entry's label is at most 120 characters and marks the truncation
           ✔  And the record holds no complete command text
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files created/modified:
        - `src/history.rs` — `bound_label` implemented (chars, `LABEL_LIMIT = 120`,
          the mark inside the bound) and the two funnels that build or accept an
          entry — `HistoryRecorder::entry` (command and `Span` entries) and
          `HistoryRecorder::record` (caller-built entries) — bound every label
          through it, so no raw subject can reach the store.
        - `tests/steps/development_record_steps.rs` — the fixture's long command
          (`LONG_VERIFY_COMMAND`, 130 characters, really executed), the
          `the user runs that command` step (which runs `givn check tasks
          --change demo` through the generic subprocess seam), and the two Then
          steps (label bound + derivation + `…` mark; the store's bytes carry no
          complete command text, checked non-vacuously against the recorded
          bounded label).
        `src/subprocess.rs` needed no change and has none: the shell choke point
        already hands its label to `HistoryRecorder::span(..).label(command)`,
        and `Span`'s entry is built by `HistoryRecorder::entry`, the funnel that
        bounds it — a second bounding site there would be duplication, not the
        labelled change.
        ```
  - [x] **REFACTOR** — one truncation implementation, no caller truncates
        itself; re-run → still exit 0. Paste it.
        ```
        Cleanup: the "exactly one `step` entry under the check command" lookup
        existed twice (Task 2's reading step and this task's assertions) and is now
        the one `verified_step_entry` helper, which finds the entry through the
        command's minted id — never by position. `bound_label` stays the only
        truncation site in the crate:

        $ grep -rn "truncat\|chars().take" src/ | grep -v "^src/history.rs"
        src/overlap/fingerprint.rs:131: … chars().take_while(…) …     # unrelated: digit parsing
        src/spec/route.rs:385:        candidates.truncate(10);        # unrelated: candidate list
        $ grep -n "bound_label\|LABEL_LIMIT" src/history.rs
        211:            label: label.map(|label| bound_label(&label)),
        224:        entry.label = entry.label.map(|label| bound_label(&label));
        398:pub const LABEL_LIMIT: usize = 120;
        404:pub fn bound_label(subject: &str) -> String {
        406:    let mut bounded: String = chars.by_ref().take(LABEL_LIMIT).collect();
        → both funnels call the one implementation; no caller truncates itself.

        $ cargo fmt --all -- --check                                 EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.73s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A recorded label is bounded to 120 characters"
        Feature: Development Record
          Scenario: A recorded label is bounded to 120 characters
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a command whose full text is longer than 120 characters
           ✔  When the user runs that command
           ✔  Then the recorded entry's label is at most 120 characters and marks the truncation
           ✔  And the record holds no complete command text
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        The shared helper's other caller still passes unchanged (no behaviour
        change):
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A verification run is recorded as its own step"
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0
        ```
  - [x] **COMMIT** — `0c8cd7707a81993fb551fe21261014fc02dba60a` — `feat(development-record): <release note>` with
        the scenario title `A recorded label is bounded to 120 characters` in
        the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A recorded label is bounded to 120 characters"
        [13:47:59] committed revision on the Git backend
        $ git rev-parse HEAD
        0c8cd7707a81993fb551fe21261014fc02dba60a

        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this
        scenario's `@wip` removed), `src/history.rs` (the bounded label),
        `tests/steps/development_record_steps.rs` (fixture, steps, shared
        lookup), and this file's Task 8 record.
        ```

## Task 9 — `Reading the development record does not extend it` (`development-record`, added)

- [x] **Scenario: `Reading the development record does not extend it`**
  - Design constraints:
    - Deliberate exclusion (design-review Q8): `givn history` and `givn timeline`
      run **without** a recorder — reading the record never mutates it, and an
      inspection that appended an entry about itself would put the newest entry
      into every report it produced and make the store's size depend on how often
      it is read.
    - The read path is `src/cli/history.rs` (`givn history --change demo --json`),
      the render path is `src/cli/timeline.rs`
      (`givn timeline --change demo --out report.html`); both are dispatched
      from `src/main.rs` and must be excluded from the capture wrapper.
    - Assertion shape: read the store's line count (and bytes) before, run both
      commands, then assert the store holds exactly the same lines — no new
      entry, no rewritten line — while `report.html` was still written.
    - Note the scope widening for the record contract: this is the only scenario
      that pins both reading commands, so the render must be exercised here too.
    - RED expected failure: the no-new-entry Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Reading the development record does not extend it"`
        → must exit non-zero. Paste it.
        ```
        The first run of this scenario failed on the fixture's own undefined
        Given (`a change "demo" with a recorded history`, "Step doesn't match any
        function", EXIT 101); the fixture step — a real setup action with its own
        assertions, never a stub — was then written so the RED lands on the new
        behaviour's assertion step, which is the failure the task predicts:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Reading the development record does not extend it"
        Feature: Development Record
          Scenario: Reading the development record does not extend it
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history
           ✔  When the user runs `givn history --change demo --json`
           ✔  And the user runs `givn timeline --change demo --out report.html`
           ✘  Then the development record holds no new entry
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:73:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)

        thread 'main' (2060686) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        RED category: undefined step — the no-new-entry contract is genuinely
        new (`givn history` and `givn timeline` exist only as `todo!()`
        skeletons), so the runner fails on the undefined Then step. The failure
        is not vacuous: both reading commands ran (the generic step captures
        their output), the record was seeded first, and no assertion step
        existed to check it.
        ```
  - [x] **GREEN** — register the two commands with the recorder excluded.
        Files: `src/cli/history.rs`, `src/cli/timeline.rs`,
        `tests/steps/development_record_steps.rs` (the report invocation reuses
        the generic run-command step from `development_report_steps.rs` — never a
        copy). `givn timeline` must already write its report file here (that is
        what this scenario observes); its full model, ranking, and rendering are
        built by Tasks 25–33. Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.89s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Reading the development record does not extend it"
        Feature: Development Record
          Scenario: Reading the development record does not extend it
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history
           ✔  When the user runs `givn history --change demo --json`
           ✔  And the user runs `givn timeline --change demo --out report.html`
           ✔  Then the development record holds no new entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files created/modified:
        - `src/cli/history.rs` — `run` (the read view: ordered record lines of the
          resolved or named scope, JSON array under `--json`, one human line per
          entry otherwise) plus the shared `scope_for`/`entries_in_scope` read
          helpers; the intake/mapper skeletons keep their not-yet-wired allowances
          (now attached to the items, since this task wires the read command).
        - `src/cli/timeline.rs` — `run` (writes the self-contained document to
          `--out` or the default `givn/history/reports/<scope>.html`, prints the
          path, returns it), `render_report` (fills the asset's single
          `/*__GIVN_REPORT_DATA__*/` slot with the serialized model), and
          `build_report_model` carrying the scope and the record lines Tasks 25–33
          will extend with bands, phase derivation, ranking, and totals.
        - `src/main.rs` — the `history` and `timeline` subcommands and their
          dispatch, plus the recorder decision: the wrapper is not entered for
          either command (`records(&cli.command)`), so reading cannot extend the
          record.
        - `src/cli/mod.rs` — the S4 skeleton allowances on the `history` and
          `timeline` modules are dropped: both modules are wired now.
        - `tests/steps/development_record_steps.rs` — the `a change "demo" with a
          recorded history` fixture (real `givn new` + `givn status` runs, then a
          byte snapshot of the store) and the `the development record holds no new
          entry` step: the store's bytes are still exactly the snapshot's, the
          report run succeeded, `report.html` exists with the data slot filled and
          the scope it renders named in it. The report invocation uses the generic
          `the user runs …` step — no report step is written here.
        Non-vacuity: the Then step fails when the record grows (any recorded entry
        changes the bytes), when the report command fails, and when the document
        still carries its unfilled slot; the store snapshot plus the written
        report together prove the reads really ran.
        ```
  - [x] **REFACTOR** — make the exclusion structural (the wrapper is not entered)
        rather than a conditional inside the recorder; re-run → still exit 0.
        Paste it.
        ```
        The exclusion now derives from the parsed command
        (`fn records(command: &Commands) -> bool`) rather than from a name
        comparison, and `HistoryRecorder::install` is simply not called for the
        two reading commands — the wrapper is never entered:

        $ grep -n "fn records" -A 8 src/main.rs
        374:/// Whether this invocation records. `givn history` and `givn timeline` are
        375:/// deliberately not recorded — reading the record never mutates it — so the
        376:/// recording wrapper is not entered for them at all: an inspection that
        377:/// appended an entry about itself would put the newest entry into every report
        378:/// it produced (design-review Q8).
        379:fn records(command: &Commands) -> bool {
        380:    !matches!(
        381:        command,
        382:        Commands::History { .. } | Commands::Timeline { .. }
        383:    )
        384:}

        $ cargo fmt --all -- --check                                    EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.40s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Reading the development record does not extend it"
        Feature: Development Record
          Scenario: Reading the development record does not extend it
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history
           ✔  When the user runs `givn history --change demo --json`
           ✔  And the user runs `givn timeline --change demo --out report.html`
           ✔  Then the development record holds no new entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        The recorder plumbing changed for every command, so the whole greened
        delta was re-run in one process as well:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        2 features
        10 scenarios (10 passed)
        54 steps (54 passed)
        EXIT 0
        ```
  - [x] **COMMIT** — `d41ab44644e3c547207b1f2da057d448d18b1855` — `feat(development-record): <release note>` with
        the scenario title `Reading the development record does not extend it` in
        the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Reading the development record does not extend it"
        [13:51:04] committed revision on the Git backend
        $ git rev-parse HEAD
        d41ab44644e3c547207b1f2da057d448d18b1855

        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this
        scenario's `@wip` removed), both read commands with the structural
        exclusion, the wiring, the steps, and this file's Task 9 record.
        ```

## Task 10 — `The record stays out of the repository's own history` (`development-record`, added)

- [x] **Scenario: `The record stays out of the repository's own history`**
  - Design constraints:
    - D1 (corrected rule): the **recorder never writes a versioned file**; the
      record path reaches versioned destinations only through the commands that
      own them (`givn init`, `givn skills sync`, `givn upgrade` —
      `write_ignores`), while the recorder asserts the never-dirty destination
      on **every** write. Realized in `src/history.rs::store_path` and
      `src/history.rs::ensure_ignored`.
    - Git destination: the `# givn:begin:history` block in
      **`.git/info/exclude` only**, resolved with
      `git rev-parse --git-path info/exclude` and written through the existing
      `upsert_local_exclude`. Re-assert it idempotently on every record write,
      not only when the store is created (a store that predates the repository
      must still become ignored). Never write `.gitignore` while a repository
      exists: the recorder runs at command start, so a versioned write would
      dirty the worktree and `givn upgrade`'s clean-checkout precondition
      (`src/cli/migration.rs`, "automatic managed upgrade requires a clean
      checkout") would refuse. `.gitignore` is written only when no repository
      exists yet — and repository existence is decided by the backend markers
      at the project root, **never** by `scm::select` succeeding (select also
      bails for a nested root, both markers, or a missing executable).
    - The Git block is a fenced managed block: re-running must not duplicate it,
      and user content outside the block stays untouched.
    - Fixture: the Git backend fixture (`BackendKind::Git` in
      `tests/common/mod.rs`) with a real `git` worktree.
    - Assertion shape: after `givn status --change demo`, the repository reports
      no pending change for the development record (`git status --porcelain`
      output contains no `givn/history/events.jsonl` entry, untracked or
      modified), for a project where the record was written by the same command.
    - Fixture class to adapt (F6 — the failure was a class, not one instance):
      `tests/steps/run_execution_e2e_steps.rs::git_managed_project`,
      `tests/steps/release_0_3_0_e2e_steps.rs::git_project_with_version`. Both
      overwrite the ignore file and then commit everything, which tracks the
      store and makes later appends show as modifications, tripping the
      clean-checkout guard in the permanent upgrade scenarios. Minimal remedy
      per fixture: the record path (`givn/history/*` for the Fossil fixture
      below, `givn/history/` for the Git `.gitignore` content here) joins the
      fixture's hand-written ignore content, or the store is removed before the
      fixture's commit.
    - Permanent-scenario acceptance evidence (extra, must be pasted below): after
      GREEN, the clean-checkout scenarios that consume those fixtures must exit
      0 —
      `cargo test --locked --test features_runner -- --exact "operate-product/declare-the-product-start.feature::The upgrade commit includes the backfilled run declaration"`
      and
      `cargo test --locked --test features_runner -- --exact "operate-product/declare-the-product-start.feature::A repeated upgrade creates no additional commit"`.
      Paste both commands and their output.
    - No push, no commit: the scenario only proves the file is ignored.
    - RED expected failure: the no-pending-change Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"`
        → must exit non-zero. Paste it.
        ```
        The first run failed on the new fixture's own undefined Given; the
        fixture step (a real setup action with its own assertion) was then
        written, so the RED lands on the assertion step the task predicts:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"
        Feature: Development Record
          Scenario: The record stays out of the repository's own history
           ✘  Given a project root with givn/ initialised under Git
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:77:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        EXIT 101   (first run: the fixture step is undefined)

        After the fixture step was written, the RED lands on the new behaviour's
        assertion step:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"
        Feature: Development Record
          Scenario: The record stays out of the repository's own history
           ✔  Given a project root with givn/ initialised under Git
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✘  Then the repository reports no pending change for the development record
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:80:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (2114497) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        RED category: undefined step — the ignore contract is genuinely new
        (`ensure_ignored` was still `todo!()`), and the fixture's own run of
        `givn status --change demo` really wrote the record the Then step reads.
        ```
  - [x] **GREEN** — implement `ensure_ignored` over the existing backend switch.
        Files: `src/history.rs`, `src/skills/rules.rs` (reuse/seam only — do not
        merge the record into the rules block), the step file
        `tests/steps/development_record_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.04s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"
        Feature: Development Record
          Scenario: The record stays out of the repository's own history
           ✔  Given a project root with givn/ initialised under Git
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  Then the repository reports no pending change for the development record
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Production files created/modified:
        - `src/history.rs` — the record's own ignore vocabulary
          (`HISTORY_BLOCK_BEGIN`/`HISTORY_BLOCK_END`, `GIT_IGNORE_ENTRY =
          "givn/history/"`, `FOSSIL_IGNORE_ENTRY = "givn/history/*"`), the real
          `ensure_ignored` (best effort, over the writer's destination table),
          and the call site: `HistoryRecorder::finish` re-asserts the record's
          ignore entry immediately before every store write, so a store that
          predates the repository still becomes ignored and no destination it
          writes can dirty the worktree (`root` is now carried by the recorder).
        - `src/skills/rules.rs` — the destination table both writers read:
          `RootBackend` decided from the markers at the project root (`.git`,
          `.fslckout`/`_FOSSIL_`; `Bare` before a repository exists;
          `Ambiguous` when both markers are present, which neither writer
          touches), `IgnoreDestination` (`Fenced(path)`/`FossilSetting`),
          `IgnoreWrite`, `Fence` (`RULES_FENCE` unchanged, `HISTORY_FENCE` the
          record's own markers), `rules_writes`, `record_writes`, one
          `apply_write`, and the `upsert_block`/`render_block` pair
          parameterized by fence — so the record never joins the
          `# givn:begin:rules` block. `write_ignores` now iterates the table
          instead of matching `scm::select`; the Fossil arm adds
          `givn/history/*` to the managed file's entry list, because a committed
          `.fossil-settings/ignore-glob` overrides the repository setting.
          `upsert_local_exclude` folded into the table; the Fossil setting writer
          is now `upsert_fossil_setting`.
        - `tests/steps/development_record_steps.rs` — the `a project root with
          givn/ initialised under Git` fixture (the Git backend fixture with a
          real worktree, asserted) and the `the repository reports no pending
          change for the development record` step: `git status --porcelain
          -uall` carries no line naming `givn/history/events.jsonl`, with the
          store asserted to exist (the scenario's own command wrote it) and
          `git check-ignore` confirming the record is ignored, not merely
          committed.
        - Fixture class (F6, declared remedy): `tests/steps/run_execution_e2e_steps.rs::git_managed_project`
          and `tests/steps/release_0_3_0_e2e_steps.rs::git_project_with_version`
          now write `givn/history/` into their hand-written `.gitignore`
          content, so their `git add --all` cannot commit the store.

        Permanent-scenario acceptance evidence (the extra evidence this task
        requires), both after this GREEN:

        $ cargo test --locked --test features_runner -- --exact "operate-product/declare-the-product-start.feature::The upgrade commit includes the backfilled run declaration"
        Feature: Declare the product start
          Scenario: The upgrade commit includes the backfilled run declaration
           ✔  Given a git-managed project with givn initialised
           ✔  And the commands file has no run declaration
           ✔  When the user runs `givn upgrade`
           ✔  Then the command exits 0
           ✔  And the working tree is clean
           ✔  And the latest revision contains "givn/commands.yaml"
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT 0

        $ cargo test --locked --test features_runner -- --exact "operate-product/declare-the-product-start.feature::A repeated upgrade creates no additional commit"
        Feature: Declare the product start
          Scenario: A repeated upgrade creates no additional commit
           ✔  Given a git-managed project with givn initialised
           ✔  And the commands file has no run declaration
           ✔  When the user runs `givn upgrade`
           ✔  And the user runs `givn upgrade` again
           ✔  Then the command exits 0
           ✔  And the latest revision is unchanged by the second upgrade
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT 0

        Before this task the first scenario failed with exit 101 at
        `the working tree is clean` (` M givn/history/events.jsonl`: the
        fixture's `git add --all` had committed the store, and the recording
        modified it).
        ```
  - [x] **REFACTOR** — one destination table shared with the rules writer, no
        duplicated Git/Fossil branches; re-run → still exit 0. Paste it.
        ```
        The Git/Fossil split exists once — in `root_backend` plus the two tables
        that read it (`rules_writes`, `record_writes`) — and one writer applies
        both (`apply_write`); neither the rules policy nor the record repeats a
        backend branch. Cleanup with no behaviour change: `upsert_local_exclude`
        is gone (the Git local exclude is a table row now), the fenced-block
        helpers take the `Fence` instead of hardcoding the rules markers, and the
        last skeleton allowance in the binary is dropped now that the step
        capture, the bounded label, and the ignore block are all wired:

        $ grep -n "mod history" src/main.rs
        8:mod history;
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.27s   (no warnings)
        $ cargo fmt --all -- --check                                    EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.27s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"
        Feature: Development Record
          Scenario: The record stays out of the repository's own history
           ✔  Given a project root with givn/ initialised under Git
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  Then the repository reports no pending change for the development record
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Re-verified after the refactor: both permanent clean-checkout scenarios
        still exit 0 (same output as the GREEN block above), and the whole
        greened delta passes in one process:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        2 features
        11 scenarios (11 passed)
        58 steps (58 passed)
        EXIT 0

        The fixtures this task adapted are consumed by a whole feature, so it was
        re-run too:
        $ GIVN_FEATURES=givn/specs/upgrade-a-project cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        1 feature
        12 scenarios (12 passed)
        65 steps (65 passed)
        EXIT 0
        ```
  - [x] **COMMIT** — `12d835a96c084de03b5b99fe63f59e7746184dd8` — `feat(development-record): <release note>` with
        the scenario title `The record stays out of the repository's own history`
        in the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The record stays out of the repository's own history"
        [14:28:21] committed revision on the Git backend
        $ git rev-parse HEAD
        12d835a96c084de03b5b99fe63f59e7746184dd8

        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this
        scenario's `@wip` removed), the record's ignore entry and its per-write
        re-assertion, the shared destination table with the owning commands'
        Fossil entry, the two adapted fixtures, the steps, and this file's
        Task 10 record.
        ```

## Task 11 — `A Fossil project ignores the record through its own ignore file` (`development-record`, added)

- [x] **Scenario: `A Fossil project ignores the record through its own ignore file`**
  - Design constraints:
    - Same D1 rule, Fossil arm: the record path reaches the versioned
      destination only through the owning commands. When
      `.fossil-settings/ignore-glob` is **absent**, the recorder writes the
      repository-level `ignore-glob` setting (`fossil settings --value
      ignore-glob` … `fossil settings ignore-glob <value>`, the existing
      `upsert_fossil_local_ignore`). When that file **is present** it overrides
      the setting, so the record path must be part of that file's **managed
      entry list**, written by `givn init` / `givn skills sync` / `givn upgrade`
      through `write_ignores`; the recorder writes nothing versioned at
      plain-command time. Writing only the setting in a managed checkout would
      leave the store an `EXTRA` and `givn upgrade`/`givn archive` would refuse
      a project that did nothing wrong.
    - Entry form: **`givn/history/*`**, never `givn/history` — a Fossil glob
      must match the whole path and `*` crosses `/`, so a bare directory path
      matches nothing. The Fossil destinations are a deduplicated entry list
      without fence markers (only the Git destinations are fenced
      `# givn:begin:history` blocks).
    - Fixture: the `BackendKind::Fossil` fixture — a real `fossil` repository kept
      outside the checkout (so it is never an `EXTRA` entry), created by
      `tests/common/mod.rs`'s existing helper.
    - Fixture requirement (F3): keep the managed file effective and the store
      untracked — run `givn init` inside the open Fossil checkout so the managed
      entry list lands in `.fossil-settings/ignore-glob` **before** the fixture
      commit, because `make_fossil_root`'s `addremove` would otherwise track the
      store. The same adaptation applies to
      `tests/steps/release_0_3_0_e2e_steps.rs::fossil_project_with_version`,
      which today sets only the repository setting and then commits.
    - Assertion shape: after `givn status --change demo`, the effective ignore
      source names the development record — read the effective `ignore-glob`
      setting **and** the managed file's entry list, never only the file the
      step happened to write, and assert the record path is not reported as a
      pending or extra change.
    - The Fossil path must not be implemented as a Git-only branch plus a stub:
      if the seam cannot express it, that is the design-review escalation in
      *Global constraints*, not a silent skip.
    - RED expected failure: the ignore-glob Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"
        Feature: Development Record
          Scenario: A Fossil project ignores the record through its own ignore file
           ✘  Given a project root with givn/ initialised in a Fossil checkout
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:84:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (2127344) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        RED category: undefined step — both the Fossil fixture's Given and the
        ignore-glob Then step are new for this scenario (the checkout fixture did
        not exist at all); the run fails at the first of them.

        The Then step's discrimination was then proved by a controlled omission,
        restored immediately: with the record entry removed from the Fossil
        managed entry list (the pre-correction behaviour, `&& false` in the
        entry-list build, never a production switch), the same scenario failed —
        the RED states the fixture-level omission, not a production toggle:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"
        Feature: Development Record
          Scenario: A Fossil project ignores the record through its own ignore file
           ✔  Given a project root with givn/ initialised in a Fossil checkout
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✘  Then the repository's ignore glob names the development record
              Step panicked. Captured output: the managed ignore file does not name the development record:
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)
        EXIT 101
        ```
  - [x] **GREEN** — extend `ensure_ignored` with the Fossil destinations. Files:
        `src/history.rs`, `src/skills/rules.rs`, `tests/steps/development_record_steps.rs`.
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.90s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"
        Feature: Development Record
          Scenario: A Fossil project ignores the record through its own ignore file
           ✔  Given a project root with givn/ initialised in a Fossil checkout
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  Then the repository's ignore glob names the development record
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Production files created/modified: none — and that is the investigated
        outcome, not an omission. The two Fossil rows belong to the one table
        Task 10 built to satisfy this task pair's shared REFACTOR requirement
        ("one destination table shared with the rules writer, no duplicated
        Git/Fossil branches"), and every line of them landed in Task 10's
        revision `12d835a9`: `rules_writes`'s Fossil arm adds
        `givn/history/*` to the managed entry list inside
        `.fossil-settings/ignore-glob` (the committed file overrides the
        repository setting), and `record_writes` writes the repository-level
        `ignore-glob` setting only while no managed file overrides it, entering
        nothing versioned at plain-command time. This scenario is the acceptance
        for that arm; the probe below shows the behaviour it observes:

        $ (fixture walk, probe): a bare root → `fossil init`/`open` → `givn init --agents opencode`
        $ cat .fossil-settings/ignore-glob
        # givn:begin:rules
        .opencode/skills/givn-*
        …
        .githooks/commit-msg
        givn/history/*
        # givn:end:rules
        $ fossil settings --value ignore-glob          # effective source
        … ,givn/history/*, …                            (the file's list)
        $ fossil addremove && fossil commit --allow-empty -m fixture
        $ fossil ls | grep history                     → (no output: the store is NOT tracked)
        $ givn status --change demo
        $ fossil extras; fossil changes                → neither names givn/history/events.jsonl

        Test files created/modified:
        - `tests/steps/development_record_steps.rs` — the `a project root with
          givn/ initialised in a Fossil checkout` fixture: a real Fossil checkout
          first, `givn init` INSIDE it (so the managed entry list is committed
          before the fixture commit), then the fixture commit (F3; `addremove`
          would otherwise record the store), plus the `the repository's ignore
          glob names the development record` step: the managed file's entry list
          is read (the effective source in a managed checkout) and the
          repository-level `ignore-glob` setting is read, so the step never
          asserts on the file it happened to write; the record is then asserted
          absent from both `fossil changes` and `fossil extras`.
        - `tests/steps/release_0_3_0_e2e_steps.rs::fossil_project_with_version`
          (the declared F3 fixture remedy): its hand-written ignore content is
          now `.opencode/*,AGENTS.md,.gitignore,givn/history/*`, so its
          `addremove` + commit cannot record the store.
        ```
  - [x] **REFACTOR** — keep the destination choice in one backend-keyed place;
        re-run → still exit 0. Paste it.
        ```
        The destination choice already lives in one backend-keyed place — the
        `root_backend` decision plus the two tables that read it (`rules_writes`,
        `record_writes`) — and this scenario's own file list carries a step-level
        helper (`fossil_glob_entries`) so the entry list is parsed in one place.
        Nothing else needed changing; no production file is modified by this task
        (see GREEN). The restored state is byte-identical to the committed
        program:

        $ git status --porcelain
         M givn/changes/development-timeline/specs/inspect-development-history/development-record.feature
         M givn/changes/development-timeline/tasks.md
         M tests/steps/development_record_steps.rs
         M tests/steps/release_0_3_0_e2e_steps.rs
        → no `src/` file is pending: the controlled-omission probe above was
          reverted before this run (the restored run is the GREEN one).

        $ cargo fmt --all -- --check                                    EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.85s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"
        Feature: Development Record
          Scenario: A Fossil project ignores the record through its own ignore file
           ✔  Given a project root with givn/ initialised in a Fossil checkout
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  Then the repository's ignore glob names the development record
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        The whole greened delta in one process:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        2 features
        12 scenarios (12 passed)
        62 steps (62 passed)
        EXIT 0

        The fixtures this task adapted are consumed by a whole feature (its
        Git and Fossil outline rows included):
        $ GIVN_FEATURES=givn/specs/upgrade-a-project cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        1 feature
        12 scenarios (12 passed)
        65 steps (65 passed)
        EXIT 0

        Both permanent clean-checkout scenarios the fixture class feeds still
        exit 0:
        $ cargo test --locked --test features_runner -- --exact "operate-product/declare-the-product-start.feature::The upgrade commit includes the backfilled run declaration"
        6 steps (6 passed)                                              EXIT 0
        $ cargo test --locked --test features_runner -- --exact "operate-product/declare-the-product-start.feature::A repeated upgrade creates no additional commit"
        6 steps (6 passed)                                              EXIT 0

        CHANGE-WIDE PERMANENT-CORPUS OBSERVATION (reported, not patched — the
        scenario boxes above are unaffected by it). The whole permanent corpus
        was run after this task:

        $ GIVN_FEATURES=givn/specs cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        49 features
        544 scenarios (530 passed, 14 failed)
        3158 steps (3144 passed, 14 failed)
        EXIT 1

        All 14 failures are one class: a permanent scenario compares raw project
        files (or an exact directory list) with the record inside the compared
        set, and the recording itself changes the record — not the ignore
        mechanism, and not the working-tree-tracking class this task pair fixed.
        The failure details name the record every time:

        - 9 × `And no file in the project changed` →
          `changed: ["modified: givn/history/events.jsonl"]`
          (the refusal scenarios of `fragments/detect-the-project-repository.feature`
          and `archive-change/archive-proof.feature`);
        - 1 × `Then no project file changed` →
          `["givn/history/events.jsonl"]`
          (`obtain-effective-guidance/serve-extended-rules.feature::A guidance request changes nothing in the project`);
        - 4 × `And the directory "givn" does not exist` after a refused
          `givn init` in a bare root
          (`initialize-project/initialize-a-project.feature`) — the bare-root
          recording creates `givn/history/` before `init` can refuse.

        The design's declared fixture class (F6: `git_managed_project`,
        `git_project_with_version`, `fossil_project_with_version`) covers the
        tracking mechanism, which this task pair fixed; it names none of these
        fixtures, and `snapshot_tree`'s own rationale ("Backend bookkeeping is
        not a project file") would have to extend to the record path for them to
        hold — a corpus-fixture change outside this task, so it is reported
        rather than patched.
        ```
  - [x] **COMMIT** — `e531f80fd0d875d0f09106055b31b6b80996c22b` — `feat(development-record): <release note>` with
        the scenario title `A Fossil project ignores the record through its own
        ignore file` in the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A Fossil project ignores the record through its own ignore file"
        [14:32:55] committed revision on the Git backend
        $ git rev-parse HEAD
        e531f80fd0d875d0f09106055b31b6b80996c22b

        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this
        scenario's `@wip` removed), the Fossil checkout fixture and its
        assertions, the adapted `fossil_project_with_version` fixture, and this
        file's Task 11 record. It carries no `src/` change because the Fossil arm
        landed with Task 10's single shared destination table (see GREEN): the
        production work of this scenario is verified here and was written there,
        exactly as the retired `ac094e72` precedent (Task 3's reuse scenario)
        recorded its own production path as already present.
        ```

## Task 11a — `A refused command outside a Givn project creates no record` (`development-record`, added)

- [x] **Scenario: `A refused command outside a Givn project creates no record`**
  - Design constraints:
    - Engagement rule (product): the recorder engages **only when the project
      root carries the Givn scaffold (`givn/`)**; before the scaffold exists it
      writes nothing and creates no directory, so a refused command in a bare
      root (for example `givn init` with no agent target, or a root without
      version control) leaves that root untouched. A successful `givn init`
      creates the scaffold and then records.
    - The rule keys on the scaffold, **not** on the VCS markers: once `givn/`
      exists, every command records — including refusals and failures — and a
      scaffolded project without a usable backend still records (its store write
      is legitimate; the upgrade clean-checkout refusal is the backend's own
      business, and the store is ignored through Task 10/11's destinations).
    - Exact files: `src/history.rs` — the engagement predicate is checked **at
      the top of `HistoryRecorder::finish`, before `ensure_ignored` and before
      the store path is created or opened**, so both the ignore re-assert and the
      store append are gated (calling `finish` in a root with no scaffold must
      write nothing and create nothing, including the `.gitignore` fallback a
      backendless root would otherwise receive) — and the step file
      `tests/steps/development_record_steps.rs` for the new steps. No new
      command, flag, or output surface.
    - Test-side class (Part B — the same task owns these edits): the
      project-state comparison helpers must exclude the record store
      (`givn/history/**`), because the store is Givn runtime state, not project
      content, and a legitimate command entry otherwise reads as "the project
      changed" in refusal and guidance scenarios:
      - `tests/steps/scm_backend_selection_steps.rs::snapshot_tree` /
        `::collect_tree` — exclude the record directory **by path**, i.e.
        `path == root.join("givn/history")`, and not by bare entry name:
        `collect_tree`'s skip list matches `entry.file_name()`, so skipping
        `Some("history")` would also drop any unrelated `history` directory
        anywhere in the tree. This helper drives `no file in the project
        changed`, shared by the refusal scenarios of
        `fragments/detect-the-project-repository.feature` and
        `archive-change/archive-proof.feature` (and it is also used by
        `archive_proof_steps.rs` and the `scm_fixture` snapshot Given steps).
      - `tests/steps/serve_extended_rules_steps.rs::project_files` /
        `::collect_project_files` — same exclusion, and this helper already
        compares paths (`path == root.join(EXTENSION_FIXTURE_DIR)` at
        `serve_extended_rules_steps.rs:481`), so the record directory joins that
        form; this drives `no project file changed` in
        `obtain-effective-guidance/serve-extended-rules.feature`.
      - Checked and out of scope (they never walk the record directory, so they
        stay untouched): `tests/steps/repository_ownership_fixtures_support.rs::snapshot_tree`
        and `::collect_files` (their callers root them at `givn/specs`),
        `tests/steps/spec_discovery_deterministic_steps.rs::collect_tree_text`
        (rooted at `givn/specs`), and
        `tests/steps/release_0_3_0_e2e_steps.rs::walk_feature_files` (feature
        files only).
      - Scope: the exclusion covers the record directory only; no other path's
        behavior changes.
    - RED expected failure: the run fails at **the `the directory "givn" does
      not exist` assertion** (today the refusal still creates `givn/` through
      the recorder, so the pre-fix behavior breaks that Then). If the runner
      reports undefined steps for the new phrasing as well — the
      `the project root holds no development record` / `holds a development
      record` pair — paste whichever output the run actually produces; the run
      must exit non-zero either way.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A refused command outside a Givn project creates no record"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A refused command outside a Givn project creates no record"
        Feature: Development Record
          Scenario: A refused command outside a Givn project creates no record
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init`
           ✔  Then the command exits non-zero
           ✘  And the directory "givn" does not exist
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:94:5
              Matched: tests/steps/mod.rs:928:1
              Step panicked. Captured output: expected directory "givn" to not exist
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (2278531) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT 101

        RED category: an existing rule changed — the run fails at exactly the
        assertion the task predicts (`the directory "givn" does not exist`),
        because the recorder still engages in a bare root and creates
        `givn/history/` (plus its backendless-root `.gitignore` fallback) for the
        refused `givn init`. The scenario's later new steps
        (`the project root holds no development record`,
        `the directory "givn" exists`, `the project root holds a development
        record`) are not reached: cucumber fails the scenario at this step, and
        the run is non-zero either way.
        ```
  - [x] **GREEN** — implement the engagement predicate in `src/history.rs` and
        the helper exclusions above; add the steps to
        `tests/steps/development_record_steps.rs`. Compile first, re-run →
        exit 0. Paste it. Also re-run the affected permanent refusal scenarios
        (`fragments/detect-the-project-repository.feature` and
        `archive-change/archive-proof.feature`) and paste their exit-0 result,
        proving the store no longer reads as a project change.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.34s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A refused command outside a Givn project creates no record"
        Feature: Development Record
          Scenario: A refused command outside a Givn project creates no record
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init`
           ✔  Then the command exits non-zero
           ✔  And the directory "givn" does not exist
           ✔  And the project root holds no development record
           ✔  When the consumer runs `givn init --no-addons --agents opencode`
           ✔  Then the command exits 0
           ✔  And the directory "givn" exists
           ✔  And the project root holds a development record
        [Summary]
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        EXIT 0

        Files created/modified:
        - `src/history.rs` — `fn engages(root) -> bool` (the one engagement
          predicate: the root carries the Givn scaffold `givn/`), checked at the
          top of `HistoryRecorder::finish` **before** `ensure_ignored` and before
          the store path is created or opened, so both the ignore re-assert and
          the store append are gated and a root with no scaffold receives
          nothing — including the backendless-root `.gitignore` fallback.
        - `tests/steps/development_record_steps.rs` — the new steps
          (`the directory "givn" exists`, `the project root holds no development
          record` — no store file, no `givn/history` directory, no
          scaffold-time `.gitignore` — and `the project root holds a development
          record`, which parses the store the way a consumer does and requires
          the successful `init` command entry), plus the shared exclusion
          `RECORD_DIR`/`is_record_dir(root, path)`.
        - `tests/steps/scm_backend_selection_steps.rs::collect_tree` — excludes
          the record directory **by path** (`is_record_dir`, never a bare
          `history` entry name), so an unrelated `history` directory anywhere in
          the tree is still walked.
        - `tests/steps/serve_extended_rules_steps.rs::collect_project_files` —
          the same shared predicate, joining that helper's existing path form
          (`path == root.join(EXTENSION_FIXTURE_DIR)`).
        No new command, flag, or output surface: the change adds the predicate,
        the steps, and the two test-side exclusions only.

        The affected permanent scenarios, the previously failing assertions
        included:

        $ GIVN_FEATURES=givn/specs/fragments cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        18 features
        163 scenarios (163 passed)
        867 steps (867 passed)
        EXIT 0
        (includes the four refusal scenarios of
        `detect-the-project-repository.feature` — `A project without version
        control is refused by name`, `A project root with both markers is refused
        as ambiguous`, `A project root nested inside a larger Git worktree is
        refused by name`, `A Git worktree without the Git executable is refused
        by name` — all failing at `no file in the project changed` before this
        task)

        $ GIVN_FEATURES=givn/specs/archive-change cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        5 features
        43 scenarios (43 passed)
        303 steps (303 passed)
        EXIT 0
        (includes `archive-proof.feature`'s refusal scenarios)

        $ GIVN_FEATURES=givn/specs/obtain-effective-guidance cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        1 feature
        5 scenarios (5 passed)
        27 steps (27 passed)
        EXIT 0
        (includes `serve-extended-rules.feature::A guidance request changes
        nothing in the project` — `Then no project file changed` now passes)

        And the whole permanent corpus, which these edits unblock:

        $ GIVN_FEATURES=givn/specs cargo test --locked --test features_runner -- --tags "not @e2e"
        [Summary]
        49 features
        544 scenarios (544 passed)
        3158 steps (3158 passed)
        EXIT 0
        (before this task: 530 passed / 14 failed)
        ```
  - [x] **REFACTOR** — keep one engagement predicate and one shared skip list
        (no per-helper duplication of the record path); re-run → still exit 0.
        Paste it.
        ```
        One engagement predicate (`src/history.rs::engages`, one call site at the
        top of `finish`) and one definition of the record path
        (`tests/steps/development_record_steps.rs::RECORD_DIR` + `is_record_dir`,
        consumed by both helpers through that predicate — neither helper names
        `givn/history` itself):

        $ grep -n "engages\|is_record_dir\|RECORD_DIR" src/history.rs tests/steps/development_record_steps.rs tests/steps/scm_backend_selection_steps.rs tests/steps/serve_extended_rules_steps.rs
        src/history.rs:257:        if !engages(&self.root) {
        src/history.rs:445:/// Whether the recorder engages in this project: only a root that carries the
        src/history.rs:453:fn engages(root: &Path) -> bool {
        tests/steps/development_record_steps.rs:493:pub(crate) const RECORD_DIR: &str = "givn/history";
        tests/steps/development_record_steps.rs:499:pub(crate) fn is_record_dir(root: &std::path::Path, path: &std::path::Path) -> bool {
        tests/steps/development_record_steps.rs:500:    path == root.join(RECORD_DIR)
        tests/steps/scm_backend_selection_steps.rs:216:        if crate::steps::development_record_steps::is_record_dir(root, &path) {
        tests/steps/serve_extended_rules_steps.rs:490:                || crate::steps::development_record_steps::is_record_dir(root, &path)

        $ cargo fmt --all -- --check                                    EXIT 0
        $ cargo clippy --locked --all-targets -- -D warnings
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.65s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A refused command outside a Givn project creates no record"
        Feature: Development Record
          Scenario: A refused command outside a Givn project creates no record
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init`
           ✔  Then the command exits non-zero
           ✔  And the directory "givn" does not exist
           ✔  And the project root holds no development record
           ✔  When the consumer runs `givn init --no-addons --agents opencode`
           ✔  Then the command exits 0
           ✔  And the directory "givn" exists
           ✔  And the project root holds a development record
        [Summary]
        1 feature
        1 scenario (1 passed)
        9 steps (9 passed)
        EXIT 0

        The implemented delta scope (every scenario whose own `@wip` has been
        removed) in one process:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        2 features
        13 scenarios (13 passed)
        71 steps (71 passed)
        EXIT 0

        The asked-for delta scope as the project runner defines it
        (`./run-tests.sh --non-e2e` = `not @e2e`, which **selects** the
        not-yet-implemented `@wip` scenarios; the runner's strict mode then fails
        them at their undefined steps):

        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
        FAIL [   0.800s] ( 9/35) inspect-development-history/agent-activity-reporting.feature::Reported token usage is recorded with the model that produced it
        …
        warning: 23/35 tests were not run due to test failure
        error: test run failed
        EXIT 100

        Run to completion with `--no-fail-fast`, the same scope is exactly the
        implemented set green plus the `@wip` set red:
        $ GIVN_CUCUMBER_TAGS="not @e2e" GIVN_FEATURES=givn/changes/development-timeline/specs cargo nextest run --no-tests=warn --locked --no-fail-fast --test features_runner
        13 PASS (the 12 implemented non-e2e scenarios plus this scenario:
        `fragments/adr-qualification-gate.feature::The migration preserves
        independent architecture history across three proposed decisions`, the ten
        other `development-record.feature` scenarios, `Reading the development
        record does not extend it`, `The record stays out of the repository's own
        history`, `A Fossil project ignores the record through its own ignore
        file`, `A refused command outside a Givn project creates no record`, …)
        22 FAIL (every non-e2e `@wip` scenario of Tasks 12–33, each at its own
        undefined step — the tasks not yet implemented by this change)
        EXIT 100
        → the scope cannot exit 0 mid-change: `--non-e2e` does not exclude
        `@wip`, and strict mode fails an undefined step by design. The
        implemented-scope run above is this task's green evidence.
        ```
  - [x] **COMMIT** — `cdd18bd900b76a0aa8730c9b7cef37f11a0c84b6` — `feat(development-record): <release note>` with
        the scenario title `A refused command outside a Givn project creates no
        record` in the body. Record the revision identifier.
        ```
        $ givn commit -m "feat(development-record): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A refused command outside a Givn project creates no record"
        [15:06:56] committed revision on the Git backend
        $ git rev-parse HEAD
        cdd18bd900b76a0aa8730c9b7cef37f11a0c84b6

        One atomic revision: RED+GREEN+REFACTOR together — the delta spec (this
        scenario's `@wip` removed), the engagement predicate, the three new
        steps and the shared record-path exclusion, the two project-state helper
        exclusions, and this file's Task 11a record. `givn commit` takes one
        `--message` (a repeated `-m` is refused by the binary), so the release
        note is the message subject and the scenario title is the message body,
        the form every scenario revision of this change uses.
        ```

        The delta scope was then re-run on the committed state:
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
        FAIL [   0.763s] ( 7/35) inspect-development-history/agent-activity-reporting.feature::The intake keeps the acceptable events of a payload with a rejected event
        …
        warning: 23/35 tests were not run due to test failure
        error: test run failed
        EXIT 100

        $ GIVN_CUCUMBER_TAGS="not @e2e" GIVN_FEATURES=givn/changes/development-timeline/specs cargo nextest run --no-tests=warn --locked --no-fail-fast --test features_runner
        35 selected; 13 PASS (the 12 implemented non-e2e scenarios plus this
        one) and 22 FAIL (every non-e2e `@wip` scenario of Tasks 12–33, each at
        its own undefined step, which strict mode fails by design until those
        tasks land)                                             EXIT 100

        → `./run-tests.sh --non-e2e` is `not @e2e`: it *selects* the `@wip`
        scenarios rather than skipping them, so the scope cannot report exit 0
        while Tasks 12–33 are unimplemented. The implemented set is green in one
        process:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        2 features
        13 scenarios (13 passed)
        71 steps (71 passed)
        EXIT 0
        ```

## Task 12 — `A reported tool call is recorded for its scope` (`agent-activity-reporting`, added)

- [x] **Scenario: `A reported tool call is recorded for its scope`**
  - Design constraints:
    - Intake: `givn history ingest` reads exactly one JSON object from stdin
      (`schema`, `reporter`, `session`, `change`, `topic`, `events`). A present
      `change` overrides the binary's own resolution (a named scope that does not
      exist is a per-event rejection, never a silent fallback); `source` for
      every ingested event is `agent`.
    - Entry shape for a tool event: `kind: "tool"`, `name` the harness tool name,
      `label` the bounded subject, `duration_ms` the measured duration (the
      harness's own where it exists, otherwise the reporter's own stamps),
      `outcome` mapped from the harness status, `parent` the enclosing turn when
      the harness reports one and that id was minted by the same process.
    - Per-kind validation lives in `src/cli/history.rs` (`IngestedEvent`): a
      required field missing for the event's kind rejects that event alone.
    - Regular scenarios drive the intake with a payload on stdin (the machine
      contract); the harnesses' own reporter entry points are the `@e2e` tier
      (Task E4) — never fake a harness here.
    - Fixture: change `demo` with `proposal.md`; the payload names the change and
      one `tool` event; steps in
      `tests/steps/agent_activity_reporting_steps.rs`, payload builder in
      `tests/steps/development_timeline_support.rs`.
    - Assertion shape: exactly one `agent`-sourced entry for the tool call, naming
      the tool and carrying the measured duration.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported tool call is recorded for its scope"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported tool call is recorded for its scope"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.02s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A reported tool call is recorded for its scope
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the agent reports a completed tool call for "demo"
           ✘  Then the development record holds one agent entry for that tool call
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:10:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (2329307) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound (`the development
        record holds one agent entry for that tool call` matches no function; the
        scenario stops there, so `that entry names the tool and its measured
        duration` never runs). The `When` step drove the real binary's
        `history ingest` entry point; the subcommand is not wired yet, which the
        step records without asserting — the RED signal is the undefined Then.
        ```
  - [x] **GREEN** — implement `givn history ingest` (payload read, per-event
        validation, append). Files: `src/cli/history.rs`, `src/history.rs`,
        `src/main.rs` (dispatch), `tests/steps/agent_activity_reporting_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.40s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported tool call is recorded for its scope"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.93s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A reported tool call is recorded for its scope
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the agent reports a completed tool call for "demo"
           ✔  Then the development record holds one agent entry for that tool call
           ✔  And that entry names the tool and its measured duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Production files:
        - `src/cli/history.rs` — `IntakePayload` (the one stdin read),
          `IngestedEvent` with the per-kind validation table (`kind` must be a
          mapped kind, `at` must parse as RFC 3339 UTC, a `tool` event must name
          its tool), the `tool` mapping to an `agent`-sourced entry, the
          payload-scope resolution (named `change`/`topic` override, missing
          named scope = per-event rejection, otherwise `resolve_scope`), the
          per-event stderr rejection naming, and the best-effort append.
        - `src/history.rs` — `HistoryEntry::reported` (the agent-source entry
          shape) and `parse_rfc3339_ms` + `days_from_civil`/`days_in_month` (the
          record's one timestamp parser, which is also the `at` validation).
        - `src/main.rs` — the `history ingest` subcommand, its dispatch, and the
          recorder engagement rule (the read view still records nothing; the
          intake records normally).
        - `tests/steps/agent_activity_reporting_steps.rs` — the `When` and the
          two `Then` steps, asserting against the payload's own values.
        - `tests/steps/development_timeline_support.rs` — `intake_payload` and
          `report_intake` (the real binary's stdin machine contract).
        ```
  - [x] **REFACTOR** — one event-mapping site per kind, no per-caller field
        copying; re-run → still exit 0. Paste it.
        ```
        The accept-or-reject gate moved into one function, so both gates an
        event passes (the payload's scope, the event's own validation) live on
        one path into the append:

        $ cargo fmt --all && cargo fmt --all -- --check
        FMT_CLEAN
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.62s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported tool call is recorded for its scope"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.99s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A reported tool call is recorded for its scope
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the agent reports a completed tool call for "demo"
           ✔  Then the development record holds one agent entry for that tool call
           ✔  And that entry names the tool and its measured duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `453ce716e2c04eb76f1ca2b713fbd226e1e0543d` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `A reported tool call is recorded for its scope`
        in the body. Record the revision identifier.
        ```
        One `givn commit` with the release note as the message subject and the
        scenario title as the body (`-m` takes the whole message; a second `-m`
        is refused by the command's single-value argument):

        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
            ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A reported tool call is recorded for its scope"
        [15:20:00] committed revision on the Git backend

        $ git show --stat 453ce716e2c04eb76f1ca2b713fbd226e1e0543d
        commit 453ce716e2c04eb76f1ca2b713fbd226e1e0543d
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:20:00 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            A reported tool call is recorded for its scope

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 105 +++++++++-
         src/cli/history.rs                                 | 222 ++++++++++++++++++++-
         src/history.rs                                     | 102 ++++++++++
         src/main.rs                                        |  45 +++--
         tests/steps/agent_activity_reporting_steps.rs      | 144 ++++++++++++-
         tests/steps/development_timeline_support.rs        |  22 ++
         7 files changed, 609 insertions(+), 33 deletions(-)
        ```
        (The revision carries production code, the steps, and the spec's `@wip`
        removal — not a spec-only revision.)

## Task 13 — `A subagent's activity is attributed to that subagent` (`agent-activity-reporting`, added)

- [x] **Scenario: `A subagent's activity is attributed to that subagent`**
  - Design constraints:
    - Entry shape: `kind: "subagent"`, the subagent's identity in the event
      (`{"kind":"subagent","at":…,"agent":…,"subagent":…,"status":…,"duration_ms":…}`
      — `data.subagent` carries the harness's own identity, e.g.
      `Architect.Scout`), a measured `duration_ms` spanning start..end, and
      `outcome` mapped from `status` (`completed` → `ok`).
    - Nesting: a subagent reported inside a turn carries `parent` = that turn's
      id; a parent is only ever an id minted by the same process, so the link
      always resolves in the store.
    - The subagent must remain an identifiable span, never collapsed into its
      parent turn's duration — the report's fan-out view (Task 30) reads these
      entries.
    - Fixture: change `demo` with reported agent activity; one start and one
      completion event for the same subagent.
    - Assertion shape: exactly one entry for that subagent; it names the subagent
      and its duration covers the reported start and end.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A subagent's activity is attributed to that subagent"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A subagent's activity is attributed to that subagent"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.24s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A subagent's activity is attributed to that subagent
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a subagent that started and completed
           ✘  Then the record holds one entry for that subagent
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:17:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2348083) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound (the first
        Then matches no function; the scenario stops there, so `that entry names
        the subagent and spans its start and its end` never runs). The `Given`
        and the `When` drive the real fixture: `givn new demo` plus a reported
        tool call, then the reported start/completion pair handed to
        `givn history ingest`. The intake still rejects `subagent` as an unknown
        kind at this point, which the `When` records without asserting — the RED
        signal is the undefined Then.
        ```
  - [x] **GREEN** — implement the subagent event kind and its parent link.
        Files: `src/cli/history.rs`, `src/history.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.07s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A subagent's activity is attributed to that subagent"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 10.81s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A subagent's activity is attributed to that subagent
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a subagent that started and completed
           ✔  Then the record holds one entry for that subagent
           ✔  And that entry names the subagent and spans its start and its end
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Regression check — the previous scenario still passes with the reworked
        mapping:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported tool call is recorded for its scope"
        … 5 steps (5 passed), EXIT=0

        Production files:
        - `src/cli/history.rs` — the `subagent` validation row (`subagent` is the
          event's identity) and its mapper: a start opens a span keyed by the
          harness's own identity, its terminal status closes it into exactly one
          entry whose `duration_ms` is the harness's own where the harness
          reports one and otherwise the measured span between the two events,
          whose `outcome` is mapped from `status` (`completed` → `ok`), and whose
          `data` carries `subagent` (the harness's identity) plus `agent`. The
          entry is nested in the turn enclosing the flush when the payload
          reported one, through the batch's parent plan. Also the batch plan
          (`IntakeBatch`/`SubagentStart`), `accept_event` writing into it, and
          `ingest` appending through `append_linked`.
        - `src/history.rs` — `HistoryStore::append_linked` (resolves each entry's
          `parent` from the batch index of the entry it is nested in, both ids
          minted by the same append under the store's lock; `append` delegates)
          and `reported_duration_ms` (the measured span between two reported
          timestamps).
        - `tests/steps/agent_activity_reporting_steps.rs` — the fixture's `Given`
          (change `demo` with one reported tool call), the `When` (the reported
          start/completion pair for `Architect.Scout`), and both `Then` steps.
        Note on the parent link: the rule is implemented as the batch's parent
        plan (`append_linked`) threaded to the enclosing turn; this fixture
        reports no turn, so the condition "reported inside a turn" does not fire
        here — the turn kind that sets the batch's turn arrives with Task 15's
        turn mapping.
        ```
  - [x] **REFACTOR** — express start/end pairing once (shared with the question
        wait path where the shape is identical); re-run → still exit 0. Paste it.
        ```
        The pairing is now one mechanism — `OpenSpans::open`/`close`/`take_unclosed`
        keyed by the harness's own call id, with `SpanStart` carrying the fields
        only the opening event knows — used by the subagent mapper instead of an
        inline map, so the identical shape (a start event plus a completion event
        become one entry) has a single expression for both paired kinds.

        $ cargo fmt --all -- --check
        FMT_CLEAN
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.79s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A subagent's activity is attributed to that subagent"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 8.66s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A subagent's activity is attributed to that subagent
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a subagent that started and completed
           ✔  Then the record holds one entry for that subagent
           ✔  And that entry names the subagent and spans its start and its end
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `1574c66aa30c758f02fbd816f853966526aa4238` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `A subagent's activity is attributed to that
        subagent` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 … ./target/debug/givn commit -m "feat(agent-activity-reporting): <release note verbatim>\n\nA subagent's activity is attributed to that subagent"
        [15:24:56] committed revision on the Git backend

        $ git show --stat 1574c66aa30c758f02fbd816f853966526aa4238
        commit 1574c66aa30c758f02fbd816f853966526aa4238
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:24:56 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            A subagent's activity is attributed to that subagent

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 117 +++++++++-
         src/cli/history.rs                                 | 245 +++++++++++++++++----
         src/history.rs                                     |  46 +++-
         tests/steps/agent_activity_reporting_steps.rs      | 218 ++++++++++++++++--
         5 files changed, 551 insertions(+), 77 deletions(-)
        ```

## Task 14 — `A question wait is recorded with its duration` (`agent-activity-reporting`, added)

- [x] **Scenario: `A question wait is recorded with its duration`**
  - Design constraints:
    - Entry shape: `kind: "question"`, `duration_ms` the wait duration (the
      harness's own where it exists, else the reporter's stamps), `label` the
      question's **first line only** — the complete question text is never
      stored (D2). Example payload event:
      `{"kind":"question","at":…,"label":"Which store should the record use?","duration_ms":480000}`.
    - A wait must be its own span, never merged into the surrounding turn, so the
      report can total waiting time (Task 26) — the requester's "waiting is
      indistinguishable from working" problem.
    - The wait's `outcome` is `ok` when the harness reports the resumed action.
    - Fixture: change `demo` with reported agent activity; the agent asks and
      resumes; the wait duration is a value the payload provides.
    - Assertion shape: one entry for the question wait carrying the reported
      duration and a bounded label whose content is the question's first line.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A question wait is recorded with its duration"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A question wait is recorded with its duration"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.65s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A question wait is recorded with its duration
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports that it asked a question and resumed
           ✘  Then the record holds one entry for the question wait
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:24:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2360142) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound (the first
        Then matches no function; the scenario stops there, so `that entry
        carries the wait duration` never runs). The `Given` and the `When` drive
        the real fixture (a change with one reported tool call, then the reported
        question wait handed to `givn history ingest`); the intake still rejects
        `question` as an unknown kind at this point, which the `When` records
        without asserting — the RED signal is the undefined Then.
        ```
  - [x] **GREEN** — implement the question event kind and the first-line label
        rule. Files: `src/cli/history.rs`, `src/history.rs` (`bound_label`),
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.35s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A question wait is recorded with its duration"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.46s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A question wait is recorded with its duration
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports that it asked a question and resumed
           ✔  Then the record holds one entry for the question wait
           ✔  And that entry carries the wait duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Production files:
        - `src/cli/history.rs` — the `question` validation row (`label` is the
          event's identity) and its mapper: one `question` entry carrying the
          reported wait duration, the outcome mapped from the reported resumed
          status, and the label through the record's label path; the entry is its
          own span (nested in the enclosing turn when the payload reported one,
          never merged into it).
        - `src/history.rs` — `bound_question_label`: the question's first line
          only (the complete question text is never stored, D2), then the same
          `bound_label` bound as every other label.
        - `tests/steps/agent_activity_reporting_steps.rs` — the two `Then` steps
          (the recorded wait's duration, and a label equal to the payload's own
          first line with the complete text's later lines asserted absent).
        ```
  - [x] **REFACTOR** — keep the first-line rule in the same bounded-label path as
        every other label; re-run → still exit 0. Paste it.
        ```
        The per-kind label rule now lives in the one function every reported
        subject goes through — `reported_label(kind, subject)` bounds the subject
        through `bound_label` and keeps a question's first line only — and both
        mappers (tool, question) call it, so no caller bounds a label itself.

        $ cargo fmt --all -- --check
        FMT_CLEAN
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.94s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A question wait is recorded with its duration"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.11s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A question wait is recorded with its duration
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports that it asked a question and resumed
           ✔  Then the record holds one entry for the question wait
           ✔  And that entry carries the wait duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `55f8bb5799fc5814e2a412ece70e172b478d98e5` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `A question wait is recorded with its duration`
        in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 … ./target/debug/givn commit -m "feat(agent-activity-reporting): <release note verbatim>\n\nA question wait is recorded with its duration"
        [15:26:51] committed revision on the Git backend

        $ git show --stat 55f8bb5799fc5814e2a412ece70e172b478d98e5
        commit 55f8bb5799fc5814e2a412ece70e172b478d98e5
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:26:51 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            A question wait is recorded with its duration

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         |  98 ++++++++++++++++++++--
         src/cli/history.rs                                 |  25 +++++-
         src/history.rs                                     |  12 +++
         tests/steps/agent_activity_reporting_steps.rs      |  93 ++++++++++++++++++++
         5 files changed, 221 insertions(+), 9 deletions(-)
        ```

## Task 15 — `Reported token usage is recorded with the model that produced it` (`agent-activity-reporting`, added)

- [x] **Scenario: `Reported token usage is recorded with the model that produced it`**
  - Design constraints:
    - Entry shape: `kind: "usage"` carrying
      `data: {input, output, cache_read, cache_write, total_tokens, model, subagent}`.
      A turn event may carry the usage inline
      (`{"kind":"turn", …, "usage": {…}}`); the appended usage entry is parented
      to that turn and keeps the model name verbatim (e.g.
      `deepseek/deepseek-flash`), never normalised or truncated.
    - Usage is recorded per turn and per subagent, so the report can total it
      overall, per model, and per subagent (Task 27).
    - Fixture: change `demo` with reported agent activity; one completed turn
      carrying input/output/cache counts and a model.
    - Assertion shape: the turn's usage is present with the reported counts and
      the model that produced it.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Reported token usage is recorded with the model that produced it"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Reported token usage is recorded with the model that produced it"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.22s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Reported token usage is recorded with the model that produced it
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a completed turn with token usage
           ✘  Then the record holds that turn's token usage
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:31:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2368915) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound (the first
        Then matches no function; the scenario stops there, so `the usage names
        the model that produced it` never runs). The `Given` and the `When` drive
        the real fixture (a change with one reported tool call, then the reported
        completed turn with its inline usage handed to `givn history ingest`);
        the intake still rejects `turn` as an unknown kind at this point, which
        the `When` records without asserting — the RED signal is the undefined
        Then.
        ```
  - [x] **GREEN** — implement the usage event kind and the turn linkage. Files:
        `src/cli/history.rs`, `src/history.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.54s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Reported token usage is recorded with the model that produced it"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 10.74s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Reported token usage is recorded with the model that produced it
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a completed turn with token usage
           ✔  Then the record holds that turn's token usage
           ✔  And the usage names the model that produced it
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Regression check — the three earlier scenarios still pass:
        $ for s in "A reported tool call is recorded for its scope" "A subagent's activity is attributed to that subagent" "A question wait is recorded with its duration"; do … done
        A reported tool call is recorded for its scope => EXIT=0
        A subagent's activity is attributed to that subagent => EXIT=0
        A question wait is recorded with its duration => EXIT=0

        Production files:
        - `src/cli/history.rs` — the `turn` validation row (usage is optional on
          a turn) and its mapper: the turn entry (name `turn <index>` when the
          harness reports one, the measured duration, the mapped outcome), the
          batch's enclosing turn set to it, and — when the harness reported
          usage inline — one `usage` entry parented to the turn's id, its `data`
          carrying every reported count, the model verbatim, and the subagent
          session where reported. `Usage` is the reported-usage struct, and
          `IntakeBatch::push` now returns the plan index so a span nests in the
          entry just planned.
        - `tests/steps/agent_activity_reporting_steps.rs` — the two `Then` steps
          (the usage entry's counts and its parent link to the turn; the model
          kept verbatim).
        Note: `src/history.rs` is unchanged by this scenario — the parent link is
        `append_linked` from Task 13.
        ```
  - [x] **REFACTOR** — one usage struct used by every harness path; re-run →
        still exit 0. Paste it.
        ```
        The usage struct now owns the record rendering of its own counts:
        `Usage::data(model, subagent)` builds the usage entry's `data` set, and
        the free `usage_data` function is gone — any harness path that produces
        a `Usage` renders it the same way.

        $ cargo fmt --all -- --check
        FMT_CLEAN
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.37s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Reported token usage is recorded with the model that produced it"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.59s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Reported token usage is recorded with the model that produced it
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a completed turn with token usage
           ✔  Then the record holds that turn's token usage
           ✔  And the usage names the model that produced it
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `22219c9d617dd3a823da7ce096eca53401ea5647` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `Reported token usage is recorded with the model
        that produced it` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 … ./target/debug/givn commit -m "feat(agent-activity-reporting): <release note verbatim>\n\nReported token usage is recorded with the model that produced it"
        [15:29:18] committed revision on the Git backend

        $ git show --stat 22219c9d617dd3a823da7ce096eca53401ea5647
        commit 22219c9d617dd3a823da7ce096eca53401ea5647
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:29:18 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            Reported token usage is recorded with the model that produced it

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 107 ++++++++++++++++++--
         src/cli/history.rs                                 | 108 ++++++++++++++++++++-
         tests/steps/agent_activity_reporting_steps.rs      | 104 ++++++++++++++++++++
         4 files changed, 312 insertions(+), 9 deletions(-)
        ```

## Task 16 — `Absent token usage is recorded as unavailable` (`agent-activity-reporting`, added)

- [x] **Scenario: `Absent token usage is recorded as unavailable`**
  - Design constraints:
    - Gap-honesty rule: missing data is recorded as missing, never as zero. A
      turn event without a `usage` object leaves the turn entry intact and marks
      the turn's usage unavailable — never a zero-filled usage entry, and never
      a fabricated model.
    - The report side of the same rule: the summary states how many turns were
      unmeasured rather than summing them as zero (`design.md` §What the report
      shows when a harness cannot deliver a signal). That rendering is asserted
      by Task 27 / Task E5, not here.
    - Fixture: change `demo` with reported agent activity; one completed turn
      with no usage.
    - Assertion shape: the turn is still recorded, and its usage is marked
      unavailable (an explicit marker the read surface exposes — the design's
      own word is "unavailable"), with no zero-valued counts anywhere for it.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Absent token usage is recorded as unavailable"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Absent token usage is recorded as unavailable"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.81s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Absent token usage is recorded as unavailable
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a completed turn without token usage
           ✘  Then the record marks that turn's token usage as unavailable
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:38:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2384114) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound (the first
        Then matches no function; the scenario stops there, so `the turn is still
        recorded` never runs). The `Given` and the `When` drive the real fixture
        (a change with one reported tool call, then the reported completed turn
        without any usage handed to `givn history ingest`); at this point the
        turn kind is mapped but records no marker, which the `When` records
        without asserting — the RED signal is the undefined Then.
        ```
  - [x] **GREEN** — implement the unavailable marker on the turn and keep the
        turn entry. Files: `src/cli/history.rs`, `src/history.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.09s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Absent token usage is recorded as unavailable"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.56s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Absent token usage is recorded as unavailable
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a completed turn without token usage
           ✔  Then the record marks that turn's token usage as unavailable
           ✔  And the turn is still recorded
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Production files:
        - `src/cli/history.rs` — the turn mapper marks a turn whose event carries
          no `usage` with `data.usage: "unavailable"` while keeping the turn
          entry itself (name, measured duration, mapped outcome); no usage entry
          is appended and no model is stored for it.
        - `tests/steps/agent_activity_reporting_steps.rs` — the two `Then` steps:
          the explicit `unavailable` marker (with a scan asserting no usage entry
          exists, no zero-valued count is recorded anywhere, and no model was
          invented) and the turn's own identity (the reported `at`, duration and
          outcome, `agent` source, the change scope).
        Note: `src/history.rs` is unchanged by this scenario's GREEN; its
        unmeasured-signal vocabulary is unified by the REFACTOR below.
        ```
  - [x] **REFACTOR** — one "unmeasured" representation shared with the unmeasured
        duration and unknown outcome states; re-run → still exit 0. Paste it.
        ```
        The record's three absences now have one home and one vocabulary
        (`src/history.rs`): an unmeasured duration is absent (no `duration_ms`),
        an unreported outcome is `OUTCOME_UNKNOWN`, and a turn the harness never
        measured carries `data.usage: USAGE_UNAVAILABLE`. The entry constructors,
        the span guard, the intake's outcome fallback and the turn marker all use
        those constants instead of their own literals.

        $ cargo fmt --all -- --check
        FMT_CLEAN
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.78s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Absent token usage is recorded as unavailable"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Absent token usage is recorded as unavailable
           ✔  Given a change "demo" with reported agent activity
           ✔  When the agent reports a completed turn without token usage
           ✔  Then the record marks that turn's token usage as unavailable
           ✔  And the turn is still recorded
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Regression check — all five bundle scenarios still green:
        $ for s in "A reported tool call is recorded for its scope" …; do …; done
        A reported tool call is recorded for its scope => EXIT=0
        A subagent's activity is attributed to that subagent => EXIT=0
        A question wait is recorded with its duration => EXIT=0
        Reported token usage is recorded with the model that produced it => EXIT=0
        Absent token usage is recorded as unavailable => EXIT=0
        ```
  - [x] **COMMIT** — `82a4ed81882248d06ce2658eb7faed02819a7601` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `Absent token usage is recorded as unavailable`
        in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 … ./target/debug/givn commit -m "feat(agent-activity-reporting): <release note verbatim>\n\nAbsent token usage is recorded as unavailable"
        [15:31:28] committed revision on the Git backend

        $ git show --stat 82a4ed81882248d06ce2658eb7faed02819a7601
        commit 82a4ed81882248d06ce2658eb7faed02819a7601
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:31:28 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            Absent token usage is recorded as unavailable

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 106 ++++++++++++++++++--
         src/cli/history.rs                                 |  19 +++-
         src/history.rs                                     |  18 +++-
         tests/steps/agent_activity_reporting_steps.rs      | 110 +++++++++++++++++++++
         5 files changed, 240 insertions(+), 15 deletions(-)
        ```

## Task 17 — `A reported event that does not match the intake contract is rejected without corrupting the record` (`agent-activity-reporting`, added)

- [x] **Scenario: `A reported event that does not match the intake contract is rejected without corrupting the record`**
  - Design constraints:
    - Per-event validation and rejection: an event whose `kind` is unknown,
      whose `at` is unparsable, or that lacks a required field for its kind is
      rejected **individually** — named on stderr — while the remaining events
      are still appended. The reporter must never be able to corrupt or block the
      record.
    - "The previously recorded history is unchanged" is the append-only property
      observed at the *content* level: the seeded lines are byte-identical after
      the rejected payload.
    - Rejection is a warning path, not silent: the named event must be legible
      enough for the reporter's author (kind and/or index), because the report
      later surfaces the dropped count (design's record-gap notice).
    - Fixture: change `demo` with a seeded record (support-file builder) and one
      contract-violating event (for example an unknown `kind` or an unparsable
      `at`).
    - Assertion shape: stderr names the rejected event; the seed lines are
      unchanged.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported event that does not match the intake contract is rejected without corrupting the record"`
        → must exit non-zero. Paste it.
        ```
        The first run failed on the fixture itself: this scenario's first Given
        is the change, and the shared `a change "demo" with a recorded history`
        step expected a project root. The Given now brings up the init root when
        the scenario did not name one (the rule the reported-activity Given
        already applies, `tests/steps/development_record_steps.rs`), and the
        fixture's `When` was written — a real setup action with its own
        assertions, never a stub — so the RED lands on the new behaviour's
        assertion step, which is the failure the task predicts:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported event that does not match the intake contract is rejected without corrupting the record"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.60s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A reported event that does not match the intake contract is rejected without corrupting the record
           ✔  Given a change "demo" with a recorded history
           ✔  When the agent reports an event that does not match the intake contract
           ✘  Then the intake names the rejected event
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:45:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2445051) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        (The first run of this scenario, before that fixture existed:)
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported event that does not match the intake contract is rejected without corrupting the record"
        Feature: Agent Activity Reporting
          Scenario: A reported event that does not match the intake contract is rejected without corrupting the record
           ✘  Given a change "demo" with a recorded history
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:43:5
              Matched: tests/steps/development_record_steps.rs:655:1
              Step panicked. Captured output: a project root is required first
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        EXIT=101

        RED category: undefined step — both Then steps are unbound (`the intake
        names the rejected event` matches no function; the scenario stops there,
        so `the previously recorded history is unchanged` never runs). The
        `Given` and the `When` drive the real fixture — a change `demo` with a
        recorded history (`givn new` + `givn status`), then one contract-
        violating event (an unmapped `kind`) handed to the real
        `givn history ingest` on stdin — and the `When` performs that setup
        without asserting, so the RED signal is the undefined Then. The
        production path the two Then steps assert already exists from Task 12
        (`IngestedEvent::validate`, the intake's one validation table, reached
        through `accept_event`), so the discrimination of both new assertions is
        proven in the GREEN block below by a controlled omission in the fixture
        (ORACLE_RED), restored immediately.
        ```
  - [x] **GREEN** — implement per-kind validation with named rejections. Files:
        `src/cli/history.rs`, `tests/steps/agent_activity_reporting_steps.rs`,
        `tests/steps/development_timeline_support.rs` (seeded-record builder).
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.18s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported event that does not match the intake contract is rejected without corrupting the record"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.21s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A reported event that does not match the intake contract is rejected without corrupting the record
           ✔  Given a change "demo" with a recorded history
           ✔  When the agent reports an event that does not match the intake contract
           ✔  Then the intake names the rejected event
           ✔  And the previously recorded history is unchanged
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Production files: **none for this scenario** — investigated, not
        assumed. The contract the scenario asserts is already carried by the
        intake Task 12 built, and this scenario drives exactly that path: the one
        validation table per kind lives in `src/cli/history.rs::IngestedEvent::validate`
        (unparsable `at`, unknown `kind`, a required field missing for its kind),
        it is reached through the one gate into the append path
        (`fn accept_event`), `fn warn_rejected` names the rejected event on
        stderr by its 1-based payload position and its kind, and a payload whose
        every event was rejected returns before the store is touched
        (`if entries.is_empty() { return Ok(()); }`). `git log -S "unknown kind"
        -- src/cli/history.rs` → `453ce716` (Task 12) is where the table was
        written. No production line was added, so both new assertions were
        proven load-bearing below.

        Fixture and steps (`tests/steps/agent_activity_reporting_steps.rs`):
        - `When the agent reports an event that does not match the intake
          contract` — snapshots the record's bytes, then hands
          `givn history ingest` (the real binary, payload on stdin) a payload
          whose only event carries `kind: "teleport"`. The snapshot, the payload
          and the violating event's position are kept in the capability's state,
          so the assertions read what was really sent.
        - `Then the intake names the rejected event` — exactly one `warning:`
          line naming the rejection, carrying the event's 1-based position
          (`event 1`) and its kind, and no `error:` line on stderr.
        - `Then the previously recorded history is unchanged` — the snapshot
          bytes are still the store's first bytes, byte for byte; everything the
          payload appended is `givn`-sourced (the intake's own command entry,
          which the design records normally); and the whole record holds no
          agent-sourced entry, so the rejected event contributed nothing.
        `tests/steps/development_timeline_support.rs` — `entries_of`, the
        byte-region parser `record_entries` now shares, so a scenario can assert
        the appended region alone instead of a re-serialized view.
        `tests/steps/development_record_steps.rs` — the shared recorded-history
        Given brings up its init root when the scenario's first Given is the
        change (three lines, the same rule the reported-activity Given already
        applied).
        Fixture note: this scenario's "seeded record" is the shared recorded-
        history fixture (real `givn new` + `givn status` runs); a builder that
        writes chosen record lines by hand is not needed by any assertion here
        and arrives with the report scenarios that need chosen durations
        (Tasks 25+), never as unused scaffolding.

        Discrimination (ORACLE_RED) — with production already holding, a
        controlled omission in the isolated fixture proves each assertion fails
        without its input property, and the fixture is restored immediately:

        (a) the violating event made acceptable (a mapped `tool` event, so the
            payload is read and nothing is rejected):
        ✘  Then the intake names the rejected event
           Step panicked. Captured output: assertion `left == right` failed: expected the payload's one contract-violating event to be named on stderr: []
             left: 0
            right: 1
        EXIT=101

        (b) an acceptable event added beside the violating one (the violating
            event first, so it is still rejected and named):
        ✔  Then the intake names the rejected event
        ✘  And the previously recorded history is unchanged
           Step panicked. Captured output: assertion `left == right` failed: the rejected event was recorded: {"at":"2026-10-16T09:12:44.321Z","duration_ms":1240,"id":4,"kind":"tool","label":"cargo test --locked --test features_runner","name":"bash","outcome":"ok","schema":1,"scope":{"id":"demo","kind":"change"},"source":"agent"}
             left: String("agent")
            right: String("givn")
        EXIT=101

        With the fixture restored the canonical run is green again (the run
        pasted above, EXIT 0): the record really is untouched and the rejection
        really is named.
        ```
  - [x] **REFACTOR** — one validation table per kind, no ad-hoc checks in the
        append path; re-run → still exit 0. Paste it.
        ```
        The intake keeps exactly one validation table (`IngestedEvent::validate`
        in `src/cli/history.rs`, one match arm per kind) and one gate into the
        append path (`accept_event`), so there is no ad-hoc check in the append
        path to remove and no production change was needed for this scenario.
        The fixture side now reaches the capability's world slot through one
        accessor instead of nine copies of the same `get_or_insert_with`:

        $ cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.90s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A reported event that does not match the intake contract is rejected without corrupting the record"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.00s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A reported event that does not match the intake contract is rejected without corrupting the record
           ✔  Given a change "demo" with a recorded history
           ✔  When the agent reports an event that does not match the intake contract
           ✔  Then the intake names the rejected event
           ✔  And the previously recorded history is unchanged
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0
        ```
  - [x] **COMMIT** — `049e5664fa18768140ad5fdbd3552ff5ef9a2902` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `A reported event that does not match the intake
        contract is rejected without corrupting the record` in the body. Record
        the revision identifier.
        ```
        One `givn commit` with the release note as the message subject and the
        scenario title as the body:

        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A reported event that does not match the intake contract is rejected without corrupting the record"
        [15:43:10] committed revision on the Git backend

        $ git show --stat 049e5664fa18768140ad5fdbd3552ff5ef9a2902
        commit 049e5664fa18768140ad5fdbd3552ff5ef9a2902
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:43:10 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            A reported event that does not match the intake contract is rejected without corrupting the record

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 186 ++++++++++++++++++-
         tests/steps/agent_activity_reporting_steps.rs      | 198 ++++++++++++++++++---
         tests/steps/development_record_steps.rs            |   7 +
         tests/steps/development_timeline_support.rs        |  10 +-
         5 files changed, 365 insertions(+), 38 deletions(-)

        One atomic revision for the scenario: the delta spec (`@wip` removed from
        this scenario only), the fixture steps, and this file's Task 17 record
        together.
        ```

## Task 18 — `A rejected report never fails the agent action it reports` (`agent-activity-reporting`, added)

- [x] **Scenario: `A rejected report never fails the agent action it reports`**
  - Design constraints:
    - Best-effort guarantee for the agent path: a record that cannot be written
      warns and still exits `0` — the reporter's caller must not fail because
      Givn's disk was full (design's intake exit-code rule). A non-zero exit is a
      hook error for Claude Code, and exit `2` blocks the tool, so the failure
      discipline is explicit: the mapper always exits `0`.
    - Fixture: change `demo` with a recorded history plus the unwritable store
      (the Task 3 fixture, reused from the support file), then a completed tool
      call reported through the intake.
    - Assertion shape: the intake reports no failure to the agent — exit code
      `0`, with the write failure visible as a warning only.
    - Do not turn this into a retry/queue feature: there is no retry path, no
      spool file, no second write location.
    - RED expected failure: the no-failure Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A rejected report never fails the agent action it reports"`
        → must exit non-zero. Paste it.
        ```
        (The first run, before the fixture's `When` existed:)
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A rejected report never fails the agent action it reports"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A rejected report never fails the agent action it reports
           ✔  Given a change "demo" with a recorded history
           ✔  And the development record cannot be written
           ✘  When the agent reports a completed tool call
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:52:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2459105) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        The `When` — a real setup action, never a stub: the fixture's completed
        tool call handed to the real `givn history ingest` — was then written so
        the RED lands on the new behaviour's assertion step, which is the failure
        the task predicts:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A rejected report never fails the agent action it reports"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.12s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A rejected report never fails the agent action it reports
           ✔  Given a change "demo" with a recorded history
           ✔  And the development record cannot be written
           ✔  When the agent reports a completed tool call
           ✘  Then the intake reports no failure to the agent
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:53:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (2460017) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the no-failure Then step is unbound
        (`the intake reports no failure to the agent` matches no function). The
        scenario's own fixture really ran: the shared recorded-history Given built
        the change and its record, the unwritable-store Given replaced the store
        path with a directory (a real `EISDIR` on the real write path), and the
        `When` handed the real intake the fixture's completed tool call — so the
        RED is the new assertion, not a vacuous setup. The behaviour the Then
        asserts already exists from Task 12/16's best-effort append, so both
        halves of the assertion are proven load-bearing in the GREEN block below
        with two controlled omissions in the isolated fixture (ORACLE_RED),
        restored immediately.
        ```
  - [x] **GREEN** — route the ingest write path through the same best-effort
        recorder. Files: `src/cli/history.rs`, `src/history.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A rejected report never fails the agent action it reports"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.94s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A rejected report never fails the agent action it reports
           ✔  Given a change "demo" with a recorded history
           ✔  And the development record cannot be written
           ✔  When the agent reports a completed tool call
           ✔  Then the intake reports no failure to the agent
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Production files for the GREEN step: **none** — investigated, not
        assumed. The intake's write path already is the best-effort recorder
        path: `ingest` appends through `HistoryStore::append_linked`, whose
        `Err` is reported and swallowed (`if let Err(error) = store.append_linked
        (…) { warning }`), so the process exits 0; the recorder's own `command`
        entry for the intake hits the same blocked path and warns the same way
        (`HistoryRecorder::finish`), and nothing is retried or written elsewhere.
        The `src/cli/history.rs`/`src/history.rs` work this task names — one
        warning path for the agent-sourced and Givn-sourced write failures —
        lands in the REFACTOR step below, where it belongs.

        Step and fixture (`tests/steps/agent_activity_reporting_steps.rs`):
        - `When the agent reports a completed tool call` — hands the fixture's
          completed tool call to the real intake for the change the recorded
          history built; the write failure is left to the fixture.
        - `Then the intake reports no failure to the agent` — the process exited
          `0` (a non-zero exit is a hook error for Claude Code and exit 2 blocks
          the tool); the write failure is visible as a `warning:` naming the
          blocked store path; no `error:` line; and the blocked store path is
          still the blocking directory, so no retry path, spool file, or second
          write location took the entry.

        Discrimination (ORACLE_RED) — controlled omissions in the isolated
        fixture, each restored immediately:

        (a) the unwritable-store fixture omitted (the store writable, so the
            write cannot fail), the warning half of the assertion must fail:
        ✔  Given a change "demo" with a recorded history
        ✔  And the development record cannot be written
        ✔  When the agent reports a completed tool call
        ✘  Then the intake reports no failure to the agent
           Step panicked. Captured output: the write failure was not reported at all: ""
        EXIT=101

        (b) the payload made unreadable (a payload-level failure, so the intake's
            exit is not 0), the exit-code half of the assertion must fail:
        ✔  Given a change "demo" with a recorded history
        ✔  And the development record cannot be written
        ✔  When the agent reports a completed tool call
        ✘  Then the intake reports no failure to the agent
           Step panicked. Captured output: assertion `left == right` failed: the intake failed the agent action it reported (exit Some(1)): warning: the development record could not be written to /tmp/.tmp39WUZu/givn/history/events.jsonl: Is a directory (os error 21)
        error: parse the intake payload as one JSON object: invalid type: string "not a payload", expected struct IntakePayload at line 1 column 15

          left: Some(1)
         right: Some(0)
        EXIT=101

        With the fixture restored the canonical run is green again (the run
        pasted above, EXIT 0).
        ```
  - [x] **REFACTOR** — one warning path for the agent-sourced and givn-sourced
        write failures; re-run → still exit 0. Paste it.
        ```
        The two writers of the record now report a store they could not write
        through one function: `src/history.rs::warn_unwritten(store, error)`
        (a warning naming the store path, never an error to the caller) replaces
        the recorder's private `warn` method and the intake's inline `eprintln!`,
        so `HistoryRecorder::finish` (Givn's own entries) and
        `src/cli/history.rs::ingest` (a reporter's payload) read identically and
        the wording has one home.

        $ cargo fmt --all && cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.84s
        CLIPPY=0

        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.55s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A rejected report never fails the agent action it reports"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.43s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A rejected report never fails the agent action it reports
           ✔  Given a change "demo" with a recorded history
           ✔  And the development record cannot be written
           ✔  When the agent reports a completed tool call
           ✔  Then the intake reports no failure to the agent
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        The warning wording has one home, so the scenario that pins it was
        re-run together with the bundle:
        $ for t in "development-record.feature::A record that cannot be written never fails the command it records" \
                   "agent-activity-reporting.feature::A reported tool call is recorded for its scope" \
                   "agent-activity-reporting.feature::A reported event that does not match the intake contract is rejected without corrupting the record" \
                   "agent-activity-reporting.feature::A rejected report never fails the agent action it reports"; do
              GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/$t" >/dev/null 2>&1
              echo "$t => EXIT=$?"
          done
        A record that cannot be written never fails the command it records => EXIT=0
        A reported tool call is recorded for its scope => EXIT=0
        A reported event that does not match the intake contract is rejected without corrupting the record => EXIT=0
        A rejected report never fails the agent action it reports => EXIT=0
        ```
  - [x] **COMMIT** — `fad0df07dd13d4502b6dbedce882862578855177` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `A rejected report never fails the agent action
        it reports` in the body. Record the revision identifier.
        ```
        One `givn commit` with the release note as the message subject and the
        scenario title as the body:

        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A rejected report never fails the agent action it reports"
        [15:46:07] committed revision on the Git backend

        $ git show --stat fad0df07dd13d4502b6dbedce882862578855177
        commit fad0df07dd13d4502b6dbedce882862578855177
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:46:07 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            A rejected report never fails the agent action it reports

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 189 ++++++++++++++++++++-
         src/cli/history.rs                                 |   5 +-
         src/history.rs                                     |  25 +--
         tests/steps/agent_activity_reporting_steps.rs      |  59 +++++++
         5 files changed, 257 insertions(+), 23 deletions(-)

        One atomic revision for the scenario: the delta spec (`@wip` removed from
        this scenario only), the step and its assertion, the one warning path
        (`src/history.rs::warn_unwritten`, used by the recorder and the intake),
        and this file's Task 18 record together.
        ```

## Task 19 — `The intake rejects a payload that is not readable JSON` (`agent-activity-reporting`, added)

- [x] **Scenario: `The intake rejects a payload that is not readable JSON`**
  - Design constraints:
    - Exit-code contract: `2` when stdin is not readable JSON at all (the
      payload-level failure), `0` when the payload was read (per-event rejections
      are warnings). Nothing in between: a stale reporter gets a distinguishable
      code it can act on.
    - "The development record gains no entry": the failed read must not write a
      partial line, must not create the store, and must not leave a lock behind.
    - Fixture: an initialised project root; the step pipes a non-JSON payload
      (raw text) into `givn history ingest` through the real binary on stdin.
    - Assertion shape: exit code exactly `2`; the record file's state is
      unchanged (absent or byte-identical).
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake rejects a payload that is not readable JSON"`
        → must exit non-zero. Paste it.
        ```
        (The first run, before the fixture's `When` existed:)
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake rejects a payload that is not readable JSON"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake rejects a payload that is not readable JSON
           ✔  Given a project root with givn/ initialised
           ✘  When the agent pipes a payload that is not JSON into `givn history ingest`
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:58:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2468814) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        The `When` — a real setup action, never a stub: it snapshots the record's
        bytes and pipes the truncated payload into the real binary's stdin — was
        then written so the RED lands on the new behaviour's assertion step,
        which is the failure the task predicts:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake rejects a payload that is not readable JSON"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 7.20s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake rejects a payload that is not readable JSON
           ✔  Given a project root with givn/ initialised
           ✔  When the agent pipes a payload that is not JSON into `givn history ingest`
           ✘  Then the intake exits 2
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:59:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2471051) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound (`the intake
        exits 2` matches no function; the scenario stops there, so `the
        development record gains no entry` never runs). This scenario's contract
        is genuinely new (the exit-code contract of `design.md` §Intake payload),
        so the undefined step is the RED; the same payload already exited `1`
        before this scenario's change, which is the OLD_PRODUCT_RED pasted in the
        GREEN block below.
        ```
  - [x] **GREEN** — implement the payload-level failure exit code. Files:
        `src/cli/history.rs`, `tests/steps/agent_activity_reporting_steps.rs`.
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.29s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake rejects a payload that is not readable JSON"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.59s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake rejects a payload that is not readable JSON
           ✔  Given a project root with givn/ initialised
           ✔  When the agent pipes a payload that is not JSON into `givn history ingest`
           ✔  Then the intake exits 2
           ✔  And the development record gains no entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        Production files:
        - `src/cli/history.rs` — `INTAKE_UNREADABLE_PAYLOAD` (the payload-level
          code) and `ingest`'s read: a payload that is not readable JSON prints
          the diagnosis and exits with that code instead of returning an error
          through the generic `1` path, so the process exits before the
          recording wrapper runs and the refused payload records nothing.
        - `tests/steps/agent_activity_reporting_steps.rs` — the `When` fixture
          (snapshot the record, pipe the truncated payload through the real
          binary's stdin) and the two `Then` steps: `out.status.code() ==
          Some(2)`, and the store's bytes still exactly the snapshot's with no
          `.lock`/`.tmp` left in `givn/history/`.
        - `tests/steps/development_timeline_support.rs` — `report_intake_raw`,
          the payload-on-stdin invocation that is not JSON (the JSON case is the
          existing `report_intake`).

        Discrimination (OLD_PRODUCT_RED) — the same fixture run against the
        pre-change write path (`let payload = read_payload()?;`, restored
        immediately) fails the new assertion, so the scenario discriminates the
        behaviour it introduces rather than an already-holding contract:

        $ # the one production line temporarily restored to its pre-change form
        ✘  Then the intake exits 2
           Step panicked. Captured output: assertion `left == right` failed: the intake did not report the payload-level failure with its own exit code (exit Some(1)): error: parse the intake payload as one JSON object: EOF while parsing an object at line 1 column 63

             left: Some(1)
            right: Some(2)
        EXIT=101

        (The old path also went on to record the intake's own command entry
        through the generic error path, which is what `the development record
        gains no entry` refuses: the new path exits inside dispatch, before the
        recording wrapper runs.)
        ```
  - [x] **REFACTOR** — make the two exit codes one enumerated outcome, not two
        unrelated returns; re-run → still exit 0. Paste it.
        ```
        The two codes are now one enumeration returned to the dispatcher:
        `src/cli/history.rs::IntakeOutcome` (`Read` → 0,
        `UnreadablePayload` → `INTAKE_UNREADABLE_PAYLOAD`) with
        `IntakeOutcome::exit_code`, `ingest() -> Result<IntakeOutcome>`, and
        `src/main.rs`'s dispatch arm exiting with the reported code — the same
        shape `lint` and `readme check` already use — so the intake's whole
        machine contract lives in one place instead of an inline
        `std::process::exit`.

        $ cargo fmt --all && cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.11s
        CLIPPY=0

        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.98s

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake rejects a payload that is not readable JSON"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 11.34s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake rejects a payload that is not readable JSON
           ✔  Given a project root with givn/ initialised
           ✔  When the agent pipes a payload that is not JSON into `givn history ingest`
           ✔  Then the intake exits 2
           ✔  And the development record gains no entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT 0

        The exit-0 side of the same contract (a payload that was read exits 0)
        is exercised by the greened scenarios of this bundle:
        $ for t in "…::A reported event that does not match the intake contract is rejected without corrupting the record" \
                   "…::A rejected report never fails the agent action it reports" \
                   "…::A reported tool call is recorded for its scope" \
                   "…::The intake rejects a payload that is not readable JSON"; do …; done
        A reported event that does not match the intake contract is rejected without corrupting the record => EXIT=0
        A rejected report never fails the agent action it reports => EXIT=0
        A reported tool call is recorded for its scope => EXIT=0
        The intake rejects a payload that is not readable JSON => EXIT=0
        ```
  - [x] **COMMIT** — `579b5971ad4d8ebcc695658964231a6fdb94d78e` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `The intake rejects a payload that is not
        readable JSON` in the body. Record the revision identifier.
        ```
        One `givn commit` with the release note as the message subject and the
        scenario title as the body:

        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The intake rejects a payload that is not readable JSON"
        [15:49:00] committed revision on the Git backend

        $ git show --stat 579b5971ad4d8ebcc695658964231a6fdb94d78e
        commit 579b5971ad4d8ebcc695658964231a6fdb94d78e
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:49:00 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            The intake rejects a payload that is not readable JSON

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 166 ++++++++++++++++++++-
         src/cli/history.rs                                 |  51 ++++++-
         src/main.rs                                        |  11 +-
         tests/steps/agent_activity_reporting_steps.rs      |  74 +++++++++
         tests/steps/development_timeline_support.rs        |   8 +
         6 files changed, 298 insertions(+), 14 deletions(-)

        One atomic revision for the scenario: the delta spec (`@wip` removed from
        this scenario only), the payload-level exit code and its one enumerated
        outcome (`IntakeOutcome`), the fixture and assertions, and this file's
        Task 19 record together.
        ```

## Task 20 — `The intake keeps the acceptable events of a payload with a rejected event` (`agent-activity-reporting`, added)

- [x] **Scenario: `The intake keeps the acceptable events of a payload with a rejected event`**
  - Design constraints:
    - Partial acceptance is the contract: one payload, one rejected event named
      on stderr, exit `0`, and the acceptable event appended. A rejected event
      never fails the whole batch (the reporter flushes a turn's buffer as one
      payload, so failing the batch would lose the acceptable events).
    - Assertion shape: exit code `0`; stderr names the rejected event; the record
      holds exactly the acceptable event (one appended line, with the acceptable
      event's kind and label), and the store's earlier bytes are a prefix of the
      new file.
    - Fixture: an initialised project root plus a two-event payload (one
      contract-violating, one valid).
    - This is the batch-level counterpart of Task 17's record-integrity check;
      both must hold for the same validation table.
    - RED expected failure: the exit-0, stderr and record Then steps are
      undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake keeps the acceptable events of a payload with a rejected event"`
        → must exit non-zero. Paste it.
        ```
        (The first run, before the fixture's `When` existed:)
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake keeps the acceptable events of a payload with a rejected event"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake keeps the acceptable events of a payload with a rejected event
           ✔  Given a project root with givn/ initialised
           ✘  When the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:65:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2481037) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        The `When` — a real setup action, never a stub: it snapshots the record's
        bytes and hands the real intake a two-event payload (the unmapped-kind
        event first, then the acceptable tool call) — was then written so the RED
        lands on the new behaviour's assertion steps, which is the failure the
        task predicts:

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake keeps the acceptable events of a payload with a rejected event"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.99s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake keeps the acceptable events of a payload with a rejected event
           ✔  Given a project root with givn/ initialised
           ✔  When the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`
           ✘  Then the intake exits 0
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:66:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2481766) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the exit-0, stderr and record Then steps
        are unbound (`the intake exits 0` matches no function; the scenario stops
        there). The batch path the Then steps assert already exists from Task 12
        (one payload read, each event gated through the same
        `accept_event`/`validate` path, the accepted entries appended in one
        batch), so all three assertions are proven load-bearing in the GREEN
        block below with two controlled omissions in the isolated fixture
        (ORACLE_RED), restored immediately.
        ```
  - [x] **GREEN** — keep the batch path appending per accepted event. Files:
        `src/cli/history.rs`, `tests/steps/agent_activity_reporting_steps.rs`.
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
        EXIT 0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake keeps the acceptable events of a payload with a rejected event"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.18s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The intake keeps the acceptable events of a payload with a rejected event
           ✔  Given a project root with givn/ initialised
           ✔  When the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`
           ✔  Then the intake exits 0
           ✔  And stderr names the rejected event
           ✔  And the development record holds the acceptable event
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Production files for the GREEN step: **none** — investigated, not
        assumed. The batch path already accepts per event and appends the
        accepted ones: `ingest` reads the payload, `accepted_entries` walks the
        payload in order (each event gated by `accept_event` → the one
        `IngestedEvent::validate` table, each rejection named by `warn_rejected`),
        and the accepted entries reach `HistoryStore::append_linked` as one
        batch — a rejected event never returns early and never fails the payload,
        which is what this scenario asserts. The `src/cli/history.rs` work this
        task names — no second code path beside Task 17's validation table —
        is verified in the REFACTOR step below.
        Steps and fixture (`tests/steps/agent_activity_reporting_steps.rs`):
        - `When the agent pipes a payload with one rejected event and one
          acceptable event into `givn history ingest`` — snapshot the record,
          build the mixed payload (`kind: "teleport"` first, then the fixture's
          acceptable `grep` tool call), and hand it to the real binary's stdin.
          The payload names no scope, so the events are recorded under the
          project scope the resolution derives.
        - `Then the intake exits 0` — the payload was read: `Some(0)`.
        - `And stderr names the rejected event` — exactly one `warning:` naming
          the rejected event's position and kind, and no `error:` line.
        - `And the development record holds the acceptable event` — the bytes
          recorded before the payload are still the store's first bytes
          (append-only, never rewritten); the appended region holds exactly one
          agent-sourced entry, and it carries the acceptable event's kind, tool
          name and label; and the record's only agent entry is that one, so the
          rejected sibling was never recorded. The intake's own `command` entry
          is the other appended line, which the design records for every
          command — the assertion counts agent-sourced lines, never all lines.

        Discrimination (ORACLE_RED) — controlled omissions in the isolated
        fixture, each restored immediately:

        (a) the rejected event omitted (a payload that is all acceptable), the
            rejection assertion must fail:
        ✔  Then the intake exits 0
        ✘  And stderr names the rejected event
           Step panicked. Captured output: assertion `left == right` failed: expected the payload's one contract-violating event to be named on stderr: []
             left: 0
            right: 1
        EXIT=101

        (b) the acceptable event omitted (a payload that is all rejected), the
            record assertion must fail:
        ✔  Then the intake exits 0
        ✔  And stderr names the rejected event
        ✘  And the development record holds the acceptable event
           Step panicked. Captured output: assertion `left == right` failed: expected exactly the acceptable event in the appended region: [
               Object {
                   "at": String("2026-10-16T13:51:36.180Z"),
                   "data": Object { "exit_code": Number(0) },
                   "duration_ms": Number(3),
                   "id": Number(2),
                   "kind": String("command"),
                   "label": String("history ingest"),
                   "name": String("history"),
                   "outcome": String("ok"),
                   "schema": Number(1),
                   "scope": Object { "id": String(".tmpB7b9wU"), "kind": String("project") },
                   "source": String("givn"),
               },
           ]
             left: 0
            right: 1
        EXIT=101

        The exit-0 half of the assertion is the same contract Task 18 and Task 19
        pin from their own sides (an unwritable store and an unreadable payload):
        a rejected *event* by construction cannot fail the batch, and the one
        payload-level failure that can (Task 19) reports its own code.

        With the fixture restored the canonical run is green again (the run
        pasted above, EXIT 0).
        ```
  - [x] **REFACTOR** — reuse Task 17's validation table without a second
        code path; re-run → still exit 0. Paste it.
        ```
        The batch path and the single-event path are one path: `accept_event` is
        the only gate into the append (`IngestedEvent::validate` is the only
        validation table), and `accepted_entries` is its only caller for a
        payload, so a rejected event and a kept event cannot diverge between
        Task 17's scenario and this one. No production line needed changing to
        reuse it; the test side lost its duplication instead — the rejection
        assertion (`assert_rejection_names_the_event`) and the append-only
        byte-range check (`appended_since`) now each have one home, shared by the
        two scenarios:

        $ cargo fmt --all && cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.87s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The intake keeps the acceptable events of a payload with a rejected event"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.34s
             Running tests/features_runner.rs (target/debug/deps/features_runner-05d317fc7c42c8e4)
        Feature: Agent Activity Reporting
          Scenario: The intake keeps the acceptable events of a payload with a rejected event
           ✔  Given a project root with givn/ initialised
           ✔  When the agent pipes a payload with one rejected event and one acceptable event into `givn history ingest`
           ✔  Then the intake exits 0
           ✔  And stderr names the rejected event
           ✔  And the development record holds the acceptable event
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT 0

        Regression check — every greened non-`@e2e` scenario of the delta, one
        process:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --tags "not @e2e and not @wip"
        [Summary]
        3 features
        22 scenarios (22 passed)
        109 steps (109 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `82579d36cf4c5893dc9d1ce4d1f23f23b750de70` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `The intake keeps the acceptable events of a
        payload with a rejected event` in the body. Record the revision
        identifier.
        ```
        One `givn commit` with the release note as the message subject and the
        scenario title as the body:

        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The intake keeps the acceptable events of a payload with a rejected event" \
          givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature \
          givn/changes/development-timeline/tasks.md \
          tests/steps/agent_activity_reporting_steps.rs
        [15:53:24] committed revision on the Git backend

        (The revision names its three paths: the shared checkout carried an
        unrelated file another session had just created, which the stage-all form
        would have swept into this scenario's revision. The first attempt was
        un-made and re-created with the paths named here; the stray file was left
        exactly as its author left it.)

        $ git show --stat 82579d36cf4c5893dc9d1ce4d1f23f23b750de70
        commit 82579d36cf4c5893dc9d1ce4d1f23f23b750de70
        Author: buster <me@bstr.eu>
        Date:   Fri Oct 16 15:53:24 2026 +0200

            feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

            The intake keeps the acceptable events of a payload with a rejected event

            Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 203 ++++++++++++++++++++-
         tests/steps/agent_activity_reporting_steps.rs      | 142 ++++++++++++--
         3 files changed, 324 insertions(+), 23 deletions(-)

        One atomic revision for the scenario: the delta spec (`@wip` removed from
        this scenario only), the fixture and its three assertions, and this file's
        Task 20 record together.
        ```

## Task 21 — `Every supported harness receives a development reporter` (`agent-activity-reporting`, added — Scenario Outline, three Examples rows)

- [x] **Scenario: `Every supported harness receives a development reporter`** (Examples: `omp`, `opencode`, `claude`)
  - Design constraints:
    - The repository's agent targets are `opencode`, `agents`, `claude`, `omp`
      (`src/skills/targets.rs::AgentTarget::ALL`); three of them execute the
      agent's work and therefore have activity to report — `omp`, `opencode`,
      `claude`. `sync_agent_integrations` writes integrations for exactly those
      three today and copies every asset **verbatim**.
    - Assets and install paths (the design's table):
      `assets/agent-integrations/omp/givn-timeline.js` →
      `.omp/hooks/pre/givn-timeline.js` (auto-discovered `pre` hook factory, no
      manifest); `assets/agent-integrations/opencode/givn-timeline.js` →
      `.opencode/plugins/givn-timeline.js` (V2 plugin: default export
      `{ id: "givn-timeline", async setup(ctx) }`, hooks plus one
      `ctx.event.subscribe`); `assets/agent-integrations/claude/givn-timeline.sh`
      → `.claude/hooks/givn-timeline.sh` plus one entry per event in
      `.claude/settings.local.json` (command hooks shaped
      `hooks.<Event>[{matcher, hooks:[{type:"command", command}]}]`).
    - Wiring: all three assets are added to `managed_generated_paths` (so
      `givn init`, `givn skills sync`, and the managed upgrade keep them current
      and the migration allowlist stages them) and listed in
      `src/skills/rules.rs::ignore_entries()` so the `local-only` policy keeps
      them out of a repository.
    - Claude registration merge: `merge_claude_settings` is extended to merge
      this reporter's per-event entries idempotently per `(event, matcher,
      command)` — so a changed matcher is re-applied — without touching user
      entries or the attribution entry (which `PreToolUse` merging already
      handles).
    - Assertion shape: `givn init --agents <target>` exits 0 and the project's
      generated guidance for `<target>` includes the development reporter at its
      install path (for `claude`: the shim plus its per-event registration), read
      from the generated project files — never asserted against the asset
      template alone.
    - The outline's three Examples rows run in one process under the canonical
      `--exact` command.
    - RED expected failure: the guidance Then step is undefined for all three
      rows.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Every supported harness receives a development reporter"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Every supported harness receives a development reporter"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.95s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents omp`
           ✔  Then the command exits 0
           ✘  And the project's generated guidance for "omp" includes the development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:75:5
              Step doesn't match any function
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents opencode`
           ✔  Then the command exits 0
           ✘  And the project's generated guidance for "opencode" includes the development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:75:5
              Step doesn't match any function
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents claude`
           ✔  Then the command exits 0
           ✘  And the project's generated guidance for "claude" includes the development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:75:5
              Step doesn't match any function
        [Summary]
        1 feature
        3 scenarios (3 failed)
        12 steps (9 passed, 3 failed)

        thread 'main' (2514704) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        3 steps failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the guidance Then step matches no
        function for every Examples row, exactly as the task predicts. The
        `Given`/`When`/`Then the command exits 0` steps are the repository's
        existing shared steps; `givn init --agents <target>` already exits 0
        today, so the RED lands on the new assertion, which fails before the
        reporter exists.
        ```
  - [x] **GREEN** — create the three assets, install them for their targets, and
        extend the Claude settings merge. Files:
        `assets/agent-integrations/omp/givn-timeline.js`,
        `assets/agent-integrations/opencode/givn-timeline.js`,
        `assets/agent-integrations/claude/givn-timeline.sh`,
        `src/skills/targets.rs`, `src/skills/rules.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first (Rust) and
        syntax-check the assets (`node --check` for the two `.js` assets,
        `sh -n` for the shim), re-run → exit 0. Paste it.
        ```
        $ node --check assets/agent-integrations/omp/givn-timeline.js
        OMP OK
        $ node --check assets/agent-integrations/opencode/givn-timeline.js
        OPENCODE OK
        $ sh -n assets/agent-integrations/claude/givn-timeline.sh
        SH OK

        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.19s
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Every supported harness receives a development reporter"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.79s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents omp`
           ✔  Then the command exits 0
           ✔  And the project's generated guidance for "omp" includes the development reporter
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents opencode`
           ✔  Then the command exits 0
           ✔  And the project's generated guidance for "opencode" includes the development reporter
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents claude`
           ✔  Then the command exits 0
           ✔  And the project's generated guidance for "claude" includes the development reporter
        [Summary]
        1 feature
        3 scenarios (3 passed)
        12 steps (12 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `assets/agent-integrations/omp/givn-timeline.js` (new — the omp
          reporter: one buffer per session, `Date.now()` stamps for the tool,
          turn, and subagent spans omp does not time, the `ask` tool's wait, and
          a flush to `givn history ingest` at the turn boundary and at
          `session_shutdown`);
        - `assets/agent-integrations/opencode/givn-timeline.js` (new — the
          opencode V2 plugin: `id: "givn-timeline"`, `setup(ctx)` registering
          the tool/permission hooks beside one `ctx.event.subscribe` iterable
          over the server's event stream, with the harness's own `created`
          timestamps for the durations and one flush per step boundary);
        - `assets/agent-integrations/claude/givn-timeline.sh` (new — the shim
          that pipes its hook payload to the binary-side mapper);
        - `src/skills/targets.rs` (modified — `sync_agent_integrations` installs
          each reporter for its executing target, `managed_generated_paths`
          lists the three reporter paths so `givn init`, `givn skills sync`, and
          the managed upgrade keep them current, and `merge_claude_settings`
          merges the reporter's per-event entries idempotently per
          `(event, matcher, command)` beside — never over — the attribution
          entry and user entries);
        - `src/skills/rules.rs` (modified — `ignore_entries()` and
          `is_rule_path()` carry the three reporter paths, so the `local-only`
          policy keeps them out of a repository, and the rule-shape unit test
          covers them);
        - `tests/steps/agent_activity_reporting_steps.rs` (modified — the
          guidance Then step reads the generated project file, asserts it is
          the shipped reporter verbatim and delivers through the intake, and,
          for Claude, that the shim is executable and the settings register
          exactly the events the mapper reads).

        Design-table notes (reported, not silently worked around): the omp and
        opencode assets implement the design's tool/turn/subagent/question and
        usage rows; the design's `session_start` session span and
        `before_subagent_spawn` dispatch record have no scenario in this change
        and no intake kind, and the opencode `session.usage.updated` /
        `form.*` rows are not covered by any scenario — they are left for the
        harness-driver bundle (Task E4) that owns those drivers.
        ```
  - [x] **REFACTOR** — one asset-to-target table instead of three bespoke
        branches; re-run → still exit 0. Paste it.
        ```
        `sync_agent_integrations` is now one loop over the target's own
        `AgentIntegration` table (`OPENCODE_INTEGRATIONS`, `OMP_INTEGRATIONS`,
        `CLAUDE_INTEGRATIONS`, `Agents => &[]`), so each target's install set is
        one place and the three `if targets.contains(…)` branches are gone. No
        behaviour change: the same files land at the same paths with the same
        executable bit, and the Claude settings merge still runs once for the
        Claude target.

        $ cargo fmt --all -- --check
        (no output)
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.09s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Every supported harness receives a development reporter"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.76s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents omp`
           ✔  Then the command exits 0
           ✔  And the project's generated guidance for "omp" includes the development reporter
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents opencode`
           ✔  Then the command exits 0
           ✔  And the project's generated guidance for "opencode" includes the development reporter
          Scenario Outline: Every supported harness receives a development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents claude`
           ✔  Then the command exits 0
           ✔  And the project's generated guidance for "claude" includes the development reporter
        [Summary]
        1 feature
        3 scenarios (3 passed)
        12 steps (12 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `ff3480e38b07189ca68f43a0402c28b5212ee395` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `Every supported harness receives a development
        reporter` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Every supported harness receives a development reporter"
        [16:08:18] committed revision on the Git backend
        EXIT=0

        Revision ff3480e38b07189ca68f43a0402c28b5212ee395 touches exactly this
        scenario's files: the three new assets, `src/skills/targets.rs`,
        `src/skills/rules.rs`, `tests/steps/agent_activity_reporting_steps.rs`,
        the delta `.feature`'s `@wip` removal, and this task file's evidence.
        ```

## Task 22 — `The neutral skills target receives no development reporter` (`agent-activity-reporting`, added)

- [x] **Scenario: `The neutral skills target receives no development reporter`**
  - Design constraints:
    - The `agents` target is a skills mirror that no harness executes; it
      receives **no** reporter. Installing one would claim activity that can
      never arrive — the "a signal a harness cannot deliver is recorded as
      missing rather than as zero" rule at the installation level.
    - Assertion shape: `givn init --agents agents` exits 0 and no development
      reporter is installed anywhere in the project — no
      `.omp/hooks/pre/givn-timeline.js`, no `.opencode/plugins/givn-timeline.js`,
      no `.claude/hooks/givn-timeline.sh`, and no reporter registration inside
      `.claude/settings.local.json`.
    - Fixture: a project root without a `givn/` directory (the uninitialised
      fixture; `givn init` creates it) and the explicit
      `--agents agents` selection (never the interactive prompt).
    - RED expected failure: the no-reporter Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The neutral skills target receives no development reporter"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The neutral skills target receives no development reporter"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The neutral skills target receives no development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents agents`
           ✔  Then the command exits 0
           ✘  And no development reporter is installed for that project
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:88:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (2521059) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the no-reporter Then step matches no
        function; the scenario itself is the mirror of Task 21's install set, so
        its RED is the missing assertion, not a missing production path.
        ```
  - [x] **GREEN** — keep the reporter set keyed to the executing targets.
        Files: `src/skills/targets.rs`,
        `tests/steps/agent_activity_reporting_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.15s
        BUILD=0

        $ cargo test --locked --lib skills::targets
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.68s
             Running unittests src/lib.rs (target/debug/deps/givn-7704d9c228643a4b)

        running 4 tests
        test skills::targets::tests::validate_frontmatter_requires_description ... ok
        test skills::targets::tests::validate_frontmatter_requires_name_match ... ok
        test skills::targets::tests::all_command_templates_are_embedded ... ok
        test skills::targets::tests::only_executing_targets_receive_a_development_reporter ... ok

        test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 268 filtered out; finished in 0.00s
        UNITTESTS=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The neutral skills target receives no development reporter"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.51s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The neutral skills target receives no development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents agents`
           ✔  Then the command exits 0
           ✔  And no development reporter is installed for that project
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Production files for this GREEN step: `src/skills/targets.rs`
        (modified — the `only_executing_targets_receive_a_development_reporter`
        unit test pins the install set to the three executing targets, asserts
        the `agents` mirror receives no integration row, and asserts every
        reporter row's asset is really embedded) and
        `tests/steps/agent_activity_reporting_steps.rs` (modified — the
        no-reporter Then step reads the project tree, not just the four
        design-named paths, and checks the Claude settings file when one
        exists). No further production path was needed: the Task 21 REFACTOR's
        target-keyed table already answers `Agents => &[]`, and `givn init
        --agents agents` therefore writes no reporter — investigated and
        asserted rather than assumed.
        ```
  - [x] **REFACTOR** — let the target list (not a hard-coded check) decide which
        targets receive a reporter, so a future target cannot silently inherit
        one; re-run → still exit 0. Paste it.
        ```
        `managed_generated_paths` no longer enumerates target paths with three
        `if targets.contains(…)` blocks: it walks the selected targets and pushes
        each target's own `integrations_of(target)` rows plus its
        `registration_of(target)` file. The install list, the allowlist, and the
        writer now all read the target-keyed table, so a result for the neutral
        `agents` target is the empty row set — a future target extends the table
        instead of inheriting a reporter, and the allowlist cannot drift from
        what `sync` writes. No behaviour change: the same files are installed
        and listed.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.79s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::The neutral skills target receives no development reporter"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.23s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: The neutral skills target receives no development reporter
           ✔  Given a project root without a givn/ directory
           ✔  When the consumer runs `givn init --agents agents`
           ✔  Then the command exits 0
           ✔  And no development reporter is installed for that project
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `eb2191810eb28efd06adf6a7a789d5ae4a660681` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `The neutral skills target receives no
        development reporter` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The neutral skills target receives no development reporter"
        [16:10:46] committed revision on the Git backend
        EXIT=0
        ```

## Task 23 — `A wait the harness does not time is still recorded with a duration` (`agent-activity-reporting`, added)

- [x] **Scenario: `A wait the harness does not time is still recorded with a duration`**
  - Design constraints:
    - Claude Code facts: `PreToolUse` payloads carry **no** timestamp and no
      duration, and `duration_ms` **excludes** time spent in permission prompts —
      so a permission wait's duration must come from the reporter's own stamps.
      `PermissionRequest` (`tool_name`, `tool_input`) and `Notification` with
      `notification_type` `permission_prompt` / `idle_prompt` mark the start; the
      matching completion consumes the pending entry.
    - Pending state: `givn/history/pending/<harness>-<session-id>.json`, shaped
      `{<tool_use_id>: {started_at, kind, label}}` — written atomically before the
      tool runs, consumed by the matching end event, pruned by age (older than
      24 h) and at session end, and it holds no payload content (only the opaque
      harness id, the start time, the entry kind, and the same bounded label the
      record would keep).
    - The generated asset stays a three-line `sh` shim that pipes stdin to the
      binary-side mapper `givn history harness-hook --harness claude`
      (`src/cli/history.rs`: `HarnessPayload`, `map_harness_event`,
      `PendingState`) — the same shape the attribution integration already uses.
      `map_harness_event` is a named non-zero production probe.
    - Failure discipline: every mapping step is best-effort and the mapper always
      exits `0`; tool-level hooks are registered `async: true` so they never slow
      the agent; `Stop` and `SessionEnd` are synchronous with a short timeout
      (`SessionEnd` shares a 1.5 s budget).
    - Driver (the design's real entry point, also used by the `@e2e` tier): run
      the generated shim directly with a real hook payload on stdin, with `PATH`
      shimming `givn` to the built binary and a fixture transcript at
      `transcript_path`.
    - Assertion shape: one wait entry exists for the permission request, with a
      duration the reporter measured itself (a positive value consistent with the
      stamped gap, not the harness's absent/excluding `duration_ms`).
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A wait the harness does not time is still recorded with a duration"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A wait the harness does not time is still recorded with a duration"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A wait the harness does not time is still recorded with a duration
           ✘  Given a project with the generated Claude Code development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:92:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (2525595) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps are unbound and the
        scenario's own Given/When are unbound too; the harness-hook entry point
        (`givn history harness-hook --harness claude`, whose mapper is
        `todo!()`), the pending state, the shim driver, and the assertions are
        all still missing.
        ```
  - [x] **GREEN** — implement the pending-state stamps and the wait mapping.
        Files: `src/cli/history.rs`, `src/history.rs` (`PendingState`),
        `assets/agent-integrations/claude/givn-timeline.sh`,
        `src/skills/targets.rs` (per-event registration),
        `tests/steps/agent_activity_reporting_steps.rs` and the shim driver in
        `tests/steps/development_timeline_support.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.39s
        BUILD=0
        $ cargo build --locked --tests
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 11.92s
        TESTS=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A wait the harness does not time is still recorded with a duration"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 3.35s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A wait the harness does not time is still recorded with a duration
           ✔  Given a project with the generated Claude Code development reporter
           ✔  When the harness reports a permission request and then completion of that same tool call
           ✔  Then the record holds one wait entry for the permission request
           ✔  And that entry carries a duration measured by the reporter itself
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        The mapped record, from a scratch run of the same real entry point
        (first payload, a 0.7 s wait, then the completion; `givn` on `PATH` is
        this build):

        $ printf '%s' '{"hook_event_name":"PermissionRequest","session_id":"s1","tool_name":"Bash","tool_input":{"command":"cargo test --locked"}}' | .claude/hooks/givn-timeline.sh
        req exit=0
        $ cat givn/history/pending/claude-s1.json
        {"starts":{"permission:Bash":{"started_at":"2026-10-16T14:15:03.054Z","kind":"question","label":"permission request: Bash"}}}
        $ sleep 0.7; printf '%s' '{"hook_event_name":"PostToolUse","session_id":"s1","tool_name":"Bash","tool_use_id":"toolu_1","tool_input":{"command":"cargo test --locked"},"duration_ms":40,"tool_response":{}}' | .claude/hooks/givn-timeline.sh
        end exit=0
        (the pending file is consumed and removed)
        $ grep '"source": "agent"' givn/history/events.jsonl
        {"schema": 1, "id": 3, ..., "source": "agent", "kind": "tool", "name": "Bash", "label": "cargo test --locked", "duration_ms": 40, "outcome": "ok", ...}
        {"schema": 1, "id": 4, ..., "source": "agent", "kind": "question", "name": "question", "label": "permission request: Bash", "duration_ms": 824, "outcome": "ok", ...}

        The wait carries the reporter's own measurement (824 ms for a 0.7 s
        wait), never the harness's 40 ms `duration_ms`, which the tool entry
        keeps — the discrimination the two Then steps assert.

        Production files created/modified by this GREEN step:
        - `src/history.rs` (modified — `PendingState` implemented: `load`,
          `open`, `take`, `take_first`, `prune` by the 24 h age, `clear`, and an
          atomic `store` that removes the file when nothing is open; plus
          `pending_path` for
          `givn/history/pending/<harness>-<session-id>.json` and the
          `now_rfc3339`/`now_ms` stamps the mapper measures with);
        - `src/cli/history.rs` (modified — `HarnessPayload` carries the Claude
          payload's fields; `map_harness_event` stamps the tool/question starts,
          closes the tool's own span with the harness's `duration_ms`, consumes
          the permission/idle wait a completion closes and measures it from the
          reporter's own stamps, and prunes everything at `SessionEnd`), and
          `harness_hook` is the always-exit-0 process-per-hook entry point;
        - `src/main.rs` (modified — the `givn history harness-hook --harness
          <name>` subcommand wired into dispatch, the entry point the generated
          shim pipes to; Task S4 deferred this wiring to "the first scenario
          that needs each command", and this is that scenario; the recording
          wrapper's read-only exemption is unchanged, so the hook invocation
          records like every other non-read command);
        - `assets/agent-integrations/claude/givn-timeline.sh` (created in
          Task 21; unchanged here — the design requires a three-line shim with
          no logic beyond the pipe);
        - `src/skills/targets.rs` (the per-event registration landed with
          Task 21's Claude settings merge; unchanged here);
        - `tests/steps/development_timeline_support.rs` (modified — the Claude
          shim driver: `givn init --agents claude`, the generated shim executed
          directly with a `PATH`-shimmed `givn`, and the permission-request /
          completion payloads);
        - `tests/steps/agent_activity_reporting_steps.rs` (modified — the
          scenario's Given/When and both Then steps, and the driver-gap state
          the duration assertion compares against).

        Scope note: an earlier draft of this GREEN step also landed Task 24's
        material (the transcript reader, the `Stop` turn mapping, the
        `SubagentStop` usage hand-off, and Task 24's steps/fixture). That is
        Task 24's own GREEN work, so it was removed from this scenario's commit
        and its scenario commit; this commit carries exactly the wait mapping
        this scenario asserts.
        ```
  - [x] **REFACTOR** — one pending-state read/write helper, one pruning rule;
        re-run → still exit 0. Paste it.
        ```
        `harness_hook` no longer reads, prunes, and writes the state inline:
        `PendingSession::read(root, harness, session_id)` owns the one read (the
        path the session id names, a missing or unreadable file as an empty
        state) and `PendingSession::write()` owns the one write (the record's
        single pruning rule — `PendingState::prune` with the 24 h
        `PENDING_MAX_AGE_MS` — then one atomic store, which removes the file when
        nothing is open). A payload without a session id maps against an empty
        state and writes nothing. No behaviour change: the same stamps, the same
        one store, and the same file removed when the wait closes.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.40s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::A wait the harness does not time is still recorded with a duration"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 12.10s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: A wait the harness does not time is still recorded with a duration
           ✔  Given a project with the generated Claude Code development reporter
           ✔  When the harness reports a permission request and then completion of that same tool call
           ✔  Then the record holds one wait entry for the permission request
           ✔  And that entry carries a duration measured by the reporter itself
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `f7365a573212d436bfae32d857be952aa13eb89f` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `A wait the harness does not time is still
        recorded with a duration` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        A wait the harness does not time is still recorded with a duration"
        [16:16:30] committed revision on the Git backend
        EXIT=0

        Revision 6160eac363bab1d8128fed183b7828ce1063a66f was the first commit
        of this scenario; it was withdrawn (soft reset, own unpushed revision)
        because it also carried Task 24's material, and replaced by the
        Task 23-only revision recorded above. The delta `.feature`'s `@wip`
        removal and this task file's evidence ride with the revision.
        ```

## Task 24 — `Token usage read from the harness's own session record is recorded with its model` (`agent-activity-reporting`, added)

- [x] **Scenario: `Token usage read from the harness's own session record is recorded with its model`**
  - Design constraints:
    - Claude Code carries no token usage or cost in hook payloads, so usage is
      read from the harness's own session record at `transcript_path`: assistant
      entries carry
      `message.usage.{input_tokens, cache_creation_input_tokens, cache_read_input_tokens, output_tokens, output_tokens_details.thinking_tokens}`
      beside `message.model`, and `type:"cost-state"` entries carry the session
      rollup per model.
    - Collection points: usage is collected only at `Stop`, `SubagentStop`, and
      `SessionEnd` — the session record is written asynchronously and may lag the
      live turn, so it is never assumed current mid-turn.
    - The mapping lives in the binary (`src/cli/history.rs`); the asset stays the
      `sh` shim. A subagent's tokens come from its own transcript at
      `SubagentStop`, and a background launch answering `status:
      "async_launched"` contributes no usage from the tool response.
    - Assertion shape: the `usage` entry for the turn carries the transcript's
      counts and names `claude-sonnet-5` verbatim (the model that produced the
      usage), with `outcome` reflecting the reported turn end.
    - Fixture: a fixture transcript JSONL holding an assistant entry with usage
      and the model name; the shim is invoked with a `Stop` payload naming that
      `transcript_path`.
    - RED expected failure: both Then steps are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Token usage read from the harness's own session record is recorded with its model"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Token usage read from the harness's own session record is recorded with its model"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Token usage read from the harness's own session record is recorded with its model
           ✔  Given a project with the generated Claude Code development reporter
           ✘  And the harness's session record holds assistant token usage for "claude-sonnet-5"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:100:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2536819) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the scenario's session-record Given (and
        both Then steps) match no function, and the mapper has no `Stop` arm at
        all: Task 23's commit carries exactly the wait mapping, so the
        transcript reader, the turn's usage mapping, and the subagent usage
        hand-off are still missing.
        ```
  - [x] **GREEN** — implement the transcript reader and the usage mapping.
        Files: `src/cli/history.rs`, `assets/agent-integrations/claude/givn-timeline.sh`
        (no logic beyond the pipe), `tests/steps/agent_activity_reporting_steps.rs`
        plus the fixture transcript in the support file. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.89s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Token usage read from the harness's own session record is recorded with its model"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 14.36s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Token usage read from the harness's own session record is recorded with its model
           ✔  Given a project with the generated Claude Code development reporter
           ✔  And the harness's session record holds assistant token usage for "claude-sonnet-5"
           ✔  When the harness reports the end of a turn with that session record's path
           ✔  Then the record holds that turn's token usage
           ✔  And the usage names "claude-sonnet-5"
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/history.rs` (modified — `HarnessPayload` gains the session
          record's path and the subagent's own run id/definition/record path;
          `map_harness_event` gains the `Stop` arm (`map_turn_end`) and the
          `SubagentStart`/`SubagentStop` arms (a pending start stamp per run id,
          one completion entry measured from it); `transcript_usage` is the
          binary-side reader of the harness's own session record (a real user
          prompt opens a turn, assistant entries sum `input_tokens`,
          `output_tokens`, `cache_read_input_tokens`,
          `cache_creation_input_tokens` and carry `message.model` verbatim,
          with one malformed line never costing the rest of the transcript),
          and `IngestedEvent::map_subagent` hands a completion's usage to one
          usage entry parented to the subagent span);
        - `assets/agent-integrations/claude/givn-timeline.sh` (unchanged — the
          three-line shim, no logic beyond the pipe);
        - `tests/steps/development_timeline_support.rs` (modified — the
          `Stop` payload the shim is driven with, the fixture transcript builder
          in the harness's own shape, and the counts/model the assertions read);
        - `tests/steps/agent_activity_reporting_steps.rs` (modified — the
          session-record Given, the turn-end When, the two Then steps, and the
          shared `the record holds that turn's token usage` step text now
          dispatching on the scenario's own fixture: the reported-payload turn
          keeps Task 15's assertions (change scope, payload counts), the
          transcript turn asserts the harness record's counts and the turn link).
        ```
  - [x] **REFACTOR** — one transcript reader shared by the turn and subagent
        paths, defensive parsing kept; re-run → still exit 0. Paste it.
        ```
        The reader is now the session record's one reader for both paths:
        `session_record_usage(path: Option<&Path>) -> Option<Usage>` owns the
        optional path, the defensive line-by-line parse, the turn boundary, and
        the accumulator (`SessionUsage`, renamed from the turn-only name), and
        both `map_turn_end` (`transcript_path`) and `map_subagent_stop`
        (`agent_transcript_path`) read through it with one call each. No
        behaviour change: same parsing rules, same sums, same `None` for an
        absent/unreadable/unmeasured record.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.48s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Token usage read from the harness's own session record is recorded with its model"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.46s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Token usage read from the harness's own session record is recorded with its model
           ✔  Given a project with the generated Claude Code development reporter
           ✔  And the harness's session record holds assistant token usage for "claude-sonnet-5"
           ✔  When the harness reports the end of a turn with that session record's path
           ✔  Then the record holds that turn's token usage
           ✔  And the usage names "claude-sonnet-5"
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Regression run of the two committed scenarios this step's shared code
        touches (`map_subagent`'s usage hand-off, the shared record-usage step):

        $ ... --exact "...::A subagent's activity is attributed to that subagent"
        1 scenario (1 passed), 4 steps (4 passed)        EXIT=0
        $ ... --exact "...::Reported token usage is recorded with the model that produced it"
        1 scenario (1 passed), 4 steps (4 passed)        EXIT=0
        ```
  - [x] **COMMIT** — `506dfe273d2fe8b84efaaa2718f85d71c5feb6cf` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `Token usage read from the harness's own session
        record is recorded with its model` in the body. Record the revision
        identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Token usage read from the harness's own session record is recorded with its model"
        [16:22:00] committed revision on the Git backend
        EXIT=0

        Revision 506dfe273d2fe8b84efaaa2718f85d71c5feb6cf touches exactly this
        scenario's files: `src/cli/history.rs`, the two step files, the delta
        `.feature`'s `@wip` removal, and this task file's evidence. The shim
        asset is unchanged — the design keeps it a three-line pipe with no logic
        beyond the pipe.
        ```

## Task 25 — `The report ranks the longest single step first` (`development-report`, added)

- [x] **Scenario: `The report ranks the longest single step first`**
  - Design constraints:
    - Command: `givn timeline --change demo --out report.html` (D4: an on-demand
      command that writes one self-contained file and prints its path; no
      `--open`, no `--json`). Default path when `--out` is omitted:
      `givn/history/reports/<scope>.html`.
    - Model and rendering: `src/cli/timeline.rs::build_report_model` turns the
      record into `ReportModel` (scope, bands, entries, summary) and
      `render_report` substitutes the JSON model into
      `assets/timeline/report.html`'s single data slot
      (`/*__GIVN_REPORT_DATA__*/`). `build_report_model` is a named non-zero
      production probe.
    - Summary content: total elapsed, the **ranked longest steps**, the entry
      count, the question-wait total and count, the subagent count and combined
      duration, token totals overall / per model / per subagent. Durations are
      the recorded `duration_ms` values; unmeasured durations render "not
      measured" and are not counted in the totals.
    - Regular-scenario assertion rule: the command's own output is the written
      report file, so the step reads that file — the JSON model in the data slot
      and the rendered text — never the store alone. The browser traversal of the
      same ranking is Task E6.
    - Fixture: change `demo` whose seeded record holds steps of different
      durations (support-file seeded-history builder).
    - Report asset rules that must hold from this task on: no third-party
      JavaScript, no CDN, no build step, inline CSS/JS/SVG, tokens from
      `docs/design/design-system.md` carried inline, the dense-timeline direction
      (D6).
    - RED expected failure: the ranking Then step is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report ranks the longest single step first"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report ranks the longest single step first"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
        warning: methods `data`, `outcome`, and `nested_in` are never used
           --> tests/steps/development_timeline_support.rs:521:12
        … (the seeded-history builder landed whole; Tasks 26 and 27 use those
        setters, so this warning disappears with the next scenario's fixture)
        warning: `givn` (test "features_runner") generated 1 warning
            Finished `test` profile [unoptimized + debuginfo] target(s) in 7.94s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report ranks the longest single step first
           ✔  Given a change "demo" whose record holds steps of different durations
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report ranks the longest step first
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:9:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2550632) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — exactly the failure this section
        predicts. The scenario's ranking Then step matches no function, and the
        run stops there: the fixture Given (a change whose seeded record holds
        steps of different durations, seeded through the store's own append
        path) and the report-generating When are in place, so the RED is the
        missing ranking assertion rather than a broken fixture. The report the
        When writes exists but ranks nothing — `build_report_model` carried only
        the scope and the record lines — so no honest assertion could pass at
        this revision.
        ```
  - [x] **GREEN** — implement the model's ranking and the renderer's summary.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `src/main.rs` (dispatch), `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.33s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report ranks the longest single step first"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.28s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report ranks the longest single step first
           ✔  Given a change "demo" whose record holds steps of different durations
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report ranks the longest step first
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `ReportModel` gains `bands` and
          `summary`; `build_report_model` assembles the scope's band and the
          window's summary; `Summary` carries the entry count, the elapsed time,
          the unmeasured-duration count, and `longest_steps`; `ranked_steps` is
          the ranking (recorded `duration_ms`, longest first, file order never
          consulted); `format_duration` is the report's one duration renderer;
          `render_report` fills the asset's view marker and its single data slot;
          `escape` owns the document's text path);
        - `assets/timeline/report.html` (modified — the design system's tokens
          carried inline, the shell with the labelled `h1` and the details
          region, the view marker, and the model's data slot as a
          `report-model` JSON element);
        - `tests/steps/development_report_steps.rs` (modified — the fixture
          Given, the report-generating When, the ranking Then, and the
          model/list readers the assertions use);
        - `tests/steps/development_timeline_support.rs` (modified — the
          seeded-history builder: `SeededHistory` writes a fixture record
          through the store's own append path, so every id is minted and every
          parent link resolved exactly as production mints them);
        - `src/main.rs` (unchanged — the `timeline` dispatch landed with the
          Task S3 skeleton and already calls `cli::timeline::run`, so this
          scenario adds no dispatch change).
        ```
  - [x] **REFACTOR** — one ranking implementation reused by the summary and the
        axis; re-run → still exit 0. Paste it.
        ```
        One aggregation pass feeds every summary number: `Summary::of` walks the
        window's entries once — the entry count, the earliest and latest recorded
        end for the elapsed time, and the spans whose duration the record does
        not carry — and the superseded per-number scans (`elapsed_ms`, the
        `unmeasured_durations` filter) are gone. The ranking lives in the single
        `ranked_steps` function the model's summary carries, so the summary list,
        the axis (Task 29) and the details view all read the record's one
        ranking instead of sorting again. No behaviour change: same ranking,
        same totals, same rendered text.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.99s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report ranks the longest single step first"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 12.13s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report ranks the longest single step first
           ✔  Given a change "demo" whose record holds steps of different durations
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report ranks the longest step first
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `12d2f04d162f7388238d2f4a0653cfa3facc5870` — `feat(development-report): <release note>` with
        the scenario title `The report ranks the longest single step first` in
        the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The report ranks the longest single step first"
        [16:33:30] committed revision on the Git backend
        EXIT=0

        Revision 12d2f04d162f7388238d2f4a0653cfa3facc5870 touches exactly this
        scenario's files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        the two step files (`development_report_steps.rs` and the
        `development_timeline_support.rs` seeded-history builder),
        `tests/steps/mod.rs` (the skeleton's `#[allow(dead_code)]` allowance on
        the report slot, removed now that the capability's first step reads it),
        the delta `.feature`'s `@wip` removal, and this task file's evidence.
        ```

## Task 26 — `The report totals the time spent waiting for questions` (`development-report`, added)

- [ ] **Scenario: `The report totals the time spent waiting for questions`**
  - Design constraints:
    - Summary field: the question-wait **total** and **count** built from the
      `question`-kind entries' measured durations (Task 14's entries). A wait
      with an unmeasured duration renders "not measured" and is excluded from
      the duration total while still counting as a wait (missing data is never
      summed as zero).
    - The wait total must be derived from the entries, not from turn durations:
      waiting is its own span (the requester's "waiting is indistinguishable from
      working" problem).
    - Fixture: change `demo` whose record holds two question waits with known
      durations, built by the support file's seeded-history builder.
    - Assertion shape: the written report names the total wait duration and the
      number of waits (rendered text and embedded model agree).
    - RED expected failure: the totals Then step is undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals the time spent waiting for questions"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the wait total/count in the model and its summary
        tile. Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — one aggregation pass over the entries feeding every
        summary number; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report totals the time spent waiting for
        questions` in the body. Record the revision identifier.

## Task 27 — `The report totals token usage per turn and per subagent` (`development-report`, added)

- [ ] **Scenario: `The report totals token usage per turn and per subagent`**
  - Design constraints:
    - Token totals come from the `usage`-kind entries (Task 15's `data` set:
      `input`, `output`, `cache_read`, `cache_write`, `total_tokens`, `model`,
      `subagent`): the report names the change's total tokens, the tokens of
      every subagent, and the total per model that produced usage.
    - Gap honesty: absent token usage renders "unavailable" and is never summed
      as zero; the summary states how many turns were unmeasured rather than
      folding them into the totals; a harness that exposes no per-subagent usage
      contributes a subagent row whose token column reads "unavailable" while
      session-level totals still include those tokens (Task 16's state renders
      here).
    - Fixture: change `demo` whose record holds reported usage for at least two
      models and one subagent, plus one turn with unavailable usage.
    - Assertion shape: the written report names the change total, each
      subagent's tokens, every model's total, and the unmeasured-turn count.
    - RED expected failure: the three Then steps are undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals token usage per turn and per subagent"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the usage roll-ups and the unmeasured count.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — one usage aggregation shared by the per-model and
        per-subagent views; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report totals token usage per turn and per
        subagent` in the body. Record the revision identifier.

## Task 28 — `The report of an ideation topic names the work done before the change exists` (`development-report`, added)

- [ ] **Scenario: `The report of an ideation topic names the work done before the change exists`**
  - Design constraints:
    - Topic window: `givn timeline --topic timeline-ideas` renders the topic's
      own entries — the part of an ideation that exists before any change
      directory does (design-review Q4). Ideation reporting is not a special case
      of the change path: a topic window is the topic's entries, and its band
      carries the topic's session work.
    - Bands are the report's rows: one per archived change, one per change or
      topic in flight, and the "outside a scope" band in the project report.
      Nothing is invented: the report names only what the record holds, so it
      names no change that does not exist yet.
    - Scope of the assertion: the report names the topic's steps under the topic
      scope and names no non-existent change — the written file's model and text,
      read from the command's own output.
    - Fixture: an ideation topic `timeline-ideas` whose record holds the topic's
      own steps (the `{"kind":"ideation","id":"timeline-ideas"}` scope from
      Task 7) and no change directory.
    - The open-report `@e2e` outline (Task E6) gains its `topic` Example from this
      same action, so the fixture must be reusable rather than scenario-local.
    - RED expected failure: both Then steps are undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report of an ideation topic names the work done before the change exists"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the topic window and its band. Files:
        `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs` (topic fixture). Compile
        first, re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — one window-selection rule for change, topic, and project
        (no third copy of the scope switch); re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report of an ideation topic names the work done
        before the change exists` in the body. Record the revision identifier.

## Task 29 — `The report orders the timeline by recorded time, not by store position` (`development-report`, added)

- [ ] **Scenario: `The report orders the timeline by recorded time, not by store position`**
  - Design constraints:
    - Ordering rule (design-review Q5): within every band the spans are ordered
      by `at` (the entry's recorded end timestamp), never by the entry's line
      position in the store — the store is append-only and concurrent writers
      make file order and time order different things, while `at` is the only
      ordering the record guarantees.
    - Fixture: a record whose **file order** contradicts time order — an entry
      appended after an entry carrying a later `at` (exactly what a late intake
      flush produces). The step asserts the two orders differ, so the assertion
      discriminates.
    - Assertion shape: the report orders the entries by their recorded end time
      (the model's ordered spans equal the entries sorted by `at`), and the
      rendered axis/tick order agrees.
    - Do not sort by `id` or by insertion: `id` is minted at append time under
      the append lock and is therefore also file-ordered — that is the bug this
      scenario exists to prevent.
    - RED expected failure: the ordering Then step is undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report orders the timeline by recorded time, not by store position"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the `at` ordering in the model. Files:
        `src/cli/timeline.rs`, `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs` (the out-of-order seed).
        Compile first, re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — one ordering comparator applied to every band; re-run →
        still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report orders the timeline by recorded time, not
        by store position` in the body. Record the revision identifier.

## Task 30 — `The report names the agent's fan-out into small subagents` (`development-report`, added)

- [ ] **Scenario: `The report names the agent's fan-out into small subagents`**
  - Design constraints:
    - Summary field: the number of subagents and their combined duration, from
      the `subagent`-kind entries (Task 13). This is the view that makes "a dozen
      tiny subagents whose combined overhead exceeds the work" visible — one of
      the three questions the requester named.
    - A subagent with an unmeasured duration renders "not measured" and is
      excluded from the combined duration while still counting toward the
      subagent count.
    - Subagent spans nest inside the step or turn that spawned them (`parent`),
      so the fan-out figure must be computed from the recorded spans, never by
      inferring spawn relationships from label text.
    - Fixture: change `demo` whose record holds many short subagents (support
      file's seeded-history builder).
    - Assertion shape: the written report names the number of subagents and their
      combined duration.
    - RED expected failure: the fan-out Then step is undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report names the agent's fan-out into small subagents"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the subagent count and combined duration. Files:
        `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — reuse Task 26's aggregation for the subagent durations;
        re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report names the agent's fan-out into small
        subagents` in the body. Record the revision identifier.

## Task 31 — `The report needs no network access to render` (`development-report`, added)

- [ ] **Scenario: `The report needs no network access to render`**
  - Design constraints:
    - Self-containment (D4 + the report technology decision): one file with
      inline CSS + vanilla JS + inline SVG, no third-party JavaScript, no CDN,
      no server, no build step, no `fetch`, no remote resource of any kind, and
      no `Date.now()` (the axis derives from the embedded timestamps, so the same
      record always renders identically).
    - Assertion shape — a resource-reference scan of the written file: no
      `<script src=`, no `<link href=`, no `<img src=`, no `@import`, no
      protocol-relative `//` resource URL, no `url(http…`, no `fetch(` and no
      `XMLHttpRequest` anywhere in the document or its inline script. The SVG
      namespace declaration (`xmlns="http://www.w3.org/2000/svg"`) is a namespace
      URI, not a resource request — the assertion must not fail on it, and it
      must not be "fixed" by removing the namespace.
    - Second half of the claim, kept as the scenario's own evidence: the command
      writes and prints the file with no network available (the suite runs with
      no service to reach, which is the Local Runnability statement).
    - Fixture: change `demo` with a recorded history (seeded builder).
    - RED expected failure: the no-remote-resource Then step is undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report needs no network access to render"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — render the asset with zero external references. Files:
        `assets/timeline/report.html`, `src/cli/timeline.rs` (escaping and the
        data-slot substitution), `tests/steps/development_report_steps.rs`.
        Compile/write the asset, re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — keep every style/script inline in one document, no
        separate assets to load; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report needs no network access to render` in the
        body. Record the revision identifier.

## Task 32 — `The report shows the empty state for a scope with no recorded activity` (`development-report`, added)

- [ ] **Scenario: `The report shows the empty state for a scope with no recorded activity`**
  - Design constraints:
    - Extension of the owning use case: no activity is recorded for the
      requested scope → the report renders an explicit empty state; it never
      shows a blank page and never errors. The report model marks a scope with no
      entries `empty`, and the screen renders the empty state.
    - Design-system primitive (must hold): the empty state carries a heading, one
      sentence explaining the state, and a way back to the report root; **no
      chart axes are drawn**; the primitive's states are `empty` and `error`; a
      summary tile with a zero value renders `0`, never a blank.
    - This is the **regular, report-level** traversal of the declared `empty`
      state; the browser traversal — and the state's evidence files — belong to
      Task E9.
    - Fixture: change `demo` with no recorded activity (no store, or an empty
      store); the report is generated with `--out report.html`.
    - Assertion shape: the written report shows the empty state for `demo` (the
      model's `empty` marker plus the rendered empty-state text) and draws no
      timeline axis for it (no axis markup/labels in the document).
    - RED expected failure: both Then steps are undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report shows the empty state for a scope with no recorded activity"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the `empty` marker and the empty-state rendering.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — let the empty state be one branch of the report root, not
        a special-cased template; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `The report shows the empty state for a scope with no
        recorded activity` in the body. Record the revision identifier.

## Task 33 — `A dense record is reported as aggregates instead of raw ticks` (`development-report`, added)

- [ ] **Scenario: `A dense record is reported as aggregates instead of raw ticks`**
  - Design constraints:
    - Aggregation rule: the timeline never draws one shape per entry when that
      would be unreadable. At the initial zoom, entries shorter than the
      time-per-pixel of the visible range — dense agent ticks and short steps in
      particular — collapse into **one aggregate marker per band and time
      bucket** that names how many entries it covers and their combined duration;
      a click on an aggregate zooms to its bucket instead of opening a details
      panel for an entry that stands for many. Individual spans replace their
      aggregate as the zoom narrows.
    - The summary and the longest-step ranking always cover the **full record**,
      never only the visible window.
    - This is a rendering rule, not a second interaction: it adds no inventory
      row, no matrix row, and no `@e2e` scenario — the zoom scenario (Task E8) is
      the axis-state evidence.
    - Fixture: change `demo` whose record holds hundreds of short agent
      activities (the seeded builder), so the initial zoom must aggregate.
    - Assertion shape: the model/markup carries aggregate markers naming their
      covered-entry count and combined duration; no individual span is drawn for
      those entries at the initial zoom; the summary's entry count still equals
      the full record's entry count.
    - RED expected failure: all three Then steps are undefined.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::A dense record is reported as aggregates instead of raw ticks"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the bucket aggregation and the aggregate marker.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) -->
        ```
  - [ ] **REFACTOR** — one bucketing function used by the initial zoom and by
        the replacement-on-zoom path; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `feat(development-report): <release note>` with
        the scenario title `A dense record is reported as aggregates instead of
        raw ticks` in the body. Record the revision identifier.

## Task E1 — E2E setup: local environment, browser tier, strictness, tags, evidence

- [ ] **E2E setup task (no scenario is written until every box here is filled in)**
  - Design constraints:
    - **Local environment / Local Runnability.** One-time browser setup, then
      the whole verification surface: `./setup-e2e-browser.sh`,
      `./run-tests.sh --non-e2e`, `./run-tests.sh --e2e`. `setup-e2e-browser.sh`
      is a new root-level script next to `run-tests.sh`, `measure-coverage.sh`,
      and `merge-coverages.sh`; it runs
      `npm ci --prefix tests/e2e && npx --prefix tests/e2e playwright install chromium`
      and it is the **only** step in this change that touches the network.
      Nothing listens on a port, nothing is fetched at run time, and **no digital
      twin exists or is needed** (`design.md` states this explicitly rather than
      leaving it blank): the record and the report are local files and the
      browser loads the report from a `file://` URL inside the temporary project
      root the scenario created, so no scenario can depend on a service that is
      "already running".
    - **E2E framework.** cucumber-rs (the project's existing runner) plus
      Playwright as the browser driver. cucumber-rs drives the CLI directly (it
      is the real interface); for the browser tier it drives Playwright through
      the JSON-line driver — the default driver named by the design instruction.
      An HTTP client would be wrong here: the report is a rendered HTML/JS
      surface with an interactive SVG, so only a browser can click, zoom, and
      read the rendered result. Node.js is resolved at setup time (the design
      machine reported `v22.20.0`; Givn does not pin it); Playwright **1.64.1**
      is pinned in `tests/e2e/package.json`; the browser under test is the
      Chromium the resolved Playwright bundles (`npx playwright install
      chromium`), never a system browser.
    - **Driver (`tests/e2e/browser-driver.mjs`).** Starts **one** browser and
      **one** context per scenario and accepts commands as JSON lines on stdin;
      the Rust step helper owns the process for the scenario's lifetime and no
      step opens a second context. It navigates with
      `page.goto("file://…")` (the report is one file with inline script and no
      imports, so there is no module resolution, no CORS, and no `fetch`), it
      emulates `prefers-reduced-motion: reduce` for the context, it awaits
      `document.fonts.ready` and a `requestIdleCallback` tick before a capture,
      and it exits on stdin EOF and on a `quit` command.
    - **Rust side (`tests/common/browser.rs`).** Spawns the driver, sends JSON
      commands, captures screenshots, and turns a non-zero driver exit, a
      timeout, or a JSON `{"error": …}` reply into a **failed** step — never into
      a skipped one. It kills the child on drop, and the scenario's `TempDir`
      removal is what deletes the report, so a stray browser cannot survive a
      failure.
    - **E2E step files — one per capability, separate from the unit/integration
      files** (design.md §Step Definitions / §E2E Smoke Test Infrastructure):
      `tests/steps/development_record_e2e_steps.rs`,
      `tests/steps/agent_activity_reporting_e2e_steps.rs`,
      `tests/steps/development_report_e2e_steps.rs`, plus the shared
      `tests/common/browser.rs` helper. Register all three in
      `tests/steps/mod.rs`; the shared harness drivers live in
      `tests/steps/development_timeline_support.rs` and are reused (never
      copied): omp — `node` imports the generated `.omp/hooks/pre/givn-timeline.js`,
      calls its factory with a fake `pi` (`on`, `events.on`, `exec`,
      `getThinkingLevel`) and fires synthetic tool/turn/subagent events;
      opencode — `node` imports the generated
      `.opencode/plugins/givn-timeline.js`, calls `setup` with a fake `ctx` that
      captures `session.hook`/`tool.hook`/`shell.hook` plus an
      `event.subscribe` async iterable, and emits the synthetic events; Claude
      Code — the generated `.claude/hooks/givn-timeline.sh` shim is executed
      directly with a real hook payload on stdin (`PreToolUse`, `PostToolUse`,
      `AskUserQuestion`, `PermissionRequest`, `SubagentStart`/`Stop`, `Stop`),
      `PATH` shimming `givn` to the built binary, with a fixture transcript at
      `transcript_path`. Every reporter calls the real `givn history ingest`.
    - **Local test infrastructure.** The built `givn` binary in a `tempfile`
      project root; `node` on `PATH`; the Playwright-installed Chromium; no
      server, port, database, or container, and no harness credentials.
    - **Strictness proof for the e2e tier.** `GivnWorld::cucumber()
      .fail_on_skipped()` plus the driver's error contract above. Run the same
      probe procedure as Task S1 against an e2e step file (one temporary
      `unimplemented!("TODO: development-timeline")` binding in the e2e tier,
      executed through the browser or CLI path it guards) and paste the non-zero
      exit; then remove the probe. A browser step must never degrade into a skip.
    - **`verify.e2e_command`** stays exactly `./run-tests.sh --e2e` (the command
      `design.md` §E2E Smoke Test Infrastructure names). It is never the same
      string as `verify.command` (`./run-tests.sh --non-e2e`): if the two are
      byte-identical the "e2e run" is silently re-running the whole suite, which
      is a hard-stop, fix-before-continuing condition.
    - **Scenario-count proof (the subset must be real).** `verify.command` runs
      the `not @e2e` tag set and `verify.e2e_command` the `@e2e` set; with this
      change in scope that is 35 vs 8 units, so the e2e count is strictly less
      than the full count. Prove it with the runner's own listing (or the
      summary line of each run) and paste both counts:
      `GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e`
      → 35 scenarios, `GIVN_FEATURES=givn/changes/development-timeline/specs
      ./run-tests.sh --e2e` → 8 scenarios.
    - **Health check before any scenario step.** `npx --prefix tests/e2e
      playwright --version` reports 1.64.1 and the driver can launch Chromium and
      load a `file://` page (a trivial smoke page in a `tempfile` root, or the
      report generated by `givn timeline` for a seeded record). Paste the smoke
      run's exit 0.
    - **CI and release prerequisites (implementation requirements).**
      `.github/workflows/ci.yml` and `.github/workflows/release.yml` gain
      `./setup-e2e-browser.sh` **before their first suite invocation**, and the
      regular workflow step runs `./run-tests.sh --non-e2e` followed by
      `./run-tests.sh --e2e` (today's bare `./run-tests.sh` plus
      `./run-tests.sh --e2e` would run the browser tier twice once the regular
      scope narrows). Node is already on both runners; the
      Playwright-bundled Chromium download is the new prerequisite. Commit those
      workflow edits with the e2e setup, not with a scenario.
    - **Evidence capture mechanism (the visual gate reads it).** `design.md`'s
      Visual Design Contract declares `Interface: web`, `Viewports: mobile
      (360x780), desktop (1920x1080)`, `States: empty`. `src/integrity_hook.rs::check_visual_evidence`
      requires, for **every** delta `@e2e` scenario title, a non-empty
      `givn/changes/development-timeline/evidence/visual/<scenario-slug>/mobile.png`
      and `…/desktop.png`, plus a non-empty `<viewport>--empty.png` for the
      declared `empty` state — so the CLI `@e2e` scenarios commit the report they
      produced, rendered in the browser, at both viewports, and the empty-scope
      scenario (Task E9) commits `mobile--empty.png` and `desktop--empty.png`
      beside its final-state pair. `scenario_slug` follows
      `givn::integrity_hook::scenario_slug` (lowercase ASCII, every run of
      non-alphanumeric characters becomes one `-`, leading and trailing `-`
      removed). The eight slugs: `consumer-runs-a-lifecycle-step-and-reads-its-recorded-actions`,
      `consumer-reads-the-development-record-as-machine-readable-output`,
      `consumer-installs-the-agent-reporter-and-its-reported-activity-lands-in-the-record`,
      `consumer-generates-the-development-report-for-a-change`,
      `consumer-opens-the-generated-development-report-in-a-browser`,
      `consumer-selects-a-timeline-element-and-reads-its-details`,
      `consumer-zooms-the-timeline-to-a-shorter-time-range`,
      `consumer-opens-the-report-of-a-scope-with-no-recorded-activity`.
      Intermediate captures, if any, go to `reports/visual/` and are not review
      evidence. Capture happens with animations disabled (reduced motion) and the
      report renders from embedded timestamps only, so a capture never depends on
      timing.
    - **Primary-assertion rule (canonical policy).** Read
      `givn instructions specs --change development-timeline` before writing or
      reviewing any e2e step: the browser scenarios assert on what the browser
      rendered (DOM text, accessible names, the rendered range label), never on
      the report file's raw bytes; the CLI scenarios assert on the subprocess's
      stdout and the files it wrote; repository checks are secondary only. For
      the browser tier the driver drives the real browser (click/type/navigate) —
      an HTTP request or an in-page `fetch()` is never a substitute. On a
      real-interface obstacle, apply `design.md`'s Anticipated Interface
      Obstacles table rather than weakening the assertion.
    - **`@e2e` tags are immutable**; nothing in this task or the following
      sections may remove one.
  - [ ] **Sub-task 1 — the local environment and the browser toolchain.**
        Create `setup-e2e-browser.sh` and `tests/e2e/package.json` (Playwright
        pinned to 1.64.1), run the script, and paste its output plus
        `npx --prefix tests/e2e playwright --version`.
        ```
        <!-- paste: setup-e2e-browser.sh + playwright --version -->
        ```
  - [ ] **Sub-task 2 — the driver and the Rust browser helper.** Create
        `tests/e2e/browser-driver.mjs` and `tests/common/browser.rs` with the
        contracts above (one context per scenario, `file://` navigation,
        reduced-motion emulation, deterministic capture, failure-never-skip
        error contract).
        ```
        <!-- paste: the two files' key sections + a smoke run that exits 0 -->
        ```
  - [ ] **Sub-task 3 — the e2e step files and the harness drivers.** Create the
        three `*_e2e_steps.rs` files, register them, wire the `GivnWorld` slots,
        and extend `tests/steps/development_timeline_support.rs` with the three
        harness drivers.
        ```
        <!-- paste: the registrations + `cargo build --locked --tests` output -->
        ```
  - [ ] **Sub-task 4 — the e2e strictness proof.** Temporary
        `unimplemented!("TODO: development-timeline")` binding in the e2e tier,
        run scoped, paste the non-zero exit, remove the probe, re-run the delta
        listing to show the e2e unit count is unchanged.
        ```
        <!-- paste: probe command + output (non-zero), then the removal + count -->
        ```
  - [ ] **Sub-task 5 — `verify.e2e_command` and the scenario-count proof.**
        Confirm `givn/commands.yaml` declares `verify.command:
        ./run-tests.sh --non-e2e` (Task S2) and `verify.e2e_command:
        ./run-tests.sh --e2e`; paste both scope runs' counts.
        ```
        <!-- paste: both commands' scenario counts (35 vs 8, e2e strictly less) + the two config lines -->
        ```
  - [ ] **Sub-task 6 — the local health check.** Run the driver against a
        trivial `file://` page (or a freshly generated report) inside a temp
        project root and paste the exit-0 smoke result.
        ```
        <!-- paste: the smoke command + output -->
        ```
  - [ ] **Sub-task 7 — CI/release browser prerequisites.** Add
        `./setup-e2e-browser.sh` before the first suite invocation in
        `.github/workflows/ci.yml` and `.github/workflows/release.yml`, and make
        the regular workflow step `./run-tests.sh --non-e2e` before
        `./run-tests.sh --e2e`.
        ```
        <!-- paste: both diffs -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the subject
        `E2E setup: browser tier, strictness, tag scopes, evidence` — the e2e
        setup lands as its own commit (never folded into a scenario), before the
        first `@e2e` scenario's RED. Record the revision identifier.

## Task E2 — `Consumer runs a lifecycle step and reads its recorded actions` (`@e2e`, `development-record`)

- [ ] **Scenario: `Consumer runs a lifecycle step and reads its recorded actions`**
  - Design constraints:
    - Matrix row: `development-record: run a lifecycle command` — real interface
      **CLI**, driving mechanism `common::run_givn`: `givn status --change demo`,
      then `givn history --change demo --json`; the assertion reads the printed
      record.
    - Steps live in `tests/steps/development_record_e2e_steps.rs`; the record
      read is the `givn history` read path in `src/cli/history.rs` (with
      `givn history` excluded from recording, Task 9).
    - Primary assertion: the subprocess's stdout — exit 0, the machine-readable
      record names the `status` command for `demo`, and that command's entry
      carries a duration. Repository/store checks are secondary.
    - E2E command and driver: targeted run with the canonical `--exact` command
      below; scope run `./run-tests.sh --e2e`; no browser driver for this
      scenario (CLI interface) — the browser is used only for its evidence
      capture.
    - **Evidence (the change-wide visual gate):** this CLI scenario commits the
      report it produced, rendered in the browser, at both viewports under
      `givn/changes/development-timeline/evidence/visual/consumer-runs-a-lifecycle-step-and-reads-its-recorded-actions/`
      — `mobile.png` (360x780) and `desktop.png` (1920x1080) — captured with
      animations disabled via `tests/e2e/browser-driver.mjs`.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer runs a lifecycle step and reads its recorded actions"`
        → must exit non-zero (the e2e steps are undefined). Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the e2e steps on the real CLI. Files:
        `tests/steps/development_record_e2e_steps.rs`,
        `src/cli/history.rs` (any read-path gap), plus the two evidence
        screenshots above. Compile first, re-run → exit 0; then run
        `./run-tests.sh --e2e` at least once for this scenario's scope health.
        Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — reuse the shared run-helper instead of a second
        invocation helper; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer runs a lifecycle step and reads its recorded actions` in
        the body (including the two evidence PNGs). Record the revision
        identifier.

## Task E3 — `Consumer reads the development record as machine-readable output` (`@e2e`, `development-record`)

- [ ] **Scenario: `Consumer reads the development record as machine-readable output`**
  - Design constraints:
    - Matrix row: `development-record: read the development record` — real
      interface **CLI**, driving mechanism `common::run_givn`:
      `givn history --change demo --json` against a seeded record; the assertion
      parses the printed entries.
    - The machine-readable view is `givn history --json` (D4 dropped a JSON mode
      on `givn timeline`): the output names every recorded entry with its `kind`,
      its `scope`, and its `duration`.
    - Steps in `tests/steps/development_record_e2e_steps.rs`; the seeded record
      comes from the shared builder in
      `tests/steps/development_timeline_support.rs`.
    - Primary assertion: the subprocess's stdout, parsed as JSON — one entry per
      recorded line, each with kind, scope, and duration; exit 0. Nothing about
      the assertion may read the store privately instead.
    - E2E command and driver: the canonical `--exact` command below; scope run
      `./run-tests.sh --e2e`; CLI interface, no browser driving.
    - **Evidence:** commit the report produced from the seeded record, rendered in
      the browser, at both viewports under
      `givn/changes/development-timeline/evidence/visual/consumer-reads-the-development-record-as-machine-readable-output/`
      — `mobile.png` and `desktop.png`, animations disabled.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer reads the development record as machine-readable output"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the JSON read output and the parsing steps. Files:
        `src/cli/history.rs`, `tests/steps/development_record_e2e_steps.rs`,
        plus the two evidence screenshots. Compile first, re-run → exit 0. Paste
        it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — one entry-to-JSON mapping shared by the read paths;
        re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer reads the development record as machine-readable output`
        in the body (including the two evidence PNGs). Record the revision
        identifier.

## Task E4 — `Consumer installs the agent reporter and its reported activity lands in the record` (`@e2e`, `agent-activity-reporting`, Scenario Outline, three Examples rows)

- [ ] **Scenario: `Consumer installs the agent reporter and its reported activity lands in the record`** (Examples: `omp`, `opencode`, `claude`)
  - Design constraints:
    - Matrix row: `agent-activity-reporting: report agent activity from a
      supported harness` — real interface **CLI (harness integration)**: install
      the project (`givn init --agents <target>`), then drive that harness's
      **generated** reporter through its real entry point, and assert on
      `givn history --change demo --json`. Every reporter calls the real
      `givn history ingest`.
    - Per-harness drivers (reused from `tests/steps/development_timeline_support.rs`,
      never re-implemented): **omp** — `node` imports
      `.omp/hooks/pre/givn-timeline.js` and calls its factory with a fake `pi`
      (`on`, `events.on`, `exec`, `getThinkingLevel`), firing synthetic
      tool/turn/subagent events; **opencode** — `node` imports
      `.opencode/plugins/givn-timeline.js` and calls `setup(ctx)` with a fake
      `ctx` capturing `session.hook`/`tool.hook`/`shell.hook` plus an
      `event.subscribe` async iterable, then emits synthetic events;
      **claude** — the generated `.claude/hooks/givn-timeline.sh` is executed
      with a real hook payload on stdin, `PATH` shimming `givn` to the built
      binary and a fixture transcript at `transcript_path` (the mapper runs in
      the binary).
    - Steps in `tests/steps/agent_activity_reporting_e2e_steps.rs`; the
      installation itself goes through `givn init --agents <target>` (never a
      hand-written file).
    - Primary assertion: the subprocess's stdout for the real reporter run and
      for `givn history --change demo --json` — the machine-readable record names
      the tool call, the subagent, and the question wait.
    - **Evidence:** one slug for the outline
      (`consumer-installs-the-agent-reporter-and-its-reported-activity-lands-in-the-record/`)
      — commit `mobile.png` and `desktop.png` from the record the run produced
      (the three Examples runs write the same two paths; the committed pair must
      be the non-empty output of the final run).
    - `@e2e` tag stays; the outline's three rows are input variation of one
      action, so they add no inventory row and no extra section.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the three driver paths and the assertions. Files:
        `tests/steps/agent_activity_reporting_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`,
        `src/cli/history.rs`/`src/skills/targets.rs` for any gap the real
        entry point exposes, plus the two evidence screenshots. Compile and
        `node --check` the drivers first, re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — one driver entry point per harness, invoked through the
        same step helpers; re-run (all three rows) → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer installs the agent reporter and its reported activity
        lands in the record` in the body (including the two evidence PNGs).
        Record the revision identifier.

## Task E5 — `Consumer generates the development report for a change` (`@e2e`, `development-report`)

- [ ] **Scenario: `Consumer generates the development report for a change`**
  - Design constraints:
    - Matrix row: `development-report: generate the development report for a
      change` — real interface **CLI**, driving mechanism `common::run_givn`:
      `givn timeline --change demo --out report.html`; the assertion reads the
      written file.
    - Phase derivation rule (asserted concretely here): Givn records one `phase`
      entry when an artifact file of the change appears, and an entry belongs to
      the stage of the latest such transition at or before its own `at`. The
      fixture seeds a record whose **last recorded artifact transition is
      `tasks.md`**, and the step recorded after that transition must land in the
      **`implement`** phase.
    - Assertions: exit 0; the command prints the path of the written report; the
      report file exists and names the recorded phases; the post-transition step
      is placed in the `implement` phase. Additional assertion allowed by the
      same design decision: the default path form (flagless `givn timeline
      --change demo`) writes and prints
      `givn/history/reports/demo.html` — D4's named default.
    - Steps in `tests/steps/development_report_e2e_steps.rs`; the seeded record
      (including the phase transitions) comes from the shared builder.
    - **Evidence:** `givn/changes/development-timeline/evidence/visual/consumer-generates-the-development-report-for-a-change/`
      — `mobile.png` and `desktop.png` of the written report, rendered in the
      browser, animations disabled.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer generates the development report for a change"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the printed path, the phase derivation, and the
        default output path. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`, plus the two evidence
        screenshots. Compile first, re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — one phase-derivation function used by the CLI and the
        renderer; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer generates the development report for a change` in the
        body (including the two evidence PNGs). Record the revision identifier.

## Task E6 — `Consumer opens the generated development report in a browser` (`@e2e`, `development-report`, Scenario Outline, `change` and `topic` rows)

- [ ] **Scenario: `Consumer opens the generated development report in a browser`** (Examples: `change`, `topic`)
  - Design constraints:
    - Matrix row: `development-report: open the report in a browser` — real
      interface **Web UI**, driving mechanism Playwright:
      `page.goto("file://<root>/report.html")`, then read the rendered heading,
      the timeline SVG, and the longest step's duration from the DOM. One outline
      with a `change` row and a `topic` row: one normalized action with input
      variants, so it adds no second inventory row (design-review Q4).
    - Browser driving is real browser driving (navigate + read the rendered page)
      — an HTTP request or an in-page `fetch()` is never a substitute. The
      browser helper is `tests/common/browser.rs` over
      `tests/e2e/browser-driver.mjs` (one context per scenario, `file://`
      navigation, reduced-motion emulation).
    - Primary assertion: what the browser rendered — the report heading and the
      development timeline, and the longest recorded step with its duration
      (DOM text and accessible names).
    - Accessibility assertions from the Visual Design Contract belong to this
      scenario's rendered surface (the design leaves their placement to the e2e
      tier): the page has a real `title`, one `h1`, `header`/`main`/`aside`
      landmarks, the timeline `svg` carries an accessible name, every control is
      reachable by `Tab` with a visible focus ring, and every span exposes its
      name plus its duration.
    - Steps in `tests/steps/development_report_e2e_steps.rs`.
    - **Evidence:** `givn/changes/development-timeline/evidence/visual/consumer-opens-the-generated-development-report-in-a-browser/`
      — `mobile.png` and `desktop.png` (the populated report at both contract
      viewports), animations disabled.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the generated development report in a browser"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the browser-open steps and the report's heading,
        timeline SVG, and accessible names. Files:
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/common/browser.rs` / `tests/e2e/browser-driver.mjs` (any gap the
        real driver exposes), `assets/timeline/report.html` (semantics and
        labels), plus the two evidence screenshots. Compile first, re-run →
        exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — one page-open helper shared by the browser scenarios;
        re-run (both rows) → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer opens the generated development report in a browser` in
        the body (including the two evidence PNGs). Record the revision
        identifier.

## Task E7 — `Consumer selects a timeline element and reads its details` (`@e2e`, `development-report`)

- [ ] **Scenario: `Consumer selects a timeline element and reads its details`**
  - Design constraints:
    - Matrix row: `development-report: select a timeline element` — real
      interface **Web UI**, driving mechanism Playwright: **click** the longest
      step's span (or focus it and press `Enter`), then read the details panel's
      rendered text.
    - Details-panel primitive (design system): a labelled region showing start,
      end, duration, outcome, model, tokens, and the bounded label, with a
      labelled close control (`Escape` closes it); selection is a focusable
      element with a visible focus ring; the panel opens on `Enter`/`Space`.
    - Primary assertion: the browser shows a details panel for that step naming
      its start, its duration, and its outcome — read from the DOM after the real
      click/keystroke, never by parsing the report file.
    - Steps in `tests/steps/development_report_e2e_steps.rs`.
    - **Evidence:** `givn/changes/development-timeline/evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/`
      — `mobile.png` and `desktop.png` captured with the panel open at both
      contract viewports, animations disabled.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement selection and the details panel. Files:
        `assets/timeline/report.html` (span focus/activation, panel markup and
        labels), `tests/steps/development_report_e2e_steps.rs`, plus the two
        evidence screenshots. Re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — one selection path for mouse and keyboard; re-run → still
        exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer selects a timeline element and reads its details` in the
        body (including the two evidence PNGs). Record the revision identifier.

## Task E8 — `Consumer zooms the timeline to a shorter time range` (`@e2e`, `development-report`)

- [ ] **Scenario: `Consumer zooms the timeline to a shorter time range`**
  - Design constraints:
    - Matrix row: `development-report: zoom the timeline` — real interface
      **Web UI**, driving mechanism Playwright: click the zoom-in control, then
      read the rendered range label before and after and assert it **narrowed**.
    - **Rendered-state evidence (the change's own rendered-state proof):** the
      assertion reads a value the page's own script produced (the visible time
      range) and observes that it changed between two observations. Pan is an
      implementation detail of the same axis (design-review Q9) and adds no
      interaction, no row, and no scenario.
    - Zoom control primitive: a visible text label (`Zoom in` / `Zoom out` /
      `Fit`), never an icon alone, `disabled` at the zoom limit. The axis may not
      read the wall clock: the report renders from embedded timestamps only (no
      `Date.now()`), which is what makes the before/after comparison
      deterministic.
    - Motion: the zoom transition is `120 ms ease-out` and is removed entirely
      under `prefers-reduced-motion: reduce` (which the driver emulates, and with
      which the screenshots are captured) — assert the emulated preference and
      that the zoom then applies instantly.
    - Steps in `tests/steps/development_report_e2e_steps.rs`.
    - **Evidence:** `givn/changes/development-timeline/evidence/visual/consumer-zooms-the-timeline-to-a-shorter-time-range/`
      — `mobile.png` and `desktop.png` at the narrowed range, both viewports,
      animations disabled.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer zooms the timeline to a shorter time range"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement zoom (control, axis range label, reduced-motion
        behaviour) and the before/after assertion. Files:
        `assets/timeline/report.html`, `tests/steps/development_report_e2e_steps.rs`,
        plus the two evidence screenshots. Re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the two screenshot paths -->
        ```
  - [ ] **REFACTOR** — one range-state derivation for wheel, buttons, and
        aggregation; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer zooms the timeline to a shorter time range` in the body
        (including the two evidence PNGs). Record the revision identifier.

## Task E9 — `Consumer opens the report of a scope with no recorded activity` (`@e2e`, `development-report`)

- [ ] **Scenario: `Consumer opens the report of a scope with no recorded activity`**
  - Design constraints:
    - Matrix row: `development-report: open the report of a scope with no
      recorded activity` — real interface **Web UI**, driving mechanism
      Playwright: generate the report for a scope with no recorded activity,
      `page.goto("file://<root>/report.html")`, then read the rendered
      empty-state heading and the absence of any axis from the DOM.
    - This is the `@e2e` traversal of the contract's declared `empty` state
      (design-review Q1): the empty state carries a heading, one sentence
      explaining the state, and a way back to the report root; no chart axes are
      drawn; the primitive's states are `empty` and `error`.
    - Primary assertion: what the browser rendered — the empty state for `demo`
      and no timeline axis (DOM-level absence), never file bytes.
    - Steps in `tests/steps/development_report_e2e_steps.rs`.
    - **Evidence (this scenario carries the declared state's captures):**
      `givn/changes/development-timeline/evidence/visual/consumer-opens-the-report-of-a-scope-with-no-recorded-activity/`
      — the final-state pair `mobile.png` + `desktop.png` **and** the state pair
      `mobile--empty.png` + `desktop--empty.png`
      (`<viewport>--<state>.png` in the same directory, the layout the integrity
      hook enforces), animations disabled.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the report of a scope with no recorded activity"`
        → must exit non-zero. Paste it.
        ```
        <!-- paste RED evidence (command + output, non-zero exit) -->
        ```
  - [ ] **GREEN** — implement the empty-state browser assertions and capture
        both pairs. Files: `assets/timeline/report.html`,
        `tests/steps/development_report_e2e_steps.rs`, plus the four evidence
        screenshots. Re-run → exit 0. Paste it.
        ```
        <!-- paste GREEN evidence (command + output, exit 0) + the four screenshot paths -->
        ```
  - [ ] **REFACTOR** — one empty-state branch shared with Task 32's
        report-level rendering; re-run → still exit 0. Paste it.
        ```
        <!-- paste REFACTOR evidence (command + output, exit 0) -->
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title `Consumer opens the report of a scope with no recorded activity` in
        the body (including the four evidence PNGs). Record the revision
        identifier.

---

**Done when:** every box above is checked with its pasted evidence and revision
identifier, `verify.command` (`./run-tests.sh --non-e2e`) and
`verify.e2e_command` (`./run-tests.sh --e2e`) both exit 0 on the full suite, the
e2e setup task's scenario-count proof is pasted (35 regular vs 8 `@e2e`, e2e
strictly less), every `@e2e` scenario has committed `mobile.png` + `desktop.png`
under its slug with the empty-scope scenario's `mobile--empty.png` /
`desktop--empty.png`, and no `@wip` tag remains on any scenario this change
implements.
