# Design: workflow-enforcement-wip-cleanup

## Technical Approach

This is a specification-lifecycle change, not a production-code change. The
existing workflow-enforcement scenarios already exercise the real `givn`
subprocess and their existing step definitions. The delta will repeat each
permanent scenario with the same title and steps, replace `@wip` with
`@givn.modified`, and let archive merge the completed versions over their
permanent targets.

No Rust module, public API, data format, or step-definition binding changes are
needed. The only durable result is that the merged scenarios no longer carry
the lifecycle tag `@wip`.

## Files and Merge Semantics

- Delta spec: `givn/changes/workflow-enforcement-wip-cleanup/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature`
- Permanent target: `givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature`
- Existing step definitions: `tests/steps/workflow_enforcement_steps.rs`
- Existing runner: `tests/features_runner.rs`

Each delta scenario uses `@givn.modified` and has the exact title of its
permanent target. During implementation, unfinished delta scenarios also carry
`@wip`; the tag is removed one scenario at a time as its existing behavior is
reverified. The final delta has no scenario-level `@wip` tag. During archive,
the modified scenario replaces its target and all `@givn.*` lifecycle tags are
stripped. The scenario text and any fixture-internal `@wip` strings remain
unchanged.

## Runner and Strict Mode

The repository uses the existing Rust `cucumber-rs` runner. The configured
verification command is:

```text
./run-tests.sh
```

This invokes `cargo test --test features_runner -- --tags 'not @e2e'`. During
givn hook execution, the hook sets `GIVN_FEATURES` to the active change's
feature directory, so the active delta is the test input before archive and
`givn/specs/` is the test input after archive. Strict mode is configured in
`tests/features_runner.rs` with `.fail_on_skipped()` on the Cucumber builder.
Undefined or skipped steps therefore fail the process. The Rust stub for any
temporarily unimplemented step would be `unimplemented!()` or `todo!()`, but
this change reuses existing implemented bindings and needs no stub.

The single-scenario command used for evidence is:

```text
GIVN_FEATURES=givn/changes/workflow-enforcement-wip-cleanup/specs \
cargo test --test features_runner -- \
  --name '^<scenario title>$'
```

The exact anchored scenario filter is passed to the Cucumber runner. `--name`
is used alone because the Cucumber CLI does not combine name and tag filters.
The active-delta suite command used for the complete metadata check is:

```text
GIVN_FEATURES=givn/changes/workflow-enforcement-wip-cleanup/specs ./run-tests.sh
```

There is no meaningful RED phase for this change: the existing bindings and
observable scenarios are already implemented, and removing a lifecycle tag
cannot make the existing behavior fail. The replacement for RED is
`givn lint --change workflow-enforcement-wip-cleanup`, followed by the
single-scenario GREEN characterization command. Each scenario still gets its
own refactoring/reverification and atomic commit.

The configured E2E command remains:

```text
./run-tests.sh --e2e
```

It invokes the same strict runner with `--tags @e2e`. This change introduces no
new `@e2e` scenario and therefore does not change the E2E interaction surface.
The existing E2E command remains a strict subset of the full suite.

## Step Definitions and Local Runnability

The capability is CLI-only. Existing steps launch the real `givn` executable
in temporary project roots and assert on its exit status and stdout/stderr.
No browser, HTTP server, database, queue, or third-party service is involved.
The local run command is `./run-tests.sh`; it starts no external dependency.
There are no digital twins because there are no external dependencies.

No interface obstacle is introduced. The real interface remains the `givn`
subprocess, not a direct library call or an in-process substitute.

## Interaction Coverage Matrix

The spec's inventory explicitly declares that this change introduces no new
user interaction. It only changes completion metadata on existing scenarios.
Consequently there are no new inventory entries and no new `@e2e` rows to
map. Existing workflow-enforcement CLI interactions retain their current
scenario coverage and step definitions.

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| None: metadata-only change | None | None | None |

## Archive and Coverage Effects

`givn archive` is itself part of verification. Its required hooks run the
configured full-suite, E2E, integrity, and coverage gates before moving the
change. Because coverage is enabled, archive may refresh the managed README
coverage badge/summary and coverage reports even though production code is
unchanged. Those generated changes are expected only when the existing
coverage transaction produces them; no source, step-definition, or unrelated
spec changes are accepted.

After archive, verify the durable metadata result with:

```text
! rg -n '^[[:space:]]+@wip[[:space:]]*$' \
  givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature
rg -n 'contains "@wip"|contains @wip' \
  givn/specs/givn-workflow-enforcement/givn-workflow-enforcement.feature
```

The first command proves that no scenario-level `@wip` tag remains. The
second proves that fixture text mentioning `@wip` was not accidentally
rewritten. Also compare the 16 scenario titles and observable steps in the
delta and archived permanent result; only lifecycle tags may differ.

## Architecture and Data Impact

No architecture component changes, data-model changes, or dependency changes
are required. The existing coverage scripts continue to exercise the same
runner and production binary; archive invokes that existing coverage process
as a gate, but this change does not modify it.

## Verification

Before archive, run:

1. `givn lint --change workflow-enforcement-wip-cleanup`
2. The single-scenario Cucumber command for every modified scenario against the active delta.
3. `GIVN_FEATURES=givn/changes/workflow-enforcement-wip-cleanup/specs ./run-tests.sh` for the active-delta suite.
4. `./run-tests.sh` for the full permanent non-E2E suite.
5. `./run-tests.sh --e2e` for the existing E2E subset.

After archive, run `givn archive --change workflow-enforcement-wip-cleanup`,
rerun both suite commands against the merged permanent specs, and execute the
static metadata checks above. Verify that the 16 target scenarios have no
`@wip` tag while fixture text that contains `@wip` is unchanged. Record any
expected coverage/README refresh in review.md.
