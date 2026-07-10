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

**First task: test infrastructure setup (always required), including
proof-of-strictness**

Before any scenario task, add a setup task. This is the single most
important task in the whole file: it exists because a runner that silently
passes undefined/empty steps makes every later "GREEN" meaningless. This has
happened in practice — every step left as an empty stub, `@wip` removed
from all of them, `verify.command` exiting 0, and 0% of the feature actually
built.

```markdown
- [ ] Setup: configure test infrastructure, strict mode, and verify.command
      - Install/configure the test runner chosen in design.md
        (e.g. add Cucumber dependency, create step definitions skeleton,
        configure test framework). Step definitions go ONE FILE PER
        CAPABILITY (from design.md's Step Definitions table) — never a
        single file for the whole change.
      - Configure the runner's strict-mode flag from design.md (undefined/
        pending steps must FAIL, not silently pass or skip).
      - PROOF OF STRICTNESS (mandatory): write one step using the language's
        "not implemented" stub from design.md (e.g. Java:
        `throw new io.cucumber.java.PendingException("TODO")`, Python:
        `raise NotImplementedError`, Rust: `unimplemented!()`). Run the
        runner. Confirm it exits NON-ZERO. Paste the command + output in
        this task. If it exits 0, fix the strict-mode config and re-prove —
        do not proceed until this is non-zero.
      - Set `verify.command` in `givn/config.yaml` to the runner from design.md
        if not already done
      - Run: <verify.command> → must exit 0 with the proof step removed/fixed
        (even with all real scenarios still @wip)
```

This task gates all others — the runner must be PROVEN strict before any RED
phase begins. A checked box on this task without pasted proof-of-strictness
output is incomplete.

**Subsequent tasks: one per non-@e2e scenario, each ending in a commit**

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
      RED:      Remove @wip from this scenario. Write step definitions — any
                step without a real assertion yet uses the language's
                "not implemented" stub (see design.md). NEVER an empty body
                (`{}`, bare `pass`, bare `return`) — that is indistinguishable
                from a pass and is the exact failure mode this process exists
                to prevent.
                Run: <verify.command>, TARGETING THIS SCENARIO ONLY (use the
                single-scenario run command from design.md) → MUST FAIL.
                Paste the output in this task as evidence.
      GREEN:    Replace stubs with real assertions. Write the minimum
                production code that makes this scenario pass, following the
                design constraints above exactly. List every production file
                created/modified — if the list is empty and this is not a
                pure step-reuse case, STOP and investigate before claiming
                GREEN.
                Run: <verify.command>, targeting this scenario only → PASSES.
                Paste the output as evidence.
      REFACTOR: Clean up. Run: <verify.command>, targeting this scenario
                only → still PASSES. Paste the output as evidence.
      COMMIT:   One atomic commit for this scenario's RED+GREEN+REFACTOR.
                Message references the scenario title verbatim, e.g.
                "feat(<capability>): <scenario title>". Record the commit
                hash in this task.
```

**After all non-@e2e scenarios: e2e infrastructure setup + @e2e scenario tasks**

After all non-@e2e scenario tasks, add the e2e setup and @e2e scenario tasks:

```markdown
- [ ] Setup: configure e2e test infrastructure, strict mode, and verify.e2e_command
      - Install/configure the e2e framework chosen in design.md
        (e.g. Playwright, Testcontainers, Cucumber with @e2e tag filtering)
      - Create the e2e step definition skeleton, one file per capability
        (separate from unit/integration step definitions)
      - Configure strict mode for the e2e runner and PROVE IT (same
        proof-of-strictness check as the main setup task, run against this
        runner). Paste the command + non-zero-exit output.
      - Set `verify.e2e_command` in `givn/config.yaml` to the e2e runner
      - Document how to start/stop local test environment if needed
      - Run: <verify.e2e_command> → must exit 0

- [ ] @e2e <capability>: <E2E Scenario title>
      Design constraints:
        - <copy the e2e infrastructure decisions from design.md>
          e.g. which browser driver, how the server starts, test database setup,
          e2e step definition location.
      RED:      Remove @wip. Write e2e step definitions — unimplemented steps
                use the language's "not implemented" stub, never an empty
                body. Run: <verify.e2e_command>, targeting this scenario
                only → MUST FAIL. Paste evidence.
      GREEN:    Set up test infrastructure, replace stubs with real
                assertions driving the actual interface. List files
                created/modified.
                Run: <verify.e2e_command>, targeting this scenario only →
                PASSES. Paste evidence.
      REFACTOR: Clean up e2e code. Run: <verify.e2e_command>, targeting this
                scenario only → still PASSES. Paste evidence.
      COMMIT:   One atomic commit, message references the scenario title.
                Record the commit hash.
```

**Rules:**
- First task is always infrastructure setup, and it is NOT done until
  proof-of-strictness output is pasted in and shows a non-zero exit.
- One scenario task = one full RED/GREEN/REFACTOR/COMMIT cycle.
- Every scenario task must include a "Design constraints" block populated from
  design.md — never leave it empty or generic.
- A step definition body is NEVER left empty. Unimplemented steps use the
  language's "not implemented" stub from design.md.
- Every RED/GREEN/REFACTOR line requires pasted command output as evidence —
  not just a description of what should happen.
- GREEN requires listing production files created/modified. An empty list on
  a non-reuse scenario is a signal the "pass" is fake.
- Every scenario ends in a COMMIT sub-task with a recorded hash.
- Order: non-@e2e scenarios first (by dependency), then e2e setup, then @e2e scenarios.
- `givn lint` is a static check only — it does NOT run tests.
- GREEN for non-@e2e is confirmed by `verify.command` only, after strictness
  is proven.
- GREEN for @e2e is confirmed by `verify.e2e_command` only, after strictness
  is proven.
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
- Read design.md before writing tasks — step def locations, strict-mode
  config, and the not-implemented stub pattern all matter and must be
  copied into the setup task verbatim.
- Every scenario task must have a populated "Design constraints" block drawn
  from design.md. A task with no constraints, or with placeholder text, is
  incomplete — the coding agent will have no anchor and will take shortcuts.
- The setup task MUST include a proof-of-strictness step with a place to
  paste the command and its non-zero-exit output. Do not omit this — it is
  the single control that prevents an entire change from being fabricated
  with empty step stubs while the runner reports GREEN.
- Every scenario task's RED/GREEN/REFACTOR lines must have a place to paste
  runner output, and GREEN must have a place to list production files
  touched. Do not write tasks that only describe what should happen without
  a place to record what actually happened.
- Every scenario task ends with a COMMIT sub-task with a place for the
  commit hash.
- Tasks must be checkboxes (`- [ ]`) so `givn apply` can track progress.
- Verify the file exists after writing.
