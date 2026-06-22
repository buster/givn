# Characterization tasks — reverse-engineer-lint-command

Descriptive specs for the `givn lint` CLI command. Each task below is one
RED/GREEN/REFACTOR cycle for a single scenario in
`specs/lint-command/lint-command.feature`, executed in feature order.

The shared RED/GREEN/REFACTOR loop mechanics live in the `givn-steps` skill.
Step definitions are shared glue in `tests/steps/mod.rs` — there is no
parallel hand-maintained test file.

## Setup

- [x] **Test infrastructure verification**
  - The Gherkin runner (`cargo test --test features_runner`) is already
    configured in `givn/config.yaml` (`verify.command`). The runner exits 0
    on the existing suite (`givn/specs/new-command`).
  - Verify the runner still exits 0 (or with only `@wip`-related failures
    once the new `lint-command` feature is included in the discovery glob)
    before characterization begins. The new feature lives under
    `givn/changes/reverse-engineer-lint-command/specs/`, which the runner
    does NOT discover by default (it reads `givn/specs/`); the scenarios
    will only run after `givn apply` copies them into `givn/specs/` OR the
    runner is pointed at the change specs. Confirm the discovery behaviour
    and, if needed, surface the new feature via `givn apply` / a temporary
    symlink — but do NOT change the runner's default glob.
  - GREEN: run `cargo test --test features_runner` → must exit 0 on the
    existing suite. The new lint-command scenarios will be RED until their
    step definitions are written in later tasks.

## Characterization cycles

- [x] **Scenario: No .feature files under givn/ reports clean and exits 0**
  - GREEN: write step definitions asserting the stdout message and exit 0.
    New Given step needed: `a .feature file at "<path>" with:` (docstring) —
    writes the content to that path under the temp root, creating parent
    dirs. This step is shared by all later scenarios.
  - REFACTOR: no seam needed. The lint command is exercised purely as a
    subprocess; no production code changes are required for any scenario in
    this feature.

- [x] **Scenario: All clean .feature files report a clean summary on stdout**
  - GREEN: reuse the file-writing Given step; assert stdout line and exit 0.
  - Seam: none.

- [x] **Scenario: A parse error is reported on stderr and exits 1**
  - GREEN: assert exit 1, stderr contains `[ERROR]`, the file name, and the
    summary line on stderr.
  - Seam: none.

- [x] **Scenario: A @wip-tagged scenario is reported on stderr and exits 2**
  - GREEN: add a `the command exits 2` Then step (the existing
    `the command exits with the argument-parsing exit code 2` step asserts
    code==2 but is named for arg-parsing; add a generic alias). Assert the
    `[WIP  ]` line and scenario-name message on stderr.
  - Seam: none.

- [x] **Scenario: A PENDING step is reported on stderr and exits 2**
  - GREEN: assert exit 2 and stderr contains `has a PENDING step`.
  - Seam: none.

- [x] **Scenario: A @wip finding outranks a parse error**
  - GREEN: assert exit 2 (NOT 1) when both a @wip finding and a parse error
    are present across files. Capture the AS-IS behaviour — this is the
    inverted-severity bug flagged in the Feature description. Assert both
    `[WIP  ]` and `[ERROR]` appear on stderr and the summary counts 2
    findings.
  - Seam: none.

- [x] **Scenario: Findings across multiple files are aggregated and files are processed in sorted order**
  - GREEN: write two feature files (zzz before aaa in insertion order) and
    assert the aaa finding line appears before the zzz finding line on
    stderr. Add a Then step for relative ordering of stderr lines, e.g.
    `the <needle-a> finding appears before <needle-b> on stderr`.
  - Seam: none.

- [x] **Scenario: --change scopes the check to that change's specs directory**
  - GREEN: place a @wip scenario under `givn/specs/` and a clean scenario
    under `givn/changes/<id>/specs/`; assert `givn lint --change <id>` exits
    0 and reports the clean summary. Reuse the file-writing Given step with
    a path under `givn/changes/<id>/specs/`.
  - Seam: none.

- [x] **Scenario: --change with no specs directory reports no files and exits 0**
  - GREEN: pre-create a change directory with no `specs/` (reuse the
    existing `an existing change named "<id>"` Given step); assert exit 0
    and the "no .feature files found under givn/changes/<id>/specs/" line.
  - Seam: none.

- [x] **Scenario: A non-existent change id is not surfaced**
  - GREEN: assert `givn lint --change does-not-exist` exits 0 and prints the
    "no .feature files found" message — capturing the AS-IS bug where a
    missing change is indistinguishable from a change with no specs.
  - Seam: none.

- [x] **Scenario: Explicit paths lint exactly those files without recursive descent**
  - GREEN: place a @wip scenario under `givn/specs/foo/` and a clean file at
    `givn/specs/target.feature`; assert `givn lint givn/specs/target.feature`
    exits 0 and reports only the target file as clean.
  - Seam: none.

- [x] **Scenario: A @givn.removed placeholder suppresses the @wip warning on itself**
  - GREEN: write a delta feature with a single `@givn.removed @wip` scenario;
    assert exit 0 and the clean summary (the @wip is suppressed because the
    scenario is a removed placeholder).
  - Seam: none.

- [x] **Scenario: A @givn.removed scenario suppresses an unrelated later @wip scenario**
  - GREEN: write a delta feature with a `@givn.removed` scenario followed by
    an unrelated `@wip` scenario later in the file; assert exit 0 and the
    clean summary. Capture the AS-IS bug — the over-broad line-based
    suppression check silences the unrelated @wip.
  - Seam: none.
