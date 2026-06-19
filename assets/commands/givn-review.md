---
description: Complete the coverage review for a givn change — record coverage, confirm GREEN, sign off
---

Complete the coverage review for a givn change.

**Input**: Optionally specify a change ID after `/givn-review`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Role: orchestrator

This command is an **orchestrator**. It collects context and then delegates all
review work to a **review subagent**. The review subagent runs the test suite,
measures coverage, records observations, and writes `review.md`.

---

## Steps

### 1. Resolve the change

```sh
givn status --change <id> --json
```

Identify the active change. Announce: "Using change: <id>".
Check the `artifacts` array: if the `tasks` entry is not `"status": "done"`,
warn the user — all tasks should be complete before review. Confirm to proceed.

### 2. Collect context

Read and assemble:
- `givn/changes/<id>/tasks.md`
- `givn/changes/<id>/specs/**/*.feature`
- `givn/changes/<id>/design.md`
- `givn/config.yaml` — resolve `verify.command`

```sh
givn instructions review --change <id>
```

Capture the full output: `output` path, `template`, `instruction`, `rules`,
`dependencies`.

### 3. Spawn a review subagent

Use the **Task tool** to spawn a review subagent (fall back to describing the
pattern in prose if the Task tool is not available to you).

Pass a self-contained prompt:

```
You are a review agent completing the review for givn change '<id>'.

## Your objective
Produce a complete, signed-off review.md at:
  givn/changes/<id>/review.md

The final line of the file MUST be either:
  REVIEW: PASS
  REVIEW: FAIL

## Review instructions
<paste full output of: givn instructions review --change <id>>

## Verify command
<verify.command from givn/config.yaml>

## Steps

### 1. Confirm all scenarios pass
Run: <verify.command>
All scenarios must be GREEN (no @wip remaining).
If any fail, stop and report — do not proceed to coverage until GREEN.

### 2. Measure coverage
Run the coverage tool specified in design.md (or a sensible default for the
tech stack). Record the overall percentage and the list of uncovered regions.

Coverage is informational — you are recording it, not enforcing a threshold.
A low number is worth noting; it does not prevent REVIEW: PASS.

### 3. Record coverage observations
For each uncovered region, note it in the coverage table with a bucket label:
- Dead code — worth deleting (but not required for sign-off)
- Missing test — worth adding (but not required for sign-off)
- Hard to test — note the reason

If coverage tooling is unavailable or impractical, mark the coverage result
as "not measured" and give a brief reason.

### 4. Write review.md
Follow the template from the review instructions. Fill in:
- Coverage result (percentage or "not measured")
- Coverage tool output
- Coverage observations table
- Test suite output
- Sign-off checklist (all items checked if conditions are met)
- Final line: REVIEW: PASS (or REVIEW: FAIL if the test suite is not GREEN)

Sign-off conditions for PASS:
- All scenarios pass (verify.command exits 0)
- No @wip tags remain in the delta spec
- The delta spec contains no implementation-layer detail
- Coverage has been measured and recorded (or explicitly marked "not measured")

### 5. Run the review hook
Run: givn check review --change <id>
Fix any failures before reporting back.

## Report back with
- Test suite result (PASS/FAIL)
- Coverage percentage (or "not measured")
- Number of coverage observations noted (not necessarily resolved)
- Path written and confirmation that REVIEW: PASS or REVIEW: FAIL is the final line
- Any issues encountered
```

### 4. Wait for the review subagent to complete

The subagent must finish, write `review.md`, and run `givn check review` before
the orchestrator proceeds.

If the subagent reports the test suite is not GREEN, surface this to the user —
the review cannot sign off until all scenarios pass.

### 5. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-archive` to archive this change."

---

## Output

```
## Review complete: <id>

**Test suite:** GREEN
**Coverage:** <percentage or "not measured">
**Observations:** N regions noted (Dead code: N, Missing test: N, Hard to test: N)

**Review written:** givn/changes/<id>/review.md  (REVIEW: PASS)

**Next:** Run `/givn-archive` to archive this change.
```

---

## Guardrails

- The orchestrator does not run tools or write review.md — it delegates.
- Coverage is a record, not a gate. The subagent must not fail the review
  solely because coverage is below some threshold.
- The only hard gates are: test suite GREEN, no @wip, no implementation detail
  in specs, coverage measured (or explicitly waived).
- `givn lint` is a static check only — GREEN comes from `<verify.command>`.
- The archive gate hard-blocks if `REVIEW: PASS` is absent or hooks fail.
