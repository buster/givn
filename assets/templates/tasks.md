# Tasks: {{CHANGE_ID}}

Implement each scenario from the spec, one at a time, following strict RED→GREEN→REFACTOR.

Legend: `[ ]` pending · `[x]` done.

---

## Scenario-by-scenario TDD plan

For each scenario:

1. **RED**: Remove `@wip` from THIS scenario only. Write step definitions with real
   assertions. Run `{{VERIFY_COMMAND}}` — this scenario MUST FAIL (no production code yet).
   If it passes, the assertions are not real — fix the steps first.
2. **GREEN**: Write the minimum production code. Run `{{VERIFY_COMMAND}}` — scenario PASSES.
3. **REFACTOR**: Clean up without changing behaviour. Runner still PASSES.

Then move to the next scenario.

---

## Tasks

- [ ] Set up step definition file(s) and test runner configuration.
- [ ] Scenario: `{{FIRST_SCENARIO_TITLE}}`
  - [ ] RED: write steps with real assertions; runner fails.
  - [ ] GREEN: write minimum production code; runner passes.
  - [ ] REFACTOR: clean up; runner still passes.

<!-- Add one section per scenario from the spec. -->

---

**Done when:** all checkboxes checked and `{{VERIFY_COMMAND}}` exits 0.
