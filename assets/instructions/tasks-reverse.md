# Instruction: tasks-reverse

Write the characterization task breakdown. Each task is a characterization
cycle — RED/GREEN/REFACTOR (same loop structure as normal TDD). See the
`givn-steps` skill for the shared step-writing mechanics; what differs for
characterization is described below.

**First task: test infrastructure setup (or verification)**

Before any characterization task, include a setup task that:
- If no Gherkin runner is configured: run `verify.command` — if it is still
  the `givn missing-testrunner` sentinel, its output prints setup guidance.
  Follow that guidance to install and configure a Gherkin/Cucumber runner
  that executes the `.feature` files under `givn/specs/` (and the change's
  `specs/` during the change). Configure the runner in **strict mode**
  (undefined/pending steps FAIL — see "Strict mode is mandatory" below). Set
  `verify.command` in `givn/config.yaml` to the exact runner invocation
  (e.g. `cargo test --test features_runner`).
  Create the Gherkin runner test binary (if not already present) and the
  step-definition module skeleton that the runner auto-discovers. Step
  definitions are shared glue — never a parallel hand-maintained test file.
  Organise step definitions one file per capability, never one file for the
  entire suite.
- If a Gherkin runner already exists (the codebase already has a runner
  wired to `.feature` files): verify it is already strict (run the
  proof-of-strictness check below regardless — do not assume). Configure
  step-definition locations for the new characterization scenarios. This
  task is verification, not setup.
- Either way: run the **proof-of-strictness check** (mandatory, see below)
  before characterization begins. Then verify the runner exits 0 (or exits
  with only @wip-related failures) on the current suite.

**Strict mode is mandatory — prove it once, in the setup task.**

A runner that treats an empty or unimplemented step as a PASS makes GREEN
meaningless — this has happened in practice, where every characterized step
was left an empty stub and the runner reported 100% GREEN. The setup task
MUST include a proof-of-strictness check:
1. Write or reuse a scenario with a step that has no definition, or a
   definition using the language's "not implemented" primitive (see the
   stub table below).
2. Run the runner and confirm it exits non-zero for that step.
3. If it exits 0, configure the runner's strict flag (e.g. `--strict` for
   `cucumber-js`/`cucumber-jvm`, `.fail_on_skipped()` for `cucumber-rs`)
   before continuing.
4. Record the proof (command + non-zero exit) in the task.

**Never write an empty step definition body.** An empty body (`{}`, `pass`,
a bare `return`) is indistinguishable from a passing test in most Cucumber
runners. Every step without a real assertion yet MUST use the language's
"not implemented" primitive:

| Language / framework | Stub (step body content) |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO: implement");` |
| Python (`behave`) | `raise NotImplementedError("TODO: implement")` |
| JavaScript/TS (`cucumber-js`) | `return "pending";` or `throw new Error("TODO: implement")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO: implement")` |

**The characterization loop (one scenario at a time, in .feature order):**

The `givn-steps` skill describes the shared RED/GREEN/REFACTOR loop with step
reuse. For characterization, the GREEN and REFACTOR phases are overlaid:

```
For each scenario:
  RED:      Remove @wip from THIS scenario only.
            Check for reusable step definitions first. New steps get the
              language's "not implemented" stub (see table above) — never
              an empty body.
            Run the test runner, TARGETING THIS SCENARIO ONLY. CAPTURE the
              output as evidence.
            Must fail (or pass immediately if ALL steps were reused from
              existing, already-proven definitions — record which steps
              were reused).
  GREEN:    Implement real assertions for new steps, probing OBSERVED
              behaviour (read the actual code). The Gherkin runner MUST
              execute the `.feature` file and bind the steps to these
              definitions — never a parallel `#[test]` fn.
            Run the test runner, targeting this scenario only → this
              scenario MUST PASS. CAPTURE the output as evidence.
            If it fails, do NOT change production code — diagnose:
              1. Infrastructure problem (most common): fix test plumbing only.
              2. Spec/implementation mismatch: correct the spec (or the
                 documentation that led to the wrong spec), never the code.
  REFACTOR: If the task's Seam block calls for a testability seam:
              Apply the seam (extract interface, DI, make public).
              Run the test runner → MUST still pass. CAPTURE evidence.
              If red: the refactor changed behaviour → REVERT the refactor.
            If no seam is needed for this scenario: skip REFACTOR.
  COMMIT:   One atomic commit per scenario (RED+GREEN+REFACTOR together),
              message referencing the scenario title, e.g.
              "characterize(<capability>): <scenario title>". A checked
              task box with no matching commit is proof the task was not
              actually done.
  Then move to the next scenario.
```

**Important distinctions:**
- `givn lint` is a static Gherkin check (syntax, @wip, tags). It runs NO tests.
- The test runner (`verify.command` from givn/config.yaml) is what confirms GREEN.
- GREEN means the test matches observed reality — NOT that the code is "correct".
- Never conflate the two.

**Spec/implementation mismatch diagnostic (when verify fails and infrastructure
is sound):**

The spec was generated by reading documentation and code. A mismatch means
either the documentation was wrong, or the code is ambiguous/misleading.

1. **Diagnose**: re-read the actual code. What does it REALLY do? Re-read the
   documentation that informed the spec (README, arc42, code comments,
   tasks.md).
   - (a) Documentation was wrong — docs said Y, code correctly does Z.
   - (b) Code is ambiguous/misleading — code does Z but reads like Y (includes
     bugs: code does Z when it should do Y, but we characterize AS-IS).

2. **Correct**:
   - If (a): update tasks.md and the .feature scenario to reflect actual
     behaviour. Update the source documentation that was wrong (README, arc42,
     code comments). Re-run verify.
   - If (b): update the .feature scenario to assert actual behaviour (AS-IS).
     Add a code comment in the language's comment syntax starting with
     `TODO: REVERSE-ENGINEERING NOTE: <what was misleading> — actual
     behaviour: <Z>. Code smell, refactor candidate.` Do NOT change code
     behaviour. Re-run verify. Flag in tasks/review as a code smell +
     follow-up candidate.

3. **NEVER**: change production code behaviour to match the spec, fix bugs
   discovered during characterization, or remove TODO: REVERSE-ENGINEERING
   NOTE markers.

**Tasks file format:**
- Use `[ ]` / `[x]` checkboxes.
- One section per scenario: RED / GREEN sub-tasks (REFACTOR only if a seam
  is needed), plus a COMMIT sub-task.
- Each RED/GREEN/REFACTOR sub-task has a place to paste the captured runner
  output — a checked box with no pasted evidence is incomplete.
- Each task includes a "Seam" block if a testability seam is needed for that
  scenario, listing the seam and the testability constraints. If no seam is
  needed, omit the block.
- `givn apply` reads these checkboxes to track progress.
- Done when: all checkboxes checked, every scenario has a commit hash
  recorded, AND the test runner exits 0 (full suite, not just per-scenario).

**Seam constraints (strictly enforced):**
- Allowed: extract interface, dependency injection, make a function/method
  public, add a test-only constructor or builder, extract a pure function
  from a side-effectful one.
- NOT allowed: rename, deduplicate, simplify logic, restructure for clarity,
  fix bugs, change error messages, change return types, change control flow.
- Every seam must be individually justifiable: "This seam enables scenario
  <title> by allowing <test input>".
