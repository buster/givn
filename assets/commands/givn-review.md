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
review work to a **review subagent**. The review subagent runs the test suite,
measures and records coverage, classifies every gap, resolves them, and writes
`review.md`.

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
- `givn/config.yaml` — resolve `verify.command` and `verify.e2e_command`

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

Do not write that line until all sign-off conditions are met.

## Review instructions
<paste full output of: givn instructions review --change <id>>

## Verify commands
Unit/integration: <verify.command from givn/config.yaml>
E2E smoke tests:   <verify.e2e_command from givn/config.yaml>

## Steps

### 0. Fabrication audit (mandatory, do this FIRST, before trusting anything)

This has happened before: a change reached review with every task in
tasks.md checked off and the runner reporting exit 0, while nearly every
step definition was an empty stub and almost no production code existed.
Do not skip this — it is a mechanical check, not a judgment call.

1. Find every step definition file for this change. Grep each for empty or
   trivial bodies: `{}`, a bare `pass`, a bare `return`, whitespace-only
   bodies, or a body that never references any assertion library, mock,
   HTTP client, or domain object. Record the count of files scanned and any
   findings (file:line).
2. For every scenario task checked `[x]` in tasks.md, confirm a commit
   exists (check the recorded commit hash, or `git log --oneline --all
   --grep="<scenario title>"`) and that `git show --stat <hash>` touches
   production source files, not just the `.feature` file or a step stub.
   Record any checked task with no commit, or a commit touching only specs.
3. For every component design.md said would be created (services,
   controllers, classes, modules — read the Architecture Impact / Step
   Definitions sections), confirm it exists in the source tree. Record
   anything promised but missing.
4. Confirm the setup task's proof-of-strictness evidence is present in
   tasks.md and shows a non-zero exit. If missing or shows zero, the whole
   suite's GREEN status is unverified.
5. If ANY finding from 1-4 turns up: this is NOT a coverage gap to classify
   later — it is unfinished implementation. Uncheck the affected task(s) in
   tasks.md, drive them through a real RED→GREEN→REFACTOR→COMMIT cycle, and
   only then continue with the rest of this review. Do not write
   `REVIEW: PASS` while any finding remains unresolved.

Paste the audit result (files scanned, findings, remediation) into the
Fabrication Audit section of review.md, even when clean.

### 1. Confirm all tests pass
Run: <verify.command>
Run: <verify.e2e_command>
All scenarios (both non-@e2e and @e2e) must be GREEN (no @wip remaining).
If any fail, diagnose:
- Infrastructure problem (fix test plumbing only).
- Coverage gap (classify and resolve per the bucket rules below).
Do NOT change production code behaviour to make tests pass.

### 2. Measure and record coverage
Run the coverage tool specified in design.md. Record:
- The overall coverage percentage (line and/or branch)
- The raw tool output

Paste both into the Coverage section of review.md. The percentage is a fact —
it is recorded regardless of its value. It is not a threshold gate.

### 3. Classify every gap
For every uncovered region, assign exactly one bucket:

| Bucket | Meaning | Action |
|---|---|---|
| Dead code | YAGNI/KISS — should not exist | Delete it; re-run tests. |
| Missing test | Behaviour not covered by a scenario | Add scenario (@givn.added), RED/GREEN/REFACTOR. |
| Hard to test | Infrastructure, I/O boundary, etc. | Write concrete technical justification. |

### 4. Resolve each gap
- Dead code: delete, re-run tests, confirm GREEN.
- Missing test: add scenario to the relevant .feature file with @givn.added tag,
  run full RED/GREEN/REFACTOR, confirm GREEN.
- Hard to test: write the justification in review.md (no code change needed).

### 5. Confirm every capability has at least one @e2e scenario
Check each capability's .feature file for at least one @e2e-tagged scenario.
If missing, classify as "missing test" and add an @e2e scenario covering the
critical happy path through the real interaction surface.

### 6. Final GREEN confirmation
Run: <verify.command>
Run: <verify.e2e_command>
All tests (both non-@e2e and @e2e) must pass after all gap resolutions.

### 7. Write review.md
Follow the template from the review instructions. Include:
- Coverage percentage and tool output
- Coverage gap table (bucket, region, action taken for each gap)
- Test suite output and result
- Sign-off checklist (all items checked)
- Final line: REVIEW: PASS

### 8. Run the review hook
Run: givn check review --change <id>
Fix any failures before reporting back.

## Report back with
- Fabrication audit result (files scanned, findings if any, remediation taken)
- Coverage percentage recorded
- Number of gaps classified (dead code / missing test / hard to test)
- Final test run result
- Path written and confirmation that REVIEW: PASS is the final line
- Any issues encountered
```

### 4. Wait for the review subagent to complete

The subagent must finish the fabrication audit, all gap resolution, write
`review.md`, and run `givn check review` before the orchestrator proceeds.

If the subagent reports a blocker (e.g. a gap it cannot classify, or a
fabrication finding it could not remediate), surface it to the user and wait
for guidance.

### 4a. Orchestrator spot-check (do not skip)

Before accepting `REVIEW: PASS`, the orchestrator independently re-runs a
small sample rather than trusting the subagent's audit report alone:

1. Pick 2-3 step definition files at random (or all of them if there are
   fewer than 5) and grep them yourself for empty bodies. Confirm the
   subagent's "0 findings" claim, if made.
2. Pick 2-3 checked-off scenario tasks at random and confirm their commits
   exist and touch production code, via `git show --stat`.
3. If either check contradicts the subagent's report, reject `REVIEW: PASS`,
   surface the discrepancy to the user, and re-spawn the review subagent
   with the specific contradiction called out.

### 5. Show final status

```sh
givn status --change <id>
```

Display next step: "Run `/givn-archive` to archive this change."

---

## Output

```
## Review complete: <id>

**Fabrication audit:** CLEAN (or: N findings remediated — see review.md)

**Coverage:** <percentage>
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
- **The fabrication audit (step 0) is mandatory and comes first.** A change
  can have every task checked off, the runner exiting 0, and still be almost
  entirely unimplemented if step definitions were left as empty stubs. The
  audit is what catches this — it must never be skipped or treated as
  optional.
- The orchestrator performs its own spot-check (step 4a) rather than trusting
  the subagent's audit report at face value.
- Any fabrication finding is unfinished implementation, not a coverage gap —
  it must be remediated with a real RED/GREEN/REFACTOR/COMMIT cycle before
  `REVIEW: PASS`, never just noted and waved through.
- The subagent must record the coverage percentage before classifying gaps.
- The coverage number is a fact, not a threshold — do not fail the review based
  on percentage alone.
- The subagent must not write `REVIEW: PASS` until all gaps are resolved AND
  the fabrication audit is clean.
- If the subagent adds new scenarios during review, they must go through full
  RED/GREEN/REFACTOR before the review can be signed off.
- `givn lint` is a static check only — GREEN comes from `<verify.command>` and
  `<verify.e2e_command>`.
- Every capability must have at least one `@e2e` smoke-test scenario.
- The archive gate will hard-block if `REVIEW: PASS` is absent or hooks fail.
