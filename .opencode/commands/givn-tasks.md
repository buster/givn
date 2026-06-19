---
description: Write the scenario-by-scenario RED/GREEN/REFACTOR task list for a givn change
---

Write the implementation task breakdown for a givn change.

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

Write `givn/changes/<id>/tasks.md` as a checkboxed scenario-by-scenario breakdown.

**Structure each task as one full RED/GREEN/REFACTOR cycle for one scenario:**

```markdown
- [ ] <capability>: <Scenario title>
      RED:      Remove @wip from this scenario. Write step definitions with real
                assertions. Run: <verify.command> → MUST FAIL.
      GREEN:    Write minimum production code. Run: <verify.command> → PASSES.
      REFACTOR: Clean up. Run: <verify.command> → still PASSES.
```

**Rules:**
- One task = one scenario = one full RED/GREEN/REFACTOR cycle.
- Order tasks by dependency (infrastructure first, feature scenarios after).
- Confirm the verify command from `givn/config.yaml`.
- `givn lint` is a static check only — it does NOT run tests.
- GREEN is confirmed by `verify.command` only, never by `givn lint`.

### 5. Check status

```sh
givn apply --change <id>
```

Confirms task tracking is live.

### 6. Show progress

```sh
givn status
```

Display next step: "Run `/givn-implement` to start the TDD loop."

---

## Output

```
## Tasks written: <id>

**Tasks written:** givn/changes/<id>/tasks.md
**Total tasks:** N (one per scenario)

**Next:** Run `/givn-implement` to work through the RED/GREEN/REFACTOR loop.
```

---

## Guardrails

- One task per scenario — do not collapse multiple scenarios into one task.
- Read design.md before writing tasks — step def locations matter.
- Tasks must be checkboxes (`- [ ]`) so `givn apply` can track progress.
- Verify the file exists after writing.
