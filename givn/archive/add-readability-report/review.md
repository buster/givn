# Review: add-readability-report

Status: coverage review complete. No fabrication findings. Coverage classified.
`README-IMPACT: updated - What it is

REVIEW: PASS`.

## Fabrication audit

### @e2e tag integrity

Two delta scenarios remain tagged `@e2e`; neither tag was removed:

- `assess-code-structure/readability-report.feature::A developer runs a readability report and sees the classified units`
- `initialize-project/enable-optional-addons.feature::Consumer enables the readability addon and it is listed as enabled`

### Empty step bodies

`0 empty step bodies found across 3 step files`
(`readability_report_steps.rs`, `readability_report_e2e_steps.rs`,
`readability_report_support.rs`; the support file contains no step bindings).
Every step body contains a real assertion or a real fixture action.

### Commit coverage

Every scenario has a real commit reachable from `HEAD`. Reuse scenarios add
their regression test in the scenario commit; the production change they
exercise was delivered by the earlier commit named here.

| Scenario | Commit | Content |
|---|---|---|
| init enables only default-on addons | `49737abb` | production: `AddonSpec`, init filter, manifest field, template |
| init lists default-off addons | `e0150ffa` | production: init print |
| experiment disabled by default | `40d1bb0a` | regression test for the addon registry |
| report unavailable when disabled | `a0afcdba` | production: CLI command and runtime gate |
| data-only type | `9e99f915` | production: neutral model, scanner, metrics, classifier, renderer, syn adapter |
| interface (trait) | `ecc00b20` | regression test for the trait rule delivered in `9e99f915` |
| cohesive type | `d71c6df3` | production: impl method spans |
| 500-line boundary | `cc4dec39` | regression test for the boundary |
| 501-line boundary | `a0c6b9d0` | production: full impl span |
| JSON report | `681236ce` | regression test for the renderer delivered in `9e99f915` |
| `--out` success | `d06722f9` | regression test for the CLI destination |
| `--out` failure | `4c4fd8ab` | regression test for the CLI error path |
| unsupported construct | `800817c9` | production: `ItemMacro` construct and diagnostics |
| schema/rules versions | `21fcf8f7` | regression test for the version constants |
| capabilities/limitations | `6606822e` | regression test for the existing capability renderer |
| conditional code | `65813756` | regression test for the accepted limitation |
| engine contract | `93650f56` | regression test with the step-support fake adapter |
| determinism | `bcd88982` | regression test over the sorted pipeline |
| one classification/action | `9a4e4a0a` | regression test over the fixture corpus |
| `--path` scope | `289ba5c3` | regression test for the scope path |
| empty scope | `af577d5a` | production: empty-scope message |
| missing explicit scope | `e21756e4` | production: explicit-path validation |
| malformed file | `c02339d4` | regression test for per-file diagnostics |
| E2E primary report | `f2a3c4fa` | real-subprocess assertions |
| E2E addon enable | `fe2b9d31` | real-subprocess assertions; step-keyword fix `30a9d3b4` |
| public-method interface | `7790a1c5` | production: inherent method visibility |
| inspect-band exception | `a243b93e` | regression test for the exception band |
| large module exception | `a8ad7e35` | production: free-function spans in the module |
| method-length finding | `fd3aecd6` | regression test for the method rule |
| test sources excluded | `e67e896b` | regression test for the scanner exclusion |

`f13c8897` removes an unused adapter parameter after the follow-up fixes.

### Design component check

All components named by `design.md` exist: `src/readability/{mod,model,analyzer,scan,metrics,classify,report,syn_adapter}.rs`
and `src/cli/readability.rs`. `src/lib.rs` exports `pub mod readability`; the
CLI subcommand is registered in `src/main.rs`.

### Strict-mode proof

The setup task records the probe feature with an undefined step exiting 100
(`1 scenario (1 failed)`) and the restored permanent baseline passing 591/591.
`features_runner` uses `fail_on_skipped()` and exits non-zero on
`skipped > 0`.

### E2E assertions and driver

Both `@e2e` Then steps assert CLI stdout (`god-type`, `PLAN`,
`decomposition plan`, `enabled addon 'readability'`,
`[enabled ]  readability`) and the primary scenario asserts no baseline file
and an unchanged project tree. The driver is `common::run_givn` on the built
`CARGO_BIN_EXE_givn` subprocess; no repository-only assertion is used.

### E2E scope

`verify.command` is `./run-tests.sh --non-e2e`; `verify.e2e_command` is
`./run-tests.sh --e2e`. They are different strings, and the delta proof is
28 non-E2E passed / 2 skipped versus 2 E2E passed / 28 skipped. The E2E scope
is a strict subset. The runner tag expressions also carry
`and not @givn.removed`, so the removal record in the addon delta is a merge
instruction in every scope and never an execution unit; this also fixes the
plain `cargo test` path, whose CLI tag filter otherwise bypassed the custom
removed-scenario predicate.

### Design conformity

The implementation follows the reviewed design: `--path` is repeatable and
root-relative, explicit missing roots exit 1, default missing `src/` is an
empty report, `--out` overwrites, JSON carries `capabilities` and both
versions, `syn` is confined to `syn_adapter.rs` and `src/cli/readability.rs`
imports none. One deliberate design-review resolution changed the test
approach (non-E2E Gherkin calling the public pipeline in process instead of
`tests/analyzer_contract.rs`); the change-level design and ADR-074 record it.

### Interaction coverage cross-reference

| Use case | Inventory entry | Design matrix row | `@e2e` scenario | Driving mechanism |
|---|---|---|---|---|
| `assess-code-structure` | run a readability report | present | A developer runs a readability report and sees the classified units | real `givn` subprocess via `common::run_givn` |
| `initialize-project` | enable the readability addon | present | Consumer enables the readability addon and it is listed as enabled | real `givn` subprocess via `common::run_givn` |

No unmapped interaction exists and no extra `@e2e` scenario was produced.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| Opt-in structural visibility goal | CH1 | affected | D1 | addon scenarios | default-off addon registry, init filter | yes |
| Default-off, advisory, deterministic constraints | CH2 TC-38 | affected | D2, DD6, DD7, DD12 | addon/gate/determinism tasks | registry, CLI gate, report constants | yes |
| Developer CLI and later gate context | CH3 | affected | D5 | all CLI scenarios | `givn readability report` | yes |
| Neutral port and `syn` adapter strategy | CH4 | affected | D3, DD3, DD8 | data-only through engine tasks | `model.rs`, `analyzer.rs`, `syn_adapter.rs` | yes |
| Building blocks `cli::readability` and `readability::*` | CH5 | affected | architecture tree | every scenario | files present; clippy clean | yes |
| Report run sequence | CH6 | affected | CLI contract | CLI scenarios | `run_report` -> analyze -> assess -> render | yes |
| Addon default-on rule and report contract | CH8.11/8.14 | affected | DD1, DD6 | addon, JSON, `--out` tasks | `AddonSpec`, renderers | yes |
| ADR-074 neutral analyzer port | CH9 | affected | DD11 | engine contract task | ADR accepted; port + fake adapter test | yes |
| Determinism, offline, advisory quality | CH10 | affected | DD6, exit table | determinism/gate tasks | sorted traversal, no network, exit 0 | yes |
| Conditional-compilation limitation | CH11 R-97 | affected | DD12 | conditional-code task | both cfg branches counted | yes |
| No stored state / deferred ledger | CH11 D-24, D-25 | affected | D2, D4 | gate/E2E tasks | no state file; manual record only | yes |
| Report vocabulary | CH12 | affected | data model | all scenarios | glossary terms used in specs and code | yes |

`ARC42 CONFORMANCE: CLEAN`

## Ubiquitous language conformance

`docs/arc42/12-glossary.md` holds the report, source unit, classification,
action, LCOM4, TCC/LCC, and production-source terms. The delta specs, design,
and code use those terms and labels consistently (`god-type`, `interface`,
`multi-type-module`, `module`, `data-only`, `cohesive-large-type`,
`method-length`, `within-limits`; `watch`, `plan`, `exception`, `advisory`,
`none`). No new unrecorded term was introduced. No finding.

## Coverage classification

1. **Dead code (YAGNI/KISS) -> delete.** None. The `#[allow(dead_code)]`
   markers cover neutral-model contract variants that the later tree-sitter
   adapter and review gate consume; ADR-074 fixes that surface, so they are
   not deleted.
2. **Missing test coverage -> add scenario.** Five design rules had no
   executable example and were added during review: public-method interface,
   inspect-band exception, large-module exception, method-length finding, and
   test-source exclusion (commits `7790a1c5`, `a243b93e`, `a8ad7e35`,
   `fd3aecd6`, `e67e896b`).
3. **Legitimately hard to test (rare) -> technical justification.** None.

The delta suite is 28 non-E2E passed / 2 skipped and 2 E2E passed / 28
skipped. The permanent non-E2E suite passes 591/591 and the permanent E2E
suite passes 104/104 after `./setup-e2e-browser.sh` installed the pinned
Playwright and Chromium.

## E2E coverage

The primary action is covered by the real CLI scenario and the addon action by
its real CLI scenario. Non-E2E scenarios are variants and error cases
(classifications, JSON, `--out`, scopes, malformed files, pipeline levels), not
duplicate user journeys. The output is plain UTF-8 text with no DOM, HTTP route,
or browser driver, so it is not a Web UI; the opposite classification does not
hold.

## Use-case and Persona conformance

The use case keeps one goal, the Developer as primary actor, the Authoring
agent as supporting actor, the five rules, and the two extensions. No capability
is absorbed from another use case; `enable-optional-addons` is a modified
existing capability and `readability-report` is new under
`assess-code-structure`.

| Persona | Disposition | Evidence |
|---|---|---|
| `developer--primary-user` | satisfies the goal | reads the stdout report; classification and recommendation asserted |
| `agent--technical-role` | consumes the stable JSON | schema v1, versions, capabilities, deterministic ordering |
| `givn-maintainer--owner` | keeps the experiment opt-in | default-off registry, no gate effect, no stored state |

Personas are review lenses, not Gherkin actors.

## Visual review

The design's `## Visual Design Contract` declares `Interface: none`; the
capability renders no screen, HTML, CSS, JavaScript, or interactive terminal
UI. The conditional visual block does not apply. `VISUAL-REVIEW: NOT
APPLICABLE`.

## Deferrals

None. The later `add-readability-review-gate` is a separate change, not a
deferral of a mandatory check in this one. The experiment exit rule and the
deferred ledger are owner decisions recorded in D4/OD1 and arc42 D-25.

README-IMPACT: updated - What it is

REVIEW: PASS
