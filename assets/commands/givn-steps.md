---
description: Shared step-writing mechanics for Gherkin scenarios — RED/GREEN/REFACTOR with step reuse. Reference skill for givn-implement and givn-characterize.
---

Shared step-writing mechanics for Gherkin `.feature` scenarios. This is a
**reference skill**, not a standalone workflow. It is read by `/givn-implement`
and `/givn-characterize` subagents before writing step definitions.

**This command does not execute a workflow.** It prints the shared
step-writing reference. To actually write steps as part of a change, use:

- `/givn-implement` — RED/GREEN/REFACTOR TDD loop (normal changes).
- `/givn-characterize` — RED/GREEN/REFACTOR characterization loop (reverse
  changes).

---

## When this skill is read

The `/givn-implement` and `/givn-characterize` orchestrators instruct their
subagents to read this skill before writing any step definitions. It provides
the mechanics that are identical between both workflows:

- Step reuse (check existing definitions first).
- The RED/GREEN/REFACTOR loop structure.
- Lint vs test distinction.
- Step definition location conventions.

What differs between workflows (production code rules, refactor scope,
mismatch diagnostic) is overlaid by each orchestrator's prompt — not
duplicated here.

---

## The `.feature` file is the executable spec

The `.feature` file is the sole source of scenario identity. Step text is the
contract; step definitions are shared glue that the Gherkin runner binds to
the scenarios. If a scenario's steps change, the test surface changes
automatically. Never maintain a parallel hand-written test file that
duplicates scenario logic.

## Step reuse — always check first

Before writing any new step definition, check whether a matching step
definition already exists. Step reuse across scenarios and capabilities is
desirable — it reduces duplication and keeps the glue maintainable.

- **Reused step**: keep the existing definition as-is. It already has real
  assertions (proven by prior use). Do not modify or replace it.
- **New step**: write a new step definition.

## Never write an empty step definition body — this is not optional

This is the single most important rule in this skill. An empty step
definition body (`{}`, a bare `pass`, a bare `return`, whitespace only) is
**indistinguishable from a passing test** in most Cucumber runner
implementations. This has happened in practice: every step in an entire
change was left as an empty stub, `@wip` was removed from every scenario,
and the test runner reported 100% GREEN — because Cucumber-JVM (and most
other implementations) treat an empty method body as a step that "ran
successfully," not as pending or undefined.

A step without a real assertion yet MUST use the language's canonical
"not implemented" primitive so the runner is forced to report it as failing
or pending:

| Language / framework | Stub for an unimplemented step |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO: implement");` |
| Python (`behave`) | `raise NotImplementedError("TODO: implement")` |
| JavaScript/TS (`cucumber-js`) | `return "pending";` or `throw new Error("TODO: implement")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO: implement")` |

This applies at every phase, not just RED. A step is only exempt from this
rule once it has a real, concrete assertion (GREEN) or is a genuinely reused
setup step that performs a real action (not merely exists).

**Before trusting any runner's "PASS" result, confirm the runner is
strict** — configured so undefined/pending steps produce a non-zero exit,
not a silent skip. The design.md for this change documents the exact
strict-mode mechanism and should have been proven once during setup (see
tasks.md's first task). If you are ever unsure whether the runner is strict,
re-verify: write a throwaway step using the stub pattern above and confirm
the runner fails non-zero before trusting any scenario's GREEN.

## E2E step definitions

`@e2e` scenarios use **separate** step definitions from unit/integration
scenarios. They drive the real interaction surface (browser, CLI, API) and
require different infrastructure (browser driver, HTTP client, test harness).

Step reuse within e2e steps follows the same rules: check for existing e2e
step definitions first. E2e and non-e2e step definitions are separate — do
not mix them.

## The RED/GREEN/REFACTOR/COMMIT loop (one scenario at a time)

Every phase below requires running the runner **targeted at this scenario
only** (by name, line number, or tag — use the single-scenario run command
from design.md), and **capturing the output verbatim** as evidence. Reading
whole-suite output and inferring one scenario's result is not acceptable —
it is exactly how empty stubs hide inside a suite that "looks green."

### RED — confirm the test CAN fail

1. Remove `@wip` from the current scenario only (leave it on all others).
2. For each step in the scenario, check if a matching step definition already
   exists:
   - **Reused step**: keep the existing definition as-is.
   - **New step**: write a step definition using the not-implemented stub
     pattern (see table above) — never an empty body.
3. Run the appropriate test runner, targeting this scenario only:
   - Non-@e2e scenario: `verify.command`
   - @e2e scenario: `verify.e2e_command`
   - Capture the full output.
   - **Scenario FAILS with non-zero exit** → RED confirmed (the test is
     real). Proceed to GREEN.
   - **Scenario PASSES (all steps reused)** → legitimate immediate GREEN
     ONLY IF every step was a genuine reuse of an already-proven definition.
     Record which steps were reused. Proceed to GREEN confirmation.
   - **Scenario passes but contains a NEW step with no real assertion** →
     something is wrong: either the step is silently empty, or the runner
     is not strict. STOP. Do not proceed until you have confirmed the
     runner is strict (see above) and fixed the stub.

### GREEN — implement real assertions

4. Implement real assertions for new steps (Then steps must check concrete
   values, not just that code ran). Replace every stub with either a real
   assertion or a real action.
   - **For normal TDD** (`/givn-implement`): also write the minimum
     production code to make this scenario pass. List every production
     source file created or modified. An empty list on a scenario that is
     not a pure step-reuse case is a red flag — investigate before
     proceeding, do not report GREEN.
   - **For characterization** (`/givn-characterize`): probe the observed
     production behaviour. No production code changes. If it fails, run the
     mismatch diagnostic (see `/givn-characterize`).
   - **For @e2e scenarios**: set up test infrastructure if needed (start
     server, launch browser, seed database). Write the e2e glue that drives
     the real interaction surface.
5. Run the appropriate test runner, targeting this scenario only → PASSES
   with a zero exit. Capture the output.

### REFACTOR — clean up without changing behaviour

6. Refactor per the workflow's scope:
   - **Normal TDD**: general cleanup (names, duplication, clarity). No
     behaviour change.
   - **Characterization**: testability seams only (extract interface, DI,
     make public). If red, revert the refactor. If no seam is needed, skip.
   - **E2E**: clean up infrastructure code, extract shared e2e helpers.
7. Run the appropriate test runner, targeting this scenario only → still
   passes. Capture the output.

### COMMIT — make the work independently auditable

8. Create ONE atomic commit covering this scenario's RED+GREEN+REFACTOR
   work. Reference the scenario title verbatim in the commit message (e.g.
   `feat(<capability>): <scenario title>`, `characterize(<capability>):
   <scenario title>`, or `test(e2e): <scenario title>`). Record the commit
   hash.
   This is what makes a checked-off task verifiable after the fact — a
   checked task with no matching commit, or a commit that touches nothing
   but the `.feature` file, is proof the work was not actually done.

### Done with scenario

9. Paste the RED, GREEN, and REFACTOR evidence, the list of files touched,
   and the commit hash into the task's entry in `tasks.md`. Only then check
   off the task: `- [ ]` → `- [x]`. A checked box with no pasted evidence is
   incomplete, not done.
10. Move to the next scenario.

## Lint vs test — do not confuse them

- `givn lint` is a STATIC Gherkin check (syntax, `@wip`, delta tags). It
  runs NO tests.
- `verify.command` confirms GREEN for non-@e2e scenarios.
- `verify.e2e_command` confirms GREEN for @e2e scenarios.
- Never conflate lint, `verify.command`, or `verify.e2e_command`.

## Guardrails

- One scenario at a time — remove `@wip` from one scenario only.
- Step definitions are shared glue — never a parallel hand-maintained test
  file.
- Step definitions are organised **one file per capability** — never one
  giant file for the whole change. A single file holding every step makes
  empty/no-op stubs easy to hide.
- A step definition body is NEVER empty. Unimplemented steps always use the
  language's not-implemented stub (see table above).
- Then steps must check concrete values, not just that code ran.
- Reuse existing step definitions whenever they match — do not duplicate.
- Every RED/GREEN/REFACTOR run targets the single scenario in question and
  its output is captured as evidence, not just described.
- GREEN requires listing production files touched; an empty list on a
  non-reuse scenario is a signal the pass is fake.
- Every scenario ends in a COMMIT before the task box is checked. No commit,
  no checked box.
