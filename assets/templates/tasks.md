# Tasks: {{CHANGE_ID}}

Test-first behavior batches. Non-@e2e first, then @e2e. One commit may serve
several scenarios in a batch.

Legend: `[ ]` pending · `[x]` done only when evidence and a batch commit are
filled in. Check boxes immediately after each batch — tasks.md must always
reflect true state.

## Scenario revisions

Allowed:

- one test-first commit for a behavior batch;
- one implementation commit that satisfies several scenarios when each
  scenario has its own `TEST-FIRST:` evidence;
- a shared `COMMIT:` revision with a `Batch: <name>` label and a commit
  message that names the batch;
- a real revision that exists and is reachable from `HEAD`.

Not allowed:

- a placeholder, invented, or non-existent hash;
- a revision not reachable from `HEAD`;
- a checked scenario without `TEST-FIRST:` evidence and a `COMMIT:` line;
- a shared revision without a `Batch:` label;
- a commit message that names neither the scenario nor its batch;
- a comment-only, tag-only, task-only, or feature-only change presented as
  behavior evidence.

`givn check tasks` mechanically verifies the revision and batch label; review
checks the evidence and sensitivity.

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
  - [ ] TEST-FIRST: add the test and step glue before production; run against
        the current system. Record the test revision and result. A test that
        passes may be kept when it protects a distinct requirement.
        ```
        <!-- paste test-first evidence -->
        ```
  - [ ] IMPLEMENTATION: implement the behavior batch (or wire the real
        implementation). Files created/modified: <!-- list them -->
        Run the relevant scenarios. Must pass.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up, no behaviour change. Re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `<batch-name>`

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
  - [ ] TEST-FIRST: add the test and glue before production; run against the
        current system. Record the test revision and result. E2E asserts the
        real external interface, never an internal stub.
        ```
        <!-- paste test-first evidence -->
        ```
  - [ ] IMPLEMENTATION: set up test infrastructure if needed; wire the real
        interface. Primary assertion on the real interface; repository checks
        are secondary. For a browser UI, drive the actual browser. Files:
        <!-- list them -->
        Run the relevant scenarios. Must pass.
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up. Re-run, must still pass.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: `<hash>` -- Batch: `<batch-name>`

<!-- One section per @e2e scenario. -->

---

**Done when:** every box checked with test-first evidence and a batch commit,
both `{{VERIFY_COMMAND}}` and `verify.e2e_command` exit 0 on the full suite.
