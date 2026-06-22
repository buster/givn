---
description: Execute the characterization loop for a reverse-engineering change — GREEN-first tests, testability refactor, reverse review
---

Execute the characterization loop for a givn reverse-engineering change.
GREEN-first: tests match observed behaviour, then production code is refactored
only for testability seams.

**Input**: Optionally specify a change ID after `/givn-characterize`. If
omitted, infer from conversation context, or auto-select if only one active
change exists. If ambiguous, list changes and ask.

---

## Role: orchestrator

This command is an **orchestrator** with two phases:

1. **Characterization loop** — for each task, spawns a **characterization
   subagent** that writes tests matching observed behaviour, confirms GREEN,
   and applies testability seams if needed.
2. **Reverse review epilogue** — after all tasks are checked, spawns a
   **review subagent** that runs the coverage review with reverse-mode rules
   (no dead-code deletion, bug flagging, behaviour-preservation invariant).

The orchestrator manages the loop, passes context, and delegates.

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

If all tasks are already checked, skip to Phase 2 (reverse review epilogue).
Otherwise continue.

### 3. Validate the spec

```sh
givn lint --change <id>
```

- Exit 0 or 2 = proceed (exit 2 means @wip scenarios present — expected).
- Exit 1 = parse error — fix before characterizing.

### 4. Collect context for subagents

Read once; pass to every subagent:

- `givn/changes/<id>/tasks.md` — task list and order (each task includes a
  "Design constraints" block; if any task is missing it, read design.md and
  add it before spawning that subagent)
- `givn/changes/<id>/specs/<capability>/<capability>.feature` — scenarios
- `givn/changes/<id>/design.md` — full design document; pass this to every
  subagent in full, not just "relevant sections"
- `givn/config.yaml` — resolve the actual `verify.command`

---

## Phase 1: Characterization loop

### 5. Loop: one characterization subagent per task

For each unchecked task (`- [ ]`) in order:

#### 5a. Spawn a characterization subagent

Use the **Task tool** to spawn a characterization subagent (fall back to
describing the pattern in prose if the Task tool is not available to you).

Pass the subagent a self-contained prompt that includes:

```
You are a characterization agent executing a single task for the givn
reverse-engineering change '<id>'.

## Your task
<paste the task line verbatim, including the Design constraints block>

## Scenario to characterize
<paste the relevant scenario(s) from the .feature file>

## Context
- Verify command: <verify.command>
- Step definitions location: <from design.md>
- Seam strategy: <from design.md, if applicable to this scenario>

## Instructions

### Before any code: read the design
Read givn/changes/<id>/design.md in full before touching any file.
The "Design constraints" block in your task summarises the seam (if any) and
the testability constraints that apply to this scenario.

### GREEN phase (characterization — GREEN-first, NOT RED-first)
1. Remove `@wip` from this scenario only (leave it on all others).
2. Read the ACTUAL CODEBASE to determine what the code really does. Probe the
   behaviour. Do NOT guess from documentation — documentation can be wrong.
3. Write step definitions that assert the OBSERVED behaviour. Then steps must
   check concrete values, not just that code ran.
4. Run: <verify.command>
   This scenario MUST PASS now. You are matching reality, not driving new code.
   Note: `verify.command` runs the full suite. Other scenarios still have `@wip`
   and may fail — that's expected. Only check that YOUR scenario passes. Do not
   treat other-scenario failures as blockers.

### If the black-box boundary proves untestable:
If the code is too tightly coupled to exercise through the proposed boundary
(e.g. no public interface, hidden I/O, deep coupling that a seam can't fix
without design changes), do NOT halt the loop. Instead:
1. Comment out the scenario in the .feature file using Gherkin comments (`#`).
2. Add a comment explaining why the boundary is untestable for this scenario.
3. Check off the task (the characterization result is: "untestable via current
   boundary — follow-up needed").
4. Continue to the next scenario.
5. Report all boundary failures at the end (in your report + flag for review.md).

### If verify FAILS — diagnose in order:

#### 1. Test infrastructure problem (most common)
  - Step definitions not wired up, runner not configured, harness missing,
    import/path issues, build errors in test code, missing test deps.
  - Fix the TEST INFRASTRUCTURE only. Do not touch production code.
  - Do not touch the .feature scenario.
  - Re-run verify. If still failing, go to step 2.

#### 2. Spec/implementation mismatch (the characterization signal)
  - Infrastructure is correct; the test runs; assertions fail because the code
    does not do what the spec (inferred from docs/code) expected.
  - The spec was generated by reading documentation and code. A mismatch means
    either the documentation was wrong, or the code is ambiguous/misleading.

  **Step 1: Diagnose**
  Re-read the actual code. What does it REALLY do? Re-read the documentation
  that informed the spec (README, arc42, code comments, design.md, tasks.md).
  Determine the root cause:
    (a) The documentation was wrong — docs said Y, code correctly does Z.
    (b) The code is ambiguous/misleading — code does Z but reads like Y.
        (This includes bugs: code does Z when it should do Y, but we
        characterize AS-IS and do NOT fix.)

  **Step 2: Correct**
  If (a) documentation was wrong:
    1. Update design.md and tasks.md FIRST to reflect actual behaviour (Z).
    2. Update the .feature scenario to assert actual behaviour (Z).
    3. Update the source documentation that was wrong (README, arc42,
       code comments) — this is a documentation fix.
    4. Re-run verify → must pass.
    No code comment needed — the code was fine.

  If (b) code is ambiguous/misleading:
    1. Update the .feature scenario to assert actual behaviour (Z) —
       we characterize AS-IS, including buggy behaviour.
    2. Add a code comment at the ambiguous/misleading point that caused the
       wrong inference. Use the language's comment syntax starting with
       `REVERSE-ENGINEERING NOTE:`:
         // REVERSE-ENGINEERING NOTE: <what was misleading> — actual
         // behaviour: <Z>. Code smell, refactor candidate.
       (Adapt the comment syntax to the language: // for Rust/C/JS, # for
       Python/Shell/Ruby, -- for SQL/Lua, <!-- --> for HTML/XML, etc.)
    3. Do NOT change code behaviour. Only add the comment.
    4. Re-run verify → must pass.
    5. Flag this as a code smell in your report.

  **NEVER:**
    - Change production code behaviour to match the spec.
    - Fix bugs discovered during characterization.
    - Remove or skip REVERSE-ENGINEERING NOTE markers.

### REFACTOR phase (only if design.md calls for a testability seam on this scenario)
  5. Apply the seam documented in design.md for this scenario:
     - Allowed: extract interface, dependency injection, make a function/method
       public, add a test-only constructor or builder, extract a pure function
       from a side-effectful one.
     - NOT allowed: rename, deduplicate, simplify logic, restructure for
       clarity, fix bugs, change error messages, change return types, change
       control flow.
  6. Run: <verify.command> → MUST still pass.
     If red: the refactor changed behaviour → REVERT the refactor entirely.
     Do NOT try to fix the test — the test was correct; the refactor broke
     behaviour.
  7. If no seam is needed for this scenario: skip REFACTOR.

### Done
  8. Check off the task in givn/changes/<id>/tasks.md: `- [ ]` → `- [x]`
  9. Report back with:
     - GREEN result (pass on first try, after diagnosis, or boundary failure)
     - If diagnosis was needed: root cause (infrastructure / documentation
       wrong / code ambiguous), what was corrected
     - If boundary failure: scenario commented out, reason, suggested follow-up
     - REFACTOR result (applied / not needed / reverted)
     - Any REVERSE-ENGINEERING NOTE markers added (file, line, content)
     - Any suspected bugs flagged
     - Any issues or decisions made
```

#### 5b. Wait for the subagent to report back

The subagent must complete the characterization cycle and check off the task
before the orchestrator proceeds to the next task.

If the subagent reports a blocker (e.g. a spec/implementation mismatch it
cannot resolve), surface it to the user and wait for guidance before spawning
the next subagent.

#### 5c. After each subagent completes

```sh
givn apply --change <id>
givn lint --change <id>
```

Confirm the task was checked off. Confirm lint exits 0 or 2 (exit 1 = parse
error from a spec correction — surface to the user before continuing). Then
move to the next task.

**IMPORTANT:** Do NOT use `givn check <artifact>` during the characterization
loop — it fires the `post: ["verify"]` hook and treats failure as a gate
violation. During characterization, verify failures are expected and are
handled by the subagent. Run `verify.command` directly (via the subagent),
not through `givn check`.

### 6. On characterization loop completion

```sh
givn apply --change <id>
```

Show progress. If all tasks are checked, proceed to Phase 2.

---

## Phase 2: Reverse review epilogue

### 7. Collect review instructions

Read the reverse-mode review instruction (source: `assets/instructions/review-reverse.md`).
Capture its full content — it will be passed to the review subagent.

```sh
givn instructions review --change <id>
```

Also capture the standard review template path from the output.

### 8. Spawn a review subagent

Use the **Task tool** to spawn a review subagent.

Pass a self-contained prompt:

```
You are a review agent completing the reverse-engineering coverage review for
givn change '<id>'.

## Your objective
Produce a complete, signed-off review.md at:
  givn/changes/<id>/review.md

The final line of the file MUST be:
  REVIEW: PASS

Do not write that line until all sign-off conditions are met.

## Reverse-mode review instructions
<paste full content of assets/instructions/review-reverse.md>

## Verify command
<verify.command from givn/config.yaml>

## Steps

### 1. Confirm all tests pass
Run: <verify.command>
All scenarios must be GREEN (no @wip remaining). If any fail, diagnose:
- Infrastructure problem (fix test plumbing only — do NOT touch production code).
- Spec/implementation mismatch (correct the spec, never the code — see the
  diagnostic in the characterization instructions).
Do NOT change production code behaviour to make tests pass.

### 2. Measure and record coverage
Run the coverage tool specified in design.md. Record:
- The overall coverage percentage (line and/or branch)
- The raw tool output
Paste both into the Coverage section of review.md.

### 3. Classify every gap (reverse-mode rules)
See the reverse-mode review instructions (pasted above) for the bucket table
and rules. Key constraint: do NOT delete dead code, do NOT add missing tests
during review — flag both as follow-up candidates.

### 4. Resolve each gap
- Dead code: flag as follow-up @givn.removed candidate. Do NOT delete.
- Missing test: flag as follow-up @givn.added candidate. Do NOT add during review.
- Hard to test: write the justification in review.md.

### 5. Behaviour-preservation invariant
Re-run: <verify.command>
All characterization tests must still be GREEN after all refactoring.
If any went RED during the change, flag it as a blocker.

### 5a. Update arc42 docs (if addon enabled and spec corrections were made)
If any spec corrections were made during characterization (scenarios updated
because the initial inference was wrong), update the affected arc42 chapters
under `docs/arc42/`. The reverse-engineering process may have revealed
architectural details that differ from what was initially documented. Update
chapters 5 (building block view), 6 (runtime view), and 8 (cross-cutting
concepts) as needed. Do NOT rewrite unrelated sections — surgical edits only.

### 6. Compile mandatory reporting sections
- Spec corrections (scenarios updated because initial inference was wrong)
- Documentation corrections (docs/design/tasks updated because they were wrong)
- Code smell markers (all REVERSE-ENGINEERING NOTE comments, with file/line)
- Suspected bugs (all bugs discovered, with follow-up @givn.modified candidate)
- Coverage gap table
- Test suite output and result
- Sign-off checklist

### 7. Write review.md
Follow the reverse-mode review instruction structure. Include all mandatory
reporting sections. Final line: REVIEW: PASS

### 8. Run the review hook
Run: givn check review --change <id>
Fix any failures before reporting back.

## Report back with
- Coverage percentage recorded
- Number of gaps classified (dead code / missing test / hard to test)
- Number of spec corrections
- Number of documentation corrections
- Number of code smell markers added
- Number of suspected bugs flagged
- Number of boundary failures (scenarios commented out)
- Final test run result
- Path written and confirmation that REVIEW: PASS is the final line
- Any issues encountered
```

### 9. Wait for the review subagent to complete

The subagent must finish all gap resolution, write `review.md`, and run
`givn check review` before the orchestrator proceeds.

If the subagent reports a blocker, surface it to the user and wait.

### 10. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-archive` to archive this change and create the
baseline spec."

---

## Output during characterization

```
## Characterizing: <id>

Task 3/7: <capability>: <Scenario title>
  → Spawning characterization subagent...
  GREEN:      PASSED (on first try / after diagnosis: <root cause>)
  REFACTOR:   APPLIED / NOT NEEDED / REVERTED
  [x] task checked off

Task 4/7: ...
```

## Output on completion

```
## Characterization complete: <id>

**Progress:** 7/7 tasks complete

**Spec corrections:** N (scenarios updated to match AS-IS behaviour)
**Documentation corrections:** N (docs/design/tasks fixed)
**Code smell markers:** N (REVERSE-ENGINEERING NOTE comments added)
**Suspected bugs flagged:** N (follow-up @givn.modified candidates)
**Boundary failures:** N (scenarios commented out — untestable via current boundary)

**Review:** givn/changes/<id>/review.md  (REVIEW: PASS)

**Next:** Run `/givn-archive` to archive this change and create the baseline spec.
```

## Output on pause

```
## Characterization paused: <id>

**Progress:** 4/7 tasks complete

**Blocker:** <description>

**Waiting for:** <what is needed to continue>
```

---

## Guardrails

- The orchestrator does not write code — it manages the loop and delegates.
- Each subagent handles exactly one task (one characterization cycle).
- GREEN must genuinely pass. If the subagent reports it failed on first run,
  the subagent must diagnose (infrastructure vs spec/mismatch) before
  proceeding — do NOT change production code behaviour to make it pass.
- Do not spawn the next subagent until the previous one has checked off its task.
- Remove `@wip` from one scenario at a time only — pass this constraint to each
  subagent explicitly.
- Do NOT use `givn check <artifact>` during the characterization loop — run
  `verify.command` directly via the subagent. `givn check` is only used at the
  review epilogue (`givn check review --change <id>`).
- If a task is ambiguous, ask the user before spawning its subagent.
- Do not batch multiple scenarios into one subagent.
- REFACTOR is only for testability seams documented in design.md — never for
  general cleanup (rename, dedup, logic simplification).
- If a REFACTOR makes a characterization test go red, REVERT the refactor — do
  not fix the test. The test was correct; the refactor broke behaviour.
- Suspected bugs are flagged, never fixed.
- All REVERSE-ENGINEERING NOTE markers are intentional debt markers — never
  removed during characterization.
- `givn lint` is a static check only — GREEN comes from `<verify.command>`.
- The archive gate will hard-block if `REVIEW: PASS` is absent or hooks fail.
