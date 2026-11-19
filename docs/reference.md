# Reference

Authoritative operational reference for the project: CLI commands,
agent skills and targets, addons, configuration authority, the runner and
receipt contracts, generated project layout, and repository development
scripts. Workflow orientation is in [workflow.md](workflow.md); step
obligations are owned by the served instructions (`givn instructions <id>`).

## CLI commands

| Command | Purpose |
| --- | --- |
| `givn init [--force] [--no-addons] [--agents <list>]` | Scaffold `givn/{specs,changes,archive}`, write `givn/config.yaml` and `givn/commands.yaml`, install selected addons, and generate skills. Every addon whose declared default is on is enabled; `ise` is opt-in and omitted by a fresh init; `--no-addons` writes explicit `false` for all ten. `--agents` accepts `opencode`, `agents`, `claude`, `omp`, `maki`, or `all`. |
| `givn new <id> [--skip a,b]` | Create a change directory and record the effective add-on configuration in `config-lock.yaml`. `--skip` writes `.givn-skip` and marks artifacts as intentionally not produced. |
| `givn instructions [<id>] [--change <id>] [--json]` | No id: project state and request routing; no-active text recommends `givn-start` and names `givn new` as the direct alternative, and JSON carries `next_command`, `next_skill`, and `overview`. With an artifact id: its rendered instruction. With a policy or step id (`start`, `dialog`, `dialog-examples`, `challenge`, `dev-principles`, `domain-modeling`, `architecture-principles`, `ise`, `review-reverse`, `specs-reverse`, `tasks-reverse`, or a stack instruction id `steps-rust`, `steps-python`, `steps-node`, `steps-java`, `steps-go`, `steps-ruby`): the same cascade without a lifecycle stage, so a stack instruction resolves with no change context and a project may override it per id. Artifact JSON carries `id`, `generates`, `requires`, `instruction`, and, while domain modeling applies, `skill_to_load`. `explore` is retired and refused. |
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
| `givn ise rebuild-all --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Write the spec copy in which the System is rebuilt: every declared `source:` entry is removed from it. Requires `ise` and `architecture_principles`; `--at` defaults to `HEAD`. |
| `givn ise rebuild-component <path>... --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Write the spec copy in which one Component is rebuilt: only the named paths are removed, and neighbouring Components and their Component Interfaces stay. Requires at least one path. |
| `givn ise compare --manifest <file> --copy <dir>` | Report frozen-input equality between the spec-copy manifest, the Specification files in the spec copy, the Specification files in the project working tree, and the End-to-end (E2E) tests. It executes no trial command and prints `Trial result: unverified`. Exit `0` clean, `1` refusal or difference. |
| `givn ise verify-trial --manifest <file> --copy <dir> --result <file> [--component-not-run <reason>]` | Execute the generated `givn/ise-verification.yaml` contract: run the component command inside the copy and the frozen End-to-end command in the checkout, require exact declared execution-unit counts with zero failed or skipped, run coverage when declared, and write an `ise-trial-result/v1` record. Exit `0` only for `clean: true`. |
| `givn eject <target> [--force]` | Materialize an embedded facet for editing, for example `design:template`, `review:instruction`, `skill:givn-propose`, `manifest`, `readme:template`. |
| `givn addons list` / `enable <name>` / `disable <name>` | Show effective state and toggle one addon. `enable` checks the registered prerequisite relation first, scaffolds addon-owned files, and resyncs; `disable` refuses while a dependent addon is on, removes the workflow effect, and leaves existing project files. While a locked change is active, mutation is refused; the operator path is `--operator --reason "<reason>"` on an interactive terminal, recorded in the change lock. |
| `givn skills sync` | Regenerate skills, reporters, managed `AGENTS.md` blocks, and ignore state for the persisted target selection. Recognizable old generated command files are removed; custom files are preserved with a manual notice. |
| `givn skills targets add <agent[,agent...]>` | Add targets to `skills.targets`, touching only that line, then sync. |
| `givn upgrade` | Refresh the managed config scaffold and generated guidance in a clean project working tree, preserving user-owned values. Refuses retired keys and executable settings misplaced in `config.yaml`. |
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

The Specification Manifest `givn/specification.yaml` holds exactly six
top-level keys: the five Specification categories `behavior`, `structure`,
`contracts`, `limits`, and `e2e` as lists of project-relative path globs, plus
one `source:` list of replaceable roots. An empty list is valid and matches
nothing. The loader refuses an unknown key, an unknown or repeated category, an
omitted category, an absent `source:` list, a non-list value, an unreadable
file, and a path that leaves the project; the refusal names the element and the
file and never rewrites it. A project with `ise` off never reads the file.

`givn lint` in the default project scope reports four advisory findings and
never changes the exit code: a declared entry missing from the tree, one entry
declared in two categories, a `.feature` file outside the `behavior` globs, and
an `e2e` category that resolves no package or a package that carries a
dependency edge into `source`. The `e2e` category's absent package is reported
by the boundary finding only. The dependency edge reads `Cargo.toml` files only:
a manifest entry that resolves no package receives the boundary finding naming
the category entry, and a malformed `Cargo.toml` skips the edge comparison for
that package. A malformed or absent Specification Manifest is a blocking
manifest error, not one of the four findings.

With the change's `config-lock.yaml` having `ise` effective, `givn check tasks`
turns the same declared `e2e` category into a blocking acceptance boundary. The
category resolves to its packages: each matched path's nearest `Cargo.toml` at or above it,
or the path itself when it resolves none. Every distinct package directory is
one resolved package and is checked on its own manifest, including direct, transitive, and
workspace-inherited edges. The manifest read covers the top-level, dev, build, and
every target-specific dependency table; an edge is refused when its package
directory is a `source:` root, lies inside one, or contains a path that root
names, so a glob such as `src/**/*.rs` reaches the package at `src`. The check
resolves the boundary before the no-behavior-change result, so an `ise`-effective
change always needs a readable Specification Manifest and a non-empty resolved
set; `givn check review` makes the same check for a change that writes no
receipt. The check refuses, names the condition, and writes no receipt when the
category matches no path, when a resolved package's `Cargo.toml` cannot be read,
when a resolved `e2e`
path sits under a `source:` root, when a resolved
package's tree contains a `source:` root, when a package has a dependency edge into
`source`, when a path that resolves no package contains a reference resolving into
a `source:` root, or when `verify.e2e_command` does not name every resolved
`e2e` root. The runner-target rule is textual: a resolved root is named by a
token equal to the root or containing `<root>/`. The `--package` value is such
a token, so `--package e2e` names the root; a bare `--e2e` tier flag names no
package, so a command whose only package evidence is that flag is refused. The
check proves naming, not execution, and a run that reported another scope is
refused by the result's scope field. Both readers are static: a registry, git,
or renamed dependency carries no project path and is admitted, so the boundary
check cannot see a published product edge; a project's E2E runner is expected to
assert its committed lockfile for that case. The import fallback reads path-resolving
references only: an external package name, or a language whose import syntax
the reader does not recognize, is admitted. A change with `ise` off reads no
Specification Manifest and gains no boundary refusal.

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
  `coverage.e2e_path`, `coverage.output_path`, and `coverage.e2e_subprocess`
  (default false; true only when the End-to-end suite runs the product as an
  uninstrumented subprocess) — archive coverage gate and trial coverage.
- `verify.product_path` (relative to the spec copy; a regular file or a
  directory of binaries) and `verify.product_env` (default `GIVN_BIN`, for
  example `PASSUFF_BIN_DIR`) — the rebuilt product the trial End-to-end run
  must drive; the result must carry its path and SHA-256. A directory digest
  covers every regular file beneath it, sorted by relative path, with the
  executable marker.
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
optional, but when present it must match the expected scope. An
`ise verify-trial` result also carries `scenarios` (one title per execution
unit, duplicates and outline rows preserved); its End-to-end result also
carries `product` and `product_sha256`. Givn's own E2E
package also reads `GIVN_BIN` (the built product binary, required) and
`GIVN_READY_FIXTURE` (the built ready fixture, required by run-execution
scenarios); `./run-tests.sh --e2e --package e2e` exports both. The harness reads
the package's parent directory as the project root for the read-only repository
files a scenario asserts; `GIVN_ROOT` overrides it when the package is not
inside the project it tests. A missing,
unreadable, malformed, non-reconciling (`passed + failed + skipped != total`),
zero-scenario, failed, or skipped result blocks. The previous result file is
removed before launch.

### Repository acceptance package

Givn's own `@e2e` tier is the standalone `e2e` package. `e2e/Cargo.toml`
declares its own `[workspace]`, `publish = false`, and no dependency on the
`givn` crate; the root package `exclude` list holds `e2e/**`. The package holds
the runner, every `@e2e` step definition, the fixtures, and the browser driver
assets under `e2e/browser/`. Feature files stay in `givn/specs/`.

`./run-tests.sh --e2e --package e2e` canonicalizes `GIVN_FEATURES`,
`GIVN_RESULT_FILE`, and the binary directory to absolute paths, sets
`GIVN_RESULT_SCOPE=e2e`, builds `givn` and `ready_fixture` unless `GIVN_BIN`
already names an executable, exports both paths, and runs
`cargo test --locked --manifest-path e2e/Cargo.toml --test e2e_runner` in one
process with `e2e/` as the working directory. The `e2e` token is the value of
`--package`, so the declared command satisfies the runner-target rule; `--e2e`
alone names no package and `givn check tasks` refuses it. The package runner
first reads `e2e/Cargo.lock` and exits 2 when a `[[package]]` named `givn`,
`givn-config`, or `givn-core` appears, then filters `@e2e`, excludes
`@givn.removed`, fails on a skipped step, and writes the four-count result with
the scope from `GIVN_RESULT_SCOPE`. A missing or non-executable `GIVN_BIN`
exits 2 before any scenario runs.

`measure-coverage.sh` exports the instrumented binary it builds with
`cargo llvm-cov run --no-report --locked --bin givn -- --version`, sets
`LLVM_PROFILE_FILE`, and calls the same runner; the runner does not rebuild.

`scripts/qualify-givn-e2e.sh` copies the project working tree to a scratch directory,
requires `cargo tree --manifest-path e2e/Cargo.toml` to name no product crate,
runs the suite green, applies one seeded product defect, requires the suite to
fail, reverts the defect, and requires the suite to pass again. The project
working tree under test is never modified.

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
`e2e` category to its packages, then — when the delta contains `@e2e` scenarios — runs
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

### Spec copy preparation

`givn ise rebuild-all` and `givn ise rebuild-component` write one selected
revision's Specification into a plain directory, the spec copy, so the agent can
rebuild the System from it. `givn ise compare` reports every difference between
that copy, the project working tree, and the spec-copy manifest the command
wrote. All three commands require the `ise` add-on and its
`architecture_principles` prerequisite; with either off they refuse by name
before the Specification Manifest or the spec-copy manifest is read.

| Command | Effect |
| --- | --- |
| `givn ise rebuild-all --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Write the spec copy in which the System is rebuilt: every declared `source:` entry is removed from it. |
| `givn ise rebuild-component <path>... --into <dir> [--at <revision>] [--manifest <file>] [--dry-run]` | Write the spec copy in which one Component is rebuilt: only the named paths are removed, and neighbouring Components and their Component Interfaces stay. |
| `givn ise compare --manifest <file> --copy <dir>` | Report every difference between the spec-copy manifest and the Specification files in the spec copy, the Specification files in the project working tree, and the End-to-end (E2E) tests. |

**Rebuild form.** `--at` defaults to `HEAD`. `rebuild-all` takes no path.
`rebuild-component` requires at least one path; none is a usage error with exit
`2` that names the missing path list. A path is a tracked file or a directory: a
file contributes itself, a directory contributes every tracked file beneath it
at the revision, a value that resolves to no tracked file removes nothing, and
a glob is never expanded. The Specification Manifest and then a Specification
category are tested first, so a matched file under a `source:` entry stays in
the copy under both commands. A path outside every `source:` entry is refused
with a sentence naming `rebuild-all`, the command that writes the copy removing
every declared `source:` entry.

**Specification Manifest.** `givn/specification.yaml` is an implicit
Specification entry, not a sixth category. Every spec copy holds the copy
recorded at the selected revision, the partition reads that same copy, and the
Specification set is a union, so a category that also matches the Specification
Manifest counts it once. Naming the Specification Manifest with a Component
path, or naming a directory whose tracked files include it, refuses; a selected
revision that does not record it refuses before any write, naming the path, the
revision, and `does not record`.

**Placement.** The spec copy is a plain directory outside the project working
tree with no version-control metadata, no ignored file, and no untracked file.
Every path is project-relative and every entry kind is preserved, and no
End-to-end (E2E) test enters it. `--manifest` writes the spec-copy manifest;
without it none is written and none is printed.

**Report.** The command prints these lines:

```text
Backend: git
Revision: 8c1f2ab7d4c0e1f2a3b4c5d6e7f8091a2b3c4d5e
Specification files: 20
Source code kept: 72
Source code removed: 2
Outside the spec copy: 3
Files beside removed paths: 2
Specification checksum: 4f1c9a2b7d01
End-to-end checksum: 82be07c3e5a9
End-to-end tests: not copied (12 files)
Copy directory: /tmp/ise-passuff
Manifest: /tmp/ise-passuff.yaml
```

The contract is eleven fixed lines and a twelfth when a manifest path was
given. `Manifest` prints only when `--manifest` was given. `--dry-run` performs
every read and refusal, writes nothing, and prints `Copy directory (would
write):` and, only when a manifest path was given, `Manifest (not written):`
instead of the two write lines.

The five counts are placement counts, one definition each. `Specification files`
is the number of tracked files the copy was written with: the Specification
Manifest and every matched file of the four content categories that the
End-to-end (E2E) tests do not cover. `Source code kept` is the number of tracked
files under a declared `source:` entry that the command did not remove and that
are not Specification files in the spec copy and not End-to-end (E2E) matches; a
matched file of a content category under a `source:` entry is a Specification
file in the spec copy, so a Whole-system rebuild with one such file reports
`Source code kept: 0`. `Source code removed` is the number of tracked files the component paths, or
every declared source root under `rebuild-all`, removed. `Outside the spec copy`
is the number of tracked files outside every declared category and `source:`
entry, which the copy neither holds nor removes. `Files beside removed paths` is a
containment count: the direct tracked files in the removed files' parent
directories that the command did not name and that are not themselves removed.
A file it counts is also counted in `Specification files`, `Source code kept`,
or `Outside the spec copy`, so the five counts do not partition the tracked
set.

`End-to-end tests: not copied (<n> files)` names the End-to-end (E2E) set's own
size, never the manifest's `end_to_end_files` length, which also holds the
feature paths that carry scenario checksums. An entry that matches the project
root covers the whole tracked tree, and an End-to-end (E2E) match under a
`source:` root is neither copied nor removed.

**Refusals.** Every refusal exits `1`, prints `ise rebuild-all:`,
`ise rebuild-component:`, or `ise compare:` to match the command, and leaves the
project working tree and the target unchanged.

| # | Condition | Named output |
| --- | --- | --- |
| 1 | The revision records the Specification Manifest but the backend returns no blob | `givn/specification.yaml`, the revision, and `returned no blob` |
| 2 | The Specification Manifest bytes at the revision are not UTF-8 | `givn/specification.yaml` and the decoding error |
| 3 | The Specification Manifest at the revision is malformed | `givn/specification.yaml` and the loader's reason |
| 4 | The revision does not record the Specification Manifest, or a declared manifest entry matches nothing | `givn/specification.yaml`, the revision, and `does not record`, or the entry and the revision |
| 5 | `ise` off, or `architecture_principles` off | the add-on and its enable command |
| 6 | Dirty project working tree | every dirty path |
| 7 | Missing, mixed, or nested backend | the selector's message |
| 8 | A component path outside every source root | the path and `rebuild-all` |
| 9 | A Component path or tracked file under a named directory is the Specification Manifest or is matched by a Specification category | the path and the Specification Manifest, or the path and the category |
| 10 | A target inside the project working tree, existing, inside the copy, an ancestor of the copy, or an existing spec-copy manifest | the target or manifest path |
| 11 | A repository link or nested marker in the copy set | the file |
| 12 | A symlink with an absolute target, even one that resolves inside the copy, or with a relative target that resolves outside the copy | the link path and the target |

R-150 covers the archive guard reading a declared manifest entry that is a
symlink into the change tree; row 12 covers a symlink entry written into the
copy, including an absolute target that resolves inside it. The two checks read
different surfaces.

**The rule the copy carries.** The spec copy is written with its Specification
files in the spec copy. During a rebuild the agent adds files, and those are the
rebuilt Source code. The agent may edit Source code kept because it is the
System's Source code, but it does not delete any file the copy was written with.
The agent does not modify any Specification file in the spec copy; adding a file
under a declared manifest entry is a Specification change, not the System, and
is refused.

**Rebuild rules.** The `ise` instruction carries these rules; the reference
states what each requires.

| Rule | Requirement |
| --- | --- |
| Files the rebuild needs | A file the build reads that is not Source code and whose content the build cannot resolve from the Specification text is declared under the category that owns its content class: a build configuration file (`Cargo.toml`) and a lockfile under `contracts`, because they fix the dependency and build-target schema; a toolchain pin under `limits`; a structural template such as `assets/templates/domain-model.md` or `assets/templates/glossary.md` under `structure`; a schema or migration asset under `contracts`; and served instruction or skill text under `behavior`. The frozen End-to-end (E2E) tests fix the bytes they assert; a structural template or a schema asset fixes its content through the contract that names its shape. The `e2e` package's own `Cargo.toml` and lockfile are the test build's inputs, not the System build's. A file the rebuild needs that the Specification Manifest leaves undeclared is a `specification defect`, with its consequence. |
| Fixed values | A contract the frozen End-to-end (E2E) tests rely on enumerates every value it fixes: literal text, headings, labels, error prefixes, exit codes, file formats, and test seams. |
| Acceptance corpus | The `@e2e` scenarios stay in the Specification under the `behavior` category with their exact titles. The declared `e2e` package holds the runner and every `@e2e` step definition. |
| Clean-room boundary | The builder reads no Source code outside the spec copy and the declared inputs. Before a copy-based rebuild, scan the local package registry's source, cache, and git stores for a published copy of the System, quarantine every hit outside the builder's reach, and record the scan on its own `Clean-room scan` line in the trial review; a store that does not exist has no hit. |
| Rebuild impact record | Every file added, modified, deleted, moved, or renamed outside the declared Specification and outside the replaced unit, and every consolidation, with its reason. The trial review records it; it is a review question, never a gate. |
| Rebuilt code: discard | A discard rebuild adopts nothing: the rebuilt work stays in the copy, and the trial review records that no rebuilt file was promoted. |

The installed toolchain and dependency cache are never declared files: the
lockfile declares the resolved dependencies, the toolchain pin declares the
toolchain, and the trial records how the dependency cache and the installed
toolchain were supplied. A locked
dependency the cache cannot resolve blocks the rebuild; the blocked rebuild is a
setup failure, not a `Spec gap`, and it is not a `specification defect`.

**Spec-copy manifest.** `ise-manifest/v1` is strict: an unreadable file, a
scope other than `ise-manifest/v1`, an unknown field, or a missing field refuses
with the manifest path and the reason. The earlier `ise-trial/v1` record shape is refused
by name and is not migrated: a spec copy is ephemeral, and one rebuild command
re-prepares it. This scope has no optional field: the reader requires every
field it lists. A field added later is optional only when the scope states its
default, and a field with no derivable default opens a new scope; a reader
refuses a mismatched scope naming the manifest and the expected value. The
manifest records no `backend`: the prepare-time backend is not evidence, and
`compare` re-selects the backend from the project working tree.

| Field | Content |
| --- | --- |
| `scope` | `ise-manifest/v1` |
| `revision` | the resolved revision |
| `removed_paths` | the sorted tracked files the command removed |
| `specification_files`, `source_kept_files`, `end_to_end_files` | the sorted path inventories; `end_to_end_files` is the sorted union of the keys of `checksums.end_to_end` and `checksums.scenarios` |
| `counts` | `specification_files`, `source_kept`, `source_removed`, `outside_spec_copy`, `files_beside_removed_paths` |
| `checksums.specification`, `checksums.end_to_end`, `checksums.scenarios` | one checksum per path; `end_to_end` holds one path checksum per `end_to_end_files` path, including a scenario-carrying feature; a scenario checksum wins in the rollup when a path is in both |
| `specification_checksum`, `end_to_end_checksum` | one rollup each |
| `copy_directory`, `givn_version` | the spec copy and the running Givn version |

`specification_files` always includes `givn/specification.yaml` and never an
End-to-end (E2E) path, and its length equals `counts.specification_files`.
`source_kept_files` is the sorted set of copy paths that are not Specification
files in the spec copy; it records paths without checksums, so a kept edit is not a difference
while a deleted kept path is refused. The manifest is ephemeral: it lives
outside the copy and the project working tree, is not archived or committed,
and only `compare` requires it.

**Checksums.** Per path:
`sha256(path NUL kind NUL executable-marker NUL content)` with kind `f`
(regular), `x` (executable regular), `l` (symlink), or `g` (gitlink), marker
`1` or `0`, and content the regular file's bytes, the symlink target's bytes,
the recorded commit id for a gitlink, or the bytes the backend returns for the
entry. An End-to-end (E2E) path may be a gitlink and hashes as kind `g`; a
gitlink in the copy set refuses because the copy cannot hold history. The value is lowercase hex. A rollup hashes each
sorted `path NUL checksum LF`; `specification_checksum` rolls up
`checksums.specification`, and `end_to_end_checksum` rolls up the union of
`checksums.end_to_end` and `checksums.scenarios` keyed by path, with a scenario
checksum winning when one path is in both maps. The report prints the first 12
lowercase hex characters; the manifest keeps the full value.

**End-to-end enumeration.** The End-to-end (E2E) set is read from the selected
revision's tracked paths, never the project working tree. Each declared `e2e` entry
matches tracked files and tracked directory prefixes. A matched file resolves
through its parent directory, and each match takes as its root the nearest
directory at or above it that holds a tracked `Cargo.toml`, or the matched path
itself when no such directory exists. The set is every tracked file at or below
a root, minus the Specification Manifest, ordered by path. An entry that
matches the project root covers the whole tracked tree. Ignored and untracked
files cannot enter. The scenarios themselves stay in the `behavior` category;
the `e2e` category holds the runner and every `@e2e` step definition.

**Scenario checksum.** The per-feature checksum is `sha256(feature path NUL
(title NUL executable-content NUL)*)` over the selected scenarios in file
order. A feature-level `@e2e` tag selects every scenario; otherwise each
scenario carrying `@e2e` is selected; a scenario under a `Rule:` is never
selected; a feature that selects none contributes no entry. The executable
content is the compact JSON array `[scenario keyword, background, tags, steps,
examples]`: `background` is null or the background steps; `tags` is the sorted,
duplicate-free union of the feature's and the scenario's tags with every tag
beginning `givn.` and the tag `wip` removed; a step is `[trimmed keyword,
Given|When|Then, text, docstring, table]`, where `And`, `But`, and `*` inherit
the preceding type (or `Given` when none precedes), the text has trailing
whitespace removed, the docstring has the common indentation removed and loses
the line break that follows the opening delimiter and the line break before the
closing delimiter, and the table is its rows with the Gherkin
escapes decoded; `examples` is one `[tags, table]` pair per `Examples:` block,
with the same tag filter. The JSON is UTF-8 with no insignificant whitespace,
`"` and `\` escaped, the short control escapes, every other code point below
U+0020 as `\u00xx`, non-ASCII as literal UTF-8, and `/` unescaped. No timestamp
is recorded.

**Entry kinds.** Git derives kind and executable from the `git ls-tree` modes:
`100644` is `f`, `100755` is `x`, `120000` is `l`, and `160000` is `g` with the
recorded commit id as the content. A `160000` entry never reaches the copy set,
which refuses a repository link by name; it may reach the End-to-end (E2E) set,
which hashes it as `g`. Fossil derives them from the
check-in manifest's `F` lines (hash, `x`, `l`) read by `fossil artifact`, with
`\s` in a path decoded to a space; Fossil records a symlink only when the
repository has `allow-symlinks on`.

**Compare.** `givn ise compare` reads the spec-copy manifest, confirms the
canonicalized `copy_directory` equals `--copy`, and requires the manifest path
to lie outside the copy. It then prints two subject lines:

```text
Specification files: match
End-to-end tests: match
```

`Specification files` and `End-to-end tests` are the two subjects. Each prints
`match` alone, or its non-zero counts in the fixed order `changed`, `missing`,
`added`, comma-separated; a kind with no difference is omitted. For example,
one changed and one added matched file prints
`Specification files: 1 changed, 1 added`. A changed or missing path prints one
`- <path>` line and an added path prints one `+ <path>` line, one line per path
per run; a path that drifts in both the copy and the project working tree prints
one line.
When a path differs on both sides, that line and its count belong to the
subject that owns the path: `End-to-end tests` when the project working tree
side owns it, `Specification files` otherwise. The copy-side and project-side
observations are one difference, not two.
The exit is `0` only when both lines say `match`; any difference exits `1`.

The `Specification files` line compares the spec-copy manifest's per-path
checksums with the copy: a differing file is `changed` and an absent file is
`missing`. It compares the spec-copy manifest's recorded paths with the project
working tree's content paths. A recorded path carrying a selected `@e2e` scenario
is owned on the project working tree side by `End-to-end tests` and contributes no
`Specification files` difference there. For every other recorded path, an
absent file is `missing` and a differing file prints
`Specification changed after the copy was written` and `changed`. A content
path matched by a declared manifest entry that the spec-copy manifest does not
record prints `added`, except a content path in the project working tree's
declared End-to-end (E2E) set or a feature path carrying a selected `@e2e`
scenario, which `End-to-end tests` owns. Membership in the project working
tree's declared End-to-end (E2E) set, or a selected `@e2e` scenario in a feature
path, decides the subject for an added path, even when another Specification
category also matches it.

The `End-to-end tests` line compares the spec-copy manifest's End-to-end (E2E)
paths with the project working tree's content paths: a recorded path that is absent is `missing`; one
whose `checksums.end_to_end` content checksum differs, or whose selected-scenario
checksum in `checksums.scenarios` differs, is `changed`; and a content path in
the project working tree's declared End-to-end (E2E) set, or a feature path
carrying a selected `@e2e` scenario, that the spec-copy manifest does not record
is `added`. For a
feature path carrying a selected `@e2e` scenario, `checksums.end_to_end` holds
the file's content checksum and `checksums.scenarios` its selected-scenario
checksum; either difference is `changed`, so an edit outside the selected
scenario is named by `End-to-end tests`.

A content path in the project working tree's declared End-to-end (E2E) set is
owned on that side by `End-to-end tests`, even when another Specification
category also matches it. Its content, or its selected-scenario checksum when it is a feature
path, decides `changed`, and no `Specification changed after the copy was
written` sentence is printed for it. The copy side still compares the same
file's bytes under `Specification files`, because the copy was written with the
file as a Specification file in the spec copy.

The project working tree's added paths are read with its Specification Manifest;
the copy's added-file refusal and its recorded paths are read with the copy's
Specification Manifest. A Specification Manifest the loader refuses is already a
`changed` or `missing` difference and contributes no categories.

The comparison refuses before printing any count when the copy holds a file
under a declared manifest entry that the spec-copy manifest does not list,
naming at most twenty sorted paths and the remainder count. A declared manifest
entry is a match of any of the five categories, so a file added under an `e2e`
entry refuses even though the copy never carries one:

```text
ise compare: the spec copy adds a file under a declared Specification path; Specification files are not the product: givn/specs/extra.feature
```

It refuses on the same terms when the copy lost a path the spec-copy manifest
lists among the Source code kept, naming the deleted path: the agent may edit
kept Source code because it is the System's Source code, but it does not delete
a file the copy was written with. Kept content is not verified, so an edit is not a difference
(R-162).

**Project working tree enumeration.** The current `e2e` package, the `behavior`
feature set, and the file bytes are read from the selected backend's content paths:
tracked files plus untracked non-ignored files, sorted and deduplicated, with
bytes read from disk. An uncommitted edit is seen, an added non-ignored file is
seen, and an ignored file is not. A recorded End-to-end (E2E) path the project
working tree no longer lists is `missing`; one whose content differs, or whose
selected-scenario checksum differs, is `changed` under `End-to-end tests`.

A clean comparison prints:

```text
Specification files: match
End-to-end tests: match
```

A copy whose Specification drifted prints:

```text
Specification files: 1 changed
- givn/specs/demo.feature
End-to-end tests: match
```

A project working tree whose End-to-end (E2E) tests drifted prints:

```text
Specification files: match
End-to-end tests: 1 changed
- tests/e2e/steps/keys.rs
```

A project working tree that gained an End-to-end (E2E) file prints:

```text
Specification files: match
End-to-end tests: 1 added
+ tests/e2e/steps/extra.rs
```

| Difference | Meaning |
| --- | --- |
| `Specification files: <n> changed` | A recorded Specification file in the spec copy differs, or a matching file in the project working tree differs when the `End-to-end tests` line does not own it. |
| `Specification files: <n> missing` | A recorded Specification file in the spec copy is absent, or its matching file is absent from the project working tree when the `End-to-end tests` line does not own it. |
| `Specification files: <n> added` | The project working tree holds a content path matched by a declared manifest entry that the spec-copy manifest does not record, that lies outside the project working tree's declared End-to-end (E2E) set, and that is not a feature path carrying a selected `@e2e` scenario; such a path is owned by the `End-to-end tests` line even when another Specification category also matches it. |
| `End-to-end tests: <n> changed` | A recorded End-to-end (E2E) path differs in the project working tree, or its selected scenarios changed. |
| `End-to-end tests: <n> missing` | A recorded End-to-end (E2E) path is absent from the project working tree. |
| `End-to-end tests: <n> added` | The project working tree's declared End-to-end (E2E) set, or a feature carrying a selected `@e2e` scenario, holds a path the spec-copy manifest does not record. |
| `Specification changed after the copy was written` | A Specification file in the project working tree that the `End-to-end tests` line does not own differs from the spec-copy manifest. |

**Specification Manifest in comparison.** The copy's Specification Manifest is
compared like every other Specification file in the spec copy, so an edited or
deleted one is a difference naming the path. The copy's Specification Manifest
derives the entries the added-file refusal tests, so the five categories
including `e2e` bound the refusal. A copy whose Specification Manifest is absent
or unreadable contributes no categories: the added-file refusal is skipped, and
the recorded path is `missing` or `changed` like any other Specification file in the spec copy.
The project working tree's Specification Manifest derives its added paths,
including an added `e2e` root and the files it brings. An edit that moves its
`structure:` or `limits:` entries changes the Specification Manifest's content like any
other edit; a content path the moved entry newly matches is an added path. A
project working tree without a Specification Manifest still compares the
spec-copy manifest's recorded paths; a recorded path absent from the project
working tree is a `missing` difference, and the added-path scan contributes no
categories.

Only the frozen inputs are compared with the project working tree: a file the
rebuild adds under a `source:` entry is Source code, and the comparison never
edits the copy.
Every check runs before the report, so one run names every differing path.
Exit codes: `0` clean or success, `1` refusal, difference, or read failure, `2`
a clap usage error.

`givn ise compare` is read-only and reports input equality only. After the two
subject lines it prints:

```text
Trial result: unverified (frozen inputs only; run `givn ise verify-trial ...`)
```

Exit `0` means the frozen inputs match; it does not mean the component corpus
ran.

**Verify trial.** `givn ise verify-trial --manifest <file> --copy <dir>
--result <file>` is the only command that reports a whole-system trial result.
It first requires `givn ise compare`'s frozen inputs to match, then reads the
generated `givn/ise-verification.yaml` from the copy. The rebuild records that
contract's path and checksum in the `ise-manifest/v1` manifest; a missing,
changed, or unlisted contract refuses before any command runs.

The contract names a component command and feature path, a frozen End-to-end
command and feature path, an execution-unit count and every scenario title for
each tier, the rebuilt product path and the environment variable the End-to-end
command reads, and the coverage commands and paths when the coverage addon is
effective. An execution unit is one plain scenario or one example row of a
Scenario Outline.

It re-derives each tier's execution-unit multiset from the frozen `.feature`
files in the copy and refuses a contract whose revision, counts, or titles
differ. It removes each raw result file, runs the component command inside the
copy with `GIVN_FEATURES`, `GIVN_RESULT_FILE`, and
`GIVN_RESULT_SCOPE=component`. It then requires the declared
`verify.product_path` as a regular non-symlink file or directory inside the
copy, hashes a file's bytes or a directory tree's regular files, and runs the frozen End-to-end command in the project working tree with
`GIVN_RESULT_SCOPE=e2e`, `verify.product_env`, `GIVN_TRIAL_PRODUCT`, and
`GIVN_TRIAL_PRODUCT_SHA256`. Each run must produce a fresh result that names
every declared scenario title and no undeclared title, and whose `total`
equals the declared count exactly, with zero failed, zero skipped, and reconciling
`passed + failed + skipped == total`. The End-to-end result must carry the same
`product` path and `product_sha256`. A missing or stale result, a count-only or
title-less result, a foreign product, a non-zero failure or skip, or a count
mismatch refuses and records `clean: false`; no pass line prints.

When the contract carries coverage, the command removes both reports, runs
`coverage.measure_command` inside the copy with `GIVN_COVERAGE_NON_E2E_PATH`,
`GIVN_COVERAGE_E2E_PATH`, `GIVN_COVERAGE_OUTPUT_PATH`, and `GIVN_FEATURES`,
requires a component report with non-zero measured lines and executed lines,
requires independent tier reports, and accepts a 0-line End-to-end report only
when `coverage.e2e_subprocess: true` is declared.

After the scripts ran, the command re-runs the frozen-input comparison. A
checkout, manifest, or contract change during the trial refuses and records
`clean: false`.

The result file is an `ise-trial-result/v1` JSON record with the revision, the
copy directory, each tier's `declared`, `executed`, and `scenarios`, the
`product` path and `sha256`, per-tier line and branch coverage, and `clean`. The
command exits `0` only when `clean` is true.

`--component-not-run <reason>` records an explicit not-run decision. It is
refused when the Specification declares component execution units; it never
turns a declared corpus into a pass.

Givn does not inspect external or manual trial reports such as
`givn/trials/**/report.md`; they are outside Givn enforcement. Only
`givn ise verify-trial` writes an `ise-trial-result/v1` with `clean: true`.

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

With `ise` effective, the fresh move also refuses before any mutation when a
manifest entry in `givn/specification.yaml` matches a content path under
`givn/changes/<id>/` and survives neither through a content path outside the
move nor through the `givn/archive/<id>/` form of the content path it matches.
The refusal names the category, the entry, the moved path, and the archived path to
declare instead; a literal entry is answered with its archived literal path and
a wildcard entry with the same pattern under `givn/archive`. An entry such as
`givn/**` that still resolves after the move is unaffected, and a project with
`ise` off reads no Specification Manifest.

A failed or incomplete candidate remains in `givn/archive/<id>/` with a pending
receipt. Repair the committed state and rerun archive; the merge is idempotent
and finalized scopes are not re-run. There is no rollback or history rewrite.
Finalization writes the proven receipt only when `HEAD` is the tested revision
and every dirty path is a declared evidence path, then publishes the
evidence-only finalization revision. An already proven archive is refused.

### Upgrade recovery

`givn upgrade` requires a clean project working tree in the selected backend and captures a
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
configuration. Un-commented configuration values are project-owned and preserved
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
./run-tests.sh --e2e --package e2e   # this repository's verify.e2e_command scope
./measure-coverage.sh        # non-E2E and E2E Cobertura reports
./merge-coverages.sh         # merged report used by README badges/blocks
```

After archiving a change, refresh the local binary and generated integrations:

```sh
cargo install --path . --locked --force
givn skills sync
givn instructions
./run-tests.sh --non-e2e
./run-tests.sh --e2e --package e2e
```

*[Documentation index](INDEX.md)*
