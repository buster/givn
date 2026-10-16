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
| 2 | 02 architecture-constraints | Yes | Two new durable technical constraints: the record is local, append-only, best-effort and never a gate, engages only once the Givn scaffold exists, and is kept out of repository history by an ownership split (the recorder only ever writes never-tracked destinations on every write; the versioned Fossil list is written by the owning commands) so recording can never dirty a worktree; the report is one self-contained file with no server, port, remote resource, or third-party JavaScript. | TC-34 (corrected ignore rule and engagement rule) and TC-35 added. |
| 3 | 03 context-and-scope | Yes | A new external entity (each supported harness's reporter), six new interfaces (`givn history ingest`, `givn history`, `givn timeline`, plus the reporter boundary for opencode, Claude Code, and omp), and the record file join the context; the scope gains the record and the report. | External entity, interface rows (the three harness boundaries extended), and in-scope bullets added; `.agents` recorded as a skills mirror with no runtime; the new use case `inspect-development-history` is named. |
| 4 | 04 solution-strategy | Yes | The strategy table gains the development record and report concern, because it spans capture, storage, intake, and rendering. | "Development record" row added to 4.1. |
| 5 | 05 building-block-view | Yes | New building blocks: `history` (record model and store), `cli::history` (read, the one intake, and the process-per-hook harness mapper), `cli::timeline` (report model and rendering), the embedded timeline asset, and the three embedded harness reporter assets. | Diagram node plus five responsibility rows added. |
| 6 | 06 runtime-view | Yes | Two new flows: best-effort capture while other commands run, and the intake/report flow, including the per-harness capture order for the three reporters. | New section 6.18 with a mermaid capture sequence and a per-harness capture table. |
| 7 | 07 deployment-view | Yes | The project footprint gains `givn/history/` (record, reports, pending state), one generated reporter per executing agent target (`.omp/hooks/pre/`, `.opencode/plugins/`, `.claude/hooks/` plus its settings registration), and the repository-only `tests/e2e/` Node project; the acceptance suite gains a Node/Chromium prerequisite installed outside the suite. | 7.3 target environments and 7.4 project footprint updated. |
| 8 | 08 crosscutting-concepts | Yes | The record's ownership, the engagement rule, the runtime-state status of the store, the three harness reporters and their documented asymmetries, best-effort discipline, scope rule, privacy bound, ignore entry, scale rule, report packaging, and browser tier are cross-cutting concepts every capture point and future reporter must respect. | New section 8.25 added, including the engagement, runtime-state, reporters, scale, and missing-data paragraphs. |
| 9 | 09 architecture-decisions | Yes | One new qualified decision (ADR-073): record ownership and the single intake interface — now carrying three harness reporters. Two further candidates were evaluated and routed to canonical artifacts instead of ADRs. | ADR-073 created and extended, register row and chapter-09 paragraph added. |
| 10 | 10-quality-requirements | Yes | New quality scenarios for intake rejection, record/render reproducibility, the record's privacy bound, rendering a multi-day record at scale, and harness parity. | Q-98 to Q-102 added. |
| 11 | 11 risks-and-technical-debt | Yes | Unbounded store growth, per-harness contract drift (including the younger opencode V2 API), best-effort loss, the new browser tier in the archive gate, the Claude Code surface gaps, and the never-dirty ignore rule (a versioned ignore write would have broken `givn upgrade`'s clean-checkout precondition — reproduced by this change's own implementation, with a Fossil residual) are risks; pruning and repair paths are deliberate debt. | R-83 to R-86, R-88, R-89 (refined), D-22, D-23 added; R-87 retired as inaccurate. |
| 12 | 12-glossary | Yes | New domain terms: development record, record entry, development scope, intake, agent reporter, bounded label, record gap, development report, band, question wait, aggregate marker, subagent span, pending state, reporter gap. | Fourteen glossary entries added with anti-terms, and the existing `Ignore block` entry widened to cover the rules block and the record entry. |

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
three reporters behind one intake, which is recorded as a consequence of that
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
other tree walkers never reach the record directory. Delta inventory is now 43
entries (35 non-`@e2e`, 8 `@e2e`).

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
  instead), and chapter 08's scope, ordering, and ignore paragraphs follow the
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
`adr/README.md`, the new `adr/ADR-073-development-record-ownership.md`, and the
new design system `docs/design/design-system.md`.

Chapters 1, 2 and 3 keep their specification-layer prose free of implementation
detail: chapter 02 records constraints only, chapter 03 names interfaces and
scope only, and the mechanics live in chapters 5, 6, and 8.

## Status

STATUS: DONE
