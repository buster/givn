# Review: workflow-enforcement-wip-cleanup

## Scope and Result

The change activates the 16 existing workflow-enforcement scenarios by removing
only their scenario-level `@wip` lifecycle tags through `@givn.modified`
deltas. Scenario titles, observable steps, bindings, fixture text, and
production code are unchanged.

## Fabrication Audit

### E2E Tag Integrity

- The delta contains no `@e2e` scenarios.
- No existing `@e2e` tag was removed. The repository's existing permanent E2E
  scenarios remain covered by `./run-tests.sh --e2e`.

### Step Definition Audit

- Existing capability file scanned: `tests/steps/workflow_enforcement_steps.rs`.
- No empty bodies, bare `pass`, bare `return`, or unimplemented step bodies
  were introduced.
- The change adds no step-definition file because all 16 scenarios reuse
  existing bindings, as required by design.md.

### Task Commit Audit

Every scenario task has a recorded implementation commit. The 16 activation
commits are:

| # | Scenario commit |
|---:|---|
| 1 | `36dccd7` |
| 2 | `45e3ec0` |
| 3 | `02737df` |
| 4 | `6c2e450` |
| 5 | `5e6b8a3` |
| 6 | `5348c61` |
| 7 | `11556c7` |
| 8 | `2eb6888` |
| 9 | `cdabeeb` |
| 10 | `aa89a3d` |
| 11 | `f7e3dbc` |
| 12 | `4816e01` |
| 13 | `1e265be` |
| 14 | `ad19322` |
| 15 | `8ce289f` |
| 16 | `17e3d71` |

Each activation commit changes the delta feature and the corresponding task
evidence. No commit changes production source because this change deliberately
has no production implementation; the behavior and bindings predate the
change. This is an intentional metadata-migration exception, not an empty-step
or fabricated-GREEN finding.

The setup proof is present in tasks.md and recorded a non-zero result:

```text
GIVN_FEATURES=tests/strict-mode-probe.feature cargo test --test features_runner -- --name '^Undefined steps fail the runner$'
1 feature
1 scenario (1 failed)
1 step (1 failed)
error: test failed, to rerun `--test features_runner`
```

The temporary probe file was removed after the proof. Strict mode is provided
by `.fail_on_skipped()` in `tests/features_runner.rs`.

### Design Conformance

- Runner: `cargo test --test features_runner`, as designed.
- Active-delta path: `GIVN_FEATURES=.../workflow-enforcement-wip-cleanup/specs`,
  as designed.
- Step definitions: existing `tests/steps/workflow_enforcement_steps.rs`, as
  designed.
- E2E command: `./run-tests.sh --e2e`, distinct from `./run-tests.sh`, as
  designed.
- No browser, HTTP, database, queue, or digital twin is involved.

### Interaction Coverage

The feature inventory contains one declarative statement that no new user
interaction is introduced. It is a metadata-only lifecycle migration, not an
unmapped interface interaction.

| Inventory entry | Matrix row | Matching `@e2e` scenario | Driving mechanism |
|---|---|---|---|
| No new user interaction; existing workflow interactions retain their behavior | No new interaction row | None required | Existing real `givn` subprocess scenarios remain unchanged |

No interaction coverage is missing or over-produced.

## Verification Evidence

### Active Delta

```text
givn lint --change workflow-enforcement-wip-cleanup
givn lint: 1 file(s) checked — clean

GIVN_FEATURES=givn/changes/workflow-enforcement-wip-cleanup/specs ./run-tests.sh
[Summary]
1 feature
16 scenarios (16 passed)
105 steps (105 passed)
```

All 16 scenarios were also run individually with the anchored `--name` filter
from design.md. Every individual run passed in both the GREEN and REFACTOR
verification.

### Permanent Non-E2E Suite

```text
./run-tests.sh
[Summary]
20 features
163 scenarios (163 passed)
926 steps (926 passed)
```

### Existing E2E Suite

```text
./run-tests.sh --e2e
[Summary]
15 features
22 scenarios (22 passed)
127 steps (127 passed)
```

The E2E suite is strictly smaller than the 163-scenario non-E2E suite and is
therefore a real filtered subset.

### Coverage Measurement

The configured coverage commands were run:

```text
./measure-coverage.sh
./merge-coverages.sh
```

Both exited 0. The merged report is freshly generated at
`coverage/cobertura-coverage.xml` and reports:

```text
lines-covered=5144 lines-valid=8140 line-rate=0.6319410319410319
branches-covered=0 branches-valid=0 branch-rate=0.0
```

The known production path `givn::gherkin::merge::merge_file` is present with
non-zero line coverage. The runner and the `givn` production binary are
instrumented by the repository's existing coverage scripts; non-E2E and E2E
reports are produced separately and merged. No production lines were added by
this change, so there is no new uncovered production gap to classify. Existing
uncovered baseline lines are neither dead code introduced here nor missing
coverage caused by this change.

## Arc42 Implementation Conformance

`addons.arc42` is enabled. The completed design and implementation introduce
no architecture fact, and the change-level marker independently marks all 12
chapters as unaffected.

| Chapter | Change marker | Design fact | Implementation evidence | Result |
|---|---|---|---|---|
| 01 Introduction and Goals | No | No goal or stakeholder change | Spec metadata only | Clean |
| 02 Architecture Constraints | No | No constraint change | No production/config change | Clean |
| 03 Context and Scope | No | No interface change | Existing CLI unchanged | Clean |
| 04 Solution Strategy | No | Existing archive strategy reused | `@givn.modified` uses existing merge | Clean |
| 05 Building-Block View | No | No component change | No source/module change | Clean |
| 06 Runtime View | No | Existing flows unchanged | Existing runner/bindings reused | Clean |
| 07 Deployment View | No | No deployment change | No environment change | Clean |
| 08 Crosscutting Concepts | No | No cross-cutting behavior change | Only lifecycle tags change | Clean |
| 09 Architecture Decisions | No | No new decision | No ADR added | Clean |
| 10 Quality Requirements | No | No new quality scenario | Existing scenarios reclassified | Clean |
| 11 Risks and Technical Debt | No | No new risk/debt | Stale metadata removed | Clean |
| 12 Glossary | No | No new domain term | No terminology change | Clean |

ARC42 CONFORMANCE: CLEAN

## Overlap dispositions

The overlap gate reports identical observable shapes, not identical behavior.
Every reported pair is retained as a boundary because the scenarios exercise
different workflow outcomes, inputs, or failure semantics. No pair is merged
or removed by this metadata-only change.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| `A modified delta matching its merge target in the same capability is allowed` | `Integrity hook passes on edited arc42 chapter` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `A modified delta matching its merge target in the same capability is allowed` | `Integrity hook passes when arc42 is disabled` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `A removed delta referencing its target by title is allowed` | `Integrity hook passes on edited arc42 chapter` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `A removed delta referencing its target by title is allowed` | `Integrity hook passes when arc42 is disabled` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive allowed when @wip only on a @givn.removed placeholder` | `Archive succeeds when merged permanent specs have no @e2e scenarios and e2e_command is unconfigured` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive allowed when @wip only on a @givn.removed placeholder` | `Uppercase checkboxes satisfy the archive gate` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Check tasks blocked when delta spec has @wip` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Detail mode — unknown change errors` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Id beginning with a hyphen passed after the argument separator is rejected by validation` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Id ending with a hyphen is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Id with a dot is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Id with non-ASCII letters is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Id with spaces is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Path traversal id is rejected by archive` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Path traversal id is rejected by check` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Path traversal id is rejected by graph` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Path traversal id is rejected by instructions` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Path traversal id is rejected by lint` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Path traversal id is rejected by status` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Review blocks with a clear message when review.md is missing` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `Uninitialized project errors` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `givn addons disable errors when givn/ is not initialised` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Archive blocked when delta spec has non-removed @wip scenario` | `givn addons enable errors when givn/ is not initialised` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Detail mode — unknown change errors` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Id beginning with a hyphen passed after the argument separator is rejected by validation` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Id ending with a hyphen is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Id with a dot is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Id with non-ASCII letters is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Id with spaces is rejected` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Path traversal id is rejected by archive` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Path traversal id is rejected by check` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Path traversal id is rejected by graph` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Path traversal id is rejected by instructions` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Path traversal id is rejected by lint` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Path traversal id is rejected by status` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Review blocks with a clear message when review.md is missing` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `Uninitialized project errors` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `givn addons disable errors when givn/ is not initialised` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Check tasks blocked when delta spec has @wip` | `givn addons enable errors when givn/ is not initialised` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Detail mode — unknown change errors` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Id beginning with a hyphen passed after the argument separator is rejected by validation` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Id ending with a hyphen is rejected` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Id with a dot is rejected` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Id with non-ASCII letters is rejected` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Id with spaces is rejected` | `Integrity hook fails on Step Definitions section that is not a table` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Path traversal id is rejected by archive` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Path traversal id is rejected by check` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Path traversal id is rejected by graph` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Path traversal id is rejected by instructions` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Path traversal id is rejected by lint` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Path traversal id is rejected by status` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Review blocks with a clear message when review.md is missing` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `Uninitialized project errors` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `givn addons disable errors when givn/ is not initialised` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on Step Definitions section that is not a table` | `givn addons enable errors when givn/ is not initialised` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on empty step-definition file` | `Integrity hook fails on missing step-definition file` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on empty step-definition file` | `Integrity hook fails on placeholder commit hash` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on empty step-definition file` | `Integrity hook fails on unedited arc42 chapter` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on empty step-definition file` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on empty step-definition file` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on missing step-definition file` | `Integrity hook fails on placeholder commit hash` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on missing step-definition file` | `Integrity hook fails on unedited arc42 chapter` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on missing step-definition file` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on missing step-definition file` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on placeholder commit hash` | `Integrity hook fails on unedited arc42 chapter` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on placeholder commit hash` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on placeholder commit hash` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on unedited arc42 chapter` | `Review blocks when a delta scenario shape-matches a permanent scenario without disposition` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook fails on unedited arc42 chapter` | `Review blocks when a long scenario in the delta has no split-or-keep entry` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `Integrity hook passes when arc42 is disabled` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `Review passes when a long scenario has a split-or-keep entry` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `Review passes when a shape match exists only between permanent scenarios` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `The default scope excludes the archive tree` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `Valid kebab-case change id is accepted by every command that accepts --change` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `givn addons disable when already disabled is idempotent` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes on edited arc42 chapter` | `givn check with known artifact proceeds to run hooks` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `Review passes when a long scenario has a split-or-keep entry` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `Review passes when a shape match exists only between permanent scenarios` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `The default scope excludes the archive tree` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `Valid kebab-case change id is accepted by every command that accepts --change` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `Verify-e2e hook resolves the change directory from a relative features path inside givn/changes` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `givn addons disable when already disabled is idempotent` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when arc42 is disabled` | `givn check with known artifact proceeds to run hooks` | boundary: distinct workflow or lifecycle outcome; retain both. |
| `Integrity hook passes when tasks.md commit hashes are present` | `Integrity hook skips arc42 check when slug cannot be resolved` | boundary: distinct workflow or lifecycle outcome; retain both. |

## Final Checks

- [x] Proposal, specs, design, arc42 assessment, design-review, tasks, and
      review artifacts are present.
- [x] Strict mode is proven with a non-zero undefined-step run.
- [x] All 16 scenario tasks are checked and have commit hashes.
- [x] Active delta lint and suite pass.
- [x] Permanent non-E2E and existing E2E suites pass.
- [x] Coverage measurement and merge pass with a non-zero production path.
- [x] No scenario-level `@wip` tag remains in the active delta; literal
      fixture references to `@wip` remain.

REVIEW: PASS
