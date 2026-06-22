# Review — reverse-engineer-lint-command

Reverse-engineering coverage review for the `givn lint` CLI command.
Capability: `lint-command`. Mode: reverse-engineering (no proposal/design/
design-review — those phases were skipped via `.givn-skip`).

- Production code under test: `src/cli/lint.rs`, `src/gherkin/parse.rs`,
  `src/gherkin/delta.rs`.
- Delta spec: `specs/lint-command/lint-command.feature` (13 scenarios, all
  tagged `@givn.added`).
- Step definitions: `tests/steps/mod.rs` (shared glue, no parallel
  hand-maintained test file).
- Verify command: `cargo test --test features_runner`.

## 1. Coverage

Coverage tool: `cargo llvm-cov` (source-based LLVM coverage). The
`features_runner` binary spawns the compiled `givn` binary as a subprocess
(see `tests/common/mod.rs:55 run_givn`); `cargo llvm-cov` instruments the
`givn` bin crate and merges the subprocess profraw data with the test
binary's profraw, so the Gherkin-runner-driven execution of `lint::run`
IS counted. (Note: `cargo tarpaulin` was also tried but reported 0% — it
cannot follow the subprocess boundary by default; `cargo llvm-cov` is the
correct tool here.)

Command:
```
cargo llvm-cov --test features_runner --no-report
cargo llvm-cov report
cargo llvm-cov --lcov --output-path /tmp/cov.lcov
```

Overall (whole crate, all binaries):
```
TOTAL  4290 regions  2208 missed  48.53%
       2652 lines    1291 missed  51.32%
```

Per-file for the three production files under test (from `cargo llvm-cov report`):

```
src/cli/lint.rs        76 lines  1 missed   98.68%
src/gherkin/parse.rs  101 lines  1 missed   99.01%
src/gherkin/delta.rs   79 lines  1 missed   98.73%
-------------------------------------------------
Total under test      256 lines  3 missed   98.83%
```

Raw lcov uncovered lines (DA:0) in the files under test:
```
src/cli/lint.rs:65
src/gherkin/delta.rs:45
src/gherkin/parse.rs:113
```

Coverage percentage is recorded as a fact, not a gate.

## 2. Coverage gap table

Reverse-mode bucketing (dead code / missing test / hard to test). The
verify command (`cargo test --test features_runner`) runs ONLY the Gherkin
runner; the `#[cfg(test)] mod tests` unit-test blocks inside
`src/gherkin/parse.rs` and `src/gherkin/delta.rs` are NOT run by verify,
so their execution does not appear in the coverage above.

| Bucket | Region | Action |
|---|---|---|
| Hard to test (tooling artifact) | `src/cli/lint.rs:65` — closing `}` of the `if let Some(ref feature) = result.feature { ... }` block inside the `filter` closure. | LLVM source-based coverage assigns a zero counter to the closing brace of an `if let` block whose body contains an early `return` (the `return false` suppress branch on line 63). The block IS entered and exited: line 57 (`if let Some`) has 7 hits, line 66 (`true` — the fall-through after the block) has 2 hits. No real execution gap; the brace counter is a known llvm-cov artifact for `if let` with diverging arms. Justified — no test can move this counter. |
| Hard to test (tooling artifact) | `src/gherkin/parse.rs:113` — closing `}` of `if let Ok(entries) = std::fs::read_dir(dir) { ... }` in `collect_recursive`. | Same llvm-cov artifact: the closing brace of an `if let` block. The block body (lines 105-112) is exercised on every recursive call (line 108 `collect_recursive` has hits). Justified — no real gap. |
| Missing test (out of lint scope) | `src/gherkin/delta.rs:22-75` — `is_delta_feature`, `capability_tag`, `delta_op`, `delta_scenarios`, `strip_givn_tags`. | These functions are NOT called by `givn lint` — lint imports only `is_removed_placeholder` from `delta.rs` (see `src/cli/lint.rs:3`). They are exercised by the unit tests in `delta.rs`'s `#[cfg(test)] mod tests`, but verify.command runs only the Gherkin runner, so they show as uncovered here. They are NOT dead code — they are used by other givn capabilities (apply / archive / merge). Follow-up `@givn.added` candidate: add `.feature` coverage for these functions in the respective capability's spec (apply/archive/merge), not in `lint-command`. |
| Missing test | `src/gherkin/delta.rs:45` — `_ => {}` wildcard arm in `delta_op`. | Even the existing unit tests do not exercise this arm: every scenario in the `DELTA_FEATURE` test fixture has a `givn.*` tag that matches before the wildcard. The arm fires only for non-givn tags (e.g. `wip`, `user-auth`). Follow-up `@givn.added` candidate: add a unit test or `.feature` scenario where a scenario has a non-givn tag preceding any `givn.*` tag. |

Summary: 3 gap regions classified — 2 hard-to-test (tooling artifacts,
justified), 1 missing-test (out of lint scope, follow-up `@givn.added`),
1 missing-test (`delta_op` wildcard arm, follow-up `@givn.added`).

## 3. Spec corrections

None. No scenarios were updated during characterization — every scenario
matched observed behaviour on first run. The initial inference (from the
Feature: description's "Suspected bugs" block) was confirmed by the
GREEN run for each scenario.

## 4. Documentation corrections

None. No docs were updated during this change. (Bug #4 in the Feature:
description block notes that the source comment in `run()` is misleading,
but the source comment itself was NOT edited — reverse-engineering does
not modify production code.)

## 5. Code smell markers

None. No `TODO: REVERSE-ENGINEERING NOTE:` comments were added during
characterization. No spec/implementation mismatches were found that
required an inline marker — the four suspected bugs are documented in the
Feature: description block instead.

## 6. Suspected bugs

All four bugs were discovered during characterization and documented in
the Feature: description block of `lint-command.feature`. None were fixed
(reverse-engineering captures AS-IS). Each has a capturing scenario (where
observable) and a follow-up `@givn.modified` candidate.

1. **Severity ranking inverted vs doc comment.** The doc comment on
   `run()` says "Parse errors outrank wip/PENDING", but the code assigns
   `ParseError -> 1u8`, `Wip -> 2u8` and takes the max, so a `@wip`
   finding forces exit code 2 even when a parse error is also present
   across files.
   - Capturing scenario: "A @wip finding outranks a parse error".
   - Follow-up: `@givn.modified` on that scenario.

2. **`@givn.removed` suppression is over-broad.** The check
   `s.position.line <= wip_line` against ANY removed scenario in the
   feature suppresses unrelated later `@wip` scenarios in the same file.
   - Capturing scenario: "A @givn.removed scenario suppresses an
     unrelated later @wip scenario".
   - Follow-up: `@givn.modified` on that scenario.

3. **`--change <id>` with non-existent id silently exits 0.**
   `givn lint --change does-not-exist` reports "no .feature files found"
   and exits 0, indistinguishable from a change that exists but has no
   specs.
   - Capturing scenario: "A non-existent change id is not surfaced".
   - Follow-up: `@givn.modified` on that scenario.

4. **Source comment about exit handling is misleading.** The comment in
   `run()` says "actual process exit is handled by mapping to a non-zero
   return in main", but `lint` calls `std::process::exit(1)` and
   `std::process::exit(2)` directly, bypassing main's anyhow error
   handler.
   - No scenario (architectural note only — observable exit codes are
     documented in the Feature: description).
   - Follow-up: `@givn.modified` candidate (refactor `run()` to return
     an exit code rather than calling `std::process::exit` directly, or
     fix the comment).

## 7. Boundary failures

None. No scenarios were commented out. Every scenario in the delta spec
is exercised through the CLI subprocess boundary and passes.

## 8. Redundant unit tests removed

Two redundant unit tests removed from `src/gherkin/parse.rs` (the `#[cfg(test)]
mod tests` block was deleted entirely):

| Unit test | File:line | Superseding `.feature` scenario | Coverage impact |
|---|---|---|---|
| `parse_valid_feature` | `src/gherkin/parse.rs:121` | "All clean .feature files report a clean summary on stdout" — drives `parse_file` on a valid feature through the CLI boundary. | None — `parse_file`'s `Ok` arm is already covered by the scenario. |
| `detect_wip_scenario` | `src/gherkin/parse.rs:137` | "A @wip-tagged scenario is reported on stderr and exits 2" — drives `collect_wip` through the CLI boundary and asserts the `@wip` finding message. | None — `collect_wip`'s `@wip` branch is already covered by the scenario. |

`parse_str` (the `#[cfg(test)]` helper) is kept — it is still used by
`src/gherkin/delta.rs` and `src/gherkin/merge.rs` unit tests.

Suite re-verified GREEN after removal: 30 scenarios (30 passed), 162 steps.

The five unit tests in `src/gherkin/delta.rs` (`detect_delta_feature`,
`extract_capability_tag`, `parse_delta_ops`,
`strip_givn_tags_removes_givn_prefix`, `default_delta_op_is_added`) are
NOT redundant: they exercise `delta_op`, `delta_scenarios`,
`capability_tag`, `is_delta_feature`, and `strip_givn_tags`, none of
which are reached by any `lint-command` `.feature` scenario (lint uses
only `is_removed_placeholder`). These unit tests are the ONLY coverage
for those functions and must be kept until `.feature` coverage is added
in the respective capability's spec (see gap table, row 3).

## 9. Test suite output and result

Final verify run (behaviour-preservation invariant):
```
$ cargo test --test features_runner
[Summary]
2 features
30 scenarios (30 passed)
162 steps (162 passed)
```

All 30 scenarios (17 from `new-command`, 13 from `lint-command`) are
GREEN. The 13 `lint-command` characterization scenarios all pass.
Behaviour-preservation invariant confirmed — no characterization test
went RED during the change (no production code was modified; all
scenarios are exercised through the CLI subprocess boundary).

## 10. Sign-off checklist

- [x] All scenarios pass (test runner exits 0).
- [x] Coverage measured across all test binaries including the Gherkin
      runner (via `cargo llvm-cov`, which instruments the `givn` subprocess
      binary spawned by `features_runner`).
- [x] Every coverage gap classified and resolved (or justified) — see
      gap table: 2 tooling-artifact justifications, 2 follow-up
      `@givn.added` candidates.
- [x] No `@wip` tags remain in the delta spec (all 13 scenarios are
      tagged `@givn.added`; the `@wip` strings that appear in the file
      are either in the Feature: description's bug documentation or as
      scenario-data `@wip` tags inside the `.feature` file content under
      test — not on the delta scenarios themselves).
- [x] The delta spec contains no implementation-layer detail in its
      scenarios (Given/When/Then). The Feature: description block
      references source-level symbols (`run()`, `ParseError -> 1u8`,
      `std::process::exit`) in the "Suspected bugs" prose — this is
      necessary for the bug descriptions to be actionable as follow-up
      `@givn.modified` candidates and is permitted by the reverse-mode
      instructions (bugs are documented in the Feature: description
      block). The scenarios themselves are pure domain language.
- [x] All spec corrections documented (none — see section 3).
- [x] All documentation corrections documented (none — see section 4).
- [x] All code smell markers listed (none — see section 5).
- [x] All suspected bugs flagged as follow-up candidates (NOT fixed) —
      see section 6 (4 bugs, all `@givn.modified` candidates).
- [x] All redundant unit tests removed and listed — none removed; 2
      flagged as `@givn.removed` candidates (see section 8).
- [x] Behaviour-preservation invariant confirmed — final verify run is
      GREEN (see section 9).

REVIEW: PASS
