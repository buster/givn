# Instruction: tasks-reverse

Write the characterization task breakdown. Each task is a RED/GREEN/REFACTOR
cycle (see `givn-steps` skill for shared mechanics). Differences from normal
TDD are below.

## Setup task (first task, always)

- If no runner configured: install one that executes `.feature` files under
  `givn/specs/`. Configure strict mode. Set `verify.command`. Create the
  runner binary and step skeleton, one file per capability.
- If a runner exists: do not assume it is strict — run the proof-of-strictness
  check below regardless.
- Either way: run the proof-of-strictness check before characterization
  begins, then confirm the runner exits 0 (or only @wip-related failures).

## Rule: prove strict mode once, with evidence

Empty/undefined steps silently reporting PASS makes GREEN meaningless.
1. Write/reuse a step using the not-implemented stub (table below) or leave
   it undefined.
2. Run the runner, confirm non-zero exit.
3. If exit 0, enable the runner's strict flag (`--strict` for
   cucumber-js/cucumber-jvm, `.fail_on_skipped()` for cucumber-rs).
4. Paste the command + non-zero-exit output into the task.

## Rule: step body is never empty

`{}`, bare `pass`, bare `return` = indistinguishable from PASS. Unimplemented
steps use:

| Language / framework | Stub |
|---|---|
| Rust (`cucumber-rs`) | `unimplemented!()` or `todo!()` |
| Java (`cucumber-jvm`) | `throw new io.cucumber.java.PendingException("TODO");` |
| Python (`behave`) | `raise NotImplementedError("TODO")` |
| JS/TS (`cucumber-js`) | `return "pending";` or `throw new Error("TODO")` |
| Go (`godog`) | `return godog.ErrPending` |
| Ruby (`cucumber-ruby`) | `pending("TODO")` |

## Characterization loop (in .feature order)

```
For each scenario:
  RED:      Remove @wip from THIS scenario only. Check for reusable steps
              first. New steps use the stub above. Run the runner TARGETING
              THIS SCENARIO ONLY. PASTE the output. Must fail — or pass
              immediately if ALL steps were reused (record which).
  GREEN:    Implement real assertions probing OBSERVED behaviour (read the
              actual code, not documentation). Run targeting this scenario
              only. PASTE the output. Must pass.
            If it fails: do NOT change production code. Diagnose (below).
  REFACTOR: Only if the task's Seam block calls for a testability seam.
              Apply it. Re-run, PASTE output. Must still pass. If red:
              revert the refactor — behaviour changed, the test was correct.
              No seam needed → skip.
  COMMIT:   One commit for RED+GREEN+REFACTOR. Message: "characterize
              (<capability>): <release note>". The message
              carries the release note as its message subject, and
              the scenario title goes in the message body.
              Record the hash. No commit = task not done.
  Then move to the next scenario.
```

## Distinctions

`givn lint` is a static check, no tests. `verify.command` confirms GREEN.
GREEN means "matches observed reality," not "code is correct." Never
conflate the two.

## Spec/implementation mismatch diagnostic

When verify fails and infrastructure is sound, the spec was wrong (built
from docs/code reading):

1. Diagnose: re-read the actual code and the documentation that informed the
   spec. Root cause is either (a) documentation was wrong, or (b) code is
   ambiguous/misleading (including bugs — code does Z when it should do Y,
   but characterize AS-IS).
2. Correct:
   - (a): update tasks.md, the scenario, and the wrong source documentation.
     Re-run verify.
   - (b): update the scenario to assert actual behaviour. Add a code comment
     `TODO: REVERSE-ENGINEERING NOTE: <what was misleading> — actual
     behaviour: <Z>. Code smell, refactor candidate.` (adapt comment syntax
     to the language). Do not change code behaviour. Re-run verify. Flag as
     a code smell + follow-up candidate.
3. Never: change production code to match the spec, fix bugs found during
   characterization, or remove `TODO: REVERSE-ENGINEERING NOTE` markers.

## Tasks file format

- `[ ]`/`[x]` checkboxes. RED/GREEN sub-tasks (REFACTOR only if a seam is
  needed) plus COMMIT.
- Each RED/GREEN/REFACTOR line has a field for pasted output.
- "Seam" block per task only when a seam is needed: seam + testability
  constraint.
- Done when: all boxes checked, every scenario has a commit hash, runner
  exits 0 on the full suite.

## Seam constraints (strict)

Allowed: extract interface, dependency injection, make public, add a
test-only constructor/builder, extract a pure function from a side-effectful
one. Not allowed: rename, dedup, simplify logic, restructure for clarity,
fix bugs, change error messages/return types/control flow. Every seam must
state: "enables scenario `<title>` by allowing `<test input>`."
