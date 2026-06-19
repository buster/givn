# Instruction: tasks

Write the implementation task breakdown using a strict scenario-by-scenario
RED→GREEN→REFACTOR loop.

**First task: test infrastructure setup**

Before any scenario task, include a setup task that:
- Installs and configures the test runner chosen in design.md
- Sets `verify.command` in `givn/config.yaml` if still the sentinel
- Verifies the runner exits 0 before any RED phase begins

**The TDD loop (one scenario at a time, in .feature order):**

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions WITH real assertions (Then asserts concrete values).
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
