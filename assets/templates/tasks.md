# Tasks: {{CHANGE_ID}}

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in.

---

## Strict-mode proof (do first)

Empty/no-op steps are indistinguishable from PASS in most runners. Prove the
runner rejects an unimplemented step before any scenario work.

- [ ] **Proof-of-strictness**: run the runner against a step using the
      not-implemented stub (see design.md). Confirm non-zero exit.
      ```
      <!-- paste command + output -->
      ```
      If exit 0, configure the strict-mode flag (design.md) before proceeding.

---

## Tasks

- [ ] Set up step definitions (one file per capability) and configure the
      runner in strict mode. Proof-of-strictness above must be filled in first.
- [ ] Scenario: `{{FIRST_SCENARIO_TITLE}}`
  - [ ] RED: unimplemented steps use the not-implemented stub, never empty.
        Run `{{VERIFY_COMMAND}}` targeting this scenario only. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: replace stubs with real assertions; write minimum production
        code. Files created/modified: <!-- list them -->
        Run targeting this scenario only. Must pass.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: one commit for RED+GREEN+REFACTOR, message references the
        scenario title. Hash: <!-- commit hash -->

<!-- One section per non-@e2e scenario. -->

---

## E2E Smoke Tests

- [ ] Set up e2e test infrastructure and step definitions (one file per
      capability, separate location). Configure and prove strict mode for
      the e2e runner. Set `verify.e2e_command`.
      ```
      <!-- paste e2e proof-of-strictness command + output -->
      ```
- [ ] @e2e Scenario: `{{FIRST_E2E_SCENARIO_TITLE}}`
  - [ ] RED: unimplemented steps use the not-implemented stub. Run
        `verify.e2e_command` targeting this scenario only. Must fail.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: set up test infrastructure if needed; replace stubs with real
        assertions driving the actual interface. Files: <!-- list them -->
        Run targeting this scenario only. Must pass.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up. Re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: one commit, message references the scenario title.
        Hash: <!-- commit hash -->

<!-- One section per @e2e scenario. -->

---

**Done when:** every box checked with evidence and commit hash, both
`{{VERIFY_COMMAND}}` and `verify.e2e_command` exit 0 on the full suite.
