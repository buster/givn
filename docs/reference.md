# Reference

Authoritative operational reference for the current checkout: CLI commands,
agent skills and targets, addons, configuration authority, the runner and
receipt contracts, generated project layout, and repository development
scripts. Workflow orientation is in [workflow.md](workflow.md); step
obligations are owned by the served instructions (`givn instructions <id>`).

## CLI commands

| Command | Purpose |
| --- | --- |
| `givn init [--force] [--no-addons] [--agents <list>]` | Scaffold `givn/{specs,changes,archive}`, write `givn/config.yaml` and `givn/commands.yaml`, install selected addons, and generate skills. Every addon whose declared default is on is enabled; `ise` is opt-in and omitted by a fresh init; `--no-addons` writes explicit `false` for all ten. `--agents` accepts `opencode`, `agents`, `claude`, `omp`, `maki`, or `all`. |
| `givn new <id> [--skip a,b]` | Create a change directory and record the effective add-on configuration in `config-lock.yaml`. `--skip` writes `.givn-skip` and marks artifacts as intentionally not produced. |
| `givn instructions [<id>] [--change <id>] [--json]` | No id: project state and request routing; no-active text recommends `givn-start` and names `givn new` as the direct alternative, and JSON carries `next_command`, `next_skill`, and `overview`. With an artifact id: its rendered instruction. With a policy or step id (`start`, `dialog`, `dialog-examples`, `challenge`, `dev-principles`, `domain-modeling`, `architecture-principles`, `ise`, `review-reverse`, `specs-reverse`, `tasks-reverse`): the same cascade without a lifecycle stage. Artifact JSON carries `id`, `generates`, `requires`, `instruction`, and, while domain modeling applies, `skill_to_load`. `explore` is retired and refused. |
| `givn lint [--change <id>] [paths...] [--json]` | Static feature and use-case analysis, including the projected interaction inventory for `--change`; runs no project tests. It reports missing or unlisted E2E interactions, undeclared capabilities, and unplaceable delta capabilities. Exit codes: `0` clean, `1` parse error, invalid use-case document, interaction mismatch, undeclared capability, or unplaceable delta capability, `2` `@wip`/`PENDING`, `3` duplicate title or empty feature. Precedence is `1 > 3 > 2 > 0`. |
| `givn check` | Prints guidance and executes nothing. |
| `givn check <artifact> [--change <id>]` | Runs the artifact's mandatory checks and then its configured `post` hooks. `tasks` runs WIP protection and change verification; `review` runs integrity, the run declaration, change-scoped lint (including the projected interaction inventory), overlap, retirement, routing, receipt, and scenario-commit checks. `givn check migration --change <id>` runs the preservation comparison; `givn check rules` reports rule cleanliness. |
| `givn status [--change <id>] [--json]` | Filesystem-derived state: `SCM: git` / `fossil` / `none`, stage, artifact checklist, task progress, pending task lines, next required/optional steps, and unproven archive candidates. Computes no tests. |
| `givn graph [--change <id>] [--json]` | Show the effective artifact graph, dependencies, optional/gates-archive flags, and hooks, optionally overlaid with live state. |
| `givn readme check` | Report open `givn:todo` placeholders, legacy scaffold remnants, damaged or ambiguous development/run managed markers, unmanaged README, or a missing README. Exits non-zero while findings remain; never rewrites the README. |
| `givn spec coverage [--baseline <json>] [--format json]` | Behavior/use-case/capability/E2E/interaction inventory; optional comparison against a prior snapshot. |
| `givn spec duplicates [--format text\|json]` | Advisory duplicate audit over permanent specs only. |
| `givn spec tree [--format text\|json]` | Group/Actor/Goal and per-capability scenario/E2E inventory. |
| `givn spec find <query>` | Lexical (not semantic) search over permanent scenario titles and steps. |
| `givn spec route [--change <id> \| --file <proposal>]` | Advisory capability-routing candidates with confidence; the proposal's recorded routing decision remains authoritative. |
| `givn history [--change <id> \| --topic <id>] [--json]` | Read the local development record. Reading never appends. |
| `givn history ingest [--with-full-content]` | The one agent intake: one reported JSON payload on stdin, per-event validation. Exit `0` when the payload was read; `2` when stdin is not readable JSON. `--with-full-content` raises this payload's mode unless `record.mode` is `off`. |
| `givn history harness-hook --harness claude` | Map one process-per-hook Claude payload into record entries. |
| `givn history settings [--json]` | Report `record.mode` and its source (`config` or `default`) without recording. |
| `givn timeline [--change <id> \| --topic <id>] [--out <path>]` | Render one change, topic, or the project into one self-contained HTML file; default `givn/history/reports/<scope>.html`. Opens nothing and records nothing. |
| `givn archive --change <id> [--verbose]` | Publish and verify a completed change forward-only on Git or Fossil. `--verbose` streams gate output with timestamps. Add-on drift from the change lock blocks archive. |
| `givn eject <target> [--force]` | Materialize an embedded facet for editing, for example `design:template`, `review:instruction`, `skill:givn-propose`, `manifest`, `readme:template`. |
| `givn addons list` / `enable <name>` / `disable <name>` | Show effective state and toggle one addon. `enable` checks the registered prerequisite relation first, scaffolds addon-owned files, and resyncs; `disable` refuses while a dependent addon is on, removes the workflow effect, and leaves existing project files. While a locked change is active, mutation is refused; the operator path is `--operator --reason "<reason>"` on an interactive terminal, recorded in the change lock. |
| `givn skills sync` | Regenerate skills, reporters, managed `AGENTS.md` blocks, and ignore state for the persisted target selection. Recognizable old generated command files are removed; custom files are preserved with a manual notice. |
| `givn skills targets add <agent[,agent...]>` | Add targets to `skills.targets`, touching only that line, then sync. |
| `givn upgrade` | Refresh the managed config scaffold and generated guidance in a clean checkout, preserving user-owned values. Refuses retired keys and executable settings misplaced in `config.yaml`. |
| `givn outline report [--json] [--out <path>] [--path <dir>...]` | Advisory structure report for Rust, Python, Java, TypeScript, and JavaScript. Defaults to `src/`; explicit `--path` roots are required when `src/` is absent. Schema version 5; metrics never change the exit code. |
| `givn run [--port <n>] [--print]` | Start the declared product. `--print` emits one sh-eval-able start line. Exit codes: `0` clean/library, `1` preflight finding, `2` readiness timeout, `130` SIGINT, `143` SIGTERM, otherwise the child's exit code. |
| `givn commit -m <message> <path...>` | Create one attributed revision through the selected backend. Explicit paths are required; unsafe, absolute, or escaping selections are refused. |
| `givn completions <bash\|zsh\|fish>` | Print a shell completion script. |
| `givn test-result --total <n> --passed <n> --failed <n> --skipped <n>` | Write the four-count runner result to `GIVN_RESULT_FILE`; a helper for simple runners. |

Hidden sentinels: `givn missing-testrunner` (default `verify.command`) and
`givn missing-run-command` (default `run.command`) are executable hidden
commands that print setup guidance and exit non-zero. `givn missing-e2e-runner`
is the default `verify.e2e_command` comparison value: the E2E hook treats it as
unconfigured and never executes it as a command. They are commands or
comparison values, not generated scripts.

## Agent skills and targets

Skills are the only generated invocation surface. `givn skills sync` writes
`<target>/skills/<name>/SKILL.md` for each selected target:

| Target | Directory | Invocation |
|---|---|---|
| `opencode` | `.opencode/` | native skill slash (for example `/givn-propose`) |
| `claude` | `.claude/` | native skill slash |
| `omp` | `.omp/` | `/skill:<name>` |
| `maki` | `.maki/` | explicit request or agent-selected skill; no generated command |
| `agents` | `.agents/` | neutral skill discovery by name |

The selection is stored in `skills.targets`. An absent block or an empty/unknown
list falls back to `opencode` for `givn skills sync`; `givn init` itself requires
a selection. Givn generates no command files.

Choose the entry point:

- Start with `givn-start` for new or unclassified work. It investigates and
  recommends answer only, direct work without a Givn change, `givn-ideation`,
  or `givn-propose`. It produces no artifacts.
- Run `givn new <change-id>` directly when the change's goal and scope are
  already clear.
- Start with `givn-propose` when a formal change is already authorized.
- Start with `givn-ideation` when the domain needs event storming, use cases,
  personas, stable terms, or several future changes. Handoff produces change
  seeds, each an input to `givn-propose`, not a change.

| Skill | When to use it |
| --- | --- |
| `givn-start` | Investigate a problem, question, or improvement and recommend answer only, direct work, ideation, or a formal change; creates no artifacts. |
| `givn-ideation` | Turn a vague idea into change-ready use cases through event storming, personas, and change seeds; does not start a change. |
| `givn-propose` | Create the change and write the proposal: WHAT and WHY, release note, capability routing, open questions; the served propose step continues through specs and design. |
| `givn-spec` | Write Gherkin delta features with observable behavior and no implementation detail. |
| `givn-design` | Write the technical design: technology, step locations, decisions, interaction matrix, visual contract, local runnability. |
| `givn-design-review` | Stress-test the plan question by question, harden the artifacts, and sign off. |
| `givn-tasks` | Break implementation into behavior batches with actual RED/GREEN/REFACTOR evidence and commit fields. |
| `givn-implement` | Execute the batch loop; the served `tasks`/`steps` instructions own the mechanics. |
| `givn-steps` | Shared step-writing mechanics: bindings, reuse, empty-step ban, RED/GREEN/REFACTOR. |
| `givn-review` | Complete the coverage review, implementation evidence check, gap classification, targeted closure, and sign-off. |
| `givn-archive` | Archive a completed change through the gate hooks, candidate publication, and verification. |
| `givn-reverse-engineer` | Discover an existing capability's black-box boundary and write descriptive baseline specs. |
| `givn-characterize` | Execute the characterization loop against existing behavior, with seams and a reverse review. |
| `givn-challenge` | Evidence-first examination of a proposal, test, spec, design, code, or whole change; no lifecycle gate. Generated while `challenge` is enabled. |
| `givn-domain-modeling` | Establish one domain vocabulary and structural model; maintain `docs/glossary.md` and `docs/domain-model.md`. Generated while `domain_modeling` is enabled, independent of arc42. |
| `givn-dev-principles` | Apply the team's engineering principles while code is read or written. Generated while `dev_principles` is enabled. |

## Addons

Each addon declares its own default. Every addon except an Opt-in addon is
enabled by default, so a missing key in a partial `addons:` block resolves that
addon's declared default; only an explicit `false` disables a default-on addon.
`ise` is the one Opt-in addon: its declared default is `false`, and a fresh
`givn init` omits its key. The retired ids
`absolute_mode`, `grill_me`, and `grill_with_docs` are refused with a manual
replacement (`communication_policy` or `challenge`); no alias is accepted.

| Addon | Effect |
| --- | --- |
| `arc42` | Adds the `arc42-docs` artifact after `design`, gates archive on it, scaffolds missing chapters at the fixed `docs/arc42/` path, and creates a missing `docs/glossary.md`. |
| `communication_policy` | Manages the `communication-policy` fenced block in `AGENTS.md`. The dialog obligation is independent and always applies. |
| `readme` | Scaffolds an initial README when none exists and enables the deterministic README completeness check and archive gate. |
| `coverage` | Archive-level gate: runs project measurement and merge commands, validates Cobertura output, and updates managed README blocks. |
| `domain_modeling` | Generates the `givn-domain-modeling` skill and creates missing `docs/glossary.md` and `docs/domain-model.md`; does not require arc42. |
| `dev_principles` | Generates the `givn-dev-principles` skill. |
| `architecture_principles` | Serves architecture-principles guidance for proposal, design, design-review, dialog, review, and tasks. |
| `challenge` | Generates the `givn-challenge` skill and serves the canonical challenge instruction. |
| `outline` | Enables `givn outline report` and its advisory evidence in steps and review; that guidance renders only when the addon is enabled. No gate, verdict, waiver, or stored baseline. |
| `ise` | Opt-in; requires `architecture_principles`. Serves `givn instructions ise`, adds the two change records to design and review guidance, and scaffolds `givn/specification.yaml` when it becomes effective. `givn addons enable ise` refuses while `architecture_principles` is off; `givn addons disable architecture_principles` refuses while `ise` is on. Neither refusal writes a flag or a file. |

### `givn/specification.yaml`

The declaration of the Specification holds exactly six top-level keys: the five
roles `behavior`, `structure`, `contracts`, `limits`, and `e2e` as lists of
project-relative path globs, plus one `source:` list of replaceable roots. An
empty list is valid and retains nothing. The loader refuses an unknown key, an
unknown or repeated role, an omitted role, an absent `source:` list, a non-list
value, an unreadable file, and a path that leaves the project; the refusal names
the element and the file and never rewrites it. A project with `ise` off never
reads the file.

`givn lint` in the default project scope reports four advisory findings and
never changes the exit code: a declared path missing from the tree, one path
declared in two roles, a `.feature` file outside the `behavior` globs, and an
`e2e` role that resolves no package or a package that carries a dependency edge
into `source`. The `e2e` role's absent package is reported by the boundary
finding only. The dependency edge reads Cargo manifests only: a declared path
that resolves no package manifest receives the boundary finding naming the role
entry, and a malformed manifest skips the edge comparison for that package.
A malformed or absent declaration is a blocking declaration error, not one of
the four findings.

With the change's `config-lock.yaml` having `ise` effective, `givn check tasks`
turns the same declared `e2e` role into a blocking acceptance boundary. The role
resolves to its packages: each matched path's nearest `Cargo.toml` at or above it,
or the path itself when it resolves none. Every distinct package directory is
one resolved package and is checked on its own manifest, including direct, transitive, and
workspace-inherited edges. The manifest read covers the top-level, dev, build, and
every target-specific dependency table; an edge is refused when its package
directory is a `source:` root, lies inside one, or contains a path that root
names, so a glob such as `src/**/*.rs` reaches the package at `src`. The check
refuses, names the condition, and writes no receipt when the role matches no
path, when a resolved package's manifest cannot be read, when a resolved `e2e`
path sits under a `source:` root, when a resolved
package's tree contains a `source:` root, when a package has a dependency edge into
`source`, when a path that resolves no package contains a reference resolving into
a `source:` root, or when `verify.e2e_command` does not name every resolved
`e2e` root. The runner-target rule is textual: the command must contain each
resolved root's directory as a whole token or a token containing
`<directory>/`. It proves naming, not execution, and a run that reported another
scope is refused by the result's scope field. The import fallback reads
path-resolving references only: an external package name, or a language whose
import syntax the reader does not recognize, is admitted. A change with `ise`
off reads no declaration and gains no boundary refusal.

## Configuration authority

`givn/config.yaml` owns policy and structure:
`givn_config_version`, `artifacts:`/`apply:` graph patches, artifact `post:`
hooks, `addons:`, `skills: targets:`, `rules:`, `record.mode`, `extensions:`,
and the managed documentation scaffold. Commented lines are refreshed by
`givn upgrade`; uncommented key/value lines are user-owned and preserved.

`givn/commands.yaml` owns executable settings only: `verify`, `coverage`, and
`run`. It is parsed with a dedicated schema, not the generic manifest overlay.
Policy keys in this file, and `verify`/`coverage`/`run` in `config.yaml`, are
refused by name before any write. A malformed or unsupported YAML presentation
for an edited `skills:` or addon block is refused with manual-edit instructions
rather than rewritten unsafely.

### `givn/commands.yaml`

- `verify.command` — the Gherkin runner. The default is
  `givn missing-testrunner`.
- `verify.e2e_command` — the E2E runner. The default comparison value is
  `givn missing-e2e-runner`; an `@e2e` scenario in scope without a configured
  runner blocks review/archive, while a scope with no `@e2e` scenario is
  `not_applicable`. With `ise` effective for a change, the command must also
  name the declared `e2e` package and is run at `givn check tasks` against the
  change's delta features.
- `verify.features_path` — where the runner reads features (default
  `givn/specs`).
- `coverage.measure_command`, `coverage.merge_command`, `coverage.non_e2e_path`,
  `coverage.e2e_path`, `coverage.output_path` — archive coverage gate.
- `run.command`, `run.product_type` (`server`/`cli`/`library`), exactly one of
  `run.port` or `run.port_env` for servers, `run.ready_path`, and
  `run.ready_timeout_s` (default 60). The block replaces whole, like `verify`.
  Validation rules R1–R10 block review and archive. `givn run` executes it.

### `givn/config.yaml` keys

- `artifacts:` — patch the embedded graph. Existing ids merge field by field;
  new ids append; `remove: true` removes; `apply.requires` selects archive
  gates; `optional: true` makes an artifact skippable.
- `post:` — additive hooks per artifact. Built-ins are `verify`,
  `verify-e2e`, `lint`, and `integrity`; arbitrary shell commands are allowed.
  Built-in names cannot be made `allow_failure`.
- `extensions:` — project rules appended to a served instruction under a
  marked `## Project extension` section. Unknown ids and malformed entries warn
  and are skipped; the built-in rules are served alone.
- `addons:` — see the addon table.
- `skills.targets:` — persisted generated-skill destinations.
- `rules:` — `local-only` (default) or `versioned`.
- `record.mode:` — `on` (default), `off`, or `full`; see the development
  record.
- `givn_config_version` — the managed scaffold version, not proof that project
  migration work is complete.

## Verification and recovery

### Runner contract

The runner receives:

| Variable | Meaning |
|---|---|
| `GIVN_FEATURES` | Feature path or scope the runner should execute. |
| `GIVN_RESULT_FILE` | Path where the runner must write a fresh JSON result. |
| `GIVN_RESULT_SCOPE` | `change`, `regular`, or `e2e`. |

The result is `{"total": n, "passed": n, "failed": n, "skipped": n}`; `scope` is
optional, but when present it must match the expected scope. A missing,
unreadable, malformed, non-reconciling (`passed + failed + skipped != total`),
zero-scenario, failed, or skipped result blocks. The previous result file is
removed before launch.

### Bootstrap the project runner

During the first useful product change, adapt or create the project's
Gherkin/Cucumber runner rather than copying a generic command line. It must:

1. Execute exactly the selected feature path from `GIVN_FEATURES`.
2. Handle `change`, `regular`, and `e2e` in `GIVN_RESULT_SCOPE`. `change`
   executes the active delta scenarios at `GIVN_FEATURES`, including both
   `@e2e` and non-E2E scenarios. `regular` and `e2e` use the selected feature
   path: regular excludes `@e2e`; E2E selects only `@e2e`. Archive passes the
   permanent corpus; an `ise`-effective change check passes the delta.
3. Remove and write `GIVN_RESULT_FILE` on every invocation with actual scenario
   totals and outcomes.
4. Exit non-zero for runner failures, including failed, undefined, pending,
   ambiguous, or skipped scenarios.
5. Be validated before behavior-batch RED/GREEN work begins. A missing binding,
   dependency, fixture, or result file is bootstrap failure, not assertion RED.

By default the change runner is `verify.command`, even for a delta that
contains `@e2e` scenarios. When the change's `config-lock.yaml` has `ise`
effective and the delta contains `@e2e` scenarios, `givn check tasks` also runs
`verify.e2e_command` against the same delta features with scope `e2e`; a change
with `ise` off keeps the single change runner. `verify.e2e_command` is not a
replacement for complete delta verification.

Use the project runner's own framework and scripts. Givn's own
[`run-tests.sh`](../run-tests.sh) is a tested reference adapter for this
contract, not a generic script to copy into another project. `givn test-result`
can write the required four-count result for a simple adapter, but does not
execute features or prove the counts. Coverage is separate: when enabled, its commands
must produce distinct regular and E2E Cobertura reports. Meaningful E2E tests
exercise the product's public boundary: a CLI process, service protocol, or
library public API, as appropriate. The archive E2E scope remains mechanically
not applicable when the merged corpus has no `@e2e` scenario; bootstrap and
review guidance, not a new archive gate, establish the method's E2E expectation.

### Change verification

`givn check tasks --change <id>` runs `verify.command` once against the change's
delta specs and writes `givn/changes/<id>/verification/tasks-verification.json`.
Before launch it captures a `VerificationInputs` witness: current revision,
selected-feature digest, command digest, relevant content digest, and declared
scope. After the run it recaptures and requires equality; then it writes the
receipt. Any launch, result, or stability failure leaves no green receipt.

When the change lock has `ise` effective, the check first resolves the declared
`e2e` role to its packages, then — when the delta contains `@e2e` scenarios — runs
`verify.e2e_command` with `GIVN_RESULT_SCOPE=e2e` and
`verification/e2e-result.json`, reads the same fail-closed counts, and requires
the E2E identity to be unchanged. The receipt then adds `e2e_identity`
(`package_digests` keyed by each resolved package directory or import-mode path,
`runner_command_digest`, `givn_version`) and `e2e_scope` (`passed` with counts,
or `not_applicable` when the delta has no `@e2e` scenario). A change with `ise`
off writes neither key: both are optional and skipped when absent, so its
receipt bytes match the pre-existing shape.

A change declaring `Behavior change: none` with the specs artifact skipped and
no delta feature files reports changed-scenario verification as not applicable.
It still checks identity and scenario commits and stores applicability without a
fabricated runner result. Contradictory declarations and a missing skip refuse.

`givn check review` runs no tests. It validates the receipt:
- the counts are green;
- the feature and command digests match;
- the tested-content digest matches (schema 2 with `change-proof/v1`);
- the tested revision is available (Git ancestry from `HEAD`; Fossil revision
  existence);
- when the change lock has `ise` effective, the E2E identity matches: the
  resolved package digests, the E2E runner command digest, the running
  Givn version, and an E2E scope that is green when the delta has an `@e2e`
  scenario and `not_applicable` when it has none.

A documentation/evidence-only revision with equal tested content reuses the
proof. Schema 1, a receipt without a known content scope, or an unknown schema
requires strict revision equality or is refused; no reuse is assumed. An
`ise`-effective change refuses a receipt without the E2E identity by name. A
failure reports every independent stale reason and routes to `givn check
tasks`.

### Archive verification and recovery

Archive publishes the merged candidate, then runs the regular scope and the E2E
scope (when the merged corpus has `@e2e`) once against that candidate. Each
successful scope is persisted immediately with its inputs, command digest,
result-file digest/status, and tested revision. On resume:

- A scope whose relevant inputs and result are unchanged is reused.
- Any source, spec, helper, coverage, or runtime-settings change invalidates
  both scopes.
- A change confined to `verify.e2e_command` invalidates E2E only; a change
  confined to `verify.command` invalidates regular only.
- Formatting-only settings changes preserve canonical equality.

A failed or incomplete candidate remains in `givn/archive/<id>/` with a pending
receipt. Repair the committed state and rerun archive; the merge is idempotent
and finalized scopes are not re-run. There is no rollback or history rewrite.
Finalization writes the proven receipt only when `HEAD` is the tested revision
and every dirty path is a declared evidence path, then publishes the
evidence-only finalization revision. An already proven archive is refused.

### Upgrade recovery

`givn upgrade` requires a clean checkout of the selected backend and captures a
checkpoint revision before writing. It stages only the calculated managed
allowlist. On failure it reports the checkpoint, completed writes, revisions
actually published, best-effort restoration, remaining partial state, and
pending work. Givn performs no reset, clean, or history rewrite and promises no
complete rollback; restore from the checkpoint manually or fix forward.

At the 1.0 migration baseline the shipped migration catalog is empty. A pre-1.0
project gets the managed refresh only when its configuration is current-format;
retired configuration is refused with a manual-transition message. Future
migration routes are resolved by the same catalog contract and are exercised by
synthetic fixtures. See [the manual 1.0 transition](migrations/README.md).

## Generated project layout

```text
givn/
├── config.yaml                 # policy, graph, addons, targets, extensions, record mode
├── commands.yaml               # verify, coverage, run
├── history/                    # ignored local runtime state: events.jsonl, pending/, reports/
├── specs/<usecase>/usecase.md  # permanent use-case contract
├── specs/<usecase>/<cap>.feature
├── specs/fragments/*.md
├── changes/<id>/               # active change
│   ├── proposal.md
│   ├── specs/<usecase>/<cap>.feature
│   ├── design.md  design-review.md  tasks.md  review.md
│   ├── arc42.md                # when arc42 is enabled
│   ├── evidence/  verification/  results/
│   └── .givn-skip
├── archive/<id>/               # archived change + verification.json + results/
├── artifacts/<id>/             # ejected templates/instructions
├── personas/<id>.md
└── ideation/<topic>/           # optional ideation state

docs/arc42/                     # fixed durable architecture path (arc42 addon)
AGENTS.md                       # user-owned except managed fenced blocks
```

The durable use-case model — identity, scoping rules, and review rejection
criteria — is documented in [use-case-model.md](use-case-model.md).

Selected targets receive:

| Target | Generated content |
|---|---|
| `.opencode/` | `skills/<name>/SKILL.md`, `plugins/givn-attribution.js`, `plugins/givn-timeline.js` |
| `.claude/` | `skills/<name>/SKILL.md`, `hooks/givn-attribution.sh`, `hooks/givn-timeline.sh`, merged `settings.local.json` |
| `.omp/` | `skills/<name>/SKILL.md`, `hooks/pre/givn-attribution.js`, `hooks/pre/givn-timeline.js` |
| `.maki/` | `skills/<name>/SKILL.md`, `lua/givn-timeline.lua`, merged `plugin.toml` grants and `init.lua` fence |
| `.agents/` | `skills/<name>/SKILL.md` only; no harness runtime |

`AGENTS.md` managed blocks are `givn-project` (always) and
`communication-policy` (when that addon is enabled). The retired
`absolute-mode` block is removed during sync while surrounding content is
preserved.

### Initialization, ownership, and regeneration

`givn init` writes `givn/config.yaml`, `givn/commands.yaml`, and the `givn/`
scaffolding; initializes enabled addon-owned paths; persists selected targets;
and runs skill synchronization. The commands file is project-owned executable
configuration. Un-commented configuration values are project-owned and retained
by upgrade; generated skills, integrations, and managed fenced blocks are
refreshed by `givn skills sync`.

Synchronization writes the target content in the table above, installs the
Git attribution hook where applicable, refreshes the managed `AGENTS.md`
blocks, and merges the documented Claude and Maki registration files. Generated
integration files are overwritten on synchronization; managed blocks preserve
surrounding user text. Custom legacy command files are preserved and reported,
not removed automatically.

The default `rules: local-only` writes ignore entries for generated Givn rules
and removes matching tracked generated rule paths from the backend index while
leaving their working files in place. Set `rules: versioned` when those files
must remain versioned. This policy covers generated skills, target integrations,
and `.githooks/commit-msg`; it does not make user-owned merged Maki files into
rules. Disabling an addon removes its workflow effect but leaves existing
project files for an explicit project decision.

Recording defaults to `record.mode: on` and writes `givn/history/`. It is
ignored by default, not private: existing tracked or historical recordings are
not removed. Set `record.mode: off` to stop new recording; it does not purge
entries or uninstall reporters. The [development record](development-report.md#settings)
documents captured data, sensitivity, and retention limits.

## Repository development scripts

```sh
cargo check
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
./run-tests.sh --non-e2e     # this repository's verify.command scope
./run-tests.sh --e2e         # this repository's verify.e2e_command scope
./measure-coverage.sh        # non-E2E and E2E Cobertura reports
./merge-coverages.sh         # merged report used by README badges/blocks
```

After archiving a change, refresh the local binary and generated integrations:

```sh
cargo install --path . --locked --force
givn skills sync
givn instructions
./run-tests.sh --non-e2e
./run-tests.sh --e2e
```

*[Documentation index](INDEX.md)*
