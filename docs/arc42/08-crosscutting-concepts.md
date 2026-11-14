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
`givn/artifacts/<id>/instructions.md`. Skill assets have their own project override paths. `givn eject` materializes an embedded asset so the
project can own it. An existing file is protected unless `--force` is used.

The cascade is per facet. Ejecting a template does not eject the instruction,
and a project override remains effective after a binary upgrade.

## 8.2 Manifest and Overlay/Patch

The effective manifest is assembled in this order:

```text
embedded default-manifest.yaml
        + givn/config.yaml overlay
        + givn/commands.yaml dedicated schema when present
```

Artifact patches are keyed by `id`. An existing artifact is merged field by
field; a new artifact is appended; `remove: true` removes it. `requires` forms
the lifecycle DAG, `optional` affects status completion, and `apply.requires`
identifies archive prerequisites. The effective graph is rejected for dangling
references or cycles.

The current `givn/commands.yaml` is intentionally separate from the structural
scaffold. It supplies `verify.command`, `verify.e2e_command`, coverage commands
and paths, and the `run:` declaration. It is parsed by a dedicated schema, not
the generic manifest overlay, so policy keys there and executable keys in
`givn/config.yaml` are refused by name.

The Arc42 addon is special overlay sugar. When `addons.arc42` changes from false
to true, overlay processing adds a required `arc42-docs` artifact after
`design`, adds it to `apply.requires`, and makes `design-review` require it.
The addon expansion is idempotent. The artifact's generated lifecycle marker is
`arc42.md`; the durable chapter files are not manifest-generated paths.

`givn/specification.yaml` is a third configuration authority, but it is not an
overlay: it is read-only and parsed once by the strict declaration loader, it
refuses any unknown or missing key, and no command writes it except the one-time
scaffold. That scaffold runs when `ise` becomes effective and the file is
absent, and the managed upgrade writes and stages it when `ise` is already
effective and the file is absent. An existing declaration is never written or
staged. It is read only while `ise` is effective, so a project without the addon
never reads a file that happens to be present.

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

Archive merges feature files and use-case/fragment documents only. Step
definitions are durable project source and are never merged, copied, or
deleted: a delta step file may live beside the change's features, but the
permanent per-capability step file is its home, and the delta copy travels into
the archive as history. Before any mutation — and again before an unproven
candidate is verified — archive compares the step phrases bound by the change's
delta step-definition files with the phrases bound by the permanent test tree,
and refuses when one is missing. A step-definition file is a file under a
directory named `steps` or ending in `_steps`, or a file whose name ends in
`_steps.<ext>`. A step phrase is the first resolvable literal bound by a step
binding: the archive reader finds the supported binding tokens anywhere in the
file (`#[given(`, `#[when(`, `#[then(`; `@given(`, `@when(`, `@then(`,
`@step(`, `@Given(`, `@When(`, `@Then(`, `@And(`, `@But(`, `@Step(`;
`Given(`, `When(`, `Then(`, `And(`, `But(`, `Step(`; `.Given(`, `.When(`,
`.Then(`, `.Step(`), consumes each binding to its matching `)` with a
string/raw-string/regex/comment-aware counter, and reads the first escaped
string, raw string (any `#` delimiter), or `/regex/` literal; adjacent Python
literals are joined and internal whitespace is normalized. A binding whose
phrase is computed dynamically yields no phrase and is skipped.

The permanent tree is the project root minus two prune sets: any directory
component named `.git`, `.hg`, `.svn`, `target`, `node_modules`, `.venv`,
`venv`, `coverage`, `dist`, `build`, `vendor`, or `__pycache__`, and any
project-relative path starting with `givn/changes`, `givn/archive`, or
`givn/history` (component-wise, so `.git` does not match `.github`). The delta
walk prunes only the component names, so the change's own step files are read.
Both walks skip unreadable and non-UTF-8 files without failing. The refusal
names the project-relative delta file, the missing phrases (capped at 20, then
`+N more`), and the repair: merge each definition into its permanent
per-capability step file and rerun the archive. The heuristic is text-level by
design; its skipped-binding and literal-comparison limits are recorded in
chapter 11 (R-123).

The merge implementation is text-level for scenario blocks. It preserves the
existing permanent feature header and surrounding text, while a replaced or
added scenario is rendered from the parsed AST. The `gherkin` AST does not
preserve every original docstring delimiter detail.

### 8.3a Normative Authoring Guidance

The resolved `specs` instruction is the single normative source for interaction
inventories and E2E evidence. Design, tasks, and review add only
artifact-specific choreography and derived checks.

The policy requires primary assertions against the named consumer interface.
Repository or database state is secondary evidence unless it is itself the
public interface. Browser E2E must use a real browser; an in-page HTTP request is
not a browser interaction substitute. An obstacle is a design gap, not
permission to weaken the assertion.

Project overrides can replace the effective policy through the cascade. Generated
skills carry trigger metadata, invocation-specific orchestration, and a
direction to `givn instructions <id>`; they carry no rule content and have no
independent policy authority. The rule an agent follows for a step is the
extended rule: the built-in, technology-agnostic rules resolved at call time
plus the project's extension when one exists. An extension adds rules; it never
overrides, suppresses, or reorders the built-in rules, and it never changes
steps, order, or gates.

The resolved `dialog` instruction is the single normative source for phrasing
and recording user-facing questions. The unconditional project/session block,
every bundled available skill, and fresh delegated contexts direct the agent to
load it once per context before the first user-facing question. The complete
rules and the worked contrasts are served by `givn instructions dialog`; the
companion `dialog-examples` instruction carries the preserved regression
examples, and derived entry points carry short references rather than competing
copies. See that policy for question content, proportional clarification,
answer consequences, and material revision costs.

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

### 8.3c Projected Interaction Integrity

The interaction inventory is the capability-aware table in a use case or
fragment. `gherkin::interaction` compares it with the `@e2e` scenario titles in
the capability feature. It also checks the reverse direction: every `.feature`
file directly under a use-case or fragment directory must be named in that
document's `## Capabilities` list, and `- none` is an empty list. A feature file
outside the list is an `undeclared_capability` blocking finding. A directory
with feature files but no use-case or fragment document is also reported as
undeclared rather than silently skipped. A declared capability whose feature is
absent is a blocking `usecase_interaction_invalid` finding. A delta capability
that cannot be placed in the projected corpus is a blocking
`interaction_mismatch` finding rather than a silent skip. A change is accepted
only if the corpus it would leave declares exactly the E2E scenarios its
features provide. `spec::projection`
reads `givn/specs` into an in-memory `SpecCorpus`, applies the change's merge
plan and narrative documents, and runs the gate on the projected corpus.
Review and change-scoped lint report the projected findings; archive runs the
same check before the pending receipt, the rename, or a resumed merge, and
rechecks the actual permanent inventory on a merged candidate before
verification.

The findings use project-relative paths directly; no temporary path exists. A project with pre-existing drift is blocked until it repairs the
inventory; the projection deliberately does not subtract pre-existing findings,
because a red corpus is not a valid baseline. `givn lint` names every finding.

## 8.4 Project Directory Layout

```text
givn/
  config.yaml                    # structural overlay, addon flags, targets
  specification.yaml             # declared Specification roles and source roots (ise)
  commands.yaml                  # verify, coverage, and run settings
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
  archive/<id>/                  # active change moved here by archive
  artifacts/<id>/                # ejected facet overrides

docs/arc42/                      # fixed durable Arc42 output path
```

`givn init` creates the core `givn/` directories and command/config files. The
current init path enables every addon whose declared default is on unless
`--no-addons` is passed; Arc42 scaffolding therefore normally appears during
initialization, and the outline report and architecture-principles guidance are
available in a fresh project. `ise` is omitted by default and is never enabled
by a fresh init. A project opts out with `givn addons disable <name>` or by
writing an explicit `false`; there is no effective `arc42.output_dir` or
chapter-list setting, and no lifecycle cascade for selecting individual durable
chapter files.

## 8.5 The Behavior Batch

The normal implementation guidance is a behavior batch, not a commit per
scenario or per phase:

```text
TESTS:    bind the scenarios; observe a meaningful RED for new behavior
GREEN:    implement the minimum code following design constraints
REFACTOR: clean up without changing behavior, optionally using the advisory outline
COMMIT:   one coherent completed batch with explicit paths
```

Related scenarios form one batch. A batch gets one test phase, one
implementation phase, one refactor phase, and one batch commit. Individual
examples do not need their own failing test or production diff.

- A new test must fail on its assertion, not on discovery, registration, step
  binding, import, fixture, or environment startup. Fix setup failures and rerun.
- A passing test is valid when it protects a distinct already-satisfied
  requirement. Record it as existing behavior protected, make no production edit
  for that requirement, and continue.
- Record actual commands and short results in the batch task as they happen.
  Never reconstruct RED or rewrite history.
- Commit the completed batch with explicit paths. The release note is the
  message subject; the batch name and, for a single-scenario batch, the scenario
  title are in the body.
- Tick each scenario when the batch tests pass and the batch commit is recorded.
- The setup task comes first: install the runner, configure strict mode, set
  `verify.command`, create one step file per capability, and prove that an
  undefined or pending step fails. Remove the probe afterwards.
- `@e2e` tags are immutable. Removing one to bypass the E2E gate is detected by
  the verify-e2e hook and by lint.
- Deviating from `design.md` requires stopping, updating `design.md`, and
  re-running design-review.

Commit identity is machine-checked at `givn check tasks`. Empty step bodies,
downgraded E2E assertions, and test strength are checked by review; the TDD
record is authoring evidence, not a machine-audited phase history.

MANDATORY: read `givn instructions steps` and `givn instructions tasks` for the
complete binding, RED, reuse, and commit rules.

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
from it. An active change and a pending candidate may coexist only when a
previous transition was interrupted; the archive classifies the pair as
recoverable when every active artifact exists byte-identically in the
candidate, and as corrupt when it conflicts. Evidence paths (the receipt,
`results/**`, `evidence/**`, and `verification/**`) are ignored on both sides
because they are outputs, not artifacts. A legacy archive directory without a
receipt is historical and is never re-published; it is not an unproven
candidate.

## 8.7 Skill and Rule Generation

`givn skills sync` renders the standard workflow skills and writes them only to
the persisted target selection:

- `.opencode/skills/<name>/SKILL.md`;
- `.agents/skills/<name>/SKILL.md`;
- `.claude/skills/<name>/SKILL.md`;
- `.omp/skills/<name>/SKILL.md`;
- `.maki/skills/<name>/SKILL.md`.

No target receives a generated command file. The standard skills are
direction-only: each file carries its trigger metadata (the frontmatter `name`
and `description`), invocation-specific orchestration (change context, subagent
boundaries, stop conditions, reporting), and a direction to
`givn instructions <id>`; rule content is served at call time and never copied
into the file. Every workflow step has an
instruction id, and every instruction id a generated file names resolves
through the CLI in the same Givn version or a later one, in the context its
direction names — an artifact direction carries a resolvable change context, a
policy or step direction resolves without one; the acceptance suite grades
that. A full copy installed by an earlier Givn version is rewritten to the
direction by the next synchronization or upgrade; a disabled addon's files are
removed by the existing reconciliation; a project-owned replacement stays
byte-for-byte and receives no injected text. A retired generated skill is
removed only when its bytes are recognizable as Givn output; custom content at
the same path is preserved with a manual transition notice. `givn-start` is the
default entry skill; `givn-explore` is no longer generated and has no alias.

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
`.opencode`, `.agents`, `.claude`, `.omp`, and `.maki` plus the `--agents` flag.

The implementation guidance states the revision-message rule: every revision a
change records carries the change's release note (authored in its proposal) as
its message subject, with the batch name and, for a single-scenario batch, the
scenario title in the message body. The rule
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
acceptance suite grades every generated skill and printed instruction against
both rules.

`AGENTS.md` is user-owned outside givn-managed fenced blocks. The sync path may
create or replace the unconditional `givn-project` block and, when enabled, the
`communication-policy` block; it removes the retired `absolute-mode` block. It
does not rewrite surrounding user content. The project block carries the
direction to `givn instructions`, the mandatory dialog-policy obligation, and
its resident task-checkoff and commit obligations rendered from the canonical
`instructions/tasks.md` asset. The communication-policy addon controls only
presentation and documentation; it does not gate or own question content. Givn never stages
`AGENTS.md` under either rules policy; committing it is the developer's explicit
act.

The generated-rules policy is a flat scalar in `givn/config.yaml`:
`rules: local-only` (the default when absent) or `rules: versioned`. An
unrecognized value warns and continues as `local-only`. Under `local-only` the
strict rules — every generated rule except the `AGENTS.md` fence — are kept out
of the repository through a fenced block in the tracked ignore file
(`.gitignore`, `.fossil-settings/ignore-glob`) and a local exclude
(`.git/info/exclude`, the Fossil `ignore-glob` setting). The block carries
prefix patterns for the reserved namespaces (`.opencode/skills/givn-*/`,
`.agents/skills/givn-*/`, `.claude/skills/givn-*/`, `.omp/skills/givn-*/`,
`.maki/skills/givn-*/`, and the fixed integration files), never individual rule
names; tracked rules are discovered by a path predicate instead. A tracked strict rule
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

- `0` for clean, including advisory `seed_status`, `persona_ref`, and the four
  `ise` declaration placement findings;
- `1` for a parse error, an invalid use-case document, an interaction mismatch, an undeclared capability, an unplaceable delta capability, or a malformed `givn/specification.yaml`;
- `3` for an active duplicate scenario title or an empty feature;
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

## 8.9a README Completeness Gate and Managed Blocks

When the readme addon is enabled, the README is a givn-managed document:

- The scaffold template marks every request-for-content section with a
  `<!-- givn:todo: <expectation> -->` open placeholder line and embeds the
  development managed block (`<!-- givn:begin:development -->` /
  `<!-- givn:end:development -->`) and the run managed block
  (`<!-- givn:begin:run -->` / `<!-- givn:end:run -->`) rendered from the
  effective command file.
- A README counts as managed when it carries at least one management marker:
  any `<!-- givn:begin:` line, any `<!-- givn:todo:` line, or a legacy
  scaffold text. Enabling the addon on a hand-written README therefore blocks
  the archive until the README is adapted or the addon is disabled.
- The previous scaffold style's placeholder sentences are matched as an exact
  string list (`git clone <repo>` is deliberately excluded — real quick-starts
  contain it). Legacy findings report the README migration as remediation.
- `givn readme check` reports open placeholders, remnants (`TBD` license line,
  `{{PROJECT_NAME}}`), damaged or unmanaged/missing state, and damaged or
  ambiguous development/run managed markers (a reversed, duplicated, or
  multiple begin/end pair). It exits non-zero while any finding remains and
  never rewrites the README.
- The archive completeness gate runs the same check before any mutation and
  blocks with a per-finding report. Both managed blocks are refreshed during
  finalization, after verification and coverage; refresh requires exactly one
  ordered marker pair, refuses ambiguity before writing, preserves surrounding
  bytes, and leaves the archive candidate unproven on failure.
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
dirty checkout before any write, records the checkpoint, stages only a
calculated list of Givn-owned paths, and creates one commit if the managed
files changed. A declared future migration route adds an aggregate maintenance
change under `givn/changes/` and a second commit containing its ordered bundle
plan. Existing active plans are detected through their visible `migration.yaml`
metadata; archived plans are historical and do not block a later managed
upgrade. On failure it reports the checkpoint, completed writes, revisions
actually published, best-effort restoration, and pending work; it performs no
reset or history rewrite.

The embedded `migrations/catalog.yaml` is the release policy. Each bundle names
its source and target versions, deterministic order, and Proposal/Design asset
paths. The resolver follows multiple bundles in sequence and treats later
patch releases within a covered minor line as compatible. The 1.0 baseline
ships `migrations: []`, so a pre-1.0 project follows the manual transition and
never receives an embedded route or alias. A transition without a declared
route is migration-free and never creates a maintenance change.

A future maintenance change is deliberately not a product feature: it must not
add a synthetic scenario to `givn/specs/`. Its bundle assets carry the
project-specific prompts, including active change review, overlap remediation,
and Arc42/ADR reconciliation. Normal design-review, tasks, review, and archive
gates remain authoritative.

For the maintenance change itself, a `specs` skip marker is an explicit
no-product-delta signal. Global verification uses the permanent feature path.
Archive
still requires completed tasks and review artifacts and does not turn this
exception into a general bypass for product changes.

## 8.11 Addons and the Arc42 Documentation Artifact

An addon is embedded behavior toggled through `addons.<name>` in the project
config. Each addon declares its own default: the embedded default flags are
true for every addon except an opt-in addon, and `givn init` enables every
addon whose declared default is on unless `--no-addons` is supplied. A missing
key in an explicit `addons:` block resolves to that addon's declared default;
only an explicit `false` disables a default-on addon. `givn init --no-addons`
writes explicit false values for every known add-on, and
`givn addons disable <name>` is the per-add-on opt-out.

`ise` is the one opt-in addon in the current registry until promotion. Its
declared default is false, a fresh init omits its key, and it requires
`architecture_principles`: enabling it while that addon is off is refused by
name, and disabling the prerequisite while `ise` is on is refused with `ise`
named first. Both refusals write nothing. The first effective enable scaffolds
`givn/specification.yaml` from its documented shape when the file is absent and
never rewrites an existing declaration. A project that never enables `ise`
receives no new key, no new file, no new finding, and no new gate.

For Arc42 specifically, `givn addons enable arc42`:

1. writes `addons.arc42: true` to `givn/config.yaml`;
2. creates `docs/arc42/`;
3. writes missing embedded chapter/index files without overwriting existing
   files;
4. creates a missing `docs/glossary.md`; and
5. makes the `arc42-docs` artifact effective after `design`, with archive and
   design-review dependencies.

Chapter 12 is a link to the standalone glossary, not a second term table. The
scaffold never rewrites an existing chapter 12; migration from an older
glossary is agent-directed and preservation-first (see 8.27).

The durable output path is always `docs/arc42/`. The current code has no
effective `arc42.output_dir`, no effective chapter list, and no cascade for
individual durable chapters. The artifact marker is `arc42.md` in the change;
the durable chapters are side-effect files maintained by the agent. Disabling
the addon removes it from the effective workflow but does not delete existing
Arc42 files.

### 8.11a The Add-on Configuration Lock

A new change records the effective nine add-on values in
`givn/changes/<id>/config-lock.yaml` (format version 1). The file also carries
operator overrides with addon, before/after, reason, and timestamp.

While a locked change is active, `givn addons enable|disable` refuses. The
operator override is `--operator --reason "<reason>"` on an interactive
terminal; it updates the recorded value and appends the override. A forced init
refuses only when its proposed add-on set differs from the record. Review and
archive compare the current effective configuration with the lock and fail on
drift. A missing lock means a change created before the lock and is skipped;
deliberate lock deletion remains a documented residual limit.

## 8.12 Hooks, Command Execution, and Mandatory Checks

There are two distinct mechanisms.

### 8.12a Commit attribution environment

OpenCode, Claude Code, omp, and Maki integrations set the process-scoped Givn
attribution variables for the shell commands they prepare. The values identify
the active model, reasoning mode, and harness, using `unavailable` where a
harness has no value. Claude exports these variables for the complete Bash
command so compound commands and shell wrappers such as RTK preserve them. The checked-in Git
`commit-msg` hook consumes the variables only when `GIVN_ATTRIBUTION_ACTIVE` is
present, stamps only the managed co-author attribution, and never blocks a
commit when context or the installed binary is absent.

Maki exports the values from a `tool.bash.input` layer that runs before the
Bash call is parsed. The layer reads `maki.model.get()` for the focused
session's model and thinking level, rewrites only a command whose text names a
revision verb, and leaves every other command untouched. Maki 0.7.0 passes a
plain table to the layer, not the `LuaCtx` that `maki.agent.resolve_model`
requires, so a subagent command is attributed to the focused session. Maki
requires the plugin to hold `run`, `session_read`, and `session_write` for that
layer; the generated manifest grants all three, and a manifest that explicitly
revokes `session_write` is preserved with a warning rather than silently
widened.

### Configurable post hooks

An artifact may declare `post` entries. Entries can name `verify`,
`verify-e2e`, `lint`, or `integrity`, or can be arbitrary shell commands. The
command is executed through `sh -c` with `GIVN_FEATURES` set. Custom shell hooks
may use `allow_failure: true`.

### Mandatory binary checks

When an explicit artifact is passed to `givn check`, `src/cli/check.rs` first
runs mandatory checks:

- `tasks`: change-delta WIP protection, the declared E2E package boundary when
  the change lock has `ise` effective, `verify.command`, the change-scoped E2E
  scope when the delta has `@e2e` scenarios, and the E2E-bound receipt;
- `review`: `integrity`, the run-declaration check, change-scoped lint, the
  named-change overlap and retirement gates, the use-case/capability-routing
  gate (which reports every schema and routing diagnostic in one pass, with
  dependent completeness findings withheld while the title or Level is
  invalid), the shared change verification receipt freshness check including
  the E2E identity, and the scenario-commit check. Review does not execute the
  test suite; scenario execution is proven by the archive verification
  receipt. There is no outline review member.

These checks are not controlled by `post` and cannot be disabled by merely
removing a hook entry. Removing the `tasks` or `review` artifact changes the
effective workflow itself and therefore no longer exposes that artifact check.
Built-in hook names also force `allow_failure: false` when they occur in a post
list. The embedded default manifest currently has empty `post` lists for tasks
and review; older generated comments showing `tasks.post: [verify]` or similar
are not the effective default.

`givn check` without an artifact prints guidance and executes nothing. It does
not invoke the explicit mandatory task/review branch or any configured post
hook.

`verify.command`, `verify.e2e_command`, coverage values, and `run:` come from
`givn/commands.yaml` through its dedicated schema in an initialized project.
The embedded `givn missing-testrunner` and `givn missing-run-command` values are
manifest fallbacks. The shipped command template keeps those sentinels and does
not invent project scripts.

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
`design.md`; the contract is style-level, never pixel-level. The section's
presence is the Persona boundary; its `Interface:` value is the evidence
boundary.

The contract declares, in machine-readable lines before its prose:

- `- Interface: web | terminal | none` — a present section is a declared
  contract. `none` means the change renders no screen: it still requires the
  persona disposition, but no visual verdict and no visual evidence. Only an
  absent section is exempt;
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
at least one confirmed Persona for every declared contract, including
`Interface: none`, which needs no verdict and no evidence. Only a change that
declares no contract may record a reasoned `none`.
`VISUAL-REVIEW: UNVERIFIED` records that the active harness cannot process
images; it blocks until a human signs off. A `DEFERRAL:` line in `review.md`
requires a matching `OPERATOR-DECISION:` record, so a project constraint
never silently defers a mandatory check. A style improvement that stays
within the contract updates the contract instead of failing review. The
binary enforces the persona disposition for every declared contract, the
verdict marker and the final-state plus per-state evidence completeness for
`web` and `terminal`, and the deferral pair in the integrity hook; the persona
judgment, the classification reasoning, and the baseline assertion quality
remain review guidance, not runtime inference.

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
persona file, while `givn check review` enforces the resolution for every
declared visual design contract's `PERSONAS:` line, including
`Interface: none`. The reasoned `none — <reason + operator confirmation>` path
stays valid only for a change that declares no contract. User confirmation
before use, the review duty, the cap (at most three segments per stakeholder
type, about ten total),
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
empty features, use-case document errors, the use-case graph findings,
interaction mismatches, and undeclared capabilities; finding covers WIP and
e2e-intent; advisory covers shape matches, subsets, long scenarios, seed
status, and persona references. The classification is
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

**Reporters.** Four supported harnesses execute an agent and get a generated
reporter — omp, opencode, Claude Code, and Maki — and each also receives its
commit-attribution integration where the harness prepares shell commands,
while the `agents` target is a skills mirror with no runtime and gets none.
Each reporter is installed beside that harness's other integration and is
discovered or registered by the harness's own convention (`pre` hook factory,
plugin, registered command hook, or a managed fence in Maki's `init.lua`);
those written in the harness's language import nothing from a type package, so
no dependency pin can drift under the asset. Maki is also a full skills
target: it receives `.maki/skills/<name>/SKILL.md`, its development reporter,
and its attribution integration, but no generated commands. Its two
configuration files (`.maki/init.lua`, `.maki/plugin.toml`) are merged, never
replaced. Their observable asymmetries, all derived from what the
installed versions actually expose, are deliberate: omp and opencode report
in-process and stamp their own timestamps where their events carry none, Claude
Code runs one process per hook so its mapping, pending start timestamps, and
session-record token read live in the binary while its generated asset stays a
thin shim, and Maki runs the reporter as a Lua plugin whose autocmds fire
without any timestamp, whose `TurnEnd` covers only the main session, and whose
`TurnStart` is `maki-ui`-only. On Maki 0.7.0 `TaskStatusChanged` reports
subagents, `TaskStopped` reports user stops, `SessionTitleChanged` reports
titles, and `maki.task.list()` names each task's chat session and cost; the
reporter records one span per task, keeps a moved subagent open, records a
subagent's transcript tool calls under its own session, and reads question
answers and full-mode tool output from `tool_result` blocks. The earlier 0.6.3
headless observation (no subagent span, no task list without a UI) is a
version-specific historical limit, not a current one. Maki
additionally gates `.maki/` configuration on folder trust, so its reporter and
attribution integration are active only in a project the developer has trusted
for Maki. A signal a harness
cannot deliver — token usage on a background subagent launch, a duration broken
out of permission waiting, per-subagent tokens Maki never reports — is recorded
as missing, never as zero, and the summary states how many entries were
unmeasured.

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

**Engagement.** The recorder engages only when the project root carries the Givn
scaffold (`givn/`). Before the scaffold exists it writes nothing and creates no
directory, so a refused `givn init` in a bare directory leaves that root exactly
as it was; a successful `givn init` creates the scaffold and then records. The
rule keys on the scaffold and never on the VCS markers: once `givn/` exists,
every command records — including refusals and failures — and a scaffolded
project without a usable backend still records.

**Runtime state, not project content.** The store and its pending state are Givn
runtime state. The acceptance suite's project-state comparison helpers therefore
exclude `givn/history/**` when they check that a command "changed nothing": the
exclusion is scoped to the record directory, so no other path's behavior
changes, and a legitimate command entry no longer reads as a modified project.

**Capture limits, not a privacy guarantee.** An entry keeps bounded metadata — the label at 120
characters, text metadata at 240, error text at 1 000 — plus whichever capture
mode is active. The record has three modes (`record.mode`, default `on`):
`off` writes nothing new and is absolute (`--with-full-content` is ignored with
one warning and exit 0); `on` captures bounded metadata plus full question text
and no LLM-facing content; and `full` adds complete LLM-facing content (tool
inputs and outputs, assistant and thinking text, user prompts, compaction
summaries, plan content, draft text, and subagent transcripts) under the
64 KiB-per-field / 256 KiB-per-entry caps. Lowering the mode is forward-looking:
already-captured entries are retained, never purged. The intake is the single
enforcement site for every bound and enforces the mode for payloads; Claude's
in-process `harness_hook` is the second `off` site (no append, no pending write,
no ignore write). An absent or invalid value falls back to `on` with one
warning, and the key is parsed leniently by `OverlayDoc` so the invalid value
never fails the config parse. Question text is the one
exception captured in full under both `on` and `full` (up to a 4 000-character
cap), because a question is the human-workflow event the report exists to
explain. Provider wire payloads and headers, MCP protocol frames and params,
harness system prompts, credentials and environment secrets as dedicated
fields, UI dialog internals, and control-surface results are never captured at
any tier. Full content is not a secret filter: a credential that appears inside
a captured command, file, prompt, or tool output travels with that work, which
is why the default `on` mode stores no LLM-facing content and `full` is the
project's explicit decision (R-92). The label bound lives in `bound_label`; the
question and content bounds live in the intake's validators, so the capture
limits have one enforcement site rather than one per capture point. Nothing
here is a redaction, encryption, access-control, or filtering guarantee: paths,
command lines, error text, question text, and any content inside `full` work
remain as sensitive as the work itself.

**Ignore entry.** Keeping the store out of repository history splits by who owns
the destination, and the recorder itself never writes a versioned file: with a
Git worktree it re-asserts the `# givn:begin:history` block in
`.git/info/exclude` on every write (not only when the store is created, so a
store that predates the repository still becomes ignored); with a Fossil
checkout whose `.fossil-settings/ignore-glob` is absent it writes the
repository-level `ignore-glob` setting, which is never tracked and effective
while no file overrides it; and with a Fossil checkout that already has that
file it writes nothing, because the file overrides the setting. The tracked
destinations are written by the owning commands (`givn init`,
`givn skills sync`, and `givn upgrade`): a Git project's `.gitignore` receives
the same fenced block, so every clone inherits the ignore even before its local
exclude is written, and a Fossil checkout receives `givn/history/*` in the
managed entry list. A `.gitignore` written before a repository exists is the
scaffold-time safety net. Repository existence comes from the backend markers at
the project root, never from backend selection succeeding (selection also fails
for a nested root, both markers, or a missing executable). It applies under both
rules policies, because the record is development data rather than a generated
rule, and the Git destinations are their own fenced block rather than the
`# givn:begin:rules` block. The never-dirty rule is the point: the recorder runs
at command start, so a versioned write on every command would dirty the working
tree before the command did anything and `givn upgrade`'s clean-checkout
precondition would refuse a project that did nothing wrong — and no ignore
destination can suppress a tracked file's modification anyway. The owning
commands write the tracked block once, and `givn upgrade` stages `.gitignore`
through its existing managed allowlist, so the command that introduces the block
also commits it. The Fossil entry is `givn/history/*`, not `givn/history`,
because a Fossil glob must match the whole path and `*` crosses `/`.

**Report packaging.** `givn timeline` writes one self-contained HTML file from
the embedded report asset: the CSS, JavaScript, and SVG are inline, the record
model arrives as one machine-readable payload element (which the file's own
script parses) plus the text-only initial view the file carries, and nothing is
fetched. The document renders from the embedded timestamps, its embedded
change-state snapshot, and its embedded linked-ideation link alone, so a given
record, change state, and link always produce the same pixels and the browser
tier can assert rendered values
deterministically; a later state or proposal edit requires a re-render (R-96). The lane log
renders the captured facts (session and title, status, stop reason, model, cost,
context size/window, tool call id, output bytes, bounded error, parent/agent,
retry, permission mode) and one bounded, scrollable, escaped content region. A
change whose proposal or seed links an existing ideation topic also renders the
topic's ideation phases as a labeled group above the change workflow, with the
topic's own entries and totals and without merging the two scopes' figures. The
report embeds full content up
to a **8 MiB total budget**; beyond it the affected entry says
`content omitted; see the record`, while the store keeps every byte. The report
model is not published as JSON: the machine-readable view is
`givn history --json`, which reads the record itself, and `givn timeline` writes
its one HTML file and prints that file's path.

**Scale.** A whole ideation can run for hours to days and produce hundreds of
tasks and thousands of tool calls; the expected order of magnitude is at most
10,000 entries and roughly 2 MB per long ideation. That stays a dependency-free
append-only JSONL file: one append per entry, one sequential scan per report,
and no embedded database — DuckDB would outweigh the feature inside a
size-optimized binary, and SQLite would add a schema and migration surface for a
workload of one append and one scan. No bottleneck has been measured, and the
plain file is the interface external tools read directly. To stay readable at
that size, an expanded phase's lanes never draw one shape per entry: the
sub-pixel aggregation threshold is applied at every zoom (Fit, 1 h, 15 m), so
dense short activities collapse into per-bucket aggregate ticks naming their
count and combined duration even at 15 m; individual ticks appear only where
the current zoom permits, the timeline's scroll height is capped, and the
summary and ranking always cover the full report model — the ranking over
every entry kind that carries a measured duration, never only subprocess steps
(D-23 tracks pruning and retention). The model is the record minus the
recorder's own command entries (the intake's flush and the process-per-hook
mapper's invocation), which are dropped before any derivation so no surface or
total counts them; the store keeps them and the read views still show them.

**Browser tier.** The report is a Web UI, so its acceptance is browser-driven:
the repository's e2e tier installs Node.js (current LTS) and Playwright's
Chromium through `./setup-e2e-browser.sh`, and a JSON-line driver holds one
browser context per scenario so page state, zoom level, and open panels persist
across steps. The artifacts under test are the real generated report and the
real `givn` binary; the driver never substitutes an API call for a click.

## 8.26 The Outline Report and the Source Analysis Boundary

The outline addon is enabled by default. Its report is an advisory read view
that appends no development-record entry. It has no review or archive gate, no
verdict vocabulary, no waiver, and no forced reviewer context. The report's
contract is:

- **Enabled by default, explicitly disableable.** `addons.outline` is enabled
  unless the project writes `false` or runs `givn addons disable outline`. The
  command is still registered so the disabled path can name the addon and the
  enabling command; that path exits 1.
- **Outline first, hints second.** Every unit is printed as a per-component call
  tree first: declarations, signatures, order, and calls. Module outlines list
  declared member types as type items. Same-unit calls nest under the caller;
  calls to another component are leaves tagged with resolution, visibility, and
  owner. A `name_only` edge is a leaf and is never expanded. The tree is
  forward-only and omits calls outside the analyzed sources. It contains no
  data, method bodies, or comments. Metrics and classification follow as
  hints. A report exits 0 regardless of how many hints it contains. There is
  no `action`, `recommendation`, or advice line. The JSON schema is version 3
  and carries the flat per-unit methods and their call-resolution labels and
  `target` under `units[].outline`; the nesting is a human rendering, not a
  JSON field.
- **Call resolution is neutral and shape-aware.** Both adapters supply whether
  a call is a free function or a method, plus a path qualifier or receiver
  evidence when the syntax proves one. A candidate is accepted only when its
  owner agrees with that evidence; an unknown method receiver is `name_only`,
  several compatible methods are `ambiguous`, and a contradictory qualifier is
  `external`. Rust adds calls found in expression-list macro arguments. The
  renderer and the cohesion metrics read the same resolved targets; only
  resolved same-unit edges join methods for LCOM4.
- **Production-only accounting.** File `code_lines` excludes recorded inline
  `#[cfg(test)]` spans. Unit `physical_lines` and `code_lines` cover the unit's
  own spans and subtract the same exclusions. Conditional code is counted as
  written and a parent `mod tests;` `#[cfg(test)]` is not propagated.
- **Hints never gate.** A report hint never enters a gate or a review verdict.
  It is evidence for a source-confirmed architecture judgment.
- **No stored state.** The report writes only stdout/stderr or the explicit
  `--out` path. There is no baseline, watchlist, cache, classification history,
  or feedback ledger.
- **Production scope.** The report scans the project-root `src/` tree by default
  or the explicit `--path` roots, and excludes `tests/`, `benches/`,
  `examples/`, `target/`, Python test modules, cache/virtualenv directories, and
  `#[cfg(test)]` items. `src/bin/**` is included. A multi-crate workspace whose
  members live outside the requested roots is not scanned unless named.
- **Unavailable measurements are not zero.** A metric the analyzer cannot
  compute is `unavailable` in text and `null` in JSON. LCC is not implemented
  and is omitted; TCC is never relabelled as LCC.
- **Deterministic and offline.** Traversal is sorted, paths are project-relative
  with `/`, output carries no timestamps or analyzer identity, line counts and
  ratio metrics are integers, and no model or network call exists.
- **Neutral analysis boundary.** `SourceAnalyzer` is the port; `syn` is the
  first adapter and the Python standard-library AST extractor is the second;
  parser types stay inside each adapter directory. Production discovery,
  metrics, classification, and both renderers run above the port. A later
  tree-sitter adapter is accepted only by passing the same contract suite
  unchanged. ADR-074 records this cross-change boundary.
- **Diagnostics are data.** A file that cannot be read or parsed, an
  item-position macro, or another unsupported construct is reported by path and
  kind; the report continues. A wholly unsupported unit is never classified and
  no data is invented for it.
- **Format versions are explicit.** `schema_version` changes only for a breaking
  document-shape change; `rules_version` changes when thresholds or
  classification rules change. The human and JSON forms render from the same
  classified model, so a rule change does not silently alter the schema.

### 8.26a Advisory Outline Use in REFACTOR and Review

The report supports one substantive architecture pass rather than a second
review authority:

- **Readable from the outline.** The outline is a readability contract, not a
  metric review. A reader states each changed unit's responsibility in one
  sentence and its ordered call flow from declarations, signatures, and call
  order alone, without bodies or comments. The unit and type names, method
  names, parameter names and types, return types, and called type names state
  the domain operation in the project's vocabulary. A generic or machinery name
  where a domain noun exists (for example `handle`, `process`, `data`, `item`,
  `manager`, `util`, `helper`) is a readability finding.
- **Once per behavior batch.** During REFACTOR, run one report for the changed
  supported components and relevant neighbors. Ask whether each unit has one
  cohesive responsibility, a narrow consumer contract, a sensible dependency
  direction, and an understandable decomposition. Confirm suspected concerns
  against source. Reuse the report until later structural changes make it stale.
- **Review reuse.** Final review reuses relevant reports from implementation
  when they are current; rerun only for structurally stale evidence. The
  reviewer reconstructs each changed unit from its outline and treats a unit
  that cannot carry the story as a substantive readability finding. The report
  itself supplies no verdict.
- **Unsupported languages.** A component the analyzer does not support is
  assessed directly from source with the same questions. It is not a report
  failure and needs no waiver.
- **Advisory, never gating.** No `OUTLINE:` marker, `PASS`/`FAIL` explanation,
  metric decision, author/reviewer identity, or independent outline context is
  required. The report cannot block review or archive. A substantive
  readability finding about the changed code is an ordinary review finding and
  can reject PASS; the report is only its evidence.
- **Serving path.** Both the steps REFACTOR block and the review advisory
  section render behind `{{#if addons.outline}}`. The review template contains
  no outline verdict phrase. The addon key is known to the renderer, the sync
  flag map (`src/skills/targets.rs`), and the instruction CLI map
  (`src/cli/instructions.rs`). A project that disables the addon receives no
  outline obligation.

Rust and Python keep their `syn` and standard-library `ast` adapters. Java,
TypeScript/TSX, and JavaScript use bundled Tree-sitter grammars through the
same neutral `SourceAnalyzer` port. The TSX grammar is selected by extension;
JavaScript uses its own grammar for `.js`, `.mjs`, and `.cjs`; Java
package-private visibility is reported as crate scope, and Java records,
anonymous classes, JavaScript class expressions, TypeScript declaration files,
and namespaces carry explicit limitations. Tree-sitter supplies syntax facts
only: calls remain name-only unless the shared resolver can prove a target, and
calls never resolve across languages.

A test-only Tree-sitter evaluation compared the Rust and Python facts with
parallel grammars. Its 0.9.14 decision retained those adapters because the
comparison found missing required units, methods, calls, and spans. That record
is not a parity claim for Java, TypeScript, or JavaScript. See
[the evaluation record](../outline-tree-sitter-evaluation.md).

The historical blocking gate and its verdict vocabulary are retired. They are
not current architecture; see [the advisory outline guide](../outline.md).

### 8.26b The Review Context Record

Givn does not prescribe an execution topology. The default is a self-review in
the current context, recorded as `Review kind: self-review`; a self-review is
not an independent review and is never labeled as one. A major change, or a
user request recorded at proposal time, may use exactly one fresh review
context — one review subagent, one separate reviewer session, or one human
reviewer. That one context covers both the design-review and final-review
obligations. No planning, design, hardening, coverage, implementation, or
additional review context is started.

The proposal records the requested kind. `design-review.md` and `review.md`
record the performed kind, the author, and the reviewer context. A proposal
recording `independent` accepts only an independent review record whose
reviewer context differs from the author. A self-review is accepted when no
independence is recorded. A gate that requires independence accepts only
`Review kind: independent`; a self-review is never upgraded by inference.
Design-review and final review must name the same reviewer context for an
independent review. No outline identity record is required.

A legacy record with no `Review kind` line keeps its existing gate result, so
the policy does not retroactively fail changes that predate it. The one
exception is a proposal that records `independent`: that record is
authoritative, and the gate fails clearly when the review record is a
self-review. If independence is required but no fresh context is available,
the reviewer asks the user; there is no silent fallback.

## 8.27 Standalone Domain Documentation

Vocabulary and structure have separate durable homes:

- `docs/glossary.md` is the sole vocabulary authority in every project.
- `docs/domain-model.md` is the thin structural map: bounded contexts, collapsed
  aggregates, external systems, boundaries, and relationships. It links to the
  governing use-case contracts and specifications rather than copying rules.
- `docs/arc42/12-glossary.md` is a heading and a link to `../glossary.md`.

The domain-modeling skill and its instruction are available whenever
`domain_modeling` is enabled, independent of arc42. The instruction owns three
checks: ownership (vocabulary, structure, or behaviour), added value (a real
boundary or relationship rather than repeated scenarios), and change impact
(edit the model only when structure changed). A no-op edit is not maintenance
evidence.

Scaffolding follows the guidance that points at each file. `givn init`
creates the glossary when `arc42` or `domain_modeling` is effective and the
model when `domain_modeling` is effective. `givn addons enable arc42` creates
a missing glossary; `givn addons enable domain_modeling` creates both missing
files. `--no-addons` creates neither. Existing files are never overwritten and
are never deleted by disabling an addon.

The embedded templates live at `assets/templates/glossary.md` and
`assets/templates/domain-model.md`. The model template requests the top-down
ELK layout and keeps its Mermaid legend in a separate section. A host must
include the ELK renderer and honour the configuration; unsupported hosts may
fall back or fail. The guidance requires a rendered-diagram review and does not
promise identical rendering across hosts.

Migration is agent-directed: an agent that finds term definitions in an older
chapter 12 moves them into `docs/glossary.md`, preserves their wording and
ordering, and replaces chapter 12 with the link. Givn does not parse or rewrite
consumer Markdown automatically. While a migration is pending, chapter 12 is
the older location and the new glossary must not be presented as a second
authority; the agent completes the move before relying on the new file.

## 8.28 Proportionate Entry and Retired Skill Reconciliation

`givn-start` is the default entry for new or unclassified work. It investigates
the problem and recommends one of four outcomes: answer and stop, direct work
without a Givn change, durable ideation, or a formal change. It creates no
workflow artifacts. Direct work skips change administration, not correctness:
the relevant contract is read, specifications and documentation stay accurate,
proportionate regression coverage and checks are used, and the modification and
its limits are reported. Authorization to edit does not authorize commit,
push, archive, or unrelated work.

`givn instructions` reports state separately from request routing. With no
active change, the text recommends `givn-start` and names `givn new` as the
direct alternative; JSON carries `next_skill: "givn-start"` and keeps
`next_command: "givn new <change-id>"` as the direct alternative. With an active
change, the reported lifecycle step applies only when continuing that change.

`givn skills sync` generates `givn-start` and removes a recognizable generated
`givn-explore` skill. Recognition requires the generated frontmatter name and
the served-instruction pointer; a same-named custom file is preserved and
reported for manual transition. Legacy generated explore command files remain
recognized for removal. The upgrade snapshot and versioned-rules staging include
retired managed paths so a tracked generated explore skill is removed from the
revision rather than left as an uncommitted deletion.

*[Architecture documentation index](README.md)*
