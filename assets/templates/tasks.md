# Tasks: {{CHANGE_ID}}

RED→GREEN→REFACTOR→COMMIT per scenario. Non-@e2e first, then @e2e.

Legend: `[ ]` pending · `[x]` done only when evidence + commit hash are filled in. Check boxes immediately after each scenario — tasks.md must always reflect true state.

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

## Domain Constraints

> Copy the owning use-case guarantees, rules, confirmed Persona constraints,
> and relevant ideation decisions here before listing scenario tasks.

| Scenario / capability | Use-case guarantee or rule | Persona impact |
|---|---|---|
| | | |

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
  - [ ] COMMIT: `<hash>` — `<message>`

<!-- One section per non-@e2e scenario. -->

---

## E2E Smoke Tests

- [ ] Bring up the local environment (design.md's Local Runnability
      section): local run command, all dependencies, all digital twins for
      external services, isolated network. Confirm clean startup.
      ```
      <!-- paste local-run command + confirmation of startup -->
      ```
- [ ] Set up e2e test infrastructure and step definitions (one file per
      capability, separate location). Configure and prove strict mode for
      the e2e runner. Set `verify.e2e_command`. Confirm the e2e runner can
      reach the running local environment.
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
        assertions driving the actual interface (primary) — repository
        checks are secondary/additional only, never the sole assertion.
        For a browser-UI capability, drive the actual browser (click/type/
        navigate) — never an HTTP request or in-page `fetch()` standing in
        for browser interaction. If a real-interface obstacle appears, fix
        it per design.md's Anticipated Interface Obstacles section — do not
        substitute a repository-only or HTTP/fetch() shortcut.
        Files: <!-- list them -->
        Run targeting this scenario only. Must pass.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up. Re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` — `<message>`

<!-- One section per @e2e scenario. -->

---

**Done when:** every box checked with evidence and commit hash, both
`{{VERIFY_COMMAND}}` and `verify.e2e_command` exit 0 on the full suite.
