# Design Review: add-readability-report

Status: all six blocking questions resolved; hardening applied.

## Blocking questions

### Q1 — The init/addon-default change has no delta

The design changes `givn init` to enable only default-on addons. The permanent
scenario still asserts "givn init defaults to all addons enabled"
(`givn/specs/initialize-project/enable-optional-addons.feature:3`). The design's
OD3 proposes a `@givn.modified` scenario that restates the title, but the merge
replaces a scenario by existing title (`src/gherkin/merge.rs`), so that
mechanism cannot retitle it. The `enable-optional-addons` capability also has no
interaction row.

Options:
- **A.** Add a delta under `specs/initialize-project/enable-optional-addons.feature`:
  one `@givn.removed` for the old title, and `@givn.added` scenarios "givn init
  enables only default-on addons" and "givn init lists default-off addons and
  how to enable them"; add a `MODIFIED in initialize-project` routing row and an
  interaction inventory/E2E for enabling the readability addon. Gain: the
  permanent contract stays true and executable. Cost: the change expands into
  `initialize-project`; more scenarios and steps.
- **B.** Edit the permanent spec directly. Gain: simpler. Cost: bypasses the
  delta workflow and the archive merge record.
- **C.** Keep the current title and behavior. Gain: no work. Cost: the permanent
  spec is false; the review gate should fail.

### Q2 — Workspace scope

The design scans project-root `src/` only. A workspace root with member crates
has no root `src`, so the report would say "no production sources" — a false
empty result. Options: A. keep root `src`, detect `[workspace]`/members, and
emit "workspace crates not scanned"; B. parse `Cargo.toml` and scan members;
C. add `--path`/`--scope`.

### Q3 — Feedback and experiment exit

The design adds no feedback ledger because a ledger "would contradict D2's
no-stored-state decision." D2 rejected a classification baseline, not owner
judgments. The experiment posture says "only after some feedback." Options:
A. no ledger, manual experiment record; B. add an opt-in ledger now; C. defer
the ledger to `add-readability-review-gate` and record Q4's owner-judged exit.

### Q4 — ADR-026 is stale

ADR-026 still says `givn init` enables all known addons. The new default-on rule
refines that consequence. Options: amend ADR-026 with a dated one-line
amendment pointing at this change/spec; or record an operator-confirmed
follow-up with owner and date. Recommend the amendment.

### Q5 — The contract and metric tests are not gated

`givn/commands.yaml` runs only `./run-tests.sh --non-e2e`, and `run-tests.sh`
runs only `features_runner`. The design keeps `tests/analyzer_contract.rs` and
`--lib readability::` out of `verify.command`. Those tests are the ADR-074
confirmation and the 499/500/501 boundary evidence. Options: add both to the
non-E2E branch of `run-tests.sh` while preserving `GIVN_RESULT_FILE`; or add a
second `verify` hook.

### Q6 — `cfg` and test-source detection can falsify numbers

The adapter handles only `#[cfg(test)]`. Non-test `#[cfg(unix)]`/`#[cfg(windows)]`
items are parsed together and double-counted. `src/tests.rs`/`src/tests/**` is
parsed independently and loses its parent `cfg`. Recommended fix: emit
`cfg-conditional`/`test-source` diagnostics or `complete: false`, propagate
`cfg(test)` across module files, add fixtures, and record any accepted
limitation as a chapter 11 risk and a scenario.

## Mechanical fixes

- Fix `ADR-0074-*` → `ADR-074-*`; change the ledger's Existing-ADR check to
  `PASS` with routing `NEW_ADR`.
- Accept ADR-074 before archive; update `adr/README.md` counts.
- Define undefined model types: `CallId`/call index, `Construct`, `Limitation`,
  `UnsupportedNote`, `MetricValue`, `Comparison`, method `finding`.
- Fix sample inconsistencies: `files: 2` vs three files; the `NONE` block; the
  `MethodMetrics.public` vs JSON `visibility`.
- Put contract fixtures under `test-corpus/readability/src/`; fix the
  production/test fixture root.
- Add `src/cli/readability.rs` to the `syn` leak scan. Do not special-case
  `src/tests.rs` or `src/tests/**`; record the accepted conditional-compilation
  limitation as R-97 and the scenario "Conditional code is counted as written".
- Add `agent--technical-role` to `usecase.md`; update the main flow and the
  2,882-line example to match the design; add workspace scope to out-of-scope.
- Correct the coverage table's `features_runner` probe.
- Replace "all addons" wording in `src/cli/init.rs` and
  `assets/config.yaml.tmpl` comments.
- Add missing scenarios: `--out` success/failure, empty scope, unsupported
  construct, `givn addons enable readability`, exact exit 0/1, schema/rules
  version, `capabilities`/`limitations` in JSON.

## Hardening applied

- Q1: `enable-optional-addons.feature` delta, use-case inventory, and the
  routing row are present.
- Q2: `--path` is in the CLI contract, exit table, data model, traceability, and
  three feature scenarios.
- Q3: D4 and OD1 record the owner-judged exit, the manual feedback record, and
  the deferred ledger; proposal D4 is answered; R-97 and D-25 in arc42 chapter
  11 match.
- Q4: ADR-026 carries the dated amendment; the default-on rule is unchanged.
- Q5: the contract, boundary, determinism, and classification evidence moved to
  non-E2E Gherkin scenarios that call the public `givn::readability` API in
  process. `tests/analyzer_contract.rs`, the `run-tests.sh` change, and the
  `verify.command` change are removed from the plan.
- Q6: DD12, R-97, and the "Conditional code is counted as written" scenario
  record the accepted limitation; no diagnostics or `complete: false` are
  planned for it.
- Model types and sample inconsistencies are defined/fixed; fixtures live under
  `test-corpus/readability/src/`; the `syn` leak scan includes
  `src/cli/readability.rs`; the use case names `agent--technical-role` and the
  revised examples; the coverage table names `./measure-coverage.sh` as the
  instrumented runner.
- Remaining implementation-time edit: the "all addons" comments in
  `src/cli/init.rs` and `assets/config.yaml.tmpl` (the design commits to them).
  ADR-074 was accepted during tasks setup because the permanent register
  scenario asserts no proposed decisions; the register now counts 26 accepted /
  0 proposed and that scenario expects 26. This does not change the reviewed
  plan.

## Resolution log

- Q1: answered 2026-10-19 (user) — option A: delta under `initialize-project`,
  `MODIFIED in initialize-project` routing row, interaction row/E2E for
  enabling the readability addon. The delta feature, use-case inventory, and
  routing table are updated.
- Q2: answered 2026-10-19 (user) — option C: add repeatable `--path <dir>`
  (default `src/`); root-relative only; overlapping roots deduplicated; explicit
  missing root is exit 1, default missing root and existing empty scope are
  exit 0. Design DD7, CLI contract, exit table, data model, traceability, and
  the three scope scenarios are updated.
- Q3: answered 2026-10-19 (user) — option C: owner-judged exit with a manual
  feedback record in the ideation topic; no ledger now; the later review-gate
  change owns any structured ledger; promotion to default-on/opt-out, removal,
  or integration is a later decision. Design D4/OD1 and proposal D4 are
  updated.
- Q4: resolved 2026-10-19 as a mechanical documentation defect — ADR-026 now
  carries a dated amendment stating that `givn init` enables only default-on
  addons and that `arc42-docs` remains default-on, so its decision is unchanged.
  No operator choice was required.
- Q5: answered 2026-10-19 (user) — non-E2E Gherkin steps may call the internal
  library interfaces in process; no CLI subprocess and no separate `cargo test`
  target are required. The contract and boundary evidence becomes non-E2E
  scenarios using the public `givn::readability` API; `tests/analyzer_contract.rs`
  is removed; `run-tests.sh` and `givn/commands.yaml` are unchanged. Design
  DD3/DD8/DD9, traceability, replaceability evidence, test strategy, and the
  feature scenarios are updated.
- Q6: answered 2026-10-19 (user) — option D: accept the limitation. Non-test
  `#[cfg(...)]` branches are counted together, and parent-module `#[cfg(test)]`
  is not propagated, so `src/tests.rs`/`src/tests/**` count as production. No
  diagnostics or `complete: false` are added. Design DD7/DD12, the SynAnalyzer
  mapping, traceability, arc42 R-97, and the scenario "Conditional code is
  counted as written" record it.

DESIGN-REVIEW: PASS
