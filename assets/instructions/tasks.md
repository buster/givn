# Instruction: tasks

Write the implementation task breakdown using a strict scenario-by-scenario
RED→GREEN→REFACTOR loop.

**First task: test infrastructure setup**

Before any scenario task, include a setup task that:
- Installs and configures the Gherkin/Cucumber runner chosen in design.md.
  The runner MUST execute the `.feature` files under `givn/specs/` (and the
  change's `specs/` during the change) — a bare unit-test runner is NOT
  sufficient (see the executable-spec invariant in `design.md`).
- Sets `verify.command` in `givn/config.yaml` to the exact runner invocation
  (e.g. `cargo test --test features_runner`). If `verify.command` is still
  the `givn missing-testrunner` sentinel, running it prints setup guidance —
  follow that guidance.
- Creates the Gherkin runner test binary (if not already present) and the
  step-definition module skeleton that the runner auto-discovers. Step
  definitions are shared glue — never a parallel hand-maintained test file.
- Verifies the runner exits 0 before any RED phase begins.

**The TDD loop (one scenario at a time, in .feature order):**

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions WITH real assertions (Then asserts concrete values).
            The Gherkin runner MUST execute the `.feature` file and bind the
              steps to these definitions — never a parallel `#[test]` fn.
            Run the test runner → this scenario MUST FAIL (no production code yet).
            If it passes here, the assertions are not real — fix the steps first.
  GREEN:    Write the minimum production code → run runner → this scenario PASSES.
  REFACTOR: Clean up without changing behaviour → runner still PASSES.
Then move to the next scenario.
```

**Important distinctions:**
- `givn lint` is a static Gherkin check (syntax, @wip, tags). It runs NO tests.
- The test runner (`verify.command` from givn/config.yaml) is what confirms GREEN.
- Never conflate the two.

**Tasks file format:**
- Use `[ ]` / `[x]` checkboxes.
- One section per scenario: RED / GREEN / REFACTOR sub-tasks.
- `givn apply` reads these checkboxes to track progress.
- Done when: all checkboxes checked AND the test runner exits 0.
