# Review: move-givn-e2e-to-black-box-package

Review kind: independent
Author: move-givn-e2e-to-black-box-package hardening author (omp, deepseek-flash)
Reviewer context: the grilling context of 2026-11-16 whose five ranked questions and five blockers this authoring context applied; the same context owns the final review, so `review.md` must record the same kind and this context identifier.

PERSONAS: agent--technical-role

README-IMPACT: none

VISUAL-REVIEW: PASS

The change moves the `@e2e` corpus into the standalone `e2e/` package, points
`verify.e2e_command` at it, writes the inert `givn/specification.yaml`, adds
the qualification script, and records ADR-079. I reviewed the plan artifacts,
the four change commits, the package, the product boundary rule, the scripts,
the declaration gate, and the moved corpus. I did not author this change.

## Implementation evidence check

### Corpus integrity

- `git diff 132d9ea3^..HEAD -- givn/specs` is one changed line:
  `the final register has 30 accepted decisions` becomes `31`. The diff
  contains no `@e2e` and no `Scenario:` line, so no scenario title and no tag
  changed in the permanent corpus.
- The permanent corpus has 57 feature files with `@e2e` scenarios both at
  `132d9ea3^` and at `HEAD`.
- Robust count at both revisions: **114 `@e2e` scenario declarations** (111
  `Scenario:` plus 3 `Scenario Outline:`), **8 outline example executions**,
  **119 executed scenarios**. The head's naive grep count of 116 includes two
  `Scenario:` lines inside step docstrings
  (`givn/specs/verify-change/review-disposition-gates.feature:62` `Solo delta`
  and `:86` `Receipt added`). No title was added or removed: the old and new
  title sets are identical.
- `tests/steps/` holds one `*_e2e_steps.rs` file at `HEAD`
  (`llm_attribution_e2e_steps.rs`), 31 at `132d9ea3^`. The 30 deleted paths are
  the moved corpus. The kept file binds fixture builders for non-`@e2e`
  attribution scenarios and duplicates its `@e2e` phrases in the package; it is
  not a second acceptance implementation because the regular scope excludes
  `@e2e`.
- The package has 85 Rust files under `e2e/tests/steps/`, 76 with step
  bindings, 543 `#[given|when|then]` attributes. Every `@e2e` phrase is bound:
  the full run is 119 executed, 119 passed.

### Step bodies

0 empty or trivial step bodies across the 85 package step files. The only
`{}` bodies are the generated fixture source strings
(`ise_clean_room_support.rs`, `archive_proof_support.rs`) and the
`#[cfg(not(unix))]` fallback `make_executable`. No step body is a no-op.

### Component promises

Every component the design promises exists: `e2e/Cargo.toml` with `[workspace]`
and `publish = false`, `e2e/Cargo.lock`, `e2e/tests/e2e_runner.rs`,
`e2e/tests/harness/**`, `e2e/tests/steps/**`, `e2e/browser/**`,
`run-tests.sh`, `measure-coverage.sh`, `givn/specification.yaml`,
`scripts/qualify-givn-e2e.sh`, `docs/arc42/adr/ADR-079-standalone-acceptance-package.md`,
and the root `exclude = ["e2e/**"]`. The one exception is the path in
design.md's Step Definitions table (finding F2); the module itself exists as
`e2e/tests/steps/workflow_enforcement_e2e_steps.rs`.

No consumer imports the package implementation. The package imports no product
crate: `grep -R 'givn::\|use givn' e2e/tests` returns 0; the only reach is
`GIVN_BIN`, `GIVN_READY_FIXTURE`, `GIVN_FEATURES`, `GIVN_RESULT_FILE`, and
`GIVN_RESULT_SCOPE`. The declared contract (`E2E runner contract`) has one
implementing component (`e2e/tests/e2e_runner.rs`) and four consumers
(`run-tests.sh`, `measure-coverage.sh`, `givn check tasks`, `givn archive`).

### Strict mode

`e2e/tests/e2e_runner.rs` calls `.fail_on_skipped()`, filters `@e2e` and
excludes `@givn.removed`, and exits 1 when `failed > 0`, `skipped > 0`, or
`stats.total() == 0`. It writes the four-count result before exiting. The
tasks' strict-mode proof is present. Probes: an absent browser driver produced
8 failed scenarios and exit 1, never a skip (see P5); an unbound permanent
`@e2e` step produced `Step doesn't match any function` and a scenario failure
(F3).

### E2E assertions and driver

The delta scenarios assert the CLI exit, receipt package digests, the receipt
runner-command digest, and the receipt E2E scope. The browser scenarios drive
`e2e/browser/browser-driver.mjs` through `BrowserDriver::start`, `goto`, and
`evaluate`, then assert page content. No browser step sends HTTP or calls
`fetch`/`XMLHttpRequest`; no step asserts repository state alone.

`verify.e2e_command` resolves to
`./run-tests.sh --e2e --package e2e` → `e2e/tests/e2e_runner.rs` and
`e2e/tests/steps/**`. The regular command is `./run-tests.sh --non-e2e` →
`tests/features_runner.rs`. `git status --porcelain` shows no untracked
parallel runner; the only other `@e2e`-named step file is the kept
`tests/steps/llm_attribution_e2e_steps.rs` described above. The package is the
stronger implementation (binary, not product imports).

### Scope selection

`run-tests.sh --e2e` exports `GIVN_RESULT_SCOPE=e2e`; the package writes that
scope. The change's own receipt records the `change` scope only because the
delta has no `@e2e` scenario; the delta fixture asserts the E2E scope for its
`@e2e` scenario. This matches the design's delta-aware rule.

### Design match

Commands, framework (cucumber-rs 0.23), driver (Playwright), and package
contents match design.md. The Step Definitions table's path does not (F2), and
the `names_root` decision is not realized as a distinct rule (F1). Both need
the review loop, not a silent shortcut.

### Interaction coverage

`givn/specs/verify-change/usecase.md` has 13 inventory entries; the design's
matrix lists all 13 plus the one touched fragment entry
(`adr-qualification-gate`), 14 rows total. The delta adds no entry.

| Inventory entry | Design row | @e2e scenario | Driving mechanism |
|---|---|---|---|
| validate-artifact-ids: check an unknown artifact | yes | Consumer invokes givn check with an unknown artifact and gets a helpful error | `run_givn` spawns `GIVN_BIN` |
| visual-review-gate: run the review check on a visual change | yes | A visual change with complete evidence passes review | `run_givn` spawns `GIVN_BIN` |
| visual-review-gate: read the visual review guidance | yes | Review guidance requires the visual verdict and persona lenses | `run_givn` spawns `GIVN_BIN` |
| spec-overlap-gates: detect a duplicate title | yes | A duplicate title across two permanent spec files is a hard error | `run_givn` spawns `GIVN_BIN` |
| spec-overlap-gates: inspect duplicate JSON | yes | The json output reports duplicate-title findings machine-readably | `run_givn` spawns `GIVN_BIN` |
| resolve-change-directories: run integrity in a change | yes | Consumer runs the integrity check on a change inside givn/changes and the correct directory is detected | `run_givn` spawns `GIVN_BIN` |
| resolve-change-directories: reject placeholder commit evidence | yes | Integrity check detects a placeholder COMMIT hash in tasks.md and fails | `run_givn` spawns `GIVN_BIN` |
| check-specification-health: lint the specification corpus | yes | All clean .feature files report a clean summary on stdout | `run_givn` spawns `GIVN_BIN` |
| givn-workflow-enforcement: run migration coverage check | yes | Consumer runs migration coverage check and sees passed evidence | `run_givn` spawns `GIVN_BIN` |
| givn-workflow-enforcement: run the change-scoped checks | yes | Consumer runs check tasks and receives a change verification receipt | `run_givn` spawns `GIVN_BIN` |
| givn-workflow-enforcement: request bare-check guidance without execution | yes | Consumer invokes bare check and receives explicit non-executing guidance | `run_givn` spawns `GIVN_BIN` |
| givn-workflow-enforcement: write a runner result | yes | The result helper writes four counts without a scope | `run_givn` spawns `GIVN_BIN` |
| givn-workflow-enforcement: run the review check on a change with use-case schema errors | yes | Consumer runs review and receives every use-case schema problem in one pass | `run_givn` spawns `GIVN_BIN` |
| adr-qualification-gate: request architecture-decision guidance (design-review selection is a regular variant) | yes | An author requests the strict ADR guidance through the Givn CLI | `run_givn` spawns `GIVN_BIN` |

The remaining `givn/specs/fragments/fragment.md` inventory entries belong to
capabilities this change does not touch; the delta modifies only the
`adr-qualification-gate` scenario.

## Probes and outcomes

### P1: no product dependency

`cargo tree --manifest-path e2e/Cargo.toml -e normal,build,dev --prefix none`
names no `givn`, `givn-config`, or `givn-core` package (0 matches). A copy of
`e2e/Cargo.lock` with an appended `[[package]] name = "givn"` made the package
runner exit 2 naming the product crate; the lockfile was restored.

### P2: corpus completeness and unbound failure

Full package run: 56 features, 119 scenarios (119 passed), 781 steps (781
passed), exit 0. The design's 116 is the miscount in F4; the 119 is 111 plain
plus 8 outline examples. No scenario title changed and no `@e2e` tag was
removed. With one step module's phrase absent (the permanent ADR `@e2e`
scenario in the component tier), the run failed with
`Step doesn't match any function`, not a skip.

### P3: runner name

`./run-tests.sh --e2e` refuses with exit 2:
`--e2e names the tier, not the package`. `./run-tests.sh --e2e --package other`
is refused with `--package names no declared e2e root`. The unit tests
`the_package_flag_names_its_value`,
`a_package_flag_naming_another_root_names_nothing`, and
`a_bare_tier_flag_names_no_package` pass. **They also pass with the new
`--package` branch disabled**, and the delta scenarios pass 6/6 with it
disabled: the branch is dead (F1). The outline's new `--e2e` row is real:
accepting the bare flag makes exactly that row fail (1 failure, exit 1).

### P4: script contract

Invoked from `/tmp` with a relative `GIVN_FEATURES` and
`GIVN_RESULT_FILE=target/probe-result.json`:

- the scoped feature ran (1 scenario, 1 passed, 4 steps, exit 0);
- the result landed at the repository root `target/probe-result.json`, not
  under `e2e/`, with `{"scope":"e2e","total":1,"passed":1,"failed":0,"skipped":0}`;
- a preset executable `GIVN_BIN` wrapper was invoked 3 times and
  `target/debug/givn`'s mtime was unchanged, so the runner did not rebuild it.

Default `./run-tests.sh` with one mixed feature ran the component tier
(nextest: 7 tests, 7 passed, 1 skipped by the `not @e2e` scope) and then the
package (1 scenario passed), exit 0.

### P5: browser

With `e2e/browser/node_modules` moved aside, the development-report browser
feature produced 8 failed scenarios and exit 1; every failure named
`the browser driver exited before answering 'goto'`. No scenario was skipped.
The `node_modules` directory was restored.

### P6: coverage

`measure-coverage.sh` builds the instrumented binary with
`cargo llvm-cov --no-run --no-report --locked --bin givn`, checks
`target/llvm-cov-target/debug/givn` exists, exports `GIVN_BIN` and
`LLVM_PROFILE_FILE`, and calls `./run-tests.sh --e2e --package e2e`. A probe
with a no-op `cargo` in `PATH` reached the check and exited 2:
`the instrumented givn binary is missing at ...`. A preset executable
`GIVN_BIN` is trusted by the runner's contract, not staleness-checked; the
coverage script owns the build, and the archive merge gate rejects empty or
duplicate reports. The existing instrumented binary is from 16:00, before the
20:45 `names_root` edit, so this change has no current coverage measurement;
the archive must regenerate both reports (finding F7).

### P7: ADR register

`docs/arc42/adr/README.md` has 31 `| accepted |` rows, 0 proposed rows, and the
ADR-079 row accepted. The permanent `@e2e` scenario passes through the package
(1 scenario, 7 steps); the component scope with `@adr-qualification-gate and
not @e2e` passes 17/17. Changing the ADR-079 row to `proposed` made the delta
`@givn.modified` fragment scenario fail at the 31-count assertion, so the check
is discriminating.

### P8: declaration inert while `ise` is off

`givn/specification.yaml` carries all six keys (`behavior`, `structure`,
`contracts`, `limits`, `e2e`, `source`). `givn lint` reports
`66 file(s) checked — clean`; with the file moved aside the output is
byte-identical, so lint does not read it. `preflight_e2e_boundary` returns
early when `lock::ise_effective_for_change` is false; `src/cli/archive.rs`
gates the declaration check on `is_effective`; the change's receipt carries no
E2E fields. `givn/config.yaml` has no `ise` key, so the opt-in addon is off.
The change does not enable it.

### P9: fabrication audit

| Check | Mutation | Result |
|---|---|---|
| Added scenario: standalone workspace accepted and receipt bound | `CargoPackage::read` refuses a `[workspace]` manifest | 6 scenarios, 5 passed, 1 failed at `Then the command exits 0`, exit 1 (`malformed manifest: standalone workspace package refused`) |
| Modified outline row: bare `--e2e` refused | `names_root` accepts the `--e2e` token | 6 scenarios, 5 passed, 1 failed on the `--e2e` row, exit 1 |
| Modified fragment scenario: 31 accepted | ADR-079 row changed to `proposed` | 1 scenario failed at `the final register has 31 accepted decisions`, exit 1 |
| Added scenario: typed `--package` acceptance | `--package` branch disabled | **6 passed, exit 0** (F1) |
| Added scenario: receipt digest/scope assertions | Not separately mutated; the scenario's first failure under the workspace mutation is the exit, so the receipt assertions were not reached | Stated, not claimed as broken |
| Outline rows 1 to 3 (pre-existing refusals) | Not mutated; unchanged behavior | Stayed green |
| `the_package_flag_names_its_value`, `a_package_flag_naming_another_root_names_nothing`, `a_bare_tier_flag_names_no_package` | `--package` branch disabled | 12 unit tests passed, including all three (F1) |

The one check that stayed green under its own removal is the typed-runner
branch. This is the critical finding.

## Findings

### F1 (critical, blocker): the typed `--package` rule is dead and its RED is not reproducible

`src/e2e_boundary/mod.rs:444-449` adds:

```rust
if tokens
    .windows(2)
    .any(|pair| pair[0] == "--package" && pair[1] == needle)
{
    return true;
}
```

The pre-existing rule at `:450-454` already accepts any token equal to the
root. For every `--package <root>`, the value token `<root>` is itself a token
equal to the root, so the new branch cannot change any result. Mutation proof:

- replacing `"--package"` with `"--package-disabled"` leaves all 12
  `e2e_boundary::tests` passing, including the three new tests;
- the delta feature still reports 6 scenarios (6 passed), exit 0.

Consequences:

- `tasks.md` batch 2's RED claim is false. It says the accepted-command
  scenario failed before `names_root` learned `--package`; the scenario passes
  without that learning.
- The three new unit tests pass on the pre-change function; they do not test
  the new branch.
- The design's "typed form ... names that package and nothing else" is not
  realized. A command such as `./run-tests.sh --bin e2e` already names the
  package under the old whole-token rule, and the design keeps that rule.

This is dead code (bucket 1) and a vacuous test set (bucket 2), and the
recorded RED evidence is fabricated. `REVIEW: PASS` is not available while it
remains.

Fix (author, production/test/task edits): delete the `--package` branch and
the three tests, or replace them with a rule that actually distinguishes the
typed form (which changes consumer-visible refusals and contradicts the
reviewed decision). Correct `design.md`, `design-review.md`, and `tasks.md` to
record that `./run-tests.sh --e2e --package e2e` is accepted by the existing
whole-token rule and that this change adds the command and the script, not a
product naming rule. Re-run this review.

### F2 (blocker, gate): design.md Step Definitions promises a file that does not exist and trips the gate

`givn check review` exits 1 with:

```text
[integrity] Step Definitions: file `e2e/tests/steps/workflow_enforcement.rs` does not exist (resolved from project root)
[integrity] Step Definitions: file `Binding file` does not exist (resolved from project root)
[integrity] Step Definitions: file `---` does not exist (resolved from project root)
integrity check failed: promised files
```

`design.md:411` names `e2e/tests/steps/workflow_enforcement.rs`; the as-built
module is `e2e/tests/steps/workflow_enforcement_e2e_steps.rs`. The second table
(`New and changed phrases`) sits under the `## Step Definitions` heading, and
the integrity parser reads every table row until the next `## ` heading, so it
also sees `Binding file` and `---` as file paths.

Fix (author): name the actual module file, move the phrases table under its own
`## ` heading (the parser does not stop at `###`), and re-run design-review for
the design.md change.

### F3 (blocker, evidence): tasks.md's permanent fragment check cannot pass as recorded

`tasks.md` batch 6 records
`GIVN_FEATURES=givn/specs/fragments cargo test -- --tags
"@adr-qualification-gate"` → `18 scenarios (18 passed)`. At `HEAD` the same
command gives **18 scenarios (17 passed, 1 failed)**: the permanent `@e2e`
scenario `An author requests the strict ADR guidance through the Givn CLI` has
no component-tier binding after the move (`Step doesn't match any function`).
The correct split is:

- component: `@adr-qualification-gate and not @e2e` → 17 scenarios (17 passed);
- package: `GIVN_CUCUMBER_TAGS='@e2e and @adr-qualification-gate'` → 1
  scenario (1 passed), 7 steps.

The behavior is green in both scopes; the recorded command and count are
wrong. Correct `tasks.md` (and the design-review claim that repeats 18/18) as
part of the same closure.

### F4 (documentation): design.md's corpus measurement is 116, the real declaration count is 114

The design's "116 `@e2e` scenarios in 57 feature files" includes two
`Scenario:` lines inside step docstrings
(`review-disposition-gates.feature:62` and `:86`). The real corpus is 114
declarations in 57 files; 3 outlines expand to 8 executions, so the runner
executes 119. No scenario is lost. Correct the measured figure in design.md so
the record states 114 declarations and 119 executions.

### F5 (documentation): chapter 04 calls the package a workspace member

`docs/arc42/04-solution-strategy.md:23` says "a standalone `e2e` workspace
member". The decided shape is the package's own workspace root (design D3,
`TC-52`, ADR-079). Fix the row to "a standalone `e2e` workspace package". The
other occurrence ("a workspace member cannot" in ADR-079) is the rejected
option and is correct.

### F6 (minor): stale references in the moved browser harness

`e2e/tests/harness/browser.rs` still says the driver is
`tests/e2e/browser-driver.mjs` and the `#[allow(dead_code)]` comments say
"Task E2–E9" and "Task E1". The real path is `e2e/browser/browser-driver.mjs`
and the task ids belong to another change. Not a behavior defect; clean up in
the same closure.

### F7 (coverage note, not a defect): no current coverage measurement

This change has no coverage report. `target/llvm-cov-target/debug/givn` is from
16:00, before the 20:45 source edit. The archive coverage gate must regenerate
both reports with the current source; the mechanism is in place and was probed
(P6). The archive must not accept the stale instrumented artifact.

## Arc42 implementation conformance

| Arc42 chapter or fact | Durable-doc source | `arc42.md` claim | `design.md` | `tasks.md` | Implementation evidence | Match? |
|---|---|---|---|---|---|---|
| TC-52 package shape, exclude, binary-only reach, `--package` command | `docs/arc42/02-architecture-constraints.md:60` | Yes | Yes | Batches 1 and 2 | `e2e/Cargo.toml`, root `Cargo.toml:18`, `run-tests.sh`, `givn/commands.yaml` | Yes |
| Acceptance package and two tiers | `docs/arc42/04-solution-strategy.md:22-24` | Yes | Yes | Batches 1, 2 | `run-tests.sh`, `e2e/` | **No, F5 wording** |
| Package building block and component-tier rename | `docs/arc42/05-building-block-view.md:27-28,66` | Yes | Yes | Batch 1 | `e2e/`, `tests/features_runner.rs` | Yes |
| 6.14 acceptance flow with `GIVN_BIN` and the result contract | `docs/arc42/06-runtime-view.md:665-690` | Yes | Yes | Batches 1, 2 | `e2e_runner.rs`, `run-tests.sh` | Yes |
| Browser tier path and no installed product reach | `docs/arc42/07-deployment-view.md:100-101` | Yes | Yes | Batch 4 | `e2e/browser/`, `setup-e2e-browser.sh` | Yes |
| 8.31 package shape, runner variables, strictness | `docs/arc42/08-crosscutting-concepts.md:1622-1648` | Yes | Yes | Batches 1, 2 | `e2e_runner.rs`, `run-tests.sh` | Yes |
| ADR-079 accepted, register 31 | `docs/arc42/adr/README.md:30,66`; chapter 09 | Yes | Yes | Batch 6 | ADR-079, register 31/0 | Yes |
| R-155 to R-159 | `docs/arc42/11-risks-and-technical-debt.md:140-144` | Yes | Yes | Batches 4, 6 | script, runner guard | Yes |
| `E2E package` term | `docs/glossary.md:73` | Yes | Yes | Batch 6 | glossary entry | Yes |

`ARC42 CONFORMANCE` is not clean because chapter 04 contradicts the decided
shape (F5). The review does not write the clean marker.

## Ubiquitous language and structural model

`E2E package` is in `docs/glossary.md` and names the package. `component tier`
is its recorded anti-term. The specs, design, ADR, and reference use the term
consistently. The only inconsistent prose is chapter 04's "workspace member"
(F5).

`docs/domain-model.md` already excludes the `e2e` package from the behavior
aggregate; the change moves a repository test asset and alters no domain
boundary. No model edit is required, and none is claimed.

## Use-case and Persona conformance

`verify-change` has one goal, the Developer as primary actor, a named trigger,
placed capabilities, extensions that reference their flow steps, and
consistent preconditions. No scope rejection criterion holds. The fragment
capability keeps its owner and gains only a count change.

Confirmed Personas: `agent--technical-role` (runs the E2E scope and reads
named failures) and `givn-maintainer--owner` (owns the package, lockfile, pin,
and qualification). Both files exist in `givn/personas/`. No invented Persona
and no biography in Gherkin.

## Visual review

The contract is `Interface: terminal`. The delta adds no `@e2e` scenario, so no
per-scenario transcript is required. The terminal baseline holds: every
failure names the scenario and step, the summary prints four counts, and a
missing binary, ready fixture, result path, or browser driver fails by name
(probes P3 to P5). Both personas reach their goals: the agent runs the scope
and reads the named failure; the maintainer sees the lockfile and tree guards.
No contract rule or Persona goal fails.

VISUAL-REVIEW: PASS

## Coverage classification

No coverage measurement exists for this change (F7); classification is from
inspection.

1. **Dead code**: `names_root`'s `--package` branch is dead (F1). Delete it.
2. **Missing test coverage**: the claimed typed-rule behavior has no
   discriminating test; the three added unit tests and the added scenario pass
   without it (F1). Add a test only if a real distinct rule is implemented.
3. **Legitimately hard to test**: none.

Redundant tests: the three `names_root` tests duplicate the existing
whole-token test's coverage of the accepted form; remove them with the branch.

Branch instrumentation is unchanged in `measure-coverage.sh`; both scopes still
report distinct Cobertura files.

## E2E coverage

The non-browser classification is correct for the product surface: the product
is a CLI, and the delta's scenarios assert CLI exit, stderr, and receipt JSON.
The opposite classification (Web UI) does not hold for this change: the browser
is an existing scenario assertion mechanism, not a new rendered product
surface. Every normalized inventory action in the touched use cases has one
`@e2e` scenario; variants remain regular scenarios; no action has two `@e2e`
scenarios. No downgraded scenario was found: the browser steps drive the real
driver, and the CLI steps spawn the binary.

## Deterministic gate dispositions

## Overlap dispositions

The gate reports `✓ overlap dispositions passed`. The `[SUBST]` lines it
prints are advisory subset matches between permanent workflow scenarios and
the added scenario; they are not blocking shape matches involving this change
and require no disposition. No blocking shape-match pair is recorded.

| Scenario A | Scenario B | Disposition |
|---|---|---|

## Split-or-keep

No scenario in this change exceeds the gate's length threshold; the added
scenario has 7 steps and the modified outline is 4 steps.

| Scenario | Decision | Reason |
|---|---|---|

Removed+added pairs: none (`net delta: 1 added, 2 modified, 0 removed`).

## Sign-off checklist

- Fabrication audit: clean after closure. F1 deleted the dead branch and its
  vacuous tests, and the false RED is corrected in `tasks.md` and `design.md`.
- Every checked task has a verified commit: yes. F3's observable command and
  counts were corrected and re-run at `5ba85549`.
- Every promised component exists: yes. F2 corrected the Step Definitions path
  and the phrase table placement.
- Every consumer uses only its declared contract: yes.
- Strict-mode proof present and passing: yes.
- Scenario execution evidence: component permanent 17/17 (91 steps), package
  permanent 1/1 (7 steps), delta 6/6 (37 steps), full package 119/119 (781
  steps); the change receipt records the 6-scenario change scope.
- Coverage classification: no coverage report for this change; the archive
  regenerates both reports (F7), as `tasks.md` batch 4 records.
- Every gap classified and resolved: yes after the F1 closure.
- Dead code deleted: yes (F1, `8fb6c3f6`).
- Missing tests added: not applicable after the dead rule was deleted; the
  accepted command is covered by the whole-token unit test and the delta
  scenario.
- Redundant unit tests removed: yes (F1).
- No `@wip` tags: yes (`givn lint` clean).
- Canonical E2E policy read and applied: yes.
- Real-interface primary assertions and reviewed driver: yes.
- The local run command starts the full stack: yes (P4).
- `verify.e2e_command` files identified and no untracked parallel runner: yes.
- Explicit E2E scope selection: yes (`GIVN_RESULT_SCOPE=e2e`).
- Design match: yes after the F1 and F2 corrections.
- Interaction coverage: yes, 13 plus 1.
- No fourth-bucket exception: none used.

## Gate output

Before this file existed, `givn check review --change
move-givn-e2e-to-black-box-package` exited 1 with the promised-files failure,
a missing verdict, and a missing persona disposition. After the F1 to F7
closures, the only remaining error was the FAIL verdict. With the re-audit at
`5ba85549` and the verdict flipped, the gate exits 0:

```text
  ✓ overlap dispositions passed
  ✓ scenario commits passed (3 scenario(s))
  ✓ change verification receipt reused across documentation-only revision
    5ba85549b49135dd883973b66343efa1a23a9a24
    (tested at 67ad51bb1466b746879684fbe4501a2f9b2f0e70, 6 scenario(s))
  ✓ scenario commits passed (3 scenario(s))
EXIT: 0
```

## Finding closure

Recorded by the implementation context. The verdict line below stays the
reviewer's.

| ID | Fix | Evidence | Commit |
|---|---|---|---|
| F1 | Deleted the dead `--package` window branch in `names_root` and its three unit tests. The declared command satisfies the pre-existing whole-token rule because `e2e` is a token equal to the declared root, so no product code remains in the change. `design.md`, `design-review.md`, `proposal.md`, `docs/reference.md`, and `tasks.md` batch 2 now record that fact. | `./run-tests.sh --e2e --package e2e` → 119 scenarios (119 passed); the delta run → 6 scenarios (6 passed), including the accepted-command scenario through the real rule; `./run-tests.sh --e2e` exits 2 with `--e2e names the tier, not the package`; `./run-tests.sh --e2e --package other` exits 2 with `--package names no declared e2e root: 'other'`. | `8fb6c3f6`; docs `cda1a089`, `67ad51bb` |
| F2 | The Step Definitions table names `e2e/tests/steps/workflow_enforcement_e2e_steps.rs`; the phrase table moved under its own `## New and Changed Step Phrases` section. | `givn check review` reports no promised-file integrity error. | `67ad51bb` |
| F3 | `tasks.md` batch 6 records both scopes: `GIVN_FEATURES=givn/specs/fragments cargo test --locked --test features_runner -- --tags "@adr-qualification-gate and not @e2e"` → 17 scenarios (17 passed), 91 steps; `GIVN_CUCUMBER_TAGS='@e2e and @adr-qualification-gate' ./run-tests.sh --e2e --package e2e` → 1 scenario (1 passed), 7 steps. `design-review.md` keeps the pre-move 18/18 run and states the post-move split. | Both commands above. | `cda1a089`, `67ad51bb` |
| F4 | `design.md` and `proposal.md` measure 111 `@e2e` `Scenario:` declarations plus 3 `Scenario Outline:` declarations whose 8 example rows execute: 119 executions in 56 tagged feature files. `review-disposition-gates.feature` is the 57th file a `@e2e` string grep counts; the tag appears there only inside step docstrings. No title and no tag changed. | `./run-tests.sh --e2e --package e2e` prints `56 features` / `119 scenarios (119 passed)` / `781 steps`; `git diff 132d9ea3^..HEAD -- givn/specs` holds no `Scenario:` or `@e2e` change. | `67ad51bb` |
| F5 | Chapter 04 reads "a standalone `e2e` workspace package with its own workspace root, excluded from the published crate". | `docs/arc42/04-solution-strategy.md:23` | `553f3c48` |
| F6 | `tests/e2e/browser-driver.mjs` no longer exists. The remaining reference, `tests/common/browser.rs`, spawned it at that dead path and no component scenario binds a browser step, so the module and its `pub mod browser;` are deleted. The package harness names `e2e/browser/browser-driver.mjs` and no longer cites another change's task ids. | `rg 'tests/e2e/browser'` → no hits; `./run-tests.sh --non-e2e` → 1081 passed, 114 skipped; the package run → 119 passed. | `1b658a63` |
| F7 | `tasks.md` batch 4 records that `coverage/` predates the source edit and that archive regenerates `coverage/non-e2e-cobertura.xml` and `coverage/e2e-cobertura.xml` from the merged candidate, and states the guard: with a no-op `cargo` in `PATH` the script exits 2 with `the instrumented givn binary is missing at <root>/target/llvm-cov-target/debug/givn`. No report was hand-edited. | Probe above; the instrumented binary was restored afterwards. | `cda1a089` |

### Re-audit at 2026-11-16

The review context re-ran every closure on `5ba85549` and the full package
run after it. This table records what the reviewer verified, not what the
implementation context reported.

| ID | Commit | Date | Re-audit result |
|---|---|---|---|
| F1 | `8fb6c3f6` plus docs `cda1a089`, `67ad51bb` | 2026-11-16 | `src/e2e_boundary/mod.rs:432-441` is the whole-token rule; no `--package` branch and no `the_package_flag_names_its_value`, `a_package_flag_naming_another_root_names_nothing`, or `a_bare_tier_flag_names_no_package` test remains. The `src/` diff against `132d9ea3^` is 4 doc-comment lines, not literally empty, and changes no behavior. `./run-tests.sh --e2e` exits 2 with `--e2e names the tier, not the package`; `./run-tests.sh --e2e --package other` exits 2 with `--package names no declared e2e root: 'other'`. Delta clean: 6 scenarios (6 passed), 37 steps. `CargoPackage::read` workspace mutation: 6 scenarios (5 passed, 1 failed) at `Then the command exits 0`, exit 1. Full package: 56 features, 119 scenarios (119 passed), 781 steps, exit 0. |
| F2 | `67ad51bb` | 2026-11-16 | The Step Definitions table names `e2e/tests/steps/workflow_enforcement_e2e_steps.rs`; the phrase table sits under its own `## New and Changed Step Phrases` heading. `givn check review` reports no promised-file integrity error. |
| F3 | `cda1a089`, `67ad51bb` | 2026-11-16 | Component `@adr-qualification-gate and not @e2e`: 17 scenarios (17 passed), 91 steps. Package `@e2e and @adr-qualification-gate`: 1 scenario (1 passed), 7 steps. Both exit 0. |
| F4 | `67ad51bb` | 2026-11-16 | Design and proposal measure 111 `@e2e` `Scenario:` declarations plus 3 outlines whose 8 example rows execute, for 119 executions in 56 feature files. `GIVN_CUCUMBER_TAGS='@e2e' ... --list` prints 119 units across 56 distinct feature paths. The 57th file a string grep counts is the docstring-only case. No title or tag changed. |
| F5 | `553f3c48` | 2026-11-16 | `docs/arc42/04-solution-strategy.md:23` reads "a standalone `e2e` workspace package with its own workspace root". |
| F6 | `1b658a63` | 2026-11-16 | `tests/common/browser.rs` and its module declaration are gone; `e2e/tests/harness/browser.rs` names `e2e/browser/browser-driver.mjs`; no `tests/e2e/browser` or "Task E" reference remains. |
| F7 | `cda1a089` | 2026-11-16 | `tasks.md` batch 4 records that `coverage/` predates the source edit and that archive regenerates both reports. With a no-op `cargo` in `PATH`, `measure-coverage.sh` exits 2 naming the missing instrumented binary. |

The design-review record was amended in place at `67ad51bb` (Q1 and blocker 2)
to state the corrected runner rule, the 56-file corpus, and the 17 plus 1 scope
split. Its `DESIGN-REVIEW: PASS` marker remains at the end, and the review
context re-read the amended record against the corrected `design.md` and
re-confirms it. The design change is a correction of findings F1, F2, F3, and
F4, not a new decision.

### Gate state after closure

`givn check tasks --change move-givn-e2e-to-black-box-package` passed at
`67ad51bb1466b746879684fbe4501a2f9b2f0e70` (6 delta scenarios, verified content
`f171013d7174e4d9cc8706fc810ee24c904cd55cbed5aee45f1e2314747876ff`); the
receipt commit is `efbd381ab1`.

After the re-audit and the verdict flip, `givn check review` exits 0. No
integrity, receipt-freshness, scenario-commit, overlap, run-declaration, lint,
visual, persona, or README error remains.

Runner results: `cargo fmt --all -- --check` exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` exit 0 and the same for
`--manifest-path e2e/Cargo.toml`; `./run-tests.sh --e2e --package e2e` → 56
features, 119 scenarios (119 passed), 781 steps (781 passed), exit 0;
`./run-tests.sh --non-e2e` → 1081 tests passed, 114 skipped, exit 0.

Not closed here: the six binaries under
`givn/archive/development-timeline/evidence/visual/**` were rewritten by the
package run of another archived change. They are outside this change's tested
content and were restored to their tracked content with
`git checkout -- givn/archive/development-timeline/evidence/visual` before
archive.

REVIEW: PASS
