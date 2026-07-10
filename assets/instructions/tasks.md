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
For each non-@e2e scenario:
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

**After all non-@e2e scenarios are GREEN: the e2e smoke test loop.**

E2e scenarios (`@e2e`) come last because they test the real interaction
surface and depend on production code already being in place.

```
For each @e2e scenario:
  RED:      Remove @wip from THIS scenario only.
            Write e2e step definitions (in the e2e step location from design.md)
              WITH real assertions that drive the actual interface (browser, CLI, API).
            Run the e2e runner (verify.e2e_command) → MUST FAIL.
  GREEN:    Set up test infrastructure if needed (start server, launch browser,
            seed database). Write the minimum e2e glue to pass.
            Run the e2e runner → this scenario PASSES.
  REFACTOR: Clean up e2e infrastructure code. Runner still PASSES.
Then move to the next @e2e scenario.
```

**Important distinctions:**
- `givn lint` is a static Gherkin check (syntax, @wip, tags). It runs NO tests.
- The test runner (`verify.command` from givn/config.yaml) is what confirms
  GREEN for non-@e2e scenarios.
- The e2e runner (`verify.e2e_command` from givn/config.yaml) is what confirms
  GREEN for @e2e scenarios.
- Never conflate lint, verify.command, or verify.e2e_command.

**E2e infrastructure setup task:**

Before the first `@e2e` scenario, include a setup task that:
- Installs and configures the e2e test framework chosen in design.md (e.g.
  Playwright, a Cucumber runner with e2e tag filtering, Testcontainers).
- Creates the e2e step definition skeleton in the e2e step location.
- Sets `verify.e2e_command` in `givn/config.yaml` to the e2e runner invocation.
- Documents how to start/stop the local test environment (application server,
  database, fake services) if not handled automatically by the test harness.

**Tasks file format:**
- Use `[ ]` / `[x]` checkboxes.
- One section per scenario: RED / GREEN / REFACTOR sub-tasks.
- Non-@e2e scenarios come first, @e2e scenarios come last.
- `givn apply` reads these checkboxes to track progress.
- Done when: all checkboxes checked AND both `verify.command` and
  `verify.e2e_command` exit 0.
