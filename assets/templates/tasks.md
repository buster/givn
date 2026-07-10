# Tasks: {{CHANGE_ID}}

Implement each scenario from the spec, one at a time, following strict RED→GREEN→REFACTOR→COMMIT.
Non-@e2e scenarios first, then @e2e smoke tests last.

Legend: `[ ]` pending · `[x]` done (only when evidence is pasted in and a commit hash is recorded).

---

## Strict-mode proof (mandatory, do this first)

Empty/no-op step definitions are indistinguishable from passing tests in most
Cucumber runners. Before any scenario work, prove the runner fails on an
undefined/unimplemented step.

- [ ] **Proof-of-strictness**: run the configured runner against a step using
      the language's "not implemented" stub (see design.md). Confirm non-zero
      exit. Paste the command and output below.

```
<!-- paste proof-of-strictness command + output here -->
```

If the runner exited 0, configure its strict-mode flag (see design.md) before
proceeding — do not continue with an unproven runner.

## Scenario-by-scenario TDD plan

For each non-@e2e scenario:

1. **RED**: Remove `@wip` from THIS scenario only. Write step definitions —
   any step without a real assertion yet uses the language's "not
   implemented" stub (never an empty body). Run `{{VERIFY_COMMAND}}`,
   targeting this scenario only — MUST FAIL. Paste the output as evidence.
2. **GREEN**: Replace stubs with real assertions. Write the minimum
   production code. List every production file created/modified. Run
   `{{VERIFY_COMMAND}}`, targeting this scenario only — PASSES. Paste output.
3. **REFACTOR**: Clean up without changing behaviour. Runner still PASSES
   (re-run, paste output).
4. **COMMIT**: One atomic commit for RED+GREEN+REFACTOR, message references
   the scenario title. Record the commit hash.

Then move to the next scenario.

## E2E smoke test plan

After all non-@e2e scenarios are GREEN, implement the @e2e scenarios.
These exercise the real interaction surface (browser, CLI, API) and use the
e2e runner (`verify.e2e_command`).

For each @e2e scenario:

1. **RED**: Remove `@wip`. Write e2e step definitions (in the e2e step location
   from design.md) — unimplemented steps use the language's stub pattern.
   Run e2e runner, targeting this scenario only — MUST FAIL. Paste evidence.
2. **GREEN**: Set up test infrastructure if needed. Replace stubs with real
   assertions driving the actual interface. List files created/modified.
   Run e2e runner, targeting this scenario only — PASSES. Paste evidence.
3. **REFACTOR**: Clean up e2e code. Runner still PASSES (paste evidence).
4. **COMMIT**: One atomic commit, message references the scenario title.
   Record the commit hash.

---

## Tasks

- [ ] Set up step definition file(s) (one file per capability) and test
      runner configuration in strict mode. Prove strictness (see above).
- [ ] Scenario: `{{FIRST_SCENARIO_TITLE}}`
  - [ ] RED: write steps (stubs for unimplemented ones); runner fails.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: write minimum production code; runner passes.
        Files created/modified: <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up; runner still passes.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: <!-- commit hash -->

<!-- Add one section per non-@e2e scenario from the spec. -->

---

## E2E Smoke Tests

- [ ] Set up e2e test infrastructure and e2e step definition skeleton (one
      file per capability). Configure strict mode and prove it for the e2e
      runner specifically.
      ```
      <!-- paste e2e proof-of-strictness command + output -->
      ```
- [ ] Set `verify.e2e_command` in `givn/config.yaml`.
- [ ] @e2e Scenario: `{{FIRST_E2E_SCENARIO_TITLE}}`
  - [ ] RED: write e2e steps (stubs for unimplemented ones); e2e runner fails.
        ```
        <!-- paste RED evidence -->
        ```
  - [ ] GREEN: implement e2e glue and infrastructure; e2e runner passes.
        Files created/modified: <!-- list them -->
        ```
        <!-- paste GREEN evidence -->
        ```
  - [ ] REFACTOR: clean up; e2e runner still passes.
        ```
        <!-- paste REFACTOR evidence -->
        ```
  - [ ] COMMIT: <!-- commit hash -->

<!-- Add one section per @e2e scenario from the spec. -->

---

**Done when:** all checkboxes checked with evidence and commit hashes recorded,
`{{VERIFY_COMMAND}}` exits 0 on the full suite, and `verify.e2e_command` exits 0
on the full suite.
