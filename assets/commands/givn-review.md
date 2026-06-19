---
description: Complete the coverage review for a givn change — classify every gap, sign off, confirm GREEN
---

Complete the coverage review for a givn change.

**Input**: Optionally specify a change ID after `/givn-review`. If omitted, infer
from conversation context, or auto-select if only one active change exists. If
ambiguous, list changes and ask.

---

## Role: orchestrator

This command is an **orchestrator**. It collects context and then delegates all
review work to a **review subagent**. The review subagent is responsible for
running tools, classifying gaps, resolving them, and writing `review.md`.

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
You are a review agent completing the coverage review for givn change '<id>'.

## Your objective
Produce a complete, signed-off review.md at:
  givn/changes/<id>/review.md

The final line of the file MUST be:
  REVIEW: PASS

Do not write that line until all gaps are resolved or justified.

## Review instructions
<paste full output of: givn instructions review --change <id>>

## Verify command
<verify.command from givn/config.yaml>

## Steps

### 1. Confirm all tests pass
Run: <verify.command>
All scenarios must be GREEN (no @wip remaining). If any fail, fix them first.

### 2. Run coverage tooling
Run the coverage tool specified in design.md. Collect uncovered lines/branches.

### 3. Classify every gap
For every uncovered region, assign exactly one bucket:

| Bucket | Meaning | Action |
|---|---|---|
| Dead code | YAGNI/KISS — should not exist | Delete it; re-run tests. |
| Missing test | Behaviour not covered by a scenario | Add scenario (@givn.added), RED/GREEN/REFACTOR. |
| Legitimately hard to test | Infrastructure, I/O boundary, etc. | Write concrete technical justification. |

There is no minimum coverage percentage. Coverage is complete when every gap
is classified and resolved or justified.

### 4. Resolve each gap
- Dead code: delete, re-run tests, confirm GREEN.
- Missing test: add scenario to the relevant .feature file with @givn.added tag,
  run full RED/GREEN/REFACTOR, confirm GREEN.
- Hard to test: write the justification in review.md (no code change needed).

### 5. Final GREEN confirmation
Run: <verify.command>
All tests must pass after all gap resolutions.

### 6. Write review.md
Follow the template from the review instructions. Include:
- Coverage gap table (bucket, region, resolution/justification for each gap)
- Sign-off checklist (all items checked)
- Final line: REVIEW: PASS

### 7. Run the review hook
Run: givn check review --change <id>
Fix any failures before reporting back.

## Report back with
- Number of gaps classified (dead code / missing test / hard to test)
- Final test run result
- Path written and confirmation that REVIEW: PASS is the final line
- Any issues encountered
```

### 4. Wait for the review subagent to complete

The subagent must finish all gap resolution, write `review.md`, and run
`givn check review` before the orchestrator proceeds.

If the subagent reports a blocker (e.g. a gap it cannot classify), surface it
to the user and wait for guidance.

### 5. Show final status

```sh
givn status --change <id>
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

- The orchestrator does not run coverage tools or write review.md — it delegates.
- The review subagent must not write `REVIEW: PASS` until all gaps are resolved.
- If the subagent adds new scenarios during review, they must go through full
  RED/GREEN/REFACTOR before the review can be signed off.
- `givn lint` is a static check only — GREEN comes from `<verify.command>`.
- The archive gate will hard-block if `REVIEW: PASS` is absent or hooks fail.
