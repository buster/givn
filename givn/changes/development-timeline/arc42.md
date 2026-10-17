# arc42 Documentation Update: development-timeline

> This is the change-level arc42 **impact assessment template**. It is not an
> ADR template and must not be used to draft, store, or combine architecture
> decisions. An architecturally significant decision gets exactly one separate
> MADR file at `docs/arc42/adr/ADR-NNNN-short-slug.md`; chapter 09 is only its
> register/index. Use the ADR guidance and the Qualification gate before
> creating that file.

## Impact assessment

| # | Chapter | Affected? | Reason | Summary of change (if Yes) |
|---|---|---|---|---|
| 1 | 01 introduction-and-goals | Yes | The change adds a product functional requirement: Givn itself records the development process and renders it as a report, so the requirements overview gains a row. | FR-45 added for the development record and the development report. |
| 2 | 02 architecture-constraints | Yes | Four new durable technical constraints: the record is local, append-only, best-effort and never a gate, engages only once the Givn scaffold exists, and is kept out of repository history by an ownership split (the recorder only ever writes never-tracked destinations on every write; the versioned Fossil list is written by the owning commands) so recording can never dirty a worktree; the report is one self-contained file with no server, port, remote resource, or third-party JavaScript; the Maki reporter is a trusted-project Lua plugin whose grants are explicit; and the capture control is one project-owned mode key enforced by the intake. | TC-34 (corrected ignore rule and engagement rule), TC-35, TC-36 (Maki plugin and trust rule), and TC-37 added. TC-37: `record.mode: on \| off \| full` (default `on`, parsed leniently so an absent/invalid value falls back with one warning) — `off` is absolute (no append/pending/ignore write; `--with-full-content` ignored with a warning and exit 0), `on` records every available metadata field (label 120, text 240, error 1 000) plus full question text (4 000), `full` adds complete content (intake-enforced, 64 KiB per field / 256 KiB per entry, inline under the ignored `givn/history/` root), and lowering the mode retains captured entries. |
| 3 | 03 context-and-scope | Yes | A new external entity (each supported harness's reporter), eight new interfaces (`givn history ingest`, `givn history settings`, `givn history`, `givn timeline`, plus the reporter boundary for opencode, Claude Code, omp, and Maki), and the record file join the context; the scope gains the record and the report. | External entity, interface rows (the four harness boundaries, Maki's including the folder-trust gate and the transcript-read question text), and in-scope bullets added; `.agents` recorded as a skills mirror with no runtime; the new use case `inspect-development-history` is named. `givn history settings` is the non-recorded mode query (source `config` or `default`); Claude resolves the mode in-process while the other three query by subprocess, and the intake enforces the mode and every bound. |
| 4 | 04 solution-strategy | Yes | The strategy table gains the development record and report concern, because it spans capture, storage, intake, and rendering. | "Development record" row added to 4.1. |
| 5 | 05 building-block-view | Yes | New building blocks: `history` (record model and store), `cli::history` (read, the settings query, the one intake, the mode/bound enforcement, and the process-per-hook harness mapper), `cli::timeline` (report model, workflow phase/current-state scaffold, lane/repeated-group/Fit-aggregate scaffold, lane-log facts, and bounded content rendering), the embedded workflow report asset, the post-command artifact-transition emission, and four embedded harness reporter integrations (five assets: omp, opencode, Claude Code, and Maki's module plus its grants file), with Maki additionally a full guidance target. | Diagram node plus five responsibility rows added. The `history` block owns the capture modes, the two metadata tiers, and every bound; `cli::timeline` owns the 8 MiB report budget and the omission marker. |
| 6 | 06 runtime-view | Yes | Two new flows: best-effort capture while other commands run, and the intake/report flow, including the per-harness mode discovery and capture order for the four reporters and the report's widened ranking of measured entries of any kind. | New section 6.18 with a mermaid capture sequence, a per-harness capture table, and the report paragraph. The capture flow states the two tiers (bounded metadata in `on`, complete content only in `full`, question text always in full), the two off-enforcement sites (the intake and Claude's in-process hook), and the in-process/subprocess discovery split; the report paragraph adds the workflow stack with the embedded current state, lanes, lane log, every-zoom phase aggregates, the 8 MiB content budget, the omission note, and determinism over record plus state snapshot. |
| 7 | 07 deployment-view | Yes | The project footprint gains `givn/history/` (record, reports, pending state), one generated reporter per executing agent target (`.omp/hooks/pre/`, `.opencode/plugins/`, `.claude/hooks/`), Maki's guidance and reporter (`.maki/skills/`, `.maki/commands/`, `.maki/lua/givn-timeline.lua`, grants merged into `.maki/plugin.toml`, and a managed fence in `.maki/init.lua`), and the repository-only `tests/e2e/` Node project; the acceptance suite gains Node/Chromium and a pinned `maki` v0.6.3 binary, provisioned outside the suite. | 7.3 target environments and 7.4 project footprint updated. Full content is not a second store: it lands inline in `givn/history/events.jsonl`, so the whole footprint stays under the one ignored root. |
| 8 | 08 crosscutting-concepts | Yes | The record's ownership, the engagement rule, the runtime-state status of the store, the four harness reporters and their documented asymmetries (including Maki's folder-trust gate and main-session-only turns), best-effort discipline, scope rule, privacy bound, ignore entry, scale rule, report packaging, and browser tier are cross-cutting concepts every capture point and future reporter must respect. | New section 8.25 added, including the engagement, runtime-state, reporters, scale, report-budget, and missing-data paragraphs; the scale rule applies the sub-pixel aggregation at every phase zoom, and report packaging states determinism over the record plus the embedded state snapshot. The privacy bound is per tier: bounded metadata in `on`/`full`, complete content only in `full`, question text always in full; `off` is absolute, lowering the mode retains captured entries, and what is never captured as structured values is enumerated (full content can carry a credential that appears inside captured work, per R-92). |
| 9 | 09 architecture-decisions | Yes | One new qualified decision (ADR-073): record ownership and the single intake interface — now carrying four harness reporters — plus an ADR-013 amendment for the `maki` target (full guidance target plus the reporter, with two merged user-owned files). Two further candidates were evaluated and routed to canonical artifacts instead of ADRs. | ADR-073 created and extended, ADR-013 amended (register row, ADR body, chapter-09 paragraph), chapter-09 paragraph added. |
| 10 | 10-quality-requirements | Yes | New quality scenarios for intake rejection, record/render reproducibility, the record's privacy bound, rendering a multi-day record at scale, harness parity, and analysis sufficiency. | Q-98 to Q-102 added, with Q-99 qualified for determinism over record plus the embedded state snapshot (R-96) and Q-101 for the every-zoom DOM bound (R-94). Also Q-103 (analysis sufficiency: the default tier carries the bounded metadata each harness exposes, a harness-unavailable field is absent rather than fabricated, and no tier stores wire payloads, MCP params, harness system prompts, or credentials as dedicated fields; full content can carry a credential inside captured work, per R-92). |
| 11 | 11 risks-and-technical-debt | Yes | Unbounded store growth, per-harness contract drift (including the younger opencode V2 API and Maki's new Lua surface plus its folder-trust gate), best-effort loss, the new browser tier in the archive gate, the Claude Code surface gaps, the never-dirty ignore rule (a versioned ignore write would have broken `givn upgrade`'s clean-checkout precondition — reproduced by this change's own implementation, with a Fossil residual), and Maki's transcript-read question-text residual are risks; pruning and repair paths are deliberate debt. | R-83 to R-86, R-88, R-89 (refined), R-90 (including the `maki.session.messages` question-text residual), D-22, D-23 added; R-87 retired as inaccurate. Plus R-91 (store growth with `full`), R-92 (full content is sensitivity-bearing; the default `on` is metadata-only, content requires explicit `full`, the store is local and ignored), and the hybrid's R-93 (per-subagent attribution depends on a reporter's own session), R-94 (every-zoom DOM bound), R-95 (phase boundaries depend on recorded artifact transitions; the current store has none), and R-96 (the current state is a generation-time snapshot). |
| 12 | 12-glossary | Yes | New domain terms: development record, record entry, development scope, intake, agent reporter, bounded label, record gap, development report, band, workflow phase, lane, lane log, repeated-step group, current state, question wait, aggregate marker, subagent span, pending state, folder trust, metadata bound, full content, record mode. | Twenty-two glossary entries added with anti-terms (the five report terms — **Workflow phase**, **Lane**, **Lane log**, **Repeated-step group**, and **Current state** — and the three capture-completeness terms — **Metadata bound**, **Full content**, and **Record mode** — are included in that count), and the existing `Ignore block` entry widened to cover the rules block and the record entry. |

## Notes

**Harness extension (same pass, 2026-10-16).** The operator extended the change
from one reporter to every supported harness after the first draft. "Supported"
was read from the code, not assumed: `src/skills/targets.rs::AgentTarget` lists
`opencode`, `agents`, `claude`, and `omp`, and `assets/agent-integrations/`
carries one integration per executing target — so three harnesses report and
`agents` (a neutral skills mirror no harness runs) does not. Each harness's
surface was verified against the installed artifact rather than memory:

- **omp 18.6.1** — the shipped types of the installed
  `@oh-my-pi/pi-coding-agent` package (event names, payload fields,
  `getUsageStatistics`, the subagent lifecycle frames on the event bus, the
  30 s/2 s handler budgets).
- **opencode 2.0.26** — `opencode --version`, the running CLI's own log
  (`~/.local/share/opencode/log/opencode.log`, `version=2.0.26`), the tagged
  upstream sources at `v2.0.26` (`packages/plugin/src/promise/*.ts`,
  `packages/schema/src/*.ts`), and the live
  `~/.local/share/opencode/opencode.db` (session version, tool names,
  message types). The reporter targets the **V2** plugin API, per the official
  migration guide (`https://opencode.ai/v2/docs/build/plugins/migrate-v1`):
  a V2 default-exported definition with a stable `id` and `setup(ctx)`,
  discovered from `.opencode/plugins/`, with no V1 string-key hooks. The V2 API
  is young, so the installed CLI version defines the surface; this working
  copy's untracked, gitignored `.opencode/package.json` dev pin names
  `@opencode/plugin` 2.0.26 (the CLI's own tagged line) and is not a design
  input.
- **Claude Code 2.1.266** — `claude --version` and the version symlink, the
  event set cross-checked against the changelog bundled with that build
  (`~/.claude/cache/changelog.md`), payload schemas from the published hook
  documentation (which describes a newer build and is therefore used for
  schemas only), corroborated by real transcripts and subagent files under
  `~/.claude/projects/-home-buster-projects-givn/`; the surface gaps
  (no timestamps, no usage in payloads, no telemetry for background subagent
  launches) are recorded as R-88 rather than papered over.

The extension adds no new ADR candidate: it extends ADR-073's intake boundary to
four reporters behind one intake, which is recorded as a consequence of that
record rather than as a parallel decision.

**Ignore-mechanism correction (same pass, 2026-10-16).** The fresh design
re-review re-opened the store's ignore mechanism (findings F1-F11, question Q-A)
and the operator settled Q-A on **option 2**: the record path joins the managed
ignore file rather than relying on the repository setting alone, because a
committed `.fossil-settings/ignore-glob` overrides that setting and Givn's rules
writer already writes that file in every managed Fossil checkout — the
setting-only arm would have left the store an `EXTRA` exactly in the projects
Givn manages. The settled rule is the ownership split recorded in TC-34, in the
design's D1 entry, and in the tasks: **the recorder never writes a versioned
file**, it re-asserts the never-tracked Git destination on every write, and the
record path reaches the versioned Fossil destination only through the commands
that own it (`givn init`, `givn skills sync`, `givn upgrade`). The Fossil entry
form is `givn/history/*` (a bare directory path matches nothing under Fossil's
whole-path globs), repository existence is read from the backend markers rather
than from backend selection succeeding, and the affected permanent fixtures are
a class (`git_managed_project`, `git_project_with_version`,
`fossil_project_with_version`) whose ignore content gains the record path. The
hazard and its Fossil residual are R-89; the glossary's `Ignore block` entry now
covers both the rules block and the record entry.

**Recorder engagement and test-state exclusion (same pass, 2026-10-16).**
Implementation found a second class after Tasks 10-11 landed, and the artifacts
now record both halves. (1) **Product rule:** the recorder engages only when the
project root carries the Givn scaffold, so a refused command in a bare root
creates neither `givn/` nor the store, while a scaffolded project records every
command including refusals and failures (the rule keys on the scaffold, not on
the VCS markers). It has its own delta scenario, `A refused command outside a
Givn project creates no record`, and Task 11a. (2) **Test-side class:** the
store is Givn runtime state, so the acceptance suite's project-state comparison
helpers exclude `givn/history/**` — `scm_backend_selection_steps.rs::snapshot_tree`/`collect_tree`
and `serve_extended_rules_steps.rs::project_files`/`collect_project_files`; the
other tree walkers never reach the record directory. Delta inventory after this
note was 43 entries (35 non-`@e2e`, 8 `@e2e`); the Maki extension below brings it
to **44 entries (36 non-`@e2e`, 8 `@e2e`)**.

**Maki extension (same pass, 2026-10-16, operator instruction).** A fourth
supported harness joined the reporters: Maki (`maki 0.6.3`, verified through
`maki --version`, its Lua API reference (`https://maki.sh/docs/lua-api/#maki-api-create_autocmd`)
and plugins guide (`https://maki.sh/docs/plugins/`), and real probe projects. Because
Maki is the one harness whose surface is its own language, its reporter is a Lua
plugin: `.maki/lua/givn-timeline.lua` plus its grants in `.maki/plugin.toml`,
loaded by a managed `-- givn:begin:timeline` `require` block in `.maki/init.lua`
(Maki runs only that file, so a module under `lua/` stays inert otherwise) — the
hand-placed start-package alternative was rejected because it lives outside the
project and would install for every project on the machine. Two harness
behaviours are recorded rather than glossed over: **folder trust** (Maki loads
nothing from `.maki/` in a folder it has not been trusted for — reproduced: an
untrusted probe project loaded no plugin, the same project with `--trust` did),
and **main-session-only turns** (`TurnEnd` fires once per turn and never for a
subagent, so per-subagent token usage is unavailable through Maki). The headless
driver relies on the verified CLI behaviour: `maki --trust prompt` loads the
project plugin and exits 0 with no model call, while `maki --trust prompt -p …`
is rejected (exit 2) and a real turn (`maki --trust -p …`) is deliberately not in
the suite. Maki needs guidance Givn must write itself: it reads skills from `.maki`,
`.claude`, `.opencode`, and `.agents`, but project **commands** only from
`.maki/commands/` and `.claude/commands/` — and Givn writes commands only for
`opencode` and `omp`. `AgentTarget::Maki` is therefore a **full target** (skills
to `.maki/skills/<name>/SKILL.md`, commands to `.maki/commands/<name>.md`,
Markdown with optional `---` frontmatter and `$ARGUMENTS`, verified against
`https://maki.sh/docs/commands/`), so a `maki`-only selection is equipped, and
the reporter adds three pieces on top: the module `.maki/lua/givn-timeline.lua`
(managed generated, with the skills and command files), a **merged** grant block
in `.maki/plugin.toml` (created with all six keys explicit, otherwise only the
reporter's three are added, never removed or flipped), and a **merged** fail-soft
`-- givn:begin:timeline` fence in `.maki/init.lua` — a bare
`pcall(require, "givn-timeline").setup()` exits 1 and was rejected. The new risk
is R-90, and the driver needs the pinned `maki` v0.6.3 binary provisioned by the
setup-script family.

**Ranking widening and Maki headless gaps (2026-10-17, operator finding).** Two
things were recorded after the implementation landed. (1) The report's ranking
filtered `kind == "step"`, so this repository's own report showed an empty
"Longest steps" list while the record held 75 measured entries (a 3.6 s
`skills sync`, a 799 ms `task` tool call). The rule is now explicit in
`design.md` §Report Generation and in chapters 06/08/10: the ranking covers
**every entry with a measured `duration_ms`** of any kind, longest first with the
existing tie-breaking (earlier `at`, then lower `id`), over the whole record;
each row carries label, **kind**, and duration (`RankedStep` gains the kind); the
heading stays "Longest steps" and an unmeasured entry is absent rather than zero.
It arrives as the delta scenario `Measured agent activity is ranked alongside
Givn's own steps` (Task 25a). (2) Maki's headless gaps were reproduced and
recorded: `TurnStart` is TUI-only (so a headless turn has usage but no duration),
and in the reproduced headless run the record carried the `task` tool entry
(799 ms) with **no subagent span**, although Maki's log shows the subagent
session opening and closing (`duration_ms 795`, `input_tokens 8512`) and
`maki.task.list()` answers `no interactive UI attached`, so a plugin cannot
enumerate subagents there. Whether `TaskStatusChanged` reaches a `task`-tool
subagent outside the TUI is **not established** (the 0.6.3 binary's bundled
reference names only `TurnStart` and `PlanReady` as `maki-ui`-only), so the
recorded fact is the run's observation — measured tool calls and turn usage, no
turn durations, and no subagent span in that run; subagent spans come from omp,
opencode, and Claude Code (R-90). Inventory
is now 47 entries (39 non-`@e2e`, 8 `@e2e`).

**Capture completeness (2026-10-17, operator addendum).** The operator added
four record-side requirements after the implementation landed, and the durable
chapters follow them. (1) **Two tiers:** bounded metadata is captured always
(label 120, text metadata 240, error text 1 000 characters), and complete
LLM-facing content only in the `full` mode — `record.mode: on | off | full` in
`givn/config.yaml`, default `on`, where `off` writes nothing at all and an
absent or invalid value falls back to `on` with one warning; the intake enforces
the mode (a one-payload `--with-full-content` override exists for tests and
manual capture), so a stale reporter cannot bloat or leak the store, and
reporters learn it from `givn history settings --json` (treating a failed query
as `on`, warning once, never blocking the harness).
(2) **Question text is the exception:** always captured in full, capped at
4 000 characters per question entry, alongside the bounded label. (3) **Caps and
placement:** 64 KiB per content field and 256 KiB per entry, stored inline in
the same `givn/history/events.jsonl` line — no second store, so the managed
ignore block covers content and reports by construction (`givn/history/` for
Git, `givn/history/*` for the managed Fossil file). (4) **Never captured at any
tier:** provider wire payloads and headers, MCP protocol frames and params,
harness system prompts, credentials, UI dialog internals, and control-surface
results. Chapter updates: TC-37 in chapter 02, the settings interface in
chapter 03, the `history` block row in chapter 05, the capture-tier paragraph in
chapter 06, the inline footprint in chapter 07, the per-tier privacy bound in
chapter 08, Q-103 in chapter 10, R-91/R-92 in chapter 11, and **Metadata bound**
plus **Full content** and **Record mode** in chapter 12; the config scaffold
(`assets/config.yaml.tmpl`) documents the mode. No new ADR candidate
qualified: the two tiers extend ADR-073's intake boundary rather than changing
the ownership or delivery decision. Delta inventory after the addendum is 54
entries (46 non-`@e2e`, 8 `@e2e`) — six new scenarios are in
`development-record.feature` (20 entries), one report-surfacing scenario is in
`development-report.feature` (16 entries), and the two ignore scenarios were
extended in place, so the `@e2e` accounting is unchanged.

**Capture-completeness hardening (2026-10-17, re-review round 5).** The fresh
grilling pass resolved four operator-level questions and applied twelve findings
without changing the record's ownership boundary. (1) `off` is absolute: the
`--with-full-content` override applies to `on`/`full` only and is ignored with a
one-line warning and exit 0 under `off`; the intake and Claude's in-process
`harness_hook` are the two enforcement sites (no append, no pending write, no
ignore write). (2) Lowering the mode is forward-looking: already-captured
entries are retained, never purged. (3) The report embeds full content up to an
**8 MiB total budget** and marks the remainder `content omitted; see the
record`, while the store stays complete. (4) Maki's question text is read from
`maki.session.messages` at the wait's close (no new grants); the every-permission
`tool.question.input` slot is rejected, and a wait with no matching question
keeps the bounded title label (R-90). The key is parsed leniently by
`OverlayDoc` (`src/config/overlay.rs`), `givn history settings` is a non-recorded
read view with a `config|default` source, the intake is the single enforcement
site for every bound, the default-metadata list is qualified per harness, and
the report's lane log/contract now carries the exact fact list and content
region. The durable chapters follow all of this; `design-review.md`'s re-review
round 5 records the evidence and the applied findings.

The ADR candidates were evaluated in order and only the first qualified:

1. **Development-record ownership and the single intake interface** — the
   complete qualification procedure was applied. Verdict:
   `qualification: QUALIFIED`, `alternatives: PASS` (proposal D3 weighed a
   Givn-owned intake, direct writes by each integration, and reading the
   harness's private session log), `architectural_impact: PASS` (it fixes who
   owns the record's format, the only writer for reported activity, and the
   dependency direction of every harness integration),
   `durable_consequence: PASS` (reversal means migrating durable record data and
   re-shipping a generated asset that is already installed in projects),
   `lower_level_artifact: PASS` (the delta Gherkin owns the observable
   behaviour and `design.md` owns the schema, but neither owns the ownership and
   delivery boundary, and no process document does),
   `existing_adr_check: PASS` (the full active and archived registers were
   searched: ADR-014 owns filesystem-derived state, ADR-013 owns generated
   agent integrations and `AGENTS.md` ownership, ADR-028 owns the hook versus
   mandatory-check boundary; none owns a durable record of development activity
   or its intake), `must_be_shared: SUPPORTING`, `routing: NEW_ADR`,
   targets `null`. Falsification tests: reversal is not a one-change,
   one-component edit (test 1 does not fail the candidate), it constrains
   future reporters and any schema change (test 2 does not fail), and it is not
   mere workflow behaviour — it moves a data-ownership boundary and a
   cross-product delivery contract, named in the verdict (test 3 does not
   fail). **ADR action: NEW_ADR**, target `ADR-073-development-record-ownership.md`.
   Cheaper-home challenge: `design.md` would own only the record schema, and
   arc42 chapter 8 would only describe the concept; the cross-product delivery
   contract with independently versioned harness integrations has no other
   canonical owner.
2. **Report packaging (self-contained file, no CDN, no third-party JavaScript)**
   — `qualification: NOT_QUALIFIED`: `alternatives: PASS` (static file versus
   local server versus CDN-delivered library),
   `architectural_impact: PASS` (it fixes a dependency and offline-rendering
   constraint), `durable_consequence: FAIL` (switching to a server or a vendored
   library is implementable and reversible inside one change and one component,
   so falsification test 1 fails), `lower_level_artifact: PASS`,
   `existing_adr_check: PASS` (no record owns report packaging).
   `routing: CANONICAL_ARTIFACT`, `canonical_artifact: design.md` (this change's
   report-generation section), with the durable constraint carried by TC-35 in
   chapter 02 and Q-99 in chapter 10.
3. **Adopting Playwright as the repository's browser driver for this screen** —
   `qualification: NOT_QUALIFIED`: the candidate restates an existing process
   policy (the design instruction already names Playwright as the default
   browser driver and resolves its version per change), so
   `lower_level_artifact: FAIL` (the owning policy already exists) and
   falsification test 3 applies (test and process policy, not architecture).
   `routing: CANONICAL_ARTIFACT`, `canonical_artifact: givn/changes/development-timeline/design.md`
   (the change-local application of the existing policy), with the browser-tier
   risk carried by R-86.

```text
ADR action: NEW_ADR
ADR target: docs/arc42/adr/ADR-073-development-record-ownership.md

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
```

**Hardening pass (design-review phase 2, 2026-10-16).** The grilling findings
and the operator's answers changed the plan, and these chapter updates follow
them:

- chapter 01 gains **FR-45** (the record and the report as a product
  functional requirement) — the previous "no product goal changes" row was
  self-contradicting, since the change adds requirements to the product;
- chapter 02's **TC-34** states the corrected ignore rule: the recorder never
  writes a versioned file (it re-asserts `.git/info/exclude` on every write and
  writes Fossil's repository `ignore-glob` setting only while no managed file
  overrides it; `.gitignore` only before a repository exists), and inside a
  Fossil checkout that has `.fossil-settings/ignore-glob` the entry
  `givn/history/*` belongs to that file's managed list through the owning
  commands — because a versioned write at recorder start would dirty the
  worktree and break `givn upgrade`'s clean-checkout precondition;
- chapter 03's `givn timeline` interface row and chapter 08's report-packaging
  paragraph drop the `--open` and `--json` surface the design no longer
  declares (the printed default path under `givn/history/reports/` is named
  instead), and chapter 08's report-packaging paragraph names the document's two
  substituted markers — the machine-readable model payload element the inline
  script parses and the text-only initial view the file carries — instead of a
  single data slot, and chapter 08's scope, ordering, and ignore paragraphs follow the
  hardened record contract (exactly-one-scope resolution, append-time ids,
  ordering by `at`);
- chapter 11 keeps the **R-87** gap with a one-line note: the identifier was
  retired as inaccurate and nothing references it, so renumbering R-88 would
  only make existing references ambiguous;
- chapter 12 gains **Band** with its anti-term, the term the report and the
  design system already use.

Files updated outside the change directory:
`docs/arc42/01-introduction-and-goals.md`,
`docs/arc42/02-architecture-constraints.md`, `03-context-and-scope.md`,
`04-solution-strategy.md`, `05-building-block-view.md`, `06-runtime-view.md`,
`07-deployment-view.md`, `08-crosscutting-concepts.md`,
`09-architecture-decisions.md`, `10-quality-requirements.md`,
`11-risks-and-technical-debt.md`, `12-glossary.md`, `README.md`,
`adr/README.md`, the new `adr/ADR-073-development-record-ownership.md`, the
new design system `docs/design/design-system.md`, and the config scaffold
`assets/config.yaml.tmpl`.

Chapters 1, 2 and 3 keep their specification-layer prose free of implementation
detail: chapter 02 records constraints only, chapter 03 names interfaces and
scope only, and the mechanics live in chapters 5, 6, and 8.

## Hybrid report delta (2026-10-17)

The operator approved the hybrid workflow report after five mockups, superseding
the dense horizontal timeline (design.md `Decides D6`, design review round 6).
The affected chapters are updated in place: chapter 05 names the workflow
phase/current-state, lane, repeated-group, and Fit-aggregate scaffold and the
post-command artifact-transition emission; chapter 06 records the state snapshot
and the resulting determinism definition; chapter 08 applies the aggregation
threshold at every phase zoom and defines report determinism over the record
plus the embedded state; chapter 10 qualifies Q-99 and Q-101; chapter 11 carries
R-93 to R-96; chapter 12 carries **Workflow phase**, **Lane**, **Lane log**,
**Repeated-step group**, and **Current state**. Delta inventory after this
delta and the review hardening is 58 entries (49 non-`@e2e`, 9 `@e2e`); the
prior 54-entry statements above stay as dated history of their rounds. No new
ADR candidate qualified: the visual direction is a design decision recorded in
`design.md` and the design system, and the attribution/scale/state boundaries
are refinements of ADR-073's intake and report surface, not a new ownership
decision.

## Recorder self-recording delta (2026-10-17)

The report recorded the recorder's own `command` entries and then displayed
them. This delta drops them from the report model (design.md *Recorder
self-recording*, design review round 7) while the store and every read view
keep them. The affected chapters are updated in place: chapter 05's
`cli::timeline` row names the first-step omission; chapter 06's report flow
drops them before any derivation and derives the empty state from the model;
chapter 08's scale paragraph defines summary and ranking coverage over the
model and names the omitted entries; chapter 10's Q-101 follows; chapter 12's
**Development report** states that every number and lane counts the model.
The exclusion covers both recorder invocations — the intake
(`givn history ingest`, with trailing flags) and the process-per-hook mapper
(`givn history harness-hook --harness <name>`) — and matches the subcommand's
word boundary, never a substring. Delta inventory after this delta is 61
entries (52 non-`@e2e`, 9 `@e2e`), one of them `@wip`; the earlier 54- and
58-entry statements stay as dated history of their rounds. No new ADR
candidate qualified: the rule is a report-model decision owned by
`design.md`, not a new ownership or intake decision.

## Status

STATUS: DONE
