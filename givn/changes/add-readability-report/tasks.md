# Tasks: add-readability-report

Status: implementation task list. Work the non-`@e2e` scenarios first, in
`.feature` order, then the `@e2e` setup and scenarios. One commit per
scenario. Tick each box only when its evidence and `COMMIT:` hash are filled.

## Use-case context (copied from `usecase.md` and the design)

- Owning use case: `assess-code-structure`; goal: see the structure of an
  existing codebase and the units that need a plan or exception.
- Actors: **Developer**; supporting: **Authoring agent**. Personas are review
  lenses, not Gherkin actors: `developer--primary-user`,
  `agent--technical-role`, `givn-maintainer--owner`.
- Rules: the report is advisory and never changes an exit code; it stores no
  baseline, watchlist, or classification history; it is deterministic and
  offline; it uses production sources only in the first release; the report
  format is independent of the analysis engine; every analyzed unit has exactly
  one classification and one action.
- Addon rule: `readability` is default-off; `givn init` enables only
  default-on addons; `givn addons enable readability` opts in.
- Accepted limitations: non-test `#[cfg(...)]` branches are counted as written;
  a parent-module `#[cfg(test)]` is not propagated, so `src/tests.rs` and
  `src/tests/**` count as production (arc42 R-97).
- Scope: default project-root `src/`; repeatable root-relative `--path <dir>`
  replaces it; absolute values and `..` are CLI usage errors; an explicit
  missing path exits 1; an existing empty scope exits 0.

## Design constraints (copied from `design.md`; apply to every scenario)

- Modules: `src/readability/{mod,model,analyzer,scan,metrics,classify,report,syn_adapter}.rs`
  and `src/cli/readability.rs`. `pub mod readability` in `src/lib.rs`.
- Public pipeline for integration-test steps: `readability::analyze(request,
  analyzer)`, `readability::assess(analysis)`, and
  `readability::report::{render_human, render_json}`. The port, model, metrics,
  classification, and report modules are `pub`; `syn` appears only in
  `syn_adapter.rs` and `src/cli/readability.rs` imports none.
- Neutral model: no `syn` types; `Clone + Debug + PartialEq`. Types and fields
  are the ones in `design.md` §Data model, including `SourceSpan`, `Construct`,
  `Limitation`, `UnsupportedNote`, `MetricValue`, `Comparison`, `MethodFinding`,
  `CallId`, and `Visibility`.
- Metrics and thresholds are `rules_version: 1`; the full precedence table and
  evidence rules are `design.md` §Classification and actions. Findings never
  change the exit code. Human and JSON render from the same classified model;
  JSON is schema v1.
- Addon registry: `AddonSpec { id, description, default_enabled }`;
  `readability` is the only `default_enabled: false` entry. `givn addons list`
  renders `[disabled]  readability`. Disabled report exit is 1 with stderr
  containing `readability experiment is disabled`.
- `--out <path>` writes the selected form and prints nothing on stdout; a
  missing parent or unwritable path exits 1 naming the path; the file is
  overwritten.
- `--path <dir>` is repeatable and root-relative; overlapping roots are
  deduplicated; the file set is sorted by `RepoPath`; each file is analyzed
  once. Default scope is `src/`.
- Report decisions: no timestamps, no absolute paths, no analyzer identity,
  deterministic ordering, relative `/`-separated paths, integer permille.
- Test infrastructure: `tests/steps/readability_report_steps.rs` for non-E2E
  steps (CLI scenarios use `common::run_givn`; pipeline scenarios call the
  public API in process), `tests/steps/readability_report_e2e_steps.rs` for
  `@e2e`, `tests/steps/readability_report_support.rs` for fixtures, and module
  registration in `tests/steps/mod.rs`. `features_runner` already has
  `fail_on_skipped()`. Do not add `tests/analyzer_contract.rs`, do not change
  `run-tests.sh`, and do not change `verify.command` or `verify.e2e_command`.
- RED/GREEN/REFACTOR runs use the design's exact command, replacing the feature
  path and title:
  `GIVN_FEATURES=givn/changes/add-readability-report/specs cargo test --locked --test features_runner -- "<feature path>::<scenario title>" --exact --nocapture`
- Step bodies are never empty. RED starts with `unimplemented!("<step>: no
  assertion yet")`; GREEN replaces every stub with a real assertion.

## Setup task — runner, strict proof, and step skeleton

- [x] Confirm the existing runner and strict mode.
  - `verify.command` is `./run-tests.sh --non-e2e` and `verify.e2e_command` is
    `./run-tests.sh --e2e` in `givn/commands.yaml`; leave them unchanged.
  - `tests/features_runner.rs` already calls
    `GivnWorld::cucumber().fail_on_skipped()` and exits non-zero on
    `skipped > 0`.
  - Create `tests/steps/readability_report_steps.rs`,
    `tests/steps/readability_report_e2e_steps.rs`, and
    `tests/steps/readability_report_support.rs`; register all three in
    `tests/steps/mod.rs`.
  - Prove strictness with a temporary feature containing one undefined step:
    `probe=$(mktemp -d); printf '@e2e\nFeature: strict probe\n  Scenario: undefined step\n    Given a step with no definition\n' > "$probe/probe.feature"; GIVN_FEATURES="$probe" ./run-tests.sh --e2e`
    must exit non-zero.
  - Remove the probe and run `GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e`
    to restore the green baseline. The change scenarios stay `@wip`; they are
    not part of that baseline until their scenario is worked.
  - ADR-074 was accepted during setup because the permanent register scenario
    asserts no proposed decisions: `docs/arc42/adr/ADR-074-*` is `accepted`,
    the register counts are 26 accepted / 0 proposed, and
    `fragments/adr-qualification-gate.feature` plus its step expect 26.
  - Evidence:
    ```
    $ GIVN_FEATURES=<probe> ./run-tests.sh --e2e
    FAIL [0.491s] probe.feature::undefined step
    1 scenario (1 failed), 1 step (1 failed)
    EXIT: 100
    ```
    ```
    $ GIVN_FEATURES=givn/specs ./run-tests.sh --non-e2e
    Summary [56.972s] 591 tests run: 591 passed, 104 skipped
    EXIT: 0
    ```

## Capability: enable-optional-addons (non-`@e2e`)

### Scenario: givn init enables only default-on addons

- [ ] RED: remove `@wip` from this scenario only; all steps start as
  `unimplemented!`. Run the targeted command and paste non-zero output.
  - Target: `initialize-project/enable-optional-addons.feature::givn init enables only default-on addons`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: replace the addon registry with `AddonSpec`, make `givn init`
  enable only `default_enabled` specs, keep the existing default-on addons and
  their generated files, and leave `readability` absent/false. Update the
  `src/cli/init.rs` and `assets/config.yaml.tmpl` comments from "all addons"
  to the default-on rule. Run the targeted command; exit 0.
  - Production files: `src/cli/addons.rs`, `src/cli/init.rs`,
    `src/config/manifest.rs` (`Addons.readability` with `#[serde(default)]`),
    `assets/config.yaml.tmpl`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: remove duplication in the enable/print loops without changing
  output; re-run the targeted command; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(enable-optional-addons): initialize with default-on addons only`
  - Body: `givn init enables only default-on addons`
  - `COMMIT:` <hash>

### Scenario: givn init lists default-off addons and how to enable them

- [ ] RED: remove `@wip` from this scenario only; stub the new steps; run the
  targeted command; must exit non-zero.
  - Target: `initialize-project/enable-optional-addons.feature::givn init lists default-off addons and how to enable them`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: after the default-on print loop, print
  `addons available (default off): readability — enable with givn addons enable readability`;
  keep `givn addons list` rendering `[disabled]  readability` and the
  `givn addons enable` hint. Run the targeted command; exit 0.
  - Production files: `src/cli/init.rs`, `src/cli/addons.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: share the addon-label formatting between init and list; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(enable-optional-addons): list default-off addons after init`
  - Body: `givn init lists default-off addons and how to enable them`
  - `COMMIT:` <hash>

## Capability: readability-report (non-`@e2e`)

### Scenario: The readability experiment is disabled by default

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run the
  targeted command; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The readability experiment is disabled by default`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: add the `readability` addon spec with `default_enabled: false`,
  the `Addons.readability` field, and the `givn addons list` rendering. The
  scenario asserts the addon is listed as disabled after `givn init` without
  enabling it. Run; exit 0.
  - Production files: `src/cli/addons.rs`, `src/config/manifest.rs`,
    `assets/config.yaml.tmpl`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep the registry the single source of truth; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): keep the readability experiment disabled by default`
  - Body: `The readability experiment is disabled by default`
  - `COMMIT:` <hash>

### Scenario: The readability report is unavailable when the experiment is disabled

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run the
  targeted command; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The readability report is unavailable when the experiment is disabled`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: register `givn readability report [--json] [--out <path>]
  [--path <dir>]...` always, gate it at runtime on the addon, and exit 1 with
  stderr `readability experiment is disabled; enable it with givn addons enable readability`.
  Run; exit 0.
  - Production files: `src/main.rs` (dispatch and `mod readability`),
    `src/lib.rs` (`pub mod readability`), `src/cli/mod.rs`,
    `src/cli/readability.rs` (addon gate and clap handler).
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep the gate in one helper used by all output paths; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): gate the report on the readability addon`
  - Body: `The readability report is unavailable when the experiment is disabled`
  - `COMMIT:` <hash>

### Scenario: A data-only type is not treated as a god type

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A data-only type is not treated as a god type`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement the minimal pipeline for one type: scan the default
  `src/`, parse with `syn`, build the neutral model, measure code lines and
  method count, and classify a struct/enum with `methods <= 1 && code_lines >
  500` as `data-only` with action `advisory` and no plan. Render the human
  report. Run; exit 0.
  - Production files: `src/readability/model.rs`, `analyzer.rs`, `scan.rs`,
    `metrics.rs`, `classify.rs`, `report.rs`, `syn_adapter.rs`, `mod.rs`;
    `src/cli/readability.rs`; `Cargo.toml` (`syn`).
  - Design constraints: rule 2 in the precedence table; a data-only unit
    never gets a plan at any size; evidence rule `type-method-count` and
    `type-code-lines`; the fixture is a 521-line enum with one method.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: separate line-kind counting from metric computation; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): exempt data-only types from the god-type rule`
  - Body: `A data-only type is not treated as a god type`
  - `COMMIT:` <hash>

### Scenario: An interface is classified as an interface, not a god type

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::An interface is classified as an interface, not a god type`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: add trait units and rule 1: `type_kind = trait && code_lines >
  500`, or `public_methods > 50`, classifies as `interface` with action `plan`
  and recommendation `trait-composition plan: split by trait composition and
  parameter objects, not by line count`. Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`, `model.rs`,
    `metrics.rs`, `classify.rs`, `report.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep trait mapping in the adapter and the rule above the port;
  re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): classify large traits as interfaces`
  - Body: `An interface is classified as an interface, not a god type`
  - `COMMIT:` <hash>

### Scenario: A cohesive type is not treated as a god type

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A cohesive type is not treated as a god type`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: add field accesses, method–field edges, LCOM4, TCC/LCC in integer
  permille, and rules 4–5: a type over 500 with `lcom4 <= 2 && tcc >= 0.8` is
  `cohesive-large-type` with `advisory`; the inspect band (TCC 0.5–0.8 or
  undefined) gets `exception`. The fixture stays below 1,000 lines so it
  exercises the advisory band. Run; exit 0.
  - Production files: `src/readability/model.rs`, `syn_adapter.rs`,
    `metrics.rs`, `classify.rs`, `report.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep TCC/LCC as integer permille and evidence rows above the
  port; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): classify cohesive large types as advisory`
  - Body: `A cohesive type is not treated as a god type`
  - `COMMIT:` <hash>

### Scenario: A type at the readable size limit is within limits

- [ ] RED: remove `@wip` from this scenario only; stub the in-process steps;
  run; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A type at the readable size limit is within limits`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: make the public pipeline callable from the step file
  (`analyze`, `assess`, renderers) and add the boundary fixture generator for
  exactly 500 code lines with cohesive methods. It must classify `within-limits`
  with action `none`; no rule fires at exactly 500. Run; exit 0.
  - Production files: `src/readability/mod.rs` (public facade), `model.rs`,
    `metrics.rs`, `classify.rs`; `tests/steps/readability_report_support.rs`,
    `tests/steps/readability_report_steps.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep the generator parameterized by line count; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): pin the 500-line type boundary`
  - Body: `A type at the readable size limit is within limits`
  - `COMMIT:` <hash>

### Scenario: A type one line over the readable size limit is a finding

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A type one line over the readable size limit is a finding`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: generate 501 code lines with cohesive methods; classify as
  `cohesive-large-type`, action `advisory`, and record 501 code lines. Run;
  exit 0.
  - Production files: `tests/steps/readability_report_support.rs`,
    `tests/steps/readability_report_steps.rs`;
    `src/readability/metrics.rs` only if the boundary exposed a defect.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: assert the exact metric and classification through the public
  model, not rendered text; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): report types over the readable size limit`
  - Body: `A type one line over the readable size limit is a finding`
  - `COMMIT:` <hash>

### Scenario: A report is available in machine-readable form

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A report is available in machine-readable form`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: add `--json` schema v1 rendering from the same classified model;
  `stdout` is only the JSON document; diagnostics stay inside the document.
  Run; exit 0.
  - Production files: `src/readability/report.rs`, `src/cli/readability.rs`.
  - Design constraints: schema fields and deterministic ordering from
    `design.md` §JSON schema v1; `capabilities` carries support, never analyzer
    identity.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: share the unit ordering between human and JSON; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): add the JSON report`
  - Body: `A report is available in machine-readable form`
  - `COMMIT:` <hash>

### Scenario: A report is written to a file

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A report is written to a file`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement `--out <path>`: write the selected form, print nothing
  to stdout, overwrite the file. Run; exit 0.
  - Production files: `src/cli/readability.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: route both human and JSON through the same destination helper;
  re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): add --out`
  - Body: `A report is written to a file`
  - `COMMIT:` <hash>

### Scenario: A report output path that cannot be written is an error

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A report output path that cannot be written is an error`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: a missing parent or unwritable `--out` path exits 1 with stderr
  naming the path; do not fall back to stdout. Run; exit 0.
  - Production files: `src/cli/readability.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: use one `--out` error path; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): report --out write failures`
  - Body: `A report output path that cannot be written is an error`
  - `COMMIT:` <hash>

### Scenario: An unsupported construct is named and not invented

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::An unsupported construct is named and not invented`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: item-position macros produce `item-macros` diagnostics and
  wholly unsupported units are excluded from classification; the report names
  the construct and does not invent a unit. Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`, `model.rs`,
    `report.rs`, `classify.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep unsupported notes in the neutral model and out of
  classification; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): name unsupported constructs`
  - Body: `An unsupported construct is named and not invented`
  - `COMMIT:` <hash>

### Scenario: The JSON report carries the schema and rules versions

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The JSON report carries the schema and rules versions`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: emit `schema_version: 1` and `rules_version: 1`; changing a
  threshold changes only `rules_version`. Run; exit 0.
  - Production files: `src/readability/report.rs`, `classify.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: centralize the two version constants; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): expose report versions`
  - Body: `The JSON report carries the schema and rules versions`
  - `COMMIT:` <hash>

### Scenario: The JSON report carries analyzer capabilities and limitations

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The JSON report carries analyzer capabilities and limitations`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: render `capabilities.language`, `supported_constructs`,
  `limitations`, and `resolution_labels` from `AnalyzerCapabilities`; no
  analyzer identity. Run; exit 0.
  - Production files: `src/readability/analyzer.rs`, `report.rs`,
    `syn_adapter.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep the capability values in one adapter-owned constant; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): expose analyzer capabilities`
  - Body: `The JSON report carries analyzer capabilities and limitations`
  - `COMMIT:` <hash>

### Scenario: Conditional code is counted as written

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::Conditional code is counted as written`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: keep the accepted limitation: both `#[cfg(unix)]` and
  `#[cfg(windows)]` methods contribute; no cfg diagnostic and no
  `complete: false` for this case. Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`; the step fixture.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: add the fixture to the shared corpus; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): count conditional code as written`
  - Body: `Conditional code is counted as written`
  - `COMMIT:` <hash>

### Scenario: The same source produces the same report through either analysis engine

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The same source produces the same report through either analysis engine`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: add a fake `SourceAnalyzer` in the step support that builds the
  equivalent neutral `Analysis`; run metrics, classification, and both
  renderers; assert byte-identical output and no analyzer identity. Run; exit 0.
  - Production files: `src/readability/analyzer.rs`, `model.rs`, `mod.rs`;
    `tests/steps/readability_report_support.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep the fake adapter in the step support, not in production;
  re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): prove engine-independent rendering`
  - Body: `The same source produces the same report through either analysis engine`
  - `COMMIT:` <hash>

### Scenario: The report is deterministic

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The report is deterministic`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: sort traversal and every array; use no timestamps, host paths, or
  floating-point values; two runs produce byte-identical human and JSON. Run;
  exit 0.
  - Production files: `src/readability/scan.rs`, `report.rs`, `classify.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: centralize ordering; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): make the report deterministic`
  - Body: `The report is deterministic`
  - `COMMIT:` <hash>

### Scenario: Every analyzed unit has one classification and one action

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::Every analyzed unit has one classification and one action`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: make classification total over every analyzed unit; unsupported
  units are unclassified, every other unit has exactly one classification and
  one action. Assert over the fixture corpus and boundary fixtures. Run;
  exit 0.
  - Production files: `src/readability/classify.rs`, `mod.rs`; the step
    corpus helper.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: encode the precedence as one match/table; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): guarantee one classification and action`
  - Body: `Every analyzed unit has one classification and one action`
  - `COMMIT:` <hash>

### Scenario: A developer scopes the report to a workspace member

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A developer scopes the report to a workspace member`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: implement repeatable root-relative `--path`; scan
  `crates/core`; classify its unit and render project-relative paths. Reject
  absolute values and `..` at CLI parsing. Run; exit 0.
  - Production files: `src/cli/readability.rs`, `src/readability/scan.rs`,
    `analyzer.rs`, `mod.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: deduplicate overlapping roots before analysis; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): add the --path scope`
  - Body: `A developer scopes the report to a workspace member`
  - `COMMIT:` <hash>

### Scenario: The report says when the scope has no production sources

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The report says when the scope has no production sources`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: an existing empty default `src/` produces the documented message
  (human text) or the empty-scope JSON model and exits 0. Run; exit 0.
  - Production files: `src/readability/scan.rs`, `report.rs`,
    `src/cli/readability.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: share the empty-scope branch between human and JSON; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): report an empty scope`
  - Body: `The report says when the scope has no production sources`
  - `COMMIT:` <hash>

### Scenario: An explicitly requested scope that does not exist is an error

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::An explicitly requested scope that does not exist is an error`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: an explicit `--path` that does not exist exits 1 with stderr
  naming the path; a missing default `src/` remains the empty-scope exit 0.
  Run; exit 0.
  - Production files: `src/cli/readability.rs`, `src/readability/scan.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep default-vs-explicit validation in one place; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): reject a missing explicit scope`
  - Body: `An explicitly requested scope that does not exist is an error`
  - `COMMIT:` <hash>

### Scenario: A file that cannot be analyzed is reported and the report continues

- [ ] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A file that cannot be analyzed is reported and the report continues`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: parse errors become `parse-failed` diagnostics with the file and
  line; the file is skipped and the remaining units are still classified.
  Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`, `model.rs`,
    `report.rs`, `mod.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep per-file failures as data, not `Err`; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `feat(readability-report): continue after a failed file`
  - Body: `A file that cannot be analyzed is reported and the report continues`
  - `COMMIT:` <hash>

## E2E setup task

- [ ] Configure and prove the E2E scope.
  - `verify.e2e_command` is `./run-tests.sh --e2e`; it is a strict subset of
    `verify.command` via `GIVN_CUCUMBER_TAGS=@e2e`.
  - Run `GIVN_FEATURES=givn/changes/add-readability-report/specs ./run-tests.sh --non-e2e`
    and record the scenario count; run the same with `--e2e` and record the
    count. The E2E count must be strictly less than the full count.
  - The E2E step file is `tests/steps/readability_report_e2e_steps.rs`; it uses
    `common::run_givn` and `CARGO_BIN_EXE_givn`, a `tempfile::TempDir`, `givn
    init --agents opencode`, `givn addons enable readability`, and generated
    `.rs` files under `src/`.
  - No browser, server, database, container, or digital twin exists.
  - Evidence:
    ```
    NON-E2E COUNT
    ```
    ```
    E2E COUNT
    ```

## E2E scenarios

### Scenario: A developer runs a readability report and sees the classified units

- [ ] RED: remove `@wip` from this scenario only; stub the E2E steps; run the
  targeted E2E command; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A developer runs a readability report and sees the classified units`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: drive the real `givn` binary in a temp project with the addon
  enabled and an over-threshold type; assert stdout names the unit, the god
  type classification, the plan recommendation, and that no baseline file was
  written and the project tree is unchanged. Run; exit 0.
  - Production files: `tests/steps/readability_report_e2e_steps.rs`,
    `tests/steps/readability_report_support.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: keep the fixture builder shared with the non-E2E support
  module; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `test(e2e): run the readable report end to end`
  - Body: `A developer runs a readability report and sees the classified units`
  - `COMMIT:` <hash>

### Scenario: Consumer enables the readability addon and it is listed as enabled

- [ ] RED: remove `@wip` from this scenario only; stub the E2E steps; run the
  targeted E2E command; must exit non-zero.
  - Target: `initialize-project/enable-optional-addons.feature::Consumer enables the readability addon and it is listed as enabled`
  - Evidence:
    ```
    RED OUTPUT AND EXIT
    ```
- [ ] GREEN: `givn addons enable readability` writes `true`, prints
  `enabled addon 'readability' in givn/config.yaml`, and `givn addons list`
  renders `[enabled ]  readability`; `skills sync` runs as for every addon but
  generates nothing for this one. Run; exit 0.
  - Production files: `src/cli/addons.rs`, `src/config/manifest.rs`.
  - Evidence:
    ```
    GREEN OUTPUT AND EXIT
    ```
- [ ] REFACTOR: share the enable/disable config write; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR OUTPUT AND EXIT
    ```
- [ ] COMMIT: `test(e2e): enable and list the readability addon`
  - Body: `Consumer enables the readability addon and it is listed as enabled`
  - `COMMIT:` <hash>

## Completion checklist

- [ ] Every non-`@e2e` scenario has RED/GREEN/REFACTOR evidence and one
  `COMMIT:` hash naming the scenario in the message body.
- [ ] Both `@e2e` scenarios have real-subprocess evidence and one commit each.
- [ ] `givn lint --change add-readability-report` exits 0 or 2 (advisories
  allowed).
- [ ] `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` exit 0 on the
  change delta and on `givn/specs`.
- [ ] The E2E setup scenario-count proof is filled and shows the E2E count is
  strictly less than the full count.
- [x] ADR-074 is accepted and the register counts are updated (done during the
  setup task: 26 accepted / 0 proposed; the count scenario expects 26).
