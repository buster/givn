---
description: Complete the coverage review for a givn change — classify every gap, sign off, confirm GREEN
---

Complete the coverage review for a givn change.

**Input**: Optionally specify a change ID after `/givn-review`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Steps

### 1. Resolve the change

```sh
givn status
```

Identify the active change. Announce: "Using change: <id>".
If any tasks are still unchecked, warn the user — all tasks should be done
before review. Confirm to proceed if they want.

### 2. Get review instructions

```sh
givn instructions review --change <id>
```

Parse the output for:
- `output`: path to write the review (typically `givn/changes/<id>/review.md`)
- `template`: structure to follow
- `instruction`: guidance on what to produce
- `context` / `rules`: constraints for you — **do not copy these into the file**
- `dependencies`: completed artifacts to read

### 3. Confirm all tests pass

Run the test suite:

```sh
<verify.command>
```

Resolved from `givn/config.yaml`. All scenarios must be GREEN (no `@wip`
remaining). If any fail, stop and fix before continuing the review.

### 4. Run coverage tooling

Run your project's coverage tool (from design.md — it specifies the runner).
Collect the list of uncovered lines/branches.

### 5. Classify every gap

For every uncovered region, assign it to exactly one bucket:

| Bucket | Action |
|---|---|
| **1. Dead code** (YAGNI/KISS) | Delete it now. |
| **2. Missing test** | Add a scenario + run full RED/GREEN/REFACTOR. |
| **3. Legitimately hard to test** | Write a concrete technical justification. |

There is **no minimum coverage percentage**. Coverage is complete when every
gap is classified and resolved (or justified). A high percentage is a side
effect, not the goal.

### 6. Resolve each gap

- **Dead code** → delete it; run tests again to confirm GREEN.
- **Missing test** → add the scenario to the relevant `.feature` file (use
  `@givn.added`), go through RED/GREEN/REFACTOR, check off tasks.
- **Hard to test** → document the justification inline in review.md.

### 7. Final GREEN confirmation

```sh
<verify.command>
```

All tests must pass after gap resolution.

### 8. Write review.md

Write `givn/changes/<id>/review.md` following the template:

- Coverage gap table (bucket, region, resolution/justification for each).
- Sign-off checklist (all items checked).
- Final line: `REVIEW: PASS`

The `REVIEW: PASS` line is required by the archive gate.

### 9. Run the review hook

```sh
givn check review --change <id>
```

This runs the configured `review.post` hooks. Fix any failures before archiving.

### 10. Show progress

```sh
givn status
```

Display next step: "Run `/givn-archive` to archive this change."

---

## Output

```
## Review complete: <id>

**Coverage gaps classified:** N
  - Dead code removed: N
  - Missing tests added: N  
  - Hard to test (justified): N

**Final test run:** GREEN

**Review written:** givn/changes/<id>/review.md  (REVIEW: PASS)

**Next:** Run `/givn-archive` to archive this change.
```

---

## Guardrails

- Do not write `REVIEW: PASS` until all gaps are resolved or justified.
- `givn lint` is a static check only — GREEN comes from `<verify.command>`.
- If new scenarios are added during review, they must go through the full
  RED/GREEN/REFACTOR loop (go back to `/givn-implement` for those tasks).
- Run `givn check review` after writing review.md to confirm hooks pass.
- The archive gate will hard-block if `REVIEW: PASS` is absent or hooks fail.
