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
skills and commands are snapshots and have no independent policy authority.

### 8.3b Machine-readable Instruction Contract

`givn instructions <artifact> --json` emits one JSON object with exactly the
effective artifact `id`, `generates`, `requires`, and resolved `instruction`.
Human-readable mode emits instruction text. The object is resolved from the
effective manifest and the project-overrides-first instruction cascade.

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

## 8.7 Skill and Rule Generation

`givn skills sync` renders the standard workflow skills and writes them only to
the persisted target selection:

- `.opencode/skills/<name>/SKILL.md` and `.opencode/commands/<name>.md`;
- `.agents/skills/<name>/SKILL.md`;
- `.claude/skills/<name>/SKILL.md`.

The target selection is stored in `skills.targets` in `givn/config.yaml`; absent,
empty, or unknown-only values fall back to opencode. The default `init` prompt
selects opencode on empty input or EOF and allows the three known targets.

`AGENTS.md` is user-owned outside givn-managed fenced blocks. The sync path may
create or replace the `givn-project` block and, when enabled, the
`absolute-mode` block. It does not rewrite surrounding user content. The
Absolute Mode addon is the only reason the current code intentionally manages a
policy block there.

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
the merge command must create a fresh output report. The merged report counts
every measured line once — the union of the source reports' measured lines —
and counts a line as covered when either source report hit it: both source
reports measure the same source tree, so summed counters would count every
shared line twice in the denominator. A merge falls back to summed counters
only when a source report carries no per-line data (aggregate-only reports),
and branch counters keep their summation because branch identity is not
retained by the parser (documented limitation). Givn validates the source
and merged reports, updates only its named README blocks, and restores specs,
README, and coverage files if the pipeline fails. A zero branch denominator is
rendered as `n/a`, not as a measured zero percent.

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
  blocks with a per-finding report. The development managed block is refreshed
  from the effective command file after the post-merge verify gates and before
  the coverage gate; refresh happens only when both markers are present, and a
  failure restores the README from the pre-archive backup.
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

For every managed upgrade, it refuses a non-Git or dirty worktree before any
write, stages only a calculated list of Givn-owned paths, and creates one commit
if the managed files changed. A declared migration route adds an aggregate
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

### Configurable post hooks

An artifact may declare `post` entries. Entries can name `verify`,
`verify-e2e`, `lint`, or `integrity`, or can be arbitrary shell commands. The
command is executed through `sh -c` with `GIVN_FEATURES` set. Custom shell hooks
may use `allow_failure: true`.

### Mandatory binary checks

When an explicit artifact is passed to `givn check`, `src/cli/check.rs` first
runs mandatory checks:

- `tasks`: change-delta WIP protection and `verify.command`;
- `review`: `verify.command`, `verify-e2e`, `integrity`, then the named-change
  overlap gate.

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
