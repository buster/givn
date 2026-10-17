# Tasks: development-timeline

Scenario-by-scenario RED→GREEN→REFACTOR→COMMIT loop. One section per executable
scenario (plus the extended-evidence Task 11b, which adds no scenario): the 43
non-`@e2e` scenario sections first, in `.feature` order (the three
`@givn.modified` entries are reconciled in S2b and Task 24a, not as scenario
tasks), then the E2E setup task, then the 8 `@e2e` scenarios in the same order.

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
`givn/changes/development-timeline/specs/` on 2026-10-16, before the hybrid
delta):** 54 executable scenario entries in
5 features = 51 `@givn.added`, 3 `@givn.modified` (the S2b corpus-pin
reconciliation plus the two target-list reconciliations in Task 24a),
0 `@givn.removed`; 46 non-`@e2e` + 8 `@e2e`. A `Scenario
Outline` counts as one entry (its Examples rows are input variation, not
additional scenarios). Every still-pending entry carries `@wip` today and stays
`@wip` until its own RED step removes the tag from that one scenario; the three
`@givn.modified` entries carry no `@wip` and are reconciled green in S2b and
Task 24a.

**Superseded by the hybrid delta (2026-10-17):** the current totals are **62
entries / 53 non-`@e2e` / 9 `@e2e`** with **15 `@wip`**, and the delta's 17 new
sections are **H1–H10** and **HE1–HE7** under **Delta part 2 — hybrid workflow
report** at the end of this file. The table below is the 2026-10-16 state and is
kept as history, not as the current count.

| Feature (path relative to `GIVN_FEATURES`) | Entries | Non-`@e2e` | `@e2e` |
|---|---|---|---|
| `inspect-development-history/development-record.feature` | 20 | 18 | 2 |
| `inspect-development-history/agent-activity-reporting.feature` | 15 | 14 | 1 |
| `inspect-development-history/development-report.feature` | 16 | 11 | 5 |
| `fragments/adr-qualification-gate.feature` (`@givn.modified`) | 1 | 1 | 0 |
| `initialize-project/initialize-a-project.feature` (`@givn.modified`, two scenarios) | 2 | 2 | 0 |
| **Total** | **54** | **46** | **8** |

**Work order (recorded choice).** `design.md` names no implementation sequencing
section, so this file follows `.feature` order — the features in the inventory
order above, scenarios in their in-file order — in three phases: Tasks 1–44
(the 46 non-`@e2e` scenarios: 43 added-scenario sections numbered Tasks 1–33,
**Task 11a**, the record-side capture tasks **11c–11h**, **Task 24a**, **Task
25a**, and **Task 25b**, plus the extended-evidence **Task 11b**), each inserted
with a suffix so the later task numbers stay stable, then Task E1 (E2E setup),
Tasks E2–E9 (the 8 `@e2e` scenarios, same feature order). The
three `@givn.modified` entries are not scenario tasks: ADR-073 raises the register
count the permanent `adr-qualification-gate` scenario pins, and the new `maki`
target extends the `all` targets list two permanent `initialize-a-project`
scenarios pin, so setup item **S2b** carries the first reconciliation and **Task
24a** carries the target-list reconciliation (both with their permanent patches),
alongside the scenario work they belong to.

**Delta part 2 (2026-10-17).** The hybrid workflow report supersedes the dense
timeline. Its 15 pending scenarios, the shared model scaffold, and the
evidence/scope-run task are appended after Task E9 as **H1–H10** and
**HE1–HE7**, non-`@e2e` first and `@e2e` last; their inventory, pre-flight, and
counts are stated in that part. Everything above stays as the recorded history
of the dense-timeline implementation.

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
parent link always names an existing entry), `session` (optional harness session
id, carried by every entry), `data` (only where the report needs a non-generic
value, e.g. `exit_code` on a step; `kind: "usage"` entries carry
`{input, output, cache_read, cache_write, total_tokens, model, subagent}`;
`question` entries always carry `data.question` = `{text, truncated, bytes}`;
`full`-mode content lives in `data.content`'s named fields, each
`{text, truncated, bytes}`, with the intake's 64 KiB/256 KiB caps).

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

**Deliberate exclusion:** `givn history`, `givn history settings`, and
`givn timeline` run **without** a recorder — reading the record never mutates
it. Every other command, including `givn history ingest`, records normally.
`src/main.rs::records` excludes the `History` read view and its `settings`
subcommand.

**Capture mode:** `record.mode: on | off | full` in `givn/config.yaml`, parsed
leniently through `src/config/overlay.rs::OverlayDoc` (the project-config parse
path — not `src/config/manifest.rs`), default `on`, and an invalid value falls
back to `on` with one warning. `off` is absolute: no append, no pending state,
no record-side ignore write, and `--with-full-content` is ignored with one
warning and exit 0. Lowering the mode is forward-looking: already-captured
entries are retained, never purged.

**Intake (`givn history ingest`):** reads one JSON object from stdin
(`schema`, `reporter`, `session`, `change`, `topic`, `events`). An event whose
`kind` is unknown, whose `at` is unparsable, or that lacks a required field for
its kind is rejected **individually**: it is named on stderr and the remaining
events are appended. Exit codes: `0` when the payload was read (rejected events
are warnings), `2` when stdin is not readable JSON at all; a record that cannot
be written warns and still exits `0`. The intake is the **single enforcement
site** for the mode and every bound: under `off` it appends nothing and warns
once; under `on` it drops `content` while keeping the event; it re-applies the
label 120, text-metadata 240, error 1 000, question 4 000, per-field 64 KiB, and
per-entry 256 KiB bounds to every event. `--with-full-content` overrides
`on`/`full` for one payload only; under `off` it is ignored with one warning and
exit 0. The Claude `harness_hook` enforces the same `off` rule in-process (no
append, no pending write, no ignore write).

**Mode query (`givn history settings`):** human output is
`record mode: <on|off|full> (source: <config|default>)`; `--json` prints
`{"mode":"on|off|full","source":"config|default"}`; both exit 0. An
unknown/unreadable value warns once and reports `on`/`default`; outside a Givn
project it exits 1 with `error: not a Givn project (givn/ not found)`. It is a
non-recorded read view (`source: flag` is unreachable and is not a value).

**Process-per-hook pending state** (Claude Code): one file per session at
`givn/history/pending/<harness>-<session-id>.json`, shaped
`{<tool_use_id>: {started_at, kind, label}}` — written atomically before the tool
runs, consumed by the matching end event, pruned by age (older than 24 h) and at
session end, no payload content. **No mode is cached in it:** Claude resolves
the mode in-process inside the binary's `harness_hook`, and under `off` writes
no pending file. Losing it costs a duration, never an entry.

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
the longest-step ranking always cover the full record. Details facts: the panel
renders session and title, status, stop reason, model, cost, context size and
window, tool call id, output bytes, bounded error, parent/agent, retry, and
permission mode (or "not reported"), plus one bounded, scrollable, escaped
content region when the entry carries `data.content`. Report content budget:
the model/payload embeds full content in record order up to **8 MiB** in total;
beyond it the affected entry carries `content_omitted: true` and the text
`content omitted; see the record`, while the store keeps every byte. Rendering:
the generator substitutes the asset's two markers — the model payload element
(`/*__GIVN_REPORT_DATA__*/`, JSON with `<` escaped) and the text-only initial
view (`<!--__GIVN_REPORT_VIEW__-->`) — and nothing else; the inline script builds
an SVG time axis, nests spans by `parent`, and wires zoom (wheel, buttons), pan
(drag), selection (click/`Enter`) and the details panel — it never calls
`fetch`, never loads a remote resource, and never uses `Date.now()`. Summary: total elapsed, the ranked longest entries, entry count,
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
  (46 scenarios, `verify.command` after Task S2 — the 43 non-`@e2e` scenario
  sections plus the three `@givn.modified` entries) and
  `GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e`
  (8 scenarios, `verify.e2e_command`). Run a `Scenario Outline`'s unit once: it
  executes all Examples rows in one process.
- **Step-definition ownership — one file per capability** (`design.md` §Step
  Definitions). Unit/integration steps:
  `tests/steps/development_record_steps.rs`,
  `tests/steps/agent_activity_reporting_steps.rs`,
  `tests/steps/development_report_steps.rs`. Shared, not per capability:
  `tests/steps/development_timeline_support.rs` (record fixtures, the
  seeded-history builder, and the four harness drivers — a fake `pi` extension
  context for omp, a fake plugin context capturing `session.hook` /
  `tool.hook` / `shell.hook` plus the event subscription for opencode, a
  payload-on-stdin invocation of the generated Claude shim with a fixture
  transcript, and for Maki the real-binary invocation plus the synthetic autocmd
  driver). Every module is registered in `tests/steps/mod.rs` and
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
| `development-record` privacy | The record keeps bounded metadata; question text is always stored in full (4 000-character cap), and complete command/tool content is stored only in `full` (D2, Tasks 11d/11g) | Tasks 8, 11d, 11g | `developer--primary-user`'s credentials and private code stay out of the store unless the project explicitly enables `full`. |
| `development-record` observation | Recording is the observation substrate: no gate, no proof, no exit code, no artifact lifecycle changes | Tasks 3, 6, 9, 21, 22 | `agent--technical-role` executes the same workflow; nothing it observes mutates. |
| `development-record` reading | Reading the record never extends it (`givn history`, `givn history settings`, `givn timeline`) | Task 9, Task 11f | `developer--primary-user` can re-read a report as often as wanted, and a mode query appends nothing. |
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
        seeded-history builder, the four harness drivers). Skeleton = module
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

## Task 11b — `The record and its content stay ignored in both backends` (extended evidence; no new scenario)

- [x] **Scenario extension: `The record stays out of the repository's own history`
      and `A Fossil project ignores the record through its own ignore file`**
  - Design constraints:
    - The two record-hygiene scenarios were extended in place (no new entry, so
      the inventory and `@e2e` accounting are unchanged) with the addendum's
      assertions: the fixture first writes a **real full-mode `data.content`
      entry** (both the Git and the Fossil arm), so the ignore property can
      actually fail; after it the worktree is still clean; after `givn timeline`
      nothing under `givn/history/` shows as pending; and in a Fossil checkout
      the effective `ignore-glob` covers the record, the content, and the
      reports, with `fossil extras`/status showing nothing under the record
      directory.
    - The Git scenario also covers the bare-root scaffold fallback: the project
      starts with `givn/` but no repository, the first full-content record write
      must leave the `# givn:begin:history` block in `.gitignore` (the
      scaffold-time safety net), and only then does `git init` happen; the
      pending-change assertions run against that repository.
    - Placement rule (design §Capture completeness): the record, the inline full
      content, the reports, and the pending state all live under
      `givn/history/`, which the managed block already covers (`givn/history/` in
      `.git/info/exclude`, re-asserted on every write; `givn/history/*` in the
      Fossil managed entry list; the scaffold `.gitignore` block before a
      repository exists). No new ignore destination.
    - Step files: `tests/steps/development_record_steps.rs` (the new
      assertions/steps for both backends).
    - RED expected failure: the new phrases are undefined — the full-content
      fixture step, the bare-root `.gitignore` assertion, the pending-state
      absence assertion, and the Fossil extra-file assertions.
  - [x] **RED** — run both scenarios before any step implementation; the new
        phrases are undefined, so the runner exits non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record stays out of the repository's own history
           ✘  Given a project root with givn/ initialised without a repository
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:77:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3804497) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A Fossil project ignores the record through its own ignore file
           ✔  Given a project root with givn/ initialised in a Fossil checkout
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✘  When the project records a full-content entry in the full mode
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:92:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (3804558) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN (evidence)** — implement the steps, re-run both scenarios and
        paste the exit-0 output, together with the raw `git status --porcelain`
        after a full-content capture and after `givn timeline`, the
        `.gitignore` fallback assertion, and the effective
        `fossil settings --value ignore-glob` plus `fossil extras`:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"`
        and the same for the Fossil scenario. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record stays out of the repository's own history"
             Finished `test` profile [unoptimized + debuginfo] target(s) in 4.09s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record stays out of the repository's own history
           ✔  Given a project root with givn/ initialised without a repository
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the project records a full-content entry in the full mode
           ✔  Then the project's ".gitignore" names the development record
           ✔  When the project becomes a Git repository
           ✔  Then the repository reports no pending change for the development record
           ✔  When the project records a full-content entry in the full mode
           ✔  Then the repository reports no pending change for the development record
           ✔  When the user runs `givn timeline --change demo`
           ✔  Then the repository reports no pending change under the record directory
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::A Fossil project ignores the record through its own ignore file"
             Finished `test` profile [unoptimized + debuginfo] target(s) in 4.02s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: A Fossil project ignores the record through its own ignore file
           ✔  Given a project root with givn/ initialised in a Fossil checkout
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the project records a full-content entry in the full mode
           ✔  Then the repository's ignore glob names the development record
           ✔  And the Fossil checkout reports no extra file under the record directory
           ✔  When a lifecycle command runs
           ✔  Then the repository's ignore glob still names the development record
           ✔  When the user runs `givn timeline --change demo`
           ✔  Then the repository's ignore glob covers the generated reports
           ✔  And the Fossil checkout reports no extra file under the record directory
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        EXIT=0

        Raw backend evidence (throwaway fixture root, the compiled binary, the
        scenario's own setup: `givn init --agents opencode`, the record block
        stripped from `.gitignore` before the write):

        --- .gitignore after the full-content capture ---
        # givn:begin:history
        givn/history/
        # givn:end:history

        --- git status --porcelain -uall after the second full-content capture ---
        ?? .githooks/commit-msg
        ?? .gitignore
        ?? .opencode/commands/givn-archive.md
        ?? .opencode/commands/givn-characterize.md
        ?? .opencode/commands/givn-design-review.md
        ?? .opencode/commands/givn-design.md
        ?? .opencode/commands/givn-explore.md
        ?? .opencode/commands/givn-ideation.md
        ?? .opencode/commands/givn-implement.md
        ?? .opencode/commands/givn-propose.md
        ?? .opencode/commands/givn-reverse-engineer.md
        ?? .opencode/commands/givn-review.md
        ?? .opencode/commands/givn-spec.md
        ?? .opencode/commands/givn-steps.md
        ?? .opencode/commands/givn-tasks.md
        ?? .opencode/commands/grill-me.md
        ?? .opencode/commands/grill-with-docs.md
        ?? .opencode/plugins/givn-attribution.js
        ?? .opencode/plugins/givn-timeline.js
        ?? .opencode/skills/givn-archive/SKILL.md
        ?? .opencode/skills/givn-characterize/SKILL.md
        ?? .opencode/skills/givn-design-review/SKILL.md
        ?? .opencode/skills/givn-design/SKILL.md
        ?? .opencode/skills/givn-dev-principles/SKILL.md
        ?? .opencode/skills/givn-domain-modeling/SKILL.md
        ?? .opencode/skills/givn-explore/SKILL.md
        ?? .opencode/skills/givn-ideation/SKILL.md
        ?? .opencode/skills/givn-implement/SKILL.md
        ?? .opencode/skills/givn-propose/SKILL.md
        ?? .opencode/skills/givn-reverse-engineer/SKILL.md
        ?? .opencode/skills/givn-review/SKILL.md
        ?? .opencode/skills/givn-spec/SKILL.md
        ?? .opencode/skills/givn-steps/SKILL.md
        ?? .opencode/skills/givn-tasks/SKILL.md
        ?? AGENTS.md
        ?? README.md
        ?? docs/arc42/01-introduction-and-goals.md
        ?? docs/arc42/02-architecture-constraints.md
        ?? docs/arc42/03-context-and-scope.md
        ?? docs/arc42/04-solution-strategy.md
        ?? docs/arc42/05-building-block-view.md
        ?? docs/arc42/06-runtime-view.md
        ?? docs/arc42/07-deployment-view.md
        ?? docs/arc42/08-crosscutting-concepts.md
        ?? docs/arc42/09-architecture-decisions.md
        ?? docs/arc42/10-quality-requirements.md
        ?? docs/arc42/11-risks-and-technical-debt.md
        ?? docs/arc42/12-glossary.md
        ?? docs/arc42/README.md
        ?? docs/arc42/adr/adr-template.md
        ?? givn/changes/demo/proposal.md
        ?? givn/commands.yaml
        ?? givn/config.yaml

        --- git status --porcelain -uall after givn timeline ---
        ?? .githooks/commit-msg
        ?? .gitignore
        ?? .opencode/commands/givn-archive.md
        ?? .opencode/commands/givn-characterize.md
        ?? .opencode/commands/givn-design-review.md
        ?? .opencode/commands/givn-design.md
        ?? .opencode/commands/givn-explore.md
        ?? .opencode/commands/givn-ideation.md
        ?? .opencode/commands/givn-implement.md
        ?? .opencode/commands/givn-propose.md
        ?? .opencode/commands/givn-reverse-engineer.md
        ?? .opencode/commands/givn-review.md
        ?? .opencode/commands/givn-spec.md
        ?? .opencode/commands/givn-steps.md
        ?? .opencode/commands/givn-tasks.md
        ?? .opencode/commands/grill-me.md
        ?? .opencode/commands/grill-with-docs.md
        ?? .opencode/plugins/givn-attribution.js
        ?? .opencode/plugins/givn-timeline.js
        ?? .opencode/skills/givn-archive/SKILL.md
        ?? .opencode/skills/givn-characterize/SKILL.md
        ?? .opencode/skills/givn-design-review/SKILL.md
        ?? .opencode/skills/givn-design/SKILL.md
        ?? .opencode/skills/givn-dev-principles/SKILL.md
        ?? .opencode/skills/givn-domain-modeling/SKILL.md
        ?? .opencode/skills/givn-explore/SKILL.md
        ?? .opencode/skills/givn-ideation/SKILL.md
        ?? .opencode/skills/givn-implement/SKILL.md
        ?? .opencode/skills/givn-propose/SKILL.md
        ?? .opencode/skills/givn-reverse-engineer/SKILL.md
        ?? .opencode/skills/givn-review/SKILL.md
        ?? .opencode/skills/givn-spec/SKILL.md
        ?? .opencode/skills/givn-steps/SKILL.md
        ?? .opencode/skills/givn-tasks/SKILL.md
        ?? AGENTS.md
        ?? README.md
        ?? docs/arc42/01-introduction-and-goals.md
        ?? docs/arc42/02-architecture-constraints.md
        ?? docs/arc42/03-context-and-scope.md
        ?? docs/arc42/04-solution-strategy.md
        ?? docs/arc42/05-building-block-view.md
        ?? docs/arc42/06-runtime-view.md
        ?? docs/arc42/07-deployment-view.md
        ?? docs/arc42/08-crosscutting-concepts.md
        ?? docs/arc42/09-architecture-decisions.md
        ?? docs/arc42/10-quality-requirements.md
        ?? docs/arc42/11-risks-and-technical-debt.md
        ?? docs/arc42/12-glossary.md
        ?? docs/arc42/README.md
        ?? docs/arc42/adr/adr-template.md
        ?? givn/changes/demo/proposal.md
        ?? givn/commands.yaml
        ?? givn/config.yaml

        --- git check-ignore -v ---
        .gitignore:2:givn/history/	givn/history/events.jsonl
        .gitignore:2:givn/history/	givn/history/reports/demo.html

        --- fossil settings --value ignore-glob (tail) ---
        .githooks/commit-msg
        givn/history/*

        --- fossil extras after the full-content capture ---
        givn/changes/demo/proposal.md
        (nothing under givn/history/)

        --- fossil extras after givn timeline ---
        givn/changes/demo/proposal.md
        (nothing under givn/history/)
        ```
  - [x] **COMMIT** — `af5ced1f2ffafc4b08b8a476defd405384344104` — `test(development-record): <release note>` with
        a body that names only the extended assertions and points at the Tasks
        10/11 commits; it MUST NOT restate the two scenario titles, so the
        per-scenario gate sees no duplicate scenario commit. Record the revision
        identifier.

## Task 11c — `Default capture keeps the metadata a review needs` (`development-record`, added)

- [x] **Scenario: `Default capture keeps the metadata a review needs`**
  - Design constraints:
    - The scenario is **reporter-driven, not seeded**: it runs the generated
      omp reporter through the shared fake `pi` extension context and the real
      `givn history ingest`, then reads the record. omp exposes every asserted
      field (per-harness availability table, design §Capture completeness), so
      no fixture stands in for the reporter.
    - Default metadata table (design §Capture completeness): a `turn` entry
      carries `model`, `cost` (real and list where present), `context_size` /
      `context_window`, `stop_reason`, `index`, `usage` (with reasoning and cache
      fields); a failed `tool` entry carries `error` bounded to 1 000 characters
      (first line kept), plus `tool_call_id`, `output_bytes`, `result_meta`, and
      `duration_source`.
    - Per-harness availability is authoritative: never assert a field the
      harness cannot expose (Claude Code has no turn index or stop reason,
      opencode no turn count or context size, headless Maki may omit model and
      title). A seeded record is used only to exercise a field a harness cannot
      expose, and then the assertion is about the record's handling of absence,
      never a substitute for the reporter path.
    - The intake's `Usage` struct widens to the inventory's fields (reasoning,
      cache-read/cache-write, per-model), and the entry envelope gains `session`.
      Schema stays 1 (additive `data`); no rewrite of existing lines.
    - Reporter field sources per harness are the inventory rows (design §Capture
      completeness, per-harness list); the opencode id-key mismatches
      (`data.id` vs `data.sessionID`, `data.form.id`,
      `permission.replied.requestID`) are fixed as part of this work.
    - Files: `src/history.rs` (bounds `text_metadata`/`error_text`, the widened
      data), `src/cli/history.rs` (intake fields + validation),
      `assets/agent-integrations/omp/givn-timeline.js` (the driven reporter),
      the other three reporter assets for the same fields where they expose
      them, `tests/steps/development_record_steps.rs`,
      `tests/steps/development_timeline_support.rs`.
    - RED expected failure: the new Given/When/Then phrases are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Default capture keeps the metadata a review needs"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Default capture keeps the metadata a review needs"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Default capture keeps the metadata a review needs
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✘  And the project records in the default mode
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:117:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (3825198) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — widen the intake and the reporters; compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.09s
        BUILD_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Default capture keeps the metadata a review needs"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Default capture keeps the metadata a review needs
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  And the project records in the default mode
           ✔  When the omp reporter reports a completed turn and a failed tool call for "demo"
           ✔  Then the turn entry names its model, its cost, its context size, and its stop reason
           ✔  And the failed tool entry keeps its error text within the documented bound, its tool call id, and its output size
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0
        ```
        Production files: `src/history.rs` (the `session` envelope,
        `text_metadata` 240 / `error_text` 1000 bounds, `bound_result_meta`),
        `src/cli/history.rs` (widened `Usage` with reasoning, turn cost/context/
        stop-reason/index/usage data, tool `tool_call_id`/`error`/`output_bytes`/
        `result_meta`/`duration_source`, the payload `session` carried onto every
        entry, Claude's error/response/question/transcript-cost reads),
        `assets/agent-integrations/omp/givn-timeline.js` (the driven reporter),
        and the opencode/Maki assets plus the generated copies for the same
        fields where their harnesses expose them.
  - [x] **REFACTOR** — one bounds module and one metadata builder shared by the
        four reporters; re-run → still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check
        FMT_EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.97s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Default capture keeps the metadata a review needs"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Default capture keeps the metadata a review needs
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  And the project records in the default mode
           ✔  When the omp reporter reports a completed turn and a failed tool call for "demo"
           ✔  Then the turn entry names its model, its cost, its context size, and its stop reason
           ✔  And the failed tool entry keeps its error text within the documented bound, its tool call id, and its output size
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0

        The bounds live in one module (`src/history.rs`): `bound_label`,
        `bound_text_metadata`, `bound_error`, `bound_question`, `bound_content`,
        and `bound_result_meta`, each applied by the intake at its one site; the
        four reporters build the same `{text, truncated, bytes}` metadata shape
        through their own small helpers (`usageOf`/`costOf`/`contextOf`/
        `reportedQuestion` in omp and opencode, `usage_of`/`transcript_question`
        in Maki, `session_record_usage`/`reported_question` in the Claude mapper).
        ```
  - [x] **COMMIT** — `509263872c637196d31e35753257809917acb64b` — `feat(development-record): <release note>` with
        the scenario title in the body. Record the revision identifier.

## Task 11d — `Question text is captured in full` (`development-record`, added)

- [x] **Scenario: `Question text is captured in full`**
  - Design constraints:
    - Question text is the one content value captured **always**, never behind
      the switch and never reduced to the label: every question and its option
      labels are joined and stored in `data.question`
      (`{text, truncated, bytes}`), capped at **4 000 characters per question
      entry** with `truncated: true` and the original `bytes` when the cap is
      hit (design bounds table and wire schema).
    - The entry's `label` stays bounded to 120 characters, so the label rule and
      the full text coexist.
    - Sources: omp `ask.questions[]`; Claude Code `AskUserQuestion.tool_input.
      questions[]`; opencode `form.fields[].description/title` and the question
      tool's `input.questions`; **Maki reads `maki.session.messages` at the
      wait's close** (live transcript, no new grants) and matches the blocking
      `question` tool_use's `input.questions[]`. The alternative
      `tool.question.input` slot is rejected because an undeclared-tool slot
      needs **every** permission (including `net`, `env`, and `agent`). Residual,
      tested and documented: when no matching question is in the transcript (a
      permission prompt, plan form, or Lua float also produces `needs_input`),
      the entry keeps the bounded `data.title` label and no full text; the wait
      duration is still recorded.
    - Files: `src/history.rs` (the question bound), `src/cli/history.rs`
      (question event validation), the four reporter assets (the Maki asset
      gains the transcript read), `tests/steps/development_record_steps.rs`,
      `tests/steps/development_timeline_support.rs` (the Maki driver's
      transcript fixture).
    - RED expected failure: the new Given/When/Then phrases (including the
      no-match residual) are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Question text is captured in full"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Question text is captured in full"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Question text is captured in full
           ✘  Given a change "demo" whose Maki reporter reads a question from its session transcript
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:124:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3839711) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — capture and cap the text; compile first, re-run → exit 0.
        Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.09s
        BUILD_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Question text is captured in full"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Question text is captured in full
           ✔  Given a change "demo" whose Maki reporter reads a question from its session transcript
           ✔  When the reported question is longer than the entry label bound
           ✔  Then the record holds the complete question text up to the 4 000-character cap
           ✔  And the entry marks the text truncated with its original size
           ✔  And the entry's label stays within the label bound
           ✔  When the Maki session transcript holds no matching question
           ✔  Then the record keeps the bounded title label and no full question text
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        EXIT=0
        ```
        Production sites (the question value is part of the same intake contract
        landed with the bundle's 11b/11c revisions): `src/history.rs`
        (`bound_question`, the 4 000-character cap), `src/cli/history.rs`
        (`data.question` on every question event, the `title` metadata bound),
        and the four reporter assets — omp's `ask` questions/options,
        opencode's form fields and question tool, Claude's
        `AskUserQuestion.tool_input`, and Maki's `maki.session.messages` read at
        the wait's close landed with 11c's `50926387`. This revision adds the
        real Maki transcript driver fixture and the scenario's assertions.
  - [x] **REFACTOR** — reuse the bounded-text helper; re-run → still exit 0.
        Paste it.
        ```
        $ cargo fmt --all -- --check
        FMT_EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.50s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Question text is captured in full"
             Finished `test` profile [unoptimized + debuginfo] target(s) in 4.31s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Question text is captured in full
           ✔  Given a change "demo" whose Maki reporter reads a question from its session transcript
           ✔  When the reported question is longer than the entry label bound
           ✔  Then the record holds the complete question text up to the 4 000-character cap
           ✔  And the entry marks the text truncated with its original size
           ✔  And the entry's label stays within the label bound
           ✔  When the Maki session transcript holds no matching question
           ✔  Then the record keeps the bounded title label and no full question text
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        EXIT=0

        The reporter-side join reuses the one value shape (`{text, truncated,
        bytes}`) and the intake's `bound_question` is the only cap; the Maki
        transcript reader returns `nil` for a wait with no matching call, so the
        residual keeps the bounded title and the intake stores no
        `data.question`.
        ```
  - [x] **COMMIT** — `f54633310bba10abae995e9dc68267cfaebe0c8c` — `feat(development-record): <release note>` with
        the scenario title in the body. Record the revision identifier.

## Task 11e — `The record can be turned off for a project` (`development-record`, added)

- [x] **Scenario: `The record can be turned off for a project`**
  - Design constraints:
    - The control is the three-state project key `record.mode: on | off | full`
      in `givn/config.yaml` (design §Capture completeness). The parse site is
      **`src/config/overlay.rs::OverlayDoc`** (the project-config path), not
      `src/config/manifest.rs`: `OverlayDoc` gains a lenient raw `record` value
      so an invalid value warns once and falls back to `on`, never failing the
      config parse. `givn upgrade` preserves the uncommented key.
    - **`off` is absolute:** no new entry at all — no store append, no
      pending-state write, and no record-side ignore write. The two enforcement
      sites are `src/cli/history.rs::ingest` and
      `src/cli/history.rs::harness_hook` (the in-process Claude path); both warn
      once and exit 0 without touching the store or pending state, even for a
      payload carrying content. `--with-full-content` is ignored under `off`
      with a one-line warning and exit 0. Reporters skip their work entirely,
      and `givn history` / `givn history settings` / `givn timeline` still read
      whatever the store already holds.
    - Lowering the mode is forward-looking: already-captured entries are
      retained, never purged; the scenario asserts the earlier entries remain
      readable after `off`.
    - An absent value resolves to **`on`** (the default); the resolved mode is
      what `givn history settings --json` reports. Every capture point also
      checks the mode before it touches the store
      (`src/history.rs::should_record`).
    - Files: `src/history.rs`, `src/config/overlay.rs` (the lenient `record`
      field), `src/cli/history.rs` (intake + harness_hook off enforcement),
      `src/main.rs` (the settings read-view exemption alongside `history` /
      `timeline`), the four reporter assets (early return when off),
      `tests/steps/development_record_steps.rs`.
    - RED expected failure: the new Then phrases (no pending state, no
      record-side ignore write, the intake warning) are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record can be turned off for a project"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record can be turned off for a project"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record can be turned off for a project
           ✘  Given a change "demo" whose record already holds entries
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:134:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3812785) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the lenient key, the resolver, the intake and
        harness_hook enforcement, and the reporters' early return; compile
        first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.81s
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record can be turned off for a project"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record can be turned off for a project
           ✔  Given a change "demo" whose record already holds entries
           ✔  When the project sets the record mode to "off"
           ✔  And a lifecycle command runs for "demo"
           ✔  Then the record gains no new entry
           ✔  And the record directory gains no pending state
           ✔  And no record-side ignore entry is written
           ✔  And the developer still reads the earlier entries for "demo"
           ✔  When the agent reports a tool call with its complete input through the intake
           ✔  Then the intake exits 0 and warns once that the record is off
           ✔  And the record gains no new entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        EXIT=0

        Settings and off-enforcement smoke (real binary, throwaway roots):
        $ givn history settings --json          # no record key
        {"mode":"on","source":"default"}                     exit 0
        $ givn history settings                   # no record key
        record mode: on (source: default)                    exit 0
        $ givn history settings --json          # record.mode: off
        {"mode":"off","source":"config"}                     exit 0
        $ givn history settings --json          # record.mode: full
        {"mode":"full","source":"config"}                    exit 0
        $ givn history settings --json          # outside givn/ (no scaffold)
        error: not a Givn project (givn/ not found)          exit 1
        $ givn history ingest --with-full-content   # off, payload carrying content
        warning: the development record is off for this project; nothing was recorded
                                                             exit 0
        store byte-identical: yes; givn/history/pending: absent
        $ givn history harness-hook --harness claude   # off, PreToolUse payload
        warning: the development record is off for this project; nothing was recorded
                                                             exit 0
        store byte-identical: yes; givn/history/pending: absent

        Reporter smoke (real harness binary, generated reporter; record.mode on
        then off): on recorded the tool entry and stored no data.content; off
        left the store byte-identical and created no pending directory.
        ```
  - [x] **REFACTOR** — one mode resolver shared by capture points and the
        settings query; re-run → still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check
        EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.18s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The record can be turned off for a project"
             Finished `test` profile [unoptimized + debuginfo] target(s) in 4.27s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The record can be turned off for a project
           ✔  Given a change "demo" whose record already holds entries
           ✔  When the project sets the record mode to "off"
           ✔  And a lifecycle command runs for "demo"
           ✔  Then the record gains no new entry
           ✔  And the record directory gains no pending state
           ✔  And no record-side ignore entry is written
           ✔  And the developer still reads the earlier entries for "demo"
           ✔  When the agent reports a tool call with its complete input through the intake
           ✔  Then the intake exits 0 and warns once that the record is off
           ✔  And the record gains no new entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `fb54192bd357c55f859d22d73625165211ab28ba` — `feat(development-record): <release note>` with
        the scenario title in the body. Record the revision identifier.

## Task 11f — `The default mode records metadata without content` (`development-record`, added)

- [x] **Scenario: `The default mode records metadata without content`**
  - Design constraints:
    - The default mode is **`on`**: all available metadata, bounded text per the
      bounds table, and question text in full — but **no** LLM-facing content
      (the scenario also asserts that no entry carries a `data.content` object).
    - The **intake** is the single enforcement site for every bound and for the
      mode on payloads: an event carrying a `content` object is appended metadata-only
      when the effective mode is not `full`, and the content is dropped while
      the event is kept; the label 120, text-metadata 240, error 1 000,
      question 4 000, per-field 64 KiB, and per-entry 256 KiB bounds are
      re-applied to every event. A stale reporter cannot bloat or leak the
      store.
    - Reporters learn the mode once per session from
      `givn history settings --json` (`{"mode": …, "source": "config|default"}`)
      so content is not sent when it would be dropped; discovery per harness and
      the failure discipline (treat as `on`, warn once, never block the harness)
      are in design §Capture completeness. `source: flag` does not exist.
    - The settings command's surface is fixed: human output
      `record mode: <mode> (source: <config|default>)`; `--json` the payload
      above; both exit 0; an unknown/unreadable value warns once and reports
      `on`/`default`; outside a Givn project it exits 1 with
      `error: not a Givn project (givn/ not found)`. It is a non-recorded read
      view: `src/main.rs::records` excludes `history settings` alongside
      `history` and `timeline`, so a query appends no `command` entry.
    - Files: `src/history.rs`, `src/config/overlay.rs` (the lenient `record`
      field), `src/cli/history.rs` (`settings` + intake enforcement),
      `src/main.rs` (the read-view exemption), the four reporter assets,
      `tests/steps/development_record_steps.rs`.
    - RED expected failure: the new Then/Givens phrases are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The default mode records metadata without content"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The default mode records metadata without content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The default mode records metadata without content
           ✘  Given a change "demo" whose project has no record mode configured
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:147:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3843973) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the settings query (human and `--json`), the
        read-view exemption, and the intake drop/bounds; compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.94s
        BUILD_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The default mode records metadata without content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The default mode records metadata without content
           ✔  Given a change "demo" whose project has no record mode configured
           ✔  When the agent reports a tool call with its complete input and output
           ✔  Then the record drops the complete content
           ✔  And no entry carries a data.content object
           ✔  And the record keeps the tool call's metadata
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
        Production files: the intake's drop (already the one site, `src/cli/history.rs`
        `attach_content` gated on `RecordMode::Full`) and the reporters'
        `full`-only content builders in `assets/agent-integrations/omp`,
        `opencode`, and `maki/givn-timeline.lua` plus the generated copies; the
        Claude in-process mapper gates its content on the resolved mode
        (`src/cli/history.rs::map_harness_event`). No reporter builds or sends
        content in `on`.
  - [x] **REFACTOR** — one mode resolver shared by intake and settings; re-run
        → still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check
        FMT_EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.15s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The default mode records metadata without content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The default mode records metadata without content
           ✔  Given a change "demo" whose project has no record mode configured
           ✔  When the agent reports a tool call with its complete input and output
           ✔  Then the record drops the complete content
           ✔  And no entry carries a data.content object
           ✔  And the record keeps the tool call's metadata
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        The intake and the settings query read the same resolver
        (`history::record_setting`); each reporter caches its one
        `givn history settings --json` result and uses it both for the `off`
        skip and the `full` content gate.
        ```
  - [x] **COMMIT** — `9210247f05fa29b5960937e6686e784f77c04e68` — `feat(development-record): <release note>` with
        the scenario title in the body. Record the revision identifier.

## Task 11g — `The full mode records complete content` (`development-record`, added)

- [x] **Scenario: `The full mode records complete content`**
  - Design constraints:
    - With `record.mode: full` (or `givn history ingest --with-full-content` for
      one payload, except under `off`), the entry keeps the complete
      LLM-facing content inline in `data.content`'s named fields:
      `tool_input`, `tool_output`, `assistant_text`, `thinking_text`,
      `user_prompt`, `compaction_summary`, `plan_content`, `draft_text`, and
      `subagent_transcript`, each `{text, truncated, bytes}` — per-field cap
      **64 KiB**, then per-entry aggregate cap **256 KiB**, applied in that
      order. A field cut at either bound keeps `truncated: true` and its original
      `bytes`.
    - **The intake is the single enforcement site** for both caps (and every
      other bound); a reporter's own truncation is an optimization. The scenario
      reports an input larger than the per-field cap and asserts the stored
      text, `truncated`, the original `bytes`, and the per-entry total.
    - Content stays inside `givn/history/events.jsonl` (no second store), so the
      placement/ignore rule holds by construction.
    - The details panel implements the exact fact list and one bounded,
      scrollable, escaped content region (design §Report rendering); the region
      names the cap and marks truncation and is not part of the accessible
      summary. `src/cli/timeline.rs` also implements the 8 MiB report content
      budget and the `content omitted; see the record` marker; Task 25b carries
      the regular scenario and the E7 assertion extension that prove them.
    - Files: `src/history.rs` (content caps), `src/cli/history.rs` (intake
      acceptance and the single bound enforcement), `src/cli/timeline.rs` +
      `assets/timeline/report.html` (panel facts, content region, budget and
      omission marker), the four reporters,
      `tests/steps/development_record_steps.rs`,
      `tests/steps/development_report_steps.rs`.
    - RED expected failure: the new Then phrases (field cap, truncated, original
      size, per-entry total) are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The full mode records complete content"`
        → must exit non-zero. Paste it.
        ```
        RED category: the scenario's contract (the content caps) already held
        from the bundle's 11b core, so the discriminating RED is the pre-step
        state: the scenario's own bindings are temporarily absent (the steps
        file was restored to HEAD for the run and restored back immediately —
        a controlled omission in the isolated fixture, never a production
        switch).

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The full mode records complete content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The full mode records complete content
           ✘  Given a change "demo" whose project sets the record mode to "full"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:155:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3851928) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101
        ```
  - [x] **GREEN** — accept, cap, store, and render the content; compile first,
        re-run → exit 0. Paste it, together with a `git status --porcelain`
        showing nothing under `givn/history/` (Task 11b's rule).
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.94s
        BUILD_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The full mode records complete content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The full mode records complete content
           ✔  Given a change "demo" whose project sets the record mode to "full"
           ✔  When the agent reports a tool call whose complete input exceeds the per-field cap
           ✔  Then the record keeps the input up to the 64 KiB per-field cap
           ✔  And the content field marks truncation and records its original size
           ✔  And the entry's content never exceeds the 256 KiB per-entry cap
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        $ git status --porcelain
         M assets/timeline/report.html
         M givn/changes/development-timeline/specs/inspect-development-history/development-record.feature
         M src/cli/timeline.rs
         M tests/steps/development_record_steps.rs
        $ git status --porcelain | grep 'givn/history/' || echo "(none)"
        (none)

        Report budget smoke (throwaway root, real binary): 33 full-mode tool
        entries × 256 KiB = 8.65 MiB of content; `givn timeline --change demo`
        embedded exactly 8 MiB (32 entries) and left the 33rd out with
        `omitted_content: [35]`, its `data.content` removed from the document,
        and the exact note `content omitted; see the record` present in the
        rendered panel code.
        ```
  - [x] **REFACTOR** — one content-capping helper and one renderer; re-run →
        still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check
        FMT_EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.37s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::The full mode records complete content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: The full mode records complete content
           ✔  Given a change "demo" whose project sets the record mode to "full"
           ✔  When the agent reports a tool call whose complete input exceeds the per-field cap
           ✔  Then the record keeps the input up to the 64 KiB per-field cap
           ✔  And the content field marks truncation and records its original size
           ✔  And the entry's content never exceeds the 256 KiB per-entry cap
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        One capping path (`history::bound_content`) and one report writer
        (`build_report_model`'s budget + `render_report`/the panel's
        `fillContent`), shared by every kind and every report; the budget was
        re-verified on the 8.65 MiB fixture (8 MiB embedded, one omission).
        ```
  - [x] **COMMIT** — `dff0a8c9238c4405229881b53d82bb3d9dd36110` — `feat(development-record): <release note>` with
        the scenario title in the body. Record the revision identifier.

## Task 11h — `An invalid record mode falls back to the default` (`development-record`, added)

- [x] **Scenario: `An invalid record mode falls back to the default`**
  - Design constraints:
    - The value is parsed through **`src/config/overlay.rs::OverlayDoc`** (the
      project-config path) as a lenient raw `record` value. A non-string or
      unknown `record.mode` resolves to **`on`** and prints one warning per
      process naming the key and the fallback; it must **not** fail the whole
      config parse (a broken value must not stop the work being recorded).
    - The fallback is visible in `givn history settings --json` as
      `{"mode": "on", "source": "default"}` so a reporter and a human see the same
      resolution; the human form prints
      `record mode: on (source: default)` and both exit 0.
    - Files: `src/config/overlay.rs` (the lenient `record` field),
      `src/history.rs` (mode parsing + the one warning),
      `src/cli/history.rs` (settings source/payload),
      `src/main.rs` (read-view exemption), `tests/steps/development_record_steps.rs`.
    - RED expected failure: the new Then phrase (`givn history settings --json`
      reporting mode `on` and source `default`) is undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::An invalid record mode falls back to the default"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::An invalid record mode falls back to the default"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: An invalid record mode falls back to the default
           ✘  Given a change "demo" whose project sets an unknown record mode
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:163:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3819034) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the parse, the warning, and the settings source;
        compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
        BUILD_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::An invalid record mode falls back to the default"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: An invalid record mode falls back to the default
           ✔  Given a change "demo" whose project sets an unknown record mode
           ✔  When a lifecycle command runs for "demo"
           ✔  Then the command warns once that the record falls back to the default mode
           ✔  And the record holds the command's bounded metadata
           ✔  And `givn history settings --json` reports mode "on" with source "default"
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
        Production sites (shared with this bundle's mode core, landed in
        `fb54192b` because Task 11e's own contract names them):
        `src/config/overlay.rs` (the lenient raw `record` field and
        `load_record_block`), `src/history.rs` (`RecordMode`,
        `record_setting`, the once-per-process `warn_invalid_record_mode`,
        `should_record`) and `src/cli/history.rs` (`settings` payload/source),
        `src/main.rs` (`records` read-view exemption). This revision carries the
        scenario's real assertions that bind the parse, the single warning, the
        bounded command metadata, and the settings resolution; the Task 11e
        smoke block above shows the same invalid value reporting
        `{"mode":"on","source":"default"}` with exactly one warning.
  - [x] **REFACTOR** — one parser and one warning site; re-run → still exit 0.
        Paste it.
        ```
        $ cargo fmt --all -- --check
        FMT_EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.98s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::An invalid record mode falls back to the default"
             Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: An invalid record mode falls back to the default
           ✔  Given a change "demo" whose project sets an unknown record mode
           ✔  When a lifecycle command runs for "demo"
           ✔  Then the command warns once that the record falls back to the default mode
           ✔  And the record holds the command's bounded metadata
           ✔  And `givn history settings --json` reports mode "on" with source "default"
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `b06e17716a70572b4016775018fbbb06b56da271` — `feat(development-record): <release note>` with
        the scenario title in the body. Record the revision identifier.

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
          only (the complete text is stored separately in `data.question` by
          Task 11d; this task's label path keeps the first line), then the same
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

## Task 24a — `Maki's reporter keeps the harness's own tool duration and marks what its harness omits` (`agent-activity-reporting`, added)

- [x] **Scenario: `Maki's reporter keeps the harness's own tool duration and marks what its harness omits`**
  - Design constraints:
    - Wiring: add `AgentTarget::Maki` (canonical name `maki`, dir `.maki`) as a
      **full target** — it writes skills to `.maki/skills/<name>/SKILL.md` and
      commands to `.maki/commands/<name>.md`, so a `maki`-only selection is
      equipped (Maki reads project commands only from `.maki/commands/` and
      `.claude/commands/`, and Givn writes commands only for `opencode` and
      `omp`). Register it in `src/config/overlay.rs::KNOWN_AGENT_TARGETS`, the
      `--agents` help text, the `givn init` installation summary, and the
      documented target lists (`README.md`), and extend `sync_agent_integrations`
      + `managed_generated_paths` + `src/skills/rules.rs::ignore_entries`
      (design §Architecture Impact).
    - Path ownership (design §Maki, mirroring the `.claude/settings.local.json`
      precedent): `.maki/lua/givn-timeline.lua`, `.maki/skills/**`, and
      `.maki/commands/**` are managed generated paths (allowlist staging,
      reconcile removal, `local-only` ignore entries); `.maki/init.lua` and
      `.maki/plugin.toml` are **merged user-owned** files and never appear in
      those lists, so no user-owned file silently becomes a strict rule.
    - Assets and install path: `assets/agent-integrations/maki/givn-timeline.lua`
      → `.maki/lua/givn-timeline.lua`; the reporter's grants **merged** into
      `.maki/plugin.toml` (create with all six keys explicit — `fs_read`,
      `fs_write`, `run` true and `net`, `env`, `agent` false — otherwise add only
      the reporter's three keys idempotently, never removing or flipping a user
      key, never touching surrounding content — the `merge_claude_settings`
      precedent); and the managed fence in `.maki/init.lua`, created when absent:
      `-- givn:begin:timeline` / `local ok, reporter = pcall(require,
      "givn-timeline")` / `if ok and type(reporter) == "table" and reporter.setup
      then reporter.setup() end` / `-- givn:end:timeline`. A bare
      `pcall(require, "givn-timeline").setup()` is **wrong** (it indexes the
      boolean status, raises, exits 1, and kills the rest of the user's
      `init.lua`); the hand-placed start-package alternative is rejected because it
      lives outside the project.
    - Mapping (design §Maki, verified against the Lua API reference and the
      installed 0.6.3, and aligned with the committed implementation):
      `ToolStart`/`ToolDone` with the host's `data.duration_ms` when the call ran
      and the reporter's own `os.clock()` stamps otherwise — a call that never
      ran is detected by the absence of **both** `data.duration_ms` and
      `data.input` and is recorded **without** a duration, never as zero; the
      entry's label is the bounded subject of the call's input (the first of its
      `command`, `file_path`, `path`, `pattern`, or `query`, cut to one line and
      the record's 120-character bound) and the tool name when the call never ran
      or carries no usable subject;
      `TurnStart`/`TurnEnd`/`TurnError` for turns and the flush, with `data.text`
      stored only in `full` as `data.content.user_prompt`; `AutoCompacting`/
      `CompactionDone` as `phase` entries (`compacting`, `compacted`) with
      `data.summary` stored only in `full` as `data.content.compaction_summary`;
      `SessionStatusChanged`
      (`needs_input` → wait open, `working` → close) for question waits, labelled
      with the harness's own bounded `data.title` or `needs input` when absent;
      `TaskStatusChanged` (`working` →
      subagent span open, `done`/`error` → close) with
      `data.usage = "unavailable"` on the entry; `ModelChanged` for the usage
      entries' model; `SessionEnd`/`SessionReset` for the final flush, which runs
      with `scope = "plugin"` because the exported `run()` can execute at load
      time where no task is active.
    - No timestamp exists in any Maki event, and `TurnEnd` fires once per turn and
      only for the main session — so the subagent span carries the
      `data.usage = "unavailable"` marker and the report's token column reads
      unavailable; that asymmetry is what this scenario asserts.
    - Reporter module discipline: keep every `maki.*` asynchronous call out of the
      module body (top-level async breaks `pcall(require)`), and flush with
      `maki.fn.jobstart({ "sh", "-c", "cat <payload-file> | <givn> history ingest" })`
      awaited by `maki.fn.jobwait(id, <timeout>)` inside the callback that owns it
      — there is no `maki.fn.system`, `jobstart` has no stdin option, and an
      unawaited task-scope job is lost.
    - Driver: the real binary with `maki --trust prompt` (verified: exit 0, loads
      the project plugin, no model call, no network) asserting the plugin's load
      marker; then a **test-only** driver whose `.maki/init.lua` requires the
      generated reporter and calls its exported `setup()`/`run()` at top level and
      which fires synthetic events with
      `maki.api.exec_autocmds("<Event>", { data = { … } })`, so the reporter's real
      callbacks call the real `givn history ingest`. The driver entry goes in
      `tests/steps/agent_activity_reporting_e2e_steps.rs` (its driver table must
      gain the `maki` row) with helpers in
      `tests/steps/development_timeline_support.rs`. Do not use
      `maki --trust -p "…"` (a real turn: model and network) and do not pass `-p`
      to the `prompt` subcommand (the CLI rejects it with exit 2).
    - Provisioning: the pinned `maki` v0.6.3 binary
      (`maki-v0.6.3-x86_64-unknown-linux-musl.tar.gz` from the
      `https://github.com/tontinton/maki/releases/download/v0.6.3/` release,
      checksum-verified, extracted into a local bin directory) joins the
      setup-script family and the CI/release prerequisites, alongside Node and
      Chromium. The row is strictly required: a missing binary fails the `@e2e`
      scope, it never skips.
    - RED expected failure: all **eight** new step phrases are undefined — one
      Given, three `When` (the three harness actions), and four `Then`/`And`
      assertions.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Maki's reporter keeps the harness's own tool duration and marks what its harness omits"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Maki's reporter keeps the harness's own tool duration and marks what its harness omits"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 11.40s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Maki's reporter keeps the harness's own tool duration and marks what its harness omits
           ✘  Given a project with the generated Maki development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:108:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (2872702) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — all eight new step phrases (one Given,
        three When, four Then/And) match no function, exactly as the task
        predicts. Sequencing note, recorded rather than glossed: the reporter
        asset, the target wiring, and the driver helpers were authored in the
        same working session before this run, and the step bindings were held
        back for it, so the failing state this evidence shows is the
        pre-binding state (no step can run), not a tree without the
        production files.
        ```
  - [x] **GREEN** — implement the full target (skills, commands, and the
        reporter), the two assets, the merged `.maki/plugin.toml` grants, the
        managed fail-soft `init.lua` fence, the autocmd mapping, and the driver
        steps. Files: `src/skills/targets.rs`, `src/skills/rules.rs` (including
        `is_rule_path` with its `tracked_rule_paths` sibling — the strict-rule
        classification for the managed `.maki` paths, never for the two merged
        files), `src/cli/addons.rs` (the hard-coded addon-removal target lists
        around lines 407 and 421, so a deselected addon's `.maki/skills/<addon>`
        directory and `.maki/commands/<addon>.md` files are removed too),
        `src/config/overlay.rs`, `src/cli/init.rs`, `src/main.rs` (the `--agents`
        help), `README.md` (the target list), `assets/agent-integrations/maki/`,
        `tests/steps/agent_activity_reporting_steps.rs`,
        `tests/steps/agent_activity_reporting_e2e_steps.rs` (the driver table's
        `maki` row), `tests/steps/development_timeline_support.rs` (the driver
        helpers), the setup script that provisions the pinned `maki` v0.6.3
        binary, and the CI/release workflow prerequisites. Compile first, re-run →
        exit 0. Paste it, together with the `maki --trust prompt` load-proof
        output.
        ```
        $ ./setup-e2e-harness.sh
        downloading https://github.com/tontinton/maki/releases/download/v0.6.3/maki-v0.6.3-x86_64-unknown-linux-musl.tar.gz
        /home/buster/.cache/givn-e2e/maki-v0.6.3-x86_64-unknown-linux-musl.tar.gz: OK
        maki 0.6.3
        maki installed at /home/buster/.cache/givn-e2e/bin/maki (GIVN_E2E_BIN=/home/buster/.cache/givn-e2e/bin)

        Load proof, with the generated fence in a real project and no model call:
        $ givn init --agents maki && maki --trust prompt >/tmp/maki-load.txt 2>&1
        LOAD_EXIT=0            # and: grep -ci "error\|warning" -> 0
        $ head -3 /tmp/maki-load.txt
        You are Maki, an interactive CLI coding agent. Use the tools available to assist the user with software engineering tasks. ...
        (the reporter's own load marker is asserted by the driver in every scenario run)

        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.19s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Maki's reporter keeps the harness's own tool duration and marks what its harness omits"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 3.51s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Maki's reporter keeps the harness's own tool duration and marks what its harness omits
           ✔  Given a project with the generated Maki development reporter
           ✔  When the Maki harness reports a completed tool call
           ✔  Then the record holds that tool call with the duration the harness reported
           ✔  When the Maki harness reports a tool call that never ran
           ✔  Then the record holds that tool call without a measured duration
           ✔  When the Maki harness reports a subagent that started and completed
           ✔  Then the record holds that subagent's span
           ✔  And the record marks that subagent's token usage as unavailable
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        EXIT=0

        Delta scopes after the implementation:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
             Summary [   3.638s] 36 tests run: 36 passed, 8 skipped
        EXIT=0
        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e
             Summary [   6.547s] 8 tests run: 8 passed, 36 skipped
        EXIT=0
        The e2e outline ran all four Examples rows (`omp`, `opencode`, `claude`,
        `maki`) in one unit
        (`...::Consumer installs the agent reporter and its reported activity lands in the record`)
        and refreshed the change-wide visual evidence, the reporter outline's
        pair included:
        givn/changes/development-timeline/evidence/visual/consumer-installs-the-agent-reporter-and-its-reported-activity-lands-in-the-record/{mobile.png,desktop.png}

        Production files created/modified by this GREEN step:
        - `assets/agent-integrations/maki/givn-timeline.lua` (new — the reporter:
          no `maki.*` call in the module body; `setup()` registers the twelve
          autocmds and `run()` flushes; `os.clock()` stamps every start and
          `ToolDone`'s `data.duration_ms` is used when the call ran; the flush
          writes one payload to Maki's state directory and delivers it with an
          awaited `maki.fn.jobstart({ "sh", "-c", "cat <payload-file> | givn
          history ingest" }, { scope = "plugin" })` + `maki.fn.jobwait`; every
          callback is wrapped, the buffer is bounded, and a failed flush counts
          its dropped events);
        - `assets/agent-integrations/maki/plugin.toml` (new — the create-case
          grants, all six keys named: `fs_read`, `fs_write`, `run` true and
          `net`, `env`, `agent` false);
        - `src/skills/targets.rs` (modified — `AgentTarget::Maki` with skills
          and commands, the reporter row in the asset-to-target table, the
          `managed_registration_file`/`target_merges` split, and the two merged
          files: `merge_maki_plugin` (per-key idempotent, never removing or
          flipping a user key, created from the asset when absent) and
          `merge_maki_init` (the exact fail-soft fence, created when absent,
          re-applied in place, never touching the rest of the file));
        - `src/skills/rules.rs` (modified — `ignore_entries` and `is_rule_path`
          cover `.maki/lua/givn-timeline.lua`, `.maki/skills/givn-*/`, and
          `.maki/commands/{givn,grill}-*`, and deliberately exclude the two
          merged user-owned files);
        - `src/cli/addons.rs` (modified — the two addon-removal target lists
          and their messages include `.maki`, so a deselected addon's guidance is
          removed there too);
        - `src/config/overlay.rs`, `src/cli/init.rs`, `src/main.rs`, `README.md`
          (modified — `maki` is a known target, appears in the `--agents` help
          and the refusal/valid-target lists, and the init summary names the
          reporter's three installed paths);
        - `src/cli/history.rs` (modified — the intake gains the `phase` kind a
          harness's own long step reports (Maki's compaction, labelled and
          measured, `data.summary` never stored) and marks a subagent entry
          whose harness reported no tokens with the record's
          `usage: "unavailable"` marker, exactly as it already does for turns);
        - `tests/steps/development_timeline_support.rs` (modified — the real
          `maki` driver: the pinned binary is resolved from `GIVN_E2E_BIN`/
          `~/.cache/givn-e2e/bin`/`PATH`; the driver's `init.lua` starts with the
          shipped fence verbatim and then fires the scripted autocmds through
          `maki.api.exec_autocmds`, asserts the load marker, and flushes with the
          reporter's `run()`; the `HarnessActivity` vocabulary converts to the
          harness's own autocmds for the e2e row; the shared `driver_path`
          helper carries the `givn` shim and the harness bin directory);
        - `tests/steps/agent_activity_reporting_steps.rs` (modified — the
          scenario's eight phrases, the `maki` row in the reporter install
          assertion with the merged files' content, and the reporter-asset table);
        - `tests/steps/agent_activity_reporting_e2e_steps.rs` (modified — the
          driver table's `maki` row);
        - `setup-e2e-harness.sh` (new — downloads the pinned
          `maki-v0.6.3-x86_64-unknown-linux-musl.tar.gz`, verifies its
          sha256, and extracts it into the local bin directory the suite
          resolves);
        - `.github/workflows/ci.yml`, `.github/workflows/release.yml` (modified —
          the Maki install step joins the browser install before the suites).

        Recorded deviations and interpretations (none silent):
        - the flush passes `{ scope = "plugin" }` to `jobstart`, because the
          flush also runs from the exported `run()` at load time, where no task
          is active and the default task scope raises `jobstart: no active task`
          (reproduced during GREEN); the design's awaited job inside the owning
          callback is unchanged;
        - the tool label is the bounded input subject (one line, ≤ 120
          characters) where the call ran, and the tool name for a call that never
          ran: the design's prose says "the label is the tool name" while the
          committed `@e2e` outline asserts the reported command for every
          harness; the bounded label keeps D2's privacy bound, and the
          never-run case is exactly where no input exists;
        - a tool call that never ran is detected by the absence of BOTH
          `data.duration_ms` and `data.input` (the API reference documents both
          as present only for a call that actually ran), which is what makes
          "a cancelled call is recorded without a duration" hold while a
          measured call without a host duration still gets the reporter's own
          stamp;
        - the wait's label for `SessionStatusChanged` is the harness's own
          `data.title` (bounded), falling back to `needs input`: the status
          change itself carries no question text (Task 11d reads the full text
          separately from `maki.session.messages`), and the record requires a
          label regardless.
        ```
  - [x] **REFACTOR** — keep one install helper and one flush helper shared with
        the other reporters; re-run → still exit 0. Paste it.
        ```
        The install side is unchanged from the table Task 21 introduced: the Maki
        rows go through the same `integrations_of` table and the same
        `write_embedded_file`, and its two merged files go through the same
        `target_merges` table as the Claude settings merge. On the driver side
        the ad-hoc `PATH` composition is gone: one `driver_path(dirs)` helper
        now builds the environment for every reporter process — the node
        drivers (`driver_path(&[&shim])`) and the Maki driver
        (`driver_path(&[maki_bin, &shim])`) share it. No behaviour change: the
        same files install, the same merges run, and every driver process gets
        the same `PATH` shape it had.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.51s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Maki's reporter keeps the harness's own tool duration and marks what its harness omits"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 3.51s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Agent Activity Reporting
          Scenario: Maki's reporter keeps the harness's own tool duration and marks what its harness omits
           ✔  Given a project with the generated Maki development reporter
           ✔  When the Maki harness reports a completed tool call
           ✔  Then the record holds that tool call with the duration the harness reported
           ✔  When the Maki harness reports a tool call that never ran
           ✔  Then the record holds that tool call without a measured duration
           ✔  When the Maki harness reports a subagent that started and completed
           ✔  Then the record holds that subagent's span
           ✔  And the record marks that subagent's token usage as unavailable
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `d098946c1e4a1fbc1c30dceac386a748f5aaeea3` — `feat(agent-activity-reporting): <release note>`
        with the scenario title `Maki's reporter keeps the harness's own tool
        duration and marks what its harness omits` in the body. Record the
        revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(agent-activity-reporting): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Maki's reporter keeps the harness's own tool duration and marks what its harness omits"
        [19:17:08] committed revision on the Git backend
        EXIT=0

        The revision carries the reporter assets, the full `maki` target, the
        two merged files' code, the intake's `phase` kind and subagent
        unavailable marker, the driver and its `maki` rows, the setup script and
        the two workflow prerequisites, the scenario's `@wip` removal, this task
        file's evidence, and the refreshed `@e2e` visual evidence the scope run
        produced (12 screenshot pairs).
        ```
  - **Corpus-pin reconciliation (2026-10-16, after the GREEN run above; no
    evidence above is altered).** The `maki` target extends the `all` shorthand,
    and two permanent scenarios in
    `givn/specs/initialize-project/initialize-a-project.feature` pin that list —
    "givn skills targets add accepts \"all\" as a shorthand for every target"
    and "The --agents flag accepts \"all\" as a shorthand for every target" —
    whose Then then read `opencode, agents, claude, omp`. This change therefore
    carries both as `@givn.modified` records in
    `givn/changes/development-timeline/specs/initialize-project/initialize-a-project.feature`
    (verbatim copies with only the list changed to
    `opencode, agents, claude, omp, maki`, no `@wip`) and patches the same two
    scenarios in the permanent file, so the delta and the corpus agree now and at
    merge. Same class as the S2b ADR-count reconciliation. The delta inventory is
    now 46 entries (43 `@givn.added`, 3 `@givn.modified`, 38 non-`@e2e`, 8
    `@e2e`), the per-feature table gained the `initialize-project` row (2/2/0),
    and every pending scope count reads 38 vs 8; the header's "sections" wording
    is unchanged because the two new entries are reconciliations, not scenario
    sections.

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
        `ranked_steps` function the model's summary carries (`longest_steps`), so
        the summary list is the record's one ranking and no later view has to
        sort a second time: Task 29's axis reads the same model, and Task 30's
        subagent figures reuse the same aggregation shape. No behaviour change:
        same ranking, same totals, same rendered text.

        Scope note (recorded on review of this box's own wording): the first
        draft said the axis "reads the ranking". It does not — the axis draws
        every span from the window's `at` order (Task 29) and reads the model,
        never a private sort; the ranking is what the summary list and the
        details panel show. The wording above is the accurate one; no code
        changed for this note.

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

## Task 25a — `Measured agent activity is ranked alongside Givn's own steps` (`development-report`, added)

- [x] **Scenario: `Measured agent activity is ranked alongside Givn's own steps`**
  - Design constraints:
    - Ranking rule (design §Report Generation): the ranking covers **every
      recorded entry with a measured `duration_ms`** — `command`, `phase`,
      `hook`, `step`, `revision`, `session`, `turn`, `tool`, `subagent`,
      `question` — longest first, with the existing tie-breaking (equal durations
      break on the earlier `at`, then the lower `id`), over the whole record and
      never the zoomed window. Today `src/cli/timeline.rs::ranked_steps` filters
      `entry.kind == STEP`, which is why this repository's own report shows an
      empty ranking while its record holds measured commands (a 3.6 s
      `skills sync`) and tool calls (a 799 ms `task`).
    - Fixture shape (the new Given): the seeded record holds a measured agent
      **tool** call longer than the longest measured `step`, so the ranking's
      first row is a non-step entry and the old `kind == "step"` filter cannot
      pass. The agent tool call is an ordinary `tool` entry built with the
      shared seeded-history builder
      (`tests/steps/development_timeline_support.rs`) — no new fixture
      machinery.
    - Each rendered row carries the entry's visible label, **its kind**, and its
      duration, so a gate step, a command, a tool call, a turn, and a subagent
      span are distinguishable. That means `RankedStep` gains a `kind` field and
      the row markup gains the kind element beside `step-label` and the
      duration; the list heading stays **"Longest steps"**,
      because these are the longest measured steps of the development whatever
      layer recorded them, so no design-system wording changes.
    - Assertions: the first Then reads the model's `summary.longest_steps` and
      finds the agent tool call first — the JSON field name stays
      `longest_steps`, because the existing Then and the `@e2e` model readers use
      it; the second Then reads the **rendered** ranking through the existing
      `list_entry_ids(&document, "longest-steps")` parser
      (`tests/steps/development_report_steps.rs:428`) and asserts the visible
      kind element names the kind of every entry the list holds, so the rendered
      markup is exercised, not only the model.
    - An entry the harness could not measure is absent from the ranking, never
      rendered as zero — the same missing-signal rule the rest of the report
      follows.
    - The written list is uncapped by design: it is bounded only by the record's
      own measured entries, because the design bounds the **drawn shapes** (the
      zoom aggregation rule), never the summary rows — so no cap, truncation, or
      "top N" is added here.
    - Files: `src/cli/timeline.rs` — the widened filter, the `kind` field, the
      row markup in `render_summary` (`~:1178-1186`), and the doc comments that
      still describe a steps-only ranking (`RankedStep`, `~:362-364`;
      `ranked_steps`, `~:711-715`; `Summary.longest_steps`, `~:158-159`) — and
      `assets/timeline/report.html`, which gains only the kind element's **style
      rule** (the asset carries the CSS; the element itself is rendered by
      `render_summary`, never by the asset). The steps go in
      `tests/steps/development_report_steps.rs`.
    - RED expected failure: the three new step phrases (one Given, two
      Then/And) are undefined.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Measured agent activity is ranked alongside Givn's own steps"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Measured agent activity is ranked alongside Givn's own steps"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 11.75s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: Measured agent activity is ranked alongside Givn's own steps
           ✔  Given a change "demo" whose record holds an agent tool call longer than any step
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report ranks the agent tool call first
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:69:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (3506849) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the three new step phrases (the Given,
        both Then/And) start undefined, and the Given and the When are stated in
        this step so the run stops exactly at the first assertion that does not
        exist yet. The report at this revision cannot satisfy it either way:
        `ranked_steps` filtered `entry.kind == STEP`, so the fixture's measured
        `tool` entry — longer than every step — is absent from the ranking
        entirely, and `RankedStep` carried no kind for a row to name.
        ```
  - [x] **GREEN** — widen the ranking, add the kind to the model and the row,
        and implement the steps; then re-run the whole existing ranking scenario
        (`The report ranks the longest single step first`) as well, so the wider
        rule is proven not to break the narrower claim. Re-run the `@e2e` scope
        (`GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh
        --e2e`) and commit the refreshed `evidence/visual/**` pairs it writes:
        the widened summary rows change the rendered report, so no committed
        pair may stay stale by content into the archive finalization (Task 24a
        refreshed the same evidence in its own commit). Compile first, re-run →
        exit 0. Paste them.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.41s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Measured agent activity is ranked alongside Givn's own steps"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 9.60s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: Measured agent activity is ranked alongside Givn's own steps
           ✔  Given a change "demo" whose record holds an agent tool call longer than any step
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report ranks the agent tool call first
           ✔  And the ranking names the kind of every entry it lists
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report ranks the longest single step first"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
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
        Both scopes, run in this GREEN step (the refreshed evidence pairs below
        are this run's output; the e2e scope was re-run after this scenario's
        REFACTOR and wrote byte-identical files, so the committed pairs are the
        post-refactor rendering):
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
        … PASS (39/39) givn::features_runner inspect-development-history/development-record.feature::A verification run is recorded as its own step
        ────────────
             Summary [   4.140s] 39 tests run: 39 passed, 8 skipped
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e
        … PASS (8/8) givn::features_runner inspect-development-history/agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record
        ────────────
             Summary [   7.631s] 8 tests run: 8 passed, 39 skipped
        EXIT=0
        ```
        Refreshed evidence pairs (14 files, 7 directories; the empty-state
        scenario's report renders no ranking rows, so its pair and its
        `--empty` pair are byte-identical and stay as committed):
        ```
        evidence/visual/consumer-generates-the-development-report-for-a-change/{mobile,desktop}.png
        evidence/visual/consumer-installs-the-agent-reporter-and-its-reported-activity-lands-in-the-record/{mobile,desktop}.png
        evidence/visual/consumer-opens-the-generated-development-report-in-a-browser/{mobile,desktop}.png
        evidence/visual/consumer-reads-the-development-record-as-machine-readable-output/{mobile,desktop}.png
        evidence/visual/consumer-runs-a-lifecycle-step-and-reads-its-recorded-actions/{mobile,desktop}.png
        evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/{mobile,desktop}.png
        evidence/visual/consumer-zooms-the-timeline-to-a-shorter-time-range/{mobile,desktop}.png
        ```

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `ranked_steps` now filters on
          `duration_ms.is_some()` alone, so every kind the record measured
          competes on its own duration; `RankedStep` gains `kind`;
          `Summary.longest_steps`' doc states the record-wide rule and keeps the
          JSON field name; the unused `STEP` constant is gone; the row markup
          emits `<span class="step-kind">` beside the label and the duration,
          and the `<li>` carries `data-kind`);
        - `assets/timeline/report.html` (modified — only the kind element's
          style rule: the muted token, tabular-free 12 px chip on the raised
          surface, `--radius-span`);
        - `tests/steps/development_report_steps.rs` (modified — the widened
          fixture Given (`MEASURED_ACTIVITY_FIXTURE`: two measured steps and one
          measured `tool` call longer than both), the `MeasuredEntry` fixture
          shape, the `RenderedRow`/`list_rows`/`row_element`/`modelled_ranking`
          readers, and both Then steps: the model's ranking is every measured
          entry plus the tool call first, and the rendered rows agree with the
          model row by row including the kind).
        ```
  - [x] **REFACTOR** — keep one ranking function and one row renderer (no
        second list for agent activity); re-run → still exit 0. Paste it.
        ```
        One row renderer: `ranked_row(step)` owns the ranking row's whole markup
        — the two data markers and the label/kind/duration elements — so the
        summary's list is one loop over the model's ranking and there is exactly
        one place a row can be rendered. There is still one ranking
        (`ranked_steps`) and one list (`#longest-steps`): the widened rule fed
        agent activity into the existing ranking rather than adding a second
        list for it, which is what the design's ranking rule asks for. No
        behaviour change: the same rows, byte for byte — the e2e scope re-run
        after this step rewrote the same 14 evidence files with no further
        change.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.32s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Measured agent activity is ranked alongside Givn's own steps"
        Feature: Development Report
          Scenario: Measured agent activity is ranked alongside Givn's own steps
           ✔  Given a change "demo" whose record holds an agent tool call longer than any step
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report ranks the agent tool call first
           ✔  And the ranking names the kind of every entry it lists
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        $ ... --exact "...::The report ranks the longest single step first"
        Feature: Development Report
          Scenario: The report ranks the longest single step first
           ✔  Given a change "demo" whose record holds steps of different durations
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report ranks the longest step first
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e
        … PASS (8/8)
        ────────────
             Summary [   7.110s] 8 tests run: 8 passed, 39 skipped
        EXIT=0
        ```
  - [x] **COMMIT** — `e4b798824f5bea96f73177b77f021a5e795ef273` — `feat(development-report): <release note>` with
        the scenario title `Measured agent activity is ranked alongside Givn's own
        steps` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Measured agent activity is ranked alongside Givn's own steps"
        [03:32:22] committed revision on the Git backend
        EXIT=0

        Revision e4b798824f5bea96f73177b77f021a5e795ef273 touches exactly this
        scenario's files: `src/cli/timeline.rs` (the widened `ranked_steps`, the
        `RankedStep.kind`, the one `ranked_row` renderer, and the doc comments
        and unused constant it retires), `assets/timeline/report.html` (only the
        kind element's style rule), `tests/steps/development_report_steps.rs`
        (the widened fixture, the rendered-row readers, and both Then steps),
        the delta `.feature`'s `@wip` removal, this task file's evidence, and the
        14 refreshed `evidence/visual/**` pairs this step's widened summary rows
        changed.
        ```
        The refreshed pairs are one commit with the scenario, exactly as Task 24a
        refreshed the same evidence in its own commit: no committed pair may stay
        stale by content into the archive finalization. The refresh ran twice —
        once in GREEN, once after REFACTOR — and the second run wrote the same 14
        files, so the committed pairs are the post-refactor rendering.

## Task 25b — `The report details panel carries the captured metadata and content` (`development-report`, added)

- [x] **Scenario: `The report details panel carries the captured metadata and content`**
  - Design constraints:
    - The fixture is a `demo` record holding the captured metadata and
      `full`-mode `data.content`; the content exceeds the 8 MiB report budget, so
      the fixture must include both embedded and omitted content.
    - The report model carries the exact facts the panel renders: session and
      title, status, stop reason, model, cost (real and list), context size and
      window, tool call id, output bytes, bounded error, parent/agent, retry,
      and permission mode. Values the entry does not carry render "not reported",
      never zero.
    - `ReportModel` gains `content_budget_bytes: 8_388_608` and `content_bytes`;
      `render_report` embeds `data.content` fields in record order up to the
      budget, then marks the affected entries `content_omitted: true` and writes
      the exact note `content omitted; see the record` in the text view and the
      model payload. The store is never modified (the scenario asserts every
      content byte remains).
    - The content region is one bounded, scrollable, escaped region beneath the
      facts, one labelled block per named content field, with its cap and
      `truncated` mark; it is not part of the accessible summary line.
    - **E7 extension (same scenario/action, no new `@e2e`):** extend the existing
      `Consumer selects a timeline element and reads its details` step assertions
      to read one new metadata fact (the tool call id) and the bounded content
      region from the panel. Re-run that scenario and refresh its two committed
      captures
      (`evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/{mobile,desktop}.png`)
      in this task's GREEN; the commit carries this task's scenario title only,
      so E7 is not committed twice. The E7 fixture's longest recorded step is a
      `tool` entry carrying `tool_call_id` and one bounded `data.content`
      field, so the same selection action reads the new fact and the region.
    - Files: `src/cli/timeline.rs` (facts passthrough, budget/omission model),
      `assets/timeline/report.html` (fact list and content region),
      `tests/steps/development_report_steps.rs` (the new scenario),
      `tests/steps/development_report_e2e_steps.rs` (the E7 extended
      assertions), `tests/steps/development_timeline_support.rs` (the
      over-budget full-content fixture), and the refreshed E7 captures.
    - RED expected failure: the new Then phrases (facts list, budget, omission
      note, store completeness) are undefined, and the extended E7 assertions
      match no step function.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report details panel carries the captured metadata and content"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report details panel carries the captured metadata and content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report details panel carries the captured metadata and content
           ✘  Given a change "demo" whose record holds captured metadata and full-mode content
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:74:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (3870571) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
        The E7 extension's new assertion also matches no step function before
        its step is added (`the panel names the entry's tool call id`), which is
        the same undefined-step RED for the `@e2e` half.
  - [x] **GREEN** — implement the facts/budget/omission rendering, extend the E7
        assertions, re-run both scenarios → exit 0, and refresh the E7 captures.
        Paste both runs and the refreshed capture list.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.94s
        BUILD_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report details panel carries the captured metadata and content"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report details panel carries the captured metadata and content
           ✔  Given a change "demo" whose record holds captured metadata and full-mode content
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report model names the session, its title, the status, the stop reason, the cost, the context size and window, the tool call id, the output size, the error, the parent, the retry, and the permission mode
           ✔  And the report embeds the content within the 8 MiB documented budget
           ✔  And the report notes "content omitted; see the record" beyond the budget
           ✔  And the development record still holds every content byte
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✔  Given an open development report with a recorded history
           ✔  When the consumer selects the longest recorded step
           ✔  Then the browser shows a details panel for that step
           ✔  And the panel names its start, its duration, and its outcome
           ✔  And the panel names the entry's tool call id
           ✔  And the panel renders the entry's content in its bounded content region
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0

        Refreshed E7 captures (written by the scenario's own
        `capture_evidence_from`, committed with this revision):
          givn/changes/development-timeline/evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/desktop.png
          givn/changes/development-timeline/evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/mobile.png
        ```
  - [x] **REFACTOR** — one fact renderer and one content-budget pass; re-run →
        still exit 0. Paste it.
        ```
        $ cargo fmt --all -- --check
        FMT_EXIT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.55s
        CLIPPY_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report details panel carries the captured metadata and content"
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        REGULAR_EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        E7_EXIT=0

        The facts are rendered by one `panelFacts`/`fillPanel` pair (with the
        one `retryText` formatter), the content region by one `fillContent`, and
        the budget by one `apply_content_budget` pass whose omission marker and
        note are read by the text view, the model payload, and the panel alike.
        ```
  - [x] **COMMIT** — `415fdea0c44d3c80d932aa6d843d65d4a83d8c16` — `feat(development-report): <release note>` with
        this scenario's title in the body and a line noting the E7 assertion
        extension (same scenario/action; no new `@e2e`). Record the revision
        identifier.

## Task 26 — `The report totals the time spent waiting for questions` (`development-report`, added)

- [x] **Scenario: `The report totals the time spent waiting for questions`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals the time spent waiting for questions"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals the time spent waiting for questions"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.17s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report totals the time spent waiting for questions
           ✔  Given a change "demo" whose record holds two question waits
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report names the total wait duration and the number of waits
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:15:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2563738) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the totals Then step matches no function,
        and the run stops exactly there. The fixture Given (two question waits
        with known durations, nested in one measured turn whose duration differs
        from their sum) and the report-generating When are in place, so the RED
        is the missing wait total, and the report at this revision carries no
        wait figure at all rather than a wrong one.
        ```
  - [x] **GREEN** — implement the wait total/count in the model and its summary
        tile. Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.25s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals the time spent waiting for questions"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.50s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report totals the time spent waiting for questions
           ✔  Given a change "demo" whose record holds two question waits
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the total wait duration and the number of waits
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `Summary` gains `waits`,
          `unmeasured_waits` and `wait_ms`, accumulated from the `question`
          entries inside the summary's one pass; `wait_note`/`wait_count` render
          the tile's note; the Waiting tile joins the summary section. The total
          is the waits' own `duration_ms` sum, never the enclosing turn's
          duration, and a wait without a measured duration counts as a wait and
          is never added as zero);
        - `assets/timeline/report.html` (unchanged — the tiles' styles already
          carry the waiting tile: one `.tile` primitive in the design system's
          tokens, and the note reuses the muted/warn text styles);
        - `tests/steps/development_report_steps.rs` (modified — the wait fixture
          Given, the tile reader, and the totals Then step: control on the
          fixture's turn duration, model/rendered agreement, and the count).
        ```
  - [x] **REFACTOR** — one aggregation pass over the entries feeding every
        summary number; re-run → still exit 0. Paste it.
        ```
        The measured-total rule got its one home: `add_measured` adds a recorded
        duration to a running `Option<u64>` and reports whether the record
        measured it at all, so the wait total (and the subagent and token totals
        the next scenarios add) can never add a missing duration as zero. The
        summary's single pass is unchanged: `Summary::of` walks the window's
        entries once and every figure — count, elapsed, unmeasured durations,
        waits, wait total — comes out of that one scan. No behaviour change: the
        same total, the same count, the same note.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.55s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals the time spent waiting for questions"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.99s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report totals the time spent waiting for questions
           ✔  Given a change "demo" whose record holds two question waits
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the total wait duration and the number of waits
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Regression run of the scenario this step's shared summary pass touches
        (the aggregation and the rendered summary):

        $ ... --exact "...::The report ranks the longest single step first"
        Feature: Development Report
          Scenario: The report ranks the longest single step first
           ✔  Given a change "demo" whose record holds steps of different durations
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report ranks the longest step first
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        ```
  - [x] **COMMIT** — `8dbdb6abff389ee7dce1bcac3d997ada5a730ea6` — `feat(development-report): <release note>` with
        the scenario title `The report totals the time spent waiting for
        questions` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The report totals the time spent waiting for questions"
        [16:35:44] committed revision on the Git backend
        EXIT=0

        Revision 8dbdb6abff389ee7dce1bcac3d997ada5a730ea6 touches exactly this
        scenario's files: `src/cli/timeline.rs` (the wait fields and the Waiting
        tile), `tests/steps/development_report_steps.rs` (the wait fixture Given
        and the totals Then), the delta `.feature`'s `@wip` removal, and this
        task file's evidence. The asset is unchanged — the tile primitive the
        design system already styles carries the new tile.
        ```

## Task 27 — `The report totals token usage per turn and per subagent` (`development-report`, added)

- [x] **Scenario: `The report totals token usage per turn and per subagent`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals token usage per turn and per subagent"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals token usage per turn and per subagent"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 10.17s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report totals token usage per turn and per subagent
           ✔  Given a change "demo" whose record holds reported token usage
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report names the total tokens for the change
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:21:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2571417) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the scenario's three Then steps all match
        no function, and the run stops at the first. The fixture Given (two turns
        with reported usage for two different models, one subagent whose span owns
        its own usage, one subagent the harness reports no usage for, and one turn
        whose usage the record marks unavailable) and the report-generating When
        are in place, so the RED is the missing token roll-up: the report at this
        revision states no token figure at all.
        ```
  - [x] **GREEN** — implement the usage roll-ups and the unmeasured count.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.48s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals token usage per turn and per subagent"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 16.69s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report totals token usage per turn and per subagent
           ✔  Given a change "demo" whose record holds reported token usage
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the total tokens for the change
           ✔  And the report names the tokens of every subagent
           ✔  And the report names the token total for every model that produced the usage
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `Summary` gains `tokens_total`,
          `tokens_by_model`, `tokens_by_subagent` and `unmeasured_turns`;
          `TokenRollUp` is the usage aggregation (`data.total_tokens` summed per
          model the entry names, per subagent span the entry belongs to);
          `usage_unavailable` reads Task 15's `data.usage: "unavailable"` marker
          so a turn without reported usage is counted, never summed as zero;
          `format_count` renders exact counts; the Tokens tile, the
          unmeasured-turns note, and the two token lists join the summary);
        - `assets/timeline/report.html` (unchanged — the new rows are the same
          tile/list/summary-note primitives, in the tokens the asset carries
          inline; no style was missing);
        - `tests/steps/development_report_steps.rs` (modified — the usage
          fixture Given with its `TurnFixture`/`SubagentFixture`/`UsageCounts`
          shapes, the seeded-usage readers, and the three Then steps, whose
          expectations are the fixture's own reported counts).
        ```
  - [x] **REFACTOR** — one usage aggregation shared by the per-model and
        per-subagent views; re-run → still exit 0. Paste it.
        ```
        The roll-up now owns both views' row shapes: `TokenRollUp::model_rows`
        and `TokenRollUp::subagent_rows` build the `TokenTotal` rows the summary
        carries, so the per-model and the per-subagent view can never be built
        from two differently-shaped scans. The `data.subagent` read moved into
        `usage_named_subagent`, the one place that decides whether a usage entry
        names a subagent (used by `usage_belongs_to`, which joins span identity
        and parent link). The total still comes from the summary's single pass
        through the same `usage_total` accessor the roll-up uses. No behaviour
        change: same totals, same rows, same rendered text.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.32s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report totals token usage per turn and per subagent"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.72s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report totals token usage per turn and per subagent
           ✔  Given a change "demo" whose record holds reported token usage
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the total tokens for the change
           ✔  And the report names the tokens of every subagent
           ✔  And the report names the token total for every model that produced the usage
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Regression runs of the two committed scenarios this step's shared
        summary code touches:

        $ ... --exact "...::The report ranks the longest single step first"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        $ ... --exact "...::The report totals the time spent waiting for questions"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        ```
  - [x] **COMMIT** — `94b20d539a16879af55bcb58b657b71a6718e94e` — `feat(development-report): <release note>` with
        the scenario title `The report totals token usage per turn and per
        subagent` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The report totals token usage per turn and per subagent"
        [16:39:37] committed revision on the Git backend
        EXIT=0

        Revision 94b20d539a16879af55bcb58b657b71a6718e94e touches exactly this
        scenario's files: `src/cli/timeline.rs` (the usage roll-up, the
        unavailable-turn count, and the token tiles/lists),
        `tests/steps/development_report_steps.rs` (the usage fixture and the
        three Then steps), the delta `.feature`'s `@wip` removal, and this task
        file's evidence. The asset is unchanged — the tile, list, and note
        primitives it already styles carry the new figures.
        ```

## Task 28 — `The report of an ideation topic names the work done before the change exists` (`development-report`, added)

- [x] **Scenario: `The report of an ideation topic names the work done before the change exists`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report of an ideation topic names the work done before the change exists"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report of an ideation topic names the work done before the change exists"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
        warning: constant `IDEATION_TOPIC` is never used
           --> tests/steps/development_timeline_support.rs:117:11
        …
        warning: `givn` (test "features_runner") generated 1 warning
            Finished `test` profile [unoptimized + debuginfo] target(s) in 7.01s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report of an ideation topic names the work done before the change exists
           ✔  Given an ideation topic "timeline-ideas" whose record holds the topic's own steps
           ✔  When the developer generates the development report for the topic "timeline-ideas"
           ✘  Then the report names the topic's steps under the topic scope
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:29:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2579764) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — both Then steps match no function, and the
        run stops at the first. The fixture Given (a written-but-unfinished topic
        session, the topic's own step entries under the topic scope, and no
        change directory at all) and the topic-generating When are in place, so
        the RED is the missing topic window: the report at this revision renders
        the scope line and the summary but no band row at all. The `IDEATION_TOPIC`
        constant the fixture had drafted went unused and was dropped with this
        GREEN step rather than kept as dead weight.
        ```
  - [x] **GREEN** — implement the topic window and its band. Files:
        `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs` (topic fixture). Compile
        first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.26s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report of an ideation topic names the work done before the change exists"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 16.61s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report of an ideation topic names the work done before the change exists
           ✔  Given an ideation topic "timeline-ideas" whose record holds the topic's own steps
           ✔  When the developer generates the development report for the topic "timeline-ideas"
           ✔  Then the report names the topic's steps under the topic scope
           ✔  And the report names no change that does not exist yet
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — the report's timeline section:
          `render_timeline`/`render_band`/`render_span` draw one visible row per
          band, each naming its scope and one span per covered entry, and the
          report's `TimeScale` maps the window's recorded range onto the drawing
          width from the embedded timestamps alone; `duration_text` is the one
          duration-or-"not measured" renderer the rows share with the summary,
          so a span's accessible name is always `<label>, <duration>`);
        - `assets/timeline/report.html` (modified — the timeline, band row, and
          span styles in the design system's tokens, with one band hue per band
          class and the reduced-motion rule the report already carried);
        - `tests/steps/development_report_steps.rs` (modified — the topic Given,
          the topic-generating When, the timeline-region reader, and both Then
          steps: the band's own coverage, the steps drawn under their band, and
          the no-invented-change claim);
        - `tests/steps/development_timeline_support.rs` (modified — the reusable
          topic fixture: `seed_ideation_topic` writes the written-but-unfinished
          session and seeds the topic's own measured steps through the one
          builder, so the browser tier's topic row reports the same topic).
        ```
  - [x] **REFACTOR** — one window-selection rule for change, topic, and project
        (no third copy of the scope switch); re-run → still exit 0. Paste it.
        ```
        The scope switch now has one home: `scope_row(scope)` derives a scope's
        class, its visible label, and its identifier, and the three callers that
        used to carry their own `match` — `Band::of`, `scope_line`, and
        `default_report_path` (plus the band row's `data-band` attribute) — all
        read that one row. The window selection itself stays one function:
        `bands_for` gives a named scope its single band (a change, a topic — the
        same rule, no topic special case) and groups an unnamed report's entries
        by the scope each belongs to, so the project report's "outside a scope"
        band is the same derivation. No behaviour change: same bands, same
        labels, same default path.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.82s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report of an ideation topic names the work done before the change exists"
        Feature: Development Report
          Scenario: The report of an ideation topic names the work done before the change exists
           ✔  Given an ideation topic "timeline-ideas" whose record holds the topic's own steps
           ✔  When the developer generates the development report for the topic "timeline-ideas"
           ✔  Then the report names the topic's steps under the topic scope
           ✔  And the report names no change that does not exist yet
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Regression runs of the three committed scenarios this step's shared
        window-selection code touches:

        $ ... --exact "...::The report ranks the longest single step first"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        $ ... --exact "...::The report totals the time spent waiting for questions"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        $ ... --exact "...::The report totals token usage per turn and per subagent"
        1 feature, 1 scenario (1 passed), 5 steps (5 passed)        EXIT=0
        ```
  - [x] **COMMIT** — `2f6f77cb2f04d11765ecfbb72bdcc7c656746429` — `feat(development-report): <release note>` with
        the scenario title `The report of an ideation topic names the work done
        before the change exists` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The report of an ideation topic names the work done before the change exists"
        [16:43:03] committed revision on the Git backend
        EXIT=0

        Revision 2f6f77cb2f04d11765ecfbb72bdcc7c656746429 touches exactly this
        scenario's files: `src/cli/timeline.rs` (the timeline bands, the
        `TimeScale`, and the one `scope_row` derivation),
        `assets/timeline/report.html` (the band/span styles),
        `tests/steps/development_report_steps.rs` (the topic Given/When and both
        Then steps), `tests/steps/development_timeline_support.rs` (the reusable
        topic fixture), the delta `.feature`'s `@wip` removal, and this task
        file's evidence.
        ```

## Task 29 — `The report orders the timeline by recorded time, not by store position` (`development-report`, added)

- [x] **Scenario: `The report orders the timeline by recorded time, not by store position`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report orders the timeline by recorded time, not by store position"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report orders the timeline by recorded time, not by store position"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 6.78s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report orders the timeline by recorded time, not by store position
           ✔  Given a change "demo" whose record holds an entry appended after an entry with a later end time
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report orders the entries by their recorded end time
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:36:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2587728) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the ordering Then step matches no
        function, and the run stops there. The fixture Given (a record whose
        second entry really ended earlier than the first, seeded through the
        store's own append path, so the entry ids are file-ordered too) and the
        When are in place. The report at this revision draws the timeline in the
        store's line order: `build_report_model` kept the lines exactly as
        `entries_in_scope` read them, and the drawn spans and axis inherited
        that order.
        ```
  - [x] **GREEN** — implement the `at` ordering in the model. Files:
        `src/cli/timeline.rs`, `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs` (the out-of-order seed).
        Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.10s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report orders the timeline by recorded time, not by store position"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 9.92s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report orders the timeline by recorded time, not by store position
           ✔  Given a change "demo" whose record holds an entry appended after an entry with a later end time
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report orders the entries by their recorded end time
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `ordered_by_recorded_time` is the
          report's one ordering: by `at`, `id` only as the tie breaker for two
          entries that record the same instant, an unparsable `at` last;
          `build_report_model` applies it to the window before it bands, ranks,
          or scales, so the model's entries, every band's spans, and the drawn
          axis share it. The axis strip arrived with it: `TimeScale::ticks`
          picks round instants at the smallest step that keeps the count within
          `MAX_TICKS`, `TimeScale::label` renders each tick's time of day, and
          `render_axis` draws them above the band rows — all derived from the
          embedded timestamps, never the wall clock);
        - `assets/timeline/report.html` (modified — the tick text style, in the
          muted token with tabular numerals);
        - `tests/steps/development_report_steps.rs` (modified — the
          out-of-order Given, the drawn-span and drawn-tick readers, and the
          ordering Then step, which first proves the fixture's file order
          contradicts its time order);
        - `tests/steps/development_timeline_support.rs` (modified — the
          out-of-order seed: `seed_out_of_order_steps` writes a record whose
          second entry really ended earlier than the first).
        ```
  - [x] **REFACTOR** — one ordering comparator applied to every band; re-run →
        still exit 0. Paste it.
        ```
        The ordering is now visibly one comparator for the whole report: the
        window is ordered by `at` once, before it is banded, ranked, or scaled, so
        every band's spans, the drawn axis, and the summary inherit that one
        order — `bands_for` states the rule instead of re-sorting anything. The
        summary's elapsed time dropped the second time scan it no longer needed:
        `recorded_span` reads the ordered window's two ends (the window is
        ordered, so its ends are its extremes) and answers "not measured" when
        an end is unreadable. No behaviour change: same order, same elapsed,
        same rendering.

        $ cargo fmt --all -- --check
        FMT OK
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.50s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report orders the timeline by recorded time, not by store position"
        Feature: Development Report
          Scenario: The report orders the timeline by recorded time, not by store position
           ✔  Given a change "demo" whose record holds an entry appended after an entry with a later end time
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report orders the entries by their recorded end time
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Regression runs of the four committed scenarios this bundle's shared
        model and renderer feed (all five scenarios of the bundle now run green
        together):

        $ ... --exact "...::The report ranks the longest single step first"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        $ ... --exact "...::The report totals the time spent waiting for questions"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        $ ... --exact "...::The report totals token usage per turn and per subagent"
        1 feature, 1 scenario (1 passed), 5 steps (5 passed)        EXIT=0
        $ ... --exact "...::The report of an ideation topic names the work done before the change exists"
        1 feature, 1 scenario (1 passed), 4 steps (4 passed)        EXIT=0
        ```
  - [x] **COMMIT** — `429fd12a47ff35040b80c1d71f844adc49cd604c` — `feat(development-report): <release note>` with
        the scenario title `The report orders the timeline by recorded time, not
        by store position` in the body. Record the revision identifier.
        ```
        $ GIVN_ATTRIBUTION_ACTIVE=1 GIVN_ATTRIBUTION_LLM=deepseek/deepseek-flash \
          GIVN_ATTRIBUTION_REASONING=max GIVN_ATTRIBUTION_HARNESS=omp \
          ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The report orders the timeline by recorded time, not by store position"
        [16:45:43] committed revision on the Git backend
        EXIT=0

        Revision 429fd12a47ff35040b80c1d71f844adc49cd604c touches exactly this
        scenario's files: `src/cli/timeline.rs` (the one `at` ordering, the axis
        strip, and the elapsed read from the ordered ends),
        `assets/timeline/report.html` (the tick style),
        `tests/steps/development_report_steps.rs` (the out-of-order Given, the
        drawn-span/drawn-tick readers, and the ordering Then step),
        `tests/steps/development_timeline_support.rs` (the out-of-order seed),
        the delta `.feature`'s `@wip` removal, and this task file's evidence.
        ```

## Task 30 — `The report names the agent's fan-out into small subagents` (`development-report`, added)

- [x] **Scenario: `The report names the agent's fan-out into small subagents`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report names the agent's fan-out into small subagents"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report names the agent's fan-out into small subagents"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report names the agent's fan-out into small subagents
           ✘  Given a change "demo" whose record holds many short subagents
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:40:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (2604757) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — neither the fan-out Given nor the
        subagent-count Then matches a function, so the run stops on the first
        unbound step. The scenario's contract did not hold before this revision:
        the summary carried no subagent count and no combined subagent duration
        at all (`Summary` had waits and tokens only), so the model could not
        name a fan-out even if the step had been bound.
        ```
  - [x] **GREEN** — implement the subagent count and combined duration. Files:
        `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.86s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report names the agent's fan-out into small subagents"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 8.06s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report names the agent's fan-out into small subagents
           ✔  Given a change "demo" whose record holds many short subagents
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the number of subagents and their combined duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `Summary` gains `subagents` (the
          count of recorded `subagent`-kind spans), `subagent_ms` (their combined
          measured duration, `None` while the record measured none) and
          `unmeasured_subagents` (the spans the record does not time);
          `Summary::of` fills them in the one pass it already makes over the
          window, through the same `add_measured` totals Task 23/26 introduced so
          an unmeasured subagent is never added as zero; `render_summary` gains
          the `subagents` tile — its value the combined duration (or "not
          measured"), its note the fan-out's size through `subagent_note` /
          `subagent_count`. The fan-out is computed from the recorded
          `subagent` entries only: no label is read, no spawn relationship is
          inferred);
        - `tests/steps/development_report_steps.rs` (modified — the fan-out
          fixture (`FAN_OUT_SUBAGENTS`, six short subagents nested in one turn,
          one of them untimed), the Given that seeds it through the
          seeded-history builder's own append path, and the Then step, which
          derives the expected count and combined duration from the seeded
          record, proves the fixture's spawn relation is the record's `parent`
          link (no subagent label names the turn) and reads the rendered tile);
        - `assets/timeline/report.html` — unchanged: the new tile is the
          existing `.tile` / `.tile-note` primitive, so the asset needed no new
          style and the scenario's report is still one self-contained file.

        The fixture's numbers: six subagents (20 s, 15 s, 12 s, 8 s, one
        unmeasured, 5 s) nested in a 1 200 s turn → the tile reads
        `1 m` with the note `6 subagents · 1 not measured`, never the turn's
        20 m, and the combined duration (60 s) is deliberately neither the
        turn's duration nor a per-subagent guess.
        ```
  - [x] **REFACTOR** — reuse Task 26's aggregation for the subagent durations;
        re-run → still exit 0. Paste it.
        ```
        The waits tile and the subagents tile now share one counted-noun helper:
        `wait_count` and `subagent_count` are replaced by `counted(count, noun)`,
        and both notes are the same shape — the count plus `· N not measured`
        only where the record withholds a duration — so the two views cannot
        drift apart in wording or in the zero-vs-unmeasured rule. No behaviour
        change: same values, same notes for every record.

        $ cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.29s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report names the agent's fan-out into small subagents"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 20.00s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report names the agent's fan-out into small subagents
           ✔  Given a change "demo" whose record holds many short subagents
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the number of subagents and their combined duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Regression run of the scenario whose aggregation the refactor touched
        (the waits tile reads its count through the same helper now):

        $ ... --exact "...::The report totals the time spent waiting for questions"
        1 feature, 1 scenario (1 passed), 3 steps (3 passed)        EXIT=0
        ```
  - [x] **COMMIT** — `51d0165d195d37820f467a9c2fe3e1a830d259c2` — `feat(development-report): <release note>` with
        the scenario title `The report names the agent's fan-out into small
        subagents` in the body. Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        The report names the agent's fan-out into small subagents"
        [16:52:20] committed revision on the Git backend
        EXIT=0

        $ git show --format='%H%n%B' --stat 51d0165d
        51d0165d195d37820f467a9c2fe3e1a830d259c2
        feat(development-report): Givn records the development process of every change and ideation — … opens in a browser.

        The report names the agent's fan-out into small subagents

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 125 ++++++++++++++++--
         src/cli/timeline.rs                                |  54 +++++++-
         tests/steps/development_report_steps.rs            | 145 +++++++++++++++++++++
         4 files changed, 311 insertions(+), 15 deletions(-)

        Revision 51d0165d touches production source (`src/cli/timeline.rs` — the
        summary's subagent count, combined duration and unmeasured count, and the
        subagents tile), the scenario's test steps, the delta `.feature`'s `@wip`
        removal, and this task file's evidence — never the spec alone.
        ```

## Task 31 — `The report needs no network access to render` (`development-report`, added)

- [x] **Scenario: `The report needs no network access to render`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report needs no network access to render"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report needs no network access to render"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report needs no network access to render
           ✔  Given a change "demo" with a recorded history
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report references no remote resource
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:48:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (2610263) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the no-remote-resource Then step matches
        no function. (The Given is the shared recorded-history fixture bound by
        Task 1's file, and the When is the same generation step the earlier
        report scenarios use, so the run reaches the unbound Then.) The claim
        itself had no assertion at this revision: nothing read the written
        document for a resource reference, so nothing kept the report
        self-contained against a future edit.
        ```
  - [x] **GREEN** — render the asset with zero external references. Files:
        `assets/timeline/report.html`, `src/cli/timeline.rs` (escaping and the
        data-slot substitution), `tests/steps/development_report_steps.rs`.
        Compile/write the asset, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.60s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report needs no network access to render"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.59s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report needs no network access to render
           ✔  Given a change "demo" with a recorded history
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report references no remote resource
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - none — and that is the investigated finding, not an omission. The
          claim is an *absence* of remote references in the file the renderer
          writes, and the renderer already writes such a file by construction:
          there is exactly one asset (`assets/timeline/report.html`, embedded
          with `include_str!`), every style lives in its one inline `<style>`,
          the model is the one inline `<script id="report-model">`, the SVG is
          generated inline by `render_view`/`render_timeline`, and the escaping
          (`escape`, `json_for_slot`) only ever turns recorded text into
          document text — it can add no element and no attribute. Neither
          `assets/timeline/report.html` nor `src/cli/timeline.rs` was therefore
          modified: the file the command wrote was scanned and carries no
          resource reference, so nothing needed changing to "render the asset
          with zero external references".
        - `tests/steps/development_report_steps.rs` (modified — the
          `remote_resource_reference` scan and its two marker lists, the
          discrimination controls, and the Then step: the scan over the written
          document, the one-document/inline-integrity assertions, and the
          command's own half of the claim).
        - the scan's own discrimination is part of the step: five control
          documents (a `<script src="https://…">`, a protocol-relative
          `<link href="//…">`, a CSS `@import`, an in-page `fetch(`, an
          `XMLHttpRequest` construction) are each detected by the same function
          the clean document is checked with, so a scan that never looks cannot
          pass. The written report itself is clean: it carries one inline
          `<style>`, exactly one `<script` element (the inline model), and no
          loading attribute pointing off-file. The SVG namespace declaration
          the design constraint names is not present in this document (the
          renderer's SVG is HTML-inline and needs no namespace); the scan reads
          only loading attributes and fetch forms, so it would not fail on it
          either way, and nothing here removes a namespace.
        - the command's half of the claim is asserted too: the fixture project
          root holds no server, no port, and no external service (the Local
          Runnability statement), the generation runs as a subprocess in it, and
          the path the command *prints* resolves to exactly the file the
          scenario's When read the report from — so "the command writes and
          prints the file with no network available" is the scenario's own
          evidence, not prose.
        ```
  - [x] **REFACTOR** — keep every style/script inline in one document, no
        separate assets to load; re-run → still exit 0. Paste it.
        ```
        Assertion adjusted 2026-10-16 (E-tier), recorded here without rewriting
        the pasted evidence above: the "exactly one `<script` element (the
        inline model)" count this block named was over-strict. `design.md`
        §Report Generation requires the asset's *inline script* to build the
        interactive axis from the model, so a design-conformant report
        necessarily carries the model payload element **plus** that inline
        interaction script. The Then step in
        `tests/steps/development_report_steps.rs` now asserts exactly one
        `<script id="report-model">` and that no `<script src` / `src="http"`
        appears anywhere — the same "nothing for the browser to fetch beyond the
        document itself" claim, aligned with the reviewed design. Revision of
        that step-file edit: see the alignment commit recorded below.
        Alignment revision: `d08992027b8b4d9dcbbb4f0afcf9e73ef0eb3657` —
        `fix(development-report): align the no-network assertion with the
        design's inline script`.

        Assertion adjusted 2026-10-16 (E-tier), recorded here without rewriting
        the pasted evidence above: the "exactly one `<script` element (the
        inline model)" count this block named was over-strict. `design.md`
        §Report Generation requires the asset's *inline script* to build the
        interactive axis from the model, so a design-conformant report
        necessarily carries the model payload element **plus** that inline
        interaction script. The Then step in
        `tests/steps/development_report_steps.rs` now asserts exactly one
        `<script id="report-model">` and that no `<script src` / `src="http"`
        appears anywhere — the same "nothing for the browser to fetch beyond the
        document itself" claim, aligned with the reviewed design. Revision of
        that step-file edit: see the alignment commit recorded below.

        Nothing was split out to load: the report stays one document with its
        one inline `<style>` and its one inline model script, and the asset was
        not touched (adding a second asset is exactly what this constraint
        forbids). The step's scan was tightened instead: the two marker lists
        moved to module scope as `LOADING_ATTRIBUTES` and `REMOTE_FETCH_FORMS`,
        so the attribute half and the fetch half of the scan are named in one
        place each and the doc comment states the rule (a `data:` URL, a
        fragment, and a relative path stay inside the file; a namespace URI is
        never a request). No behaviour change: same scan, same markers, same
        verdicts.

        $ cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.14s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report needs no network access to render"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 5.21s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report needs no network access to render
           ✔  Given a change "demo" with a recorded history
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report references no remote resource
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `50468ec60d74aea2be403969487d907254675fc0` — `feat(development-report): <release note>` with
        the scenario title `The report needs no network access to render` in the
        body. Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — … opens in a browser.

        The report needs no network access to render"
        [16:54:44] committed revision on the Git backend
        EXIT=0

        $ git show --stat --format='%H' 50468ec6
        50468ec60d74aea2be403969487d907254675fc0

         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 135 +++++++++++++++++++--
         tests/steps/development_report_steps.rs            | 121 ++++++++++++++++++
         3 files changed, 249 insertions(+), 9 deletions(-)

        Revision 50468ec6 touches the scenario's test steps (the scan, its
        discrimination controls, and the command-prints-the-path assertion), the
        delta `.feature`'s `@wip` removal, and this task file's evidence. It
        carries no production-source change by design: the claim is an absence
        of remote references in the written document, the one-asset renderer
        already satisfies it, and the scan is what makes it provable and keeps
        it proven — the investigated finding recorded under GREEN above.
        ```

## Task 32 — `The report shows the empty state for a scope with no recorded activity` (`development-report`, added)

- [x] **Scenario: `The report shows the empty state for a scope with no recorded activity`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report shows the empty state for a scope with no recorded activity"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report shows the empty state for a scope with no recorded activity"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report shows the empty state for a scope with no recorded activity
           ✘  Given a change "demo" with no recorded activity
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:52:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (2615618) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the empty-record Given matches no
        function, and the two Then steps (the empty state, the absent axis) are
        unbound too, so the run stops on the first one. The contract did not hold
        before this revision: `ReportModel` had no `empty` marker at all, the
        report root always rendered the summary and the timeline, and a
        scope-with-no-entries report was written as a timeline section whose
        band row carried no spans — a blank axis rather than a stated state.
        ```
  - [x] **GREEN** — implement the `empty` marker and the empty-state rendering.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.82s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report shows the empty state for a scope with no recorded activity"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 8.65s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report shows the empty state for a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report shows the empty state for "demo"
           ✔  And the report draws no timeline axis for it
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `ReportModel` gains the `empty`
          marker (`build_report_model`: the window holds no entry), `bands_for`
          returns no rows for an empty window so the two branches agree that
          there is nothing to draw, `render_view` branches on the marker, and
          `render_empty_state` renders the state: the heading "No recorded
          activity", one sentence naming the scope, and the link back to the
          report root (`#report-title`). No summary and no `render_timeline`
          call on this branch, so no axis, tick, band, or span markup is
          written at all);
        - `assets/timeline/report.html` (modified — the `.empty-state`
          primitive's styles: the raised surface, the border, the panel radius,
          the 24 px padding, the muted sentence, and the accent-coloured way
          back — the tokens the design system's empty state uses);
        - `tests/steps/development_report_steps.rs` (modified — the
          no-recorded-activity Given, the empty-state region reader, and both
          Then steps);
        - `tests/steps/development_timeline_support.rs` (modified — the
          `clear_record` fixture helper: the change exists and the record holds
          nothing, so "no recorded activity" is the record's own state and not
          an accidental filter).
  - [x] **REFACTOR** — let the empty state be one branch of the report root, not
        a special-cased template; re-run → still exit 0. Paste it.
        ```
        The empty state is now visibly one branch of the one report root and
        nothing else: `render_view` derives the scope's line once, writes it,
        and then either appends `render_empty_state` — the same markup for every
        scope that records nothing — or the summary and the timeline. The empty
        state no longer re-derives the scope line from the model (the second
        `scope_line` call is gone; the function takes the root's already-derived
        line), so a scope can still be named in exactly one way, and no
        template, no second root, and no per-scope special case exists. The
        model side keeps the matching invariant: `bands_for` yields no rows for
        an empty window, so `empty`, the absent rows, and the absent axis are one
        fact stated in three places at most once each. No behaviour change: the
        same markup, the same text, the same numbers.

        $ cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.47s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report shows the empty state for a scope with no recorded activity"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: The report shows the empty state for a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report shows the empty state for "demo"
           ✔  And the report draws no timeline axis for it
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0

        Regression runs of the seven committed report scenarios whose shared
        model and renderer this revision touched (the `empty` marker, the
        `bands_for` invariant, and the report root's branch):

        $ ... --exact "...::The report ranks the longest single step first"                → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report totals the time spent waiting for questions"        → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report totals token usage per turn and per subagent"       → 1 passed, 5 steps  EXIT=0
        $ ... --exact "...::The report of an ideation topic names the work done before the change exists" → 1 passed, 4 steps  EXIT=0
        $ ... --exact "...::The report orders the timeline by recorded time, not by store position" → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report names the agent's fan-out into small subagents"     → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report needs no network access to render"                  → 1 passed, 3 steps  EXIT=0
        ```
  - [x] **COMMIT** — `77f7f73bc00516dd0f08dd8a1a1132ef68ad84e4` — `feat(development-report): <release note>` with
        the scenario title `The report shows the empty state for a scope with no
        recorded activity` in the body. Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — … opens in a browser.

        The report shows the empty state for a scope with no recorded activity"
        [16:57:13] committed revision on the Git backend
        EXIT=0

        $ git show --stat --format='%H' 77f7f73b
        77f7f73bc00516dd0f08dd8a1a1132ef68ad84e4

         assets/timeline/report.html                        |  14 +++
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 131 +++++++++++++++++++--
         src/cli/timeline.rs                                |  42 ++++++-
         tests/steps/development_report_steps.rs            | 116 ++++++++++++++++++
         tests/steps/development_timeline_support.rs        |  14 +++
         6 files changed, 303 insertions(+), 16 deletions(-)

        Revision 77f7f73b touches production source (`src/cli/timeline.rs` — the
        `empty` marker, the `bands_for` invariant, the report root's branch, and
        `render_empty_state`; `assets/timeline/report.html` — the empty-state
        styles), the scenario's test steps and fixture helper, the delta
        `.feature`'s `@wip` removal, and this task file's evidence — never the
        spec alone.
        ```

## Task 33 — `A dense record is reported as aggregates instead of raw ticks` (`development-report`, added)

- [x] **Scenario: `A dense record is reported as aggregates instead of raw ticks`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::A dense record is reported as aggregates instead of raw ticks"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::A dense record is reported as aggregates instead of raw ticks"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: A dense record is reported as aggregates instead of raw ticks
           ✘  Given a change "demo" whose record holds hundreds of short agent activities
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:59:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (2621424) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101

        RED category: undefined step — the dense-record Given matches no
        function (all three Then steps are unbound too). The contract did not
        hold before this revision: the report drew one `<g class="span">` per
        entry, so a record of hundreds of short activities produced hundreds of
        2 px marks — an unreadable axis, and exactly the failure this scenario
        exists to prevent.
        ```
  - [x] **GREEN** — implement the bucket aggregation and the aggregate marker.
        Files: `src/cli/timeline.rs`, `assets/timeline/report.html`,
        `tests/steps/development_report_steps.rs`. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.29s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::A dense record is reported as aggregates instead of raw ticks"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 16.80s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: A dense record is reported as aggregates instead of raw ticks
           ✔  Given a change "demo" whose record holds hundreds of short agent activities
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report collapses the short agent activities into aggregate markers
           ✔  And each aggregate marker names how many entries it covers and their total duration
           ✔  And the report summary still counts every recorded entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Production files created/modified by this GREEN step:
        - `src/cli/timeline.rs` (modified — `Aggregate` (the band's scope, the
          bucket's own bounds, the covered entry ids, their combined measured
          duration) and `ReportModel::aggregates`; the one bucketing
          `aggregates_for` plus its threshold `collapses` ("the entry's drawn
          extent is shorter than the time one pixel of the window spends on"),
          read from the one place that derives it, `TimeScale::time_per_pixel`;
          `DRAWING_WIDTH` and `AGGREGATE_BUCKET_PX` (16 units, so the initial
          zoom draws at most the drawing width divided by 16 markers per band —
          a bounded number of shapes for a record of any size);
          `TimeScale::bucket` (a marker's bucket tiled onto the axis, never
          narrower than the minimum mark); `render_band` drawing each entry as
          its own span or as the marker standing for it, once per bucket; and
          `render_aggregate`, the marker's markup — `data-count`,
          `data-duration-ms`, `data-bucket-start-ms`/`data-bucket-end-ms` (the
          zoom path's own inputs), an accessible name, and no `data-entry-id`
          (a marker never pretends to be one entry's span));
        - `assets/timeline/report.html` (modified — the marker's style: the
          band's own hue at .8 opacity and the muted label, in the tokens the
          design system declares);
        - `tests/steps/development_report_steps.rs` (modified — the dense
          fixture (240 activities, 4 s apart, 250 ms each, seeded through the
          builder), the drawn-marker reader, and all three Then steps);
        - `src/cli/timeline.rs`'s summary path is deliberately untouched: the
          summary and the longest-step ranking still read the whole record, so
          the aggregation can only ever change what is *drawn*, never what is
          counted.

        The fixture's numbers: 240 activities over 956 s → the window's
        time-per-pixel is 1 160 ms, every activity is 250 ms (far below it), the
        bucket is 18 560 ms, and the report draws 52 markers standing for the
        240 entries (1 marker covering 3 entries, 18 covering 4, 33 covering 5);
        the entries tile still reads 240. Checked independently of the scenario
        against a scratch project root holding the same 240 activities:
        `grep -c '<g class="aggregate"' report.html` → 52,
        `grep -c '<g class="span"' report.html` → 0, and the markers'
        `data-count` attributes sum to 240.
        ```
  - [x] **REFACTOR** — one bucketing function used by the initial zoom and by
        the replacement-on-zoom path; re-run → still exit 0. Paste it.
        ```
        There is exactly one bucketing and both the initial zoom and the
        replacement path read it: `aggregates_for` computes the markers once in
        `build_report_model`, they travel in the model (`ReportModel::aggregates`
        — scope, bucket bounds, covered ids, combined duration), and
        `render_band` draws whatever the model carries. Nothing re-buckets at
        render time, and the future zoom path has the marker — its bucket, its
        covered ids, its duration — already in hand, so as the zoom narrows an
        individual span can only replace the aggregate that already stands for
        it, never a second, differently-bucketed grouping. The row's pass was
        tightened too: the band's markers are indexed by the entries they cover
        once per band (`BTreeMap<u64, &Aggregate>`), instead of searching the
        window's markers for every entry, so the drawing pass is one lookup per
        entry. No behaviour change: the same markers, the same shapes, the same
        numbers.

        $ cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.40s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::A dense record is reported as aggregates instead of raw ticks"
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: A dense record is reported as aggregates instead of raw ticks
           ✔  Given a change "demo" whose record holds hundreds of short agent activities
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report collapses the short agent activities into aggregate markers
           ✔  And each aggregate marker names how many entries it covers and their total duration
           ✔  And the report summary still counts every recorded entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        Regression runs — the eight committed report scenarios whose shared
        model, renderer, and drawing pass this revision touched:

        $ ... --exact "...::The report ranks the longest single step first"                → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report totals the time spent waiting for questions"        → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report totals token usage per turn and per subagent"       → 1 passed, 5 steps  EXIT=0
        $ ... --exact "...::The report of an ideation topic names the work done before the change exists" → 1 passed, 4 steps  EXIT=0
        $ ... --exact "...::The report orders the timeline by recorded time, not by store position" → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report names the agent's fan-out into small subagents"     → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report needs no network access to render"                  → 1 passed, 3 steps  EXIT=0
        $ ... --exact "...::The report shows the empty state for a scope with no recorded activity" → 1 passed, 4 steps  EXIT=0
        ```
  - [x] **COMMIT** — `24f5525166c1a5c976acb6af726f2f56d1ebee49` — `feat(development-report): <release note>` with
        the scenario title `A dense record is reported as aggregates instead of
        raw ticks` in the body. Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "feat(development-report): Givn records the development process of every change and ideation — … opens in a browser.

        A dense record is reported as aggregates instead of raw ticks"
        [17:01:25] committed revision on the Git backend
        EXIT=0

        $ git show --stat --format='%H' 24f55251
        24f5525166c1a5c976acb6af726f2f56d1ebee49

         assets/timeline/report.html                        |   5 +
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 149 +++++++++-
         src/cli/timeline.rs                                | 194 ++++++++++++-
         tests/steps/development_report_steps.rs            | 305 +++++++++++++++++++++
         5 files changed, 640 insertions(+), 15 deletions(-)

        Revision 24f55251 touches production source (`src/cli/timeline.rs` — the
        `Aggregate` model, the one bucketing, the threshold read from
        `TimeScale::time_per_pixel`, and the marker markup;
        `assets/timeline/report.html` — the marker's style), the scenario's test
        steps and dense fixture, the delta `.feature`'s `@wip` removal, and this
        task file's evidence — never the spec alone.
        ```

## Task E1 — E2E setup: local environment, browser tier, strictness, tags, evidence

- [x] **E2E setup task (no scenario is written until every box here is filled in)**
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
      change in scope that is 46 vs 8 units, so the e2e count is strictly less
      than the full count. Prove it with the runner's own listing (or the
      summary line of each run) and paste both counts:
      `GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e`
      → 46 scenarios, `GIVN_FEATURES=givn/changes/development-timeline/specs
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
      Playwright-bundled Chromium download is the new prerequisite. The same
      step also provisions the **pinned `maki` v0.6.3 binary**
      (`maki-v0.6.3-x86_64-unknown-linux-musl.tar.gz` from
      `https://github.com/tontinton/maki/releases/download/v0.6.3/`,
      checksum-verified, extracted into a local bin dir), because Task 24a's
      `@e2e` row runs the real harness and must never skip. The **browser-setup
      edits** (the `./setup-e2e-browser.sh` step and the `--non-e2e`/`--e2e`
      split, whose setup commit E1 already landed) stay with the e2e setup; the
      Maki provisioning script entry and its CI/release lines belong to **Task
      24a's own scenario commit**, whose GREEN/COMMIT fields are their single
      home.
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
  - [x] **Sub-task 1 — the local environment and the browser toolchain.**
        Create `setup-e2e-browser.sh` and `tests/e2e/package.json` (Playwright
        pinned to 1.64.1), run the script, and paste its output plus
        `npx --prefix tests/e2e playwright --version`.
        ```
        $ cat setup-e2e-browser.sh
        #!/bin/sh
        # One-time setup for the @e2e browser tier: install the Playwright pinned in
        # tests/e2e/package.json and the Chromium that Playwright bundles.
        #
        # This is the only step of the change that touches the network, and it runs
        # once, outside the suite. The tier itself needs no server, no port, no
        # database, and no external service: the report under test is one
        # self-contained file and the record is a local file, so the browser loads the
        # report from a file:// URL inside the temporary project root a scenario
        # created. Node.js is resolved from PATH (the design does not pin it); the
        # browser under test is always this Chromium, never a system browser.
        set -eu

        cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"

        npm ci --prefix tests/e2e
        npx --prefix tests/e2e playwright install chromium

        $ cat tests/e2e/package.json
        {
          "name": "givn-e2e-browser",
          "version": "0.0.0",
          "private": true,
          "description": "The @e2e browser tier's driver dependency: Playwright, pinned exactly as the design names it. `./setup-e2e-browser.sh` installs this package and the Chromium the pinned Playwright bundles; the tier itself runs offline against file:// URLs.",
          "type": "module",
          "dependencies": {
            "playwright": "1.64.1"
          }
        }

        $ npm install --package-lock-only --prefix tests/e2e
        up to date in 1s
        $ grep -n '"playwright"' tests/e2e/package-lock.json | head -2
        11:        "playwright": "1.64.1"
        23:        "playwright": "cli.js"

        First run (the actual download; the script is idempotent, the second run
        is the cached one below):
        $ ./setup-e2e-browser.sh
        ... FFmpeg (playwright ffmpeg v1013) downloaded to /home/buster/.cache/ms-playwright/ffmpeg-1013
        ... Downloading Chrome Headless Shell 156.0.8078.4 (playwright chromium-headless-shell v1248) from https://cdn.playwright.dev/builds/cft/156.0.8078.4/linux64/chrome-headless-shell-linux64.zip
        ... Chrome Headless Shell 156.0.8078.4 (playwright chromium-headless-shell v1248) downloaded to /home/buster/.cache/ms-playwright/chromium_headless_shell-1248
        SETUP_EXIT=0

        Second run, everything already installed:
        $ ./setup-e2e-browser.sh
        added 2 packages in 306ms
        SETUP_EXIT=0

        $ npx --prefix tests/e2e playwright --version
        Version 1.64.1
        EXIT=0

        node --version → v22.20.0 (the design's own resolved version; Node is
        resolved from PATH, never pinned). The script's dependency is the only
        new runtime dependency of this change, and it is exactly the pinned
        Playwright: `tests/e2e/package.json` + `package-lock.json` (committed),
        `tests/e2e/node_modules` (ignored by `tests/e2e/.gitignore` — E1's own
        directory — and recreated by the script). The browser download lands in
        `~/.cache/ms-playwright`, outside the repository.
        ```
  - [x] **Sub-task 2 — the driver and the Rust browser helper.** Create
        `tests/e2e/browser-driver.mjs` and `tests/common/browser.rs` with the
        contracts above (one context per scenario, `file://` navigation,
        reduced-motion emulation, deterministic capture, failure-never-skip
        error contract).
        ```
        `tests/e2e/browser-driver.mjs` (new, 175 lines): `chromium.launch()` +
        ONE `newContext({ reducedMotion: "reduce", viewport: {width: 1280,
        height: 900} })` + one page, started lazily on the first command; the
        commands `goto` (`target.goto(url, {waitUntil: "load"})`),
        `viewport`, `click`, `press` (focuses the selector first when one is
        named), `text`, `count`, `attribute`, `evaluate`, `screenshot`
        (`{ path, animations: "disabled" }`), `quit`; `settled()` awaits
        `document.fonts.ready` and one `requestIdleCallback` tick before every
        read/capture; requests are handled strictly one at a time in arrival
        order (`chain = chain.then(...)`), so the page's state (zoom, open
        panel) survives across a scenario's steps; replies are
        `{"id":n,"ok":true,"value":…}` / `{"id":n,"ok":false,"error":…}`;
        stdin EOF and `quit` both close the browser and exit 0.

        `tests/common/browser.rs` (new, 330 lines): `BrowserDriver::start()`
        spawns `node tests/e2e/browser-driver.mjs` from the repository root with
        a reader thread feeding an `mpsc` channel; `command()` matches the
        reply's `id`, panics on `ok: false` ("the browser driver failed
        `cmd`: …"), on a 60 s silence ("did not answer within 60s"), and on a
        driver that exited first — a **failed** step in every case, never a
        skip; typed helpers (`goto`, `viewport`, `click`, `press`, `text`,
        `count`, `attribute`, `evaluate`, `screenshot`, `quit`);
        `capture_evidence`/`capture_state_evidence` write the contract's two
        viewports through `evidence_file`/`state_evidence_file`, which lay out
        `givn/changes/development-timeline/evidence/visual/<scenario_slug>/…`
        with `givn::integrity_hook::scenario_slug` (the gate's own rule, never a
        second convention); `file_url()` canonicalizes the report into a
        `file://` URL; `Drop` kills the child, so no browser survives a failure.

        Driver-level smoke (the documented failure contract included):
        $ printf '%s\n' '{"id":1,"command":"goto","url":"file:///tmp/e2e-smoke/page.html"}' \
            '{"id":2,"command":"text","selector":"#report-title"}' \
            '{"id":3,"command":"count","selector":".note"}' \
            '{"id":4,"command":"count","selector":"svg"}' \
            '{"id":5,"command":"viewport","width":360,"height":780}' \
            '{"id":6,"command":"screenshot","path":"/tmp/e2e-smoke/mobile.png"}' \
            '{"id":7,"command":"viewport","width":1920,"height":1080}' \
            '{"id":8,"command":"screenshot","path":"/tmp/e2e-smoke/desktop.png"}' \
            '{"id":9,"command":"quit"}' | node tests/e2e/browser-driver.mjs
        {"id":1,"ok":true,"value":null}
        {"id":2,"ok":true,"value":"Smoke page"}
        {"id":3,"ok":true,"value":1}
        {"id":4,"ok":true,"value":0}
        {"id":5,"ok":true,"value":null}
        {"id":6,"ok":true,"value":null}
        {"id":7,"ok":true,"value":null}
        {"id":8,"ok":true,"value":null}
        {"id":9,"ok":true,"value":null}
        EXIT=0
        -rw-r--r-- 13659 /tmp/e2e-smoke/desktop.png   -rw-r--r-- 6997 /tmp/e2e-smoke/mobile.png

        $ printf '%s\n' '{"id":1,"command":"goto",…}' '{"id":2,"command":"text","selector":"#missing"}' '{"id":3,"command":"nope"}' | node tests/e2e/browser-driver.mjs
        {"id":1,"ok":true,"value":null}
        {"id":2,"ok":false,"error":"text: page.innerText: Timeout 10000ms exceeded. …waiting for locator('#missing')"}
        {"id":3,"ok":false,"error":"unknown command: nope"}
        EOF_EXIT=0        # stdin EOF closes the browser and exits cleanly

        Rust-level smoke (Task E1's temporary probe, removed again in sub-task 4
        — `GIVN_FEATURES=givn/changes/development-timeline/probe-e2e`, one
        scenario over a trivial `file://` page in a fixture root, driven through
        `BrowserDriver`):
        $ GIVN_FEATURES=givn/changes/development-timeline/probe-e2e cargo test --locked --test features_runner -- --exact "probe.feature::The e2e tier's browser helper drives a real page"
        Feature: E2E strictness probe
          Scenario: The e2e tier's browser helper drives a real page
           ✔  Given the e2e browser smoke page
        [Summary]
        1 feature
        1 scenario (1 passed)
        1 step (1 passed)
        SMOKE_EXIT=0
        ```
  - [x] **Sub-task 3 — the e2e step files and the harness drivers.** Create the
        three `*_e2e_steps.rs` files, register them, wire the `GivnWorld` slots,
        and extend `tests/steps/development_timeline_support.rs` with the three
        harness drivers.
        ```
        Created (one file per capability, separate from the unit/integration
        files — no shared e2e file):
        - tests/steps/development_record_e2e_steps.rs — module doc +
          `DevelopmentRecordE2eState { entries: Vec<Value> }` (the parsed
          machine-readable record Task E3 reads);
        - tests/steps/agent_activity_reporting_e2e_steps.rs — module doc +
          `AgentActivityReportingE2eState { reporter_output: Option<Output> }`
          (Task E4);
        - tests/steps/development_report_e2e_steps.rs — module doc +
          `DevelopmentReportE2eState { report_path: Option<PathBuf>, browser:
          Option<BrowserDriver> }` (Tasks E5–E9; dropping the state closes the
          browser).

        Registrations (tests/steps/mod.rs):
          line 15: pub mod agent_activity_reporting_e2e_steps;
          line 28: pub mod development_record_e2e_steps;
          line 30: pub mod development_report_e2e_steps;
          line 13: pub mod browser;                   (tests/common/mod.rs)

        GivnWorld slots (the same `Option<…State>` pattern, one per e2e
        capability, each with an `#[allow(dead_code)]` naming the task that
        first reads it — the S3 precedent, because nothing reads a slot before
        its first scenario does):
          pub(crate) development_record_e2e: Option<development_record_e2e_steps::DevelopmentRecordE2eState>,
          pub(crate) agent_activity_reporting_e2e: Option<agent_activity_reporting_e2e_steps::AgentActivityReportingE2eState>,
          pub(crate) development_report_e2e: Option<development_report_e2e_steps::DevelopmentReportE2eState>,

        Harness drivers (tests/steps/development_timeline_support.rs): one
        shared vocabulary `HarnessActivity` (`Tool`, `Question`, `Subagent`,
        `Turn`, `Shutdown`) consumed by both node drivers, and
        `drive_omp_reporter` / `drive_opencode_reporter`, which write their
        driver script into the fixture root and run it through `node` with the
        project root as cwd and `PATH` carrying the `givn` shim
        (`shim_givn_on_path`, extracted from `report_harness_hook` so the Claude
        shim and both new drivers share one shim writer). The omp driver calls
        the generated reporter's factory with a fake `pi` (`on`, `events.on`,
        `exec` via `spawnSync`, `getThinkingLevel`) and fires
        tool/question/subagent/turn/shutdown at its own harness events; the
        opencode driver calls `setup(ctx)` with a fake context capturing
        `session.hook`/`tool.hook`/`shell.hook`/`permission.hook` plus an
        `event.subscribe` async iterable the driver feeds the harness's own
        event shapes into. The third driver is the already-existing Claude
        invocation (`report_harness_hook`: the generated
        `.claude/hooks/givn-timeline.sh` with a real payload on stdin,
        `PATH`-shimmed `givn`, fixture transcript) — reused, never copied.
        These items carry `#[allow(dead_code)]` naming Task E4, their first
        caller: their own verification is Task E4's scenario (Task E1 builds
        and compiles them, per this sub-task's own evidence line), and Task E4
        removes the allowances.

        $ cargo build --locked --tests
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.54s
        BUILD=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.74s
        CLIPPY=0
        $ cargo fmt --all -- --check        FMT=0
        ```
  - [x] **Sub-task 4 — the e2e strictness proof.** Temporary
        `unimplemented!("TODO: development-timeline")` binding in the e2e tier,
        run scoped, paste the non-zero exit, remove the probe, re-run the delta
        listing to show the e2e unit count is unchanged.
        ```
        Probe directory: `givn/changes/development-timeline/probe-e2e/`
        (outside `specs/`, so no delta count, lint finding, or permanent-corpus
        scenario changes).

        Probe A — an undefined step fails the runner:
        $ GIVN_FEATURES=givn/changes/development-timeline/probe-e2e cargo test --locked --test features_runner -- --exact "probe.feature::An e2e probe step that has no binding fails the runner"
        Feature: E2E strictness probe
          Scenario: An e2e probe step that has no binding fails the runner
           ✘  Given an e2e probe step that has no binding
              Step failed:
              Defined: givn/changes/development-timeline/probe-e2e/probe.feature:11:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed, to rerun pass `--test features_runner`
        PROBE_A_EXIT=101

        Probe B — the same step bound in the e2e tier with exactly
        `unimplemented!("TODO: development-timeline")`
        (tests/steps/development_report_e2e_steps.rs):
        $ GIVN_FEATURES=givn/changes/development-timeline/probe-e2e cargo test --locked --test features_runner -- --exact "probe.feature::An e2e probe step that has no binding fails the runner"
        Feature: E2E strictness probe
          Scenario: An e2e probe step that has no binding fails the runner
           ✘  Given an e2e probe step that has no binding
              Step failed:
              Defined: givn/changes/development-timeline/probe-e2e/probe.feature:11:5
              Matched: tests/steps/development_report_e2e_steps.rs:43:1
              Step panicked. Captured output: not implemented: TODO: development-timeline
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed, to rerun pass `--test features_runner`
        PROBE_B_EXIT=101

        Removal (probe directory and both probe bindings deleted; the step files
        back to their skeletons) and the unchanged delta listing:
        $ rm -rf givn/changes/development-timeline/probe-e2e
        $ ls givn/changes/development-timeline/
        arc42.md  design-review.md  design.md  proposal.md  specs  tasks.md
        $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='@e2e' cargo nextest list --locked --test features_runner | grep -c 'givn::features_runner'
        8
        $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='not @e2e' cargo nextest list --locked --test features_runner | grep -c 'givn::features_runner'
        35
        $ GIVN_FEATURES=givn/changes/development-timeline/specs GIVN_CUCUMBER_TAGS='' cargo nextest list --locked --test features_runner | grep -c 'probe'
        0

        The e2e tier is strict: an undefined step and an `unimplemented!()` step
        each fail the run (exit 101), and no probe title survives anywhere. A
        browser step therefore cannot degrade into a skip — the Rust helper's own
        error contract (non-zero exit, timeout, `{"ok": false, …}` → failed step)
        is the other half, proven in sub-task 2 above.
        ```
  - [x] **Sub-task 5 — `verify.e2e_command` and the scenario-count proof.**
        Confirm `givn/commands.yaml` declares `verify.command:
        ./run-tests.sh --non-e2e` (Task S2) and `verify.e2e_command:
        ./run-tests.sh --e2e`; paste both scope runs' counts.
        ```
        $ cat givn/commands.yaml | head -3
        # Project-owned command scripts. The README documents these same commands.
        verify:
          command: "./run-tests.sh --non-e2e"
          e2e_command: "./run-tests.sh --e2e"
        (identical strings are never accepted: the two commands differ, and the
        two tag sets below prove the runtime difference too.)

        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
             Summary [   3.626s] 35 tests run: 35 passed, 8 skipped
        NON_E2E_EXIT=0                                    # verify.command → 35 scenarios

        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e
             FAIL [   0.597s] (4/8) givn::features_runner …Consumer opens the generated development report in a browser
             FAIL [   0.633s] (5/8) givn::features_runner …Consumer generates the development report for a change
             FAIL [   0.696s] (6/8) givn::features_runner …Consumer reads the development record as machine-readable output
             FAIL [   0.702s] (7/8) givn::features_runner …Consumer runs a lifecycle step and reads its recorded actions
             FAIL [   0.711s] (8/8) givn::features_runner …Consumer installs the agent reporter and its reported activity lands in the record
        error: test run failed
        E2E_EXIT=100                                      # verify.e2e_command → 8 scenarios

        The e2e scope is **8** scenarios against the regular scope's **35**: the
        filter is real and the e2e count is strictly less (the 8 `@e2e`
        scenarios are Tasks E2–E9, whose steps arrive with those tasks; their
        RED is the undefined-step failure this run already shows). The listing
        proof agrees: `@e2e` → 8 units, `not @e2e` → 35 units, 43 in total.
        ```
  - [x] **Sub-task 6 — the local health check.** Run the driver against a
        trivial `file://` page (or a freshly generated report) inside a temp
        project root and paste the exit-0 smoke result.
        ```
        The toolchain reports the pinned version:
        $ npx --prefix tests/e2e playwright --version
        Version 1.64.1

        And the driver launches Chromium over a real generated report. The
        project root `/tmp/e2e-health` is a `givn init` root whose change
        `demo` ran a lifecycle command, so `givn timeline` wrote a report with
        a rendered timeline — one file, no server, no port:
        $ cd /tmp/e2e-health && givn init --agents omp && givn new demo && givn status --change demo && givn timeline --change demo --out report.html
        report.html
        $ printf '%s\n' '{"id":1,"command":"goto","url":"file:///tmp/e2e-health/report.html"}' \
            '{"id":2,"command":"text","selector":"#report-title"}' \
            '{"id":3,"command":"count","selector":"svg.timeline"}' \
            '{"id":4,"command":"text","selector":"#report-scope"}' \
            '{"id":5,"command":"quit"}' | node tests/e2e/browser-driver.mjs
        {"id":1,"ok":true,"value":null}
        {"id":2,"ok":true,"value":"Development report"}
        {"id":3,"ok":true,"value":1}
        {"id":4,"ok":true,"value":"Change: demo"}
        {"id":5,"ok":true,"value":null}
        HEALTH_EXIT=0

        The browser really rendered the report: the page's heading, exactly one
        timeline `svg`, and the rendered scope line all come from the DOM, with
        the `file://` URL the browser scenarios will use.
        ```
  - [x] **Sub-task 7 — CI/release browser prerequisites.** Add
        `./setup-e2e-browser.sh` before the first suite invocation in
        `.github/workflows/ci.yml` and `.github/workflows/release.yml`, and make
        the regular workflow step `./run-tests.sh --non-e2e` before
        `./run-tests.sh --e2e`.
        ```
        $ git diff .github/workflows/ci.yml
        @@ -70,10 +70,14 @@ jobs:
               - name: Install pinned git-cliff for the changelog scenarios
                 shell: bash
                 run: |
                   set -euo pipefail
                   if ! git-cliff --version 2>/dev/null | grep -Fq 'git-cliff 2.13.1'; then
                     cargo install git-cliff --version 2.13.1 --locked
                   fi

        +      - name: Install the pinned Playwright browser
        +        run: ./setup-e2e-browser.sh
        +
               - name: Run repository acceptance suites
                 run: |
                   set -euo pipefail
        -          ./run-tests.sh
        +          ./run-tests.sh --non-e2e
                   ./run-tests.sh --e2e

        $ git diff .github/workflows/release.yml
        @@ -325,10 +325,14 @@ jobs:
                   cargo clippy --locked --all-targets -- -D warnings
                   cargo nextest run --locked --lib --no-tests=warn

        +      - name: Install the pinned Playwright browser
        +        shell: bash
        +        run: ./setup-e2e-browser.sh
        +
               - name: Run repository acceptance suites
                 shell: bash
                 run: |
                   set -euo pipefail
        -          ./run-tests.sh
        +          ./run-tests.sh --non-e2e
                   ./run-tests.sh --e2e

        Node is already on both runners (the design states it), so the only new
        prerequisite is the Playwright-bundled Chromium the setup script
        downloads. The regular step now runs the two declared scopes back to
        back: the bare `./run-tests.sh` would run the whole suite including
        `@e2e` and then `./run-tests.sh --e2e` again, running the browser tier
        twice once the regular scope narrows. Both workflow edits ride in this
        setup commit, never in a scenario commit.
        ```
  - [x] **COMMIT** — `1f03e66ed7db83f8082a0427e2b147129bf61e77` — `test(e2e): <release note>` with the subject
        `E2E setup: browser tier, strictness, tag scopes, evidence` — the e2e
        setup lands as its own commit (never folded into a scenario), before the
        first `@e2e` scenario's RED. Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        E2E setup: browser tier, strictness, tag scopes, evidence"
        [17:11:37] committed revision on the Git backend
        EXIT=0

        $ git show --stat --format='%H' 1f03e66e
        1f03e66ed7db83f8082a0427e2b147129bf61e77

         .github/workflows/ci.yml                          |   5 +-
         .github/workflows/release.yml                     |   6 +-
         givn/changes/development-timeline/tasks.md        | 360 +++++++++++++++++++++-
         setup-e2e-browser.sh                              |  17 +
         tests/common/browser.rs                           | 301 ++++++++++++++++++
         tests/common/mod.rs                               |   1 +
         tests/e2e/.gitignore                              |   4 +
         tests/e2e/browser-driver.mjs                      | 165 ++++++++++
         tests/e2e/package-lock.json                       |  42 +++
         tests/e2e/package.json                            |  10 +
         tests/steps/agent_activity_reporting_e2e_steps.rs |  21 ++
         tests/steps/development_record_e2e_steps.rs       |  22 ++
         tests/steps/development_report_e2e_steps.rs       |  27 ++
         tests/steps/development_timeline_support.rs       | 352 ++++++++++++++++++++-
         14 files changed, 1326 insertions(+), 33 deletions(-)

        The commit carries the setup only: the browser toolchain and its pinned
        dependency, the driver and its Rust helper, the three per-capability
        `@e2e` step-file skeletons with their registrations and `GivnWorld`
        slots, the four harness drivers, the CI/release prerequisites, and this
        task file's evidence. No `@e2e` tag was touched, and
        `tests/e2e/node_modules` is never committed (`tests/e2e/.gitignore`).
        ```

## Task E2 — `Consumer runs a lifecycle step and reads its recorded actions` (`@e2e`, `development-record`)

- [x] **Scenario: `Consumer runs a lifecycle step and reads its recorded actions`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer runs a lifecycle step and reads its recorded actions"`
        → must exit non-zero (the e2e steps are undefined). Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer runs a lifecycle step and reads its recorded actions"
        Feature: Development Record
          Scenario: Consumer runs a lifecycle step and reads its recorded actions
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  And the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✘  And the machine-readable record names the status command for "demo"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:108:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        6 steps (5 passed, 1 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: genuinely new behaviour — the scenario's two Then steps
        are undefined (the E2E tier is strict, proven in Task S1/E1 sub-task 4),
        so the run exits 101 and the scenario cannot pass silently. The five
        reused steps (root, change fixture, both `givn` invocations, exit code)
        already bind; only this scenario's own assertions are missing.
        ```
  - [x] **GREEN** — implement the e2e steps on the real CLI. Files:
        `tests/steps/development_record_e2e_steps.rs`,
        `src/cli/history.rs` (any read-path gap), plus the two evidence
        screenshots above. Compile first, re-run → exit 0; then run
        `./run-tests.sh --e2e` at least once for this scenario's scope health.
        Paste it.
        ```
        $ cargo build --locked
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.38s
        BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer runs a lifecycle step and reads its recorded actions"
        Feature: Development Record
          Scenario: Consumer runs a lifecycle step and reads its recorded actions
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  And the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✔  And the machine-readable record names the status command for "demo"
           ✔  And the machine-readable record carries a duration for that command
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        EXIT=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e
             PASS [   1.309s] (8/8) givn::features_runner inspect-development-history/development-record.feature::Consumer runs a lifecycle step and reads its recorded actions
        ────────────
             Summary [   1.309s] 8 tests run: 1 passed, 7 failed, 35 skipped
        E2E_EXIT=100

        Scope health at this scenario's commit point: this scenario is the one
        e2e scenario that passes; the other seven are Tasks E3–E9, whose steps
        are their own RED (undefined step), and this task's commit does not
        claim them.

        The two evidence screenshots (the change-wide visual gate):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-runs-a-lifecycle-step-and-reads-its-recorded-actions/
        total 80
        -rw-r--r-- 1 buster buster 46816 Oct 16 17:22 desktop.png
        -rw-r--r-- 1 buster buster 32172 Oct 16 17:22 mobile.png
        $ file givn/changes/development-timeline/evidence/visual/consumer-runs-a-lifecycle-step-and-reads-its-recorded-actions/*.png
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced

        Production files: **none for this scenario** — investigated, not
        assumed. The read view the scenario asserts on already exists and is
        the whole of the interface it drives: `givn history --change <id>
        --json` prints `serde_json::to_string(&entries)` of
        `src/cli/history.rs::run` (the `entries_in_scope` read path, filtered to
        one `Scope::Change`), and the recorder writes the `command` entry the
        assertion finds through `HistoryRecorder::finish`
        (`src/history.rs`), carrying the resolved scope, the bounded invocation
        label, the measured `duration_ms`, the outcome, and
        `data.exit_code`. The step does not need a read-path gap filled, so no
        production line was added.

        Step files (`tests/steps/development_record_e2e_steps.rs`, in this
        scenario's commit):
        - `Then the machine-readable record names the status command for "…"` —
          parses the read view's own stdout (the `When`'s `givn history
          --change demo --json`) into the capability's
          `DevelopmentRecordE2eState.entries` *once*, then requires a
          `command`/`status` entry whose `scope` is exactly
          `{"kind":"change","id":"demo"}` and whose bounded label names the
          invocation (`status --change demo`).
        - `Then the machine-readable record carries a duration for that command`
          — the same parse (never a second invocation, never a private store
          read) must carry `duration_ms` for that entry, within the command's
          own wall time.
        - `commit_evidence` — the change-wide visual gate: renders the report the
          scenario's own record produced (`givn timeline --change demo --out
          report.html`; the read view records nothing, so the record is
          untouched), opens that one `file://` document in the real browser
          (`tests/e2e/browser-driver.mjs` through `BrowserDriver`), and captures
          `mobile.png`/`desktop.png` at 360x780 and 1920x1080 with animations
          disabled — after the scenario's final assertion.
        ```
  - [x] **REFACTOR** — reuse the shared run-helper instead of a second
        invocation helper; re-run → still exit 0. Paste it.
        ```
        Cleanup, no behaviour change:
        - the report/evidence helper now takes the scenario's project root
          (`&common::TestEnv`) instead of a whole `&GivnWorld`, so it depends on
          exactly what it uses;
        - every subprocess in the file goes through the shared run-helper
          `common::run_givn` (the CLI's one test entry point) and every capture
          through `common::browser::BrowserDriver`/`capture_evidence` — no second
          invocation helper and no second capture path exists;
        - the browser tier is the scenario's only browser use, and the scenario
          itself stays CLI-only (the matrix's driving mechanism);
        - the E3 scenario's own step (`the output names every recorded entry …`)
          is held back to Task E3: this scenario's commit is atomic for this
          scenario, and that step is Task E3's RED→GREEN work.

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.17s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer runs a lifecycle step and reads its recorded actions"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.53s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Consumer runs a lifecycle step and reads its recorded actions
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the user runs `givn status --change demo`
           ✔  And the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✔  And the machine-readable record names the status command for "demo"
           ✔  And the machine-readable record carries a duration for that command
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        REFACTOR_EXIT=0

        The REFACTOR re-run refreshed both captures (17:22, same 32172/46816
        bytes at 360x780/1920x1080): the evidence is the final code's own
        output, not the GREEN iteration's leftover.
        ```
  - [x] **COMMIT** — `398b343e33eb04e559a209bdb6c43c6e20496435` — `test(e2e): <release note>` with the scenario
        title `Consumer runs a lifecycle step and reads its recorded actions` in
        the body (including the two evidence PNGs). Record the revision
        identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer runs a lifecycle step and reads its recorded actions"
        [17:22:38] committed revision on the Git backend
        COMMIT=0

        $ git log -1 --format='%B' 398b343e
        test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        Consumer runs a lifecycle step and reads its recorded actions

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

        $ git show --stat --format='%H' 398b343e
        398b343e33eb04e559a209bdb6c43c6e20496435
         .../desktop.png                                    | Bin 0 -> 47086 bytes
         .../mobile.png                                     | Bin 0 -> 32176 bytes
         .../desktop.png                                    | Bin 0 -> 46816 bytes
         .../mobile.png                                     | Bin 0 -> 32172 bytes
         .../development-record.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 148 ++++++++++++++++++++-
         tests/steps/development_record_e2e_steps.rs        | 115 +++++++++++++++-
         7 files changed, 256 insertions(+), 9 deletions(-)

        The revision carries this scenario's RED+GREEN+REFACTOR as one atomic
        commit: the `@wip` removal from this scenario alone, the two e2e Then
        steps with their helpers, this task file's evidence, and the evidence
        pair at both viewports. One note, stated rather than hidden: while this
        scenario's RED ran, a `./run-tests.sh --e2e` scope-health run also bound
        the *sibling* machine-readable scenario's then-existing step, so that
        run captured the sibling slug's evidence pair as well
        (`consumer-reads-the-development-record-as-machine-readable-output/`,
        47086/32176 bytes) and the untracked evidence directory went in with
        this commit. Task E3's own runs re-capture its pair and rewrite it in
        Task E3's revision, so the pair that stands after this change is E3's
        own final-state capture; nothing about this scenario's evidence or
        assertions depends on it.
        ```

## Task E3 — `Consumer reads the development record as machine-readable output` (`@e2e`, `development-record`)

- [x] **Scenario: `Consumer reads the development record as machine-readable output`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer reads the development record as machine-readable output"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer reads the development record as machine-readable output"
        Feature: Development Record
          Scenario: Consumer reads the development record as machine-readable output
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history
           ✔  When the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✘  And the output names every recorded entry with its kind, its scope, and its duration
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:117:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        5 steps (4 passed, 1 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined step — this scenario's own Then
        (`the output names every recorded entry with its kind, its scope, and its
        duration`) is unbound. The shared recorded-history Given really ran (real
        `givn new demo` + `givn status --change demo` against the fixture's init
        root) and the `When` is the real `givn history --change demo --json`
        read, so the scenario stops at its new assertion, not at a vacuous setup.
        The read path the assertion drives already exists (Task 9's read view),
        so the discrimination of both halves of the assertion is proven in the
        GREEN block below with controlled omissions in the isolated fixture
        (ORACLE_RED), restored immediately.
        ```
  - [x] **GREEN** — implement the JSON read output and the parsing steps. Files:
        `src/cli/history.rs`, `tests/steps/development_record_e2e_steps.rs`,
        plus the two evidence screenshots. Compile first, re-run → exit 0. Paste
        it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
        BUILD=0
        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.86s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer reads the development record as machine-readable output"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.61s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Consumer reads the development record as machine-readable output
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history
           ✔  When the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✔  And the output names every recorded entry with its kind, its scope, and its duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        GREEN_EXIT=0

        The two evidence screenshots (the change-wide visual gate; the report the
        scenario's own seeded record produced, rendered in the real browser):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-reads-the-development-record-as-machine-readable-output/
        total 80
        -rw-r--r-- 1 buster buster 47028 Oct 16 17:25 desktop.png
        -rw-r--r-- 1 buster buster 32173 Oct 16 17:25 mobile.png
        $ file givn/changes/development-timeline/evidence/visual/consumer-reads-the-development-record-as-machine-readable-output/*.png
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced

        Production files: **none for this scenario** — investigated, not
        assumed. The machine-readable view the scenario asserts on is the read
        view's JSON mode, and it already exists: `givn history --change <id>
        --json` prints `serde_json::to_string(&entries)` over
        `src/cli/history.rs::entries_in_scope` (the read is filtered to the
        named `Scope::Change`), and it was introduced by Task 9's revision
        `d41ab44644e3c547207b1f2da057d448d18b1855` (`git log -S 'Emit the record
        as JSON' -- src/main.rs` → `d41ab446`). The scenario's work is therefore
        its own assertion — the machine contract's completeness — and no
        production line was added.

        Step file (`tests/steps/development_record_e2e_steps.rs`):
        - `Then the output names every recorded entry with its kind, its scope,
          and its duration` — the primary assertion is the read view's own
          stdout (the `When`'s `givn history --change demo --json`, parsed once
          into `DevelopmentRecordE2eState.entries`): every printed entry carries
          a non-empty `kind`, a `scope` with a kind and an id, and a `duration_ms`
          number, and the whole read covers exactly one scope.
        - `assert_read_is_complete` — the secondary completeness check: the
          printed entries are compared with the record's own lines *of that same
          scope*, so the read cannot silently drop an entry (the design's "one
          entry per recorded line").
        - `commit_evidence` (shared with Task E2's scenario) — renders the report
          the scenario's own record produced (`givn timeline --change demo --out
          report.html`) and captures `mobile.png`/`desktop.png` at 360x780 and
          1920x1080 through the real browser, after the final assertion.

        Discrimination (ORACLE_RED) — the read path already exists, so both
        halves of the new assertion are proven load-bearing by controlled
        omissions in the isolated fixture (the scenario's own temp root), each
        restored immediately:

        (a) one recorded line of the read's scope made duration-less (a real
            `step` line appended to the fixture's store and read back through
            the real `givn history --change demo --json`), so the per-entry half
            must fail:
        ✔  Then the output names every recorded entry with its kind, its scope, and its duration
        ✘  Step panicked. Captured output: the printed step entry names no duration: {
             "at": "2026-10-16T09:00:00.000Z",
             "id": 9001,
             "kind": "step",
             "name": "probe",
             "outcome": "ok",
             "schema": 1,
             "scope": { "id": "demo", "kind": "change" },
             "source": "givn"
           }
        PROBE_A_EXIT=101

        (b) one more recorded line of the read's scope than the read printed (a
            `command` line appended after the read), so the completeness half
            must fail:
        ✘  Step panicked. Captured output: assertion `left == right` failed: the machine-readable read printed 1 of the scope's 2 recorded entries: [
        PROBE_B_EXIT=101

        Both probes were removed from the step immediately after their runs; the
        restored step was re-run and the scenario is green again (the
        `RESTORED_EXIT=0` run above is the restored tree).
        ```
  - [x] **REFACTOR** — one entry-to-JSON mapping shared by the read paths;
        re-run → still exit 0. Paste it.
        ```
        The read paths already share exactly one entry-to-JSON mapping, and it is
        the record's own type: `HistoryEntry`'s serde derive in `src/history.rs`.
        `givn history --json` prints `serde_json::to_string(&entries)` over that
        type (`src/cli/history.rs::run`), and `givn timeline` embeds the same
        `Vec<HistoryEntry>` inside its `ReportModel` (`src/cli/timeline.rs`,
        serialized and `<`-escaped by `render_report`), so there is no second,
        hand-rolled entry mapping to fold in:

        $ grep -rn '"duration_ms"\|"outcome"' src/ | grep -v '^src/history.rs'
        (no match)

        Every entry field reaches JSON through that one derive — no read path
        builds an entry object of its own — so no production change is needed for
        this scenario's refactor.

        Step-side consolidation (no behaviour change): the read's scope is
        derived once — `assert_entries_complete` returns it, and the completeness
        check takes it as an argument instead of re-deriving `entries[0]["scope"]`
        — and the temporary ORACLE_RED probe code is gone from the step.

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.81s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::Consumer reads the development record as machine-readable output"
           Compiling givn v0.9.0 (/home/buster/projects/givn)
            Finished `test` profile [unoptimized + debuginfo] target(s) in 4.12s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Record
          Scenario: Consumer reads the development record as machine-readable output
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history
           ✔  When the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✔  And the output names every recorded entry with its kind, its scope, and its duration
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        REFACTOR_EXIT=0

        This run also re-captured the scenario's evidence pair from the final
        code (`desktop.png` 46816 bytes / `mobile.png` 32172 bytes, 17:26,
        1920x1080 and 360x780), so the committed pair is this scenario's own
        output, not the earlier iteration's.
        ```
  - [x] **COMMIT** — `64a5dca3bebcfb9920181edcdb09304e2e87b29a` — `test(e2e): <release note>` with the scenario
        title `Consumer reads the development record as machine-readable output`
        in the body (including the two evidence PNGs). Record the revision
        identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer reads the development record as machine-readable output"
        [17:26:21] committed revision on the Git backend

        $ git show --stat --format='%H' 64a5dca3
        64a5dca3bebcfb9920181edcdb09304e2e87b29a
         .../desktop.png                    | Bin 47086 -> 46816 bytes
         .../mobile.png                     | Bin 32176 -> 32172 bytes
         .../development-record.feature     |   2 +-
         givn/changes/development-timeline/tasks.md         | 221 ++++++++++++++++++++-
         tests/steps/development_record_e2e_steps.rs        |  82 +++++++-
         5 files changed, 290 insertions(+), 15 deletions(-)

        The revision is this scenario's RED+GREEN+REFACTOR as one atomic commit:
        the `@wip` removal from this scenario alone, its Then step with the
        extracted assertion helpers, this task file's evidence, and its own
        re-captured evidence pair (the sibling slug's pair is untouched here —
        the pair this commit rewrites is
        `consumer-reads-the-development-record-as-machine-readable-output/`).
        ```

## Task E4 — `Consumer installs the agent reporter and its reported activity lands in the record` (`@e2e`, `agent-activity-reporting`, Scenario Outline, three Examples rows)

- [x] **Scenario: `Consumer installs the agent reporter and its reported activity lands in the record`** (Examples: `omp`, `opencode`, `claude`)
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record"
        Feature: Agent Activity Reporting
          Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
           ✔  Given a project root with givn/ initialised
           ✘  And the change agent target "omp" installs the development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:108:5
              Step doesn't match any function
          Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
           ✔  Given a project root with givn/ initialised
           ✘  And the change agent target "opencode" installs the development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:108:5
              Step doesn't match any function
          Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
           ✔  Given a project root with givn/ initialised
           ✘  And the change agent target "claude" installs the development reporter
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/agent-activity-reporting.feature:108:5
              Step doesn't match any function
        [Summary]
        1 feature
        3 scenarios (3 failed)
        6 steps (3 passed, 3 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined step — the outline's own Given, When, and Then
        are unbound (`the change agent target "…" installs the development
        reporter` matches no function), and all three Examples rows fail on it
        in the one process this unit runs. The reused first Given really built
        the fixture (a real `givn init --agents opencode` root), so the RED is
        the missing install/drive/assert path, never a vacuous setup: this
        scenario is the only caller that installs each target's reporter and
        then drives the generated asset, which is what the GREEN below adds.
        ```
  - [x] **GREEN** — implement the three driver paths and the assertions. Files:
        `tests/steps/agent_activity_reporting_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`,
        `src/cli/history.rs`/`src/skills/targets.rs` for any gap the real
        entry point exposes, plus the two evidence screenshots. Compile and
        `node --check` the drivers first, re-run → exit 0. Paste it.
        ```
        The two node driver scripts the support file writes into the fixture
        root, checked before they were driven (`node --check` needs a file; the
        scripts were extracted from their raw strings into /tmp):
        $ node --check /tmp/omp-driver.mjs        CHECK_OK
        $ node --check /tmp/opencode-driver.mjs   CHECK_OK

        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
        BUILD=0
        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.46s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record"
        Feature: Agent Activity Reporting
          Scenario Outline: Consumer installs the agent reporter and its reported activity lands in the record
           ✔  Given a project root with givn/ initialised
           ✔  And the change agent target "omp" installs the development reporter
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the installed "omp" reporter reports a completed tool call, a subagent, and a question wait
           ✔  And the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✔  And the machine-readable record names the tool call, the subagent, and the question wait
          Scenario Outline: … (opencode row)
           ✔  … the change agent target "opencode" installs the development reporter
           ✔  When the installed "opencode" reporter reports a completed tool call, a subagent, and a question wait
           ✔  Then the machine-readable record names the tool call, the subagent, and the question wait
          Scenario Outline: … (claude row)
           ✔  … the change agent target "claude" installs the development reporter
           ✔  When the installed "claude" reporter reports a completed tool call, a subagent, and a question wait
           ✔  Then the machine-readable record names the tool call, the subagent, and the question wait
        [Summary]
        1 feature
        3 scenarios (3 passed)
        21 steps (21 passed)
        GREEN_EXIT=0

        Scope health (`./run-tests.sh --e2e`, the `verify.e2e_command`):
             PASS [   2.574s] (7/8) …Consumer reads the development record as machine-readable output
             PASS [   5.322s] (8/8) …Consumer installs the agent reporter and its reported activity lands in the record
             Summary [   5.323s] 8 tests run: 3 passed, 5 failed, 35 skipped
        E2E_EXIT=100
        (the five failures are Tasks E5–E9, whose steps are still their own
        unbound RED; Tasks E2, E3, E4 pass.) The regular scope is untouched by
        this scenario's shared-file edits and stays green:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
             Summary [   3.976s] 35 tests run: 35 passed, 8 skipped
        NON_E2E_EXIT=0

        The evidence pair (one slug for the outline; the three Examples rows
        write the same two paths, so the committed pair is always a run's own
        output — here the claude row, the last one to write):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-installs-the-agent-reporter-and-its-reported-activity-lands-in-the-record/
        -rw-r--r-- 1 buster buster 39786 Oct 16 17:31 mobile.png
        -rw-r--r-- 1 buster buster 75066 Oct 16 17:31 desktop.png
        $ file …/*.png
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced

        The rendered report proves the record the reporter really delivered:
        `Entries 13`, `Waiting 40 ms (1 wait)`, `Subagents 8 ms (1 subagent)`,
        `Tokens 22,095`, `Tokens per model: claude-sonnet-5 22,095`, `Tokens per
        subagent: Architect.Scout unavailable` — the reported tool call, the
        subagent, the question wait, the turn, and the usage read from the
        session record, all at both viewports.

        Production files: **none for this scenario** — investigated, not
        assumed. The three layer the scenario exercises already exist:
        `givn init --agents <target>` writes each target's generated reporter
        (`src/skills/targets.rs`, whose integration table names the same three
        assets), every reporter delivers through the real intake
        (`givn history ingest` for omp and opencode, the generated Claude shim's
        `givn history harness-hook --harness claude` for the third —
        `src/cli/history.rs`), and the mapper's per-event kinds land as the
        entries the assertion reads (Tasks 12–24). No gap was exposed, so no
        production line was added.

        One install fact the fixture has to honour (and the step documents):
        `givn init --agents <target>` on an *already initialised* root keeps the
        existing `givn/config.yaml` and its recorded `skills.targets`, so the
        flag decides nothing there and no reporter is written; the Given
        therefore runs the same real command with `--force`, which is what makes
        init re-select the targets and install the reporter. Verified against
        the real binary before the step was written:
        $ givn init --agents opencode; givn init --agents omp; ls .omp/hooks/pre/
        ls: cannot access '.omp/hooks/pre/': No such file or directory
        $ givn init --force --agents omp; ls .omp/hooks/pre/
        givn-attribution.js  givn-timeline.js

        Files this scenario adds/changes (test infrastructure and the fixture,
        never production):
        - `tests/steps/agent_activity_reporting_e2e_steps.rs` — the outline's
          three steps: the install Given (`givn init --force --agents <target>`),
          the drive When (one driver entry point per harness, chosen by the
          Examples row), and the assertion Then, which reads the reporter run's
          own output and then requires the three entries in the machine-readable
          record from `givn history --change demo --json` — the tool call by
          name and label, the subagent by identity and agent definition, the
          question wait by the question it asked, each with a measured duration
          and the change's scope. `reported_activity()` is the one vocabulary
          list all three harnesses are driven with.
        - `tests/steps/development_timeline_support.rs` — the third harness
          driver, `drive_claude_reporter` (the generated shim executed once per
          harness event with a real payload on stdin, `PATH`-shimmed `givn`,
          and the session record written for a `Turn`, because Claude Code
          reports a turn's usage through its record and never inline), plus the
          opencode driver's own session-end handling (a `shutdown` activity
          emits the boundary the reporter flushes on, with the unconditional
          final boundary kept as the fallback) — and the Task-E4 `#[allow
          (dead_code)]` allowances on the vocabulary, the drivers, and their
          scripts removed, because this scenario is now their caller.
        - `tests/steps/mod.rs` — the two `#[allow(dead_code)]` allowances on the
          `development_record_e2e` and `agent_activity_reporting_e2e` world slots
          removed: Tasks E2/E3/E4 read them (`development_report_e2e` keeps its
          allowance for Tasks E5–E9).
        - the two evidence screenshots above.

        Discrimination (ORACLE_RED) — the reporters and the intake already
        exist, so the assertion's own discrimination is proven by a controlled
        omission in the isolated fixture (the driver's activity list), restored
        immediately:

        the reported spans omitted (only the session's turn boundary and end
        driven), so the record holds none of them and the assertion must fail:
        ✘  Then the machine-readable record names the tool call, the subagent, and the question wait
           Step panicked. Captured output: the machine-readable record holds no `tool` entry: [
        (the same failure on all three Examples rows)
        3 scenarios (3 failed)
        21 steps (18 passed, 3 failed)
        PROBE_EXIT=101

        The probe was removed and the restored tree re-run: `RESTORED_EXIT=0`
        (the GREEN run above is that restored run).
        ```
  - [x] **REFACTOR** — one driver entry point per harness, invoked through the
        same step helpers; re-run (all three rows) → still exit 0. Paste it.
        ```
        The property is now literal in the fixture: each harness has exactly one
        driver entry point in the shared support file, all three with the same
        signature, and the scenario reaches them through one table instead of a
        per-target branch of its own:

        $ grep -n "pub fn drive_" tests/steps/development_timeline_support.rs
        791:pub fn drive_omp_reporter(
        817:pub fn drive_opencode_reporter(
        849:pub fn drive_claude_reporter(

        $ grep -n "driver_for\|let output = driver" tests/steps/agent_activity_reporting_e2e_steps.rs
        136:fn driver_for(target: &str) -> fn(&TestEnv, &[HarnessActivity]) -> Output {
        138:        "omp" => support::drive_omp_reporter,
        139:        "opencode" => support::drive_opencode_reporter,
        140:        "claude" => support::drive_claude_reporter,
        193:    let driver = driver_for(&target);
        194:    let output = driver(env, &activities);

        The three drivers share the same vocabulary (`reported_activity()` is
        built once and handed to whichever one the row names) and the same
        failure discipline (a driver that cannot run fails the fixture, never
        the assertion). The reporter's own outcome moved out of the Then step
        into `assert_reporter_delivered`, so the step reads as its three parts:
        the reporter's run, the CLI's returned record, and the record's own
        three entries.

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.81s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record"
        Feature: Agent Activity Reporting
          Scenario Outline: … (omp row)
           ✔  Given a project root with givn/ initialised
           ✔  And the change agent target "omp" installs the development reporter
           ✔  And a change "demo" exists with artifact "proposal.md"
           ✔  When the installed "omp" reporter reports a completed tool call, a subagent, and a question wait
           ✔  And the user runs `givn history --change demo --json`
           ✔  Then the command exits 0
           ✔  And the machine-readable record names the tool call, the subagent, and the question wait
          Scenario Outline: … (opencode row)
           ✔  When the installed "opencode" reporter reports a completed tool call, a subagent, and a question wait
           ✔  Then the machine-readable record names the tool call, the subagent, and the question wait
          Scenario Outline: … (claude row)
           ✔  When the installed "claude" reporter reports a completed tool call, a subagent, and a question wait
           ✔  Then the machine-readable record names the tool call, the subagent, and the question wait
        [Summary]
        1 feature
        3 scenarios (3 passed)
        21 steps (21 passed)
        REFACTOR_EXIT=0

        All three rows re-ran in the one process this unit is, and the refactor
        refreshed the evidence pair from the final code (39786/75066 bytes,
        360x780 and 1920x1080).
        ```
  - [x] **COMMIT** — `0b33499477da79845601aed8cc55e042d2f5f4b8` — `test(e2e): <release note>` with the scenario
        title `Consumer installs the agent reporter and its reported activity
        lands in the record` in the body (including the two evidence PNGs).
        Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer installs the agent reporter and its reported activity lands in the record"
        [17:33:28] committed revision on the Git backend

        $ git show --stat --format='%H' 0b334994
        0b33499477da79845601aed8cc55e042d2f5f4b8
         .../desktop.png                                    | Bin 0 -> 73034 bytes
         .../mobile.png                                     | Bin 0 -> 39705 bytes
         .../agent-activity-reporting.feature               |   2 +-
         givn/changes/development-timeline/tasks.md         | 262 ++++++++++++++++++-
         tests/steps/agent_activity_reporting_e2e_steps.rs  | 278 ++++++++++++++++++++-
         tests/steps/development_timeline_support.rs        | 142 +++++++++--
         tests/steps/mod.rs                                 |   4 -
         7 files changed, 660 insertions(+), 28 deletions(-)

        The revision is this scenario's RED+GREEN+REFACTOR as one atomic
        commit: the `@wip` removal from the outline alone, its three steps with
        the driver table and the reporter-delivery helper, the shared support
        file's third driver (with the opencode driver's session-end handling and
        the Task-E4 dead-code allowances removed), the two world-slot allowances
        in `tests/steps/mod.rs`, this task file's evidence, and the outline's one
        evidence pair.

        One scope-run note, stated rather than hidden: `./run-tests.sh --e2e`
        regenerates *every* scenario's evidence pair (each capture is the
        report of that scenario's own record), so an e2e scope run during this
        task re-wrote Tasks E2 and E3's committed pairs too; this revision
        restores those two pairs to their own revisions' captures, so this
        scenario's commit carries only this scenario's evidence.
        ```

## Task E5 — `Consumer generates the development report for a change` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer generates the development report for a change`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer generates the development report for a change"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer generates the development report for a change"
            Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
             Running tests/features_runner.rs (target/debug/deps/features_runner-df7905ed6a10ceaf)
        Feature: Development Report
          Scenario: Consumer generates the development report for a change
           ✔  Given a project root with givn/ initialised
           ✘  And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:68:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)

        thread 'main' (2688926) panicked at …/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined step — the artifact-transition Given has no
        binding at all, and the reused root Given really built the fixture (a
        real `givn init --agents opencode` root), so the RED is the missing
        phase-derivation seed and assertion path, never a vacuous setup. The
        scenario is the only caller that seeds a record whose last recorded
        artifact transition is a named artifact and then reads the written
        report's phase placement.
        ```
  - [x] **GREEN** — implement the printed path, the phase derivation, and the
        default output path. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`, plus the two evidence
        screenshots. Compile first, re-run → exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.03s
        BUILD=0
        $ cargo build --locked --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.85s
        TESTS_BUILD=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer generates the development report for a change"
        Feature: Development Report
          Scenario: Consumer generates the development report for a change
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
           ✔  When the user runs `givn timeline --change demo --out report.html`
           ✔  Then the command exits 0
           ✔  And the command prints the path of the written report
           ✔  And the report file exists and names the recorded phases
           ✔  And the report file places the step recorded after that transition in the "implement" phase
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        GREEN_EXIT=0

        Evidence pair (both files non-empty, at the contract's two viewports;
        the capture opens the report this scenario's own `givn timeline` wrote,
        after the final assertion, animations disabled by the driver):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-generates-the-development-report-for-a-change/
        -rw-r--r-- 118181 desktop.png
        -rw-r--r--  49447 mobile.png
        $ file …/*.png
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced

        Two adjustments the real interface forced, both recorded rather than
        hidden. (1) The command prints the report path *as the caller named it*
        (`report.html`, not the absolute path) — the scenario's assertion
        resolves the printed path against its own fixture root and requires it
        to be the file the command was asked to write, which is the observable
        contract; an earlier draft demanding an absolute path failed against the
        real binary and was corrected, never the production side. (2) The
        artifact-transition fixture seeds the phases as the record's own `phase`
        entries (`kind: "phase"`, label naming the artifact), the shape the
        report's derivation reads; no capture point is added, because the design
        names none for this transition and no scenario requires one.

        Production files (this scenario's own):
        - `src/cli/timeline.rs` — the phase derivation (`WORKFLOW_STAGES`,
          `STAGE_ARTIFACTS`, `stage_opened_by`, `artifact_transition`,
          `stage_assignments`), the model's `stages` assignments and each band's
          `stages`, the row's visible phase line (`<text class="band-phases">`),
          each span's `data-stage`, and the row height the phase line needs;
        - `tests/steps/development_timeline_support.rs` — `WORKFLOW_ARTIFACTS`,
          the one-hour `TRANSITION_AT` ladder, `STAGE_WORK`, and
          `seed_report_history`, which writes the transitions through the
          record's own append path and returns the post-transition step the
          assertion names;
        - `tests/steps/development_report_e2e_steps.rs` — the scenario's four
          steps and the CLI evidence capture;
        - the two evidence screenshots above.

        The CLI's default path form (`givn timeline --change demo`, no `--out`)
        is already exercised by the fixture path the design's D4 names
        (`givn/history/reports/demo.html`, `src/cli/timeline.rs::default_report_path`);
        this scenario's own feature text has no step for a second invocation, so
        the additional assertion the design allows is not added — the scenario
        asserts exactly its own steps, never an extra hidden action.

        Discrimination (probe, isolated fixture, restored immediately): the
        fixture's last transition moved after the post-transition step, so the
        record no longer places that step in the phase the transition opened:
        ✘  And the report file places the step recorded after that transition in the "implement" phase
           Step panicked. Captured output: … <g class="span" data-entry-id="13" data-kind="step" data-at="2026-10-16T13:30:00.000Z" data-stage="tasks" aria-label="cargo test --locked --test features_runner, 30 m"> …
           left: "tasks"
           right: "implement"
        1 scenario (1 failed)
        7 steps (6 passed, 1 failed)
        PROBE_EXIT=101
        The fixture was restored byte-identically (`diff` → identical) and the
        restored tree re-run: `RESTORED_EXIT=0` (the GREEN run above is that
        restored run).
        ```
  - [x] **REFACTOR** — one phase-derivation function used by the CLI and the
        renderer; re-run → still exit 0. Paste it.
        ```
        The derivation is one function (`stage_assignments`) and the renderer no
        longer searches the assignments itself: `ReportModel::stage_of(entry)` is
        the model's single accessor, so the CLI's rendering and the browser
        tier's placement assertion read the same assignment:
        $ grep -n "fn stage_of\|fn stage_assignments\|model.stage_of" src/cli/timeline.rs
        592:impl ReportModel {
        598:    fn stage_of(&self, entry: u64) -> Option<&str> {
        605:/// Every band's entries with the stage they worked in: an entry belongs to the
        614:fn stage_assignments(bands: &[Band], entries: &[HistoryEntry]) -> Vec<StageAssignment> {
        934:                let stage = model.stage_of(entry.id);

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 9.40s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer generates the development report for a change"
        Feature: Development Report
          Scenario: Consumer generates the development report for a change
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
           ✔  When the user runs `givn timeline --change demo --out report.html`
           ✔  Then the command exits 0
           ✔  And the command prints the path of the written report
           ✔  And the report file exists and names the recorded phases
           ✔  And the report file places the step recorded after that transition in the "implement" phase
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        REFACTOR_EXIT=0

        The re-run refreshed the evidence pair from the final code (49447/118181
        bytes, 360x780 and 1920x1080). The rendered report reads, at the change's
        own band: the phase line `propose · specs · design · arc42 ·
        design-review · tasks · implement` and, beneath it, the record's steps —
        the post-transition step's own span drawn under `implement`.
        ```
  - [x] **COMMIT** — `ec81a9c34c9fc5aa084dcbf61de8e00c803dcfe6` — `test(e2e): <release note>` with the scenario
        title `Consumer generates the development report for a change` in the
        body (including the two evidence PNGs). Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        Consumer generates the development report for a change"
        [17:49:41] committed revision on the Git backend

        $ git show --stat --format='%H' ec81a9c3
        ec81a9c34c9fc5aa084dcbf61de8e00c803dcfe6
         .../desktop.png                                    | Bin 0 -> 118181 bytes
         .../mobile.png                                     | Bin 0 -> 49447 bytes
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 154 +++++++++++++-
         src/cli/timeline.rs                                | 200 +++++++++++++++++-
         tests/steps/development_report_e2e_steps.rs        | 235 ++++++++++++++++++++-
         tests/steps/development_timeline_support.rs        | 134 ++++++++++++
         7 files changed, 702 insertions(+), 23 deletions(-)

        The revision is this scenario's RED+GREEN+REFACTOR as one atomic
        commit: the `@wip` removal from this scenario alone, the phase
        derivation and its rendering in `src/cli/timeline.rs`, the scenario's
        own steps with the shared fixture builder, this task file's evidence,
        and the scenario's one evidence pair. No other scenario's tag, step, or
        capture is touched.
        ```

## Task E6 — `Consumer opens the generated development report in a browser` (`@e2e`, `development-report`, Scenario Outline, `change` and `topic` rows)

- [x] **Scenario: `Consumer opens the generated development report in a browser`** (Examples: `change`, `topic`)
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the generated development report in a browser"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the generated development report in a browser"
        Feature: Development Report
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a change "demo" with a recorded history
           ✘  And the development report for "demo" has been generated
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:78:5
              Step doesn't match any function
          Scenario Outline: Consumer opens the generated development report in a browser
           ✘  Given a topic "demo" with a recorded history
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:77:5
              Step doesn't match any function
        [Summary]
        1 feature
        2 scenarios (2 failed)
        3 steps (1 passed, 2 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined steps. Both rows fail inside the one process this
        outline unit runs — the change row on the unbound report-generation
        Given (its own `<scope>` Given is the shared, real `givn new demo` +
        `givn status --change demo` fixture, which really ran and really
        recorded), the topic row on the topic Given that does not exist yet.
        The RED is therefore the missing fixture, generation, browser-open and
        rendered-surface assertion path, never a vacuous setup.
        ```
  - [x] **GREEN** — implement the browser-open steps and the report's heading,
        timeline SVG, and accessible names. Files:
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/common/browser.rs` / `tests/e2e/browser-driver.mjs` (any gap the
        real driver exposes), `assets/timeline/report.html` (semantics and
        labels), plus the two evidence screenshots. Compile first, re-run →
        exit 0. Paste it.
        ```
        $ cargo build --locked
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.91s
        BUILD=0
        $ node --check /tmp/report-layer.mjs      # the asset's inline layer, extracted
        NODE_CHECK=0
        $ cargo fmt --all -- --check
        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.14s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the generated development report in a browser"
        Feature: Development Report
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a change "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the development timeline
           ✔  And the browser shows the longest recorded step with its duration
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a topic "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the development timeline
           ✔  And the browser shows the longest recorded step with its duration
        [Summary]
        1 feature
        2 scenarios (2 passed)
        10 steps (10 passed)
        GREEN_EXIT=0

        Evidence pair (the outline has one slug; the topic row runs last and
        writes the pair the commit carries — the populated report at both
        contract viewports, captured after the final assertion with animations
        disabled by the driver):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-opens-the-generated-development-report-in-a-browser/
        -rw-r--r-- 66470 desktop.png
        -rw-r--r-- 44956 mobile.png
        $ file …/*.png
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced

        Production files (this scenario's own):
        - `assets/timeline/report.html` — the whole interactive layer: the
          script parses the model payload element, builds the axis, the band
          rows, the phase line, the spans (focusable, with `data-entry-id`,
          `data-kind`, `data-at`, `data-stage`, `data-duration-ms`, the
          accessible name `<name>, <duration>`, and the label-omitted state
          where the duration is drawn inline or the SVG title carries name and
          duration), and the aggregate markers for the entries a window cannot
          tell apart. It is built programmatically — no markup text, no
          `class="…"` literal, no remote reference — so a recorded label can
          never become a node and the empty-state document still carries none of
          the axis markers Task 32's assertions look for. The landmarks the
          contract names are real: `<header>` around the heading, `<main>` for
          the view, `<aside>` for the details region; and the responsive rules
          the design system's breakpoints state (mobile: the horizontally
          scrollable axis; from 720px: the summary beside the timeline; from
          1200px with the panel open: the panel on the right).
        - `src/cli/timeline.rs` — one rendering defect the browser surfaced and
          the visual baseline's "no visible content is duplicated" forbids: the
          summary's unmeasured-duration note rendered "6 recorded duration not
          measured not measured" (the helper's own text already ends in "not
          measured" and the call site appended it again). The doubled phrase is
          gone; the note still states every missing duration.
        - `tests/steps/development_report_e2e_steps.rs` — this outline's steps:
          the topic-row Given (the shared `seed_ideation_topic` fixture), the
          report-generation Given for either scope, the browser-open When, and
          the two Thens — the rendered heading/timeline/landmark/accessible-name
          assertions with the Tab-reachability and visible-focus-ring check, and
          the longest-recorded-step assertion (the record's own longest measured
          entry, its accessible name, and the browser's own text for it).
        - the two evidence screenshots above.

        The accessibility assertions the design leaves to the e2e tier are
        asserted here, at every interaction, by reading the browser's own DOM
        and focus state: the page's real `title`, exactly one `h1`, the
        `header`/`main`/`aside` landmarks, the timeline `svg`'s accessible name,
        every drawn span's accessible name (`<name>, <duration>`) and
        `tabindex`, and every span reached by real `Tab` presses with a focus
        ring the browser really rendered (the accent stroke for a span, the
        outline for a control) — never a class name the page wrote.

        The two rows' records, stated plainly rather than implied: the change
        row opens the record the shared Given really produced (`givn new demo` +
        `givn status --change demo`, whose change scope holds the recorded
        `status` command as its longest measured entry), and the topic row opens
        the topic fixture's own steps (`givn instructions ideation · 20 s`,
        `givn instructions eventstorming · 10 m`, `givn instructions usecase ·
        2 m`). The longest recorded step the second Then names is therefore the
        record's own longest measured entry in each row — read from the store,
        never a fixture constant — and the browser is required to draw it and to
        carry its name and duration in the element's own text and its accessible
        name.
        ```
  - [x] **REFACTOR** — one page-open helper shared by the browser scenarios;
        re-run (both rows) → still exit 0. Paste it.
        ```
        The page-open path is now one helper, `open_report`, used by this
        outline's When and by the CLI evidence capture: one navigation through
        the driver's `file://` URL, one context per scenario, no second page and
        no second browser. Everything the browser tier reads stays in the
        asset's single script, so there is one renderer to drive.
        $ grep -n "fn open_report\|open_report(world)" tests/steps/development_report_e2e_steps.rs
        149:fn open_report(world: &mut GivnWorld) {
        165:    open_report(world);
        482:    open_report(world);

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.14s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the generated development report in a browser"
        Feature: Development Report
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a change "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the development timeline
           ✔  And the browser shows the longest recorded step with its duration
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a topic "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the development timeline
           ✔  And the browser shows the longest recorded step with its duration
        [Summary]
        1 feature
        2 scenarios (2 passed)
        10 steps (10 passed)
        REFACTOR_EXIT=0

        Both rows re-ran in the one process this outline unit is, and the
        refactor refreshed the evidence pair from the final code (44956/66470
        bytes, 360x780 and 1920x1080). The regular scope is untouched and still
        green with the asset's new structure:
        $ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
             Summary [   3.853s] 35 tests run: 35 passed, 8 skipped
        NON_E2E_EXIT=0

        Two visual corrections the browser's own rendering exposed while the
        tier was driven (both in the asset, both visible in the evidence pair):
        the axis' last tick label was clipped at the viewBox edge and is now
        anchored back so its own text fits inside the axis, and the mobile
        viewport squeezed the whole 1000-unit axis into 360px — it is now the
        horizontally scrollable axis the design system's breakpoint table names
        (`min-width: 680px` with `overflow-x: auto` below 720px), so the mobile
        capture shows a readable axis instead of an unreadable squeeze.
        ```
  - [x] **COMMIT** — `4c8cba403cf9321625ea6002bb583deea18809d5` — `test(e2e): <release note>` with the scenario
        title `Consumer opens the generated development report in a browser` in
        the body (including the two evidence PNGs). Record the revision
        identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        Consumer opens the generated development report in a browser"
        [17:59:35] committed revision on the Git backend

        $ git show --stat --format='%H' 4c8cba40
        4c8cba403cf9321625ea6002bb583deea18809d5
         assets/timeline/report.html                        | 458 ++++++++++++++++++++-
         .../desktop.png                                    | Bin 0 -> 66470 bytes
         .../mobile.png                                     | Bin 0 -> 44956 bytes
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 171 +++++++-
         src/cli/timeline.rs                                |   2 +-
         tests/steps/development_report_e2e_steps.rs        | 361 +++++++++++++-
         7 files changed, 970 insertions(+), 24 deletions(-)

        The revision is this outline's RED+GREEN+REFACTOR as one atomic commit:
        the `@wip` removal from the outline alone, the asset's interactive layer
        with its landmarks and responsive rules, the doubled-note rendering fix,
        this outline's steps, this task file's evidence (including the Task 31
        alignment note and its revision line recorded in the separate
        `fix(development-report)` commit `d0899202`), and the outline's one
        evidence pair. No other scenario's tag, step, or capture is touched.

        Two out-of-bundle records, stated rather than hidden: (1) the Task 31
        assertion alignment landed as its own commit before this one
        (`d08992027b8b4d9dcbbb4f0afcf9e73ef0eb3657`), because design.md requires
        the inline script that assertion's count forbade; (2) this scenario's
        RED evidence was pasted into tasks.md before that alignment commit and
        therefore rides in it — the RED run's output and the GREEN/REFACTOR
        evidence in this section are the same runs either way.
        ```

## Task E7 — `Consumer selects a timeline element and reads its details` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer selects a timeline element and reads its details`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"
        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✘  Given an open development report with a recorded history
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:90:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined step — the scenario's own first Given has no
        binding, so the run fails before the report is even generated. The RED
        is the missing open-report fixture, selection step, and details-panel
        assertion path; the panel's markup exists in the asset but nothing opens
        it without the selection the GREEN below adds.
        ```
  - [x] **GREEN** — implement selection and the details panel. Files:
        `assets/timeline/report.html` (span focus/activation, panel markup and
        labels), `tests/steps/development_report_e2e_steps.rs`, plus the two
        evidence screenshots. Re-run → exit 0. Paste it.
        ```
        $ node --check /tmp/report-layer.mjs      # the asset's inline layer, extracted
        NODE_CHECK=0
        $ cargo build --locked --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
        TESTS_BUILD=0
        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.14s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"
        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✔  Given an open development report with a recorded history
           ✔  When the consumer selects the longest recorded step
           ✔  Then the browser shows a details panel for that step
           ✔  And the panel names its start, its duration, and its outcome
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        GREEN_EXIT=0

        Evidence pair (the panel open at both contract viewports, captured after
        the final assertion, animations disabled; the page is brought to the
        panel the assertion read, because at the mobile viewport the panel is a
        full-width sheet below the timeline):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/
        -rw-r--r-- 104180 desktop.png
        -rw-r--r--  42881 mobile.png
        $ file …/*.png
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced

        Production files (this scenario's own):
        - `assets/timeline/report.html` — the selection and the details panel:
          `select(entryId)` fills the panel's facts (the bounded label, start,
          end, duration, outcome, and the model and tokens the record reports —
          each missing value stated, never a zero), unhides the labelled region,
          and marks the span selected; `clearSelection` closes it; the pointer's
          click and the keyboard's `Enter`/`Space` reach the one activation, and
          `Escape` closes the panel wherever the focus is.
        - `tests/common/browser.rs` — `scroll_into_view` and
          `capture_evidence_from`, so a capture shows the state the scenario
          asserted even where that state sits below the fold at the mobile
          viewport (`capture_evidence` is the same helper anchored at the report
          title).
        - `tests/steps/development_report_e2e_steps.rs` — the scenario's four
          steps: the open-report Given (the shared artifact-transition fixture,
          generated and opened), the selection When (a real click, then
          `Escape`, then `Enter` on the focused span — both activation paths),
          and the two Thens, whose expectations come from the record (the
          scope's longest measured entry, its own recorded end minus its
          duration as the start, its duration, and its outcome).
        - the two evidence screenshots above.

        What the assertion reads, precisely: the panel's own DOM — that it is
        open, that it is the labelled region (`role="region"`,
        `aria-label="Details"`), that its close control carries the visible label
        `Close`, that its text names the selected step, and that its facts carry
        the start the record implies (`13:00:00`), the measured duration
        (`30 m`), and the recorded outcome (`ok`) with their written labels. The
        start comes from the embedded timestamps (the browser's own clock of the
        record), never from the wall clock.
        ```
  - [x] **REFACTOR** — one selection path for mouse and keyboard; re-run → still
        exit 0. Paste it.
        ```
        One selection path, literal in the asset: `select(entryId)` is the only
        place the panel opens and a span is marked, and both the pointer and the
        keyboard reach it through the one `activate(event)` handler — the
        keyboard listener now only filters its keys instead of repeating the
        activation.
        $ grep -n "function select\|function activate\|addEventListener(\"click\", activate)\|addEventListener(\"keydown\"" assets/timeline/report.html
        629:  function select(entryId) {
        656:  function activate(event) {
        661:  timeline.addEventListener("click", activate);
        663:  timeline.addEventListener("keydown", (event) => {

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.00s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"
        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✔  Given an open development report with a recorded history
           ✔  When the consumer selects the longest recorded step
           ✔  Then the browser shows a details panel for that step
           ✔  And the panel names its start, its duration, and its outcome
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        REFACTOR_EXIT=0

        The re-run refreshed the evidence pair from the final code (42881/104180
        bytes, 360x780 and 1920x1080). The desktop capture shows the panel as the
        design system's right-hand side panel with the selected span stroke-marked
        in the accent colour, and the mobile capture shows the same panel as the
        full-width sheet below the timeline.
        ```
  - [x] **COMMIT** — `88334ae790b3135e3165f1d8c5d26859ee4ef326` — `test(e2e): <release note>` with the scenario
        title `Consumer selects a timeline element and reads its details` in the
        body (including the two evidence PNGs). Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        Consumer selects a timeline element and reads its details"
        [18:03:15] committed revision on the Git backend

        $ git show --stat --format='%H' 88334ae7
        88334ae790b3135e3165f1d8c5d26859ee4ef326
         assets/timeline/report.html                        | 126 ++++++++++++--
         .../desktop.png                                    | Bin 0 -> 104180 bytes
         .../mobile.png                                     | Bin 0 -> 42881 bytes
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 130 ++++++++++++++-
         tests/common/browser.rs                            |  22 +++
         tests/steps/development_report_e2e_steps.rs        | 183 +++++++++++++++++++++
         7 files changed, 445 insertions(+), 18 deletions(-)

        The revision is this scenario's RED+GREEN+REFACTOR as one atomic commit:
        the `@wip` removal from this scenario alone, the selection and the
        details panel in the asset, the browser helper's anchored capture, the
        scenario's four steps, this task file's evidence, and the scenario's one
        evidence pair. No other scenario's tag, step, or capture is touched.
        ```

## Task E8 — `Consumer zooms the timeline to a shorter time range` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer zooms the timeline to a shorter time range`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer zooms the timeline to a shorter time range"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer zooms the timeline to a shorter time range"
        Feature: Development Report
          Scenario: Consumer zooms the timeline to a shorter time range
           ✔  Given an open development report with a recorded history
           ✘  When the consumer zooms the timeline in
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:98:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined step — the zoom When has no binding, and the
        reused open-report Given really built and opened the fixture (the shared
        artifact-transition record's own report), so the RED is the missing zoom
        control, the missing rendered range state, and the missing before/after
        assertion, never a vacuous setup.
        ```
  - [x] **GREEN** — implement zoom (control, axis range label, reduced-motion
        behaviour) and the before/after assertion. Files:
        `assets/timeline/report.html`, `tests/steps/development_report_e2e_steps.rs`,
        plus the two evidence screenshots. Re-run → exit 0. Paste it.
        ```
        $ node --check /tmp/report-layer.mjs      # the asset's inline layer, extracted
        NODE_CHECK=0
        $ cargo build --locked --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.78s
        TESTS_BUILD=0
        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.00s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer zooms the timeline to a shorter time range"
        Feature: Development Report
          Scenario: Consumer zooms the timeline to a shorter time range
           ✔  Given an open development report with a recorded history
           ✔  When the consumer zooms the timeline in
           ✔  Then the browser shows a narrower time range than before
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        GREEN_EXIT=0

        Evidence pair (the narrowed range at both contract viewports, captured
        after the assertions, animations disabled):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-zooms-the-timeline-to-a-shorter-time-range/
        -rw-r--r-- 88869 desktop.png
        -rw-r--r-- 49417 mobile.png
        $ file …/*.png
        …/mobile.png:  PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/desktop.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced
        The desktop capture shows the three controls with their visible labels
        (`Zoom in` / `Zoom out` / `Fit`), the rendered range label
        `Visible range 09:15:00 → 12:05:00` (narrowed from the record's own
        `07:50:00 → 13:30:00`), and the axis drawn to the narrowed window.

        Production files (this scenario's own):
        - `assets/timeline/report.html` — the zoom: the visible window as mutable
          state inside the record's own bounds, the `applyRange(next)` derivation
          every path goes through, `zoom(factor, anchor)` around the pointer's
          own instant, the three labelled controls (`Zoom in` / `Zoom out` /
          `Fit`, `disabled` at their limits) created only where a timeline
          exists, the rendered range label (visible text plus
          `data-range-start-ms` / `data-range-end-ms`), the wheel and drag paths
          on the same axis, a click on an aggregate marker zooming to its
          bucket, and the token's zoom transition (`report-zoom`, 120 ms) that
          the reduced-motion block removes entirely.
        - `tests/steps/development_report_e2e_steps.rs` — the scenario's When and
          Then plus the two rendered-state probes (`rendered_range`,
          `rendered_motion`), and the control expectations the other browser
          steps share: every rendered control carries a visible text label, only
          controls that are not `disabled` at their own limit are required to be
          Tab-reachable, and the zoom-out limit reveals itself when the range
          narrows.
        - the two evidence screenshots above.

        Rendered-state evidence, as the design demands: the assertion reads the
        visible time range the page's own script produced, twice — the range the
        browser showed before the click and the range it showed after — and
        requires the second to be strictly narrower and its label to have
        changed; the window's own bounds are asserted against the record's range
        as well. Two limit-state facts are asserted against the live DOM rather
        than assumed: the zoom-out control is `disabled` while the whole range is
        shown and enabled once the range has narrowed. The motion contract is
        asserted from the browser's computed style: the context emulates
        `prefers-reduced-motion: reduce`, the zoom animation resolves to `none`
        there, and the narrowed range above is what the change applied
        instantly.

        One honest note about the other browser scenario this step also touches:
        with the zoom controls present, the change row's record is a single
        instant wide, so all three controls are correctly `disabled` (there is
        nothing to zoom) — the shared expectation therefore requires only the
        controls a limit leaves enabled, which is what the two rows' runs show.
        ```
  - [x] **REFACTOR** — one range-state derivation for wheel, buttons, and
        aggregation; re-run → still exit 0. Paste it.
        ```
        One range-state derivation, literal in the asset: `applyRange(next)` is
        the only place the visible window changes, and every interaction reaches
        it — the buttons, the wheel, the drag, a double click, and a click on an
        aggregate marker — so the drawing, the label, and the controls' limits
        can never disagree with the window.
        $ grep -n "function applyRange\|applyRange(" assets/timeline/report.html
        623:    fit.addEventListener("click", () => applyRange(bounds));
        666:  function applyRange(next) {
        685:    applyRange({ start, end: start + nextSpan });
        718:    applyRange({ start: dragging.range.start + shift, end: dragging.range.end + shift });
        728:  if (section) section.addEventListener("dblclick", () => applyRange(bounds));
        829:      if (Number.isFinite(start) && Number.isFinite(end)) applyRange({ start, end });

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.00s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer zooms the timeline to a shorter time range"
        Feature: Development Report
          Scenario: Consumer zooms the timeline to a shorter time range
           ✔  Given an open development report with a recorded history
           ✔  When the consumer zooms the timeline in
           ✔  Then the browser shows a narrower time range than before
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        REFACTOR_EXIT=0

        The re-run refreshed the evidence pair from the final code, and the
        outline this scenario's control expectations live in re-ran green as well
        (`Consumer opens the generated development report in a browser`, 2
        scenarios / 10 steps, exit 0) — with its own committed evidence pair left
        untouched, restored to its revision's captures after the re-run.
        ```
  - [x] **COMMIT** — `35b80e06845efd9237ddff6015771de5af514c77` — `test(e2e): <release note>` with the scenario
        title `Consumer zooms the timeline to a shorter time range` in the body
        (including the two evidence PNGs). Record the revision identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        Consumer zooms the timeline to a shorter time range"
        [18:07:31] committed revision on the Git backend

        $ git show --stat --format='%H' 35b80e06
        35b80e06845efd9237ddff6015771de5af514c77
         assets/timeline/report.html                        | 186 +++++++++++++++++++-
         .../desktop.png                                    | Bin 0 -> 88869 bytes
         .../mobile.png                                     | Bin 0 -> 49417 bytes
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 141 ++++++++++++++-
         tests/steps/development_report_e2e_steps.rs        | 191 +++++++++++++++++++++
         6 files changed, 506 insertions(+), 14 deletions(-)

        The revision is this scenario's RED+GREEN+REFACTOR as one atomic commit:
        the `@wip` removal from this scenario alone, the zoom and its controls in
        the asset, the scenario's When and Then with the shared control
        expectations, this task file's evidence, and the scenario's one evidence
        pair. No other scenario's tag, step, or capture is touched (the outline's
        committed evidence pair was restored to its own revision's captures after
        the re-run).
        ```

## Task E9 — `Consumer opens the report of a scope with no recorded activity` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer opens the report of a scope with no recorded activity`**
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
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the report of a scope with no recorded activity"`
        → must exit non-zero. Paste it.
        ```
        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the report of a scope with no recorded activity"
        Feature: Development Report
          Scenario: Consumer opens the report of a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✘  Then the browser shows the empty state for "demo"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:106:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)
        error: test failed, to rerun pass `--test features_runner`
        RED_EXIT=101

        RED category: undefined step — the two Thens that read the rendered
        empty state have no binding. The first three steps really ran (the
        no-activity fixture, the real `givn timeline`, and the real browser
        navigation), so the RED is the missing rendered-surface assertion and the
        missing declared-state captures, never a vacuous setup.
        ```
  - [x] **GREEN** — implement the empty-state browser assertions and capture
        both pairs. Files: `assets/timeline/report.html`,
        `tests/steps/development_report_e2e_steps.rs`, plus the four evidence
        screenshots. Re-run → exit 0. Paste it.
        ```
        $ node --check /tmp/report-layer.mjs      # the asset's inline layer, extracted
        NODE_CHECK=0
        $ cargo build --locked --tests
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.24s
        TESTS_BUILD=0
        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.98s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the report of a scope with no recorded activity"
        Feature: Development Report
          Scenario: Consumer opens the report of a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the empty state for "demo"
           ✔  And the browser draws no timeline axis for it
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        GREEN_EXIT=0

        Evidence: the final-state pair and the declared `empty` state's pair, all
        four non-empty and at the contract viewports, captured after the
        assertions with animations disabled (the final state of this scenario *is*
        the declared state, so both pairs show the empty report — the state pair
        is the one the visual gate requires for the contract's `empty` state):
        $ ls -l givn/changes/development-timeline/evidence/visual/consumer-opens-the-report-of-a-scope-with-no-recorded-activity/
        -rw-r--r-- 29446 desktop--empty.png
        -rw-r--r-- 29446 desktop.png
        -rw-r--r-- 22571 mobile--empty.png
        -rw-r--r-- 22567 mobile.png
        $ file …/*.png
        …/mobile.png:        PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/mobile--empty.png: PNG image data, 360 x 780, 8-bit/color RGB, non-interlaced
        …/desktop.png:       PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced
        …/desktop--empty.png: PNG image data, 1920 x 1080, 8-bit/color RGB, non-interlaced
        The captures show the rendered empty state: the heading, the one sentence
        naming the scope, the way back to the report root — and no axis at all.

        Production files (this scenario's own):
        - `assets/timeline/report.html` — the interactive layer's own half of the
          empty state: it is inert when the report draws no timeline, so the
          `<svg class="timeline">` element is the one thing that decides whether
          the axis, the controls, and the range label exist at all.
        - `tests/steps/development_report_e2e_steps.rs` — the scenario's two
          Thens and the `rendered_empty_state` probe: the empty state's
          presence, its declared state, its heading, the one sentence naming the
          scope, the labelled way back (with its `#report-title` target), and the
          DOM-level absence of every axis marker — `svg`, `svg.timeline`,
          `.band`, `.span`, `.tick`, `.aggregate`, and `.controls button` all
          counted as zero in the browser's own DOM.
        - the four evidence screenshots above.
        ```
  - [x] **REFACTOR** — one empty-state branch shared with Task 32's
        report-level rendering; re-run → still exit 0. Paste it.
        ```
        One empty-state branch, shared with Task 32's report-level rendering: the
        generator renders either the timeline section or `render_empty_state` for
        a scope with no activity, and the browser layer now states the same
        condition once — `drawsTimeline` — instead of two early returns, so
        "there is no timeline to draw" is one decision in each tier and the
        browser's blindness in the empty state (no axis, no controls, no range
        label) follows from the generator's own branch rather than from a second
        rule of its own.
        $ sed -n '219,231p' assets/timeline/report.html
          const SVG = "http://www.w3.org/2000/svg";
          const modelElement = document.getElementById("report-model");
          const timeline = document.querySelector("svg.timeline");
          const model = modelElement ? JSON.parse(modelElement.textContent) : null;
          /* One condition for "this report draws a timeline": the generator renders the
             timeline section only for a record with activity, and the empty state — the
             same branch Task 32's scenario asserts at the report level — therefore
             carries no axis, no controls, and no range label. The browser layer is inert
             there instead of drawing over nothing. */
          const drawsTimeline = Boolean(timeline && model && !model.empty);
          if (!drawsTimeline) return;

        $ cargo fmt --all -- --check        FMT=0
        $ cargo clippy --locked --all-targets -- -D warnings
            Checking givn v0.9.0 (/home/buster/projects/givn)
            Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.98s
        CLIPPY=0

        $ GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the report of a scope with no recorded activity"
        Feature: Development Report
          Scenario: Consumer opens the report of a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the empty state for "demo"
           ✔  And the browser draws no timeline axis for it
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        REFACTOR_EXIT=0
        ```
  - [x] **COMMIT** — `11647c8be76c5f0030a3f74774ba089e3d53d162` — `test(e2e): <release note>` with the scenario
        title `Consumer opens the report of a scope with no recorded activity` in
        the body (including the four evidence PNGs). Record the revision
        identifier.
        ```
        $ ./target/debug/givn commit -m "test(e2e): Givn records the development process of every change and ideation — … opens in a browser.

        Consumer opens the report of a scope with no recorded activity"
        [18:08:50] committed revision on the Git backend

        $ git show --stat --format='%H' 11647c8b
        11647c8be76c5f0030a3f74774ba089e3d53d162
         assets/timeline/report.html                        |  11 +-
         .../desktop--empty.png                             | Bin 0 -> 29446 bytes
         .../desktop.png                                    | Bin 0 -> 29446 bytes
         .../mobile--empty.png                              | Bin 0 -> 22571 bytes
         .../mobile.png                                     | Bin 0 -> 22567 bytes
         .../development-report.feature                     |   2 +-
         givn/changes/development-timeline/tasks.md         | 129 +++++++++++++++++++--
         tests/steps/development_report_e2e_steps.rs        |  97 ++++++++++++++++

        The revision is this scenario's RED+GREEN+REFACTOR as one atomic commit:
        the `@wip` removal from this scenario alone (with it, no `@wip` remains on
        any scenario this change implements), the asset's one `drawsTimeline`
        condition, the scenario's two Thens and its probe, this task file's
        evidence, and the scenario's four captures — the final-state pair the
        visual gate reads for every `@e2e` scenario plus the contract's declared
        `empty` state's pair. No other scenario's tag, step, or capture is
        touched.
        ```

---

## Final scope runs (2026-10-16, after Task E9)

Both declared scopes were run on the complete delta, after Task E9's commit and
with no `@wip` tag left anywhere in the delta (`grep -rn "@wip"
givn/changes/development-timeline/specs/ | wc -l` → `0`):

```
$ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --non-e2e
     PASS [   2.790s] (35/35) …inspect-development-history/development-record.feature::A verification run is recorded as its own step
────────────
     Summary [   3.611s] 35 tests run: 35 passed, 8 skipped
NON_E2E_EXIT=0                                    # verify.command → 35 scenarios

$ GIVN_FEATURES=givn/changes/development-timeline/specs ./run-tests.sh --e2e
     PASS [   1.760s] (3/8) …development-report.feature::Consumer zooms the timeline to a shorter time range
     PASS [   2.177s] (4/8) …development-report.feature::Consumer generates the development report for a change
     PASS [   2.486s] (5/8) …development-report.feature::Consumer opens the report of a scope with no recorded activity
     PASS [   2.995s] (6/8) …development-report.feature::Consumer opens the generated development report in a browser
     PASS [   3.789s] (7/8) …development-report.feature::Consumer selects a timeline element and reads its details
     PASS [   4.972s] (8/8) …agent-activity-reporting.feature::Consumer installs the agent reporter and its reported activity lands in the record
────────────
     Summary [   4.973s] 8 tests run: 8 passed, 35 skipped
E2E_EXIT=0                                        # verify.e2e_command → 8 scenarios
```

The eight `@e2e` scenarios' evidence directories, as committed (each pair from
its own scenario's revision; a full e2e scope run re-captures every pair, so the
pairs were restored to their revisions' captures afterwards):

```
$ for d in givn/changes/development-timeline/evidence/visual/*/; do printf '%-72s %s\n' "$(basename $d)" "$(ls $d | tr '\n' ' ')"; done
consumer-generates-the-development-report-for-a-change                   desktop.png mobile.png
consumer-installs-the-agent-reporter-and-its-reported-activity-lands-in-the-record desktop.png mobile.png
consumer-opens-the-generated-development-report-in-a-browser             desktop.png mobile.png
consumer-opens-the-report-of-a-scope-with-no-recorded-activity           desktop--empty.png desktop.png mobile--empty.png mobile.png
consumer-reads-the-development-record-as-machine-readable-output         desktop.png mobile.png
consumer-runs-a-lifecycle-step-and-reads-its-recorded-actions            desktop.png mobile.png
consumer-selects-a-timeline-element-and-reads-its-details                desktop.png mobile.png
consumer-zooms-the-timeline-to-a-shorter-time-range                      desktop.png mobile.png
```

Every file is non-empty (so `src/integrity_hook.rs::check_visual_evidence`
finds a capture for every `@e2e` scenario and both declared-state files for the
empty-scope scenario), and `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets -- -D warnings`, and `node --check` on the
asset's extracted inline layer were green at every commit of the tier.

---

**Done when:** every box above is checked with its pasted evidence and revision
identifier, `verify.command` (`./run-tests.sh --non-e2e`) and
`verify.e2e_command` (`./run-tests.sh --e2e`) both exit 0 on the full suite, the
e2e setup task's scenario-count proof is pasted (46 regular vs 8 `@e2e`, e2e
strictly less), every `@e2e` scenario has committed `mobile.png` + `desktop.png`
under its slug with the empty-scope scenario's `mobile--empty.png` /
`desktop--empty.png`, and no `@wip` tag remains on any scenario this change
implements.

---

## Delta part 2 — hybrid workflow report (2026-10-17)

The operator approved the **hybrid workflow report** on 2026-10-17 after five
rendered mockups; design-review round 6 passed at `6b56c193` on the design delta
`307c0755`. The hybrid supersedes the dense horizontal timeline (D6) that
Tasks 25–33 and E5–E9 implemented: the report now reads top to bottom — the
header with the derived current state, the KPI strip, the nine-phase workflow
stack (each phase expandable into a vertical lane timeline with the selected
lane's log), then the statistics. This part schedules the delta's **15 pending
`@wip` scenarios** (14 in `development-report.feature`, 1 in
`development-record.feature`), the **shared model scaffold** the design names
(a shared task with a unit-level proof, because no scenario asserts its three
KPI totals directly), the **asset rewrite**, and one change-wide evidence and
scope-run task.

The earlier sections stay untouched and historical: their pasted evidence
records the dense-timeline implementation the delta re-tagged. A scenario this
delta re-opened is implemented again in its own section below and committed
again there; nothing above is re-run or rewritten.

### Preconditions and interaction-coverage pre-flight (verified 2026-10-17)

- `givn status --change development-timeline`: `design-review` **done**
  (`DESIGN-REVIEW: PASS` at `6b56c193`), `tasks` at **274/274** boxes, next
  required **review**. The tasks precondition holds and no `design.md` edit
  un-gated it.
- **Pre-flight result: PASS — 9 inventory entries, 9 matrix rows, 9 existing
  `@e2e` scenarios, one-to-one.** The use case's
  `specs/inspect-development-history/usecase.md` §Interactions (9 rows), the
  design's §Interaction Coverage Matrix (9 rows), and the delta features' nine
  `@e2e` titles name the same title set. The 8-row table near the top of this
  file is the 2026-10-16 state; the delta added the **expand a workflow phase**
  row (review F9) and retargeted the select and zoom rows at the hybrid (lane
  selection; the `15 m` control). The two lists order the rows differently
  (the use case lists the empty scope before the expand; the matrix lists the
  expand before select/zoom and the empty scope last) — the sets match exactly.
  No gap, and no task below is written for a scenario that does not exist.

| # | `usecase.md` §Interactions — capability: consumer action | `design.md` matrix — real interface / driving mechanism | `@e2e` title (`.feature`) |
|---|---|---|---|
| 1 | development-record: run a lifecycle command | CLI / `common::run_givn`: `givn status --change demo`, then `givn history --change demo --json` | `Consumer runs a lifecycle step and reads its recorded actions` |
| 2 | development-record: read the development record | CLI / `givn history --change demo --json` against a seeded record | `Consumer reads the development record as machine-readable output` |
| 3 | agent-activity-reporting: report agent activity from a supported harness | CLI (harness integration) / one outline, one Examples row per harness, driving each generated reporter through its real entry point | `Consumer installs the agent reporter and its reported activity lands in the record` |
| 4 | development-report: generate the development report for a change | CLI / `givn timeline --change demo --out report.html`, assertion reads the written file | `Consumer generates the development report for a change` |
| 5 | development-report: open the report in a browser | Web UI / Playwright `page.goto("file://<root>/report.html")`, one outline with a `change` and a `topic` row; mobile viewport assertion | `Consumer opens the generated development report in a browser` |
| 6 | development-report: expand a workflow phase | Web UI / Playwright: click the `implement` phase expander; read the vertical axis, one lane per session, and the Fit / 1 h / 15 m controls | `Consumer expands a workflow phase and sees its lanes` |
| 7 | development-report: select a timeline element | Web UI / Playwright: expand the phase, then select the `Architect.Scout` lane; read the lane log's start/duration/outcome, the tool call id, and the bounded content region | `Consumer selects a timeline element and reads its details` |
| 8 | development-report: zoom the timeline | Web UI / Playwright: expand the phase, click `15 m`, compare the rendered range label before and after | `Consumer zooms the timeline to a shorter time range` |
| 9 | development-report: open the report of a scope with no recorded activity | Web UI / Playwright: open the generated empty-scope report; read the empty-state heading and the absence of any phase | `Consumer opens the report of a scope with no recorded activity` |

**Primary-assertion rule (copied from the matrix):** every browser scenario
asserts on what the browser rendered (DOM text, accessible names, the rendered
range label) — never on the report file's raw bytes as its primary assertion;
the CLI scenarios assert on the subprocess's stdout and the files it wrote.
Repository checks are secondary only.

### Delta inventory (verified mechanically against the five delta features)

| Feature (path relative to `GIVN_FEATURES`) | Entries | Non-`@e2e` | `@e2e` |
|---|---|---|---|
| `inspect-development-history/development-record.feature` | 21 | 19 | 2 |
| `inspect-development-history/agent-activity-reporting.feature` | 15 | 14 | 1 |
| `inspect-development-history/development-report.feature` | 23 | 17 | 6 |
| `fragments/adr-qualification-gate.feature` (`@givn.modified`) | 1 | 1 | 0 |
| `initialize-project/initialize-a-project.feature` (`@givn.modified`, two) | 2 | 2 | 0 |
| **Total** | **62** | **53** | **9** |

- **62** executable scenario entries = **59 `@givn.added`** + **3
  `@givn.modified`**; **53 non-`@e2e`** + **9 `@e2e`**. The third column counts
  a `Scenario Outline` as one entry (its Examples rows are input variation).
- **15 `@wip`** scenarios are pending: 1 in `development-record.feature` and 14
  in `development-report.feature` (8 non-`@e2e`, 6 `@e2e`). `givn lint --change
  development-timeline` at the baseline of this part reports exactly those 15
  `[WIP]` findings plus the pre-existing `[SUBST] advisory` lines, exit 2 — the
  pending tags, not a parse error.
- **Scheduled here:** 17 sections = **15 scenario tasks** (H1, H3–H10, HE1–HE6)
  + **1 shared prerequisite** (H2, report-model scaffold; no new scenario) +
  **1 change-wide evidence/scope-run task** (HE7; no new scenario). The three
  `@givn.modified` entries stay with S2b and Task 24a; this part adds none.
- **Box accounting:** 17 sections × 5 boxes = **85 new boxes**; `tasks.md`
  moves from **274/274** boxes to **359 total / 274 done / 85 pending**.
  `givn status` reports the tasks artifact as partial until every new box is
  checked with its evidence and revision.
- **Work order:** H1 (the dispatcher writer its own scenario needs) → H2 (the
  shared model scaffold every report scenario asserts through) → H3–H10 (the
  eight regular report scenarios in `.feature` order) → HE1–HE6 (the six
  `@e2e` scenarios in `.feature` order, the asset rewrite beginning in HE2) →
  HE7 (evidence refresh and both scope runs). Non-`@e2e` first, `@e2e` last, as
  the tasks instruction requires.

### Decision constraints that apply to every task in this part

- **The workflow order** is exactly `WORKFLOW_STAGES` in `src/cli/timeline.rs`
  (nine phases): `propose`, `specs`, `design`, `arc42`, `design-review`,
  `tasks`, `implement`, `review`, `archive`. No phase name outside that list is
  ever rendered or invented.
- **The record is the only evidence.** A duration the record does not carry
  reads "not measured"; an outcome it does not carry reads "unknown"; a fact the
  harness did not report reads "not reported"/"unavailable", never zero. Never
  modify `givn/history/events.jsonl`.
- **Determinism** is over the record **plus the embedded change-state snapshot**
  (R-96): the asset never calls `fetch`, never loads a remote resource, never
  reads the wall clock (`Date.now()`), and the same record + state always
  renders identically. Two renders may differ only when the change state moved.
- **Server absence:** the report stays one self-contained HTML document; no
  server, no third-party JavaScript, no `Date.now()`, and the generator
  substitutes exactly the two documented markers once each
  (`/*__GIVN_REPORT_DATA__*/` with `<` escaped as `\u003c`, and
  `<!--__GIVN_REPORT_VIEW__-->`).
- **No store change:** `ReportModel`, `WorkflowPhase`, `Lane`, `RepeatedGroup`,
  `PhaseAggregate`, and `CurrentState` are serialized only inside the generated
  report. The record schema gains no field: the transition is a `phase` entry
  with an existing label form.
- **Evidence rule (Visual Design Contract):** each `@e2e` scenario commits its
  final-state pair under
  `givn/changes/development-timeline/evidence/visual/<scenario-slug>/{mobile.png,desktop.png}`
  — mobile at **360x780**, desktop at **1920x1080**, animations disabled, no
  intermediate captures under the change directory. The **existing titles and
  therefore slugs are preserved**; the open, select, zoom, generate, and empty
  scenarios refresh their pairs in place, and only the **new expand** scenario
  introduces a new directory. `scenario_slug` is lowercase ASCII, every run of
  non-alphanumerics becomes one `-`, leading/trailing `-` removed.
- Every command below is the canonical single-scenario form the earlier tasks
  use. **`--name` is banned.** Never infer a scenario's green from whole-suite
  output.

## Task H1 — `An artifact appearance is recorded as one phase transition` (`development-record`, added)

- [x] **Scenario: `An artifact appearance is recorded as one phase transition`** (`@wip` until its RED)
  - Design constraints:
    - **Capture point.** The command dispatcher emits exactly one `phase`
      entry — the report's artifact-appearance transition — labelled with the
      artifact's `STAGE_ARTIFACTS` filename, at the **end of the first recorded
      command that observes the artifact `Done`**. Nothing is emitted before
      the project carries the Givn scaffold; the emission is best-effort and
      never changes the command's outcome or exit code.
    - **Per-artifact dedupe, never a state diff.** An artifact is newly observed
      `Done` when this change's record holds **no** `phase` entry with that same
      filename label yet. A later recorded command that sees the same artifact
      `Done` emits nothing; existing records are never backfilled and no entry
      is ever rewritten.
    - **Gate phases are never transitions.** `givn check --artifact <id>`
      labels its mandatory-check phase with the **artifact id** (`proposal`,
      `tasks`, …), never with a filename, so it can neither dedupe away nor
      stand in for a transition. The scenario asserts both facts: the gate
      entry keeps label `proposal`, and the record still holds exactly one
      `proposal.md` transition.
    - **State → filename mapping** (the design's current-phase table, read in
      the same direction, with the filename at the mapped stage's index):
      `proposal`→`propose`→`proposal.md`; `specs`→`specs`→`specs`;
      `design`→`design`→`design.md`; `arc42-docs`→`arc42`→`arc42.md`;
      `design-review`→`design-review`→`design-review.md`;
      `tasks`→`tasks`→`tasks.md` (a `partial` tasks artifact is not `Done`, so
      it emits nothing); `review`→`review`→`review.md`. Read the filename from
      `STAGE_ARTIFACTS` at the mapped `WORKFLOW_STAGES` index in
      `src/cli/timeline.rs`; make both lookup tables `pub(crate)` instead of
      copying them. An artifact the table does not map emits nothing.
    - **Scope.** The entry belongs to the change scope whose post-command state
      observed the artifact: re-resolve the change from the post-command root
      (its `givn/changes/<id>` directory exists) instead of reusing the
      start-resolved scope, because `givn new <id>` creates the change and its
      `proposal.md` *during* the command, when the recorder's start-resolved
      scope was still the project. The command's own `command` entry keeps its
      start-resolved scope (no rewrite, no backfill). An ambiguous root
      (several active changes) resolves to none: no transition is emitted.
    - **Entry shape.** `kind: "phase"`, `source: "givn"`, `name` and `label`
      both the artifact filename (`HistoryRecorder::record` bounds the label at
      120), `outcome: "ok"`, `duration_ms: None` (an observation instant, never
      an invented duration), parented to this command's entry by the recorder's
      own flush, `data` absent.
    - **Ordering.** Transitions for the artifacts one command first observes
      are emitted in workflow order at the command's end instant; the report's
      derivation orders by `at`, and workflow order breaks the tie
      deterministically.
    - Files: `src/main.rs` (the post-dispatch emission beside
      `recorder.finish`), `src/history.rs` (the scope's existing `phase` labels
      and the post-command scope resolution), `src/cli/timeline.rs` (the two
      tables' `pub(crate)` visibility), `tests/steps/development_record_steps.rs`
      (the scenario's steps), `tests/steps/development_timeline_support.rs`
      (the fixture).
    - RED expected failure: the three new Then phrases are undefined and no
      production command emits a transition.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-record.feature::An artifact appearance is recorded as one phase transition"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Record
          Scenario: An artifact appearance is recorded as one phase transition
           ✘  Given a change "demo" whose proposal.md appears while a recorded command runs
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-record.feature:171:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (4009226) panicked at cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the emission, the dedupe, and the gate-label
        distinction. Files: `src/main.rs`, `src/history.rs`,
        `src/cli/timeline.rs`, `tests/steps/development_record_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Compile first, then
        re-run this scenario → exit 0. Paste it.
        ```
        Feature: Development Record
          Scenario: An artifact appearance is recorded as one phase transition
           ✔  Given a change "demo" whose proposal.md appears while a recorded command runs
           ✔  When the command finishes
           ✔  Then the record holds one phase entry labelled "proposal.md" for "demo"
           ✔  And a later recorded command that sees the same artifact records no second transition
           ✔  And a gate check for the same artifact keeps its artifact id label and is not a transition
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one emission path (post-command state → mapped filename
        → label-absent check → one entry); `cargo fmt --all -- --check` and
        `cargo clippy --locked --all-targets -- -D warnings` green; re-run
        this scenario → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Record
          Scenario: An artifact appearance is recorded as one phase transition
           ✔  Given a change "demo" whose proposal.md appears while a recorded command runs
           ✔  When the command finishes
           ✔  Then the record holds one phase entry labelled "proposal.md" for "demo"
           ✔  And a later recorded command that sees the same artifact records no second transition
           ✔  And a gate check for the same artifact keeps its artifact id label and is not a transition
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `6a12865b` — `feat(development-record): <release note>` with
        the scenario title `An artifact appearance is recorded as one phase
        transition` in the body. Record the revision identifier.
        ```
        commit 6a12865be9f6bc88d9cb87ed996e538b57e85149
        feat(development-record): record an artifact's appearance as one phase transition

        An artifact appearance is recorded as one phase transition

        The command dispatcher emits one phase entry labelled with the artifact's
        STAGE_ARTIFACTS filename at the end of the first recorded command that observes
        the artifact Done, scoped to the post-command change, deduped per label, and
        never standing in for a gate check's artifact-id phase.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-record.feature                     |   2 +-
         src/cli/timeline.rs                                |  40 +++++-
         src/history.rs                                     |  61 ++++++++
         src/main.rs                                        |   1 +
         tests/steps/development_record_steps.rs            | 158 +++++++++++++++++++++
         tests/steps/development_timeline_support.rs        |  24 ++++
         6 files changed, 283 insertions(+), 3 deletions(-)
        ```

## Task H2 — Shared report-model scaffold and text view (no new scenario; the delta's model prerequisite)

- [x] **Shared prerequisite: `cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals`**
  - Design constraints:
    - **Why a shared task.** Every report scenario below asserts facts only the
      new model can carry (phases, statuses, lanes, repeated groups, Fit
      aggregates, the current state, the no-stage group), and the KPI strip's
      three new totals are asserted nowhere else. This task adds the scaffold
      and proves it at the unit boundary; the scenarios below prove the
      behaviours. It introduces no scenario, no feature tag, and no `@e2e`.
    - **`ReportModel` gains** `workflow: Vec<WorkflowPhase>`,
      `current_state: Option<CurrentState>`, and
      `unstaged: Option<WorkflowPhase>` (the same scaffold with `stage: None`
      and no status, absent when every entry has a stage), serialized only
      inside the report. The existing `bands`, `entries`, `stages`, `empty`,
      `aggregates`, and content-budget fields stay; the one phase derivation
      (`stages`) stays the only assignment source.
    - **`WorkflowPhase`** carries: stage, status, first/last recorded instants,
      wall/active/waiting ms, tool count, subagent count,
      `lanes: Vec<Lane>` (session key, visible name, entry ids, first/last,
      measured duration), `repeated: Vec<RepeatedGroup>` (kind, normalized
      label, count, combined measured duration, entry ids), and
      `aggregates: Vec<PhaseAggregate>` (Fit-zoom bucket bounds, entry ids,
      count, combined measured duration).
    - **`CurrentState`** carries the change stage label, the current workflow
      phase, the next required artifact, and task progress
      (`TasksProgress { total, done }`) where the change state has them. The
      struct is `None` for a topic, the project report, and an archived change.
    - **`Summary` gains** `active_ms: Option<u64>`, `tool_calls: usize`, and
      `cost_total: Option<f64>`. `active` is elapsed minus waiting when both
      are measured, else "not measured". `tool_calls` counts `kind == "tool"`
      entries. `cost_total` sums every recorded real cost — `data.cost.total`
      when `data.cost` is the cost object, `data.cost` when it is a number —
      and is `None` ("unavailable") when the record carries none.
    - **The scaffold holds ids, names, and totals only.** The entries appear
      once in `entries`; the model must never carry a second copy of the
      record. The unit test asserts `entries.len()` is unchanged by the
      scaffold and that every lane/group/aggregate member id exists in
      `entries`.
    - **Text view.** `render_view` prints the current-state line, one line per
      workflow phase (status, wall/active/waiting, tools, subagents, its lane
      names, its repeated-group count, its Fit aggregate count), the
      "no recorded stage transition" line when `unstaged` is present, the
      summary including the six KPI facts, and the empty state. The model
      payload element carries the same facts. The project report keeps its band
      rows instead of the phase stack.
    - **The content budget is unchanged:** `content_budget_bytes: 8_388_608`,
      `content_bytes`, and the omission markers stay; the scaffold never
      re-embeds content.
    - Files: `src/cli/timeline.rs` only (`build_report_model`, the new structs,
      `render_view`, and the `#[cfg(test)] mod tests` fixture).
    - RED expected failure: the unit test does not compile (the fields do not
      exist), so `cargo test` exits non-zero.
  - [x] **RED** — add the unit test under `#[cfg(test)] mod tests` in
        `src/cli/timeline.rs` (a small entry vector: one `proposal.md`
        transition, one tool entry in one session, one numeric cost, one cost
        object, one question wait) and run:
        `cargo test --locked --lib cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals`
        → must exit non-zero. Paste it.
        ```
        error[E0609]: no field `current_state` on type `ReportModel`
        error[E0609]: no field `current_state` on type `ReportModel`
        error[E0609]: no field `workflow` on type `ReportModel`
        error[E0609]: no field `workflow` on type `ReportModel`
        error[E0609]: no field `workflow` on type `ReportModel`
        error[E0609]: no field `active_ms` on type `timeline::Summary`
        error[E0609]: no field `tool_calls` on type `timeline::Summary`
        error[E0609]: no field `cost_total` on type `timeline::Summary`
        error[E0609]: no field `unstaged` on type `ReportModel`
        error: could not compile `givn` (lib test) due to 9 previous errors
        EXIT=101
        ```
  - [x] **GREEN** — add the structs, the builders, the three `Summary` totals,
        the `render_view` lines, and the model payload fields. File:
        `src/cli/timeline.rs`. Re-run the unit test → exit 0. Paste it.
        ```
        running 1 test
        test cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 272 filtered out; finished in 0.00s
        EXIT=0
        ```
  - [x] **REFACTOR** — one phase builder feeding the stack and the unstaged
        group, one lane builder, one group/aggregate builder; `cargo fmt --all
        -- --check` and `cargo clippy --locked --all-targets -- -D warnings`
        green; re-run the unit test → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        running 1 test
        test cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals ... ok

        test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 272 filtered out; finished in 0.00s
        EXIT=0
        ```
  - [x] **COMMIT** — `0d363970` — `feat(development-report): <release note>` with
        the body naming the shared scaffold (model fields, the three KPI totals,
        the text-view lines) and the unit test; no scenario changes. Record the
        revision identifier.
        ```
        commit 0d363970d5e899a053588617380594aaefb37c68
        feat(development-report): carry the hybrid workflow phase scaffold

        Shared prerequisite: cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals

        ReportModel gains the nine WorkflowPhase entries (status, first/last,
        wall/active/waiting, tools, subagents, lanes, repeated groups, Fit aggregates),
        the live CurrentState, and the no-stage group; Summary gains active_ms,
        tool_calls, and cost_total; render_view prints the current-state line, one line
        per phase, the no-stage line, and the six KPI facts. The scaffold holds ids and
        totals only: the record stays embedded once.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         src/cli/timeline.rs | 969 +++++++++++++++++++++++++++++++++++++++++++++++++++-
         1 file changed, 968 insertions(+), 1 deletion(-)
        ```

## Task H3 — `The report shows the empty state for a scope with no recorded activity` (`development-report`, added)

- [x] **Scenario: `The report shows the empty state for a scope with no recorded activity`** (`@wip` until its RED)
  - Design constraints:
    - **The empty state is the whole report root.** For a scope with no
      recorded activity the report renders the heading, the one sentence naming
      the scope, and the labelled way back to the report root — never a blank
      page, an error, or a stack drawn over nothing. This branch is shared with
      the browser tier (HE6).
    - **No workflow phase is drawn.** The delta replaces the old assertion:
      the report draws **no workflow phase** for the scope — the text view
      prints no phase line and no phase name, and the asset draws no phase
      card, no axis, and no lane. `model.empty` gates the branch once; the
      workflow stack and the KPI strip do not render.
    - The workflow order is exactly the nine `WORKFLOW_STAGES`; the empty state
      must not name any of them.
    - **Determinism:** the empty state reads only the record and the embedded
      state snapshot; no wall clock, no network.
    - Files: `src/cli/timeline.rs` (the shared empty-state branch and
      `render_view`) and `tests/steps/development_report_steps.rs` (the new
      Then step replacing `the report draws no timeline axis for it`; that old
      step and its phrase are deleted, not kept as an alias).
    - RED expected failure: `the report draws no workflow phase for it` is
      undefined; the reused empty-state Given/When still build a real empty
      fixture, so the RED is the missing branch assertion, never a vacuous
      setup.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report shows the empty state for a scope with no recorded activity"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report shows the empty state for a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report shows the empty state for "demo"
           ✘  And the report draws no workflow phase for it
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:55:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        4 steps (3 passed, 1 failed)

        thread 'main' (4018819) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the single `empty` branch and the step. Files:
        `src/cli/timeline.rs`, `tests/steps/development_report_steps.rs`.
        Re-run this scenario → exit 0. Paste it.
        ```
        Feature: Development Report
          Scenario: The report shows the empty state for a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report shows the empty state for "demo"
           ✔  And the report draws no workflow phase for it
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one `render_empty_state` used by the report root, and
        the absence of phases expressed by the one `empty` condition; `cargo
        fmt --all -- --check` and `cargo clippy --locked --all-targets --
        -D warnings` green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report shows the empty state for a scope with no recorded activity
           ✔  Given a change "demo" with no recorded activity
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report shows the empty state for "demo"
           ✔  And the report draws no workflow phase for it
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `8b652924` — `feat(development-report): show the empty state and draw no workflow phase` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit 8b652924d7051b6ddd8cc6986cb106c33f52798b
        feat(development-report): show the empty state and draw no workflow phase

        The report shows the empty state for a scope with no recorded activity

        A scope that recorded nothing renders the empty state as its whole report root:
        a heading, one sentence naming the scope, and the labelled way back; no phase
        card, axis, lane, or KPI tile is drawn, and the model's bands stay empty. The
        old no-axis assertion is replaced by the delta's no-workflow-phase phrase.

        The workflow stack is gated once by model.empty, so the nine phases render for
        a scope with recorded activity and never over nothing.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |  2 +-
         tests/steps/development_report_steps.rs            | 28 ++++++++++++++++++----
         2 files changed, 24 insertions(+), 6 deletions(-)
        ```

## Task H4 — `The report derives the current workflow state` (`development-report`, added)

- [x] **Scenario: `The report derives the current workflow state`** (`@wip` until its RED)
  - Design constraints:
    - **Source of the state.** For a change scope whose directory exists at
      `givn/changes/<id>`, `givn timeline` reads the change's derived state
      (`src/state.rs`: `stage_label()`, `next_required()`, task progress, and
      the `.givn-skip` list) through `crate::config::overlay::load_effective()`
      — the same manifest `givn status` uses, addons included — and reports the
      stage label, the next required artifact, and the task counts. The step
      **drives a real change directory whose artifacts produce the state**; no
      step may supply the label.
    - **The artifact-id → phase table** (never by list position):

      | State artifact id | Workflow phase |
      |---|---|
      | `proposal` | `propose` |
      | `specs` | `specs` |
      | `design` | `design` |
      | `arc42-docs` | `arc42` |
      | `design-review` | `design-review` |
      | `tasks` | `tasks`, or `implement` while its status is `partial` |
      | `review` | `review` |

      The current phase is that mapping of `next_required()`; a `partial`
      `tasks` artifact means the change is implementing, so the current phase
      is `implement`. With every required artifact done the current phase is
      `archive`. A `next_required()` artifact outside the table leaves the
      current phase **underived**: the header still prints the stage label and
      the artifact, and no phase is marked active instead of guessing one.
    - **Phase statuses.** `WORKFLOW_STAGES` order (nine phases). Phases before
      the current phase are `done`, the current phase is `active`, phases after
      it are `queued`. A phase whose mapped state artifact is `skipped` is
      `skipped`, which wins over `done` and `queued`; a skipped artifact is
      never the current phase because `next_required()` ignores it. A phase
      with no recorded entries renders its status and **zero recorded
      minutes**, never an invented duration.
    - **Header facts, and only those.** The header states the scope, the stage
      label and the current phase as separate words (so "tasks-done, next
      review" and the active `review` phase are not conflated), the next
      required artifact, the task counts, and the count of done phases of nine.
      It **does not** count review findings or claim a running time — the state
      exposes neither (review F13, accepted residual).
    - **Resolution.** `givn/changes/<id>` existing → live state. Else
      `givn/archive/<id>` existing → archived (all phases `done`, no current
      state; HE6/H7). Neither → the report says "recorded only, no live change
      state" and marks **no** phase `done`, never assuming an archive. A topic
      or the project report has `current_state: None` and the header says "no
      live change state".
    - **Model and text view.** `ReportModel.current_state: Option<CurrentState>`
      (stage label, current phase, next required artifact, task progress) and
      the nine `workflow` statuses; `render_view` prints the current-state line
      and one line per phase carrying its status. The scenario's steps read the
      written file's text view: the stage label verbatim, each phase's status,
      and the absence of any name outside the nine.
    - **Fixture.** A project initialised without the arc42 addon; a real change
      directory holding `proposal.md`, `specs/demo.feature`, `design.md`,
      `design-review.md`, and a `tasks.md` whose boxes are all checked (so the
      artifact is `Done`, not `Partial`, and the task counts are non-zero),
      plus a seeded record whose phase transitions run up to `tasks.md`.
    - Files: `src/cli/timeline.rs` (state resolution, statuses, current state,
      text view), `tests/steps/development_report_steps.rs` (the three Givens
      and the five Thens), `tests/steps/development_timeline_support.rs` (the
      real-change fixture).
    - RED expected failure: the new Given/Then phrases are undefined and the
      model has no current state or statuses.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report derives the current workflow state"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report derives the current workflow state
           ✘  Given a change "demo" whose record holds the workflow phases up to "tasks"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:59:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (4019835) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the mapping, the statuses, the header, and the
        text view. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run → exit 0. Paste
        it.
        ```
        Feature: Development Report
          Scenario: The report derives the current workflow state
           ✔  Given a change "demo" whose record holds the workflow phases up to "tasks"
           ✔  And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the current stage "tasks-done"
           ✔  And the report marks the phases before "review" done
           ✔  And the report marks "review" active
           ✔  And the report marks "archive" queued
           ✔  And the report names no phase the workflow does not have
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one id→phase mapping and one status pass shared by the
        header, the stack, and the text view; `cargo fmt --all -- --check` and
        `cargo clippy --locked --all-targets -- -D warnings` green; re-run →
        still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report derives the current workflow state
           ✔  Given a change "demo" whose record holds the workflow phases up to "tasks"
           ✔  And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names the current stage "tasks-done"
           ✔  And the report marks the phases before "review" done
           ✔  And the report marks "review" active
           ✔  And the report marks "archive" queued
           ✔  And the report names no phase the workflow does not have
        [Summary]
        1 feature
        1 scenario (1 passed)
        8 steps (8 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `130d2066` — `feat(development-report): derive the current workflow state from a live change` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit 130d20661dbf65e9eaf0c9590ede8a91a89163c1
        feat(development-report): derive the current workflow state from a live change

        The report derives the current workflow state

        The fixture drives a real change directory whose artifact files make the
        production state derive tasks-done with review next; the written report states
        the stage label, the mapped current phase, the next artifact, and the task
        counts, and marks the phases before review done, review active, and archive
        queued. The header names the stage and the phase separately, and no phase
        outside the nine is ever named.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |   2 +-
         tests/steps/development_report_steps.rs            | 192 +++++++++++++++++++++
         tests/steps/development_timeline_support.rs        |  49 ++++++
         3 files changed, 242 insertions(+), 1 deletion(-)
        ```

## Task H5 — `The report lays parallel sessions side by side` (`development-report`, added)

- [x] **Scenario: `The report lays parallel sessions side by side`** (`@wip` until its RED)
  - Design constraints:
    - **One lane per distinct recorded session** inside a phase; entries whose
      `session` the record does not name go to the **unattributed lane** (an
      absent session key; the design's own word `unattributed` is its visible
      name).
    - **Naming rule (review F1, R-93).** A lane is named by a recorded subagent
      span **only when the span's `data.subagent` equals the lane's session
      id** (opencode's child-session flush); it then carries that span's
      `data.agent` definition as well. Otherwise the lane is named by the
      recorded `session` entry's `data.title` when one exists (the `session`
      kind entry for that session id), else by the session's **short id** —
      the first 8 characters of the session id, the whole id when shorter.
    - **A lane that merely contains a subagent span keeps its own name.** The
      span is drawn inside that lane and never renames it, and **no "subagent
      lane" is created** for a span the record places in the spawning session.
    - **Tool calls are never assigned to a subagent the record does not name.**
      omp and Maki report one session, so their subagent spans sit in the
      agent's lane; Claude's subagent transcript is read for usage, not for
      tool entries; opencode's flush carries one session id for its whole
      batch, so a flush that mixed sessions attributes its entries to the
      flush's session id. The report **states each limit instead of inventing
      per-entry parentage**: when a lane holds a subagent span with no
      attributable tool calls, the text view prints the R-93 sentence (no tool
      call was attributable to that span) and the asset renders the same
      statement in the selected span's log.
    - **Lanes beyond the four defined hues cycle those hues**; the lane name,
      never the hue, carries identity.
    - **Model.** `WorkflowPhase.lanes: Vec<Lane>` with session key, visible
      name, member entry ids, first/last, and measured duration; entries appear
      once in `entries`. The text view prints the phase's lane names, so the
      scenario reads them from the written file.
    - **Fixture.** One phase (after a `tasks.md` transition, i.e. `implement`)
      holding two sessions: a subagent's own recorded session whose `session`
      id matches a subagent span's `data.subagent` and whose `data.agent` names
      `Architect.Scout` (its own entries carry that session id); and a spawning
      session whose own entries carry its session id and which holds a subagent
      span the record places in it; plus one entry with no `session` field and
      one tool call under the subagent's session id so the attribution is
      observable.
    - Files: `src/cli/timeline.rs`, `tests/steps/development_report_steps.rs`,
      `tests/steps/development_timeline_support.rs` (the two-session fixture and
      its `session` entries with `data.title`).
    - RED expected failure: the six new Given/Then phrases are undefined and no
      lane scaffold exists.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report lays parallel sessions side by side"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report lays parallel sessions side by side
           ✘  Given a change "demo" whose record holds one phase with two sessions
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:70:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (4021708) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the lane builder, the naming rule, the
        unattributed lane, and the R-93 statement. Files:
        `src/cli/timeline.rs`, `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run → exit 0. Paste
        it.
        ```
        Feature: Development Report
          Scenario: The report lays parallel sessions side by side
           ✔  Given a change "demo" whose record holds one phase with two sessions
           ✔  And one session is a subagent's own recorded session named "Architect.Scout"
           ✔  And one session holds a subagent span the record places in the spawning session
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report draws one lane per session beside each other in that phase
           ✔  And the lane named "Architect.Scout" carries that subagent's own entries
           ✔  And the spawning session's lane keeps its own name and carries the subagent span
           ✔  And an entry whose session the record does not name stays in the unattributed lane
           ✔  And the report never assigns an entry to a subagent the record does not name
           ✔  And the report states when no tool call was attributable to a subagent span
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one `lane_for` naming function (span/session-id match →
        title → short id) and one attribution-limit derivation; `cargo fmt
        --all -- --check` and `cargo clippy --locked --all-targets --
        -D warnings` green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report lays parallel sessions side by side
           ✔  Given a change "demo" whose record holds one phase with two sessions
           ✔  And one session is a subagent's own recorded session named "Architect.Scout"
           ✔  And one session holds a subagent span the record places in the spawning session
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report draws one lane per session beside each other in that phase
           ✔  And the lane named "Architect.Scout" carries that subagent's own entries
           ✔  And the spawning session's lane keeps its own name and carries the subagent span
           ✔  And an entry whose session the record does not name stays in the unattributed lane
           ✔  And the report never assigns an entry to a subagent the record does not name
           ✔  And the report states when no tool call was attributable to a subagent span
        [Summary]
        1 feature
        1 scenario (1 passed)
        10 steps (10 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `85564850` — `feat(development-report): lay parallel sessions side by side` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit 855648507971171d7c5b60c04e0f2b5e63f73201
        feat(development-report): lay parallel sessions side by side

        The report lays parallel sessions side by side

        A lane is named by a recorded subagent span only when the span's data.subagent
        equals the lane's session id; otherwise the session title, then the short id.
        A spawning session's lane keeps its own name and carries the span, tool calls
        are never attributed to a subagent the record does not name, and the text view
        states the R-93 limit for every span whose child session flushed no tool call.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |   2 +-
         src/cli/timeline.rs                                |  76 +++-
         tests/steps/development_report_steps.rs            | 397 +++++++++++++++++++++
         3 files changed, 464 insertions(+), 11 deletions(-)
        ```

## Task H6 — `The report marks a skipped workflow phase` (`development-report`, added)

- [x] **Scenario: `The report marks a skipped workflow phase`** (`@wip` until its RED)
  - Design constraints:
    - **Skip source.** The state's `.givn-skip` list (`src/state.rs`:
      `is_skipped`, `ArtifactStatus::Skipped`) makes the mapped phase
      `skipped`. The fixture enables the arc42 addon (`givn addons enable
      arc42`) so `arc42-docs` exists in the effective manifest, then writes
      `.givn-skip` with `arc42-docs`; the report reads the real state through
      `load_effective()`.
    - **Precedence.** `skipped` wins over `done` and `queued`. A skipped
      artifact is never the current phase because `next_required()` ignores it;
      the scenario's state stays `tasks-done` with next required `review`.
    - **Never both.** The report marks `arc42` skipped and **not** done — one
      status per phase, no two labels.
    - **Phase with no recorded entries** still renders its status and zero
      recorded minutes (the skipped phase may have no entries).
    - The workflow is exactly the nine `WORKFLOW_STAGES`; `arc42` is the
      mapped phase for `arc42-docs`.
    - Files: `src/cli/timeline.rs` (skip precedence in the one status pass),
      `tests/steps/development_report_steps.rs` (the skip Given and the two
      Thens), `tests/steps/development_timeline_support.rs` (the addon + skip
      fixture).
    - RED expected failure: the skip Given and the two Then phrases are
      undefined; the current model has no `skipped` status.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report marks a skipped workflow phase"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report marks a skipped workflow phase
           ✔  Given a change "demo" whose record holds the workflow phases up to "tasks"
           ✔  And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
           ✘  And the change skips the "arc42-docs" artifact
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:85:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (4024418) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement skip precedence and the steps. Files:
        `src/cli/timeline.rs`, `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run → exit 0. Paste
        it.
        ```
        Feature: Development Report
          Scenario: The report marks a skipped workflow phase
           ✔  Given a change "demo" whose record holds the workflow phases up to "tasks"
           ✔  And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
           ✔  And the change skips the "arc42-docs" artifact
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report marks "arc42" skipped
           ✔  And the report does not mark "arc42" done
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — the status pass stays one function (skipped → done →
        active → queued precedence in one place); `cargo fmt --all --
        --check` and `cargo clippy --locked --all-targets -- -D warnings`
        green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report marks a skipped workflow phase
           ✔  Given a change "demo" whose record holds the workflow phases up to "tasks"
           ✔  And the change's artifacts make its derived state "tasks-done" with the next required artifact "review"
           ✔  And the change skips the "arc42-docs" artifact
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report marks "arc42" skipped
           ✔  And the report does not mark "arc42" done
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `d4dadf61` — `feat(development-report): mark a skipped workflow phase` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit d4dadf61d11aff64b7483a85cac1ce221899f483
        feat(development-report): mark a skipped workflow phase

        The report marks a skipped workflow phase

        A change that records an artifact in .givn-skip derives that artifact skipped
        through the real state, and the report marks the mapped phase skipped instead
        of done or queued: one status per phase, the skipped artifact never current
        because next_required ignores it, and the state stays tasks-done with review
        next.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |  2 +-
         tests/steps/development_report_steps.rs            | 86 ++++++++++++++++++++++
         2 files changed, 87 insertions(+), 1 deletion(-)
        ```

## Task H7 — `The report of an archived change marks every phase done` (`development-report`, added)

- [x] **Scenario: `The report of an archived change marks every phase done`** (`@wip` until its RED)
  - Design constraints:
    - **Archive resolution.** A change id whose `givn/changes/<id>` directory
      does **not** exist is resolved against `givn/archive/<id>`. When the
      archive directory exists the change is **archived**: every one of the
      nine phases is `done`, `current_state` is `None`, and there is **no**
      next required artifact. When neither directory exists the report says
      "recorded only, no live change state" and marks **no** phase done —
      never assume an archive for a missing directory.
    - **Archived header.** The header says "no live change state"; it prints no
      stage label, no next artifact, and no task counts, because the archived
      change exposes no live state.
    - **Every phase done, including empty ones.** A phase with no recorded
      entries is still `done` (archived) and shows zero recorded minutes; no
      status is invented from the record.
    - The scenario asserts three things against the written file: no live
      change state is named, every workflow phase reads done, and no next
      required artifact is named.
    - **Fixture.** `givn/archive/demo/` exists (a real archived change
      directory), `givn/changes/demo` does not, and the seeded record holds
      phase transitions up to `review.md`; the state is never read from a live
      change.
    - Files: `src/cli/timeline.rs` (the archive/neither-dir resolution and the
      archived statuses), `tests/steps/development_report_steps.rs` (the
      archived Given and the three Thens),
      `tests/steps/development_timeline_support.rs` (the archive fixture).
    - RED expected failure: the archived Given and the three Then phrases are
      undefined; the model has no archived resolution.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report of an archived change marks every phase done"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report of an archived change marks every phase done
           ✘  Given an archived change "demo" whose record holds the workflow phases up to "review"
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:92:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (4026109) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the three-way resolution (live / archived /
        neither) and the archived statuses. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run → exit 0. Paste
        it.
        ```
        Feature: Development Report
          Scenario: The report of an archived change marks every phase done
           ✔  Given an archived change "demo" whose record holds the workflow phases up to "review"
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names no live change state
           ✔  And the report marks every workflow phase done
           ✔  And the report names no next required artifact
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one `resolve_change_state` returning live/archived/
        absent, read by the header and the status pass; `cargo fmt --all --
        --check` and `cargo clippy --locked --all-targets -- -D warnings`
        green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report of an archived change marks every phase done
           ✔  Given an archived change "demo" whose record holds the workflow phases up to "review"
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names no live change state
           ✔  And the report marks every workflow phase done
           ✔  And the report names no next required artifact
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `4b6c497b` — `feat(development-report): mark every phase done for an archived change` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit 4b6c497bba1f71a264bd2667a10708445b7124fa
        feat(development-report): mark every phase done for an archived change

        The report of an archived change marks every phase done

        A change id with no live directory but a givn/archive/<id> directory resolves
        to an archived change: every one of the nine phases reads done — including a
        phase with no recorded entries, which states zero recorded minutes — and the
        header states that no live change state exists, naming no stage label, next
        artifact, or task count.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |  2 +-
         tests/steps/development_report_steps.rs            | 93 ++++++++++++++++++++++
         tests/steps/development_timeline_support.rs        | 34 ++++++++
         3 files changed, 128 insertions(+), 1 deletion(-)
        ```

## Task H8 — `The report states a record with no stage transition` (`development-report`, added)

- [x] **Scenario: `The report states a record with no stage transition`** (`@wip` until its RED)
  - Design constraints:
    - **Phase derivation rule (the one derivation).** Givn records one `phase`
      entry when an artifact file of the change appears — the dispatcher emits
      it (Task H1), labelled with the artifact's `STAGE_ARTIFACTS` filename —
      and an entry belongs to the stage of the **latest such transition at or
      before its own `at`**. The gate entry `givn check --artifact <id>` keeps
      the artifact id label and is **never** a transition.
    - **No-stage group.** Entries the record assigns to no stage belong to
      **one** "no recorded stage transition" group rendered below the phase
      stack, with the same lane timeline, and the report **states the gap**.
      They are never placed under a phase the record does not support and
      never under a guessed opening phase.
    - **Opening-stage rule.** Entries recorded before the band's first
      transition stay in the opening stage (`propose`) **only when that first
      transition is `proposal.md`** (the record starts where the workflow
      starts). When the first transition is later, those entries belong to the
      no-stage group. A scope with no transitions gets that one group instead
      of an invented phase name.
    - **Known defect this task must fix:** `stage_assignments` in
      `src/cli/timeline.rs` currently places every pre-first-transition entry
      in `WORKFLOW_STAGES[0]` unconditionally. Replace that with the rule
      above; the later-first-transition case is asserted by a production unit
      test in the same module (no new scenario, no feature edit), because the
      delta's scenario covers the no-transition case and the rule must not be
      lost.
    - **Model.** `ReportModel.unstaged: Option<WorkflowPhase>` — the same
      scaffold with `stage: None` and **no status** — absent when every entry
      has a stage; `stages` entries for unstaged work stay `None` (no invented
      name). The text view prints the group (its entry count) and the exact gap
      sentence; it prints **no** phase line for that work.
    - **Fixture.** A change profile whose record holds entries (steps/tools)
      and **no** `phase` transition at all; the report must print the gap and
      group the work rather than naming `propose`.
    - Files: `src/cli/timeline.rs` (the derivation fix, `unstaged`, the text
      view, the unit test), `tests/steps/development_report_steps.rs` (the
      Given and the three Thens),
      `tests/steps/development_timeline_support.rs` (the no-transition
      fixture).
    - RED expected failure: the three Then phrases are undefined and the
      current derivation invents `propose` for the work.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report states a record with no stage transition"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report states a record with no stage transition
           ✘  Given a change "demo" whose record holds no artifact transition
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:100:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (4026926) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — fix the derivation, add `unstaged`, print the gap, and add
        the later-first unit assertion. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run this scenario →
        exit 0 and the unit test → exit 0. Paste both.
        ```
        Feature: Development Report
          Scenario: The report states a record with no stage transition
           ✔  Given a change "demo" whose record holds no artifact transition
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report groups the recorded work under no workflow phase
           ✔  And the report states that the record carries no stage transition
           ✔  And the report names no phase for that recorded work
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
        ```
        running 2 tests
        test cli::timeline::tests::entries_before_a_later_first_transition_belong_to_no_stage ... ok
        test cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals ... ok

        test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 272 filtered out; finished in 0.00s
        EXIT=0
        ```
  - [x] **REFACTOR** — the derivation stays one function returning
        `Option<&'static str>` per entry, read by `stages`, `workflow`, and
        `unstaged`; `cargo fmt --all -- --check` and `cargo clippy --locked
        --all-targets -- -D warnings` green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report states a record with no stage transition
           ✔  Given a change "demo" whose record holds no artifact transition
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report groups the recorded work under no workflow phase
           ✔  And the report states that the record carries no stage transition
           ✔  And the report names no phase for that recorded work
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        running 2 tests
        test cli::timeline::tests::entries_before_a_later_first_transition_belong_to_no_stage ... ok
        test cli::timeline::tests::hybrid_model_carries_the_phase_scaffold_and_kpi_totals ... ok

        test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 272 filtered out; finished in 0.00s
        EXIT=0
        ```
  - [x] **COMMIT** — `1839d419` — `feat(development-report): state a record with no stage transition` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit 1839d419b8b024fe905d4f7eb089f1f4550d6dd6
        feat(development-report): state a record with no stage transition

        The report states a record with no stage transition

        The one phase derivation now keeps pre-first-transition entries in the opening
        stage only when the first transition is proposal.md; a later first transition
        leaves them, like a record with no transition at all, in one no-stage group
        rendered below the phase stack with its entry count and the exact gap sentence.
        No workflow phase is invented for that work. A production unit test pins the
        later-first-transition rule.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |   2 +-
         src/cli/timeline.rs                                | 145 ++++++++++++++++++---
         tests/steps/development_report_steps.rs            | 127 ++++++++++++++++++
         3 files changed, 257 insertions(+), 17 deletions(-)
        ```

## Task H9 — `The report groups repeated steps` (`development-report`, added)

- [x] **Scenario: `The report groups repeated steps`** (`@wip` until its RED)
  - Design constraints:
    - **Grouping rule.** Within a phase, entries with the **same kind and the
      same normalized label** (trimmed, whitespace collapsed) are grouped when
      the group holds **at least five** entries. The report names the **count**
      and the **combined measured duration**.
    - **Different kinds never merge**, and a group **never hides an entry from
      the ranking**: the ranking still lists every measured entry, one row
      each, alongside the group.
    - **Model.** `WorkflowPhase.repeated: Vec<RepeatedGroup>` (kind, normalized
      label, count, combined measured duration, entry ids); the group holds
      ids and totals only, never a second copy of the entries. The text view
      prints each phase's repeated-group count; the asset marks the group's
      members.
    - **Phase scope.** Groups are computed per phase from the entries the
      phase derivation assigned to it — never across phases; the same command
      repeated under two phases yields one group per phase.
    - **Fixture.** Six identical `command` entries in one phase (after the
      `tasks.md` transition, i.e. `implement`) with distinct measured
      durations, plus the ranking's other measured entries; the steps assert
      one group, its count 6, its combined duration (the sum), and that every
      measured entry is still ranked.
    - Files: `src/cli/timeline.rs` (the grouping pass and the text view),
      `tests/steps/development_report_steps.rs` (the Given and the two Thens),
      `tests/steps/development_timeline_support.rs` (the six-command fixture).
    - RED expected failure: the Given and the two Then phrases are undefined;
      no group scaffold exists.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::The report groups repeated steps"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: The report groups repeated steps
           ✘  Given a change "demo" whose record holds the same command six times in one phase
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:108:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)

        thread 'main' (4029934) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the normalized-label grouping and the text-view
        count. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run → exit 0. Paste
        it.
        ```
        Feature: Development Report
          Scenario: The report groups repeated steps
           ✔  Given a change "demo" whose record holds the same command six times in one phase
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names one repeated-step group with its count and combined duration
           ✔  And the report still ranks every measured entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one `normalized_label` used by the grouping and the
        ranking rows; `cargo fmt --all -- --check` and `cargo clippy --locked
        --all-targets -- -D warnings` green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: The report groups repeated steps
           ✔  Given a change "demo" whose record holds the same command six times in one phase
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report names one repeated-step group with its count and combined duration
           ✔  And the report still ranks every measured entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        4 steps (4 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `bef49d86` — `feat(development-report): group repeated steps within a phase` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit bef49d8662b3189b232a3e7507d6b2937450625e
        feat(development-report): group repeated steps within a phase

        The report groups repeated steps

        Entries of the same kind and the same normalized label are grouped when the
        group holds at least five entries; the phase carries the group's count, its
        combined measured duration, and its member ids, and the text view names them.
        Different kinds never merge, the group never hides an entry from the ranking,
        and the one normalized label now feeds both the grouping and the ranking rows.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |   2 +-
         src/cli/timeline.rs                                |  24 +++-
         tests/steps/development_report_steps.rs            | 150 +++++++++++++++++++++
         3 files changed, 168 insertions(+), 8 deletions(-)
        ```

## Task H10 — `A dense record is reported as aggregates instead of raw ticks` (`development-report`, added)

- [x] **Scenario: `A dense record is reported as aggregates instead of raw ticks`** (`@wip` until its RED)
  - Design constraints:
    - **Threshold.** Within an expanded phase, a lane never draws one shape per
      entry when that would be unreadable. The threshold is the lane's
      **time-per-pixel at the current zoom**, applied **at every level and not
      only at Fit**. Entries shorter than that threshold — dense agent ticks
      and short steps in particular — collapse into **one aggregate tick per
      time bucket**; the tick names **how many entries it covers and their
      combined measured duration**. Individual ticks replace their aggregate as
      the zoom narrows and the threshold permits.
    - **Bounded DOM.** A record of tens of thousands of entries renders a
      bounded number of shapes at Fit, 1 h, and 15 m; the timeline's scroll
      height is capped and the lane scrolls, so 15 m never forces one tick per
      sub-second entry.
    - **Full-record summary.** The summary and the longest-step ranking always
      cover the **full record**, never only the visible window or the
      aggregates; clicking an aggregate zooms the phase to that bucket instead
      of opening a log entry that stands for many.
    - **Model.** `WorkflowPhase.aggregates: Vec<PhaseAggregate>` carries the
      **Fit-zoom** bucket bounds, member entry ids, count, and combined
      measured duration; the asset re-applies the same criterion for the
      narrower zooms. The text view prints each phase's Fit aggregate count so
      the regular scenario reads it from the written file.
    - **Fixture.** A record holding hundreds of short `tool` activities in one
      phase (after a transition) plus a few ordinary entries, so the aggregate
      covers the dense entries while the summary still counts every entry.
    - The delta's phrase is **aggregate tick on the phase lane** (not the old
      "aggregate marker"); the old step phrases
      `the report collapses the short agent activities into aggregate markers`
      and `each aggregate marker names how many entries it covers and their
      total duration` are replaced, not aliased.
    - Files: `src/cli/timeline.rs` (the per-phase bucket derivation shared with
      the axis scale and the text view),
      `tests/steps/development_report_steps.rs` (the Given and the three
      Thens), `tests/steps/development_timeline_support.rs` (the dense
      fixture).
    - RED expected failure: the three new Then phrases are undefined; the
      model carries no per-phase aggregates.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::A dense record is reported as aggregates instead of raw ticks"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: A dense record is reported as aggregates instead of raw ticks
           ✔  Given a change "demo" whose record holds hundreds of short agent activities
           ✔  When the developer generates the development report for "demo"
           ✘  Then the report collapses the short agent activities into aggregate ticks on the phase lane
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:117:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        3 steps (2 passed, 1 failed)

        thread 'main' (4033412) panicked at /home/buster/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/cucumber-0.23.0/src/cucumber.rs:1243:13:
        1 step failed
        error: test failed, to rerun pass `--test features_runner`
        EXIT=101
        ```
  - [x] **GREEN** — implement the per-phase Fit buckets and the three
        assertions. Files: `src/cli/timeline.rs`,
        `tests/steps/development_report_steps.rs`,
        `tests/steps/development_timeline_support.rs`. Re-run → exit 0. Paste
        it.
        ```
        Feature: Development Report
          Scenario: A dense record is reported as aggregates instead of raw ticks
           ✔  Given a change "demo" whose record holds hundreds of short agent activities
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report collapses the short agent activities into aggregate ticks on the phase lane
           ✔  And each aggregate tick names how many entries it covers and their total duration
           ✔  And the report summary still counts every recorded entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **REFACTOR** — one bucket criterion (`Scale::time_per_pixel`) feeding
        the model's aggregate list and the asset's every-zoom threshold;
        `cargo fmt --all -- --check` and `cargo clippy --locked --all-targets
        -- -D warnings` green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: A dense record is reported as aggregates instead of raw ticks
           ✔  Given a change "demo" whose record holds hundreds of short agent activities
           ✔  When the developer generates the development report for "demo"
           ✔  Then the report collapses the short agent activities into aggregate ticks on the phase lane
           ✔  And each aggregate tick names how many entries it covers and their total duration
           ✔  And the report summary still counts every recorded entry
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `55078235` — `feat(development-report): collapse dense entries into phase aggregate ticks` with
        the scenario title in the body. Record the revision identifier.
        ```
        commit 55078235b16ec123b66831517611e38369ae3ee5
        feat(development-report): collapse dense entries into phase aggregate ticks

        A dense record is reported as aggregates instead of raw ticks

        Each workflow phase carries its Fit-zoom aggregate ticks: the entries shorter
        than the lane's time-per-pixel bucket into one tick per time bucket, and each
        tick names how many entries it covers and their combined measured duration (or
        states that the record measured none). The text view prints every tick and the
        phase's Fit aggregate count, while the summary still counts every recorded
        entry. The delta's phase-lane phrases replace the old marker phrases.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                     |   2 +-
         src/cli/timeline.rs                                |  14 +
         tests/steps/development_report_steps.rs            | 296 +++++++++++-----------
         3 files changed, 159 insertions(+), 153 deletions(-)
        ```

## Task HE1 — `Consumer generates the development report for a change` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer generates the development report for a change`** (`@wip` until its RED)
  - Design constraints:
    - Matrix row 4: real interface **CLI**; driving mechanism `common::run_givn`:
      `givn timeline --change demo --out report.html`; the assertion reads the
      **written file**. The e2e runner, browser driver, and evidence helper are
      Task E1's; this task adds no setup.
    - **Phase derivation, asserted concretely:** the record holds artifact
      transitions up to **`tasks.md`**, and the step recorded after that
      transition must land in the **`implement`** phase (the artifact file of
      stage N opens stage N+1; `tasks.md` opens `implement`). The
      gate-entry distinction is Task H1's; this scenario's fixture seeds the
      transitions in the record.
    - **Workflow names, not "recorded phases":** the delta's Then is
      `the report file exists and names the workflow phases` — the text view
      must name all **nine** `WORKFLOW_STAGES` in order. The old step phrase
      `the report file exists and names the recorded phases` and its step are
      replaced, not aliased.
    - Assertions: exit 0; the printed path of the written report; the file
      exists and names the nine workflow phases; the post-transition step is
      placed in `implement`. The default-path assertion (flagless
      `givn timeline --change demo` writes `givn/history/reports/demo.html`) is
      allowed by the same decision and stays from Task E5.
    - Steps in `tests/steps/development_report_e2e_steps.rs`; the seeded record
      comes from the shared builder in
      `tests/steps/development_timeline_support.rs`; the evidence capture is
      the shared `commit_evidence(world, GENERATE_SCENARIO)` helper — the
      title is untouched, so the slug stays
      `consumer-generates-the-development-report-for-a-change`.
    - **Evidence:** refresh
      `evidence/visual/consumer-generates-the-development-report-for-a-change/{mobile.png,desktop.png}`
      in place — the report this scenario wrote, rendered in the browser at
      360x780 and 1920x1080, animations disabled, both files non-empty.
    - Files: `src/cli/timeline.rs` (the nine-phase text view),
      `tests/steps/development_report_e2e_steps.rs`,
      `tests/steps/development_timeline_support.rs`, the two refreshed PNGs.
    - RED expected failure: `names the workflow phases` is undefined (the old
      phrase binds) and the text view cannot yet name a phase stack.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer generates the development report for a change"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer generates the development report for a change
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
           ✔  When the user runs `givn timeline --change demo --out report.html`
           ✔  Then the command exits 0
           ✔  And the command prints the path of the written report
           ✘  And the report file exists and names the workflow phases
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:144:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        6 steps (5 passed, 1 failed)
        EXIT=101
        ```
  - [x] **GREEN** — implement the workflow-phase text view and the step. Files:
        `src/cli/timeline.rs`, `tests/steps/development_report_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`, plus the refreshed pair.
        Compile first, re-run → exit 0. Paste it and the refreshed capture
        list.
        ```
        Feature: Development Report
          Scenario: Consumer generates the development report for a change
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
           ✔  When the user runs `givn timeline --change demo --out report.html`
           ✔  Then the command exits 0
           ✔  And the command prints the path of the written report
           ✔  And the report file exists and names the workflow phases
           ✔  And the report file places the step recorded after that transition in the "implement" phase
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        EXIT=0

        consumer-generates-the-development-report-for-a-change/desktop.png 187683 bytes
        consumer-generates-the-development-report-for-a-change/mobile.png   54146 bytes
        ```
  - [x] **REFACTOR** — one phase-line renderer shared by the text view's stack
        and the no-stage group; `cargo fmt --all -- --check` and `cargo clippy
        --locked --all-targets -- -D warnings` green; re-run → still exit 0.
        Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0

        Feature: Development Report
          Scenario: Consumer generates the development report for a change
           ✔  Given a project root with givn/ initialised
           ✔  And a change "demo" with a recorded history whose last recorded artifact transition is "tasks.md"
           ✔  When the user runs `givn timeline --change demo --out report.html`
           ✔  Then the command exits 0
           ✔  And the command prints the path of the written report
           ✔  And the report file exists and names the workflow phases
           ✔  And the report file places the step recorded after that transition in the "implement" phase
        [Summary]
        1 feature
        1 scenario (1 passed)
        7 steps (7 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `70a0750e` — `test(e2e): <release note>` with the scenario
        title in the body (including the refreshed evidence pair). Record the
        revision identifier.
        ```
        commit 70a0750e
        test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer generates the development report for a change

        The written report's assertions read the hybrid workflow text view: the report
        names the change workflow's nine phases in order, and the step recorded after
        the tasks.md transition is placed in the implement phase. The evidence pair is
        refreshed in place.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../consumer-generates-the-development-report-for-a-change/desktop.png | Bin 96476 -> 187683 bytes
         .../consumer-generates-the-development-report-for-a-change/mobile.png  | Bin 54073 -> 54146 bytes
         .../development-report.feature                |   2 +-
         tests/steps/development_report_e2e_steps.rs   |  74 ++++++++++++-----------
         tests/steps/development_timeline_support.rs   |   9 +--
         5 files changed, 40 insertions(+), 45 deletions(-)
        ```

## Task HE2 — `Consumer opens the generated development report in a browser` (`@e2e`, `development-report`, Scenario Outline, `change` and `topic` rows) — the asset rewrite

- [x] **Scenario: `Consumer opens the generated development report in a browser`** (`@wip` until its RED)
  - Design constraints:
    - Matrix row 5: real interface **Web UI**; Playwright opens
      `file://<root>/report.html` and reads the **rendered** heading, the
      current-state line, the workflow phase list, and the longest recorded
      step's duration from the DOM. One outline with a `change` row and a
      `topic` row: one normalized action with input variants, one inventory
      row, one evidence set.
    - **This task carries the asset rewrite to the hybrid workflow view**
      (`assets/timeline/report.html`): the header (scope, stage label and
      current phase as separate words, done phases of nine), the KPI strip
      (elapsed, active, waiting, tool calls, subagent spans, cost — "not
      measured"/"unavailable" where the record has none), and the **workflow
      stack top to bottom** (the nine phase cards in `WORKFLOW_STAGES` order,
      each with status, wall/active/waiting, tool and subagent counts, its lane
      names, and an activity sparkline). The inline script renders the stack
      from the model payload, never by calling `fetch` and never with
      `Date.now()`.
    - **Then assertions:** the browser shows the report heading and the
      **current workflow state**; the workflow phases appear **top to bottom**
      in the nine-phase order; the longest recorded step appears with its
      duration; at the **mobile viewport (360x780)** the page lays out in **one
      column without page-level horizontal scrolling** —
      `document.documentElement.scrollWidth <= window.innerWidth` — with the KPI
      strip in two columns and the phase cards full width. The lanes of an
      expanded phase may still scroll horizontally **inside** their card.
    - **Accessibility (Visual Design Contract):** a real `title`, one `h1`,
      `header`/`main` landmarks; the phase expander is a button carrying
      `aria-expanded`; every control is Tab-reachable with a visible focus ring;
      the driver emulates `prefers-reduced-motion: reduce` and screenshots are
      captured with animations disabled.
    - **Fixtures:** the `change` row creates a real change directory whose
      artifacts yield a live derived state (the H4 fixture shape); the `topic`
      row has no live change state and its header says so.
    - Steps in `tests/steps/development_report_e2e_steps.rs`; the shared
      builder and the real-change fixture live in
      `tests/steps/development_timeline_support.rs`; capture with the shared
      helper under the untouched title, so the slug stays
      `consumer-opens-the-generated-development-report-in-a-browser`.
    - **Evidence:** refresh
      `evidence/visual/consumer-opens-the-generated-development-report-in-a-browser/{mobile.png,desktop.png}`
      in place (desktop 1920x1080 and mobile 360x780 of the hybrid view,
      animations disabled).
    - Files: `assets/timeline/report.html` (the rewrite: tokens, breakpoints,
      header, KPI strip, phase cards, sparkline, mobile one-column rules),
      `tests/steps/development_report_e2e_steps.rs`,
      `tests/steps/development_timeline_support.rs`, the two refreshed PNGs.
    - RED expected failure: `the report heading and the current workflow state`,
      `the workflow phases top to bottom`, and the mobile-layout assertion are
      undefined; the current asset draws the superseded dense timeline.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the generated development report in a browser"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a change "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✘  Then the browser shows the report heading and the current workflow state
              Step doesn't match any function
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a topic "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✘  Then the browser shows the report heading and the current workflow state
              Step doesn't match any function
        [Summary]
        1 feature
        2 scenarios (2 failed)
        8 steps (6 passed, 2 failed)
        EXIT=101
        ```
  - [x] **GREEN** — rewrite the asset to the hybrid workflow view and implement
        the three Then steps (heading + current state, top-to-bottom phases,
        mobile one-column with no page-level horizontal scrolling). Files:
        `assets/timeline/report.html`, `src/cli/timeline.rs` (only if the
        rendered model needs a field the text view already prints),
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`, plus the refreshed pair.
        `node --check` on the extracted inline layer, compile, re-run → exit 0.
        Paste it.
        ```
        Feature: Development Report
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a change "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the current workflow state
           ✔  And the browser shows the workflow phases top to bottom
           ✔  And the browser shows the longest recorded step with its duration
           ✔  And the browser lays the page out in one column without page-level horizontal scrolling at the mobile viewport
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a topic "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the current workflow state
           ✔  And the browser shows the workflow phases top to bottom
           ✔  And the browser shows the longest recorded step with its duration
           ✔  And the browser lays the page out in one column without page-level horizontal scrolling at the mobile viewport
        [Summary]
        1 feature
        2 scenarios (2 passed)
        14 steps (14 passed)
        EXIT=0

        consumer-opens-the-generated-development-report-in-a-browser/desktop.png 113107 bytes
        consumer-opens-the-generated-development-report-in-a-browser/mobile.png   55909 bytes
        ```
  - [x] **REFACTOR** — one phase-card renderer, one KPI tile renderer, one
        `prefers-reduced-motion` block; `cargo fmt --all -- --check`,
        `cargo clippy --locked --all-targets -- -D warnings`, and `node --check`
        green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0
        node --check /tmp/report-inline.js → exit 0

        Feature: Development Report
          Scenario Outline: Consumer opens the generated development report in a browser
           ✔  Given a topic "demo" with a recorded history
           ✔  And the development report for "demo" has been generated
           ✔  When the consumer opens the development report in a browser
           ✔  Then the browser shows the report heading and the current workflow state
           ✔  And the browser shows the workflow phases top to bottom
           ✔  And the browser shows the longest recorded step with its duration
           ✔  And the browser lays the page out in one column without page-level horizontal scrolling at the mobile viewport
        [Summary]
        1 feature
        2 scenarios (2 passed)
        14 steps (14 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `c776da2c` — `test(e2e): <release note>` with the scenario
        title in the body (including the refreshed evidence pair). Record the
        revision identifier.
        ```
        commit c776da2c
        test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer opens the generated development report in a browser

        The report asset is the hybrid workflow view: the current-state header, the
        six-fact KPI strip, the nine workflow phase cards top to bottom with statuses,
        stats, and sparklines, the longest recorded step in the statistics, and the
        one-column mobile layout without page-level horizontal scrolling. The evidence
        pair is refreshed in place.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         assets/timeline/report.html                        | 1794 ++++++++++++---------
         .../consumer-opens-the-generated-development-report-in-a-browser/desktop.png | Bin 73818 -> 113107 bytes
         .../consumer-opens-the-generated-development-report-in-a-browser/mobile.png  | Bin 46204 -> 55909 bytes
         .../development-report.feature                      |   2 +-
         tests/steps/development_report_e2e_steps.rs         |  442 +++---
         5 files changed, 1214 insertions(+), 1024 deletions(-)
        ```

## Task HE3 — `Consumer expands a workflow phase and sees its lanes` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer expands a workflow phase and sees its lanes`** (`@wip` until its RED)
  - Design constraints:
    - Matrix row 6: real interface **Web UI**; Playwright clicks the
      **`implement`** phase expander and reads from the DOM: the phase's
      **vertical timeline with time flowing down**, **one lane per session
      beside each other**, the lane names, and the **Fit**, **1 h**, and
      **15 m** zoom controls (visible text labels, never icons alone).
    - **This scenario is the new interaction** (review F9): the expander changes
      the visible timeline and is its own inventory row, `@e2e` scenario, and
      evidence set. It is **not** part of select and not navigation.
    - **Accessibility/semantics:** the expander is a button carrying
      `aria-expanded`; the expanded phase is a labelled region; each lane's
      **header** (not the lane column containing the ticks) is a focusable
      button whose accessible name carries the lane name and its duration;
      ticks are separate focusable elements and **no button nests another
      control**.
    - **Mobile rule:** after expanding at desktop, resize to 360x780 and assert
      the **expanded phase survives** the layout change (still expanded, still
      one lane per session, lanes scroll horizontally inside the card only).
    - **Fixture:** an open report whose record holds parallel sessions (the H5
      fixture: a subagent-owned session `Architect.Scout` and a spawning
      session holding a subagent span), so the expanded phase has at least two
      lanes.
    - Steps in `tests/steps/development_report_e2e_steps.rs`; capture with the
      shared helper under the **new** title.
    - **Evidence (the only new directory this delta adds):**
      `evidence/visual/consumer-expands-a-workflow-phase-and-sees-its-lanes/{mobile.png,desktop.png}`
      — the expanded phase at 360x780 and 1920x1080, animations disabled, both
      non-empty.
    - Files: `assets/timeline/report.html` (phase expansion, the vertical axis,
      lane columns and lane headers, the per-phase zoom controls and their
      labels), `tests/steps/development_report_e2e_steps.rs`,
      `tests/steps/development_timeline_support.rs`, the two new PNGs.
    - RED expected failure: the Given, When, and three Then phrases are all
      undefined (the scenario is new).
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer expands a workflow phase and sees its lanes"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer expands a workflow phase and sees its lanes
           ✘  Given an open development report with a recorded history holding parallel sessions
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:164:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        1 step (1 failed)
        EXIT=101
        ```
  - [x] **GREEN** — implement expansion, the vertical axis, the lanes, and the
        three controls. Files: `assets/timeline/report.html`,
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`, plus the new pair.
        `node --check`, compile, re-run → exit 0. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer expands a workflow phase and sees its lanes
           ✔  Given an open development report with a recorded history holding parallel sessions
           ✔  When the consumer expands the "implement" phase
           ✔  Then the browser shows that phase's vertical timeline with time flowing down
           ✔  And the browser shows one lane per session beside each other
           ✔  And the browser shows the Fit, 1 h, and 15 m zoom controls
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0

        consumer-expands-a-workflow-phase-and-sees-its-lanes/desktop.png 65829 bytes
        consumer-expands-a-workflow-phase-and-sees-its-lanes/mobile.png   27404 bytes
        ```
  - [x] **REFACTOR** — one `expandPhase`/`renderAxis`/`renderLanes` path shared
        with select and zoom; `cargo fmt --all -- --check`, `cargo clippy
        --locked --all-targets -- -D warnings`, and `node --check` green;
        re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0
        node --check /tmp/report-inline.js → exit 0

        Feature: Development Report
          Scenario: Consumer expands a workflow phase and sees its lanes
           ✔  Given an open development report with a recorded history holding parallel sessions
           ✔  When the consumer expands the "implement" phase
           ✔  Then the browser shows that phase's vertical timeline with time flowing down
           ✔  And the browser shows one lane per session beside each other
           ✔  And the browser shows the Fit, 1 h, and 15 m zoom controls
        [Summary]
        1 feature
        1 scenario (1 passed)
        5 steps (5 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `0e6306aa` — `test(e2e): <release note>` with the scenario
        title in the body (including the new evidence pair). Record the
        revision identifier.
        ```
        commit 0e6306aa
        test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer expands a workflow phase and sees its lanes

        The hybrid report's implement phase expands into a vertical time axis with one
        lane per recorded session side by side, the subagent's own Architect.Scout lane
        and the unattributed lane included, and the per-phase Fit / 1 h / 15 m controls.
        The expanded phase and its lane scroll survive the mobile layout. The new
        evidence directory is committed.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../development-report.feature                       |   2 +-
         .../development_report_e2e_steps.rs                  | 244 ++++++++++++++++++
         .../development_timeline_support.rs                  |  69 +++++
         .../consumer-expands-a-workflow-phase-and-sees-its-lanes/desktop.png | Bin 0 -> 65829 bytes
         .../consumer-expands-a-workflow-phase-and-sees-its-lanes/mobile.png  | Bin 0 -> 27404 bytes
         5 files changed, 315 insertions(+), 1 deletion(-)
        ```

## Task HE4 — `Consumer selects a timeline element and reads its details` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer selects a timeline element and reads its details`** (`@wip` until its RED)
  - Design constraints:
    - Matrix row 7: real interface **Web UI**; Playwright expands the phase,
      then clicks the **`Architect.Scout`** lane header (or focuses it and
      presses `Enter`) and reads the **lane log** from the DOM. The delta
      removed the expand from this scenario's observable behaviour: **select
      only selects**.
    - **Lane log facts:** the selected lane's entries sorted by `at` — tool
      calls, turns, revisions, questions, and the subagent span itself — each
      with its recorded fact list, outcome, duration, and content when
      captured; failed and repeated entries are marked. The Thens assert the
      lane's **start**, its **duration**, its **outcome**, the entry's **tool
      call id**, and the **bounded content region**.
    - **R-93 honesty:** the log never assigns a tool call to a subagent the
      record does not name; when the selected span has no attributable tool
      calls, the log **states** that instead of inventing parentage.
    - **Content budget:** the log embeds content only within the 8 MiB
      `content_budget_bytes`; beyond it the entry reads exactly
      `content omitted; see the record` and the store keeps every byte. The
      content region is one bounded, scrollable, escaped block per named field,
      with its truncation mark; it is not part of the accessible summary line.
    - **Fixture:** the `Architect.Scout` lane is a subagent's own session; its
      span carries `tool_call_id` and one bounded `data.content` field, so the
      same selection reads the tool call id and the content region.
    - Steps in `tests/steps/development_report_e2e_steps.rs`; capture with the
      shared helper under the untouched title, so the slug stays
      `consumer-selects-a-timeline-element-and-reads-its-details`.
    - **Evidence:** refresh
      `evidence/visual/consumer-selects-a-timeline-element-and-reads-its-details/{mobile.png,desktop.png}`
      in place, anchored on the lane log, both viewports, animations disabled.
    - Files: `assets/timeline/report.html` (lane selection, the lane log, the
      fact list, the content region), `tests/steps/development_report_e2e_steps.rs`,
      `tests/steps/development_timeline_support.rs`, the two refreshed PNGs.
    - RED expected failure: the new Given/When/Then phrases are undefined (the
      old `selects the longest recorded step` / `details panel` steps are
      replaced, not aliased).
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer selects a timeline element and reads its details"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✔  Given an open development report with a recorded history holding parallel sessions
           ✘  When the consumer selects the "Architect.Scout" lane
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:173:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        EXIT=101
        ```
  - [x] **GREEN** — implement lane selection and the lane log. Files:
        `assets/timeline/report.html`,
        `tests/steps/development_report_e2e_steps.rs`,
        `tests/steps/development_timeline_support.rs`, plus the refreshed pair.
        `node --check`, compile, re-run → exit 0. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✔  Given an open development report with a recorded history holding parallel sessions
           ✔  When the consumer selects the "Architect.Scout" lane
           ✔  Then the browser shows that lane's log
           ✔  And the log names the lane's start, its duration, and its outcome
           ✔  And the log names the entry's tool call id
           ✔  And the log renders the entry's content in its bounded content region
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0

        consumer-selects-a-timeline-element-and-reads-its-details/desktop.png 95895 bytes
        consumer-selects-a-timeline-element-and-reads-its-details/mobile.png   49534 bytes
        ```
  - [x] **REFACTOR** — one `selectLane`/`renderLog` path and one content-region
        renderer shared with the omission marker; `cargo fmt --all -- --check`,
        `cargo clippy --locked --all-targets -- -D warnings`, and `node --check`
        green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0
        node --check /tmp/report-inline.js → exit 0

        Feature: Development Report
          Scenario: Consumer selects a timeline element and reads its details
           ✔  Given an open development report with a recorded history holding parallel sessions
           ✔  When the consumer selects the "Architect.Scout" lane
           ✔  Then the browser shows that lane's log
           ✔  And the log names the lane's start, its duration, and its outcome
           ✔  And the log names the entry's tool call id
           ✔  And the log renders the entry's content in its bounded content region
        [Summary]
        1 feature
        1 scenario (1 passed)
        6 steps (6 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `2771b25e` — `test(e2e): <release note>` with the scenario
        title in the body (including the refreshed evidence pair). Record the
        revision identifier.
        ```
        commit 2771b25e
        test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer selects a timeline element and reads its details

        Selecting the Architect.Scout lane header renders the lane log: the lane's
        start, duration, and outcome, the entry's tool call id, and the escaped,
        bounded, scrollable content region inside the log. Failed and repeated entries
        are marked, and the R-93 limit is stated when no tool call is attributable.
        The evidence pair is refreshed in place.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../consumer-selects-a-timeline-element-and-reads-its-details/desktop.png | Bin 31 -> 95895 bytes
         .../consumer-selects-a-timeline-element-and-reads-its-details/mobile.png  | Bin 33 -> 49534 bytes
         .../development-report.feature                       |   2 +-
         .../development_report_e2e_steps.rs                  | 382 ++++++++++-----------
         4 files changed, 150 insertions(+), 234 deletions(-)
        ```

## Task HE5 — `Consumer zooms the timeline to a shorter time range` (`@e2e`, `development-report`)

- [x] **Scenario: `Consumer zooms the timeline to a shorter time range`** (`@wip` until its RED)
  - Design constraints:
    - Matrix row 8: real interface **Web UI**; Playwright expands the phase and
      clicks the **`15 m`** control (the next narrower level), then reads the
      **rendered range label** before and after and asserts it **narrowed**.
      There is no global zoom-in button in the hybrid: the control is
      **per phase** (review F14).
    - **Rendered-state evidence:** the assertion reads a value the page's own
      script produced (the visible time range) and observes it change between
      two observations; the report never reads the wall clock (`Date.now()`),
      which is what makes the comparison deterministic.
    - **Every-zoom threshold:** the aggregation threshold (H10) applies at Fit,
      1 h, and **15 m** alike; the lane's scroll height is capped and the lane
      scrolls, so 15 m never forces one tick per sub-second entry. Individual
      ticks replace their aggregate only where the threshold permits.
    - **Motion:** the zoom transition is short and removed entirely under
      `prefers-reduced-motion: reduce` (the driver emulates it, screenshots are
      captured with animations disabled); assert the emulated preference and
      that the zoom then applies instantly.
    - **Mobile rule:** after zooming, resize to 360x780 and assert the **zoom
      state survives** the layout change (the narrowed range is still the
      rendered range).
    - Steps in `tests/steps/development_report_e2e_steps.rs`; capture with the
      shared helper under the untouched title, so the slug stays
      `consumer-zooms-the-timeline-to-a-shorter-time-range`.
    - **Evidence:** refresh
      `evidence/visual/consumer-zooms-the-timeline-to-a-shorter-time-range/{mobile.png,desktop.png}`
      in place at the narrowed range, both viewports, animations disabled.
    - Files: `assets/timeline/report.html` (per-phase zoom state, the range
      label, the every-zoom threshold, the capped lane height),
      `tests/steps/development_report_e2e_steps.rs`, the two refreshed PNGs.
    - RED expected failure: `zooms the expanded phase's timeline in` is
      undefined (the old `zooms the timeline in` step is replaced) and the
      asset has no per-phase control.
  - [x] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer zooms the timeline to a shorter time range"`
        → must exit non-zero. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer zooms the timeline to a shorter time range
           ✔  Given an open development report with a recorded history
           ✘  When the consumer zooms the expanded phase's timeline in
              Step failed:
              Defined: givn/changes/development-timeline/specs/inspect-development-history/development-report.feature:182:5
              Step doesn't match any function
        [Summary]
        1 feature
        1 scenario (1 failed)
        2 steps (1 passed, 1 failed)
        EXIT=101
        ```
  - [x] **GREEN** — implement the per-phase zoom, the rendered range, and the
        before/after assertion. Files: `assets/timeline/report.html`,
        `tests/steps/development_report_e2e_steps.rs`, plus the refreshed pair.
        `node --check`, compile, re-run → exit 0. Paste it.
        ```
        Feature: Development Report
          Scenario: Consumer zooms the timeline to a shorter time range
           ✔  Given an open development report with a recorded history
           ✔  When the consumer zooms the expanded phase's timeline in
           ✔  Then the browser shows a narrower time range than before
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0

        consumer-zooms-the-timeline-to-a-shorter-time-range/desktop.png 65394 bytes
        consumer-zooms-the-timeline-to-a-shorter-time-range/mobile.png   28737 bytes
        ```
  - [x] **REFACTOR** — one range derivation every zoom path goes through, one
        threshold function used at every level; `cargo fmt --all -- --check`,
        `cargo clippy --locked --all-targets -- -D warnings`, and `node --check`
        green; re-run → still exit 0. Paste it.
        ```
        cargo fmt --all -- --check → exit 0
        cargo clippy --locked --all-targets -- -D warnings → Finished, exit 0
        node --check /tmp/report-inline.js → exit 0

        Feature: Development Report
          Scenario: Consumer zooms the timeline to a shorter time range
           ✔  Given an open development report with a recorded history
           ✔  When the consumer zooms the expanded phase's timeline in
           ✔  Then the browser shows a narrower time range than before
        [Summary]
        1 feature
        1 scenario (1 passed)
        3 steps (3 passed)
        EXIT=0
        ```
  - [x] **COMMIT** — `dbcb3a7c` — `test(e2e): <release note>` with the scenario
        title in the body (including the refreshed evidence pair). Record the
        revision identifier.
        ```
        commit dbcb3a7c
        test(e2e): Givn records the development process of every change and ideation — each step it runs, the activity reported by every supported coding-agent harness, question waits, and token usage — and renders it as an interactive timeline report that opens in a browser.

        Consumer zooms the timeline to a shorter time range

        The per-phase 15 m control narrows the expanded implement phase's rendered
        range; the assertion compares the text the page's own script produced before
        and after. The reduced-motion preference removes the transition, the one
        aggregation threshold bounds each lane at every zoom, and the narrowed range
        survives the mobile layout. The evidence pair is refreshed in place.

        Co-Authored-By: deepseek/deepseek-flash/max@omp with Givn 0.9.0

         .../consumer-zooms-the-timeline-to-a-shorter-time-range/desktop.png | Bin 32 -> 65394 bytes
         .../consumer-zooms-the-timeline-to-a-shorter-time-range/mobile.png  | Bin 33 -> 28737 bytes
         .../development-report.feature                       |   2 +-
         .../development_report_e2e_steps.rs                  | 175 +++++++++++-----------
         4 files changed, 92 insertions(+), 116 deletions(-)
        ```

## Task HE6 — `Consumer opens the report of a scope with no recorded activity` (`@e2e`, `development-report`)

- [ ] **Scenario: `Consumer opens the report of a scope with no recorded activity`** (`@wip` until its RED)
  - Design constraints:
    - Matrix row 9: real interface **Web UI**; the scenario generates the report
      for a scope with no recorded activity and reads the rendered **empty-state
      heading** and the **absence of any phase** from the DOM. This is the
      `@e2e` traversal of the contract's declared **`empty`** state.
    - **Then assertions:** the browser shows the empty state for `demo`; the
      browser draws **no workflow phase** for it — `section`/`.phase-card`/
      phases named in the DOM all count zero, and no axis is drawn. The old
      `draws no timeline axis` step is replaced, not aliased.
    - The empty state shares H3's one `empty` branch: heading, one sentence
      naming the scope, the labelled way back to the report root.
    - Steps in `tests/steps/development_report_e2e_steps.rs`; capture with the
      shared helper under the untouched title, so the slug stays
      `consumer-opens-the-report-of-a-scope-with-no-recorded-activity`.
    - **Evidence:** refresh the final-state pair **and** the declared state's
      pair in place:
      `evidence/visual/consumer-opens-the-report-of-a-scope-with-no-recorded-activity/{mobile.png,desktop.png,mobile--empty.png,desktop--empty.png}`
      — all four non-empty; the enforced rule requires a non-empty
      `<viewport>--<state>.png` for every declared state and viewport.
    - Files: `assets/timeline/report.html` (the empty branch and the inert
      script), `tests/steps/development_report_e2e_steps.rs`, the four
      refreshed PNGs.
    - RED expected failure: `the browser draws no workflow phase for it` is
      undefined and the asset still draws the superseded timeline markup.
  - [ ] **RED** — remove `@wip` from this scenario only, then run:
        `GIVN_FEATURES=givn/changes/development-timeline/specs cargo test --locked --test features_runner -- --exact "inspect-development-history/development-report.feature::Consumer opens the report of a scope with no recorded activity"`
        → must exit non-zero. Paste it.
        ```
        PASTE THE NON-ZERO RED RUN HERE
        ```
  - [ ] **GREEN** — implement the asset's empty branch and the two Thens. Files:
        `assets/timeline/report.html`,
        `tests/steps/development_report_e2e_steps.rs`, plus the four refreshed
        PNGs. `node --check`, compile, re-run → exit 0. Paste it.
        ```
        PASTE THE GREEN RUN HERE
        ```
  - [ ] **REFACTOR** — the same one `empty` condition gates the stack, the
        axis, and the log; `cargo fmt --all -- --check`, `cargo clippy
        --locked --all-targets -- -D warnings`, and `node --check` green;
        re-run → still exit 0. Paste it.
        ```
        PASTE THE REFACTOR RUN HERE
        ```
  - [ ] **COMMIT** — `<hash>` — `test(e2e): <release note>` with the scenario
        title in the body (including the four refreshed captures). Record the
        revision identifier.
        ```
        PASTE THE COMMIT AND `git show --stat` HERE
        ```

## Task HE7 — Evidence refresh, slug preservation, and both declared scope runs (no new scenario)

- [ ] **Shared verification task: nine `@e2e` evidence sets and the two scope runs**
  - Design constraints:
    - **Slug preservation (review F10).** The delta keeps the existing titles,
      so these slugs are unchanged and their directories are refreshed in
      place: `consumer-generates-the-development-report-for-a-change`,
      `consumer-opens-the-generated-development-report-in-a-browser`,
      `consumer-selects-a-timeline-element-and-reads-its-details`,
      `consumer-zooms-the-timeline-to-a-shorter-time-range`,
      `consumer-opens-the-report-of-a-scope-with-no-recorded-activity`. The
      **only new directory** is
      `consumer-expands-a-workflow-phase-and-sees-its-lanes`. The record and
      agent-reporter `@e2e` pairs are untouched.
    - Every one of the nine change-level `@e2e` scenarios has a non-empty
      `desktop.png` (1920x1080) and `mobile.png` (360x780) under its slug; the
      empty-scope scenario additionally has non-empty `desktop--empty.png` and
      `mobile--empty.png`; no intermediate captures live under the change
      directory; every PNG is the contract viewport size.
    - **Both declared scopes exit 0:** `./run-tests.sh --non-e2e`
      (`verify.command`) and `./run-tests.sh --e2e` (`verify.e2e_command`) on
      the full delta; paste the summary lines. Every scenario this part
      schedules appears PASS in its own tier; the earlier scenarios stay green;
      the e2e count is strictly less than the full count (Task E1's proof
      still holds — paste the two summary lines side by side).
    - `givn lint --change development-timeline` exits **0** with no `[WIP]`
      finding (the only remaining output is the pre-existing `[SUBST]
      advisory` lines).
    - The inventory stays 62/53/9. If a runner summary disagrees by name,
      reconcile by name against the inventory and report it — never edit the
      inventory to match a surprise.
    - Files: none in production; this task records evidence, the two scope
      summaries, and the evidence listing.
  - [ ] **VERIFY** — run the two scopes and paste both summaries with the
        per-scenario PASS lines for the 15 new scenarios.
        ```
        PASTE BOTH SCOPE RUNS HERE
        ```
  - [ ] **EVIDENCE** — list the nine directories and their files with sizes and
        `file` dimensions; confirm the only new directory is the expand
        scenario's and all files are non-empty.
        ```
        PASTE THE EVIDENCE LISTING HERE
        ```
  - [ ] **LINT** — `givn lint --change development-timeline` → exit 0, no
        `[WIP]` finding.
        ```
        PASTE THE LINT RUN HERE
        ```
  - [ ] **COMMIT** — `<hash>` — `docs(development-timeline): hybrid workflow
        report evidence and scope runs` with the scenario titles and the counts
        in the body. Record the revision identifier.
        ```
        PASTE THE COMMIT AND `git show --stat` HERE
        ```

**Part-2 done when:** all 15 pending scenarios have completed RED/GREEN/REFACTOR
loops with pasted evidence and a revision each, H2's unit test and H8's
later-first-transition unit assertion are green, the asset is the hybrid
workflow view, the nine `@e2e` scenarios each have a committed non-empty pair
at both contract viewports (plus the empty-scope declared-state pairs), the
existing five report slugs are unchanged, the expand scenario owns the only new
directory, `git grep -c "@wip" -- givn/changes/development-timeline/specs`
finds none, `givn lint --change development-timeline` exits 0, and both
`verify.command` and `verify.e2e_command` exit 0 on the full delta. The box
counts then read **359 total / 359 done / 0 pending**.

