Review kind: independent
Author: DeepSeek Flash execution agent
Reviewer context: DesignReviewGate (independent reviewer subagent)

# Design review: ise-component-tier-gate

## Grilling branches

| Branch | Outcome |
|---|---|
| Scope | The proposal, delta specs, and design cover the generated contract, the read-only comparison label, the exact-count trial gate, coverage, and the not-run rule. No extra command or result field is present. |
| Tech choices | Rust modules in `src/ise` reuse the existing `RunnerResult`, Cobertura parser, subprocess seam, and SCM seam. A separate `verify-trial` command is the only executable path; `compare` stays read-only. |
| Missing scenarios | The delta covers contract generation, input-only comparison, missing runner, exact counts, exact mismatch, and the not-run bypass. Coverage edges are covered by unit tests; the 0-line e2e declaration is a unit test because the fixture would need a full Cobertura run. |
| Testability | Every delta scenario drives the real binary through a real Git fixture. Strict mode is `.fail_on_skipped()`. The single-scenario command is `GIVN_FEATURES=... cargo test --locked --test features_runner -- --exact "<usecase>/<capability>.feature::<title>"`. A not-implemented body uses `unimplemented!()`. |
| E2E fidelity | The verifier is a CLI; the gate drives the real binary and real `sh` commands. The existing `@e2e` compare scenario remains the one real-interface entry for the verifier action; `verify-trial` is a variant of that action, so no second inventory row is added. |
| Interaction Coverage | The `prepare-an-ise-test` inventory row for `trial-input-verification` now names the read-only comparison and the trial verification variants; the existing `Verifier compares the spec copy with the manifest` `@e2e` scenario remains the real-interface evidence. |
| Risk | The highest risk is a runner whose scenario accounting differs from the expanded Gherkin count. The exact refusal names `component_executed=<executed>/<declared>`, so the runner is corrected rather than the gate weakened. |
| Use-case and Persona context | The owning use case is `prepare-an-ise-test`; Personas are `developer--primary-user`, `agent--technical-role`, and `givn-maintainer--owner`. The terminal contract names `developer--primary-user`. |
| Decision ledger | `D1` exact counts, `D2` read-only compare plus separate gate, `D3` `verify-trial` name, `D4` expanded execution units, and `D5` `coverage.e2e_subprocess` each resolve in the design's Decisions ledger. `Ise rebuild decision: No rebuild` is recorded with no `Rebuilt code` line. |
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

## Verification

- `cargo test --test ise_component_tier_gate -- --test-threads=1` — 8 passed.
- `GIVN_FEATURES=givn/changes/ise-component-tier-gate/specs ./run-tests.sh --non-e2e` — 7 passed.
- `cargo test --lib ise::` — 18 passed.
- `givn lint --change ise-component-tier-gate` — clean (only advisory overlap findings).
- `./run-tests.sh --non-e2e` — 1126 passed, 115 skipped.
- `./run-tests.sh --e2e --package e2e` — 120 passed, 785 steps.
- `GIVN_FEATURES=givn/specs/prepare-an-ise-test ./run-tests.sh --e2e --package e2e` — 2 passed.

## Re-pass

The reviewer re-ran the reserved-path probes, the result-path traversal, the broken-symlink probe, the acceptance test, the delta suite, and lint after each fix. The final pass reported no remaining findings.

DESIGN-REVIEW: PASS
