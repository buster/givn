# Instruction: tasks

Write the implementation task breakdown using a strict scenario-by-scenario
RED→GREEN→REFACTOR loop.

**First task: test infrastructure setup**

Before any scenario task, include a setup task that:
- Installs and configures the Gherkin/Cucumber runner chosen in design.md.
  The runner MUST execute the `.feature` files under `givn/specs/` (and the
  change's `specs/` during the change) — a bare unit-test runner is NOT
  sufficient (see the executable-spec invariant in `design.md`).
- Configures the runner in **strict mode** (undefined/pending steps FAIL the
  build, they do not silently pass or skip). See "Strict mode is mandatory"
  below — this is not optional.
- Sets `verify.command` in `givn/config.yaml` to the exact runner invocation
  (e.g. `cargo test --test features_runner`). If `verify.command` is still
  the `givn missing-testrunner` sentinel, running it prints setup guidance —
  follow that guidance.
- Creates the Gherkin runner test binary (if not already present) and the
  step-definition module skeleton that the runner auto-discovers. Step
  definitions are shared glue — never a parallel hand-maintained test file.
  Organise step definitions **one file per capability** (e.g.
  `CompanyOnboardingSteps`, `AuthenticationSteps`) — never one file holding
  every step for the whole change. A single giant step file is a red flag: it
  makes empty/no-op stubs easy to hide.
- **Proves the runner is strict** (mandatory, see below) before any RED phase
  begins.

**Strict mode is mandatory — prove it once, in the setup task.**

The single failure mode that makes GREEN meaningless is a runner that treats
an empty or unimplemented step as a PASS. This has happened in practice:
every scenario body was left as an empty stub, `@wip` was removed from all of
them, and the runner reported 100% GREEN because empty methods and
"undefined step" results are not failures by default in most Cucumber
implementations.

The setup task MUST include a **proof-of-strictness check**:
1. Write one throwaway scenario (or reuse the first real scenario) with a
   step that has NO definition at all, or a step definition using the
   language's "not implemented" primitive (see the stub patterns below).
2. Run the runner.
3. Confirm the runner **exits non-zero** for that undefined/pending step.
4. If it exits 0 (the runner silently passes undefined/pending steps),
   configure the runner's strict flag before continuing:
   - `cucumber-js`: `--strict` (or `strict: true` in `cucumber.js` config —
     default is strict in modern versions, but confirm).
   - `behave` (Python): pending steps raise by default; if using
     `@step(...)` decorators with empty bodies, `pass` is NOT a failure —
     the stub itself must raise (see stub patterns below). Consider
     `behave --tags=~@wip --junit` and inspect the report for skipped/passed
     counts.
   - `cucumber-jvm` (Java): pass `--strict` on the CLI runner, or for the
     JUnit Platform Suite runner set
     `@ConfigurationParameter(key = Constants.PLUGIN_PROPERTY_NAME, value = "...")`
     with a plugin that fails on undefined/pending, AND make every step
     definition explicitly `throw new io.cucumber.java.PendingException(...)`
     during RED (never an empty `{}` body — see stub patterns below). An
     empty method body in Java/Cucumber is a **pass**, not a pending/failing
     result — this is the exact defect that must never recur.
   - `cucumber-rs`: `.fail_on_skipped()` on the `Cucumber` runner builder
     (already the default pattern used by this project).
5. Record the proof-of-strictness result in the task itself: paste the
   command run and the confirmation that it failed/exited non-zero.

Only after strict mode is proven does the setup task's final check apply:
run the full suite (all scenarios still `@wip`) → must exit 0 (nothing to
run yet is not the same as "everything silently passed").

**Never write an empty step definition body. Ever.**

A step definition with an empty body (`{}`, `pass`, `# TODO`, a bare
`return`) is **indistinguishable from a passing test** in most Cucumber
runners. This is banned outright — not just during RED, but at every phase.
If a step is not yet implemented, its body MUST actively fail using the
language's canonical "not implemented" primitive:

| Language / framework | RED-phase stub (step body content) |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO: implement");` |
| Python (`behave`) | `raise NotImplementedError("TODO: implement")` |
| JavaScript/TS (`cucumber-js`) | `return "pending";` (Cucumber-js's own pending marker) or `throw new Error("TODO: implement")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO: implement")` |

A step is allowed to have a body with no explicit stub call **only** when
it already contains real, concrete assertions (the GREEN state) or when it
is a genuine reused `Given` step with no assertion by design (e.g. pure
setup) — and even then it must perform the setup action, not merely exist.
"Reused with no assertion" is never an excuse for a brand-new step to be
empty.

**The TDD loop (one scenario at a time, in .feature order):**

```
For each non-@e2e scenario:
  RED:      Remove @wip from THIS scenario only.
            Write step definitions using the RED-phase stub pattern for any
              step with no real assertion yet (see table above — never an
              empty body).
            The Gherkin runner MUST execute the `.feature` file and bind the
              steps to these definitions — never a parallel `#[test]` fn.
            Run the test runner, TARGETING THIS SCENARIO ONLY (by name, line
              number, or tag — never rely on reading whole-suite output to
              spot one scenario's result).
            CAPTURE the runner's output for this scenario as evidence.
            This scenario MUST FAIL — either a real assertion fails, or the
              stub's PendingException/NotImplementedError/unimplemented!()
              propagates as a failure.
            If it exits 0, the runner is not strict, or a step was left
              empty — fix before continuing. This is a hard stop.
  GREEN:    Replace stub bodies with real, concrete assertions (Then checks
              an actual value, not just that code ran).
            Write the minimum production code to make this scenario pass.
            List every production source file created or modified for this
              scenario. If GREEN required zero new/changed production files
              and the scenario is not a pure step-reuse case, that is a red
              flag — the "pass" is very likely fake. Investigate before
              proceeding.
            For compiled/typed languages: run the build/compile step
              (e.g. `./gradlew compileJava`, `cargo build`, `tsc --noEmit`)
              and confirm it succeeds BEFORE running the test runner.
            Run the test runner, targeting this scenario only → PASSES.
            CAPTURE the runner's output for this scenario as evidence.
  REFACTOR: Clean up without changing behaviour → runner still PASSES
              (re-run targeting this scenario, capture evidence).
  COMMIT:   Create one atomic commit for this scenario's RED+GREEN+REFACTOR
              work. Commit message references the scenario title verbatim,
              e.g. "feat(<capability>): <scenario title>". This makes every
              claimed GREEN independently auditable via `git show` —
              a checked task box with no matching commit, or a commit with
              no production-code diff, is proof the task was not actually done.
Then move to the next scenario.
```

**After all non-@e2e scenarios are GREEN: the e2e smoke test loop.**

E2e scenarios (`@e2e`) come last because they test the real interaction
surface and depend on production code already being in place.

```
For each @e2e scenario:
  RED:      Remove @wip from THIS scenario only.
            Write e2e step definitions (in the e2e step location from design.md)
              using the RED-phase stub pattern for unimplemented steps.
            Run the e2e runner (verify.e2e_command), targeting this scenario
              only. CAPTURE the failure output as evidence. MUST FAIL.
  GREEN:    Set up test infrastructure if needed (start server, launch browser,
            seed database). Replace stubs with real assertions that drive the
            actual interface. Write the minimum e2e glue to pass.
            List every file created/modified (glue code and, if needed,
              production code).
            Run the e2e runner, targeting this scenario only → PASSES.
              CAPTURE the output as evidence.
  REFACTOR: Clean up e2e infrastructure code. Runner still PASSES (capture
              evidence).
  COMMIT:   One atomic commit for this @e2e scenario, message referencing the
              scenario title, e.g. "test(e2e): <scenario title>".
Then move to the next @e2e scenario.
```

**Important distinctions:**
- `givn lint` is a static Gherkin check (syntax, @wip, tags). It runs NO tests.
- The test runner (`verify.command` from givn/config.yaml) is what confirms
  GREEN for non-@e2e scenarios — but ONLY once strict mode has been proven
  (see "Strict mode is mandatory" above). A non-strict runner reporting exit
  0 is not evidence of anything.
- The e2e runner (`verify.e2e_command` from givn/config.yaml) is what confirms
  GREEN for @e2e scenarios.
- Never conflate lint, verify.command, or verify.e2e_command.

**E2e infrastructure setup task:**

Before the first `@e2e` scenario, include a setup task that:
- Installs and configures the e2e test framework chosen in design.md (e.g.
  Playwright, a Cucumber runner with e2e tag filtering, Testcontainers).
- Creates the e2e step definition skeleton in the e2e step location,
  organised one file per capability (same rule as non-@e2e step definitions).
- Configures the e2e runner in strict mode and proves it (same
  proof-of-strictness check as the main setup task, run against the e2e
  runner specifically).
- Sets `verify.e2e_command` in `givn/config.yaml` to the e2e runner invocation.
- Documents how to start/stop the local test environment (application server,
  database, fake services) if not handled automatically by the test harness.

**Tasks file format:**
- Use `[ ]` / `[x]` checkboxes.
- One section per scenario: RED / GREEN / REFACTOR / COMMIT sub-tasks.
- Each RED/GREEN/REFACTOR sub-task has a place to paste the captured runner
  evidence (do not just describe it — paste the actual output).
- Each scenario section has a place for the commit hash once committed.
- Non-@e2e scenarios come first, @e2e scenarios come last.
- A checkbox may only be checked `[x]` once its evidence is pasted in and its
  commit hash is recorded. A checked box with empty evidence fields is
  incomplete, not done.
- `givn apply` reads these checkboxes to track progress.
- Done when: all checkboxes checked, every scenario has a commit, AND both
  `verify.command` and `verify.e2e_command` exit 0 on the full suite (not
  just scenario-by-scenario).
