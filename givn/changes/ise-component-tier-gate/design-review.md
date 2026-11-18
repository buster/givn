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
