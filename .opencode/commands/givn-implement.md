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
  subagent in full, not just "relevant sections"
- `givn/config.yaml` — resolve the actual `verify.command`

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
- Verify command: <verify.command>
- Step definitions location: <from design.md>

## Instructions

### Before any code: read the design
Read givn/changes/<id>/design.md in full before touching any file.
The "Design constraints" block in your task summarises the decisions that apply
to this scenario specifically. The full design.md is the authoritative source.
Do not substitute a simpler approach that happens to make a test pass —
implement the architecture as specified.

### RED phase
1. Remove `@wip` from this scenario only (leave it on all others).
2. Write step definitions with real assertions — Then steps must check
   concrete values, not just that code ran.
3. Run: <verify.command>
   This scenario MUST fail now (no production code yet).
   If it passes, the assertions are not real — fix them.

### GREEN phase
4. Write the minimum production code to make this scenario pass,
   following the design constraints in the task and the architecture in
   design.md exactly. "Minimum" means fewest lines that satisfy the
   scenario — it does not mean a simpler architecture than the one designed.
5. Run: <verify.command> → this scenario passes; others may still fail.

### REFACTOR phase
6. Clean up code, names, duplication — without changing behaviour.
7. Run: <verify.command> → still passes.

### Done
8. Check off the task in givn/changes/<id>/tasks.md: `- [ ]` → `- [x]`
9. Report back with:
   - RED result (pass/fail as expected)
   - GREEN result
   - REFACTOR result
   - Any issues or decisions made
```

#### 5b. Wait for the subagent to report back

The subagent must complete RED/GREEN/REFACTOR and check off the task before
the orchestrator proceeds to the next task.

If the subagent reports a blocker, surface it to the user and wait for guidance
before spawning the next subagent.

#### 5c. After each subagent completes

```sh
givn apply --change <id>
```

Confirm the task was checked off. Then move to the next task.

### 6. On completion

```sh
givn apply --change <id>
```

Show final progress. If all tasks are checked:

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
- Each subagent handles exactly one task (one RED/GREEN/REFACTOR cycle).
- RED must genuinely fail before production code is written. If the subagent
  reports it passed on RED, reject and re-spawn with corrected instructions.
- Do not spawn the next subagent until the previous one has checked off its task.
- Remove `@wip` from one scenario at a time only — pass this constraint to each
  subagent explicitly.
- `givn lint` is a STATIC check only — GREEN comes from `<verify.command>` only.
- If a task is ambiguous, ask the user before spawning its subagent.
- Do not batch multiple scenarios into one subagent.
- If a subagent reports GREEN but deviated from the design (e.g. used a simpler
  architecture that happens to pass the test), reject the result and re-spawn
  with an explicit instruction to follow the design. GREEN means the scenario
  passes AND the implementation matches the design.
