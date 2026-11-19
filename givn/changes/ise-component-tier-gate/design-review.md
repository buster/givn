Review kind: independent
Author: DeepSeek Flash execution agent
Reviewer context: DesignReviewGate (independent reviewer subagent)

# Design review: ise-component-tier-gate

## Grilling branches

| Branch | Outcome |
|---|---|
| Scope | The proposal, delta specs, and design cover the generated contract, the read-only comparison label, the exact-count and scenario-title trial gate, the product binding, coverage, the not-run rule, and the honest external-report boundary. |
| Tech choices | Rust modules in `src/ise` reuse the existing `RunnerResult`, Cobertura parser, subprocess seam, and SCM seam. A separate `verify-trial` command is the only executable path; `compare` stays read-only. |
| Missing scenarios | The delta covers contract generation, input-only comparison, missing runner, count-only result, foreign product, exact counts, exact mismatch, and the not-run bypass. Coverage, extra-title, product-path, and post-run-drift edges are covered by unit and acceptance tests. |
| Testability | Every delta scenario drives the real binary through a real Git fixture. Strict mode is `.fail_on_skipped()`. The single-scenario command is `GIVN_FEATURES=... cargo test --locked --test features_runner -- --exact "<usecase>/<capability>.feature::<title>"`. A not-implemented body uses `unimplemented!()`. |
| E2E fidelity | The verifier is a CLI; the gate drives the real binary and real `sh` commands. The existing `@e2e` compare scenario remains the one real-interface entry for the verifier action; `verify-trial` is a variant of that action, so no second inventory row is added. |
| Interaction Coverage | The `prepare-an-ise-test` inventory row for `trial-input-verification` now names the read-only comparison and the trial verification variants; the existing `Verifier compares the spec copy with the manifest` `@e2e` scenario remains the real-interface evidence. |
| Risk | The highest risk is a runner or End-to-end command that reports counts without corpus/product identity. The gate refuses a count-only result, an undeclared title, and a foreign product, so the runner is corrected rather than the gate weakened. |
| Use-case and Persona context | The owning use case is `prepare-an-ise-test`; Personas are `developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner`. The terminal contract names `developer--primary-user`. |
| Decision ledger | `D1` exact counts, `D2` read-only compare plus separate gate, `D3` `verify-trial` name, `D4` expanded execution units, `D5` `coverage.e2e_subprocess`, `D6` product path/digest binding, `D7` scenario titles, and `D8` no phrase-triggered workflow gate; external/manual reports are outside enforcement each resolve in the design's Decisions ledger. `Ise rebuild decision: No rebuild` is recorded with no `Rebuilt code` line. |
| Contracts and components | The design names three contracts and one implementing component each. |
| Decision timing | All decisions are answered; none is carried. |
| Visual design contract | Interface is `terminal`; no rendered screen exists, so viewports and design-system tokens are not applicable. The fixed line order is the contract. |
| Persona disposition | `developer--primary-user` is the primary persona for the CLI output; the agent and maintainer are review lenses. |
| Interface classification | The opposite classification (web) does not hold: there is no HTML, CSS, or JavaScript surface. |
| ADR qualification | ADR-076 is amended, not replaced: it already owns the clean-room record and command surface. The amendment records the generated contract, the read-only comparison, and the trial gate. |
| Deferrals | None. |

## Findings and closure

| ID | Severity | Finding | Resolution |
|---|---|---|---|
| F1 | P2 | The reserved contract path was checked only against the copy set, so a revision that tracked `givn/ise-verification.yaml` outside the copied paths could still get a generated contract. | `src/ise/mod.rs` now checks every `admission.entries` path. Regression: `a_revision_tracking_the_reserved_contract_path_refuses`. |
| F2 | P2 | `--result ../copy/trial.json` was joined lexically and escaped the containment check. | `src/ise/verify.rs` now resolves the deepest existing ancestor through `super::resolved_target` before comparing with the canonical copy. Regression: `a_result_path_escaping_into_the_copy_refuses`. |
| F3 | P3 | Guidance and reference named `component_executed/component_total` while the JSON carries `component.declared` and `component.executed`. | `assets/instructions/ise.md`, `docs/reference.md`, and `proposal.md` now name the serialized fields; the refusal label remains `component_executed=<n>/<declared>`. |
| F4 | P2 | A broken symlink at `--result` was not resolved and `write_result` followed it into the copy. | `src/ise/verify.rs` refuses any final-component symlink via `symlink_metadata`. Regression: `a_symlink_result_path_refuses`. |
| F11 | P2 | The End-to-end run was not bound to the rebuilt product. | Contract `product.path`/`product.env`; gate hashes the copy product, passes `GIVN_TRIAL_PRODUCT`/`GIVN_TRIAL_PRODUCT_SHA256`, and requires the result's product path and digest. |
| F12 | P2 | A count-only result could pass 10/10. | Both tiers require every declared scenario title; count-only and title-less results refuse. |
| F13 | P2 | A phrase-triggered `check tasks` gate did not cover `givn/trials` and wrote the receipt first. | The gate is removed; Givn states that external/manual reports are outside enforcement and only `verify-trial` writes a clean result. |
| F14 | P2 | Coverage omitted `GIVN_COVERAGE_OUTPUT_PATH`. | Contract `coverage.output_path`; the gate passes it. |
| F15 | P2 | Frozen inputs could change after the pre-run comparison. | The gate re-runs `compare` after the scripts and writes `clean: false` on drift. |
| F22 | P1 | A phrase-triggered `check tasks` gate missed `givn/trials` and wrote the receipt first. | The gate is removed; external/manual reports are outside Givn enforcement. |
| F23 | P2 | A single-file product could not bind Passuff's two binaries. | `product.path` accepts a directory; the digest covers every regular file; `PASSUFF_BIN_DIR` is supported. |
| F24 | P2 | Duplicate and outline titles collapsed. | Scenario evidence is a multiset, one title per execution unit. |
| F25 | P2 | A copy without the E2E package could not produce an E2E report. | A missing/0-line E2E report is accepted with `coverage.e2e_subprocess: true`. |
| F26 | P2 | Stale raw results and product changes were not fully fail-closed. | Removal fails closed; product is re-hashed; initial manifest/contract bytes are re-read. |
| F27 | P2 | The product env was only echoed by the fixture. | The Passuff-shaped fixture executes both rebuilt binaries and records the directory digest. |

## Verification

- `cargo test --test ise_component_tier_gate -- --test-threads=1` — 12 passed; `cargo test --test ise_passuff_trial -- --test-threads=1` — 2 passed.
- `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 9 passed.
- `cargo test --lib ise::` — 24 passed.
- `givn lint --change ise-component-tier-gate` — clean (only advisory overlap findings).
- `./run-tests.sh --non-e2e` — 1126 passed, 115 skipped.
- `./run-tests.sh --e2e --package e2e` — 120 passed, 785 steps.
- `GIVN_FEATURES=givn/specs/prepare-an-ise-test ./run-tests.sh --e2e --package e2e` — 2 passed.

## Re-pass

The reviewer re-ran the reserved-path probes, the result-path traversal, the broken-symlink probe, the acceptance test, the delta suite, and lint after each fix. The final pass reported no remaining findings.

DESIGN-REVIEW: PASS

## Final independent re-review

Reviewer context: DesignReviewGate (independent reviewer subagent)

This section is an independent re-review of the fixed final state, not a continuation of the earlier pass. The re-review read `proposal.md`, `design.md`, `arc42.md`,
`tasks.md`, the three delta features, the delta use case, `src/ise/contract.rs`,
`src/ise/verify.rs`, `src/ise/mod.rs`, `src/ise/compare.rs`, `src/ise/manifest.rs`,
`src/config/manifest.rs`, `src/cli/check.rs`, `src/cli/ise.rs`, `src/main.rs`,
`tests/ise_component_tier_gate.rs`, `tests/ise_passuff_trial.rs`, the delta step
bindings, `assets/commands.yaml.tmpl`, `assets/instructions/ise.md`,
`docs/reference.md`, the affected arc42 chapters, and the review records. It ran
the targeted acceptance, Passuff, ISE unit, and delta suites plus a temporary
end-to-end probe against the final `target/debug/givn`. The prior
`DESIGN-REVIEW: PASS` is superseded by this section.

### Findings

| ID | Severity | Kind | Finding and evidence | Closed? |
|---|---|---|---|---|
| F1 | P1 | Design defect | `verify-trial` treats the supplied manifest and generated contract as the authority for the declared corpus; it never re-derives `component.execution_units` or `component.scenarios` from the frozen behavior features in the copy or from the manifest revision. Probe `/tmp/ise-review-probe.iseKPI`: the copy's `givn/specs/demo.feature` has ten component scenarios; after setting `component.execution_units: 0` and `component.scenarios: []` in the copy's `givn/ise-verification.yaml` and replacing the manifest's `verification.checksum` with the matching SHA-256, `givn ise verify-trial` printed `Component scenarios: not applicable (0 declared)` and `Trial result: passed` with `clean: true`. `src/ise/verify.rs:337-364` checks only the contract checksum; `src/ise/verify.rs:387-389` returns `not_applicable` at declared zero; `src/ise/mod.rs:216-240` writes the contract but no verify path re-reads the feature corpus. The design's stated trust boundary covers an arbitrary script's self-report (review F27), not a preparer editing the manifest and contract. | Open in code and design. |
| F2 | P2 | Code defect | The contract reader does not enforce its own invariant that each tier carries one scenario title per execution unit, and `require_scenarios` returns `Ok` when the declared list is empty. Probe: with `component.execution_units: 10` and `component.scenarios: []` in the contract, a count-only component result (no `scenarios` field) printed `Component scenarios: 10/10 executed` and `Trial result: passed`; the record's `component.scenarios` is `[]`. `src/ise/contract.rs:157-197` validates only coverage and product paths; `src/ise/verify.rs:896-899` returns early on an empty declared list. This contradicts design D7 (`design.md:259-262`), review F24, and arc42 R-167. | Open. |
| F3 | P2 | Code defect | `Contract.revision` is deserialized but never consumed by `verify-trial`; the result records the manifest revision. Probe: the contract declared `revision: deadbeefdeadbeefdeadbeefdeadbeefdeadbeef` while the manifest declared `599215b9f22340357b398f973027a3d94f084591`; `verify-trial` still printed `Trial result: passed`, and the result recorded `599215...`. `src/ise/contract.rs:33`; `src/ise/verify.rs:337-364`; result revision at `src/ise/verify.rs:179` and successors. A contract from another revision can be paired with a manifest whose checksum was repaired. | Open. |
| F4 | P2 | Documentation/guidance defect | The canonical runner contract still documents the result as only `{"total", "passed", "failed", "skipped"}` and the adapter template still requires only those four counts; the trial gate requires `scenarios` for both tiers and `product`/`product_sha256` for End-to-end. Evidence: `docs/reference.md:253-263`; `assets/commands.yaml.tmpl:18-21`; the new schema at `src/ise/verify.rs:116-130` and `src/ise/verify.rs:964-989`. A project that follows the documented runner contract produces a count-only result, which the gate refuses when the generated contract carries titles; the documentation does not tell the project to emit the new evidence. | Open in documentation. |
| F5 | P2 | Artifact/specification inconsistency | The delta use case's normative product rule says the gate requires a regular file, refuses a non-regular file, and hashes its bytes (`givn/changes/ise-component-tier-gate/specs/prepare-an-ise-test/usecase.md:101`), while design D6 and the code support a regular file or directory with a tree digest (`design.md:75`, `design.md:255-258`; `src/ise/verify.rs:489-498`, `src/ise/verify.rs:515-560 and 592-627`). `docs/reference.md:778`, `docs/arc42/08-crosscutting-concepts.md:1475-1477`, and `docs/arc42/11-risks-and-technical-debt.md:153` also retain the file-only wording; `assets/instructions/ise.md:149-151` has the correct wording. A merge of this delta would leave the permanent use case contradicting the accepted directory behavior. | Open in artifacts and docs. |
| F6 | P3 | Test gap | No end-to-end test changes the product after binding. `tests/ise_component_tier_gate.rs` tests contract drift (`a_contract_change_during_the_scripts_refuses`) but not a product rewrite; `tests/ise_passuff_trial.rs`'s wrong-product test records a different digest rather than changing the bound product. `ProductBinding::ensure_unchanged` (`src/ise/verify.rs:489-498`) and the second post-coverage call (`src/ise/verify.rs:271-287`) are unexercised by an integration test, while design D6 and review F26 claim product re-hashing. | Open (test gap). |
| F7 | P3 | Test gap | No end-to-end test exercises duplicate scenario titles or Scenario Outline expansion through `verify-trial`; both fixtures use plain scenarios (`tests/ise_component_tier_gate.rs:80-119`; `tests/ise_passuff_trial.rs:100-130`). The multiset logic is covered only by `src/ise/contract.rs` and `src/ise/verify.rs` unit tests. Design D7 and review F24 claim the exact multiset is demonstrated; the claim is supported at unit level, not by a running gate. | Open (test gap). |
| F8 | P3 | Review/tasks evidence | `tasks.md:121-122` records `cargo test --lib ise::` as 19 passed; the current run is 24 passed. `design.md:213` lists only `tests/ise_component_tier_gate.rs` as the acceptance test and omits the new `tests/ise_passuff_trial.rs`; the final commit adds the latter. `review.md:21-33` still says it reviewed batch `ed411f57` and documentation-only revisions, while the final code is `94b1bb`; `git status` shows the receipt modified to head `3eaddcb` but not committed. The review records do not bind the final revision and the tasks count is stale. | Open in artifacts. |
| F9 | P3 | Refusal-semantics mismatch | The use case requires that a result declaring zero scenarios name `component_executed=<executed>/<declared>` (`givn/changes/ise-component-tier-gate/specs/prepare-an-ise-test/usecase.md:66`), but `read_counts` refuses with `the component runner result ... declares zero scenarios` (`src/ise/verify.rs:867-872`) and never names `component_executed=0/<declared>`. The missing-result path does name it; the zero-scenario path does not. | Open (minor). |

### Verification performed

- `cargo test --test ise_component_tier_gate -- --test-threads=1` — 12 passed.
- `cargo test --test ise_passuff_trial -- --test-threads=1` — 2 passed.
- `cargo test --lib ise::` — 24 passed (not the 19 recorded in `tasks.md`).
- `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 9 passed.
- `target/debug/givn check review --change ise-component-tier-gate` — exit 0 with the uncommitted receipt at `3eaddcb`; only `[SHAPE]`/`[SUBST]` advisories.
- Probe at `/tmp/ise-review-probe.iseKPI`: a valid preparation with ten component scenarios; the copy contract was edited to declare zero units and the manifest's `verification.checksum` was repaired; `givn ise verify-trial` exited 0 with `Component scenarios: not applicable (0 declared)`, `End-to-end tests: 1/1 executed`, and `Trial result: passed`. A second run kept `execution_units: 10` but emptied `component.scenarios`; a count-only component result passed with `Component scenarios: 10/10 executed`. A third run set the contract revision to `deadbeef...` while the manifest revision stayed `599215...`; the pass result recorded the manifest revision.

The probe demonstrates that the gate does not bind the declared corpus to the frozen behavior features or the manifest revision. The component script in the probe did not execute any Gherkin scenario; it only wrote counts. Even if one treats a runner's self-report as trusted, the contract reader still accepted a contract that contradicted its own data model.

### Verdict

FAIL. The targeted implementation and the Passuff-shaped fixture are real and green, and the removal of the phrase-triggered `check tasks` gate is correctly reflected in the served guidance. However, the central claim is not met: a manifest/contract pair prepared or edited by the builder can declare zero component units (or declare units with no titles) while the frozen behavior corpus remains in the copy, and `givn ise verify-trial` prints `Trial result: passed`. The gate must re-derive the declared counts and titles from the frozen behavior corpus or the manifest revision and refuse a mismatch (or require the verifier to prepare the manifest itself and state the manifest/contract as a trusted input), and the contract reader must reject `execution_units != scenarios.len()` and a contract/manifest revision mismatch. The prior `DESIGN-REVIEW: PASS` is not accepted; `review.md` is left unchanged because the blocking findings are design/code findings, not changes to the prior review record, and it must be rerun after the fixes.

DESIGN-REVIEW: FAIL

## Final independent re-review closure (fixed state)

Reviewer context: DesignReviewGate (independent reviewer subagent)

This closure re-checks the fixes in `451aa128` and `cdcb7fb0`, against the
remaining findings from the previous final re-review. The targeted suites and
the original probes were rerun against the final `target/debug/givn`; the
temporary probes remain outside the repository.

### Closure evidence for F1-F9

| ID | Previous finding | Closure evidence | Status |
|---|---|---|---|
| F1 | A contract edited to hide the frozen corpus passed | Original probe `/tmp/ise-review-probe.iseKPI` with `component.execution_units: 0` and a repaired manifest checksum now exits 1: `the verification contract declares 0 component scenario unit(s), but the frozen features declare 10; the contract does not describe the frozen corpus`. New test `a_contract_that_hides_the_frozen_corpus_refuses` passes. | Closed. |
| F2 | `execution_units` did not have to match the title list; a count-only result passed | Original probe with `component.execution_units: 10` and `component.scenarios: []` now exits 1: `the verification contract's component declares 10 execution unit(s) but 0 scenario title(s); the counts must match the corpus`. `Contract::parse` enforces the invariant; test `a_contract_count_must_match_its_scenario_titles` passes. | Closed. |
| F3 | The contract revision was ignored | Original probe with contract revision `deadbeef...` and manifest revision `599215...` now exits 1: `the verification contract records revision 'deadbeef...' but the manifest records '599215...'; the contract must describe the frozen revision`. Test `a_contract_from_another_revision_refuses` passes. | Closed. |
| F4 | The canonical runner contract omitted `scenarios`/`product`/`product_sha256` | `docs/reference.md` and `assets/commands.yaml.tmpl` now document the additional result fields; the served `ise` instruction repeats the trial evidence. | Closed. |
| F5 | The delta use case and docs described the product as a file only | The delta use case, glossary, `docs/reference.md`, arc42 06/08/11, and the served instruction now state a regular file or directory with the tree digest. | Closed. |
| F6 | No end-to-end test changed the product after binding | `a_product_changed_after_binding_refuses` passes: the E2E script mutates the product after writing its result, and the gate refuses with `clean: false`. The post-coverage re-hash uses the same guard; it is not separately mutated by a test. | Closed for the binding claim; the post-coverage mutation path remains unexercised. |
| F7 | No end-to-end test exercised duplicate titles or outline rows | `duplicate_titles_and_outline_rows_survive_the_running_gate` passes with four units (`dup`, `dup`, `outline`, `outline`) and records the exact multiset; the collapsed-result refusal remains a unit test. | Closed. |
| F8 | Tasks and review counts were stale | `tasks.md` now records 25 ISE unit tests and 16 acceptance tests. `review.md` still records 12 acceptance tests and batch `ed411f57`; `design.md:213` still names only the component acceptance test. The new `ise_passuff_trial` test is not in the design's test-runner list. | **Partially closed:** `tasks.md` is current; `review.md` and the design test plan remain stale. |
| F9 | A zero-scenario result did not name `component_executed` | Probe with a component runner writing `total: 0` now exits 1 with `the component runner result ... declares zero scenarios (component_executed=0/10)` and writes `clean: false`. | Closed. |

### Remaining findings

| ID | Severity | Kind | Finding and evidence | Closed? |
|---|---|---|---|---|
| F10 | P2 | Code defect and test gap | `contract::counts_for_feature_files` re-derives the corpus from every `.feature` path in the manifest's `specification_files`, not from the behavior role. `Contract::build` still counts `partition::behavior_features` (`src/ise/contract.rs:243-248`), while `counts_for_feature_files` (`src/ise/contract.rs:252-268`) reads all specification `.feature` files and `verify_trial` (`src/ise/verify.rs:156-167`) compares the two. Probe `/tmp/ise-review-mixed.KtdPZk`: a project with `behavior: ["givn/specs/**"]` and `contracts: ["docs/**"]`, plus `docs/example.feature` holding one Scenario, builds a contract with 10 component units; `givn ise verify-trial` exits 1 with `the verification contract declares 10 component scenario unit(s), but the frozen features declare 11; the contract does not describe the frozen corpus`. This is an honest trial refused because a non-behavior Specification file is counted as a component unit. The delta use case says the declared counts are read from the behavior feature files (`.../specs/prepare-an-ise-test/usecase.md:107`). | Open. Fix: pass the copy's `givn/specification.yaml` declaration to `counts_for_feature_files` and count only `partition::behavior_features`; add a test with a `.feature` file under `contracts`/`structure`. |
| F8 | P3 | Review/design evidence | `review.md` still records 12 acceptance tests and batch `ed411f57`; `design.md:213` still names only the component acceptance test and omits `tests/ise_passuff_trial.rs`; the receipt is stale for the final content (`givn check review` reports tested content changed from `0aa2e2...` to `8645e2...`). `tasks.md` is current at 25 unit tests and 16 acceptance tests. | Open. |

### Verification performed

- `cargo test --test ise_component_tier_gate -- --test-threads=1` — 16 passed.
- `cargo test --test ise_passuff_trial -- --test-threads=1` — 2 passed.
- `cargo test --lib ise::` — 25 passed.
- `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 9 passed.
- Original probes at `/tmp/ise-review-probe.iseKPI`: hidden-corpus contract, empty-title contract, and cross-revision contract all refuse; the zero-scenario probe names `component_executed=0/10`.
- Probe at `/tmp/ise-review-mixed.KtdPZk`: the non-behavior `.feature` false refusal described in F10.
- `givn check review --change ise-component-tier-gate` currently reports the receipt stale for the final content and a review-context mismatch introduced by the previous fail section; the closure section reuses `DesignReviewGate (independent reviewer subagent)` so the context is consistent. The receipt must be refreshed after the final edit; `review.md` still needs its counts and scope updated.

### Verdict

FAIL. F1-F7 and F9 are closed by the implementation and the rerun probes. F8 remains a review-evidence gap in `review.md` and the design test plan. F10 is a new P2 correctness defect: the re-derivation counts every specification `.feature` file as a component or End-to-end unit, while the contract is generated from the behavior role. An honest trial with a non-behavior `.feature` under `contracts` or `structure` refuses. The gate must select the same behavior feature set for the re-derivation as `Contract::build`.

DESIGN-REVIEW: FAIL

## Final independent re-review closure (F10 and F8)

This section supersedes the remaining F10 and F8 rows in the previous closure
section. The code was re-checked at `7008911c`, the mixed-role probe was rerun
against the final `target/debug/givn`, and the targeted suites were rerun.

### Closure evidence

- **F10 closed.** `verify_trial` now loads the copy's
  `givn/specification.yaml` through `SpecificationDeclaration::load`, selects
  `partition::behavior_features`, and passes only that set to
  `counts_for_feature_files` (`src/ise/verify.rs:155-172`). The new test
  `a_non_behavior_feature_does_not_enter_the_component_corpus` passes. The
  mixed-role probe at `/tmp/ise-review-mixed.KtdPZk` — `behavior:
  ["givn/specs/**"]`, `contracts: ["docs/**"]`, and a `docs/example.feature`
  holding one Scenario — now exits 0, reports `Component scenarios: 10/10
  executed`, and records `clean: true` instead of the previous 10-vs-11
  refusal.
- **F8 closed for the review, tasks, and receipt evidence.** `tasks.md` records
  17 acceptance tests and 25 ISE unit tests; `review.md` records 17 acceptance
  tests and batch `7008911c`; the task receipt is committed at `610037` with
  head `3bd79d0c`; `givn check review` was green at that revision before this
  closure edit. The design's test-runner section still names only
  `tests/ise_component_tier_gate.rs` as the acceptance command; the Passuff
  test is recorded in `review.md` and `tasks.md`. This is a non-blocking
  documentation omission, not a correctness defect.

### Verification performed

- `cargo test --test ise_component_tier_gate -- --test-threads=1` — 17 passed,
  including `a_non_behavior_feature_does_not_enter_the_component_corpus`.
- `cargo test --test ise_passuff_trial -- --test-threads=1` — 2 passed.
- `cargo test --lib ise::` — 25 passed.
- `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 9 passed.
- Mixed-role probe at `/tmp/ise-review-mixed.KtdPZk` — exit 0, 10/10
  component units, 1/1 End-to-end, `clean: true`.
- The hidden-corpus, empty-title, and cross-revision probes at
  `/tmp/ise-review-probe.iseKPI` were closed by the previous revision and
  remain covered by the 17 acceptance tests; the zero-scenario probe names
  `component_executed=0/10`.
- The full suites at `7008911` were reported separately as non-e2e 1126 passed
  and e2e 120 passed, 785 steps; they were not rerun here.

This closure edit changes the change-tree content, so the committed receipt at
`610037` must be refreshed (`givn check tasks`) after the edit is committed;
the review gate was green at `610037` before this section was added.

DESIGN-REVIEW: PASS
