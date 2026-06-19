---
description: Execute the RED/GREEN/REFACTOR TDD loop for a givn change, one scenario at a time
---

Implement a givn change using scenario-by-scenario TDD.

**Input**: Optionally specify a change ID after `/givn-implement`. If omitted,
infer from conversation context, or auto-select if only one active change exists.
If ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".

### 2. Check current progress

```sh
givn apply --change <id>
```

Shows tasks complete / remaining. If all tasks are already checked, congratulate
and suggest `/givn-review`. Otherwise continue.

### 3. Validate the spec

```sh
givn lint
```

- Exit 0 or 2 = proceed (exit 2 means @wip scenarios present — expected).
- Exit 1 = parse error — fix before implementing.

### 4. Read context

Read:
- `givn/changes/<id>/tasks.md` — task list and order
- `givn/changes/<id>/specs/<capability>/<capability>.feature` — scenarios
- `givn/changes/<id>/design.md` — architecture, step def locations, verify command
- `givn/config.yaml` — resolve the actual `verify.command`

### 5. Implement tasks (loop until done or blocked)

Work through each unchecked task (`- [ ]`) in order:

#### RED phase

1. Remove `@wip` from **this scenario only** (keep it on all others).
2. Write step definitions with **real assertions** — `Then` steps must check
   concrete values, not just that the code ran.
3. Run the test suite:
   ```sh
   <verify.command>
   ```
   This scenario **must fail** at this point (no production code yet).
   If it passes, the assertions are not real — fix the step definitions.

#### GREEN phase

4. Write the **minimum** production code to make this scenario pass.
5. Run: `<verify.command>` → this scenario passes; others still fail (correct).

#### REFACTOR phase

6. Clean up code, names, duplication — without changing behaviour.
7. Run: `<verify.command>` → still passes.

#### Done with scenario

8. Check off the task: `- [ ]` → `- [x]`
9. Move to the next task.

**Pause if:**
- A task is unclear → ask for clarification before touching code.
- Implementation reveals a design issue → suggest updating design.md, then continue.
- A blocker is encountered → report and wait for guidance.

### 6. On completion

```sh
givn apply --change <id>
```

Show final progress. If all tasks are checked:

```sh
givn status
```

Suggest next step: "Run `/givn-review` to complete the coverage review."

---

## Output during implementation

```
## Implementing: <id>

Working on task 3/7: <capability>: <Scenario title>
  RED: removed @wip, wrote assertions, ran tests → FAILED as expected
  GREEN: wrote production code, ran tests → PASSED
  REFACTOR: cleaned up, ran tests → still PASSED
  [x] task marked complete

Working on task 4/7: ...
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

**Issue:** <description>

**Options:**
1. <option>
2. <option>
```

---

## Guardrails

- Work through tasks in order — do not skip or reorder.
- RED must genuinely fail before writing production code. If it passes, fix the
  assertions.
- `givn lint` is a STATIC check only — it does NOT run tests and does NOT
  confirm GREEN. GREEN comes from `<verify.command>` only.
- Remove `@wip` from one scenario at a time only.
- Check off tasks immediately after the REFACTOR phase passes.
- If a task is ambiguous, ask — do not guess and write the wrong code.
- Do not batch multiple scenarios into one RED/GREEN cycle.
