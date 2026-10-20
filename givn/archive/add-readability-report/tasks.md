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

- [x] RED: remove `@wip` from this scenario only; all steps start as
  `unimplemented!`. Run the targeted command and paste non-zero output.
  - Target: `initialize-project/enable-optional-addons.feature::givn init enables only default-on addons`
  - Evidence:
    ```
    Scenario: givn init enables only default-on addons
     ✔ Given a project root without a givn/ directory
     ✘ When the user runs "givn init --agents opencode"
        Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: replace the addon registry with `AddonSpec`, make `givn init`
  enable only `default_enabled` specs, keep the existing default-on addons and
  their generated files, and leave `readability` absent/false. Update the
  `src/cli/init.rs` and `assets/config.yaml.tmpl` comments from "all addons"
  to the default-on rule. Run the targeted command; exit 0.
  - Production files: `src/cli/addons.rs`, `src/cli/init.rs`,
    `src/config/manifest.rs` (`Addons.readability` with `#[serde(default)]`),
    `assets/config.yaml.tmpl`.
  - Evidence:
    ```
    targeted scenario exit 0 (1 passed)
    ```
- [x] REFACTOR: remove duplication in the enable/print loops without changing
  output; re-run the targeted command; exit 0.
  - Evidence:
    ```
    targeted scenario exit 0
    ```
- [x] COMMIT: `feat(enable-optional-addons): initialize with default-on addons only`
  - Body: `givn init enables only default-on addons`
  - `COMMIT:` 49737abba0042a64b49742b90f59fa75ac0d9545

### Scenario: givn init lists default-off addons and how to enable them

- [x] RED: remove `@wip` from this scenario only; stub the new steps; run the
  targeted command; must exit non-zero.
  - Target: `initialize-project/enable-optional-addons.feature::givn init lists default-off addons and how to enable them`
  - Evidence:
    ```
    Scenario: givn init lists default-off addons and how to enable them
     ✔ Given a project root without a givn/ directory
     ✔ When the user runs "givn init --agents opencode"
     ✔ Then the command exits 0
     ✘ And stdout contains "addons available (default off): readability"
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: after the default-on print loop, print
  `addons available (default off): readability — enable with givn addons enable readability`;
  keep `givn addons list` rendering `[disabled]  readability` and the
  `givn addons enable` hint. Run the targeted command; exit 0.
  - Production files: `src/cli/init.rs`, `src/cli/addons.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 7 steps (7 passed)
    EXIT: 0
    ```
- [x] REFACTOR: share the addon-label formatting between init and list; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(enable-optional-addons): list default-off addons after init`
  - Body: `givn init lists default-off addons and how to enable them`
  - `COMMIT:` e0150ffa57cf306abcb5fd555e22f4c1a08ea86c

## Capability: readability-report (non-`@e2e`)

### Scenario: The readability experiment is disabled by default

- [x] RED: remove `@wip` from this scenario only; stub the steps; run the
  targeted command; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The readability experiment is disabled by default`
  - Evidence:
    ```
    ✘ Given a Givn project with the readability experiment disabled
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: add the `readability` addon spec with `default_enabled: false`,
  the `Addons.readability` field, and the `givn addons list` rendering. The
  scenario asserts the addon is listed as disabled after `givn init` without
  enabling it. Run; exit 0.
  - Production files: `src/cli/addons.rs`, `src/config/manifest.rs`,
    `assets/config.yaml.tmpl`.
  - Evidence:
    ```
    1 scenario (1 passed), 3 steps (3 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep the registry the single source of truth; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): keep the readability experiment disabled by default`
  - Body: `The readability experiment is disabled by default`
  - `COMMIT:` 40d1bb0a2bb79b8769a3368816156aad4710330d

### Scenario: The readability report is unavailable when the experiment is disabled

- [x] RED: remove `@wip` from this scenario only; stub the steps; run the
  targeted command; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The readability report is unavailable when the experiment is disabled`
  - Evidence:
    ```
    ✘ And stderr contains "readability experiment is disabled"
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: register `givn readability report [--json] [--out <path>]
  [--path <dir>]...` always, gate it at runtime on the addon, and exit 1 with
  stderr `readability experiment is disabled; enable it with givn addons enable readability`.
  Run; exit 0.
  - Production files: `src/main.rs` (dispatch and `mod readability`),
    `src/lib.rs` (`pub mod readability`), `src/cli/mod.rs`,
    `src/cli/readability.rs` (addon gate and clap handler).
  - Evidence:
    ```
    1 scenario (1 passed), 4 steps (4 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep the gate in one helper used by all output paths; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): gate the report on the readability addon`
  - Body: `The readability report is unavailable when the experiment is disabled`
  - `COMMIT:` a0afcdba5c5d00e534bb3d596bbd8be517bb8c99

### Scenario: A data-only type is not treated as a god type

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A data-only type is not treated as a god type`
  - Evidence:
    ```
    Scenario: A data-only type is not treated as a god type
     ✔ Given a Givn project with the readability experiment enabled
     ✔ And a production source unit with a data-only enum over the readable size
     ✔ When the developer runs "givn readability report"
     ✘ Then the report classifies the unit as data-only
        expected a data-only classification, got:
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: implement the minimal pipeline for one type: scan the default
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
    1 scenario (1 passed), 6 steps (6 passed)
    EXIT: 0
    ```
- [x] REFACTOR: separate line-kind counting from metric computation; re-run;
  exit 0.
  - Evidence:
    ```
    cargo fmt; 1 scenario (1 passed), 6 steps (6 passed)
    EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): exempt data-only types from the god-type rule`
  - Body: `A data-only type is not treated as a god type`
  - `COMMIT:` 9e99f915294361bf0c4e42551366af74e168e3c7

### Scenario: An interface is classified as an interface, not a god type

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::An interface is classified as an interface, not a god type`
  - Evidence:
    ```
    ✘ And a production source unit with a trait over the readable size
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: add trait units and rule 1: `type_kind = trait && code_lines >
  500`, or `public_methods > 50`, classifies as `interface` with action `plan`
  and recommendation `trait-composition plan: split by trait composition and
  parameter objects, not by line count`. Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`, `model.rs`,
    `metrics.rs`, `classify.rs`, `report.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep trait mapping in the adapter and the rule above the port;
  re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): classify large traits as interfaces`
  - Body: `An interface is classified as an interface, not a god type`
  - `COMMIT:` ecc00b209bdabc1bdd814a5ab3f7f0c876a63f28

### Scenario: A cohesive type is not treated as a god type

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A cohesive type is not treated as a god type`
  - Evidence:
    ```
    ✘ And a production source unit with a cohesive type over the readable size
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: add field accesses, method–field edges, LCOM4, TCC/LCC in integer
  permille, and rules 4–5: a type over 500 with `lcom4 <= 2 && tcc >= 0.8` is
  `cohesive-large-type` with `advisory`; the inspect band (TCC 0.5–0.8 or
  undefined) gets `exception`. The fixture stays below 1,000 lines so it
  exercises the advisory band. Run; exit 0.
  - Production files: `src/readability/model.rs`, `syn_adapter.rs`,
    `metrics.rs`, `classify.rs`, `report.rs`.
  - Evidence:
    ```
    1 scenario (1 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep TCC/LCC as integer permille and evidence rows above the
  port; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): classify cohesive large types as advisory`
  - Body: `A cohesive type is not treated as a god type`
  - `COMMIT:` d71c6df3191d56637b4235f68881977c8cbcf417

### Scenario: A type at the readable size limit is within limits

- [x] RED: remove `@wip` from this scenario only; stub the in-process steps;
  run; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A type at the readable size limit is within limits`
  - Evidence:
    ```
    1 scenario (1 failed), 1 step (1 failed)
    EXIT: 101
    ```
- [x] GREEN: make the public pipeline callable from the step file
  (`analyze`, `assess`, renderers) and add the boundary fixture generator for
  exactly 500 code lines with cohesive methods. It must classify `within-limits`
  with action `none`; no rule fires at exactly 500. Run; exit 0.
  - Production files: `src/readability/mod.rs` (public facade), `model.rs`,
    `metrics.rs`, `classify.rs`; `tests/steps/readability_report_support.rs`,
    `tests/steps/readability_report_steps.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 4 steps (4 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep the generator parameterized by line count; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): pin the 500-line type boundary`
  - Body: `A type at the readable size limit is within limits`
  - `COMMIT:` cc4dec39ecb2c1986a5c385004960bfdbc1fcd45

### Scenario: A type one line over the readable size limit is a finding

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A type one line over the readable size limit is a finding`
  - Evidence:
    ```
    1 scenario (1 failed), 1 step (1 failed)
    EXIT: 101
    ```
- [x] GREEN: generate 501 code lines with cohesive methods; classify as
  `cohesive-large-type`, action `advisory`, and record 501 code lines. Run;
  exit 0.
  - Production files: `tests/steps/readability_report_support.rs`,
    `tests/steps/readability_report_steps.rs`;
    `src/readability/metrics.rs` only if the boundary exposed a defect.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: assert the exact metric and classification through the public
  model, not rendered text; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): report types over the readable size limit`
  - Body: `A type one line over the readable size limit is a finding`
  - `COMMIT:` a0c6b9d01cee452c72010c4244c7ad74e8d31f59

### Scenario: A report is available in machine-readable form

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A report is available in machine-readable form`
  - Evidence:
    ```
    ✘ And a production source unit that is over the readable size
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: add `--json` schema v1 rendering from the same classified model;
  `stdout` is only the JSON document; diagnostics stay inside the document.
  Run; exit 0.
  - Production files: `src/readability/report.rs`, `src/cli/readability.rs`.
  - Design constraints: schema fields and deterministic ordering from
    `design.md` §JSON schema v1; `capabilities` carries support, never analyzer
    identity.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: share the unit ordering between human and JSON; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): add the JSON report`
  - Body: `A report is available in machine-readable form`
  - `COMMIT:` 681236cee031e94f259b4e6de0c91fcdaf36a115

### Scenario: A report is written to a file

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A report is written to a file`
  - Evidence:
    ```
    ✘ Then the report is written to "report.txt"
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: implement `--out <path>`: write the selected form, print nothing
  to stdout, overwrite the file. Run; exit 0.
  - Production files: `src/cli/readability.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: route both human and JSON through the same destination helper;
  re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): add --out`
  - Body: `A report is written to a file`
  - `COMMIT:` d06722f9a46cf213e7c49316ae20dfdc30b4e7d8

### Scenario: A report output path that cannot be written is an error

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A report output path that cannot be written is an error`
  - Evidence:
    ```
    ✘ And stderr names "missing/report.txt"
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: a missing parent or unwritable `--out` path exits 1 with stderr
  naming the path; do not fall back to stdout. Run; exit 0.
  - Production files: `src/cli/readability.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: use one `--out` error path; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): report --out write failures`
  - Body: `A report output path that cannot be written is an error`
  - `COMMIT:` 4c4fd8abc8b273a8c3e51e292eeef8e4050a78c6

### Scenario: An unsupported construct is named and not invented

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::An unsupported construct is named and not invented`
  - Evidence:
    ```
    ✘ And a production source file with an item-position macro
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: item-position macros produce `item-macros` diagnostics and
  wholly unsupported units are excluded from classification; the report names
  the construct and does not invent a unit. Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`, `model.rs`,
    `report.rs`, `classify.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep unsupported notes in the neutral model and out of
  classification; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): name unsupported constructs`
  - Body: `An unsupported construct is named and not invented`
  - `COMMIT:` 800817c95b6dfc4a061974a78440fc76c162d551

### Scenario: The JSON report carries the schema and rules versions

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The JSON report carries the schema and rules versions`
  - Evidence:
    ```
    ✘ And a production source unit
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: emit `schema_version: 1` and `rules_version: 1`; changing a
  threshold changes only `rules_version`. Run; exit 0.
  - Production files: `src/readability/report.rs`, `classify.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: centralize the two version constants; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): expose report versions`
  - Body: `The JSON report carries the schema and rules versions`
  - `COMMIT:` 21fcf8f71cdb4675a7bb8631daad4231c29707c5

### Scenario: The JSON report carries analyzer capabilities and limitations

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The JSON report carries analyzer capabilities and limitations`
  - Evidence:
    ```
    ✘ Then the JSON report names the supported constructs
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: render `capabilities.language`, `supported_constructs`,
  `limitations`, and `resolution_labels` from `AnalyzerCapabilities`; no
  analyzer identity. Run; exit 0.
  - Production files: `src/readability/analyzer.rs`, `report.rs`,
    `syn_adapter.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 4 steps (4 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep the capability values in one adapter-owned constant; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): expose analyzer capabilities`
  - Body: `The JSON report carries analyzer capabilities and limitations`
  - `COMMIT:` 6606822e62fce43e2e85e067611f2d926fe7376b

### Scenario: Conditional code is counted as written

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::Conditional code is counted as written`
  - Evidence:
    ```
    ✘ Given a production source unit with platform-conditional methods
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: keep the accepted limitation: both `#[cfg(unix)]` and
  `#[cfg(windows)]` methods contribute; no cfg diagnostic and no
  `complete: false` for this case. Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`; the step fixture.
  - Evidence:
    ```
    1 scenario (1 passed), 3 steps (3 passed)
    EXIT: 0
    ```
- [x] REFACTOR: add the fixture to the shared corpus; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): count conditional code as written`
  - Body: `Conditional code is counted as written`
  - `COMMIT:` 65813756df6eca1bab91b2000764e744abc598b0

### Scenario: The same source produces the same report through either analysis engine

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The same source produces the same report through either analysis engine`
  - Evidence:
    ```
    ✘ Given a production source unit supported by two analysis engines
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: add a fake `SourceAnalyzer` in the step support that builds the
  equivalent neutral `Analysis`; run metrics, classification, and both
  renderers; assert byte-identical output and no analyzer identity. Run; exit 0.
  - Production files: `src/readability/analyzer.rs`, `model.rs`, `mod.rs`;
    `tests/steps/readability_report_support.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 3 steps (3 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep the fake adapter in the step support, not in production;
  re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): prove engine-independent rendering`
  - Body: `The same source produces the same report through either analysis engine`
  - `COMMIT:` 93650f5617d0d9112d492637b8e258c42e3d0423

### Scenario: The report is deterministic

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The report is deterministic`
  - Evidence:
    ```
    ✘ When the readability report is produced twice
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: sort traversal and every array; use no timestamps, host paths, or
  floating-point values; two runs produce byte-identical human and JSON. Run;
  exit 0.
  - Production files: `src/readability/scan.rs`, `report.rs`, `classify.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 3 steps (3 passed)
    EXIT: 0
    ```
- [x] REFACTOR: centralize ordering; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): make the report deterministic`
  - Body: `The report is deterministic`
  - `COMMIT:` bcd889829688d027dccfe3a7a8dfb148a74e377a

### Scenario: Every analyzed unit has one classification and one action

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::Every analyzed unit has one classification and one action`
  - Evidence:
    ```
    ✘ Given the readability fixture corpus
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: make classification total over every analyzed unit; unsupported
  units are unclassified, every other unit has exactly one classification and
  one action. Assert over the fixture corpus and boundary fixtures. Run;
  exit 0.
  - Production files: `src/readability/classify.rs`, `mod.rs`; the step
    corpus helper.
  - Evidence:
    ```
    1 scenario (1 passed), 3 steps (3 passed)
    EXIT: 0
    ```
- [x] REFACTOR: encode the precedence as one match/table; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): guarantee one classification and action`
  - Body: `Every analyzed unit has one classification and one action`
  - `COMMIT:` 9a4e4a0a51c0df3e3fc4f6b4ea6c3dd641fdff42

### Scenario: A developer scopes the report to a workspace member

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A developer scopes the report to a workspace member`
  - Evidence:
    ```
    ✘ And a workspace member "crates/core" with a production source unit that is over the readable size
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: implement repeatable root-relative `--path`; scan
  `crates/core`; classify its unit and render project-relative paths. Reject
  absolute values and `..` at CLI parsing. Run; exit 0.
  - Production files: `src/cli/readability.rs`, `src/readability/scan.rs`,
    `analyzer.rs`, `mod.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: deduplicate overlapping roots before analysis; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): add the --path scope`
  - Body: `A developer scopes the report to a workspace member`
  - `COMMIT:` 289ba5c32b0e5fc65f540fe4f50409360dc73c0a

### Scenario: The report says when the scope has no production sources

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::The report says when the scope has no production sources`
  - Evidence:
    ```
    ✘ And no production source files
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: an existing empty default `src/` produces the documented message
  (human text) or the empty-scope JSON model and exits 0. Run; exit 0.
  - Production files: `src/readability/scan.rs`, `report.rs`,
    `src/cli/readability.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 5 steps (5 passed)
    EXIT: 0
    ```
- [x] REFACTOR: share the empty-scope branch between human and JSON; re-run;
  exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): report an empty scope`
  - Body: `The report says when the scope has no production sources`
  - `COMMIT:` af577d5ae89d1bcb99412f44864afb0e078f6ab1

### Scenario: An explicitly requested scope that does not exist is an error

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::An explicitly requested scope that does not exist is an error`
  - Evidence:
    ```
    ✘ Then the command exits with code 1
      left: Some(0)
      right: Some(1)
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: an explicit `--path` that does not exist exits 1 with stderr
  naming the path; a missing default `src/` remains the empty-scope exit 0.
  Run; exit 0.
  - Production files: `src/cli/readability.rs`, `src/readability/scan.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 4 steps (4 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep default-vs-explicit validation in one place; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): reject a missing explicit scope`
  - Body: `An explicitly requested scope that does not exist is an error`
  - `COMMIT:` e21756e482b01d017cb99ff99ab2d513feebc86c

### Scenario: A file that cannot be analyzed is reported and the report continues

- [x] RED: remove `@wip` from this scenario only; stub the steps; run;
  must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A file that cannot be analyzed is reported and the report continues`
  - Evidence:
    ```
    ✘ And a well-formed production source unit
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: parse errors become `parse-failed` diagnostics with the file and
  line; the file is skipped and the remaining units are still classified.
  Run; exit 0.
  - Production files: `src/readability/syn_adapter.rs`, `model.rs`,
    `report.rs`, `mod.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 6 steps (6 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep per-file failures as data, not `Err`; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `feat(readability-report): continue after a failed file`
  - Body: `A file that cannot be analyzed is reported and the report continues`
  - `COMMIT:` c02339d446a3f3837d96e61a74c424eb191d09fa

## E2E setup task

- [x] Configure and prove the E2E scope.
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
    $ GIVN_FEATURES=givn/changes/add-readability-report/specs ./run-tests.sh --non-e2e
    Summary [2.134s] 23 tests run: 23 passed, 2 skipped
    EXIT: 0
    ```
    ```
    $ GIVN_FEATURES=givn/changes/add-readability-report/specs ./run-tests.sh --e2e
    Summary [1.253s] 2 tests run: 0 passed, 2 failed, 23 skipped
    EXIT: 100
    ```
    The E2E scope runs 2 of the 25 delta scenarios, strictly fewer than the
    non-E2E scope's 23 (the two are skipped there by the tag filter).

## E2E scenarios

### Scenario: A developer runs a readability report and sees the classified units

- [x] RED: remove `@wip` from this scenario only; stub the E2E steps; run the
  targeted E2E command; must exit non-zero.
  - Target: `assess-code-structure/readability-report.feature::A developer runs a readability report and sees the classified units`
  - Evidence:
    ```
    ✘ And the report recommends a plan
      Step doesn't match any function
    1 scenario (1 failed), 5 steps (4 passed, 1 failed)
    EXIT: 101
    ```
- [x] GREEN: drive the real `givn` binary in a temp project with the addon
  enabled and an over-threshold type; assert stdout names the unit, the god
  type classification, the plan recommendation, and that no baseline file was
  written and the project tree is unchanged. Run; exit 0.
  - Production files: `tests/steps/readability_report_e2e_steps.rs`,
    `tests/steps/readability_report_support.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 6 steps (6 passed)
    EXIT: 0
    ```
- [x] REFACTOR: keep the fixture builder shared with the non-E2E support
  module; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `test(e2e): run the readable report end to end`
  - Body: `A developer runs a readability report and sees the classified units`
  - `COMMIT:` f2a3c4fad30126decedae65a3f3437f7bca9b6e7

### Scenario: Consumer enables the readability addon and it is listed as enabled

- [x] RED: remove `@wip` from this scenario only; stub the E2E steps; run the
  targeted E2E command; must exit non-zero.
  - Target: `initialize-project/enable-optional-addons.feature::Consumer enables the readability addon and it is listed as enabled`
  - Evidence:
    ```
    ✘ And the readability addon is disabled
      Step doesn't match any function
    1 scenario (1 failed)
    EXIT: 101
    ```
- [x] GREEN: `givn addons enable readability` writes `true`, prints
  `enabled addon 'readability' in givn/config.yaml`, and `givn addons list`
  renders `[enabled ]  readability`; `skills sync` runs as for every addon but
  generates nothing for this one. Run; exit 0.
  - Production files: `src/cli/addons.rs`, `src/config/manifest.rs`.
  - Evidence:
    ```
    1 scenario (1 passed), 7 steps (7 passed)
    EXIT: 0
    ```
- [x] REFACTOR: share the enable/disable config write; re-run; exit 0.
  - Evidence:
    ```
    REFACTOR EXIT: 0
    ```
- [x] COMMIT: `test(e2e): enable and list the readability addon`
  - Body: `Consumer enables the readability addon and it is listed as enabled`
  - `COMMIT:` fe2b9d31c721250d5fdda48945ad5b0e8a95214e

## Completion checklist

- [x] Every non-`@e2e` scenario has RED/GREEN/REFACTOR evidence and one
  `COMMIT:` hash naming the scenario in the message body.
- [x] Both `@e2e` scenarios have real-subprocess evidence and one commit each.
- [x] `givn lint --change add-readability-report` exits 0 or 2 (advisories
  allowed).
- [x] `./run-tests.sh --non-e2e` and `./run-tests.sh --e2e` exit 0 on the
  change delta (28 passed / 2 skipped, and 2 passed / 28 skipped); the
  permanent non-E2E suite passes 591/591 and the permanent E2E suite passes
  104/104 after `./setup-e2e-browser.sh` installed the pinned Playwright and
  Chromium.
- [x] The E2E setup scenario-count proof is filled and shows the E2E count is
  strictly less than the full count.
- [x] ADR-074 is accepted and the register counts are updated (done during the
  setup task: 26 accepted / 0 proposed; the count scenario expects 26).

## Review follow-up scenarios (missing coverage found during review)

These five scenarios were added after the first task list was complete. They
cover design rules that had no executable example; each followed the same
RED→GREEN→REFACTOR→COMMIT loop.

### Scenario: A type with many public methods is an interface

- [x] RED: the step was undefined; after adding the fixture the report
  returned `0 findings` because inherent method visibility was private.
- [x] GREEN: carry `syn::Visibility` into neutral methods; the 51-public-method
  type is an `interface` with a `trait-composition plan`.
- [x] COMMIT: `7790a1c52825325df0f50f1d875da698c67dffe1`
  (`feat(readability-report): count public methods in the interface rule`)

### Scenario: A large type in the inspect band needs an exception

- [x] RED: the fixture step was undefined.
- [x] GREEN: 3 methods / 2 fields with TCC 666 permille and LCOM4 1 classify as
  `cohesive-large-type` with action `exception`.
- [x] COMMIT: `a243b93e794c3e1c9bbc41c2be13149949d88a8e`
  (`feat(readability-report): classify the inspect band as an exception`)

### Scenario: A module over the plan limit needs an exception

- [x] RED: the fixture step was undefined.
- [x] GREEN: free-function spans now contribute to the module unit; a
  1,100+ code-line module is `module` with action `exception`.
- [x] COMMIT: `a8ad7e35a88eacc79092e49db270108bb89295b6`
  (`feat(readability-report): classify large modules as exceptions`)

### Scenario: A long method is a method-length finding

- [x] RED: the fixture step was undefined.
- [x] GREEN: a 112-code-line method under a within-size type is
  `method-length` with action `watch`.
- [x] COMMIT: `fd3aecd6ef94d81948e5361fe6c9d9712fa302e7`
  (`feat(readability-report): report long methods`)

### Scenario: Test sources are not scanned

- [x] RED: the fixture step was undefined.
- [x] GREEN: a large `tests/big.rs` fixture is absent from the report while
  `src/small.rs` is classified.
- [x] COMMIT: `e67e896b59fabd27f4eb86b7d4e50d86e759ae99`
  (`feat(readability-report): exclude test source trees`)
