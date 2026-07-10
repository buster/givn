---
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time
---

Implement a givn change using scenario-by-scenario TDD.

**Input**: Optionally specify a change ID after `/givn-implement`. If omitted,
infer from conversation context, or auto-select if only one active change exists.
If ambiguous, list changes and ask.

---

## Role: orchestrator

This command is an **orchestrator**. It does not write code directly. For each
task it spawns a **coding subagent** that handles the complete RED/GREEN/REFACTOR
cycle in isolation. The orchestrator manages the loop, passes context, and
decides when to stop or pause.

**The orchestrator does not trust subagent self-reports.** A subagent
reporting "RED failed as expected, GREEN passed" is a claim, not proof. This
has failed in practice: a subagent (or a whole run of them) checked off
every task with "PASSED" while every step definition was an empty stub and
the runner silently reported GREEN because it was never configured to fail
on undefined/pending steps. The orchestrator MUST independently verify each
subagent's work before checking off its task — see step 5c below. This is
not optional and is not skippable to save time.

---

## Steps

### 1. Resolve the change

```sh
givn status --json
```

Identify the active change from `active[0].id` (or ask if multiple).
Announce: "Using change: <id>".

### 2. Check current progress

```sh
givn apply --change <id>
```

If all tasks are already checked, suggest `/givn-review`.
Otherwise continue.

### 3. Validate the spec

```sh
givn lint --change <id>
```

- Exit 0 or 2 = proceed (exit 2 means @wip scenarios present — expected).
- Exit 1 = parse error — fix before implementing.

### 4. Collect context for subagents

Read once; pass to every subagent:

- `givn/changes/<id>/tasks.md` — task list and order (each task includes a
  "Design constraints" block; if any task is missing it, read design.md and
  add it before spawning that subagent)
- `givn/changes/<id>/specs/<capability>/<capability>.feature` — scenarios
- `givn/changes/<id>/design.md` — full design document; pass this to every
  subagent in full, not just "relevant sections" — in particular the Strict
  Mode section (strict-mode flag + not-implemented stub pattern) and the
  single-scenario run command
- `givn/config.yaml` — resolve the actual `verify.command` and `verify.e2e_command`

### 4a. Confirm the setup task's proof-of-strictness before looping

Before spawning any scenario subagent, confirm the first (setup) task in
tasks.md is checked off AND has pasted proof-of-strictness output showing a
NON-ZERO exit. If it is checked but has no pasted evidence, or the pasted
evidence shows a zero exit, do NOT proceed — spawn a setup subagent (or
handle it directly) to actually prove strictness first. Every scenario
subagent's RED phase depends on this being true; skipping it means nothing
downstream can be trusted.

### 5. Loop: one coding subagent per task

For each unchecked task (`- [ ]`) in order:

#### 5a. Spawn a coding subagent

Use the **Task tool** to spawn a coding subagent (fall back to describing the
pattern in prose if the Task tool is not available to you).

Pass the subagent a self-contained prompt that includes:

```
You are a coding agent executing a single TDD task for the givn change '<id>'.

## Your task
<paste the task line verbatim, including the Design constraints block>

## Scenario to implement
<paste the relevant scenario(s) from the .feature file>

## Context
- Verify command (non-@e2e): <verify.command>
- Verify command (@e2e):     <verify.e2e_command>
- Single-scenario run command: <from design.md — use this to run ONLY this
  scenario during RED/GREEN/REFACTOR, never rely on reading whole-suite output>
- Step definitions location: <from design.md — one file per capability>
- E2E step definitions location: <from design.md>
- Not-implemented stub pattern for this language: <from design.md — e.g.
  Java: throw new io.cucumber.java.PendingException("TODO"); Python:
  raise NotImplementedError(...); Rust: unimplemented!()>

## Absolute rule: never write an empty step definition body

An empty body (`{}`, bare `pass`, bare `return`, whitespace-only) is
indistinguishable from a PASS in most Cucumber runners. This is the exact
defect that has previously caused an entire change to be checked off as
complete while nothing was actually implemented. Every step without a real
assertion yet MUST use the not-implemented stub pattern above. This applies
in RED and at every intermediate point — never leave a stub as an empty body
"to fill in later."

## Before any code: read the shared step-writing skill
Read the givn-steps skill at `.agents/skills/givn-steps/SKILL.md` (or the
equivalent path in your agent's skill directory: `.claude/skills/givn-steps/`
or `.opencode/skills/givn-steps/`) in full before writing any step definitions.
It describes the RED/GREEN/REFACTOR loop, step reuse, and lint-vs-test
distinction that you must follow.

## TDD stance (overlay on givn-steps)

The givn-steps skill describes the shared loop. For normal TDD, these rules
OVERLAY the GREEN and REFACTOR phases:

### Before any code: read the design
Read givn/changes/<id>/design.md in full before touching any file.
The "Design constraints" block in your task summarises the decisions that apply
to this scenario specifically. The full design.md is the authoritative source.
Do not substitute a simpler approach that happens to make a test pass —
implement the architecture as specified.

### For non-@e2e scenarios:

#### RED phase
1. Remove `@wip` from this scenario only (leave it on all others).
2. Check for reusable step definitions first. For new steps, write step
   definitions using the not-implemented stub pattern (never an empty body).
3. Run: <verify.command>, TARGETING THIS SCENARIO ONLY via the
   single-scenario run command. Capture the full output verbatim.
   This scenario MUST fail now (no production code yet) with a non-zero exit.
   If it exits 0, either the runner is not strict or a step is empty —
   STOP and fix this before writing any production code. Do not proceed on
   a false RED.

#### GREEN phase
4. Implement real assertions for new steps (replace every stub). Write the
   minimum production code to make this scenario pass, following the design
   constraints in the task and the architecture in design.md exactly.
   "Minimum" means fewest lines that satisfy the scenario — it does not mean
   a simpler architecture than the one designed.
5. List every production source file you created or modified for this
   scenario. If this list is empty and the scenario is not a pure
   step-reuse case, STOP — a scenario with no assertions and no production
   code cannot legitimately be GREEN.
6. Run: <verify.command>, targeting this scenario only via the
   single-scenario run command → PASSES. Capture the full output verbatim.

#### REFACTOR phase
7. Clean up code, names, duplication — without changing behaviour.
8. Run: <verify.command>, targeting this scenario only → still passes.
   Capture the output verbatim.

### For @e2e scenarios:

#### RED phase
1. Remove `@wip` from this @e2e scenario only.
2. Check for reusable e2e step definitions first. For new steps, write e2e
   step definitions (in the e2e step location from design.md) using the
   not-implemented stub pattern — never an empty body — for steps that don't
   yet drive the real interface (browser, CLI, API).
3. Run: <verify.e2e_command>, targeting this scenario only. Capture output.
   This scenario MUST fail now with a non-zero exit.
   If it exits 0, STOP and fix before writing any e2e glue.

#### GREEN phase
4. Set up test infrastructure if needed (start server, launch browser, seed DB).
   Replace stubs with real e2e glue that drives the actual interface,
   following the design constraints exactly.
5. List every file created/modified (e2e glue and, if needed, production code).
6. Run: <verify.e2e_command>, targeting this scenario only → PASSES.
   Capture output verbatim.

#### REFACTOR phase
7. Clean up e2e infrastructure code, extract shared helpers.
8. Run: <verify.e2e_command>, targeting this scenario only → still passes.
   Capture output verbatim.

### Done
9. Create ONE atomic commit for this scenario's RED+GREEN+REFACTOR work.
   Message: "feat(<capability>): <scenario title>" (or "test(e2e): <title>"
   for @e2e). Record the commit hash.
10. Update the task in givn/changes/<id>/tasks.md: paste the RED, GREEN, and
    REFACTOR evidence into the task's respective sub-items, list the
    production files touched, record the commit hash, THEN check the box:
    `- [ ]` → `- [x]`. A checked box with no pasted evidence is not
    acceptable — do not check the box until the evidence is written in.
11. Report back with:
    - RED output (verbatim, showing the non-zero exit)
    - GREEN output (verbatim, showing the zero exit)
    - REFACTOR output (verbatim)
    - List of production files created/modified
    - Commit hash
    - Any issues or decisions made
```

#### 5b. Wait for the subagent to report back

The subagent must complete RED/GREEN/REFACTOR, commit, and check off the task
before the orchestrator proceeds to the next task.

If the subagent reports a blocker, surface it to the user and wait for guidance
before spawning the next subagent.

#### 5c. Independently verify — do not trust the self-report

Before moving to the next task, the orchestrator itself (not another
subagent, not by re-reading the subagent's own words) performs these checks.
This step exists specifically because self-reported "PASSED" has been wrong
before — an entire change was checked off complete with every step
definition left as an empty stub, because nobody independently re-ran
anything.

1. **Re-run the scenario yourself.** Run <verify.command> (or
   <verify.e2e_command>) targeting just this scenario. Confirm it exits 0
   right now, independent of what the subagent reported.
2. **Check for a commit.** Run `git log --oneline -1` (or check the hash the
   subagent reported) and `git show --stat <hash>`. Confirm:
   - The commit exists.
   - It touches production source files (not only the `.feature` file or
     step definition stubs), unless the task was a legitimate pure
     step-reuse case.
3. **Scan the step definition(s) touched by this task for empty bodies.**
   Grep for `{}`, bare `pass`, bare `return`, or whitespace-only bodies in
   the files the subagent reported touching. Any match means the subagent's
   "PASSED" report is not trustworthy — reject the task (see below).
4. **Confirm the task in tasks.md has pasted evidence**, not just a checked
   box: RED/GREEN/REFACTOR output present, production files listed, commit
   hash recorded.

If ANY of these checks fail: **uncheck the task**, do not proceed to the
next task, and re-spawn a subagent for this task with the specific failure
called out explicitly (e.g. "your GREEN report was accepted but
`FooSteps.java:42` has an empty body — this is not implemented, redo it").

Only if all checks pass:

```sh
givn apply --change <id>
```

Confirm the task is checked off. Then move to the next task.

### 6. On completion

```sh
givn apply --change <id>
```

Show final progress. If all tasks are checked, run one final independent
check before handing off — do not skip this because individual tasks were
already verified in step 5c; this is a whole-suite sanity check:

1. Run the FULL suite: <verify.command> and <verify.e2e_command> (not
   scenario-targeted this time). Confirm both exit 0.
2. Grep every step definition file in the change for empty/no-op bodies
   (`{}`, bare `pass`, bare `return`). Report the count — it must be 0.
3. Count commits made during this implementation run and compare to the
   number of scenario tasks. A mismatch (fewer commits than scenarios) means
   some task was checked off without an atomic commit — investigate before
   proceeding.

```sh
givn status --change <id>
```

Suggest next step: "Run `/givn-review` to complete the coverage review."

---

## Output during implementation

```
## Implementing: <id>

Task 3/7: <capability>: <Scenario title>
  → Spawning coding subagent...
  RED:      FAILED as expected
  GREEN:    PASSED
  REFACTOR: PASSED
  [x] task checked off

Task 4/7: ...
```

## Output on completion

```
## Implementation complete: <id>

**Progress:** 7/7 tasks complete

**Next:** Run `/givn-review` to complete the coverage review.
```

## Output on pause

```
## Implementation paused: <id>

**Progress:** 4/7 tasks complete

**Blocker:** <description>

**Waiting for:** <what is needed to continue>
```

---

## Guardrails

- The orchestrator does not write code — it manages the loop and delegates.
- Each subagent handles exactly one task (one RED/GREEN/REFACTOR/COMMIT cycle).
- RED must genuinely fail before production code is written. If the subagent
  reports it passed on RED, reject and re-spawn with corrected instructions.
- **Never trust a subagent's self-reported PASS/FAIL.** Step 5c
  (independent verification) is mandatory for every single task, not a
  spot-check. Skipping it is how an entire change gets checked off complete
  with nothing actually implemented.
- A step definition body is NEVER empty (`{}`, bare `pass`, bare `return`).
  If found during verification, the task is rejected regardless of what the
  subagent reported.
- Every scenario task requires an atomic commit before the box is checked.
  No commit → task is not done, regardless of subagent claims.
- Do not spawn the next subagent until the previous one's task has been
  independently verified (step 5c) and checked off.
- Remove `@wip` from one scenario at a time only — pass this constraint to each
  subagent explicitly.
- `givn lint` is a STATIC check only — GREEN comes from `<verify.command>` (non-@e2e)
  or `<verify.e2e_command>` (@e2e scenarios), and only once strict mode is
  proven (step 4a).
- If a task is ambiguous, ask the user before spawning its subagent.
- Do not batch multiple scenarios into one subagent.
- If a subagent reports GREEN but deviated from the design (e.g. used a simpler
  architecture that happens to pass the test), reject the result and re-spawn
  with an explicit instruction to follow the design. GREEN means the scenario
  passes AND the implementation matches the design AND independent
  verification confirms it.
