---
description: (Re-)write the scenario-by-scenario RED/GREEN/REFACTOR task list for a givn change
---

Write (or re-write) the implementation task breakdown for a givn change.

For starting a new change, use `/givn-propose` instead — it writes the
proposal, spec, design, and tasks in one pass.
Use this command to redo or extend the task list for an existing change.

**Input**: Optionally specify a change ID after `/givn-tasks`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".

### 2. Get tasks instructions

```sh
givn instructions tasks --change <id>
```

Parse the output for:
- `output`: path to write the tasks (typically `givn/changes/<id>/tasks.md`)
- `template`: structure to follow
- `instruction`: guidance on what to produce
- `context` / `rules`: constraints for you — **do not copy these into the file**
- `dependencies`: completed artifacts to read (proposal, specs, design)

### 3. Read the inputs

Read:
- `givn/changes/<id>/design.md` — HOW layer (which modules, step def locations)
- `givn/changes/<id>/specs/<capability>/<capability>.feature` — the scenarios to implement
- `givn/changes/<id>/proposal.md` — WHAT/WHY for context

### 4. Write the task list

Write `givn/changes/<id>/tasks.md` as a checkboxed breakdown.

**First task: test infrastructure setup (always required)**

Before any scenario task, add a setup task:

```markdown
- [ ] Setup: configure test infrastructure and verify.command
      - Install/configure the test runner chosen in design.md
        (e.g. add Cucumber dependency, create step definitions skeleton,
        configure test framework)
      - Set `verify.command` in `givn/config.yaml` to the runner from design.md
        if not already done
      - Run: <verify.command> → must exit 0 (even with no scenarios yet)
```

This task gates all others — the runner must be working before any RED phase begins.

**Subsequent tasks: one per scenario**

Each task body must embed the design constraints that are relevant to that
specific scenario. The coding agent that executes the task may not read
design.md — everything it needs to follow the architecture must be in the task.

```markdown
- [ ] <capability>: <Scenario title>
      Design constraints:
        - <copy the architectural decisions from design.md that apply here>
          e.g. which layer/module handles this, which protocol or data format
          to use, which framework/library to call, which response shape to
          return. Be specific enough that there is no ambiguous shortcut.
      RED:      Remove @wip from this scenario. Write step definitions with real
                assertions. Run: <verify.command> → MUST FAIL.
      GREEN:    Write the minimum production code that makes this scenario pass,
                following the design constraints above exactly.
                Run: <verify.command> → PASSES.
      REFACTOR: Clean up. Run: <verify.command> → still PASSES.
```

**Rules:**
- First task is always infrastructure setup — never skip it.
- One scenario task = one full RED/GREEN/REFACTOR cycle.
- Every scenario task must include a "Design constraints" block populated from
  design.md — never leave it empty or generic.
- Order scenario tasks by dependency (foundational scenarios before edge cases).
- `givn lint` is a static check only — it does NOT run tests.
- GREEN is confirmed by `verify.command` only, never by `givn lint`.

### 5. Check status

```sh
givn apply --change <id>
```

Confirms task tracking is live.

### 6. Show next step

```sh
givn status --change <id> --json
```

Read `next_required.id`. If it is `review` (or `all_required_done` is true),
report: "Ready to implement. Run `/givn-implement`."
If there are further planning artifacts after `tasks`, report that instead.

---

## Output

```
## Tasks written: <id>

**Tasks written:** givn/changes/<id>/tasks.md
**Total tasks:** N (one per scenario)

**Next:** Run `/givn-implement` to start the TDD loop (or the next planning
artifact if the manifest defines one after `tasks`).
```

---

## Guardrails

- One task per scenario — do not collapse multiple scenarios into one task.
- Read design.md before writing tasks — step def locations matter.
- Every scenario task must have a populated "Design constraints" block drawn
  from design.md. A task with no constraints, or with placeholder text, is
  incomplete — the coding agent will have no anchor and will take shortcuts.
- Tasks must be checkboxes (`- [ ]`) so `givn apply` can track progress.
- Verify the file exists after writing.
