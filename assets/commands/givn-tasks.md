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

**Subsequent tasks: one per non-@e2e scenario**

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

**After all non-@e2e scenarios: e2e infrastructure setup + @e2e scenario tasks**

After all non-@e2e scenario tasks, add the e2e setup and @e2e scenario tasks:

```markdown
- [ ] Setup: configure e2e test infrastructure and verify.e2e_command
      - Install/configure the e2e framework chosen in design.md
        (e.g. Playwright, Testcontainers, Cucumber with @e2e tag filtering)
      - Create the e2e step definition skeleton (separate from unit/integration
        step definitions)
      - Set `verify.e2e_command` in `givn/config.yaml` to the e2e runner
      - Document how to start/stop local test environment if needed
      - Run: <verify.e2e_command> → must exit 0

- [ ] @e2e <capability>: <E2E Scenario title>
      Design constraints:
        - <copy the e2e infrastructure decisions from design.md>
          e.g. which browser driver, how the server starts, test database setup,
          e2e step definition location.
      RED:      Remove @wip. Write e2e step definitions with real assertions
                that drive the actual interface. Run: <verify.e2e_command> → MUST FAIL.
      GREEN:    Set up test infrastructure, write e2e glue.
                Run: <verify.e2e_command> → PASSES.
      REFACTOR: Clean up e2e code. Run: <verify.e2e_command> → still PASSES.
```

**Rules:**
- First task is always infrastructure setup — never skip it.
- One scenario task = one full RED/GREEN/REFACTOR cycle.
- Every scenario task must include a "Design constraints" block populated from
  design.md — never leave it empty or generic.
- Order: non-@e2e scenarios first (by dependency), then e2e setup, then @e2e scenarios.
- `givn lint` is a static check only — it does NOT run tests.
- GREEN for non-@e2e is confirmed by `verify.command` only.
- GREEN for @e2e is confirmed by `verify.e2e_command` only.
- Never conflate lint, `verify.command`, or `verify.e2e_command`.

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
