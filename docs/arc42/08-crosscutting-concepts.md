# 8. Cross-cutting Concepts

This chapter describes the mechanisms shared by the lifecycle, generated
guidance, and gates. The current source code is authoritative
where generated comments or historical ADR text differ.

## 8.1 The Cascade Engine (Override Files)

For an individual asset facet, the resolver checks project content first and
embedded content second:

```text
1. project override under givn/
2. embedded core or addon asset in the binary
```

Templates use `givn/artifacts/<id>/<filename>`. Instructions use
`givn/artifacts/<id>/instructions.md`. Skill and command assets have their own
project override paths. `givn eject` materializes an embedded asset so the
project can own it. An existing file is protected unless `--force` is used.

The cascade is per facet. Ejecting a template does not eject the instruction,
and a project override remains effective after a binary upgrade.

## 8.2 Manifest and Overlay/Patch

The effective manifest is assembled in this order:

```text
embedded default-manifest.yaml
        + givn/config.yaml overlay
        + givn/commands.yaml overlay when present
```

Artifact patches are keyed by `id`. An existing artifact is merged field by
field; a new artifact is appended; `remove: true` removes it. `requires` forms
the lifecycle DAG, `optional` affects status completion, and `apply.requires`
identifies archive prerequisites. The effective graph is rejected for dangling
references or cycles.

The current `givn/commands.yaml` is intentionally separate from the structural
scaffold. It supplies `verify.command`, `verify.e2e_command`, and coverage
commands and paths. Because it is applied as a later overlay, those values are
the effective runtime values in this checkout.

The Arc42 addon is special overlay sugar. When `addons.arc42` changes from false
to true, overlay processing adds a required `arc42-docs` artifact after
`design`, adds it to `apply.requires`, and makes `design-review` require it.
The addon expansion is idempotent. The artifact's generated lifecycle marker is
`arc42.md`; the durable chapter files are not manifest-generated paths.

## 8.3 The Gherkin Delta Model

The `.feature` file is the behavior specification. Active change features use:

| Tag | Location | Meaning |
|---|---|---|
| `@givn.delta` | Feature | Marks a feature as an active delta. |
| `@givn.added` | Scenario | Add a scenario on archive. |
| `@givn.modified` | Scenario | Replace a same-title permanent scenario. |
| `@givn.removed` | Scenario | Remove a same-title permanent scenario. |
| `@wip` | Scenario | Marks unfinished implementation. |

An operation-less scenario defaults to Added. Removed scenarios require a
parseable placeholder; lint suppresses WIP reporting for that removed
placeholder. Archive validates all merge preconditions, applies the operations,
and strips `@givn.*` tags from the permanent output.

The merge implementation is text-level for scenario blocks. It preserves the
existing permanent feature header and surrounding text, while a replaced or
added scenario is rendered from the parsed AST. The `gherkin` AST does not
preserve every original docstring delimiter detail.

### 8.3a Normative Authoring Guidance

The resolved `specs` instruction is the single normative source for interaction
inventories and E2E evidence. Design, tasks, review, skills, and commands add
only artifact-specific choreography and derived checks.

The policy requires primary assertions against the named consumer interface.
Repository or database state is secondary evidence unless it is itself the
public interface. Browser E2E must use a real browser; an in-page HTTP request is
not a browser interaction substitute. An obstacle is a design gap, not
permission to weaken the assertion.

Project overrides can replace the effective policy through the cascade. Generated
skills and commands carry trigger metadata, invocation-specific orchestration,
and a direction to `givn instructions <id>`; they carry no rule content and have
no independent policy authority. The rule an agent follows for a step is the
extended rule: the built-in, technology-agnostic rules resolved at call time
plus the project's extension when one exists. An extension adds rules; it never
overrides, suppresses, or reorders the built-in rules, and it never changes
steps, order, or gates.

The resolved `dialog` instruction is the single normative source for phrasing
and recording user-facing questions. The current delivered references cover
only part of the authoring workflow. The approved target contract in
`make-user-questions-decision-ready`, pending implementation, makes the
obligation explicit at unconditional project/session startup, every bundled
available skill and command, and fresh delegated contexts. The complete rules
and five hypothetical worked contrasts belong only to the effective policy
served by `givn instructions dialog`; derived entry points carry short
references rather than competing copies. See that policy for question content,
proportional clarification, answer consequences, and material revision costs.

Distribution does not establish agent compliance. Real-interface delivery and
override/preservation checks are deterministic evidence; unseen fresh-context
question exercises require separate authoring/review judgment. Neither is a
new model-dependent gate or runtime interception mechanism.

### 8.3b Machine-readable Instruction Contract

`givn instructions <artifact> --json` emits one JSON object with exactly the
effective artifact `id`, `generates`, `requires`, and resolved `instruction`.
Human-readable mode emits instruction text. The object is resolved from the
effective manifest and the project-overrides-first instruction cascade. Before
printing, the resolved text is rendered against the effective manifest's addon
flags (conditional sections resolved when guidance is requested) and the
project's extension for the id is appended under the marked `## Project
extension` section; the `instruction` field carries that same effective text
on both paths.

A policy instruction (`dialog`) has no lifecycle stage, so it is not a manifest
artifact and is not resolved through the artifact graph. When the id is not a
manifest artifact, the command falls back to the instruction cascade; a
resolvable policy emits the same JSON shape with empty `generates` and
`requires` arrays, and an unresolvable id keeps the unknown-artifact error.
This lookup boundary is recorded here because it is not a qualified ADR
candidate: reversal is a single-function change with no migration or consumer
interface break beyond the documented shape.

## 8.4 Project Directory Layout

```text
givn/
  config.yaml                    # structural overlay, addon flags, targets
  commands.yaml                  # project commands and coverage settings
  specs/<usecase-id>/usecase.md # permanent use-case contract
  specs/<usecase-id>/<cap>.feature # permanent tag-stripped specification
  specs/fragments/fragment.md   # reusable behavior contract
  changes/<id>/                  # active change artifacts and delta features
    proposal.md
    specs/<usecase-id>/<cap>.feature
    design.md
    design-review.md
    tasks.md
    review.md
    arc42.md                     # when the Arc42 addon is enabled
    .givn-skip                   # optional artifact skip list
  archive/<id>/                  # copied active change after archive
  artifacts/<id>/                # ejected facet overrides

docs/arc42/                      # fixed durable Arc42 output path
```

`givn init` creates the core `givn/` directories and command/config files. The
current init path also enables all known addons unless `--no-addons` is passed;
Arc42 scaffolding therefore normally appears during initialization. There is
no effective `arc42.output_dir` or chapter-list setting, and no lifecycle
cascade for selecting individual durable chapter files.

## 8.5 The TDD Loop

The normal implementation guidance is scenario-by-scenario:

```text
RED:      make this scenario's assertions genuinely fail
GREEN:    implement the minimum code following design constraints
REFACTOR: clean up without changing behavior
```

The test runner configured in `verify.command` executes the feature files and
confirms GREEN. `givn lint` only parses and analyzes feature files; it never
executes the project tests. Mandatory gate execution is described in section
8.12.

### 8.5f Permanent Duplicate Audit

`givn spec duplicates` is an advisory complete audit of current permanent
scenarios only. It excludes active changes and archive contents. It performs
deterministic exact-behavior and Fingerprint structural analysis and writes no
audit state.

Exact behavior includes backgrounds, step values, docstrings, tables, and
examples while excluding title, path, and tags. Structural evidence uses the
existing masked assertion shape/subset and Fingerprint fact model; it does not
compare Given/When action structure.

## 8.6 Filesystem-derived State

There is no lifecycle state file. `state` walks the effective manifest graph and
observes generated paths:

| Observation | State |
|---|---|
| No generated path exists | `Pending` |
| Some generated paths exist | `Partial` |
| All ordinary paths exist | `Done` |
| `tasks.md` contains unchecked boxes | `Partial` |
| `DESIGN-REVIEW: PASS`, `REVIEW: PASS`, or `STATUS: DONE` is present where required | Marker contributes to `Done` |
| Artifact id is listed in `.givn-skip` | `Skipped` |

`next_required` and `all_required_done` use the effective graph and treat
`Skipped` as satisfied. `apply.requires` separately determines the archive
gate. State inference does not edit or repair artifacts.

Archive provenance is a separate, receipt-backed state layer:
`givn/archive/<id>/verification.json` records `pending` or `proven`, and
`status`, `instructions`, and the archive itself derive "unproven candidate"
from it. A legacy archive directory without a receipt is historical and is
never re-published; it is not an unproven candidate.

## 8.7 Skill and Rule Generation

`givn skills sync` renders the standard workflow skills and writes them only to
the persisted target selection:

- `.opencode/skills/<name>/SKILL.md` and `.opencode/commands/<name>.md`;
- `.agents/skills/<name>/SKILL.md`;
- `.claude/skills/<name>/SKILL.md`;
- `.omp/skills/<name>/SKILL.md` and `.omp/commands/<name>.md`.

The standard skills and commands are direction-only: each file carries its
trigger metadata (the frontmatter `name` and `description`), invocation-specific
orchestration (change context, subagent boundaries, stop conditions,
reporting), and a direction to `givn instructions <id>`; rule content is served
at call time and never copied into the file. Every workflow step has an
instruction id, and every instruction id a generated file names resolves
through the CLI in the same Givn version or a later one, in the context its
direction names — an artifact direction carries a resolvable change context, a
policy or step direction resolves without one; the acceptance suite grades
that. A full copy installed by an earlier Givn version is rewritten to the
direction by the next synchronization or upgrade; a disabled addon's files are
removed by the existing reconciliation; a project-owned replacement stays
byte-for-byte and receives no injected text.

A project extends one instruction's built-in rules through the `extensions:`
map in `givn/config.yaml`: an instruction id maps to project-owned rule
strings, appended to the served content under a marked `## Project extension`
section. The extension is additive — it never overrides, suppresses, or
reorders the built-in rules and never changes steps, order, or gates. It is
read on every request, never written into generated files, and never part of
the managed upgrade revision; an unknown instruction id or a malformed entry
warns and is dropped, and the built-in rules are served alone. When a
project-owned replacement of the instruction facet exists for the same id, the
replacement is the base the extension extends; the replacement file itself is
never modified.

A target that owns a shell integration also receives its attribution
integration: `.opencode/plugins/givn-attribution.js`,
`.claude/hooks/givn-attribution.sh` with `.claude/settings.local.json`, and
`.omp/hooks/pre/givn-attribution.js`.

The target selection is stored in `skills.targets` in `givn/config.yaml`; an
absent, empty, or unknown-only list falls back to opencode when `givn skills
sync` runs, so projects initialized by older versions keep syncing.
Initialization itself requires a selection: the interactive prompt opens with
`.opencode` marked and re-asks — with an error naming the requirement — until a
target is marked, while an empty or unknown-only answer and EOF without the
`--agents` flag refuse with a non-zero exit before the first write, naming
`.opencode`, `.agents`, `.claude`, and `.omp` plus the `--agents` flag.

The implementation guidance states the revision-message rule: every revision a
change records carries the change's release note (authored in its proposal) as
its message subject, with the scenario title in the message body. The rule
lives in the `tasks` instruction; the project-start block renders its resident
rules (the task-checkoff and commit obligations) from that same
`instructions/tasks.md` asset, so the block's text and the served instruction
cannot disagree. The rule is guidance only — `givn commit` neither composes nor
validates the subject.

Generated guidance is written for the consumer project. Every file reference
it carries must resolve in the project that receives it: a file the project
has, a location a named creation step creates, a project-owned destination
the guidance itself defines, or a skill resolved by name across the
configured agent targets. Every instruction id a direction names must resolve
through the CLI in the same Givn version or a later one, in the context the
direction names: an artifact direction carries a resolvable change context,
and a policy or step direction resolves without one (an artifact request
without a resolvable change context refuses by name). A reference into the
Givn distribution (`assets/…`, a repository-only document) is a defect; the
acceptance suite grades every generated skill, command, and printed
instruction against both rules.

`AGENTS.md` is user-owned outside givn-managed fenced blocks. The sync path may
create or replace the unconditional `givn-project` block and, when enabled, the
`absolute-mode` block. It does not rewrite surrounding user content. The block
carries the direction to `givn instructions`, the dialog-policy obligation,
and its resident task-checkoff and commit obligations rendered from the
canonical `instructions/tasks.md` asset. The pending
decision-ready-question contract puts policy discovery in `givn-project`, not
behind a tone addon; the addon defers question content to the effective dialog
policy. Givn never stages `AGENTS.md` under either rules policy; committing it
is the developer's explicit act.

The generated-rules policy is a flat scalar in `givn/config.yaml`:
`rules: local-only` (the default when absent) or `rules: versioned`. An
unrecognized value warns and continues as `local-only`. Under `local-only` the
strict rules — every generated rule except the `AGENTS.md` fence — are kept out
of the repository through a fenced block in the tracked ignore file
(`.gitignore`, `.fossil-settings/ignore-glob`) and a local exclude
(`.git/info/exclude`, the Fossil `ignore-glob` setting). The block carries
prefix patterns for the reserved namespaces (`.opencode/skills/givn-*/`,
`.opencode/commands/givn-*.md`, `.opencode/commands/grill-*.md`,
`.omp/skills/givn-*/`, `.omp/commands/givn-*.md`, `.omp/commands/grill-*.md`,
and the fixed integration files), never individual rule names; tracked rules are discovered
by a path predicate instead. A tracked strict rule
becomes local-only through `git rm --cached` or `fossil rm --soft`; the file
stays on disk, and the staged removals join the upgrade's managed revision or
wait for the developer after `givn init`. Under `versioned` the fenced blocks
are removed and the rules are repository content. Every command that touches
rules reports tracked and reachable-history occurrences; `givn check rules`
exits non-zero on a violation, while init, sync, and upgrade never block.
Givn itself never rewrites history: historical occurrences are reported, never
repaired — a project that must purge them does so with its own SCM tooling
outside Givn, and the reachable local refs boundary (remotes and forks are
outside the guarantee) is named, not acted on.

## 8.8 Error Handling and Exit Codes

Library-specific errors use typed errors and application paths add context.

`givn lint` returns:

- `0` for clean;
- `1` for a parse error;
- `3` for an active duplicate scenario title;
- `2` for `@wip` or PENDING.

Precedence is `1 > 3 > 2 > 0`. Shape, subset, and long-scenario findings are
warnings and do not change lint's exit code. Other commands return zero on
success and non-zero on failure.

## 8.9 Coverage Gate

Coverage is an archive-level addon, not a default lifecycle artifact. The
project-owned commands live in `givn/commands.yaml`:

```yaml
verify:
  command: "./run-tests.sh"
  e2e_command: "./run-tests.sh --e2e"
coverage:
  measure_command: "./measure-coverage.sh"
  merge_command: "./merge-coverages.sh"
  non_e2e_path: "coverage/non-e2e-cobertura.xml"
  e2e_path: "coverage/e2e-cobertura.xml"
  output_path: "coverage/cobertura-coverage.xml"
```

The measurement command must create distinct non-E2E and E2E Cobertura reports;
the merge command must create a fresh output report. Givn runs the two
project-owned commands, validates the source reports, requires them to be
distinct, and validates the merged report before publishing it. The merge
arithmetic belongs to the project's merge command; this repository's merge
script unions the per-line data so a line measured by both reports counts once
and counts as covered when either report hit it (summed counters would count
every shared line twice in the denominator), while branch counters stay summed
because branch identity is not retained by the parser (documented limitation).
Givn updates only its named README blocks. A coverage
failure leaves the archive candidate unproven in place — the permanent
corpus, README, and coverage files are not rolled back. A zero branch
denominator is rendered as `n/a`, not as a measured zero percent.

### 8.9b Semantic Migration Evidence

Corpus migrations use a deterministic receipt separate from the permanent
specification tree. `givn spec coverage --format json` records behavior hashes,
use-case ownership, capability, E2E state, interaction mapping, and declared
Persona impact. `givn check migration --change <id>` compares before and after
snapshots, writes `coverage-comparison.json`, and blocks missing behavior, lost
E2E evidence, or lower interaction coverage.

When source evidence is present, the same check compares line and branch
coverage ratios and writes `source-coverage-comparison.json`. All evidence stays
with the active migration change until archive, where it becomes historical
review evidence. A migration ledger remains the human-readable explanation for
every move, split, merge, fragment classification, creation, rename, or
retirement.

## 8.9a README Completeness Gate and the Development Managed Block

When the readme addon is enabled, the README is a givn-managed document:

- The scaffold template marks every request-for-content section with a
  `<!-- givn:todo: <expectation> -->` open placeholder line and embeds the
  development managed block (`<!-- givn:begin:development -->` /
  `<!-- givn:end:development -->`) rendered from the effective command file.
- A README counts as managed when it carries at least one management marker:
  any `<!-- givn:begin:` line, any `<!-- givn:todo:` line, or a legacy
  scaffold text. Enabling the addon on a hand-written README therefore blocks
  the archive until the README is adapted or the addon is disabled.
- The previous scaffold style's placeholder sentences are matched as an exact
  string list (`git clone <repo>` is deliberately excluded — real quick-starts
  contain it). Legacy findings report the README migration as remediation.
- `givn readme check` reports open placeholders, remnants (`TBD` license line,
  `{{PROJECT_NAME}}`), damaged or unmanaged/missing state, and development
  managed block damage (exactly one begin/end marker present). It exits
  non-zero while any finding remains.
- The archive completeness gate runs the same check before any mutation and
  blocks with a per-finding report. The development and run managed blocks are
  refreshed during finalization, after verification and coverage; refresh
  happens only when both markers are present, and a failure leaves the archive
  candidate unproven without restoring the previous README.
- The review artifact records a machine-checkable README impact decision:
  `README-IMPACT: none` or `README-IMPACT: updated - <sections>`. The
  integrity check refuses review sign-off without it while the readme addon is
  enabled.

## 8.10 Configuration Versioning and Command Files

`givn/config.yaml` is the structural, documented scaffold. It contains the
version marker, addon flags, overlay examples, and skill-target setting. The
test-runner and coverage values are in the separate `givn/commands.yaml` file.

`givn upgrade` re-renders the scaffold, preserves uncommented user blocks, bumps
`givn_config_version`, synchronizes generated integrations, and leaves legacy
user values for review where it cannot map them. Addon flags are preserved;
enabling a new addon remains an explicit `givn addons enable` operation.
Ejected facets are not automatically replaced.

For every managed upgrade, it refuses a project without a usable backend or a
dirty checkout before any write, stages only a calculated list of Givn-owned
paths, and creates one commit if the managed files changed. A declared migration route adds an aggregate
maintenance change under `givn/changes/` and a second commit containing its
ordered bundle plan. Existing active plans are detected through their visible
`migration.yaml` metadata; archived plans are historical and do not block a
later managed upgrade.

The embedded `migrations/catalog.yaml` is the release policy. Each bundle names
its source and target versions, deterministic order, and Proposal/Design asset
paths. The resolver follows multiple bundles in sequence and treats later
patch releases within a covered minor line as compatible. A transition without
a declared route is migration-free and never creates a maintenance change.

The maintenance change is deliberately not a product feature: it must not add
a synthetic scenario to `givn/specs/`. The public migration guide is an ordered
LLM prompt pack for the remaining project-specific work, including active
change review, overlap remediation, and Arc42/ADR reconciliation.
Normal design-review, tasks, review, and archive gates remain authoritative.

For the maintenance change itself, a `specs` skip marker is an explicit
no-product-delta signal. Global verification uses the permanent feature path.
Archive
still requires completed tasks and review artifacts and does not turn this
exception into a general bypass for product changes.

## 8.11 Addons and the Arc42 Documentation Artifact

An addon is embedded behavior toggled through `addons.<name>` in the project
config. The embedded default flags are false, but the current `givn init` path
enables every registered addon unless `--no-addons` is supplied.

For Arc42 specifically, `givn addons enable arc42`:

1. writes `addons.arc42: true` to `givn/config.yaml`;
2. creates `docs/arc42/`;
3. writes missing embedded chapter/index files without overwriting existing
   files; and
4. makes the `arc42-docs` artifact effective after `design`, with archive and
   design-review dependencies.

The durable output path is always `docs/arc42/`. The current code has no
effective `arc42.output_dir`, no effective chapter list, and no cascade for
individual durable chapters. The artifact marker is `arc42.md` in the change;
the durable chapters are side-effect files maintained by the agent. Disabling
the addon removes it from the effective workflow but does not delete existing
Arc42 files.

## 8.12 Hooks, Command Execution, and Mandatory Checks

There are two distinct mechanisms.

### 8.12a Commit attribution environment

OpenCode and Claude integrations set the process-scoped Givn attribution
variables for every shell command they prepare. The values identify the active
model, reasoning mode, and harness, using `unavailable` where a harness has no
value. Claude exports these variables for the complete Bash command so compound
commands and shell wrappers such as RTK preserve them. The checked-in Git
`commit-msg` hook consumes the variables only when `GIVN_ATTRIBUTION_ACTIVE` is
present, stamps only the managed co-author attribution, and never blocks a
commit when context or the installed binary is absent.

### Configurable post hooks

An artifact may declare `post` entries. Entries can name `verify`,
`verify-e2e`, `lint`, or `integrity`, or can be arbitrary shell commands. The
command is executed through `sh -c` with `GIVN_FEATURES` set. Custom shell hooks
may use `allow_failure: true`.

### Mandatory binary checks

When an explicit artifact is passed to `givn check`, `src/cli/check.rs` first
runs mandatory checks:

- `tasks`: change-delta WIP protection and `verify.command`;
- `review`: `integrity`, the run-declaration check, and the named-change
  overlap gate. Review does not execute the test suite; scenario execution is
  proven by the archive verification receipt.

These checks are not controlled by `post` and cannot be disabled by merely
removing a hook entry. Removing the `tasks` or `review` artifact changes the
effective workflow itself and therefore no longer exposes that artifact check.
Built-in hook names also force `allow_failure: false` when they occur in a post
list. The embedded default manifest currently has empty `post` lists for tasks
and review; older generated comments showing `tasks.post: [verify]` or similar
are not the effective default.

`givn check` without an artifact runs only configured post hooks for applicable
outputs. It does not invoke the explicit mandatory task/review branch.

`verify.command`, `verify.e2e_command`, and coverage values come from the
effective overlays, normally `givn/commands.yaml` in an initialized project.
The embedded `givn missing-testrunner` value is only the manifest fallback when
no later project command replaces it. The current generated command template
uses `./run-tests.sh` and `./run-tests.sh --e2e` instead.

## 8.13 Reverse-engineering Flow

Reverse engineering is a peer flow rather than a normal proposal/design flow.
It produces a descriptive feature file, `tasks.md`, and `review.md`; proposal,
design, and design-review are skipped explicitly. Characterization records what
the existing system does. It may introduce testability seams but does not fix
behavior. Suspected bugs and dead code become follow-up candidates.

Both normal implementation and characterization use the shared
`givn-steps` RED/GREEN/REFACTOR mechanics. The empty-baseline merge can create a
first permanent feature when no permanent text exists.

## 8.14 Arc42 Implementation Conformance Review

Arc42 has two review points when enabled:

1. `design-review` checks the planned chapter assessment before tasks.
2. Completion review compares durable chapters and change-level `arc42.md`
   against design, mapped tasks, and implementation evidence.

The mandatory integrity path mechanically checks each resolvable change
assessment marked `Affected: Yes` and requires that the corresponding chapter
file differs from the embedded template. If no corresponding file can be
resolved, the current helper skips that row rather than failing. The prompt-layer
review supplies the semantic judgment; the binary does not infer architecture
from source names.

The reviewer records `ARC42 CONFORMANCE: CLEAN` only after the evidence rows are
resolved, then can record `REVIEW: PASS`. When the addon is disabled, the
conformance block is not applicable.

## 8.15 ADR Qualification and Canonical Sources

The complete ADR qualification procedure and MADR template are owned by the
resolved Arc42 addon instruction. Design-review guidance delegates to that
source and does not reproduce its procedure or verdict schema. Generated skills
and commands are snapshots; project overrides may replace the effective
instruction through the normal cascade. The resolved instruction is the sole
normative source for the qualification dimensions, verdict schema, and routing
values.

Current architecture facts have one home in the relevant Arc42 chapter;
observable behavior has one home in Gherkin; implementation details have one
home in code or its design artifact; and workflow, prompt, test, coverage,
licensing, roadmap, and repository rules have one owning project document. An
ADR stores the rationale for one independent durable architecture decision and
links to those facts. A refinement amends an existing ADR. The runtime does not
infer architectural significance or validate the LLM verdict.

Three falsification tests make the gate fail fast, applied after the five
dimensions: single-change reversibility (implementable and reversible within
one change, one component); no multi-change constraint (constrains fewer than
two future changes or components); and the product-behavior exclusion
(observable workflow behavior ownable by a use case plus Gherkin fails the
lower-level-artifact dimension unless it also moves a component, authority, or
deployment boundary stated in the verdict). The tests classify by kind, not
count; there is no numeric ADR cap.

The structured verdict is recorded in the change's Arc42 impact assessment
before any MADR file is created; the ADR template ships blank and is filled
from that record. For every proposed `NEW_ADR`, design-review must name the
cheaper home — the specific canonical artifact that would own the rationale
instead — and a missing challenge is a finding. The chapter-selection guidance
treats correlated rows as a double-check; an affected chapter does not by
itself qualify an ADR. Technology-decision justification belongs in
`design.md` and is not promoted to ADR candidacy unless the falsification
tests pass.

## 8.16 The Run Declaration and `givn run`

The `run:` block in `givn/commands.yaml` declares how the built product starts:
`command`, `product_type` (`server` | `cli` | `library`), exactly one port
source for servers (`port` or `port_env`), `ready_path`, and
`ready_timeout_s` (default 60). It is applied as a replace-whole overlay unit,
identical to `verify` and `coverage`: a partial block yields defaults for the
fields it omits.

`givn init` scaffolds the block with the sentinel command
`givn missing-run-command`; `givn upgrade` backfills the same scaffold into
existing projects through the managed allowlist. `givn missing-run-command`
(hidden) prints configuration guidance and exits 1 so a copy-executed sentinel
explains itself.

Deterministic validation rules (no heuristics, no repository inspection):

| Rule | Condition | Kind |
|---|---|---|
| R1 | command empty or sentinel (fires only for unset/server/cli types — a library may keep the sentinel) | `UNCF` |
| R2 | product_type unset | `UNCF` |
| R3 | server without `ready_path` | `MISS` |
| R4 | server with neither `port` nor `port_env` | `MISS` |
| R5 | server with both port sources | `FORB` |
| R6 | server-only field on a cli/library product | `FORB` |
| R7 | server with `ready_timeout_s: 0` | `MISS` |
| R8 | relative `ready_path` | `MISS` |
| R9 | first command token on the no-op list (`true`, `:`, `echo`, `printf`, `sleep`, `cat`, `test`, `[`) or command `exit 0` | `NOOP` |
| R10 | library with a real command | allowed, rendered |

With product_type unset, R3–R8 are skipped and R9 still runs. The same
findings block `givn check review` and — before the README completeness gate,
independently of the readme addon — `givn archive`.

The README carries a managed run block (`<!-- givn:begin:run -->` /
`<!-- givn:end:run -->`). Unconfigured, its body is exactly one
`<!-- givn:todo: set run.command and run.product_type in givn/commands.yaml -->`
placeholder, so the standard open-placeholder and damaged-block mechanics
apply. Configured, it renders the start command; for a server also the port
source, readiness path, and timeout; for a library a single "nothing to start"
line. The block is refreshed at archive time next to the development block.

`givn run` executes the declaration. Its machine contract: the ready line
`givn run: ready at http://127.0.0.1:<port><ready_path>` on stdout after the
first 2xx probe response; exit 0 for a library and for clean shutdown; exit 1
for preflight findings (printed to stderr) or `--port` without `port_env`;
exit 2 on readiness timeout; the child's exit code forwarded exactly for early
server exits and cli products; 130 on SIGINT and 143 on SIGTERM, both forwarded
to the child first (unix-only). The readiness probe is a blocking
`std::net::TcpStream` HTTP/1.0 request — 2xx accepts, everything else retries
until the deadline; no HTTP client dependency is introduced.

## 8.17 Visual Review Discipline

A consumer interface whose evidence is what the consumer sees (web UI) or
reads (terminal) is a visual interface. A capability whose interface renders
HTML, CSS, or JavaScript in a browser is a Web UI even when it needs only a
page request and exposes no controls; the browser driver rule then applies and
an HTTP client is not a valid driver. A change that introduces or visibly
changes a screen records a `## Visual Design Contract` section in its
`design.md`; the contract is style-level, never pixel-level.

The contract declares, in machine-readable lines before its prose:

- `- Interface: web | terminal | none` — `none` or an absent section means no
  visual contract and the review gate is exempt;
- `- Viewports: mobile (360x780), desktop (1920x1080)` for web — the default
  pair is the Samsung S24 class and Full HD;
- `- States: empty, loading, error` for web — optional; when present, each
  declared state needs committed evidence;
- `- Design system: docs/design/design-system.md` — the durable project style
  source.

The evidence path convention is fixed, not a parsed contract line:
`givn/changes/<id>/evidence/visual/<scenario-slug>/<viewport>.png` (web),
`.../<viewport>--<state>.png` for each declared state, or
`.../<scenario-slug>/transcript.txt` (terminal).

The canonical `specs` instruction owns the visual baseline: at every
interaction, controls are labeled, no internal identifier or state name is
visible, the page has a real title and heading, no visible content is
duplicated, the error state has a heading and a way back, the configured
language is used, and reduced-motion is honored. Every delta `@e2e` scenario
of a web interface commits one final-state screenshot per contract viewport;
a terminal interface commits a transcript. Per-assertion captures are debug
artifacts under `reports/` and are not review evidence.

Review records `VISUAL-REVIEW: PASS` only after the evidence is complete, the
baseline holds, and a fresh-context vision review per confirmed Persona has
been dispositioned; the disposition is recorded as a `PERSONAS:` line naming
at least one confirmed Persona for a declared visual interface.
`VISUAL-REVIEW: UNVERIFIED` records that the active harness cannot process
images; it blocks until a human signs off. A `DEFERRAL:` line in `review.md`
requires a matching `OPERATOR-DECISION:` record, so a project constraint
never silently defers a mandatory check. A style improvement that stays
within the contract updates the contract instead of failing review. The
binary enforces the verdict marker, the persona disposition, the deferral
pair, and the final-state plus per-state evidence completeness in the
integrity hook; the persona judgment, the classification reasoning, and the
baseline assertion quality remain review guidance, not runtime inference.

## 8.18 SCM Backend Selection

Repository-state commands do not assume Git. `givn commit` creates one
attributed revision through the selected backend on both backends, and the
attribution plumbing is installed only where the backend supports it: Git
receives the shared hook and its local activation, Fossil relies on the
`givn commit` path and receives no hook. A single seam
(`src/scm.rs::select`) probes the project root and returns a backend handle or
a named refusal:

- a Git worktree selects Git;
- a Fossil checkout selects Fossil when the `fossil` executable is available;
- both markers at the root are refused as ambiguous;
- no marker is refused as "no version-controlled project";
- a project root nested inside a larger Git worktree is refused by name;
- a Git or Fossil marker without its executable is refused by name.

Detection order is fixed (Git first, then Fossil); ancestor Fossil markers are
ignored. No configuration key declares the backend (ADR-071). Refusals name the
project root and the missing prerequisite, mutate nothing, and return before
any write. The Git backend exposes clean-state witness, revision identity,
scoped staging (force only for the calculated managed allowlist), and scoped
commits; the managed upgrade consumes them. The Fossil backend exposes the
clean-state witness (including `EXTRA` entries), revision identity,
witness-bounded publication, allowlist staging (`fossil add [--force]`, force
only for the calculated managed allowlist), and revision-content checks; the
archive and the managed upgrade consume them. Publication is witness-bounded
on both backends (ADR-072): the exact dirty set is validated against the
resolved publication paths before anything is recorded.

## 8.19 The Change Seed Lifecycle

A change seed (`givn/ideation/<topic-id>/changes/<change-id>.md`) is the handoff
record between ideation and proposal planning. Seeds are cut by deliverable
guarantee, not by artifact mapping: one seed is one change whose guarantee is
true end to end and can ship alone; a seed may cover several use cases or
fragments, and one use case may yield several seeds. No seed may create an
intermediate state that makes a confirmed rule untrue — a setting no command
honors, a decision no artifact applies. It carries a fixed schema (use-case
IDs, topic source, Persona IDs, accepted decisions with their option
treatment, full open questions, dependencies, and the suggested start order)
and a status line as the first line after its title:

```text
Status: ready
Status: proposed by <change-id>
Status: implemented in <change-id>
Status: superseded by <reference>
Status: withdrawn
```

Handoff writes `ready`; proposal planning stamps `proposed by <change-id>`
and records the seed path in the proposal's Use-Case Context; archiving stamps
`implemented in <change-id>` or `withdrawn` with a one-line reason, or records
"no seed". These transitions are authoring guidance, not runtime code: no
command reads or writes a seed except the advisory lint scan. A superseding
decision edits only the status line; the seed's original text stays.

The default whole-tree `givn lint` scope reports advisory `seed_status`
findings (label `[SEED ]`) for a missing or unrecognised status line, a
`proposed by` change that does not exist, and a `proposed by` change that is
already archived. The scan parses only the first `Status:` line, is a no-op
without `givn/ideation/`, and never changes the exit-code contract.

## 8.20 Term Introduction Discipline

The ideation term record (`givn/ideation/<topic-id>/domain-terms.md`) uses a
fixed entry schema: term, status (`candidate`, `confirmed`, `rejected`),
one-sentence definition, contrast ("X, not Y"), first use (file:line), and the
collision check. A `candidate` needs only the definition and first use; a
`confirmed` term requires the term introduction test, a recorded
search-before-coin check, and the user's verified name (asked immediately when
the name fails, otherwise at the phase's term check); a `rejected` term keeps
its definition and reason.

The search-before-coin procedure is executed and recorded before `confirmed`:
grep the durable glossary for the term and its synonyms; grep every sibling
topic's term record and the repository; record the commands and hit counts.
"No collision" without recorded commands is not a record. The term
introduction test requires a genus and differentia definition, a contrast
pair, and one concrete domain sentence; a term whose genus slot cannot be
filled is not ready.

This is authoring guidance, not a machine check: the generated ideation and
domain-modeling skills carry the schema, the procedure, and the test, and the
ideation quality review verifies recorded checks. Handoff proposes only
`confirmed` terms for the durable glossary and reports `rejected` terms with
their reason. No parser, lint, or glossary gate is introduced.

## 8.21 Use-Case Model Discipline

A use case is a contract between one primary actor and the system: the
actor's single goal as observable outcome, with the success and minimal
guarantees the system makes while pursuing it. The model's eight scoping
rules gate authoring: one goal and one primary actor; goal-shaped rather than
component-shaped names; level discipline (`!` owns capabilities directly, `+`
owns only child leaves, a leaf with three or more distinct actor goals is
promoted and split); mutation containment; extension discipline; precondition
and flow consistency; implementation independence; and relationship
semantics.

The durable document `docs/use-case-model.md` carries the definition, the
rules, the authoring requirements, the ten review rejection criteria (R1–R10),
and the worked before/after example. The generated ideation guidance carries
the definition, the rules, and the authoring requirements; the review
instruction carries the criteria; the specs instruction and the spec skill
carry the capability placement test for new-capability routing; and the
generated `usecase.md` template carries the authoring conventions.

This is authoring guidance, not a machine check: no parser warning, router
rule, or archive-time capability move is introduced. The one-capability-leaf
warning and every other lint finding are unchanged.

## 8.22 Persona Identity and Reuse

A persona is a stable identity, not a topic stance. A new persona exists only
when it differs from an existing one in at least one of three disjunctive
criteria: authority boundary (what it may do or decide), evaluation capability
(what it can judge), or own vocabulary (terms it uses or refuses). The
ideation guidance carries the persona schema: a stable core (role, authority
boundary, evaluation capability, own vocabulary, success criterion) and an
append-only running record with a dated topic-stances appendix; topic-specific
demands never enter the stable core.

Reuse comes first: inventory `givn/personas/` and every sibling topic's
`personas/` against the stakeholder types, match by ID or record a
one-sentence new-segment justification naming the distinguishing criterion,
keep a reused topic file to the standard header line (`Reuse:` / `New:`)
plus new record entries, and append record entries to the project persona at
handoff with a user-confirmed merged diff. The topic file stays as historical
record; the biography lives in one place.

Reference validation is deliberately asymmetric: `givn lint` reports an
advisory `[PERS ]` finding when a `## Personas` entry in an active change
use-case document or an ideation use case resolves to no project or topic
persona file, while `givn check review` enforces the resolution for a
declared visual interface's `PERSONAS:` line. The reasoned
`none — <reason + operator confirmation>` path stays valid for a change
without a user-visible surface. User confirmation before use, the review
duty, the cap (at most three segments per stakeholder type, about ten total),
advisory findings with user arbitration, and no auto-promotion are unchanged.

## 8.23 Guarantee Qualifiers and the Decision Ledger

A use-case rule or guarantee that depends on an unresolved question carries
`(open: Q<N>)` after the statement, naming the question in `questions.md`.
The qualifier stays until the question closes, and closing the question
removes it in the same edit; an unqualified guarantee whose prerequisite is
open is a review inconsistency finding. The canonical wording lives in the
ideation skill and is referenced from the review instruction.

Proposal questions carry stable `D<n>` IDs with the dialog policy's option
treatment and a disposition: **answered** with its source, **carried** with
an owner and the deciding artifact, or **out of scope** with a reason. For a
change sourced from an ideation seed, every intersecting seed question must
be dispositioned before specs; the rule is advisory for other changes, and a
bare "resolved in `design.md`" is not a disposition. The design records
`Decides D<n>: <decision> — <where it is realized>` for every question, and
design-review verifies that each `D<n>` resolves or is carried with a named
owner; an unresolved, unrecorded item is a finding. The qualifier and the
ledger are authoring contracts, not runtime checks.

## 8.24 Finding Severity and Route Confidence

Every lint finding kind is classified in one data-driven table as `advisory`,
`finding`, or `blocking`. Blocking covers parse errors, duplicate titles,
empty features, use-case document errors, and the use-case graph findings;
finding covers WIP and e2e-intent; advisory covers shape matches, subsets,
long scenarios, seed status, and persona references. The classification is
descriptive: the existing exit-code flags still drive the exit code, so
advisory-only runs exit 0, and the prose output appends the severity after
the existing short label while the JSON output carries a `severity` field.
New finding kinds register in the one table without touching rendering.

`givn spec route` reports an explicit confidence verdict. Scoring weights
ownership evidence above lexical overlap: command-surface invocations,
structural tokens, the owning use case's Goal and Rules text, and the
capability's interaction consumer actions; a candidate with no goal ownership
and no command overlap is penalized. No signal is reported when there are no
candidates, when the top two candidates tie within `0.01`, or when the
new-use-case path has a tied or below-floor top affinity; a no-signal report
prints its reason and omits the ranked list. The recorded routing decision in
the proposal remains the declaration.

## 8.25 The Development Record and the Development Report

Givn keeps one local, append-only record of the development process at
`givn/history/events.jsonl`. Each line is one entry with an id, a timestamp, one
scope, a kind, a bounded label, a measured duration, an outcome, and an optional
parent. The record is not derived state and not a gate: `src/state.rs` keeps its
filesystem-derived contract (ADR-014), and no gate reads the record. Entries are
ordered by their `at` end timestamp rather than by position in the file, the
`id` is minted at append time under the store's append lock (so two Givn
processes appending at once never collide), and a `parent` link only ever names
an id the same process minted.

**Ownership.** ADR-073 fixes the boundary: the record's format has exactly one
owner (`src/history.rs`), and harness integrations deliver activity through the
one intake command `givn history ingest` rather than appending lines themselves.
A second reporter therefore costs one generated asset and no format work, and a
stale reporter degrades to rejected events with a warning instead of a corrupt
store.

**Reporters.** Three supported harnesses execute an agent and get a generated
reporter — omp, opencode, and Claude Code — while the `agents` target is a
skills mirror with no runtime and gets none. Each reporter is installed beside
that harness's commit-attribution integration and is discovered by the
harness's own convention (`pre` hook factory, plugin, or registered command
hook); none of them imports a harness type package, so no dependency pin can
drift under the asset. Their observable asymmetries, all derived from what the
installed versions actually expose, are deliberate: omp and opencode report
in-process and stamp their own timestamps where their events carry none, while
Claude Code runs one process per hook, so its mapping, its pending start
timestamps, and its session-record token read live in the binary and its
generated asset stays a thin shim. A signal a harness cannot deliver — token
usage on a background subagent launch, a duration broken out of permission
waiting — is recorded as missing, never as zero, and the summary states how many
entries were unmeasured.

**Best-effort discipline.** Every write is best-effort: an unwritable store
produces one warning and the recorded command keeps its own exit code, and a
rejected reported event never fails the agent action it reports. The record
reports its own gaps instead of hiding them — the reporter keeps the number of
events it dropped when its buffer overflows or a flush fails, and the report
renders a record-gap notice.

**Scope rule.** Every entry belongs to exactly one scope: the active change, the
active ideation topic, or the project. Resolution is change, then topic, then
project, and each scope is claimed only when it is unambiguous — exactly one
active change (the rule `givn instructions` already applies), then exactly one
unfinished ideation topic (a topic whose `session.md` carries no handoff
completion section); several candidates at one level fall through to the next,
because a recorder never guesses between two. Work outside a change or an
ideation is recorded rather than discarded, because "where did the time go" does
not stop at the workflow boundary, and the project window renders it in the
report's "outside a scope" band. The intake payload's `change`/`topic` are
optional overrides; a named scope that does not exist is a rejected event, never
a silent fallback.

**Privacy bound.** An entry keeps metadata plus one label truncated to 120
characters. Complete tool inputs, complete command lines, and complete question
text are never stored. The bound lives in one function (`bound_label`), so the
privacy rule has one enforcement site rather than one per capture point.

**Ignore block.** Givn writes and maintains a fenced `# givn:begin:history`
block through the same backend switch the rules writer uses: in a Git worktree
in `.gitignore` plus the worktree-local exclude file (`git rev-parse --git-path
info/exclude`), in a Fossil checkout in `.fossil-settings/ignore-glob` plus the
repository-local `ignore-glob` setting. It applies under both rules policies,
because the record is development data rather than a generated rule; this is a
separate block from the `# givn:begin:rules` block so neither policy can
silently drop the other's entries.

**Report packaging.** `givn timeline` writes one self-contained HTML file from
the embedded report asset: the CSS, JavaScript, and SVG are inline, the record
model is substituted into a single data slot, and nothing is fetched. The
document renders from the embedded timestamps alone, so a given record always
produces the same pixels and the browser tier can assert rendered values
deterministically. The report model is not published as JSON: the
machine-readable view is `givn history --json`, which reads the record itself,
and `givn timeline` writes its one HTML file and prints that file's path.

**Scale.** A whole ideation can run for hours to days and produce hundreds of
tasks and thousands of tool calls; the expected order of magnitude is at most
10,000 entries and roughly 2 MB per long ideation. That stays a dependency-free
append-only JSONL file: one append per entry, one sequential scan per report,
and no embedded database — DuckDB would outweigh the feature inside a
size-optimized binary, and SQLite would add a schema and migration surface for a
workload of one append and one scan. No bottleneck has been measured, and the
plain file is the interface external tools read directly. To stay readable at
that size, the timeline never draws one shape per entry: at low zoom, dense
short activities collapse into per-bucket aggregate markers naming their count
and combined duration, individual spans appear as the zoom narrows, and the
summary and ranking always cover the full record (D-23 tracks pruning and
retention).

**Browser tier.** The report is a Web UI, so its acceptance is browser-driven:
the repository's e2e tier installs Node.js (current LTS) and Playwright's
Chromium through `./setup-e2e-browser.sh`, and a JSON-line driver holds one
browser context per scenario so page state, zoom level, and open panels persist
across steps. The artifacts under test are the real generated report and the
real `givn` binary; the driver never substitutes an API call for a click.

*[Architecture documentation index](README.md)*
