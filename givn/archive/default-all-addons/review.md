# Review: default-all-addons

Review kind: self-review
Author: agent
Reviewer context: agent

README-IMPACT: none

PERSONAS: developer--primary-user

## Fabrication audit

- No empty step bodies in `tests/steps/default_all_addons_steps.rs`; the
  remaining steps are existing assertions.
- Commit `9a0115c9` carries the add-on default implementation, the
  `--no-addons` opt-out, the Arc42 overlay removal, the outline-gate skip, the
  changed specs, and the fixture updates. The task-evidence commit follows.
- Strict mode is inherited: `tests/features_runner.rs` calls
  `.fail_on_skipped()`.
- No `@e2e` tag was removed. The existing init and outline-addon E2E scenarios
  cover the real CLI. `verify.e2e_command` remains distinct from
  `verify.command`.
- Interaction coverage: the default add-on state is an invariant of the
  existing init action; the per-add-on opt-out uses the existing outline
  add-on action.

## Use-Case And Persona Conformance

`initialize-project` gains the all-enabled default and the explicit
`--no-addons` opt-out; `assess-code-structure` gains the outline report as a
default capability. Existing capabilities, guarantees, and typed
relationships are preserved. `developer--primary-user` and
`agent--technical-role` are the review lenses; the Gherkin actor remains the
developer.

## Visual review

The design declares `Interface: none`. No screenshot, transcript, or visual
verdict is required. The persona disposition is recorded above.

## Retirements

RETIRE: none.

## Arc42 implementation conformance

The 12-row assessment is complete. Chapters 02 (TC-45), 05 (`cli::init` and
`cli::addons`), 06 (init flow), 08 (8.11 and 8.26), 10 (Q-121), 11 (R-126), and
12 (add-on and add-on default) match the implementation.

ARC42 CONFORMANCE: CLEAN

## Coverage

The change has no archive receipt yet. Coverage is measured once by the archive
against the committed candidate. The new branches are the all-enabled default,
the explicit false writer, the partial-block serde default, and the Arc42
disable removal.

## Scenario execution evidence

`givn check tasks --change default-all-addons` recorded a current receipt:
6 scenarios, 0 failed, 0 skipped at `7688fa15`. The full non-E2E suite ran 728
tests with 728 passed; the E2E suite ran 112 tests with 112 passed.

**Receipt status:** proven for the change scope.

## Overlap dispositions

Every pair is a `boundary`: the init and add-on scenarios assert default
configuration state, not the permanent scenario's distinct behavior.

| Scenario A | Scenario B | Disposition |
|---|---|---|
| givn init enables every addon by default | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| The outline experiment is disabled by default | A project can explicitly disable the outline experiment | boundary |
| Canonical policy uses transcripts for terminal interfaces | givn init enables every addon by default | boundary |
| Canonical policy uses transcripts for terminal interfaces | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| Canonical policy requires a rendered-state assertion | a partial addons block keeps unnamed addons enabled | boundary |
| Canonical policy requires evidence for every declared UI state | a partial addons block keeps unnamed addons enabled | boundary |
| Canonical policy refuses a silent deferral of a mandatory check | a partial addons block keeps unnamed addons enabled | boundary |
| Design guidance keeps improvements inside the style | givn init enables every addon by default | boundary |
| Design guidance keeps improvements inside the style | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| Design guidance classifies a browser-rendered screen as a Web UI | a partial addons block keeps unnamed addons enabled | boundary |
| Design guidance names the default browser driver | a partial addons block keeps unnamed addons enabled | boundary |
| Design guidance requires a driver for every Web UI matrix row | givn init enables every addon by default | boundary |
| Design guidance requires a driver for every Web UI matrix row | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| Design guidance names the user-visible persona boundary | a partial addons block keeps unnamed addons enabled | boundary |
| Design-review guidance tests the opposite interface classification | a partial addons block keeps unnamed addons enabled | boundary |
| Design-review guidance records the persona disposition | a partial addons block keeps unnamed addons enabled | boundary |
| Design-review guidance requires an operator question before a deferral | a partial addons block keeps unnamed addons enabled | boundary |
| Proposal guidance requires confirmed personas or a reasoned none | a partial addons block keeps unnamed addons enabled | boundary |
| Review instructions require deterministic gate dispositions | a partial addons block keeps unnamed addons enabled | boundary |
| givn init lists default-off addons and how to enable them | givn init enables every addon by default | boundary |
| givn init lists default-off addons and how to enable them | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| Consumer enables the outline addon and it is listed as enabled | The outline experiment is enabled by default | boundary |
| Review guidance treats style improvements as contract updates | a partial addons block keeps unnamed addons enabled | boundary |
| Review guidance records an unverified verdict without vision | givn init enables every addon by default | boundary |
| Review guidance records an unverified verdict without vision | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| Review guidance tests the opposite interface classification | a partial addons block keeps unnamed addons enabled | boundary |
| Review guidance fails a deferral without an operator decision | a partial addons block keeps unnamed addons enabled | boundary |
| Review guidance records the persona disposition | a partial addons block keeps unnamed addons enabled | boundary |
| Review guidance names the state screenshots the vision review saw | givn init enables every addon by default | boundary |
| Review guidance names the state screenshots the vision review saw | givn init --no-addons writes an explicit opt-out for every addon | boundary |
| Review guidance requires a persona for a declared interface none contract | givn init enables every addon by default | boundary |
| Review guidance requires a persona for a declared interface none contract | givn init --no-addons writes an explicit opt-out for every addon | boundary |

## Split-or-keep

No delta scenario exceeds the long-scenario threshold.

| Scenario | Decision |
|---|---|

## Deferrals

None.

## Sign-off

- [x] Fabrication audit clean.
- [x] Every checked task has a verified commit touching production source or
      its guidance surface.
- [x] Every promised component exists.
- [x] Strict-mode proof present and passing.
- [x] Scenario execution evidence recorded per scope.
- [x] Coverage classified from inspection; no new gap.
- [x] No `@wip` tags remain in the delta specs.
- [x] The delta spec contains no implementation-layer detail.
- [x] Local run command unchanged; no external dependency.
- [x] `verify.e2e_command` isolation proved by the existing scope counts.
- [x] Implementation matches `design.md`.
- [x] No finding excused outside the three coverage buckets.

OUTLINE-COMMAND: givn outline report --path src
OUTLINE: AUTHOR agent
OUTLINE: REVIEWER agent
OUTLINE: PASS module:src/cli/addons.rs | owns the add-on registry, list, enable, disable, and the explicit all-disabled write; the registry default drives the list, then a mutation writes the config and syncs generated guidance
OUTLINE: PASS module:src/cli/init.rs | owns the init sequence; it writes config and commands, enables every add-on or writes explicit false values with disable_all, then syncs skills and attribution
OUTLINE: PASS module:src/config/manifest.rs | owns the manifest and add-on data model; Addons fields default to enabled and the manual Default agrees with the embedded manifest
OUTLINE: PASS module:src/config/overlay.rs | merges the project overlay onto the embedded manifest; the add-on block is applied, Arc42 is added or removed with its artifact and dependencies, then artifact patches follow
OUTLINE: PASS module:src/outline/gate.rs | evaluates the change-relative outline review; it resolves base and head, skips a change with no touched production path, analyzes the rest, and validates markers and identity
OUTLINE: PASS type:src/config/manifest.rs::Addons | names the ten add-on switches; serde fills a missing key from enabled_by_default and Default returns all true
OUTLINE: PASS type:src/config/manifest.rs::Coverage | names the coverage commands and paths; the command fields fall back to the sentinel and the paths to the fixed Cobertura defaults
OUTLINE: PASS type:src/config/manifest.rs::Manifest | names the artifact, apply, verify, run, coverage, add-on, and version fields; the embedded document is parsed first and the overlay is applied later
OUTLINE: PASS type:src/config/manifest.rs::Run | names the product start declaration; the command defaults to the sentinel and the readiness timeout has its default
OUTLINE: PASS type:src/config/manifest.rs::Verify | names the verify commands and features path; the command has no default and the e2e command and features path use their defaults
OUTLINE: PASS type:src/config/overlay.rs::ExtensionEntries | names the project instruction extensions; parsing drops unknown ids and the merge appends the marked section
OUTLINE: PASS type:src/config/overlay.rs::RulesPolicy | names the generated-rules policy; the default resolves to local-only when the value is absent
OUTLINE: PASS type:src/outline/gate.rs::GateFailure | names and orders one gate failure; the stable sort key is derived from the group, variant, path, unit, and rendered message
OUTLINE: PASS type:src/outline/gate.rs::GateOutcome | holds the gate result and renders it; pass and early construct the outcome and rendering walks the final line and sorted failures
OUTLINE: PASS type:src/outline/gate.rs::GateRequest | carries the change and outline scope into the gate; construction resolves the OUTLINE-COMMAND roots and root rendering exposes them
OUTLINE: PASS type:src/outline/gate.rs::Side | labels the base or head side of the comparison; the label is the only call
OUTLINE: PASS

REVIEW: PASS
