# Design Review: workflow-enforcement-wip-cleanup

## Grilling

### Scope

**PASS.** The proposal asks only for removal of stale lifecycle metadata from
the 16 already implemented workflow-enforcement scenarios. The delta contains
exactly those 16 permanent titles and keeps their observable steps unchanged.
No production code, step binding, unrelated fixture text, or other capability
is included.

### Technical Choices

**PASS.** Reusing the existing `cucumber-rs` runner and existing
`workflow_enforcement_steps.rs` bindings is the smallest correct approach.
Using `@givn.modified` preserves the archive workflow and title-based merge
semantics. Directly editing the permanent spec would bypass the change
lifecycle and is rejected.

### Missing Scenarios

**PASS.** No new behaviour is introduced, so no new behaviour scenario is
needed. The design now adds explicit static verification for the durable result:
scenario-level `@wip` tags must be absent after archive, while fixture text
that mentions `@wip` must remain present.

### Testability

**PASS after hardening.** A meaningful RED phase does not exist for a
metadata-only migration: the bindings and observable behavior already work,
and removing a tag cannot make the behavior fail. The plan therefore uses
`givn lint --change` as the metadata RED-equivalent and runs each existing
scenario against the active delta with the strict Cucumber runner. The
single-scenario command sets `GIVN_FEATURES` to the delta and uses an anchored
`--name` filter without combining it with `--tags`.

The command was verified with the first scenario:

```text
GIVN_FEATURES=givn/changes/workflow-enforcement-wip-cleanup/specs cargo test --test features_runner -- --name '^Integrity hook passes when tasks\.md commit hashes are present$'
```

Result: 1 feature, 1 scenario, 6 steps, all passed.

During the implementation loop, each unfinished delta scenario temporarily
retains `@wip`; the tag is removed only for the scenario currently being
reverified. The final delta is lint-clean and contains no scenario-level
`@wip` tag.

### Risk

**PASS after hardening.** The most likely failure is false evidence from
running the permanent suite instead of the active delta. The design now sets
`GIVN_FEATURES` for every delta command and separately runs the permanent
suite after archive. Archive-side coverage and README updates are explicitly
treated as existing gate side effects and must be reviewed rather than hidden.

### E2E Fidelity

**PASS.** This change introduces no interaction and no new `@e2e` scenario.
The existing CLI scenarios use the real `givn` subprocess through existing
bindings. The existing `./run-tests.sh --e2e` command remains configured for
the repository's existing E2E scenarios.

### Architecture Documentation

**PASS for this change.** `addons.arc42` is enabled. The independent
12-chapter assessment agrees with `arc42.md`: all rows are No because this
change modifies only scenario lifecycle metadata and introduces no component,
runtime, deployment, interface, configuration, or quality change. No chapter
or ADR update is justified by this change.

The existing Arc42 corpus contains older flow-format documentation in runtime
chapters. Those blocks predate this change and are not modified by it; changing
them here would add unrelated architecture-documentation scope to a
metadata-only change. They remain a separate documentation concern.

### ADR Coverage

**PASS.** No new architecture decision or trade-off is introduced. Existing
runner and archive decisions are reused, so no new MADR entry or risk entry is
required.

## Hardening Applied

- Corrected the single-scenario command to set `GIVN_FEATURES` and use only an
  anchored `--name` filter.
- Replaced the impossible production-code RED requirement with lint and
  existing-scenario characterization evidence.
- Added explicit post-archive checks for scenario-level `@wip` removal and
  preservation of fixture literals.
- Documented archive verification and the existing coverage/README side
  effects.

## Decision

DESIGN-REVIEW: PASS
